use super::text_expression::TextExpression;

pub struct NullValue(String);

impl From<&str> for NullValue {
	fn from(value: &str) -> Self {
		NullValue(value.to_string())
	}
}

impl From<String> for NullValue {
	fn from(value: String) -> Self {
		NullValue(value)
	}
}

impl TextExpression for NullValue {
	fn build_raw_text(&self, buff: &mut String) {
		buff.push_str(&self.0)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::elements::text_expression::test_helper::assert_text_expression;
	use crate::elements::value::test_helper::add_spaces;
	use crate::parser::trimmed_output::test_helper::add_ws;
	#[test]
	fn text_expression() {
		let expected = add_ws("null");
		let act = NullValue::from(expected.as_str());

		assert_text_expression(&act, expected.as_str())
	}

	#[test]
	fn from_str() {
		let act = NullValue::from("null");
		assert_eq!(act.0.as_str(), "null");
	}

	#[test]
	fn from_string() {
		let txt = add_spaces("null");
		let act = NullValue::from(txt.clone());

		assert_eq!(act.0, txt)
	}

	#[test]
	fn raw_text() {
		let txt = NullValue::from("null");
		assert_eq!(txt.raw_text(), "null")
	}
}
