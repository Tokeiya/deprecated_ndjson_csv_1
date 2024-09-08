use super::text_presentation::TextPresentation;

pub struct WithRawText<T> {
	value: T,
	raw_text: String,
}

impl<T> WithRawText<T> {
	pub fn new(value: T, raw_text: String) -> Self {
		todo!()
	}

	pub fn value(&self) -> &T {
		todo!()
	}
}

impl<T> TextPresentation for WithRawText<T> {
	fn raw_text(&self) -> &str {
		todo!()
	}

	fn trimmed_text(&self) -> &str {
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
		pub fn assert_trimmed(&self, expected: &str) {
			assert_eq!(self.trimmed_text(), expected)
		}
	}
}
