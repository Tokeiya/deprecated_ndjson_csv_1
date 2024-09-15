use super::text_expression::TextExpression;
use super::value::Value;
use std::vec::Vec;

pub struct ArrayValue {
	content: Vec<Value>,
	begin: String,
	end: String,
}

impl ArrayValue {
	pub fn new(content: Vec<Value>, begin: String, end: String) -> Self {
		Self {
			content,
			begin,
			end,
		}
	}

	pub fn contents(&self) -> &[Value] {
		&self.content
	}

	pub fn beigin(&self) -> &str {
		&self.begin
	}

	pub fn end(&self) -> &str {
		&self.end
	}
}

impl TextExpression for ArrayValue {
	fn build_raw_text(&self, buff: &mut String) {
		buff.push_str(&self.begin);

		for elem in self.content.iter() {
			buff.push_str(&elem.raw_string());
			buff.push(',');
		}

		if self.content.len() != 0 {
			buff.pop();
		}

		buff.push_str(&self.end);
	}
}

#[cfg(test)]
mod tests {
	use super::super::number_value::from_i128;
	use super::super::number_value::test_helper as num;
	use super::super::text_expression::test_helper::assert_text_expression;
	use super::super::value::StringValue;
	use super::*;
	use crate::elements::null_value::NullValue;
	use crate::elements::value::{BooleanValue, Value};

	fn gen_vec() -> Vec<Value> {
		let mut vec = Vec::<Value>::new();

		vec.push(Value::String(StringValue::new(
			"hello world".to_string(),
			r#""hello world""#.to_string(),
		)));
		vec.push(Value::Boolean(BooleanValue::new(true, "true".to_string())));
		vec.push(Value::Null(NullValue::from("null")));
		vec.push(Value::Number(from_i128(42, "42".to_string())));

		vec
	}
	#[test]
	fn raw_string() {
		let vec = gen_vec();
		let fixture = ArrayValue::new(
			vec,
			"     [         ".to_string(),
			"    ]      ".to_string(),
		);
		
		assert_text_expression(
			&fixture,
			"     [         \"hello world\",true,null,42    ]      ",
		);
	}

	#[test]
	fn new() {
		let vec = gen_vec();

		let fixture = ArrayValue::new(
			vec,
			"     [         ".to_string(),
			"    ]      ".to_string(),
		);

		assert_eq!(&fixture.begin, "     [         ");
		assert_eq!(&fixture.end, "    ]      ");

		assert_eq!(fixture.content.len(), 4);
		assert_eq!(fixture.content[0].extract_string().value(), "hello world");
		assert_eq!(fixture.content[1].extract_bool().value(), &true);
		_ = fixture.content[2].extract_null();
		num::is_integer(fixture.content[3].extract_number(), 42);
	}

	#[test]
	fn contents() {
		let mut vec = Vec::<Value>::new();

		vec.push(Value::String(StringValue::new(
			"hello world".to_string(),
			r#""hwllo world""#.to_string(),
		)));
		vec.push(Value::Boolean(BooleanValue::new(true, "true".to_string())));
		vec.push(Value::Null(NullValue::from("null")));
		vec.push(Value::Number(from_i128(42, "42".to_string())));

		let fixture = ArrayValue::new(vec, "[".to_string(), "]".to_string());

		let contents = fixture.contents();
		assert_eq!(contents.len(), 4);
		assert_eq!(contents[0].extract_string().value(), "hello world");
		assert_eq!(contents[1].extract_bool().value(), &true);
		_ = contents[2].extract_null();
		num::is_integer(contents[3].extract_number(), 42);
	}

	#[test]
	fn begin_end() {
		let fixture = ArrayValue::new(Vec::new(), "      [ ".to_string(), "]".to_string());

		assert_eq!(fixture.beigin(), "      [ ");
		assert_eq!(fixture.end(), "]");
	}
}
