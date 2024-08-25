pub struct WithRawValue<T> {
	raw: String,
	value: T,
}

impl<T> WithRawValue<T> {
	pub fn new(raw: &str, value: T) -> Self {
		WithRawValue {
			raw: raw.to_string(),
			value,
		}
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
	fn new() {
		let fixture = WithRawValue::new("1", 1i32);
		assert_eq!(fixture.value, 1);
		assert_eq!(&fixture.raw, "1");
	}

	#[test]
	fn raw() {
		let fixture = WithRawValue::new("42", 42);
		assert_eq!(fixture.raw(), "42");
	}

	#[test]
	fn value() {
		let fixture = WithRawValue::new("52", 52);
		assert_eq!(fixture.value(), &52)
	}
}
