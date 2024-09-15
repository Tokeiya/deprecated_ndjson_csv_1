use super::key_value::KeyValue;
//use crate::elements::value::StringValue;
use super::text_expression::TextExpression;

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
		let mut buff = self.begin.to_string();

		for elem in self.content.iter() {
			buff.push_str(&elem.raw_string())
		}

		buff.push_str(self.end());

		buff
	}
}

impl TextExpression for ObjectValue {
	fn build_raw_text(&self, buff: &mut String) {
		buff.push_str(&self.begin);

		for elem in self.content.iter() {
			elem.key().build_raw_text(buff);
			buff.push(':');
			elem.value().build_raw_text(buff);
			buff.push(',');
		}

		if self.content.len() != 0 {
			buff.pop();
		}

		buff.push_str(&self.end)
	}
}

#[cfg(test)]
mod test {
	use super::super::null_value::NullValue;
	use super::super::number_value::from_i128;
	use super::super::number_value::test_helper as num_helper;
	use super::super::text_expression::test_helper::assert_text_expression;
	use super::super::value::BooleanValue;
	use super::super::value::StringValue;
	use super::super::value::Value;
	use super::*;
	use crate::parser::trimmed_output::test_helper::add_ws;

	fn context() -> Vec<KeyValue> {
		vec![
			KeyValue::new(
				StringValue::new("foo".to_string(), add_ws(r#""foo""#)),
				Value::from(NullValue::from("null")),
			),
			KeyValue::new(
				StringValue::new("bar".to_string(), add_ws(r#""bar""#)),
				Value::from(BooleanValue::new(true, "true".to_string())),
			),
			KeyValue::new(
				StringValue::new("hoge".to_string(), add_ws(r#""hoge""#)),
				Value::from(from_i128(42, "42".to_string())),
			),
		]
	}
	#[test]
	fn raw_string() {
		let content = context();

		let fixture = ObjectValue::new(content, add_ws("{"), add_ws("}"));
		println!("{:?}", fixture.raw_string());

		let mut buff = add_ws("{");
		buff.push_str(&add_ws(r#""foo""#));
		buff.push(':');
		buff.push_str("null");
		buff.push(',');

		buff.push_str(&add_ws(r#""bar""#));
		buff.push(':');
		buff.push_str(&"true");
		buff.push(',');

		buff.push_str(&add_ws(r#""hoge""#));
		buff.push(':');
		buff.push_str(&"42");
		buff.push_str(&add_ws("}"));

		assert_text_expression(&fixture, &buff);
	}

	#[test]
	fn new() {
		let content = context();

		let fixture = ObjectValue::new(content, add_ws("{"), add_ws("}"));

		assert_eq!(&fixture.begin, &add_ws("{"));
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
		let content = context();

		let fixture = ObjectValue::new(content, add_ws("{"), add_ws("}"));

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
