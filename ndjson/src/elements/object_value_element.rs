use super::string_value::StringValue;
use super::value::Value;
use std::collections::HashMap;

pub enum ObjectValueElement {
	Single(Value),
	Many(Vec<Value>),
}
