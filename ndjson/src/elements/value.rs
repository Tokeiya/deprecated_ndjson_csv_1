use super::with_raw_text::WithRawText;
use crate::elements::array_value::ArrayValue;
use crate::elements::null_value::NullValue;
use crate::elements::number_value::Number;
use crate::elements::object_value_element::ObjectValueElement;
use crate::elements::parse_number_error::ParseNumberError;
use crate::elements::string_value::StringValue;
use crate::elements::text_presentation::TextPresentation;
use std::collections::HashMap;
pub type BooleanValue = WithRawText<bool>;

pub type NumberValue = WithRawText<Result<Number, ParseNumberError>>;

pub type ObjectValue = HashMap<StringValue, ObjectValueElement>;
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
	fn raw(&self) -> &str {
		todo!()
	}

	fn trimmed(&self) -> &str {
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

		pub fn extract_array(&self) -> &[Value] {
			let Value::Array(a) = self else {
				unreachable!()
			};
			a
		}
	}
}
