use super::with_raw_text::WithRawText;
use crate::elements::array_value::ArrayValue;
use crate::elements::null_value::NullValue;
use crate::elements::number_value::NumberValue;
use crate::elements::object_value::ObjectValue;
use crate::elements::text_expression::TextExpression;
pub type StringValue = WithRawText<String>;
pub type BooleanValue = WithRawText<bool>;

pub enum Value {
	Boolean(BooleanValue),
	Null(NullValue),
	String(StringValue),
	Number(NumberValue),
	Array(ArrayValue),
	Object(ObjectValue),
}

impl Value {
	pub fn raw_string(&self) -> String {
		match self {
			Value::Boolean(b) => b.raw_text(),
			Value::Null(n) => n.raw_text().to_string(),
			Value::String(s) => s.raw_text().to_string(),
			Value::Number(n) => n.raw_text().to_string(),
			Value::Array(arr) => arr.raw_text(),
			Value::Object(obj) => obj.raw_string(),
		}
	}
}

impl From<BooleanValue> for Value {
	fn from(value: BooleanValue) -> Self {
		Value::Boolean(value)
	}
}

impl From<NullValue> for Value {
	fn from(value: NullValue) -> Self {
		Value::Null(value)
	}
}

impl From<StringValue> for Value {
	fn from(value: StringValue) -> Self {
		Value::String(value)
	}
}

impl From<NumberValue> for Value {
	fn from(value: NumberValue) -> Self {
		Value::Number(value)
	}
}

impl From<ArrayValue> for Value {
	fn from(value: ArrayValue) -> Self {
		Value::Array(value)
	}
}

impl From<ObjectValue> for Value {
	fn from(value: ObjectValue) -> Self {
		Value::Object(value)
	}
}

impl TextExpression for Value {
	fn build_raw_text(&self, buff: &mut String) {
		match self {
			Value::Boolean(b) => b.build_raw_text(buff),
			Value::Null(n) => n.build_raw_text(buff),
			Value::String(s) => s.build_raw_text(buff),
			Value::Number(n) => n.build_raw_text(buff),
			Value::Array(a) => a.build_raw_text(buff),
			Value::Object(o) => o.build_raw_text(buff),
		}
	}
}

#[cfg(test)]
pub mod test_helper {
	use super::*;

	pub fn add_spaces(target: &str) -> String {
		format!("\t \r  \t \n   {target}   \r\n")
	}

	impl Value {
		pub fn extract_bool(&self) -> &BooleanValue {
			let Value::Boolean(b) = self else {
				unreachable!()
			};
			b
		}

		pub fn extract_null(&self) -> &NullValue {
			let Value::Null(n) = self else { unreachable!() };
			n
		}

		pub fn extract_string(&self) -> &StringValue {
			let Value::String(s) = self else {
				unreachable!()
			};
			s
		}

		pub fn extract_number(&self) -> &NumberValue {
			let Value::Number(n) = self else {
				unreachable!()
			};
			n
		}

		pub fn extract_array(&self) -> &ArrayValue {
			let Value::Array(a) = self else {
				unreachable!()
			};
			a
		}

		pub fn extract_object(&self) -> &ObjectValue {
			let Value::Object(obj) = self else {
				unreachable!()
			};
			obj
		}
	}
}

#[cfg(test)]
mod tests {
	use super::super::number_value::from_i128;
	use super::super::number_value::test_helper::is_integer;
	use super::Value;
	use super::*;
	use crate::elements::key_value::KeyValue;
	use crate::elements::null_value::NullValue;
	use crate::elements::number_value::Number;
	use crate::elements::text_expression::test_helper::assert_text_expression;
	#[test]
	fn raw_string() {
		let value = Value::from(NullValue::from("null"));
		assert_eq!(value.raw_string(), "null");

		let value = Value::from(BooleanValue::new(true, "true".to_string()));
		assert_eq!(value.raw_string(), "true");

		let value = Value::from(StringValue::new(
			"hello world".to_string(),
			r#""hello world""#.to_string(),
		));
		assert_eq!(value.raw_string(), r#""hello world""#);

		let value = Value::from(NumberValue::new(Ok(Number::from(42)), "42".to_string()));
		assert_eq!(value.raw_string(), "42");

		let mut array = Vec::<Value>::new();
		array.push(Value::from(BooleanValue::new(true, "true".to_string())));
		array.push(Value::from(StringValue::new(
			"hello".to_string(),
			"\"hello\"".to_string(),
		)));
		array.push(Value::from(NumberValue::new(
			Result::Ok(Number::from(42.195)),
			"42.195".to_string(),
		)));

		array.push(Value::Null(NullValue::from("null")));

		let value = Value::from(ArrayValue::new(array, "[".to_string(), "]".to_string()));

		assert_text_expression(&value, r#"[true,"hello",42.195,null]"#);
	}

	#[test]
	fn bool() {
		let value = Value::from(BooleanValue::new(true, "   true    ".to_string()));

		assert_eq!(value.extract_bool().value(), &true);

		let value = Value::from(BooleanValue::new(false, "   false    ".to_string()));

		assert_eq!(value.extract_bool().value(), &false);
	}

	#[test]
	fn null() {
		let value = Value::from(NullValue::from("   null    ".to_string()));
		_ = value.extract_null();
	}

	#[test]
	fn string() {
		let value = Value::from(StringValue::new(
			"hello".to_string(),
			"   \"hello\"    ".to_string(),
		));

		assert_eq!(value.extract_string().value(), "hello");
	}

	#[test]
	fn number() {
		let value = Value::from(NumberValue::new(
			Ok(Number::from(123)),
			"   123    ".to_string(),
		));

		let fixture = value.extract_number().value();
		let Result::Ok(num) = fixture else {
			unreachable!()
		};
		num.is_integer(123)
	}

	#[test]
	fn array() {
		let array = vec![
			Value::from(BooleanValue::new(true, "true".to_string())),
			Value::from(StringValue::new(
				"hello".to_string(),
				"\"hello\"".to_string(),
			)),
			Value::from(NumberValue::new(
				Result::Ok(Number::from(42.195)),
				"42.195".to_string(),
			)),
			Value::Null(NullValue::from("null")),
		];

		let fixture = Value::from(ArrayValue::new(array, "[".to_string(), "]".to_string()));

		assert_eq!(fixture.extract_array().contents().len(), 4);

		let piv = fixture.extract_array().contents()[0].extract_bool();

		assert_text_expression(piv, "true");
		assert_eq!(piv.value(), &true);

		let piv = fixture.extract_array().contents()[1].extract_string();
		assert_eq!(piv.raw_text(), r#""hello""#);

		_ = fixture.extract_array().contents()[2].extract_number();
		_ = fixture.extract_array().contents()[3].extract_null();
	}

	#[test]
	fn object() {
		let vec = vec![
			KeyValue::new(
				StringValue::new("key1".to_string(), r#""key1""#.to_string()),
				Value::from(from_i128(42, "42".to_string())),
			),
			KeyValue::new(
				StringValue::new("key2".to_string(), r#"key2"#.to_string()),
				Value::from(NullValue::from("null")),
			),
		];

		let obj = ObjectValue::new(vec, " { ".to_string(), "}  ".to_string());

		let value = Value::from(obj);

		let fixture = value.extract_object();
		assert_eq!(fixture.begin(), " { ");
		assert_eq!(fixture.end(), "}  ");

		assert_eq!(fixture.content().len(), 2);

		assert_eq!(fixture.content()[0].key().value(), "key1");
		is_integer(fixture.content()[0].value().extract_number(), 42);

		assert_eq!(fixture.content()[1].key().value(), "key2");
		_ = fixture.content()[1].value().extract_null();
	}
}
