pub trait TextExpression {
	fn raw_text(&self) -> String {
		let mut buff = String::new();
		self.build_raw_text(&mut buff);
		buff
	}
	fn build_raw_text(&self, buff: &mut String);
}

#[cfg(test)]
pub mod test_helper {
	use chrono::prelude::*;
	use std::fs::File;
	use std::io::prelude::*;

	pub fn dump_to_file(text: &str) {
		let path = format!("../artifact/{}.txt", Local::now().format("%b_%d_%H_%M"));
		let mut file = File::create(&path).unwrap();

		_ = file.write_all(text.as_bytes()).unwrap();
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	struct Dummy;

	impl TextExpression for Dummy {
		fn build_raw_text(&self, buff: &mut String) {
			buff.push_str("hello");
		}
	}

	#[test]
	fn raw_text() {
		let mock = Dummy;
		assert_eq!(mock.raw_text(), "hello");
	}
}
