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

#[cfg(test)]
mod tests {
	use super::super::number_value::test_helper as num_helper;
	use super::super::value::test_helper as value_helper;
	use super::*;
	use crate::elements::number_value::{Number, NumberValue};
	use crate::elements::value::{BooleanValue, Value};

	// #[test]
	// fn new() {
	// 	let mut vec = Vec::<Value>::new();
	//
	// 	let tmp= Value::Number(NumberValue::new(Number::from(42.195),"   42.195    ".to_string());
	//
	// 	vec.push(tmp);
	// }
}
