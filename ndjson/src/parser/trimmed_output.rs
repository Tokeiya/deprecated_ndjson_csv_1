use super::white_space::is_ws;

pub struct TrimmedOutput {
	raw: String,
	left_fence: Option<usize>,
	right_fence: Option<usize>,
}

impl TrimmedOutput {
	fn detect_left(value: &str) -> Option<usize> {
		let len = value.chars().take_while(is_ws).count();

		if len == 0 {
			None
		} else {
			Some(len)
		}
	}

	fn detect_right(value: &str) -> Option<usize> {
		let len = value.chars().rev().take_while(is_ws).count();

		if len == 0 {
			None
		} else {
			Some(value.len() - len)
		}
	}
	pub fn new(raw_text: String) -> Self {
		let left_fence = Self::detect_left(&raw_text);
		let right_fence = Self::detect_right(&raw_text);

		Self {
			raw: raw_text,
			left_fence,
			right_fence,
		}
	}
	pub fn raw(&self) -> &str {
		&self.raw
	}

	pub fn trimmed(&self) -> &str {
		self.raw.trim_matches(|x| is_ws(&x))
	}

	pub fn left(&self) -> &str {
		if self.left_fence.is_none() {
			""
		} else {
			&self.raw[..self.left_fence.unwrap()]
		}
	}

	pub fn right(&self) -> &str {
		if self.right_fence.is_none() {
			""
		} else {
			&self.raw[self.right_fence.unwrap()..]
		}
	}
}

#[cfg(test)]
pub mod test_helper {
	use super::*;
	use std::sync::LazyLock;

	pub static WS: LazyLock<String> = LazyLock::new(|| "\u{20}\u{09}\u{0A}\u{0D}".to_string());

	pub fn add_ws(value: &str) -> String {
		format!("{}{value}{}", WS.as_str(), WS.as_str())
	}

	pub fn assert(actual: &(TrimmedOutput, &str), rem: Option<&str>, raw: &str) {
		if let Some(r) = rem {
			assert_eq!(actual.1, r)
		} else {
			assert_eq!(actual.1, "")
		}

		let exp = TrimmedOutput::new(raw.to_string());

		let tmp = &actual.0;
		assert_eq!(tmp.raw(), raw);
		assert_eq!(tmp.trimmed(), exp.trimmed());
		assert_eq!(tmp.left(), exp.left());
		assert_eq!(tmp.right(), exp.right());
	}
}

#[cfg(test)]
mod test {
	use super::test_helper::WS;
	use super::*;

	#[test]
	fn new() {
		let mut buff = String::new();
		buff.push_str(WS.as_str());
		buff.push_str("value");
		buff.push_str(WS.as_str());

		let fixture = TrimmedOutput::new(buff.to_string());
		assert_eq!(&fixture.raw, buff.as_str());
		assert_eq!(fixture.left_fence.unwrap(), 4);
		assert_eq!(fixture.right_fence.unwrap(), 9);

		buff.clear();
		buff.push_str(WS.as_str());
		buff.push_str("value");

		let fixture = TrimmedOutput::new(buff.to_string());
		assert_eq!(&fixture.raw, buff.as_str());
		assert_eq!(fixture.left_fence.unwrap(), 4);
		assert!(fixture.right_fence.is_none());

		buff.clear();
		buff.push_str("value");
		buff.push_str(WS.as_str());

		let fixture = TrimmedOutput::new(buff.to_string());
		assert_eq!(&fixture.raw, buff.as_str());
		assert!(fixture.left_fence.is_none());
		assert_eq!(fixture.right_fence.unwrap(), 5);
	}

	#[test]
	fn trimmed() {
		let mut buff = String::new();
		buff.push_str(WS.as_str());
		buff.push_str("value");
		buff.push_str(WS.as_str());

		let fixture = TrimmedOutput::new(buff);
		assert_eq!(fixture.trimmed(), "value")
	}

	#[test]
	fn raw() {
		let mut buff = String::new();
		buff.push_str(WS.as_str());
		buff.push_str("value");
		buff.push_str(WS.as_str());

		let fixture = TrimmedOutput::new(buff.to_string());
		assert_eq!(fixture.raw(), buff.as_str());
	}

	#[test]
	fn left() {
		let mut buff = String::new();
		buff.push_str(WS.as_str());
		buff.push_str("value");
		buff.push_str(WS.as_str());

		let fixture = TrimmedOutput::new(buff.to_string());
		assert_eq!(fixture.left(), WS.as_str());

		buff.clear();
		buff.push_str("value");

		let fixture = TrimmedOutput::new(buff.to_string());
		assert_eq!(fixture.left(), "");
	}

	#[test]
	fn right() {
		let mut buff = String::new();
		buff.push_str(WS.as_str());
		buff.push_str("value");
		buff.push_str(WS.as_str());

		let fixture = TrimmedOutput::new(buff.to_string());
		assert_eq!(fixture.right(), WS.as_str());

		buff.clear();
		buff.push_str("value");

		let fixture = TrimmedOutput::new(buff.to_string());
		assert_eq!(fixture.right(), "");
	}
}
