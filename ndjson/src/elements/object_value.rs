use super::text_presentation::TextPresentation;
use crate::elements::object_value_element::ObjectValueElement;
use crate::elements::string_value::StringValue;
use std::collections::HashMap;

pub struct ObjectValue {
	content: HashMap<StringValue, ObjectValueElement>,
	left: String,
	right: String,
}

impl ObjectValue {
	pub fn new(
		content: HashMap<StringValue, ObjectValueElement>,
		left: String,
		right: String,
	) -> Self {
		todo!()
	}
}

impl TextPresentation for ObjectValue {
	fn build_raw(&self, buffer: &mut String) {
		todo!()
	}

	fn build_trimmed(&self, buffer: &mut String) {
		todo!()
	}
}
