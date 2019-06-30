use std::fmt::{Display, Debug};
use std::iter::{Iterator, Peekable};

use super::types::*;
use super::types::Output::*;
use super::types::Error::*;
use super::values::*;


pub fn eof(input: InputRef<impl InputIter>) -> Output<()> {
	match input.clone().next() {
		Ok(x) => Error(UnexpectedCharacter{
			context: input.context,
			expected: format!("<eof>"),
			found: format!("{}", x)
		}),
		_ => Ok(()),
	}
}
pub fn tab(input: InputRef<impl InputIter>) -> Output<&str> {
	let x = input.next()?;
	if is_tab(x) {
		return Ok(x);
	}
	Error(UnexpectedCharacter{
		context: input.context,
		expected: format!("<tab>"),
		found: format!("{}", x)
	})
}
pub fn space(input: InputRef<impl InputIter>) -> Output<&str> {
	let x = input.next()?;
	if is_space(x) {
		return Ok(x);
	}
	Error(UnexpectedCharacter{
		context: input.context,
		expected: format!("<space>"),
		found: format!("{}", x)
	})
}

pub fn any_char(input: InputRef<impl InputIter>) -> Output<&str> {
	let x = input.next()?;
	Ok(x)
}
pub fn any_regular_char(input: InputRef<impl InputIter>) -> Output<&str> {
	let x = input.next()?;
	if is_regular(x.as_ref()) {
		Ok(x)
	}
	else { Error(UnexpectedCharacter{
		context: input.context,
		expected: format!("any valid character"),
		found: x.into()
	})}		
}
pub fn any_special_char(input: InputRef<impl InputIter>) -> Output<&str> {
	let x = input.next()?;
	if is_special(x.as_ref()) {
		Ok(x)
	}
	else { Error(UnexpectedCharacter{
		context: input.context,
		expected: format!("any special character"),
		found: x.into()
	})}		
}

pub fn discard<I: InputIter + 'a, R>(parser: impl ParseFn<'a, I, R>) -> impl ParseFn<'b, I, ()> {
	move |input: InputRef<I>| -> Output<()> { parser(input).discard() }
}

pub fn all_until<I: InputIter + 'a + 'b + 'c, R, T>(parser: impl ParseFn<'a, I, R>, until: impl ParseFn<'b, I, T>) -> impl ParseFn<'c, I, Vec<R>> {
	move |input: InputRef<I>| -> Output<Vec<R>> {
		let mut result = Vec::new();
		loop {
			match parser(input) {
				Ok(next) => {
					result.push(next);
				},
				Error(err) => return Error(err),
				Critical(err) => return Critical(err)
			}
			match until(input) {
				Ok(_) => break,
				Error(_) => continue,
				Critical(x) => return Critical(x)
			}
		}
		Ok(result)
	}
}

pub fn zero_or_more<I: InputIter + 'a + 'b, R>(parser: impl ParseFn<'a, I, R>) -> impl ParseFn<'b, I, Vec<R>> {
	move |input: InputRef<I>| {
		let mut result = Vec::new();
		while let Ok(next) = parser(input) {
			result.push(next);
		}
		Ok(result)
	}
}

pub fn multiple<I: InputIter + 'a + 'b, R>(times: usize, parser: impl ParseFn<'a, I, R>) -> impl ParseFn<'b, I, ()> {
	move |input: InputRef<I>| -> Output<()> {
		for _ in 0..times {
			// parser(input)?;
			match parser(input) {
				Ok(_) => continue,
				Error(x) => return Error(x),
				Critical(x) => return Critical(x)
			}
		}
		Ok(())
	}
}


// pub fn all_chars(input: InputRef<impl InputIter>) -> Output<Vec<String>> {
// 	all_until(any_regular_char, eof)(input)
// }
// pub fn all_chars(input: InputRef<impl InputIter>) -> Output<Vec<String>> {

pub fn test(input: InputRef<impl InputIter>) {
	let ff = discard(any_regular_char)(input);
}
