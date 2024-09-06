pub trait TextPresentation {
	fn raw(&self) -> String {
		let mut buffer = String::new();
		self.build_raw(&mut buffer);
		buffer
	}
	fn trimmed(&self) -> String {
		let mut buffer = String::new();
		self.build_trimmed(&mut buffer);
		buffer
	}

	fn build_raw(&self, buffer: &mut String);

	fn build_trimmed(&self, buffer: &mut String);
}

#[cfg(test)]
pub mod test_helper {
	use super::*;

	pub fn assert_raw(actual: &impl TextPresentation, expected: &str) {
		let mut buff = String::new();
		actual.build_raw(&mut buff);

		assert_eq!(buff, expected);

		let fixture = actual.raw();
		assert_eq!(fixture, expected);
	}

	pub fn assert_trimmed(actual: &impl TextPresentation, expected: &str) {
		let mut buff = String::new();
		actual.build_trimmed(&mut buff);

		assert_eq!(buff, expected);
		assert_eq!(actual.trimmed(), expected)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use mockall::mock;

	mock! {
		Dummy{}
		impl TextPresentation for Dummy{
			fn build_raw(&self, buffer: &mut String);
			fn build_trimmed(&self, buffer: &mut String);
		}
	}

	#[test]
	fn raw() {
		let mut dummy = MockDummy::new();
		dummy.expect_build_raw().returning_st(|x| {
			x.push_str("hello")
		});

		assert_eq!(dummy.raw(), "hello".to_string())
	}

	#[test]
	fn trimmed() {
		let mut dummy = MockDummy::new();

		dummy.expect_build_trimmed().returning_st(|x| {
			x.push_str("world")
		});

		assert_eq!(dummy.trimmed(), "world");
	}
}
