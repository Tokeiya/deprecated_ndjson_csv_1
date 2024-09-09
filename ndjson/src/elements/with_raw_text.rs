use super::text_presentation::TextPresentation;

pub struct WithRawText<T> {
	value: T,
	raw_text: String,
}

impl<T> WithRawText<T> {
	pub fn new(value: T, raw_text: String) -> Self {
		Self { value, raw_text }
	}

	pub fn value(&self) -> &T {
		&self.value
	}
}

impl<T> TextPresentation for WithRawText<T> {
	fn build_raw(&self, buffer: &mut String) {
		buffer.push_str(&self.raw_text)
	}

	fn build_trimmed(&self, buffer: &mut String) {
		buffer.push_str(self.raw_text.trim())
	}
}

#[cfg(test)]
pub mod test_helper {
	use super::*;

	impl<T> WithRawText<T> {
		pub fn assert_raw(&self, expected: &str) {
			assert_eq!(self.raw_text, expected)
		}
		pub fn assert_trimmed(&self, expected: &str) {
			assert_eq!(self.trimmed(), expected)
		}
	}
}

#[cfg(test)]
mod test {
	use super::super::text_presentation::test_helper::*;
	use super::*;

	#[test]
	fn from_string() {
		let fixture = WithRawText::new(20, 20.to_string());
		assert_eq!(fixture.value, 20);
		assert_eq!(fixture.raw_text, "20");
	}

	#[test]
	fn text_presentation() {
		let expected = add_spaces("42.195");
		let fixture = WithRawText::new(420195, expected.to_string());

		assert_raw(&fixture, expected.as_str());
		assert_trimmed(&fixture, "42.195");
	}

	#[test]
	fn value() {
		let fixture = WithRawText::new(20, 20.to_string());
		assert_eq!(fixture.value(), &20);
	}
}
