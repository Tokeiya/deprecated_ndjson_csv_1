pub trait TextPresentation {
	fn raw_text(&self) -> &str;
	fn trimmed_text(&self) -> &str;
}

#[cfg(test)]
pub mod test_helper {
	use super::*;
	impl dyn TextPresentation + '_ {
		pub fn assert_raw(&self, expected: &str) {
			assert_eq!(self.raw_text(), expected)
		}

		pub fn assert_trimmed(&self, expected: &str) {
			assert_eq!(self.trimmed_text(), expected)
		}

		pub fn assert_all(&self, expected_raw: &str) {
			self.assert_raw(expected_raw);
			self.assert_trimmed(expected_raw.trim())
		}
	}

	pub fn assert_raw(actual: &dyn TextPresentation, expected: &str) {
		actual.assert_raw(expected)
	}

	pub fn assert_trimmed(actual: &dyn TextPresentation, expected: &str) {
		actual.assert_trimmed(expected)
	}

	pub fn assert_all(actual: &dyn TextPresentation, expected: &str) {
		actual.assert_all(expected)
	}

	pub fn add_spaces(target: &str) -> String {
		format!("\t \r  \t \n   {target}   \r\n")
	}
}
