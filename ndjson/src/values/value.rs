use super::number::Number;
use super::with_raw_value::WithRawValue;
use std::collections::HashMap;
use std::vec::Vec;

pub enum Value {
	Boolean(bool),
	Null,
	Object(HashMap<String, WithRawValue<Value>>),
	Array(Vec<WithRawValue<Value>>),
	Number(Number),
	String(String),
}

impl From<bool> for Value {
	fn from(value: bool) -> Self {
		Value::Boolean(value)
	}
}

impl From<Number> for Value {
	fn from(value: Number) -> Self {
		Value::Number(value)
	}
}

impl From<String> for Value {
	fn from(value: String) -> Self {
		Value::String(value)
	}
}

impl From<&str> for Value {
	fn from(value: &str) -> Self {
		Value::String(value.to_string())
	}
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn from_boolean() {
		let fixture = Value::from(true);
		assert!(matches!(fixture,Value::Boolean(b) if b==true));

		let fixture = Value::from(false);
		assert!(matches!(fixture,Value::Boolean(b) if b==false));
	}

	#[test]
	fn from_number() {
		let fixture = Value::from(Number::from(42));
		assert!(matches!(fixture,Value::Number(num) if matches!(num,Number::Integer(i) if i==42)));

		assert!(matches!(Value::from(Number::from(std::f64::consts::PI)),
			Value::Number(num) if matches!(num,Number::Float(f) if f==std::f64::consts::PI)))
	}

	#[test]
	fn from_string() {
		assert!(matches!(Value::from("hello world".to_string()),
			Value::String(str) if str=="hello world"));
	}

	#[test]
	fn from_str() {
		assert!(matches!(Value::from("hello rust"),
			Value::String(str) if str=="hello rust"));
	}
}
