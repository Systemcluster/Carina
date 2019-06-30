use std::fmt::Display;
use std::error::Error;
use std::iter::Iterator;
use std::collections::*;
use derive_more::*;
use itertools::*;
use unicode_reader::Graphemes;
use unicode_segmentation::UnicodeSegmentation;


#[derive(Debug, Display)]
pub enum RuntimeError {
	#[display(fmt = "Generic error: {}", "_0")]
	Generic(String),

	IOError(std::io::Error),

	#[display(fmt = "Parse error: Unexpected EOF in line {} at position {}",
        "line", "position")]
	UnexpectedEOF {
        line: usize,
		position: usize
	},

	#[display(fmt = "Parse error: Illegal character in line {} at position {}: {:x?}",
        "line", "position", "character")]
	IllegalCharacter {
        line: usize,
		position: usize,
		character: String
	},

	#[display(fmt = "Parse error: Expected {:x?} in line {} at position {}, found {:x?}",
        "expected", "line", "position", "found")]
	ExpectedCharacter {
        line: usize,
		position: usize,
		found: String,
		expected: String
	},

	#[display(fmt = "Parse error: Expected expression in line {} at position {}, found {:x?}",
        "line", "position", "found")]
	ExpectedExpression {
        line: usize,
		position: usize,
		found: String,
	}
}
impl Error for RuntimeError{}
impl From<std::io::Error> for RuntimeError {
	fn from(error: std::io::Error) -> Self {
		Self::IOError(error)
	}
}
impl From<std::option::NoneError> for RuntimeError {
	fn from(error: std::option::NoneError) -> Self {
		Self::Generic("Option not satisfied".into())
	}
}
use RuntimeError::*;


#[derive(Debug, Display)]
pub struct SuccessInfo {
	pub message: String
}


#[derive(Clone, Debug, PartialEq, Eq)]
struct FundamentalType {
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct ExpressionType {
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct BindingType {
	name: String,
	expression: Box<ExpressionType>
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ModuleType {
	name: String,
	children: HashMap<String, BindingType>
}


#[derive(Clone, Debug, PartialEq, Eq)]
enum Class {
	Module(ModuleType),
	Binding(BindingType),
	Expression(ExpressionType),
	Fundamental(FundamentalType)
}
use Class::*;


type Token<'a> = &'a str;
type Source<'a> = Vec<Token<'a>>;

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParserState<'a> {
	parser: &'a Parser<'a>,
    i: usize,
    l: usize,
    p: usize,
}
impl<'a> ParserState<'a> {
	fn advance(mut self, by: usize) -> Self {
		self.i += by;
		self
	}
	fn withpos(mut self, pos: usize) -> Self {
		self.i = pos;
		self
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TokenType {
	Identifier(String),
	Colon,

}
use TokenType::*;

#[derive(Debug, Default, PartialEq, Eq)]
struct Parser<'a> {
	source: Source<'a>,
	length: usize,
    // state: ParserState,
}

const SPACE: Token = " ";
const COLON: Token = ":";

impl<'a> Parser<'a> {
	fn new<'s: 'a>(source: Source<'s>) -> Self {
		Self {
            length: source.len(),
            source
        }
	}

	fn is_special(&self, token: Token) -> bool {
		// special tokens break up identifiers
		[
			" ", "#", ":", ".", ",",  "[", "]", "{", "}",
			"(", ")", "/", "\\", "\"", "\n", "\r\n", "\t"
		].contains(&token)
	}

	fn is_invalid(&self, token: Token) -> bool {
        // exclude non-printable control characters
		for i in 0x00..0x1F {
            // but allow \t, \n, \r
			if i != 0x09 && i != 0x0A && i != 0x0D
            && std::str::from_utf8(&[i]) == Ok(token) {
				return true;
			}
		}
		false
	}

	fn next_token(&'a self, state: ParserState<'a>) -> Result<(String, ParserState), RuntimeError> {
		let mut state = state;
		while state.i < self.source.len() && self.source[state.i] == SPACE {
			state = state.advance(1);
		}
		if self.is_invalid(self.source[state.i]) {
			return Err(IllegalCharacter{
                line: state.l,
                position: state.p,
                character: self.source[state.i].into()
            })
		}
		if self.is_special(self.source[state.i]) {
			return Ok((self.source[state.i ..= state.i].join(""), state.advance(1)));
		}
		let until = self.source[state.i..].iter().enumerate().find(|(_, &x)| self.is_special(x));
		if let Some((until, _)) = until {
			let until = until + state.i;
			return Ok((self.source[state.i .. until].join(""), state.withpos(until)));
		}
		Ok((self.source[state.i..self.length].join(""), state.withpos(self.length)))
	}


	fn try_parse_binding(&self, class: &Class, state: ParserState<'a>) -> Result<(BindingType, ParserState), RuntimeError> {
		#[derive(Debug, Copy, Clone, PartialEq, Eq)]
		enum PState {
			Name, Colon1, Type, Colon2, Expression
		};
		let mut state = state;
		let mut pstate = PState::Name;
		let mut name = "".into();
		let mut expression: Option<Box<ExpressionType>> = None;
		let mut explicittype: String;
		let mut lasttoken = "".into();
		loop {
			println!(" pstate {:?} state {:?}", pstate, state);
			let (token, newstate) = self.next_token(state)?;
			lasttoken = token.clone();
			state = newstate;
			match (pstate, token.as_str()) {
				(PState::Name, _) => {
					name = token;
					pstate = PState::Colon1;
				}
				(PState::Colon1, COLON) => {
					pstate = PState::Type;
				}
				(PState::Type, COLON) => {
					explicittype = token;
					pstate = PState::Expression;
				}
				(PState::Colon2, COLON) => {
					pstate = PState::Expression;
				}
				(PState::Expression, _) => {
					return Ok((BindingType{
						name, expression: expression.ok_or(ExpectedExpression{line: state.l, position: state.p, found: lasttoken})?
					}, state));
				}
				_ => {
					break;
				}
			}
		}
		Err(ExpectedCharacter{line: state.l, position: state.p, found: lasttoken, expected: COLON.into()})
		// let (name, state) = self.next_token(state)?;
		// let (colon, state) = self.next_token(state)?;
		// if colon != COLON {
		// 	return Err(ExpectedCharacter{line: state.l, position: state.p, found: colon, expected: COLON});
		// }
		// Ok((BindingType{name, expression: box ExpressionType{}}, state))
	}
	fn try_parse(&self, class: Class, state: ParserState<'a>) -> Result<ParserState, RuntimeError> {

        let mut state = state;
        let mut class = class;

		let mut stack: Vec<Class>;

		while state.i < self.source.len() {
			if self.source[state.i] == SPACE {
				state.i += 1;
				continue;
			}
			let (binding, newstate) = self.try_parse_binding(&class, state)?;
			state = newstate;
			// debug!("token: {:x?} from {} until {}", token, state.i, sourcepart.1);
			// let sourcepart = self.next_token(state)?;
            // let token = sourcepart.0.join("");
			// state.i = sourcepart.1;

			// match class {
			//     Module(ref mut module) => {
			// 		let (binding, newstate) = self.try_parse_binding(class, state)?;
			// 		module.children.insert(binding.name, binding);
			// 		state = newstate;
			//     }
			//     Binding(_) => {

			//     }
			//     Expression(_) => {

			//     }
			//     _ => unimplemented!()
			// }
		}

		Ok(state)
	}
}


pub fn main(input: std::path::PathBuf) -> Result<SuccessInfo, RuntimeError> {
	// let sourcefile = std::fs::File::open(input)?;
	// let graphemes = Graphemes::from(std::io::BufReader::new(sourcefile));
	// let source = graphemes.by_ref().map(|Ok(g)|g).collect::<Vec<_>>();
	let name = input.file_stem()?.to_owned().to_str()?.into();
	let source = std::fs::read_to_string(input)?;
	let source = source.as_str().graphemes(true).collect::<Vec<_>>();

	let parser = Parser::new(source);
	let state = parser.try_parse(Module(ModuleType{
			name,
			children: Default::default()
		}), ParserState{ parser: &parser, i: 0, l: 0, p: 0 });
    let state = state?;
	if state.i < parser.source.len() {
		return Err(UnexpectedEOF{line: state.l, position: state.p});
	}

	Ok(SuccessInfo{message: String::from("Success!")})
}
