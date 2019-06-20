use std::fmt::Display;
use std::error::Error;
use std::iter::Iterator;
use derive_more::*;
use itertools::*;
use unicode_reader::Graphemes;
use unicode_segmentation::UnicodeSegmentation;


#[derive(Debug, Display)]
pub enum RuntimeError {
	#[display(fmt = "Generic error: {}", "_0")]
	GenericError(String),
	#[display(fmt = "Parse error: {}", "_0")]
	ParseError(String),
	IOError(std::io::Error),
	#[display(fmt = "Parse error: Unexpected EOF at position {}", "position")]
	UnexpectedEOFError {
		position: usize
	},
	#[display(fmt = "Parse error: Unexpected EOF at position {}: {:x?}", "position", "character")]
	IllegalCharacterError {
		position: usize,
		character: String
	}
}
impl Error for RuntimeError{}
impl From<std::io::Error> for RuntimeError {
	fn from(error: std::io::Error) -> Self {
		Self::IOError(error)
	}
}

#[derive(Debug, Display)]
pub struct SuccessInfo {
	pub message: String
}


#[derive(Copy, Clone, Debug, Display, PartialEq, Eq)]
enum FundamentalType {
	Function
}

#[derive(Copy, Clone, Debug, Display, PartialEq, Eq)]
enum NodeType {
	Module,
	Expression,
	Fundamental
}
use NodeType::*;

#[derive(Debug, PartialEq, Eq)]
struct Node {
	state: NodeType,
	children: Vec<Node>,
	start: usize, end: usize
}
impl Node {
	fn new(state: NodeType, start: usize) -> Self { Self {
		state, children: Default::default(), start, end: start} }
}

type Token<'a> = &'a str;
type Source<'a> = Vec<Token<'a>>;

const SPACE: Token = " ";

#[derive(Debug, Default)]
struct Parser<'a> {
	source: Source<'a>,
	length: usize
}
impl<'a> Parser<'a> {
	fn new<'s: 'a>(source: Source<'s>) -> Self {
		Self { length: source.len(), source }
	}

	fn is_special(&self, token: Token) -> bool {
		match token {
			" " | ":" | "." | "," | "[" | "]" | "(" | ")" | "/" | "\\" | "\"" | "\n" | "\r\n" | "\t" => true,
			_ => false
		}
	}

	fn is_invalid(&self, token: Token) -> bool {
		for i in 0x00..0x1F {
			if std::str::from_utf8(&[i]) == Ok(token) {
				return true;
			}
		}
		false
	}

	fn next_token(&self, i: usize) -> Result<(String, usize), RuntimeError> {
		if self.is_invalid(self.source[i]) {
			return Err(RuntimeError::IllegalCharacterError{position: i, character: self.source[i].into()})
		}
		if self.is_special(self.source[i]) {

			let mut token = self.source[i..i+1].concat();

			if token == "\n" || token == "\r\n" {
				token = "\\n".into();
			}
			if token == "\t" {
				token = "\\t".into();
			}
			return Ok((token, i+1));
		}

		let until = self.source[i..].iter().enumerate().find(|(_, &x)| self.is_special(x));
		if let Some((until, _)) = until {
			let until = until + i;
			return Ok((self.source[i..until].concat(), until));
		}


		Ok((self.source[i..self.length].concat(), self.length))
	}

	fn try_parse(&self, state: NodeType, i: usize) -> Result<Node, RuntimeError> {

		let node = Node::new(state, i);
		let mut i = i;

		while i < self.source.len() {
			if self.source[i] == SPACE {
				i += 1;
				continue;
			}
			let token = self.next_token(i)?;

			info!("token: {} from {} until {}", token.0, i, token.1);
			i = token.1;
			// match state {
			//     Module => {
			//         let x = self.source[i..i+2].concat();
			//         match x.as_str() {
			//             "fn" => {
			//                 println!("found a fn!");
			//                 node.children.push(self.try_parse(Fundamental, i)?)
			//             }
			//             _ => unimplemented!()
			//         }
			//     }
			//     Fundamental => {
			//         let until =
			//         println!("checking fundamental {}", );
			//     }
			//     Expression => {

			//     }
			//     _ => unimplemented!()
			// }
		}

		Ok(node)
	}
}


pub fn main(input: std::path::PathBuf) -> Result<SuccessInfo, RuntimeError> {
	// let sourcefile = std::fs::File::open(input)?;
	// let graphemes = Graphemes::from(std::io::BufReader::new(sourcefile));
	// let source = graphemes.by_ref().map(|Ok(g)|g).collect::<Vec<_>>();
	let source = std::fs::read_to_string(input)?;
	let source = source.as_str().graphemes(true).collect::<Vec<_>>();

	let parser = Parser::new(source);
	let node = parser.try_parse(Module, 0)?;
	if node.end < parser.source.len() {
		return Err(RuntimeError::UnexpectedEOFError{position: node.end});
	}

	Ok(SuccessInfo{message: String::from("Success!")})
}
