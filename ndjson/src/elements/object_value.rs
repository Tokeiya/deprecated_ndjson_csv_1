use crate::elements::object_value_element::ObjectValueElement;
use crate::elements::string_value::StringValue;
use std::collections::HashMap;

pub struct ObjectValue {
	content: HashMap<StringValue, ObjectValueElement>,
	begin: String,
	end: String,
}

impl ObjectValue {
	pub fn new(
		content: HashMap<StringValue, ObjectValueElement>,
		begin: String,
		end: String,
	) -> Self {
		Self {
			content,
			begin,
			end,
		}
	}

	pub fn content(&self) -> &HashMap<StringValue, ObjectValueElement> {
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
		// let mut buff = String::new();
		// 
		// buff.push_str(&self.begin);
		// 
		// 
		// for (k, v) in self.content() {
		// 	buff.push_str(&k.raw_text());
		// 	buff.push(':');
		// 	buff.push_str(&v.ra)
		// }
	}
}

#[cfg(test)]
mod test {
	use super::super::null_value::NullValue;
	use super::super::number_value::from_i128;
	use super::super::number_value::test_helper as num_helper;
	use super::super::object_value_element::ObjectValueElement;
	use super::super::string_value::StringValue;
	use super::super::value::Value;
	use super::*;
	use std::collections::HashMap;

	#[test]
	fn raw_string() {
		let mut map = HashMap::new();
		map.insert(
			StringValue::new("num".to_string(), r#""num""#.to_string()),
			ObjectValueElement::from(Value::from(from_i128(42, "42".to_string()))),
		);
		map.insert(
			StringValue::new("null".to_string(), r#"null"#.to_string()),
			ObjectValueElement::from(Value::from(NullValue::from("null"))),
		);

		let mut arr = Vec::new();
		arr.push(Value::Number(from_i128(42, "42".to_string())));
		arr.push(Value::Null(NullValue::from("null")));

		map.insert(
			StringValue::new("multi".to_string(), r#"multi"#.to_string()),
			ObjectValueElement::from(arr),
		);

		let fixture = ObjectValue::new(map, "{".to_string(), "}".to_string());

		println!("{}", fixture.raw_string());

		todo!()
	}

	#[test]
	fn new() {
		let mut map = HashMap::new();
		map.insert(
			StringValue::new("num".to_string(), r#""num""#.to_string()),
			ObjectValueElement::from(Value::from(from_i128(42, "42".to_string()))),
		);
		map.insert(
			StringValue::new("null".to_string(), r#"null"#.to_string()),
			ObjectValueElement::from(Value::from(NullValue::from("null"))),
		);

		let mut arr = Vec::new();
		arr.push(Value::Number(from_i128(42, "42".to_string())));
		arr.push(Value::Null(NullValue::from("null")));

		map.insert(
			StringValue::new("multi".to_string(), r#"multi"#.to_string()),
			ObjectValueElement::from(arr),
		);

		let fixture = ObjectValue::new(map, "{".to_string(), "}".to_string());

		assert_eq!(&fixture.begin, "{");
		assert_eq!(&fixture.end, "}");

		assert_eq!(fixture.content.len(), 3);

		num_helper::is_integer(
			fixture.content[&StringValue::new("num".to_string(), r#""num""#.to_string())]
				.extract_single()
				.extract_number(),
			42,
		);

		_ = &fixture.content[&StringValue::new("null".to_string(), r#""null""#.to_string())]
			.extract_single()
			.extract_null();

		_ = &fixture.content[&StringValue::new("multi".to_string(), r#""multi""#.to_string())]
			.extract_multi();
	}

	#[test]
	fn content() {
		let mut map = HashMap::new();
		map.insert(
			StringValue::new("num".to_string(), r#""num""#.to_string()),
			ObjectValueElement::from(Value::from(from_i128(42, "42".to_string()))),
		);
		map.insert(
			StringValue::new("null".to_string(), r#"null"#.to_string()),
			ObjectValueElement::from(Value::from(NullValue::from("null"))),
		);

		let mut arr = Vec::new();
		arr.push(Value::Number(from_i128(42, "42".to_string())));
		arr.push(Value::Null(NullValue::from("null")));

		map.insert(
			StringValue::new("multi".to_string(), r#"multi"#.to_string()),
			ObjectValueElement::from(arr),
		);

		let fixture = ObjectValue::new(map, "{".to_string(), "}".to_string());

		assert_eq!(fixture.content().len(), 3);
		_ = fixture.content()[&StringValue::new("null".to_string(), r#""num""#.to_string())]
			.extract_single()
			.extract_number();
		_ = fixture.content()[&StringValue::new("null".to_string(), r#""null""#.to_string())]
			.extract_single()
			.extract_null();
		_ = fixture.content()[&StringValue::new("multi".to_string(), r#""multi""#.to_string())]
			.extract_multi();
	}

	#[test]
	fn begin_end() {
		let fixture = ObjectValue::new(HashMap::new(), "{".to_string(), "}".to_string());

		assert_eq!(fixture.begin(), "{");
		assert_eq!(fixture.end(), "}");
	}
}
