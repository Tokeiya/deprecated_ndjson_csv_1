pub struct WithRawValue<T> {
	raw: String,
	value: T,
}

impl<T> WithRawValue<T> {
	pub fn new_from_str(raw: &str, value: T) -> Self {
		WithRawValue {
			raw: raw.to_string(),
			value,
		}
	}

	pub fn new_from_string(raw: String, value: T) -> Self {
		WithRawValue { raw, value }
	}

	pub fn raw(&self) -> &str {
		self.raw.as_str()
	}

	pub fn value(&self) -> &T {
		&self.value
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn new_from_str() {
		let fixture = WithRawValue::new_from_str("1", 1i32);
		assert_eq!(fixture.value, 1);
		assert_eq!(&fixture.raw, "1");
	}

	#[test]
	fn new_from_string() {
		let fixture = WithRawValue::new_from_string("42".to_string(), 42i32);

		assert_eq!(fixture.value, 42i32);
		assert_eq!(fixture.raw, "42")
	}

	#[test]
	fn raw() {
		let fixture = WithRawValue::new_from_str("42", 42);
		assert_eq!(fixture.raw(), "42");
	}

	#[test]
	fn value() {
		let fixture = WithRawValue::new_from_str("52", 52);
		assert_eq!(fixture.value(), &52)
	}
}
