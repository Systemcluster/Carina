use std::fmt::{Display, Debug};
use std::iter::{Iterator, Peekable};
use std::ops::{Try};
use std::option::{NoneError};

use itertools::*;
use derive_more::*;

use super::values::*;


pub trait InputIterItem = Debug + Display + Clone + AsRef<str> + PartialEq + Eq;
pub trait InputIter<T: InputIterItem> = Debug + Clone + Iterator::<Item = T>;


#[derive(Debug, Clone)]
pub enum Error {
	Unspecified,
	UnexpectedEOF {
		context: InputContext,
		expected: String
	},
	UnexpectedCharacter {
		context: InputContext,
		expected: String,
		found: String
	},
	NoneMatched,

}


#[derive(Debug, Clone)]
pub enum Output<R> {
	Ok(R),
	Partial{result: R, error: Error},
	Error(Error),
	Critical(Error)
}
impl<R> Output<R> {
	pub fn map_value<T: From<R>>(self) -> Output<T> {
		match self {
			Self::Ok(x) => Output::Ok(x.into()),
			Self::Partial{result, error}
				=> Output::Partial {
					result: result.into(),
					error
				},
			Self::Error(x) => Output::Error(x),
			Self::Critical(x) => Output::Critical(x)
		}
	}
	pub fn discard_value(self) -> Output<()> {
		match self {
			Self::Ok(x) => Output::Ok(()),
			Self::Partial{result, error}
				=> Output::Partial {
					result: (),
					error
				},
			Self::Error(x) => Output::Error(x),
			Self::Critical(x) => Output::Critical(x)
		}
	}
}
impl<R> Try for Output<R> {
	type Ok = R;
	type Error = Self;
    fn into_result(self) -> Result<R, Output<R>> {
        match self {
			Self::Ok(r) => Result::Ok(r),
			_ => Result::Err(self)
		}
    }
    fn from_ok(v: R) -> Self {
		Self::Ok(v)
    }
    fn from_error(v: Self) -> Self {
        v
    }
}
impl<R> From<NoneError> for Output<R> {
	fn from(_: NoneError) -> Self {
		Self::Critical(Error::Unspecified)
	}
}
// impl<T> From<Output<T>> for Output<()> where  {
// }
// impl<F, R> From<Output<F>> for Output<R> {
// }

#[derive(Debug, Clone, Copy)]
pub struct InputContext
{
	pub indent: usize,
	pub line: usize,
	pub position: usize
}

#[derive(Debug, Clone)]
pub struct Input<I: InputIter>
{
	pub iter: I,
	pub context: InputContext
}
impl<I: InputIter> Input<I> {
	pub fn next(&mut self) -> Output<I::Item> {
		match self.iter.next() {
			None => {
				Output::Critical(Error::UnexpectedEOF{
					context: self.context,
					expected: format!("1 more character")
				})
			},
			Some(x) if is_newline(x.as_ref()) => {
				self.context.line += 1;
				self.context.position = 0;
				Output::Ok(x)
			}
			Some(x) => {
				self.context.position += 1;
				Output::Ok(x)
			}
		}
	}
	pub fn advance(&mut self, by: usize) -> Output<String> {
		let mut result = String::new();
		for i in 0..by {
			match self.iter.next() {
				None => {
					return Output::Critical(Error::UnexpectedEOF{
						context: self.context,
						expected: format!("{} more characters", by - i)
					})
				},
				Some(x) if is_newline(x.as_ref()) => {
					self.context.line += 1;
					self.context.position = 0;
					result.push_str(x.as_ref());
				}
				Some(x) => {
					self.context.position += 1;
					result.push_str(x.as_ref());
				}
			}
		}
		Output::Ok(result)
	}
}

pub type InputRef<'a, I> = &'a mut Input<I>;

pub trait ParseFn<I: InputIter, R> = Fn(InputRef<I>) -> Output<R>;
