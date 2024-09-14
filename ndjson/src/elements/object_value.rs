use crate::elements::object_value_element::ObjectValueElement;
use crate::elements::value::StringValue;
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
	use super::super::value::StringValue;
	use super::super::value::Value;
	use super::*;
	use std::collections::HashMap;

	#[test]
	fn raw_string() {
		todo!()
	}

	#[test]
	fn new() {
		todo!()
	}

	#[test]
	fn content() {
		todo!()
	}

	#[test]
	fn begin_end() {
		let fixture = ObjectValue::new(HashMap::new(), "{".to_string(), "}".to_string());

		assert_eq!(fixture.begin(), "{");
		assert_eq!(fixture.end(), "}");
	}
}
