use super::with_raw_text::WithRawText;
use crate::elements::array_value::ArrayValue;
use crate::elements::null_value::NullValue;
use crate::elements::number_value::Number;
use crate::elements::object_value::ObjectValue;
use crate::elements::parse_number_error::ParseNumberError;
use crate::elements::string_value::StringValue;
use crate::elements::text_presentation::TextPresentation;

pub type BooleanValue = WithRawText<bool>;

pub type NumberValue = WithRawText<Result<Number, ParseNumberError>>;

pub enum Value {
	Boolean(BooleanValue),
	Null(NullValue),
	String(StringValue),
	Number(NumberValue),
	Array(ArrayValue),
	Object(ObjectValue),
}

impl From<BooleanValue> for Value {
	fn from(value: BooleanValue) -> Self {
		todo!()
	}
}

impl From<NullValue> for Value {
	fn from(value: NullValue) -> Self {
		todo!()
	}
}

impl From<StringValue> for Value {
	fn from(value: StringValue) -> Self {
		todo!()
	}
}

impl From<NumberValue> for Value {
	fn from(value: NumberValue) -> Self {
		todo!()
	}
}

impl From<ArrayValue> for Value {
	fn from(value: ArrayValue) -> Self {
		todo!()
	}
}

impl From<ObjectValue> for Value {
	fn from(value: ObjectValue) -> Self {
		todo!()
	}
}

impl TextPresentation for Value {
	fn build_raw(&self, buffer: &mut String) {
		todo!()
	}

	fn build_trimmed(&self, buffer: &mut String) {
		todo!()
	}
}

#[cfg(test)]
pub mod test_helper {
	use super::*;

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
	}
}

#[cfg(test)]
mod tests {
	use super::super::number_value::test_helper;
	use super::super::text_presentation::test_helper::*;
	use super::test_helper as num_helper;
	use super::Value;
	use super::*;
	#[test]
	fn bool() {
		let value = Value::from(BooleanValue::new(true, "   true    ".to_string()));
		assert_raw(&value, "   true    ");
		assert_trimmed(&value, "true");

		assert_eq!(value.extract_bool().value(), &true);

		let value = Value::from(BooleanValue::new(false, "   false    ".to_string()));
		assert_raw(&value, "   false    ");
		assert_trimmed(&value, "false");

		assert_eq!(value.extract_bool().value(), &false);
	}

	#[test]
	fn null() {
		let value = Value::from(NullValue::from("   null    ".to_string()));
		assert_raw(&value, "   null    ");
		assert_trimmed(&value, "null");
	}

	#[test]
	fn string() {
		let value = Value::from(StringValue::from("   \"hello\"    ".to_string()));
		assert_raw(&value, "   \"hello\"    ");
		assert_trimmed(&value, "\"hello\"");

		assert_eq!(value.extract_string().value(), "hello");
	}

	#[test]
	fn number() {
		let value = Value::from(NumberValue::new(
			Ok(Number::from(123)),
			"   123    ".to_string(),
		));
		assert_raw(&value, "   123    ");
		assert_trimmed(&value, "123");

		let fixture = value.extract_number().value();
		let Result::Ok(num) = fixture else {
			unreachable!()
		};
		num.is_integer(123)
	}

	#[test]
	fn array() {
		let mut array = Vec::<Value>::new();
		array.push(Value::from(BooleanValue::new(
			true,
			"   true   ".to_string(),
		)));
		array.push(Value::from(StringValue::from("   \"hello\"    ")));
		array.push(Value::from(NumberValue::new(
			Result::Ok(Number::from(42.195)),
			"      42.195    ".to_string(),
		)));

		todo!()
	}
}
