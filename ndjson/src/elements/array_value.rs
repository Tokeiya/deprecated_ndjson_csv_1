use super::text_presentation::TextPresentation;
use super::value::Value;
use std::vec::Vec;
pub type ArrayValue = Vec<Value>;

impl TextPresentation for ArrayValue {
	fn raw(&self) -> &str {
		todo!()
	}

	fn trimmed(&self) -> &str {
		todo!()
	}
}

impl TextPresentation for &[Value] {
	fn raw(&self) -> &str {
		todo!()
	}

	fn trimmed(&self) -> &str {
		todo!()
	}
}
