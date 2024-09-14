use super::value::StringValue;
use super::value::Value;

pub struct KeyValue {
	key: StringValue,
	value: Value,
}

impl KeyValue {
	pub fn new(key: StringValue, value: Value) -> Self {
		KeyValue { key, value }
	}

	pub fn key(&self) -> &StringValue {
		todo!()
	}

	pub fn value(&self) -> &Value {
		todo!()
	}

	pub fn raw_string(&self) -> String {
		todo!()
	}
}

#[cfg(test)]
mod test {
	use super::super::number_value::{test_helper as num_helper, Number, NumberValue};
	use super::super::value::StringValue;
	use super::super::value::{test_helper as value_helper, BooleanValue, Value};
	use super::*;
	#[test]
	fn new() {
		let fixture = KeyValue::new(
			StringValue::new("foo".to_string(), "bar".to_string()),
			Value::from(StringValue::new("elem".to_string(), "hoge".to_string())),
		);

		assert_eq!(fixture.key.value(), &"foo");
		assert_eq!(fixture.key.raw_text(), "bar");

		assert_eq!(fixture.value.extract_string().value(), "elem");
		assert_eq!(fixture.value.extract_string().raw_text(), "hoge");
	}

	#[test]
	fn key() {
		let fixture = KeyValue::new(
			StringValue::new("foo".to_string(), "bar".to_string()),
			Value::from(StringValue::new("elem".to_string(), "elem".to_string())),
		);

		assert_eq!(fixture.key().value(), &"foo");
		assert_eq!(fixture.key().raw_text(), "bar");
	}

	#[test]
	fn value() {
		let fixture = KeyValue::new(
			StringValue::new("foo".to_string(), "bar".to_string()),
			Value::from(StringValue::new("elem".to_string(), "hoge".to_string())),
		);

		assert_eq!(fixture.value().extract_string().value(), "elem");
		assert_eq!(fixture.value().extract_string().raw_text(), "hoge");
	}
}
