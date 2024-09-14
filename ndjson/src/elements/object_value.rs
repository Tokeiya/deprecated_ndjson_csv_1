use super::key_value::KeyValue;
use crate::elements::object_value_element::ObjectValueElement;
use crate::elements::value::StringValue;
use std::collections::HashMap;
pub struct ObjectValue {
	content: Vec<KeyValue>,
	begin: String,
	end: String,
}

impl ObjectValue {
	pub fn new(content: Vec<KeyValue>, begin: String, end: String) -> Self {
		Self {
			content,
			begin,
			end,
		}
	}

	pub fn content(&self) -> &[KeyValue] {
		&self.content
	}

	pub fn begin(&self) -> &str {
		&self.begin
	}

	pub fn end(&self) -> &str {
		&self.end
	}

	pub fn raw_string(&self) -> String {
		todo!()
	}
}

#[cfg(test)]
mod test {
	use super::super::null_value::NullValue;
	use super::super::number_value::from_i128;
	use super::super::number_value::test_helper as num_helper;
	use super::super::object_value_element::ObjectValueElement;
	use super::super::value::BooleanValue;
	use super::super::value::StringValue;
	use super::super::value::Value;
	use super::*;
	use crate::parser::trimmed_output::test_helper::add_ws;
	use std::collections::HashMap;
	#[test]
	fn raw_string() {
		let mut content = Vec::<KeyValue>::new();
		content.push(KeyValue::new(
			StringValue::new("foo".to_string(), add_ws(r#""foo""#)),
			Value::from(NullValue::from("null")),
		));
		content.push(KeyValue::new(
			StringValue::new("bar".to_string(), add_ws(r#""bar""#)),
			Value::from(BooleanValue::new(true, "true".to_string())),
		));

		content.push(KeyValue::new(
			StringValue::new("hoge".to_string(), add_ws(r#""hoge""#)),
			Value::from(from_i128(42, "42".to_string())),
		));

		let fixture = ObjectValue::new(content, add_ws("{"), add_ws("}"));
		println!("{}", fixture.raw_string());

		todo!()
	}

	#[test]
	fn new() {
		let mut content = Vec::<KeyValue>::new();
		content.push(KeyValue::new(
			StringValue::new("foo".to_string(), add_ws(r#""foo""#)),
			Value::from(NullValue::from("null")),
		));
		content.push(KeyValue::new(
			StringValue::new("bar".to_string(), add_ws(r#""bar""#)),
			Value::from(BooleanValue::new(true, "true".to_string())),
		));

		content.push(KeyValue::new(
			StringValue::new("hoge".to_string(), add_ws(r#""hoge""#)),
			Value::from(from_i128(42, "42".to_string())),
		));

		let fixture = ObjectValue::new(content, add_ws("{"), add_ws("}"));

		assert_eq!(&fixture.end, &add_ws("{"));
		assert_eq!(&fixture.end, &add_ws("}"));
		assert_eq!(fixture.content.len(), 3);

		let content = &fixture.content;
		assert_eq!(content[0].key().value(), "foo");
		_ = content[0].value().extract_null();

		assert_eq!(content[1].key().value(), "bar");
		assert!(content[1].value().extract_bool().value());

		assert_eq!(content[2].key().value(), "hoge");
		num_helper::is_integer(content[2].value().extract_number(), 42);
	}

	#[test]
	fn content() {
		let mut content = Vec::<KeyValue>::new();
		content.push(KeyValue::new(
			StringValue::new("foo".to_string(), add_ws(r#""foo""#)),
			Value::from(NullValue::from("null")),
		));
		content.push(KeyValue::new(
			StringValue::new("bar".to_string(), add_ws(r#""bar""#)),
			Value::from(BooleanValue::new(true, "true".to_string())),
		));

		content.push(KeyValue::new(
			StringValue::new("hoge".to_string(), add_ws(r#""hoge""#)),
			Value::from(from_i128(42, "42".to_string())),
		));

		let fixture = ObjectValue::new(content, add_ws("{"), add_ws("}"));

		assert_eq!(&fixture.end, &add_ws("{"));
		assert_eq!(&fixture.end, &add_ws("}"));
		assert_eq!(fixture.content.len(), 3);

		assert_eq!(fixture.content()[0].key().value(), "foo");
		_ = fixture.content()[0].value().extract_null();

		assert_eq!(fixture.content()[1].key().value(), "bar");
		assert!(fixture.content()[1].value().extract_bool().value());

		assert_eq!(fixture.content()[2].key().value(), "hoge");
		num_helper::is_integer(fixture.content()[2].value().extract_number(), 42);
	}

	#[test]
	fn begin_end() {
		let fixture = ObjectValue::new(Vec::new(), add_ws("{"), add_ws("}"));

		assert_eq!(fixture.begin(), &add_ws("{"));
		assert_eq!(fixture.end(), &add_ws("}"));
	}
}
