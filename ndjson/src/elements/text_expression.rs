pub trait TextExpression {
	fn raw_text(&self) -> String;
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
