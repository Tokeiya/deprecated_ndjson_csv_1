use super::number::Number;
use super::with_raw_value::WithRawValue;
use std::collections::HashMap;
use std::vec::Vec;
use crate::values::text_presentation::TextPresentation;
use super::{bool_value::BoolValue, null_value::NullValue, number_value::NumberValue, string_value::StringValue};
pub enum Value {
	Boolean(BoolValue),
	Null(NullValue),
	Object(HashMap<String, Value>),
	Array(Vec<Value>),
	Number(NumberValue),
	String(StringValue),
}

impl TextPresentation for Value {
	fn build_raw(&self, buffer: &mut String) {
		todo!()
	}
	fn build_trimmed(&self, buffer: &mut String) {
		todo!()
	}
}



