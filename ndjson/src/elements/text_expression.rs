pub trait TextExpression {
	fn raw_text(&self) -> String;
	fn build_raw_text(&self, buff: &mut String);
}

#[cfg(test)]
pub mod test_helper {
	use chrono::prelude::*;
	pub fn dump_to_file(text: &str) {}
}