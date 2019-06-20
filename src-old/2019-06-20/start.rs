use std::fmt::Display;
use std::error::Error;
use std::iter::Iterator;
use derive_more::*;
use itertools::*;
use unicode_reader::Graphemes;


#[derive(Debug, Display)]
pub enum RuntimeError {
	#[display(fmt = "Generic error: {}", "_0")]
	GenericError(String),
    #[display(fmt = "Parse error: {}", "_0")]
	ParseError(String),
	IOError(std::io::Error)
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


type Token<'a> = &'a str;



pub trait Node {
    type Type = Self;
    type ChildrenIter = dyn Iterator<Item = dyn Node>;
    fn new() -> Self;
    fn try_parse(self, token: Token) -> Result<Self, RuntimeError>
        where Self: std::marker::Sized;
    fn children(&self) -> Self::ChildrenIter;
}
impl<T> Copy for dyn Node<Type=T> {}

struct Module {
}
impl Node for Module {
    fn new() -> Self {
        Self {}
    }
    fn try_parse(self, token: Token) -> Result<Self, RuntimeError> {
        Ok(self)
    }
    fn children(&self) -> Self::ChildrenIter {
        let vec: Vec<impl Node> = Vec::new();
        vec.iter()
    }
}



pub fn main(input: std::path::PathBuf) -> Result<SuccessInfo, RuntimeError> {
	let sourcefile = std::fs::File::open(input)?;
    let graphemes = Graphemes::from(std::io::BufReader::new(sourcefile));
    let mut parser: Box<impl Node> = Box::from(Module::new());
    for graph in graphemes {
        let graph = graph?;
        parser = parser.try_parse(&graph)?;

    }
	Ok(SuccessInfo{message: String::from("Success!")})
}


// ---------------------------------------------------


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


#[derive(Debug, Display, PartialEq, Eq)]
enum Fundamental {

}

#[derive(Debug, Display, PartialEq, Eq)]
enum NodeType {
    None,
    Module,
    Expression
}
impl Default for NodeType {
    fn default() -> Self { Self::None }
}
use NodeType::*;

#[derive(Debug, PartialEq, Eq, Default)]
struct Node {
    node: NodeType,
    children: Vec<Node>
}

type Token<'a> = &'a str;
type Source<'a> = Vec<Token<'a>>;

#[derive(Debug, Default)]
struct Parser<'a> {
    source: Source<'a>
}
impl<'a> Parser<'a> {
    fn new<'s: 'a>(source: Source<'s>) -> Self {
        Self { source }
    }

    fn try_parse(&self, state: NodeType, i: usize) -> Result<usize, RuntimeError> {

        let mut node = Node::default();
        let mut i = 0;
        while i < self.source.len() {
            match state {
                Module => {
                    let x = self.source[i..i+2].concat();
                    match x.as_str() {
                        "fn" => {
                            println!("found a fn!");

                        }
                        _ => unimplemented!()
                    }
                }
                Expression => {

                }
                _ => unimplemented!()
            }
        }

        Ok(i)
    }
}


pub fn main(input: std::path::PathBuf) -> Result<SuccessInfo, RuntimeError> {
	// let sourcefile = std::fs::File::open(input)?;
    // let graphemes = Graphemes::from(std::io::BufReader::new(sourcefile));
    // let source = graphemes.by_ref().map(|Ok(g)|g).collect::<Vec<_>>();
    let source = std::fs::read_to_string(input)?;
    let source = UnicodeSegmentation::graphemes(source.as_str(), true).collect::<Vec<_>>();

    let mut current = Node::default();
    let mut parser = Parser::new(source);
    let end = parser.try_parse(Module, 0)?;
    if end < parser.source.len() {
        return Err(RuntimeError::UnexpectedEOFError{position: end});
    }

	Ok(SuccessInfo{message: String::from("Success!")})
}


// ---------------------------------------------------
