use super::text_presentation::TextPresentation;

pub struct NullValue(String);

impl From<&str> for NullValue {
	fn from(value: &str) -> Self {
		todo!()
	}
}

impl From<String> for NullValue {
	fn from(value: String) -> Self {
		todo!()
	}
}

impl TextPresentation for NullValue {
	fn raw_text(&self) -> &str {
		todo!()
	}

	fn trimmed_text(&self) -> &str {
		todo!()
	}
}
