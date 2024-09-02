use super::number::Number;
use std::collections::HashMap;
use std::vec::Vec;
use super::with_raw_value::WithRawValue;


pub enum TypedValue {
	Boolean(bool),
	Null,
	Object(HashMap<String, WithRawValue<TypedValue>>),
	Array(Vec<WithRawValue<TypedValue>>),
	Number(Number),
	String(String),
}

impl From<bool> for TypedValue {
	fn from(value: bool) -> Self {
		TypedValue::Boolean(value)
	}
}

impl From<Number> for TypedValue {
	fn from(value: Number) -> Self {
		TypedValue::Number(value)
	}
}

impl From<String> for TypedValue {
	fn from(value: String) -> Self {
		TypedValue::String(value)
	}
}

impl From<&str> for TypedValue {
	fn from(value: &str) -> Self {
		TypedValue::String(value.to_string())
	}
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn from_boolean() {
		let fixture = TypedValue::from(true);
		assert!(matches!(fixture,TypedValue::Boolean(b) if b==true));

		let fixture = TypedValue::from(false);
		assert!(matches!(fixture,TypedValue::Boolean(b) if b==false));
	}

	#[test]
	fn from_number() {
		let fixture = TypedValue::from(Number::from(42));
		assert!(
			matches!(fixture,TypedValue::Number(num) if matches!(num,Number::Integer(i) if i==42))
		);

		assert!(
			matches!(TypedValue::from(Number::from(std::f64::consts::PI)),
			TypedValue::Number(num) if matches!(num,Number::Float(f) if f==std::f64::consts::PI))
		)
	}

	#[test]
	fn from_string() {
		assert!(matches!(TypedValue::from("hello world".to_string()),
			TypedValue::String(str) if str=="hello world"));
	}

	#[test]
	fn from_str() {
		assert!(matches!(TypedValue::from("hello rust"),
			TypedValue::String(str) if str=="hello rust"));
	}
}
