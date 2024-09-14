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
	use super::super::string_value::StringValue;
	use super::super::value::{test_helper as value_helper, BooleanValue, Value};
	use super::*;
	#[test]
	fn new() {}
}
