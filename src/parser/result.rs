use std::fmt::Display;
use derive_more::*;

#[derive(Debug, Display, Clone)]
pub enum ParseError {
	Empty,

	#[display(fmt = "Generic error: {}", "_0")]
	Generic(String),

	IOError(String),

	#[display(fmt = "Parse error: Unexpected EOF in line {} at position {}",
        "line", "position")]
	UnexpectedEOF {
        line: usize,
		position: usize
	},

	#[display(fmt = "Parse error: Unexpected character: {:x?}", "found")]
	UnexpectedCharacter {
		found: String
	},

	#[display(fmt = "Parse error: Illegal character in line {} at position {}: {:x?}",
        "line", "position", "character")]
	IllegalCharacter {
        line: usize,
		position: usize,
		character: String
	},

	#[display(fmt = "Parse error: Expected {:x?}",
        "expected")]
	ExpectedCharacter {
		expected: String,
		found: String
	},
	// #[display(fmt = "Parse error: Expected {:x?} in line {} at position {}, found {:x?}",
    //     "expected", "line", "position", "found")]
	// ExpectedCharacter {
    //     line: usize,
	// 	position: usize,
	// 	found: String,
	// 	expected: String
	// },

	#[display(fmt = "Parse error: Expected expression in line {} at position {}, found {:x?}",
        "line", "position", "found")]
	ExpectedExpression {
        line: usize,
		position: usize,
		found: String,
	}
}
impl std::error::Error for ParseError{}
impl From<std::io::Error> for ParseError {
	fn from(error: std::io::Error) -> Self {
		Self::IOError(format!("{}", error))
	}
}
impl From<std::option::NoneError> for ParseError {
	fn from(error: std::option::NoneError) -> Self {
		Self::Generic("Option not satisfied".into())
	}
}
use ParseError::*;


#[derive(Debug, Display)]
pub struct SuccessInfo {
	pub message: String
}
