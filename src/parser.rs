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
use ParseError::*;



type Token<'a> = &'a str;
type OwnedToken = String;

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

type ParseResult<R> = Result<R, Vec<ParseError>>;
trait ParseFn<R> = Fn(&mut Input) -> ParseResult<R>;


fn advance(input: &mut Input, by: usize)
{
	input.source = &input.source[by..];
}



fn is_special(token: &Token) -> bool
{
	for i in 0x20..=0x2F {
		if std::str::from_utf8(&[i]) == Ok(token) {
			return true;
		}
	}
	for i in 0x3A..=0x40 {
		if std::str::from_utf8(&[i]) == Ok(token) {
			return true;
		}
	}
	for i in 0x5B..=0x60 {
		if std::str::from_utf8(&[i]) == Ok(token) {
			return true;
		}
	}
	for i in 0x7B..=0x7F {
		if std::str::from_utf8(&[i]) == Ok(token) {
			return true;
		}
	}
	false
}

fn is_invalid(token: &Token) -> bool {
	for i in 0x00..=0x1F {
		if i != 0x09 && i != 0x0A && i != 0x0D // allow \t, \n, \r
		&& std::str::from_utf8(&[i]) == Ok(token) {
			return true;
		}
	}
	false
}

fn is_newline(token: &Token) -> bool
{
	[	"\r\n", "\n", "\r"
	].contains(&token)
}


// ---


fn map<A, R>(
    parser: impl ParseFn<A>,
    map_fn: impl Fn(A) -> R
)
-> impl ParseFn<R>
{
    move |input: &mut Input| parser(input).map(|result| map_fn(result))
}


fn discard<A>(
	parser: impl ParseFn<A>
)
-> impl ParseFn<()>
{
	move |input: &mut Input| parser(input).map(|result|())
}


fn any_of<'a, R>(
	of: &'a [&dyn ParseFn<R>]
)
-> impl ParseFn<R> + 'a
{
	move |input: &mut Input| {
		let mut errors = Vec::new();
		for option in of {
			let result = option(input);
			match result {
				Ok(_) => return result,
				Err(mut err) => {
					errors.append(&mut err);
					continue
				}
			}
		}
		Err(errors)
	}
}


fn zero_or_more<R>(
	parser: impl ParseFn<R>
)
-> impl ParseFn<Vec<R>>
{
	move |input: &mut Input| {
		let mut result = Vec::new();
		while let Ok(next) = parser(input) {
			result.push(next);
		}
		Ok(result)
	}
}


fn one_or_more<R>(
	parser: impl ParseFn<R>
)
-> impl ParseFn<Vec<R>>
{
	move |input: &mut Input| {
		let mut result = Vec::new();
		let last_err;
		loop  {
			match parser(input) {
				Ok(next) => result.push(next),
				Err(err) => {
					last_err = err;
					break;
				}
			}
		}
		if result.is_empty() {
			return Err(last_err);
		}
		Ok(result)
	}
}


fn optional<R>(
	parser: impl ParseFn<R>
)
-> impl ParseFn<Option<R>>
{
	move |input: &mut Input| Ok(parser(input).ok())
}


fn pair<RL, RR>(
	parserl: impl ParseFn<RL>,
	parserr: impl ParseFn<RR>
)
-> impl ParseFn<(RL, RR)>
{
	move |input: &mut Input| {
		#[allow(clippy::clone_double_ref)]
		let input_copy = &mut input.clone();
		match parserl(input_copy) {
			Ok(resultl) => {
				match parserr(input_copy) {
					Ok(resultr) => {
						*input = *input_copy;
						Ok((resultl, resultr))
					},
					Err(resultr) => {
						Err(resultr)
					}
				}
			},
			Err(resultl) => {
				Err(resultl)
			}
		}
	}
}


fn left<RL, RR>(
	parserl: impl ParseFn<RL>,
	parserr: impl ParseFn<RR>
)
-> impl ParseFn<RL>
{
	map(pair(parserl, parserr), |(resultl, resultr)|resultl)
}


fn right<RL, RR>(
	parserl: impl ParseFn<RL>,
	parserr: impl ParseFn<RR>
)
-> impl ParseFn<RR>
{
	map(pair(parserl, parserr), |(resultl, resultr)|resultr)
}


fn pred<R>(
	parser: impl ParseFn<R>,
	pred_fn: impl Fn(&R) -> bool,
	err_fn: impl Fn(&R) -> ParseResult<R>
)
-> impl ParseFn<R> 
{
	move |input: &mut Input| {
		#[allow(clippy::clone_double_ref)]
		let input_copy = &mut input.clone();
		match parser(input_copy) {
			Ok(result) => {
				if pred_fn(&result) {
					*input = *input_copy;
					Ok(result)
				}
				else {
					err_fn(&result)
				}
			},
			Err(result) => {
        		Err(result)
			}
		}
	}
}


fn any_until<U>(
	until: impl ParseFn<U>
)
-> impl ParseFn<String>
{
	move |input: &mut Input| {
		let mut comment = String::new();
		let mut last_err = vec!();
		while let Err(err) = until(input)  {
			last_err = err;
			let next = next(input);
			if let Ok(next) = next {
				comment.push_str(next.as_str());
				advance(input, 1);
			}
			else {
				return next;
			}
		}
		if comment.is_empty() {
			return Err(last_err);
		}
		Ok(comment)
	}
}


fn all_until<R, U>(
	parser: impl ParseFn<R>,
	until: impl ParseFn<U>
)
-> impl ParseFn<Vec<R>>
{
	move |input: &mut Input| {
		let mut result = Vec::new();
		loop {
			match parser(input) {
				Ok(next) => result.push(next),
				Err(err) => {
					return Err(err)
				}
			}
			if let Ok(_) = until(input) {
				break;
			}
		}
		Ok(result)
	}
}


fn literal(
	expected: &'static str
)
-> impl ParseFn<bool>
{
	move |input: &mut Input| {
		let len = expected.graphemes(true).count();
		let next = input.source.iter().take(len).join("");
		debug!("{}⮬ matching literal {:?} with {:?}", String::from("   ").repeat(input.context.indent-1), next, expected);
		if next == expected {
			advance(input, len);
			Ok(true)
		}
		else {
			Err(vec!(ExpectedCharacter{expected: expected.into(), found: next}))
		}
	}
}


fn debug<R>(message: &'static str, parser: impl ParseFn<R>) -> impl ParseFn<R> {
	move |input: &mut Input| {
		debug!("{}{}", String::from("   ").repeat(input.context.indent-1), message);
		parser(input)
	}
}

fn peek<R>(
	parser: impl ParseFn<R>,
)
-> impl ParseFn<R>
{
	move |input: &mut Input| {
		#[allow(clippy::clone_double_ref)]
		let input_copy = &mut input.clone();
		parser(input_copy)
	}
}


// ---


fn newline(input: &mut Input) -> ParseResult<()>
{
	discard(right(
		zero_or_more(tab), 
		pred(
			next, 
			|r|is_newline(&r.as_str()), 
			|r|Err(vec!(ExpectedCharacter{expected: "<newline>".into(), found: r.into()}))
		)))(input)
}


fn next(input: &mut Input) -> ParseResult<OwnedToken>
{
	let result = input.source.iter().next();
	if let Some(&result) = result {
		advance(input, 1);
		Ok(result.into())
	}
	else {
		Err(vec!(UnexpectedEOF{line: 0, position: 0}))
	}
}


fn space(input: &mut Input) -> ParseResult<()>
{
	discard(literal(" "))(input)
}

fn tab(input: &mut Input) -> ParseResult<()>
{
	discard(literal("\t"))(input)
}

fn eof(input: &mut Input) -> ParseResult<()>
{
	if input.source.is_empty() {
		Ok(())
	}
	else {
		Err(vec!(ExpectedCharacter{expected: "<eof>".into(), found: input.source[0].into()}))
	}
}

fn indent(input: &mut Input) -> ParseResult<()>
{
	if input.context.indent > 1 {
		let mut iter = input.source.iter();
		for i in 1..input.context.indent {
			if let Some(&next) = iter.next() {
				if next != "\t" {
					return Err(vec!(ExpectedCharacter{expected: "\t".into(), found: next.into()}));
				}
			}
			else {
				return Err(vec!(UnexpectedEOF{line:0,position:0}));
			}
		}
		advance(input, input.context.indent - 1);
	}
	Ok(())
}


fn identifier(input: &mut Input) -> ParseResult<String>
{
	let iter = input.source.iter();
	let mut id = String::new();
	let mut result = Err(vec!());
	for next in iter {
		if is_invalid(next) {
			result = Err(vec!(UnexpectedCharacter{found: (*next).into()}));
			break;
		}
		if is_special(next) {
			result = Err(vec!(UnexpectedCharacter{found: (*next).into()}));
			break;
		}
		if is_newline(next) {
			result = Err(vec!(UnexpectedCharacter{found: (*next).into()}));
			break;
		}
		id.push_str(next);
	}
	if !id.is_empty() {
		advance(input, id.len());
		result = Ok(id);
	}
	result
}



#[derive(Debug, Clone)]
enum SyntaxElement {
	Expression(Expression),
	Comment(Comment),
}


type Comment = String;
fn comment(input: &mut Input) -> ParseResult<SyntaxElement>
{
	map(right(literal("#"), any_until(newline)), SyntaxElement::Comment)(input)
}

#[derive(Debug, Clone)]
struct Expression {
	identifiers: Vec<String>,
	block: Block
}
fn expression(input: &mut Input) -> ParseResult<SyntaxElement>
{
	map(
		left(
			pair(one_or_more(left(identifier, zero_or_more(space))), block), 
			peek(any_of(&[&newline, &eof]))
		), 
		|(identifiers, block)| {
			SyntaxElement::Expression(Expression {
				identifiers,
				block
			})
		}
	)(input)
}


#[derive(Debug, Clone)]
enum Block {
	Valid(Vec<SyntaxElement>),
	Malformed(Vec<ParseError>),
	None
}
fn block(input: &mut Input) -> ParseResult<Block>
{
	let debug_indent = String::from("   ").repeat(input.context.indent);

	input.context.indent += 1;
	debug!("{}⇲ NEW BLOCK with indent {}", debug_indent, input.context.indent);

	let result = zero_or_more(right(
		debug("⬤ newline", zero_or_more(newline)),
		right(indent, any_of(&[
			&debug("⬤ expression!", expression),
			&debug("⬤ comment!", comment),
		])),
	))(input);
	
	// let result = all_until(
	// 	right(
	// 		debug("⬤ newline", zero_or_more(newline)),
	// 		right(indent, any_of(&[
	// 			&debug("⬤ expression!", expression),
	// 			&debug("⬤ comment!", comment),
	// 		])),
	// 	),
	// 	right(zero_or_more(newline), any_of(&[&eof]))
	// )(input);

	// let result: ParseResult<Vec<SyntaxElement>>;
	// loop {
	// 	let _ = debug("⬤ newline", zero_or_more(newline))(input);
	// 	let expr = right(indent, any_of(&[
	// 		&debug("⬤ expression!", expression),
	// 		&debug("⬤ comment!", comment),
	// 	]));
	// }

	debug!("{}⇱ END BLOCK with indent {}, ⭑block: {:#?}\n", debug_indent, input.context.indent, result);
	input.context.indent -= 1;

	match result {
		Ok(elements) if elements.is_empty() => {
			Ok(Block::None)
		},
		Ok(elements) => {
			Ok(Block::Valid(elements))
		}
		Err(errors) => {
			Ok(Block::Malformed(errors))
		}
	}
}



pub fn main(input: std::path::PathBuf) -> Result<SuccessInfo, ParseError> {
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
	let result = block(input);
	info!("{:#?}", result);

	if format!("{:#?}", result) == parser_expected {
		Ok(SuccessInfo{message: String::from("Success!")})
	}
	else {
		Err(Generic("does not match expected results".into()))
	}
}



const parser_expected: &str = r#"Ok(
    Valid(
        [
            Expression(
                Expression {
                    identifiers: [
                        "xxxxx",
                        "111",
                    ],
                    block: Valid(
                        [
                            Expression(
                                Expression {
                                    identifiers: [
                                        "yyyyy",
                                        "222",
                                    ],
                                    block: Valid(
                                        [
                                            Expression(
                                                Expression {
                                                    identifiers: [
                                                        "zzzzzz",
                                                        "333",
                                                    ],
                                                    block: None,
                                                },
                                            ),
                                            Expression(
                                                Expression {
                                                    identifiers: [
                                                        "aaaaaa",
                                                        "444",
                                                    ],
                                                    block: None,
                                                },
                                            ),
                                        ],
                                    ),
                                },
                            ),
                            Expression(
                                Expression {
                                    identifiers: [
                                        "bbbbb",
                                        "555",
                                    ],
                                    block: None,
                                },
                            ),
                        ],
                    ),
                },
            ),
            Expression(
                Expression {
                    identifiers: [
                        "cccc",
                        "666",
                    ],
                    block: None,
                },
            ),
        ],
    ),
)"#;


// fn dedent(input: &mut Input) -> ParseResult<()>
// {
// 	let mut counter = 1;
// 	let mut iter = input.source.iter();
// 	loop {
// 		match iter.next() {
// 			Some(&"\t") => counter += 1,
// 			// Some(&"\n") => return Err(vec!(Empty)),
// 			None => return Ok(()),
// 			Some(next) => {
// 				info!("...newline? {:#?} {:?} in the following substring after {} idents: {:#?}", next, is_newline(next), counter, input.source[..10].join(""));
// 				if is_newline(next) {
// 					info!("advancing..");
// 					advance(input, 1);
// 					return Err(vec!(Empty));
// 				}
// 				else {
// 					break;
// 				}
// 			}
// 		}
// 	}
// 	info!("⇊ COMPARING indent {} (found) with {} (context)", counter, input.context.indent);
// 	info!("⇊ with following substring: {:#?}", input.source[..10].join(""));
// 	if counter < input.context.indent {
// 		Ok(())
// 	}
// 	else {
// 		Err(vec!(Empty))
// 	}
// }
