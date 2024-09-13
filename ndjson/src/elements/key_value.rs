use super::string_value::StringValue;
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
mod test {}