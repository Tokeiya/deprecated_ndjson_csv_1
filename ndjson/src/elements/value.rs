use super::with_raw_text::WithRawText;
use crate::elements::array_value::ArrayValue;
use crate::elements::null_value::NullValue;
use crate::elements::number_value::Number;
use crate::elements::object_key::ObjectKey;
use crate::elements::object_value_element::ObjectValueElement;
use crate::elements::parse_number_error::ParseNumberError;
use crate::elements::string_value::StringValue;
use crate::elements::text_presentation::TextPresentation;
use std::collections::HashMap;
pub type Value = WithRawText<TypedValue>;
pub type BooleanValue = WithRawText<bool>;

pub type NumberValue = WithRawText<Result<Number, ParseNumberError>>;

pub type ObjectValue = HashMap<ObjectKey, ObjectValueElement>;
pub enum TypedValue {
	Boolean(BooleanValue),
	Null(NullValue),
	String(StringValue),
	Number(NumberValue),
	Array(ArrayValue),
	Object(ObjectValue),
}

impl From<BooleanValue> for TypedValue {
	fn from(value: BooleanValue) -> Self {
		todo!()
	}
}

impl From<NullValue> for TypedValue {
	fn from(value: NullValue) -> Self {
		todo!()
	}
}

impl From<StringValue> for TypedValue {
	fn from(value: StringValue) -> Self {
		todo!()
	}
}

impl From<NumberValue> for TypedValue {
	fn from(value: NumberValue) -> Self {
		todo!()
	}
}

impl From<ArrayValue> for TypedValue {
	fn from(value: ArrayValue) -> Self {
		todo!()
	}
}

impl From<ObjectValue> for TypedValue {
	fn from(value: ObjectValue) -> Self {
		todo!()
	}
}

impl TextPresentation for TypedValue {
	fn raw_text(&self) -> &str {
		todo!()
	}

	fn trimmed_text(&self) -> &str {
		todo!()
	}
}
