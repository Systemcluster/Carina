use super::types::InputIterItem;

///
/// Detect non-alphanumeric chars in the ASCII range
///
pub fn is_special(token: impl InputIterItem) -> bool {
	for i in 0x20..=0x2F {
		if std::str::from_utf8(&[i]) == Ok(token.as_ref()) {
			return true;
		}
	}
	for i in 0x3A..=0x40 {
		if std::str::from_utf8(&[i]) == Ok(token.as_ref()) {
			return true;
		}
	}
	for i in 0x5B..=0x60 {
		if std::str::from_utf8(&[i]) == Ok(token.as_ref()) {
			return true;
		}
	}
	for i in 0x7B..=0x7F {
		if std::str::from_utf8(&[i]) == Ok(token.as_ref()) {
			return true;
		}
	}
	false
}

///
/// Detect non-printable chars in the lower ASCII range
///
pub fn is_valid(token: impl InputIterItem) -> bool {
	for i in 0x00..=0x1F {
		// allow \t, \n, \r
		if i != 0x09 && i != 0x0A && i != 0x0D && std::str::from_utf8(&[i]) == Ok(token.as_ref()) {
			return false;
		}
	}
	true
}

///
/// Detect usual newline literals
///
pub fn is_newline(token: impl InputIterItem) -> bool {
	["\r\n", "\n", "\r"].contains(&token.as_ref())
}

///
/// Detect space
///
pub fn is_space(token: impl InputIterItem) -> bool {
	token.as_ref() == " "
}

///
/// Detect tab
///
pub fn is_tab(token: impl InputIterItem) -> bool {
	token.as_ref() == "\t"
}

///
/// Detect regular characters
///
pub fn is_regular(token: impl InputIterItem) -> bool {
	is_valid(&token) && !is_special(&token) && !is_newline(&token)
}
