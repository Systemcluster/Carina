use std::fmt::Display;
use std::error::Error;
use derive_more::*;
use pest::{Parser};
use pest_derive;
use std::fs;



#[derive(Parser)]
#[grammar = "carina.pest"]
pub struct Carina;


#[derive(Debug, Display)]
pub enum RuntimeError {
    #[display(fmt = "Generic error: {}", "_0")]
    GenericError(String),
    ParseError(String),
    IOError(std::io::Error)
}
impl Error for RuntimeError{}
impl From<std::io::Error> for RuntimeError {
    fn from(error: std::io::Error) -> Self {
        Self::IOError(error)
    }
}
impl From<pest::error::Error<Rule>> for RuntimeError {
    fn from(error: pest::error::Error<Rule>) -> Self {
        Self::ParseError(format!("Parsing error:{}", error))
    }
}

#[derive(Debug, Display)]
pub struct SuccessInfo {
    pub message: String
}


pub fn main() -> Result<SuccessInfo, RuntimeError> {

    let source = fs::read_to_string("src/test.ca")?;
    let parser = Carina::parse(Rule::module, &source)?
        .next().ok_or_else(|| RuntimeError::ParseError(String::from("Could not create parser!")))?;

    println!("{:?}", parser);

    for line in parser.into_inner() {
        println!("{}", line);
        match line.as_rule() {
            Rule::assignment => {
            }
            Rule::expression => {
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    Ok(SuccessInfo{message: String::from("Success!")})
}
