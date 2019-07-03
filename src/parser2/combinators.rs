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
pub fn tab(input: InputRef<impl InputIter>) -> Output<impl InputIterItem> {
	let x = input.next()?;
	if is_tab(x.as_ref()) {
		return Ok(x);
	}
	Error(UnexpectedCharacter{
		context: input.context,
		expected: format!("<tab>"),
		found: format!("{}", x)
	})
}
pub fn space(input: InputRef<impl InputIter>) -> Output<impl InputIterItem> {
	let x = input.next()?;
	if is_space(&x) {
		return Ok(x);
	}
	Error(UnexpectedCharacter{
		context: input.context,
		expected: format!("<space>"),
		found: format!("{}", x)
	})
}

pub fn any_char(input: InputRef<impl InputIter>) -> Output<impl InputIterItem> {
	let x = input.next()?;
	Ok(x)
}
pub fn any_regular_char(input: InputRef<impl InputIter>) -> Output<impl InputIterItem> {
	let x = input.next()?;
	if is_regular(x.as_ref()) {
		Ok(x)
	}
	else { Error(UnexpectedCharacter{
		context: input.context,
		expected: format!("any regular character"),
		found: format!("{}", x)
	})}
}
pub fn any_special_char(input: InputRef<impl InputIter>) -> Output<impl InputIterItem> {
	let x = input.next()?;
	if is_special(x.as_ref()) {
		Ok(x)
	}
	else { Error(UnexpectedCharacter{
		context: input.context,
		expected: format!("any special character"),
		found: format!("{}", x)
	})}
}

pub fn wrap<I: InputIter, R>(parser: impl ParseFn<I, R>) -> impl ParseFn<I, R> {
	move |input: InputRef<I>| -> Output<R> { parser(input) }
}

pub fn discard<I: InputIter, R>(parser: impl ParseFn<I, R>) -> impl ParseFn<I, ()> {
	move |input: InputRef<I>| -> Output<()> { parser(input).discard_value() }
}

pub fn map<I: InputIter, R, M>(parser: impl ParseFn<I, R>, mapper: impl Fn(R) -> M) -> impl ParseFn<I, M> {
	move |input: InputRef<I>| {
		match parser(input) {
			Ok(next) => Ok(mapper(next)),
			Partial{result, error}
				=> Partial {
					result: mapper(result),
					error
				},
			Error(error) => Error(error),
			Critical(error) => Critical(error)
		}
	}
}

pub fn any_of<'of, I: InputIter, R>(of: &'of [&dyn ParseFn<I, R>]) -> impl ParseFn<I, R> + 'of {
	move |input: InputRef<I>| -> Output<R> {
		let input_orig = input.clone();
		for option in of {
			match option(input) {
				Ok(next) => return Ok(next),
				Partial{..} |
				Error(_) |
				Critical(_) => {
					input = &mut input_orig.clone();
					continue
				}
			}
		}
		input = &mut input_orig.clone();
		Error(NoneMatched)
	}
}

pub fn either<I: InputIter, R: From<L>, L: From<R>, LL, RR>(l: LL, r: RR) -> impl ParseFn<I, R>
where LL: ParseFn<I, L>, RR: ParseFn<I, R>{
	move |input: InputRef<I>| {
		let input_orig = input.clone();
		let resultl = l(input);
		if let Ok(_) = resultl { return resultl.map_value(); };
		input = &mut input_orig.clone();
		let resultr = r(input);
		match resultr {
			Ok(_) => resultr.map_value(),
			Partial{..} => {
				if let Partial{..} = resultl {
					return resultl.map_value();
				}
				resultr.map_value()
			}
			_ => {
				input = &mut input_orig.clone();
				resultr.map_value()
			}
		}
	}
}

pub fn all_until<I: InputIter, R, T>(parser: impl ParseFn<I, R>, until: impl ParseFn<I, T>) -> impl ParseFn<I, Vec<R>> {
	move |input: InputRef<I>| -> Output<Vec<R>> {
		let mut result = Vec::new();
		loop {
			match parser(input) {
				Ok(next) => {
					result.push(next);
				},
				Partial{result: next, error} => {
					result.push(next);
					return Partial{
						result, error
					}
				},
				Error(error) => {
					return Partial{
						result, error
					}
				},
				Critical(error) => return Critical(error)
			}
			match until(input) {
				Ok(_) => break,
				Error(_) => continue,
				Partial{..} => continue,
				Critical(x) => return Critical(x)
			}
		}
		Ok(result)
	}
}

pub fn zero_or_more<I: InputIter, R>(parser: impl ParseFn<I, R>) -> impl ParseFn<I, Vec<R>> {
	move |input: InputRef<I>| {
		let mut result = Vec::new();
		while let Ok(next) = parser(input) {
			result.push(next);
		}
		Ok(result)
	}
}

pub fn multiple<I: InputIter, R>(times: usize, parser: impl ParseFn<I, R>) -> impl ParseFn<I, Vec<R>> {
	move |input: InputRef<I>| -> Output<Vec<R>> {
		let mut result = Vec::new();
		for _ in 0..times {
			match parser(input) {
				Ok(next) => {
					result.push(next);
				},
				Partial{result: next, error} => {
					result.push(next);
					return Partial{
						result, error
					}
				},
				Error(error) => {
					return Partial{
						result, error
					}
				},
				Critical(error) => return Critical(error)
			}
		}
		Ok(result)
	}
}



pub fn all_chars<I>(input: InputRef<impl InputIter>) -> Output<Vec<impl InputIterItem>> {
	all_until(either(space, any_regular_char), eof)(input)
}
