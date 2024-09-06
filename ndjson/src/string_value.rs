use super::text_presentation::TextPresentation;

pub struct StringValue(String);

impl StringValue {
	pub fn value(&self) -> &str {
		self.0.trim().trim_matches('"')
	}
}

impl From<String> for StringValue {
	fn from(value: String) -> Self {
		StringValue(value)
	}
}

impl From<&str> for StringValue {
	fn from(value: &str) -> Self {
		StringValue(value.to_string())
	}
}

impl TextPresentation for StringValue {
	fn build_raw(&self, buffer: &mut String) {
		buffer.push_str(self.0.as_str())
	}

	fn build_trimmed(&self, buffer: &mut String) {
		buffer.push_str(self.0.trim())
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use super::super::text_presentation::test_helper::*;

	#[test]
	fn from_string() {
		let act = StringValue::from("hello".to_string());
		assert_eq!(act.0, "hello");
	}

	#[test]
	fn from_str() {
		let act = StringValue::from("world");
		assert_eq!(act.0, "world");
	}

	#[test]
	fn value() {
		let act = StringValue::from("     \"hello world\"   \r \n");
		assert_eq!(act.value(), "hello world");
	}

	#[test]
	fn text_presentation() {
		let act = StringValue::from("  \r\n  \"   hello world   \" \t ");
		assert_raw(&act, "  \r\n  \"   hello world   \" \t ");
		assert_trimmed(&act, r#""   hello world   ""#);
	}
}


