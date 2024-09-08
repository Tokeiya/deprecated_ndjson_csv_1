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
	fn raw(&self) -> &str {
		self.0.as_str()
	}
	fn trimmed(&self) -> &str {
		"null"
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
		(&act as &dyn TextPresentation).assert_all(txt.as_str())
	}
}
