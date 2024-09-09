pub struct WithRawText<T> {
	value: T,
	raw_text: String,
}
impl<T> WithRawText<T> {
	pub fn new(value: T, raw_text: String) -> Self {
		Self { value, raw_text }
	}

	pub fn value(&self) -> &T {
		&self.value
	}

	pub fn raw_text(&self) -> &str {
		todo!()
	}
}

#[cfg(test)]
pub mod test_helper {
	use super::*;

	impl<T> WithRawText<T> {
		pub fn assert_raw(&self, expected: &str) {
			assert_eq!(self.raw_text, expected)
		}
	}
}

#[cfg(test)]
mod test {
	use super::super::text_presentation::test_helper::*;
	use super::*;

	#[test]
	fn from_string() {
		let fixture = WithRawText::new(20, 20.to_string());
		assert_eq!(fixture.value, 20);
		assert_eq!(fixture.raw_text, "20");
	}

	#[test]
	fn value() {
		let fixture = WithRawText::new(20, 20.to_string());
		assert_eq!(fixture.value(), &20);
	}
}
