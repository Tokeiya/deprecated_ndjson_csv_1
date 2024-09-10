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

impl NullValue {
	pub fn raw_text(&self) -> &str {
		todo!()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::elements::value::test_helper::add_spaces;

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
}
