use super::with_raw_text::WithRawText;
use std::hash::{Hash, Hasher};
//pub struct StringValue(String);
pub type StringValue = WithRawText<String>;

impl Hash for StringValue {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.value().hash(state)
	}
}

impl PartialEq<Self> for StringValue {
	fn eq(&self, other: &Self) -> bool {
		self.value() == other.value()
	}
}

impl Eq for StringValue {}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn eq() {
		let a = StringValue::new("key".to_string(), r#"     "key"        "#.to_string());
		let b = StringValue::new("key".to_string(), r#""key""#.to_string());
		assert!(a == b)
	}
}
