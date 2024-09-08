use std::hash::{Hash, Hasher};
use super::text_presentation::TextPresentation;
pub struct StringValue(String);

impl From<String> for StringValue {
	fn from(value: String) -> Self {
		StringValue(value)
	}
}

impl From<&str> for StringValue {
	fn from(value: &str) -> Self {
		StringValue(value.to_string())
	}
}

impl StringValue {
	pub fn value(&self) -> &str {
		let a = self.trimmed_text();
		println!("trim:{}", a);
		a.trim_matches('"')
	}
}

impl TextPresentation for StringValue {
	fn raw_text(&self) -> &str {
		self.0.as_str()
	}

	fn trimmed_text(&self) -> &str {
		self.0.trim()
	}
}

impl Hash for StringValue {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.value().hash(state)
	}
}

#[cfg(test)]
mod test {
	use std::hash::DefaultHasher;
	use super::*;
	use super::super::text_presentation::test_helper::*;


	#[test]
	fn from_string() {
		let fixture = StringValue::from(r#""Hello, World!""#.to_string());
		assert_eq!(fixture.value(), r#"Hello, World!"#);
	}
	#[test]
	fn from_str() {
		let fixture = StringValue::from(r#""Hello, World!""#);
		assert_eq!(fixture.value(), r#"Hello, World!"#);
	}

	#[test]
	fn text_presentation() {
		let expected = add_spaces(r#""Hello,       World!""#);
		let fixture = StringValue::from(expected.as_str());

		assert_raw(&fixture, expected.as_str());
		assert_trimmed(&fixture, r#""Hello,       World!""#);
	}

	#[test]
	fn value() {
		let fixture = StringValue::from("   \n   \"hello world!\"    \t   \r     ");
		assert_eq!(fixture.value(), "hello world!")
	}
	#[test]
	fn hash() {
		let fixture = StringValue::from("   \n   \"hello world!\"    \t   \r     ");
		let mut hasher = DefaultHasher::new();

		fixture.hash(&mut hasher);

		let mut expected = DefaultHasher::new();
		"hello world!".hash(&mut expected);
		assert_eq!(hasher.finish(), expected.finish())
	}
}