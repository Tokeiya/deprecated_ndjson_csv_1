use super::text_presentation::TextPresentation;
use super::value::Value;
use std::vec::Vec;
pub type ArrayValue = Vec<Value>;

impl TextPresentation for ArrayValue {
	fn raw_text(&self) -> &str {
		todo!()
	}

	fn trimmed_text(&self) -> &str {
		todo!()
	}
}
