use super::text_expression::TextExpression;
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
		&self.key
	}

	pub fn value(&self) -> &Value {
		&self.value
	}

	pub fn raw_string(&self) -> String {
		format!("{}:{}", self.key.raw_text(), self.value.raw_string())
	}
}

impl TextExpression for KeyValue {
	fn build_raw_text(&self, buff: &mut String) {
		self.key.build_raw_text(buff);
		buff.push(':');
		self.value.build_raw_text(buff)
	}
}

#[cfg(test)]
mod test {
	use super::super::value::StringValue;
	use super::super::value::Value;
	use super::*;
	use crate::elements::number_value::from_i128;
	use crate::elements::text_expression::test_helper::assert_text_expression;
	use crate::parser::trimmed_output::test_helper::add_ws;

	#[test]
	fn raw_string() {
		let key = StringValue::new("key".to_string(), add_ws(r#""key""#));
		let value = Value::from(from_i128(42, add_ws("42")));

		let fixture = KeyValue::new(key, value);

		assert_text_expression(
			&fixture,
			&format!(
				"{}:{}",
				fixture.key().raw_text(),
				fixture.value().raw_string()
			),
		);
	}

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
