use super::text_presentation::TextPresentation;
use super::value::Value;
use std::vec::Vec;

pub struct ArrayValue {
	content: Vec<Value>,
	left: String,
	right: String,
}

impl ArrayValue {
	pub fn new(content: Vec<Value>, left: String, right: String) {
		todo!()
	}
}

impl TextPresentation for ArrayValue {
	fn build_raw(&self, buffer: &mut String) {
		todo!()
	}

	fn build_trimmed(&self, buffer: &mut String) {
		todo!()
	}
}
