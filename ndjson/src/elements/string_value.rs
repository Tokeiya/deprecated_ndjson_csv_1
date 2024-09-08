use super::text_presentation::TextPresentation;
pub struct StringValue(String);

impl From<String> for StringValue {
	fn from(value: String) -> Self {
		todo!()
	}
}

impl From<&str> for StringValue {
	fn from(value: &str) -> Self {
		todo!()
	}
}

impl StringValue {
	pub fn value(&self) -> &String {
		todo!()
	}
}

impl TextPresentation for StringValue {
	fn raw_text(&self) -> &str {
		todo!()
	}

	fn trimmed_text(&self) -> &str {
		todo!()
	}
}
