const tab: char = 9 as char;
// const blockstart: &str = "\x1B\x12";
// const blockend: &str = "\x1B\x14";
const blockstart: &str = "{";
const blockend: &str = "}";

use std::string::String;
pub fn preprocess_source(src: &str) -> String {
	let mut res = String::with_capacity((src.len() as f64 * 1.5f64) as usize);
	let mut prev_indent  = 0 as usize;
	let mut outstanding_indents = 0 as usize;
	for ref line in src.lines() {
		let mut indent = 0 as usize;
		if line.len() != 0 {
			for ref cur in line.chars() {
				if *cur == tab {indent += 1}
				else {break}
			}
			// debug!("line with {} indents (prev {}, outstanding {}): {}", indent, prev_indent, outstanding_indents, line);
			if indent != prev_indent {
				if indent == prev_indent + 1 {
					for _ in prev_indent..indent {
						res.push_str(blockstart);
					}
				}
				else if prev_indent > indent {
					if outstanding_indents > 0 {
						res.push_str(blockend);
						res.push_str(blockstart);
						outstanding_indents = outstanding_indents - (prev_indent-indent);
					}
					else {
						for _ in indent..prev_indent {
							res.push_str(blockend);
						}
					}
				}
				else if indent > prev_indent + 1 {
					outstanding_indents = indent - (prev_indent + 1);
					res.push_str(blockstart);
				}
				prev_indent = indent;
			}
		}
		res.push_str(line);
		res.push('\n');
	}
	// debug!("preprocessing done, prev_indent: {} outstanding_indents: {}", prev_indent, outstanding_indents);
	if 0 < prev_indent {
		for _ in outstanding_indents..prev_indent {
			res.push_str(blockend);
		}
	}
	res
}
