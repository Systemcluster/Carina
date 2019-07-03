///
/// Carina Programming Language Parser
///

use std::fmt::{Display, Debug};
use std::iter::{Iterator, Peekable};
use derive_more::*;
use unicode_segmentation::UnicodeSegmentation;

mod types;
pub use types::*;

mod combinators;
pub use combinators::*;

mod values;
pub use values::*;


#[derive(Debug, Display)]
pub struct SuccessInfo {
	pub message: String
}
pub fn parse(input: std::path::PathBuf) -> Result<SuccessInfo, std::io::Error> {
	let source = std::fs::read_to_string(input)?;
	let source = source.as_str().graphemes(true).collect::<Vec<_>>();
	let input = &mut Input{iter: (source.into_iter()), context: InputContext {
		indent: 0,
		line: 0,
		position: 0
	} };
	let result = all_chars(input);
	let result = SuccessInfo{message: format!("{:?}", result)};
	Ok(result)
}
