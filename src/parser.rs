use std::fmt::{Display, Debug};
use std::iter::Iterator;
use std::collections::*;
use std::ops::*;

use derive_more::*;
use itertools::*;
use unicode_reader::Graphemes;
use unicode_segmentation::UnicodeSegmentation;


mod result;
use result::*;
use result::ParseError::*;


type Token<'a> = &'a str;
type OwnedToken = String;


trait InputIterItem = Debug + Display + Into<String> + AsRef<str> + PartialEq + Eq + ToOwned;
// trait InputIterItem: Debug + Display + Into<String> + AsRef<str> + PartialEq + Eq + ToOwned {}
// impl<T> InputIterItem for T where T: Debug + Display + Into<String> + AsRef<str> + PartialEq + Eq + ToOwned {}
trait InputIter = Clone + Debug + Iterator::<Item: InputIterItem>;


#[derive(Debug, Clone)]
struct InputContext
{
	indent: usize,
}

#[derive(Debug, Clone)]
struct Input<I: Clone>
{
	source: I,
	context: InputContext
}
impl<I: InputIter> Input<I> {
	fn from(source: I) -> Self {
		Input {
			source,
			context: InputContext {
				indent: 0
			}
		}
	}
	fn advance(&mut self, by: usize)
	{
		for i in 0..by {
			debug!("..ad..  {}", self.source.next().unwrap());
		}
	}
}

type InputRef<'a, I> = &'a mut Input<I>;

trait Error {

}

type ParseResult<R> = Result<R, Vec<ParseError>>;
trait ParseFn<R, I> = Fn(&mut Input<I>) -> ParseResult<R>;






fn is_special(token: Token) -> bool
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

fn is_invalid(token: Token) -> bool {
	for i in 0x00..=0x1F {
		if i != 0x09 && i != 0x0A && i != 0x0D // allow \t, \n, \r
		&& std::str::from_utf8(&[i]) == Ok(token) {
			return true;
		}
	}
	false
}

fn is_newline(token: Token) -> bool
{
	[	"\r\n", "\n", "\r"
	].contains(&token)
}


// ---


fn map<A, R, I: InputIter>(
    parser: impl ParseFn<A, I>,
    map_fn: impl Fn(A) -> ParseResult<R>
)
-> impl ParseFn<R, I>
{
    move |input: InputRef<I>| parser(input).and_then(|result| map_fn(result))
}

fn map_err<R, I: InputIter>(
    parser: impl ParseFn<R, I>,
    map_fn: impl Fn(Vec<ParseError>) -> ParseResult<R>,
)
-> impl ParseFn<R, I>
{
    move |input: InputRef<I>| parser(input).or_else(|result| map_fn(result))
}


fn discard<A, I: InputIter>(
	parser: impl ParseFn<A, I>
)
-> impl ParseFn<(), I>
{
	move |input: InputRef<I>| parser(input).map(|result|())
}


fn any_of<'a, R, I: InputIter>(
	of: &'a [&dyn ParseFn<R, I>]
)
-> impl ParseFn<R, I> + 'a
{
	move |input: InputRef<I>| {
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


fn zero_or_more<R, I: InputIter>(
	parser: impl ParseFn<R, I>
)
-> impl ParseFn<Vec<R>, I>
{
	move |input: InputRef<I>| {
		let mut result = Vec::new();
		while let Ok(next) = parser(input) {
			result.push(next);
		}
		Ok(result)
	}
}


fn one_or_more<R, I: InputIter>(
	parser: impl ParseFn<R, I>
)
-> impl ParseFn<Vec<R>, I>
{
	move |input: InputRef<I>| {
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


fn optional<R, I: InputIter>(
	parser: impl ParseFn<R, I>
)
-> impl ParseFn<Option<R>, I>
{
	move |input: InputRef<I>| Ok(parser(input).ok())
}


fn pair<RL, RR, I: InputIter>(
	parserl: impl ParseFn<RL, I>,
	parserr: impl ParseFn<RR, I>
)
-> impl ParseFn<(RL, RR), I>
{
	move |input: InputRef<I>| {
		#[allow(clippy::clone_double_ref)]
		let input_copy = &mut input.clone();
		match parserl(input_copy) {
			Ok(resultl) => {
				match parserr(input_copy) {
					Ok(resultr) => {
						*input = (*input_copy).clone();
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


fn left<RL, RR, I: InputIter>(
	parserl: impl ParseFn<RL, I>,
	parserr: impl ParseFn<RR, I>
)
-> impl ParseFn<RL, I>
{
	map(pair(parserl, parserr), |(resultl, resultr)|Ok(resultl))
}


fn right<RL, RR, I: InputIter>(
	parserl: impl ParseFn<RL, I>,
	parserr: impl ParseFn<RR, I>
)
-> impl ParseFn<RR, I>
{
	map(pair(parserl, parserr), |(resultl, resultr)|Ok(resultr))
}


fn pred<R, I: InputIter>(
	parser: impl ParseFn<R, I>,
	pred_fn: impl Fn(&R) -> bool,
	err_fn: impl Fn(&R) -> ParseResult<R>
)
-> impl ParseFn<R, I> 
{
	move |input: InputRef<I>| {
		#[allow(clippy::clone_double_ref)]
		let input_copy = &mut input.clone();
		match parser(input_copy) {
			Ok(result) => {
				if pred_fn(&result) {
					*input = (*input_copy).clone();
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


fn any_until<U, I: InputIter>(
	until: impl ParseFn<U, I>
)
-> impl ParseFn<String, I>
{
	move |input: InputRef<I>| {
		let mut comment = String::new();
		let mut last_err = vec!();
		while let Err(err) = until(input)  {
			last_err = err;
			let next = next(input);
			if let Ok(next) = next {
				comment.push_str(next.as_str());
				input.advance(1);
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


fn all_until<R, U, I: InputIter>(
	parser: impl ParseFn<R, I>,
	until: impl ParseFn<U, I>
)
-> impl ParseFn<Vec<R>, I>
{
	move |input: InputRef<I>| {
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


fn literal<I: InputIter>(
	expected: &'static str
)
-> impl ParseFn<bool, I>
{
	move |input: InputRef<I>| {
		let len = expected.graphemes(true).count();
		let next = input.source.by_ref().take(len).join("");
		let dx = String::from("   ").repeat(input.context.indent-1);
		debug!("{}⮬ matching found literal {:?} with expected {:?}", dx, next, expected);
		if next.as_str() == expected {
			debug!("{}  found, index {}", dx, input.context.indent-1);
			input.advance(len);
			debug!("{}  index now {} after advancing {}", dx, input.context.indent-1, len);
			Ok(true)
		}
		else {
			Err(vec!(ExpectedCharacter{expected: expected.into(), found: next}))
		}
	}
}


fn debug<R, I: InputIter>(
	message: &'static str, parser: impl ParseFn<R, I>
) 
-> impl ParseFn<R, I>
{
	move |input: InputRef<I>| {
		debug!("{}{}", String::from("   ").repeat(input.context.indent-1), message);
		parser(input)
	}
}

fn peek<R, I: InputIter>(
	parser: impl ParseFn<R, I>,
)
-> impl ParseFn<R, I>
{
	move |input: InputRef<I>| {
		#[allow(clippy::clone_double_ref)]
		let input_copy = &mut input.clone();
		parser(input_copy)
	}
}


// ---


fn newline<I: InputIter>(input: InputRef<I>) -> ParseResult<()>
{
	discard(right(
		zero_or_more(tab), 
		pred(
			next, 
			|r|is_newline(&r.as_str()), 
			|r|Err(vec!(ExpectedCharacter{expected: "<newline>".into(), found: r.into()}))
		)))(input)
}


fn next<I: InputIter>(input: InputRef<I>) -> ParseResult<OwnedToken>
{
	let result = input.source.next();
	if let Some(result) = result {
		input.advance(1);
		Ok(result.into())
	}
	else {
		Err(vec!(UnexpectedEOF{line: 0, position: 0}))
	}
}


fn space<I: InputIter>(input: InputRef<I>) -> ParseResult<()>
{
	discard(literal(" "))(input)
}

fn tab<I: InputIter>(input: InputRef<I>) -> ParseResult<()>
{
	discard(literal("\t"))(input)
}

fn eof<I: InputIter>(input: InputRef<I>) -> ParseResult<()>
{
	if input.source.by_ref().peekable().peek().is_some() {
		Ok(())
	}
	else {
		Err(vec!(ExpectedCharacter{expected: "<eof>".into(), found: format!("{:?}", input.source.by_ref().peekable().peek()) }))
	}
}

fn indent<I: InputIter>(input: InputRef<I>) -> ParseResult<()>
{
	if input.context.indent > 1 {
		let iter = input.source.by_ref();
		for i in 1..input.context.indent {
			if let Some(next) = iter.next() {
				if next.as_ref() != "\t" {
					return Err(vec!(ExpectedCharacter{expected: "\t".into(), found: format!("{}", next)}));
				}
			}
			else {
				return Err(vec!(UnexpectedEOF{line:0,position:0}));
			}
		}
		input.advance(input.context.indent - 1);
	}
	Ok(())
}


fn identifier<I: InputIter>(input: InputRef<I>) -> ParseResult<String>
{
	let iter = input.source.clone();
	let mut id = String::new();
	let mut result = Err(vec!());
	for next in iter {
		if is_invalid(next.as_ref()) {
			result = Err(vec!(UnexpectedCharacter{found: format!("{}", next)}));
			break;
		}
		if is_special(next.as_ref()) {
			result = Err(vec!(UnexpectedCharacter{found: format!("{}", next)}));
			break;
		}
		if is_newline(next.as_ref()) {
			result = Err(vec!(UnexpectedCharacter{found: format!("{}", next)}));
			break;
		}
		id.push_str(next.as_ref());
	}
	if !id.is_empty() {
		input.advance(id.len());
		result = Ok(id);
	}
	result
}



#[derive(Debug, Clone)]
enum SyntaxElement {
	Expression(Expression),
	Comment(Comment),
	Invalid
}


type Comment = String;
fn comment<I: InputIter>(input: InputRef<I>) -> ParseResult<SyntaxElement>
{
	map(right(literal("#"), any_until(newline)), |result|Ok(SyntaxElement::Comment(result)))(input)
}

#[derive(Debug, Clone)]
struct Expression {
	identifiers: Vec<String>,
	block: Block
}
fn expression<I: InputIter>(input: InputRef<I>) -> ParseResult<SyntaxElement>
{
	let i = input.context.indent-1;
	map_err(map(
		left(
			pair(one_or_more(left(identifier, zero_or_more(space))), block), 
			peek(any_of(&[&newline, &eof]))
		), 
		|(identifiers, block)| {
			Ok(SyntaxElement::Expression(Expression {
				identifiers,
				block
			}))
		}
	), move |err| {
		error!("{}❗ expression error {:?}", String::from("   ").repeat(i).to_owned(), err);
		Ok(SyntaxElement::Invalid)
	})(input)
}



#[derive(Debug, Clone)]
enum Block {
	Valid(Vec<SyntaxElement>),
	Malformed(Vec<ParseError>),
	None
}
fn block<I: InputIter>(input: InputRef<I>) -> ParseResult<Block>
{
	let debug_indent = String::from("   ").repeat(input.context.indent);

	input.context.indent += 1;
	debug!("{}⇲ NEW BLOCK with indent {}", debug_indent, input.context.indent);

	let result = zero_or_more(map(
		map_err(
			right(
				debug("⬤ newline", zero_or_more(newline)),
				right(indent, any_of(&[
					&debug("⬤ expression!", expression),
					&debug("⬤ comment!", comment),
				]))
			),
			|err|{
				error!("{}❗ block error {:?}", debug_indent, err);
				Err(err)
			}
		),
		|result|{
			Ok(result)
		}
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
	let input = &mut Input::from(source.into_iter());

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


// fn dedent(input: InputRef<I>) -> ParseResult<()>
// {
// 	let mut counter = 1;
// 	let mut iter = input.source;
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
