pub trait TextPresentation {
	fn build_raw(&self, buffer: &mut String);
	fn raw(&self) -> String {
		todo!()
	}
	fn build_trimmed(&self, buffer: &mut String);
	fn trimmed(&self) -> String {
		todo!()
	}
}

#[cfg(test)]
pub mod test_helper {
	use super::*;

	pub fn assert_raw(actual: &dyn TextPresentation, expected: &str) {
		let mut buff = String::new();
		actual.build_raw(&mut buff);
		assert_eq!(buff, expected);

		assert_eq!(actual.raw(), expected);
	}

	pub fn assert_trimmed(actual: &dyn TextPresentation, expected: &str) {
		let mut buff = String::new();
		actual.build_trimmed(&mut buff);
		assert_eq!(buff, expected);
		assert_eq!(actual.trimmed(), expected);
	}

	pub fn add_spaces(target: &str) -> String {
		format!("\t \r  \t \n   {target}   \r\n")
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use mockall::mock;

	mock! {
			Dummy{}
		impl TextPresentation for Dummy {
			fn build_raw(&self, buffer: &mut String);
			fn build_trimmed(&self, buffer: &mut String);
		}

	}

	#[test]
	fn raw() {
		let mut mock = MockDummy::new();
		mock.expect_build_trimmed().returning(|buff| {
			buff.push_str("called")
		});
	}
}
