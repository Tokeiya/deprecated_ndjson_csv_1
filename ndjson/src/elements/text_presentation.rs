pub trait TextPresentation {
	fn raw_text(&self) -> &str;
	fn trimmed_text(&self) -> &str;
}

#[cfg(test)]
pub mod test_helper {
	use super::*;
	impl dyn TextPresentation {
		pub fn assert_raw(&self, expected: &str) {
			assert_eq!(self.raw_text(), expected)
		}

		pub fn assert_trimmed(&self, expected: &str) {
			assert_eq!(self.trimmed_text(), expected)
		}
	}
}
