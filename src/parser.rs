use std::fmt::Display;
use std::error::Error;
use std::iter::Iterator;
use std::collections::*;

use derive_more::*;
use itertools::*;
use unicode_reader::Graphemes;
use unicode_segmentation::UnicodeSegmentation;


mod result;
use result::*;



type Token<'a> = &'a str;

#[derive(Debug, Clone, Copy)]
struct InputContext
{
	indent: usize
}
#[derive(Debug, Clone, Copy)]
struct Input<'a>
{
	source: &'a [Token<'a>],
	context: InputContext
}

type ParseResult<Result> = Option<Result>;
trait ParseFn<R> = Fn(&mut Input) -> ParseResult<R>;



fn advance(input: &mut Input, by: usize)
{
	input.source = &input.source[by..];
}



fn is_special(token: Token) -> bool
{
	[	" ", "#", ":", ".", ",",  "[", "]", "{", "}",
		"(", ")", "/", "\\", "\"", "\n", "\r\n", "\t"
	].contains(&token)
}

fn is_invalid(token: Token) -> bool {
	for i in 0x00..0x1F {
		if i != 0x09 && i != 0x0A && i != 0x0D // allow \t, \n, \r
		&& std::str::from_utf8(&[i]) == Ok(token) {
			return true;
		}
	}
	false
}



fn map<A, R>(
    parser: impl ParseFn<A>,
    map_fn: impl Fn(A) -> R)
-> impl ParseFn<R>
{
    move |input: &mut Input| parser(input).map(|result| map_fn(result))
}


fn discard<A>(parser: impl ParseFn<A>) -> impl ParseFn<()>
{
	move |input: &mut Input| parser(input).map(|result|())
}

fn any_of<'a, R>(of: &'a [Box<dyn ParseFn<R>>]) -> impl ParseFn<R> + 'a {
	move |input: &mut Input| {
		for option in of {
			let result = option(input);
			match result {
				Some(_) => return result,
				_ => continue
			}
		}
		None
	}
}


fn zero_or_more<R>(parser: impl ParseFn<R>) -> impl ParseFn<Vec<R>>
{
	move |input: &mut Input| {
		let mut result = Vec::new();
		while let Some(next) = parser(input) {
			result.push(next);
		}
		Some(result)
	}
}
fn one_or_more<R>(parser: impl ParseFn<R>) -> impl ParseFn<Vec<R>>
{
	move |input: &mut Input| {
		let mut result = Vec::new();
		while let Some(next) = parser(input) {
			result.push(next);
		}
		if result.is_empty() {
			return None;
		}
		Some(result)
	}
}
fn optional<R>(parser: impl ParseFn<R>) -> impl ParseFn<Option<R>>
{
	move |input: &mut Input| Some(parser(input))
}


fn pair<RL, RR>(parserl: impl ParseFn<RL>, parserr: impl ParseFn<RR>) -> impl ParseFn<(RL, RR)>
{
	move |input: &mut Input| {
		#[allow(clippy::clone_double_ref)]
		let input_copy = &mut input.clone();
		if let Some(resultl) = parserl(input_copy) {
			if let Some(resultr) = parserr(input_copy) {
				*input = *input_copy;
				Some((resultl, resultr))
			} else { None }
		} else { None }
	}
}
fn left<RL, RR>(parserl: impl ParseFn<RL>, parserr: impl ParseFn<RR>) -> impl ParseFn<RL>
{
	map(pair(parserl, parserr), |(resultl, resultr)|resultl)
}
fn right<RL, RR>(parserl: impl ParseFn<RL>, parserr: impl ParseFn<RR>) -> impl ParseFn<RR>
{
	map(pair(parserl, parserr), |(resultl, resultr)|resultr)
}


fn pred<R, F>(parser: impl ParseFn<R>, pred_fn: impl Fn(&R) -> bool) -> impl ParseFn<R>
{
	move |input: &mut Input| {
		if let Some(result) = parser(input) {
            if pred_fn(&result) {
                return Some(result);
            }
        }
        None
	}
}


fn any_until<R>(parser: impl ParseFn<R>) -> impl ParseFn<String>
{
	move |input: &mut Input| {
		let mut comment = String::new();
		while let None = parser(input) {
			println!("check while not none ");
			comment.push_str(input.source[0]);
			advance(input, 1);
		}
		if comment.is_empty() {
			return None;
		}
		Some(comment)
	}
}


fn literal(expected: &'static str) -> impl ParseFn<bool>
{
	move |input: &mut Input| {
		let len = expected.graphemes(true).count();
		let next = input.source.iter().take(len).join("");
		println!("↶ matching literal {:?} with {:?}", next, expected);
		if next == expected {
			advance(input, len);
			Some(true)
		}
		else {
			None
		}
	}
}

fn newline(input: &mut Input) -> ParseResult<()>
{
	discard(any_of(&[
		box literal("\r\n"),
		box literal("\n"),
		box literal("\r"),
	]))(input)
}

fn space(input: &mut Input) -> ParseResult<()>
{
	discard(literal(" "))(input)
}

fn indent(input: &mut Input) -> ParseResult<()>
{
	if input.context.indent > 1 {
		let mut iter = input.source.iter();
		for i in 1..input.context.indent {
			if Some(&"\t") != iter.next() {
				return None;
			}
		}
		advance(input, input.context.indent - 1);
	}
	Some(())
}

fn identifier(input: &mut Input) -> ParseResult<String>
{
	let iter = input.source.iter();
	let mut id = String::new();
	for next in iter {
		if is_invalid(next) {
			break;
		}
		if is_special(next) {
			break;
		}
		id.push_str(next);
	}
	if id.is_empty() {
		return None;
	}
	advance(input, id.len());
	Some(id)
}


fn debug<R>(message: &'static str, parser: impl ParseFn<R>) -> impl ParseFn<R> {
	move |input: &mut Input| {
		println!("{}", message);
		parser(input)
	}
}


#[derive(Debug, Clone)]
enum SyntaxElement {
	Expression(Expression),
	Comment(Comment)
}


type Comment = String;
fn comment(input: &mut Input) -> ParseResult<SyntaxElement>
{
	map(right(literal("#"), any_until(newline)), SyntaxElement::Comment)(input)
}

#[derive(Debug, Clone)]
struct Expression {
	identifiers: Vec<String>,
	block: Option<Block>
}
fn expression(input: &mut Input) -> ParseResult<SyntaxElement>
{
	map(pair(one_or_more(left(identifier, zero_or_more(space))), optional(block)), |(identifiers, block)| SyntaxElement::Expression(Expression {
		identifiers,
		block
	}))(input)
}

type Block = Vec<SyntaxElement>;
fn block(input: &mut Input) -> ParseResult<Block>
{
	input.context.indent += 1;
	let r = one_or_more(right(
		debug("zom nl", zero_or_more(newline)),
		any_of(&[
			box debug("zom comm", right(indent, comment)),
			box debug("expression!", right(indent, expression))
		])
	))(input);
	input.context.indent -= 1;
	r
}



pub fn main(input: std::path::PathBuf) -> Result<SuccessInfo, RuntimeError> {
	let name = input.file_stem()?.to_owned().to_str()?;
	let source = std::fs::read_to_string(input)?;
	let source = source.as_str().graphemes(true).collect::<Vec<_>>();
	let iter = &mut &source[..];
	let input = &mut Input {
		source: iter,
		context: InputContext {
			indent: 0
		}
	};

	// println!("{:?}", pair(literal("f"), literal("function"))(iter));
	// println!("{:?}", pair(literal("f"), literal("unction"))(iter));
	// println!("{:?}", literal("no")(iter));
	// println!("{:?}", zero_or_more(literal("no"))(iter));
	// println!("{:?}", discard(one_or_more(space))(iter));
	// println!("{:?}", literal("calls")(iter));
	println!("{:#?}", block(input));
	// println!("{:?}", one_or_more(map(newline, |i|"<newline>"))(input));
	// println!("{:?}", expression(input));

	Ok(SuccessInfo{message: String::from("Success!")})
}
