use derive_more::*;
///
/// Carina Programming Language Parser
///
use std::fmt::{Debug, Display};
use std::iter::{Iterator, Peekable};
use unicode_segmentation::UnicodeSegmentation;

mod types;
pub use types::*;

mod combinators;
pub use combinators::*;

mod values;
pub use values::*;

#[derive(Debug, Display)]
pub struct SuccessInfo {
	pub message: String,
}
pub fn parse(input: std::path::PathBuf) -> Result<SuccessInfo, std::io::Error> {
	let source = std::fs::read_to_string(input)?;
	let source = source.as_str().graphemes(true).collect::<Vec<_>>();
	let input = &mut Input::new(source.into_iter());
	let result = block(input);
	let result = SuccessInfo {
		message: format!("{:#?}", result),
	};
	Ok(result)
}
