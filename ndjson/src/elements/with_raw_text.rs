use super::text_expression::TextExpression;

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

	pub fn raw_text(&self) -> &str {
		&self.raw_text
	}
}

impl<T> TextExpression for WithRawText<T> {
	fn build_raw_text(&self, buff: &mut String) {
		buff.push_str(&self.raw_text)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::elements::text_expression::test_helper::assert_text_expression;

	#[test]
	fn from_string() {
		let fixture = WithRawText::new(20, 20.to_string());
		assert_eq!(fixture.value, 20);
		assert_eq!(fixture.raw_text, "20");
	}

	#[test]
	fn value() {
		let fixture = WithRawText::new(20, 20.to_string());
		assert_eq!(fixture.value(), &20);
	}

	#[test]
	fn raw_text() {
		let fixture = WithRawText::new(20, 20.to_string());
		assert_eq!(fixture.value(), &20);

		assert_text_expression(&fixture, "20");
	}
}
