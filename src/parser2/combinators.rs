use itertools::*;
use std::fmt::{Debug, Display};
use std::iter::{Iterator, Peekable};
use trace::trace;

trace::init_depth_var!();

use super::types::Output::*;
use super::types::ParseError::*;
use super::types::*;
use super::values::*;

pub fn eof<T: InputIterItem>(input: InputRef<T, impl InputIter<T>>) -> Output<()> {
	match input.clone().next() {
		Ok(x) => Error(UnexpectedCharacter {
			context: input.context,
			expected: format!("<eof>"),
			found: format!("{}", x),
		}),
		_ => Ok(()),
	}
}
// pub fn literal<T: InputIterItem>(input: InputRef<T, impl InputIter<T>>) -> Output<T> {
// 	// TODO
// }
pub fn tab<T: InputIterItem>(input: InputRef<T, impl InputIter<T>>) -> Output<T> {
	let x = input.clone().next()?;
	if is_tab(&x) {
		input.next()?;
		return Ok(x);
	}
	Error(UnexpectedCharacter {
		context: input.context,
		expected: format!("<tab>"),
		found: format!("{}", x),
	})
}
pub fn space<T: InputIterItem>(input: InputRef<T, impl InputIter<T>>) -> Output<T> {
	let x = input.clone().next()?;
	if is_space(&x) {
		input.next()?;
		return Ok(x);
	}
	Error(UnexpectedCharacter {
		context: input.context,
		expected: format!("<space>"),
		found: format!("{}", x),
	})
}
pub fn newline<T: InputIterItem>(input: InputRef<T, impl InputIter<T>>) -> Output<T> {
	let x = input.clone().next()?;
	if is_newline(&x) {
		input.next()?;
		return Ok(x);
	}
	Error(UnexpectedCharacter {
		context: input.context,
		expected: format!("<space>"),
		found: format!("{}", x),
	})
}

pub fn any_char<T: InputIterItem>(input: InputRef<T, impl InputIter<T>>) -> Output<T> {
	let x = input.next()?;
	Ok(x)
}
pub fn any_regular_char<T: InputIterItem>(input: InputRef<T, impl InputIter<T>>) -> Output<T> {
	let x = input.clone().next()?;
	if is_regular(&x) {
		input.next()?;
		return Ok(x);
	}
	Error(UnexpectedCharacter {
		context: input.context,
		expected: format!("any regular character"),
		found: format!("{}", x),
	})
}
pub fn any_special_char<T: InputIterItem>(input: InputRef<T, impl InputIter<T>>) -> Output<T> {
	let x = input.clone().next()?;
	if is_special(&x) {
		input.next()?;
		return Ok(x);
	}
	Error(UnexpectedCharacter {
		context: input.context,
		expected: format!("any special character"),
		found: format!("{}", x),
	})
}

pub fn wrap<T: InputIterItem, I: InputIter<T>, R>(
	parser: impl ParseFn<T, I, R>,
) -> impl ParseFn<T, I, R> {
	move |input: InputRef<T, I>| -> Output<R> { parser(input) }
}

pub fn discard<T: InputIterItem, I: InputIter<T>, R>(
	parser: impl ParseFn<T, I, R>,
) -> impl ParseFn<T, I, ()> {
	move |input: InputRef<T, I>| -> Output<()> { parser(input).discard_value() }
}

pub fn map<T: InputIterItem, I: InputIter<T>, R, M>(
	parser: impl ParseFn<T, I, R>,
	mapper: impl Fn(R) -> M,
) -> impl ParseFn<T, I, M> {
	move |input: InputRef<T, I>| match parser(input) {
		Ok(next) => Ok(mapper(next)),
		Partial { result, error } => Partial {
			result: mapper(result),
			error,
		},
		Error(error) => Error(error),
		Critical(error) => Critical(error),
	}
}

pub fn any_of<'of, T: InputIterItem, I: InputIter<T>, R>(
	of: &'of [&dyn ParseFn<T, I, R>],
) -> impl ParseFn<T, I, R> + 'of {
	move |input: InputRef<T, I>| -> Output<R> {
		let input_orig = input.clone();
		for option in of {
			match option(input) {
				Ok(next) => return Ok(next),
				Partial { .. } | Error(_) | Critical(_) => {
					(*input) = input_orig.clone();
					continue;
				}
			}
		}
		(*input) = input_orig;
		Error(NoneMatched)
	}
}

pub fn either<T: InputIterItem, I: InputIter<T>, R: From<L>, L: From<R>>(
	l: impl ParseFn<T, I, L>,
	r: impl ParseFn<T, I, R>,
) -> impl ParseFn<T, I, R> {
	move |input: InputRef<T, I>| {
		let input_orig = input.clone();
		let resultl = l(input);
		if let Ok(_) = resultl {
			return resultl.map_value();
		};
		(*input) = input_orig.clone();
		let resultr = r(input);
		match resultr {
			Ok(_) => resultr.map_value(),
			Partial { .. } => {
				if let Partial { .. } = resultl {
					return resultl.map_value();
				}
				resultr.map_value()
			}
			_ => {
				(*input) = input_orig;
				resultr.map_value()
			}
		}
	}
}

pub fn all_until<T: InputIterItem, I: InputIter<T>, R, U>(
	parser: impl ParseFn<T, I, R>,
	until: impl ParseFn<T, I, U>,
) -> impl ParseFn<T, I, Vec<R>> {
	move |input: InputRef<T, I>| -> Output<Vec<R>> {
		let mut result = Vec::new();
		loop {
			match until(input) {
				Ok(_) => {
					break;
				}
				Error(_) => {}
				Partial { .. } => {}
				Critical(x) => return Critical(x),
			}
			match parser(input) {
				Ok(next) => {
					result.push(next);
				}
				Partial {
					result: next,
					error,
				} => {
					result.push(next);
					return Partial { result, error };
				}
				Error(error) => return Partial { result, error },
				Critical(error) => return Critical(error),
			}
		}
		Ok(result)
	}
}

pub fn zero_or_more<T: InputIterItem, I: InputIter<T>, R>(
	parser: impl ParseFn<T, I, R>,
) -> impl ParseFn<T, I, Vec<R>> {
	move |input: InputRef<T, I>| {
		let mut result = Vec::new();
		while let Ok(next) = parser(input) {
			result.push(next);
		}
		Ok(result)
	}
}

pub fn one_or_more<T: InputIterItem, I: InputIter<T>, R>(
	parser: impl ParseFn<T, I, R>,
) -> impl ParseFn<T, I, Vec<R>> {
	move |input: InputRef<T, I>| {
		let mut result = Vec::new();
		let next = parser(input);
		if let Ok(next) = next {
			result.push(next);
		} else {
			return Error(NoneMatched);
		}
		let mut more = zero_or_more(&parser)(input)?;
		result.append(&mut more);
		Ok(result)
	}
}

pub fn multiple<T: InputIterItem, I: InputIter<T>, R>(
	times: usize,
	parser: impl ParseFn<T, I, R>,
) -> impl ParseFn<T, I, Vec<R>> {
	move |input: InputRef<T, I>| -> Output<Vec<R>> {
		let input_orig = input.clone();
		let mut result = Vec::new();
		for _ in 0..times {
			match parser(input) {
				Ok(next) => {
					result.push(next);
				}
				Partial {
					result: next,
					error,
				} => {
					result.push(next);
					(*input) = input_orig;
					return Partial { result, error };
				}
				Error(error) => {
					(*input) = input_orig;
					return Partial { result, error };
				}
				Critical(error) => {
					(*input) = input_orig;
					return Critical(error);
				}
			}
		}
		Ok(result)
	}
}

pub fn wrap_ok<T: InputIterItem, I: InputIter<T>, R>(
	parser: impl ParseFn<T, I, R>,
) -> impl ParseFn<T, I, Output<R>> {
	move |input: InputRef<T, I>| -> Output<Output<R>> { Ok(parser(input)) }
}

pub fn all_chars<T: InputIterItem>(input: InputRef<T, impl InputIter<T>>) -> Output<Vec<T>> {
	all_until(any_of(&[&space, &any_regular_char, &newline]), eof)(input)
}

#[derive(Debug, Clone)]
pub enum Block {
	Block(Vec<Expression>),
	None,
}

#[derive(Debug, Clone)]
pub struct Expression {
	identifiers: Vec<String>,
	block: Block,
}

pub fn dedent<T: InputIterItem>(input: InputRef<T, impl InputIter<T>>) -> Output<()> {
	let input_orig = input.clone();
	let result = multiple(input.context.indent, tab)(input);
	(*input) = input_orig;
	match result {
		Ok(_) => Error(NoneMatched),
		_ => Ok(()),
	}
}

#[trace]
pub fn expression<T: InputIterItem>(input: InputRef<T, impl InputIter<T>>) -> Output<Expression> {
	multiple(input.context.indent, tab)(input);
	let result = all_until(
		all_until(any_regular_char, one_or_more(any_of(&[&space]))),
		any_of(&[&discard(newline), &eof]),
	)(input);
	let identifiers = match result {
		Ok(i) => i,
		Partial { result, .. } => result,
		_ => return Error(NoneMatched),
	};
	one_or_more(newline)(input);
	input.context.indent += 1;
	let exprsblock = block(input);
	input.context.indent -= 1;
	Ok(Expression {
		identifiers: identifiers.iter().map(|i| i.iter().join("")).collect::<_>(),
		block: exprsblock,
	})
}

#[trace]
pub fn block<T: InputIterItem>(input: InputRef<T, impl InputIter<T>>) -> Block {
	match multiple(input.context.indent, tab)(&mut input.clone()) {
		Ok(_) => {}
		_ => return Block::None,
	}
	let exprs = all_until(expression, any_of(&[&eof, &dedent]))(input);
	match exprs {
		Ok(exprs) if exprs.len() > 0 => Block::Block(exprs),
		_ => Block::None,
	}
}
