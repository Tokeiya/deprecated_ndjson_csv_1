use super::text_presentation::TextPresentation;

pub struct BoolValue {
	value: bool,
	raw_text: String,
}

impl BoolValue {
	pub fn new(value: bool, raw_text: String) -> Self {
		BoolValue {
			value,
			raw_text,
		}
	}

	pub fn value(&self) -> bool {
		self.value
	}
}

impl TextPresentation for BoolValue {
	fn build_raw(&self, buffer: &mut String) {
		buffer.push_str(self.raw_text.as_str())
	}
	fn build_trimmed(&self, buffer: &mut String) {
		match self.value {
			true => buffer.push_str("true"),
			false => buffer.push_str("false")
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::text_presentation::test_helper::*;

	#[test]
	fn new() {
		let fixture = BoolValue::new(false, "  false   ".to_string());
		assert_eq!(fixture.value, false);
		assert_eq!(&fixture.raw_text, "  false   ")
	}

	#[test]
	fn value() {
		let fixture = BoolValue::new(true, "   true".to_string());
		assert_eq!(fixture.value(), true)
	}


	#[test]
	fn text_presentation() {
		let fixture = BoolValue::new(true, "  \t true   \n ".to_string());

		assert_raw(&fixture, "  \t true   \n ");
		assert_trimmed(&fixture, "true");
	}
}