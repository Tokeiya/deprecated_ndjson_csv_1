use super::value::Value;

pub enum ObjectValueElement {
	Single(Value),
	Many(Vec<Value>),
}

impl From<Value> for ObjectValueElement {
	fn from(value: Value) -> Self {
		Self::Single(value)
	}
}

impl From<Vec<Value>> for ObjectValueElement {
	fn from(values: Vec<Value>) -> Self {
		Self::Many(values)
	}
}

#[cfg(test)]
pub mod test_helper {
	use super::*;

	impl ObjectValueElement {
		pub fn extract_single(&self) -> &Value {
			let ObjectValueElement::Single(v) = self else { unreachable!() };
			v
		}

		pub fn extract_multi(&self) -> &[Value] {
			let ObjectValueElement::Many(v) = self else { unreachable!() };
			v
		}
	}
}

#[cfg(test)]
mod test {
	use super::super::number_value as num;
	use super::super::number_value::test_helper as num_helper;
	use super::*;
	use crate::elements::null_value::NullValue;

	#[test]
	fn from_single() {
		let value = Value::Null(NullValue::from("null"));
		let fixture = ObjectValueElement::from(value);

		_ = fixture.extract_single().extract_null()
	}

	#[test]
	fn from_multi() {
		let mut vec = Vec::new();
		vec.push(Value::from(num::from_i128(42, "42".to_string())));
		vec.push(Value::from(num::from_f64(42.195, "42.195".to_string())));

		let fixture = ObjectValueElement::from(vec);
		num_helper::is_integer(fixture.extract_multi()[0].extract_number(), 42);
		num_helper::is_float(fixture.extract_multi()[1].extract_number(), 42.195);
	}
}