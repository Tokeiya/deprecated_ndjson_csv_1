use super::number::Number;
use super::value::Value;
use super::with_raw_value::WithRawValue;

pub struct Member {
	name: String,
	value: WithRawValue<Value>,
}

impl Member {
	pub fn new(name: String, value: WithRawValue<Value>) -> Self {
		Member {
			name,
			value,
		}
	}

	pub fn name(&self) -> &str {
		&self.name
	}

	pub fn value(&self) -> &WithRawValue<Value> {
		&self.value
	}
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn new_test() {
		let fixture = Member::new(
			"foo".to_string(),
			WithRawValue::new_from_str("42", Value::from(Number::from(42))),
		);

		assert_eq!(&fixture.name, "foo");
		assert!(
			matches!(&fixture.value.value(),Value::Number(num) if matches!(num,Number::Integer(i) if i==&42))
		);


		assert_eq!(fixture.value.raw(), "42");
	}

	#[test]
	fn name_test() {
		let fixture = Member::new(
			"foo".to_string(),
			WithRawValue::new_from_str("null", Value::Null),
		);

		assert_eq!(fixture.name(), "foo")
	}

	#[test]
	fn value_test() {
		let fixture = Member::new(
			"foo".to_string(),
			WithRawValue::new_from_str("null", Value::Null),
		);

		assert!(matches!(fixture.value().value(),Value::Null));
		assert_eq!(fixture.value().raw(), "null");
	}
}
