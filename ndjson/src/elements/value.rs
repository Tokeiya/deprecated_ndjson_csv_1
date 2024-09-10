use super::with_raw_text::WithRawText;
use crate::elements::array_value::ArrayValue;
use crate::elements::null_value::NullValue;
use crate::elements::number_value::{Number, NumberValue};
use crate::elements::object_value::ObjectValue;
use crate::elements::string_value::StringValue;

pub type BooleanValue = WithRawText<bool>;

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
	}
}

#[cfg(test)]
mod tests {
	use super::Value;
	use super::*;
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
	}

	#[test]
	fn string() {
		let value = Value::from(StringValue::from("   \"hello\"    ".to_string()));

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

//todo:move to with_raw_text module.
