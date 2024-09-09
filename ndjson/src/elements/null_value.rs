use super::text_presentation::TextPresentation;

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

impl TextPresentation for NullValue {
	fn build_raw(&self, buffer: &mut String) {
		buffer.push_str(&self.0)
	}

	fn build_trimmed(&self, buffer: &mut String) {
		buffer.push_str("null")
	}
}

#[cfg(test)]
mod tests {
	use super::super::text_presentation::test_helper::*;
	use super::*;

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
	fn text_presentation() {
		let txt = add_spaces("null");
		let act = NullValue::from(txt.as_str());
		//assert_all(txt.as_str())
		assert_raw(&act, &txt);
		assert_trimmed(&act, "null");
	}
}
