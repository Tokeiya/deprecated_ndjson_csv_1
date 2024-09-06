use super::text_presentation::TextPresentation;

pub struct NullValue(String);

impl NullValue {
	pub fn new(raw_text: String) -> Self {
		NullValue(raw_text)
	}
}

impl TextPresentation for NullValue {
	fn build_raw(&self, buffer: &mut String) {
		buffer.push_str(self.0.as_str())
	}

	fn build_trimmed(&self, buffer: &mut String) {
		buffer.push_str("null")
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use super::super::text_presentation::test_helper::*;
	#[test]
	fn new() {
		let fixture = NullValue::new("  \t null \r \n    ".to_string());
		assert_eq!(fixture.0, "  \t null \r \n    ");
	}

	#[test]
	fn text_presentation() {
		let fixture = NullValue::new("  \t null \r \n    ".to_string());

		assert_trimmed(&fixture, "null");
		assert_raw(&fixture, "  \t null \r \n    ");
	}
}