use super::text_presentation::TextPresentation;
use super::value::Value;
use std::vec::Vec;
pub type ArrayValue = Vec<Value>;

impl TextPresentation for ArrayValue {
	fn build_raw(&self, buffer: &mut String) {
		todo!()
	}

	fn build_trimmed(&self, buffer: &mut String) {
		todo!()
	}
}

impl TextPresentation for &[Value] {
	fn build_raw(&self, buffer: &mut String) {
		todo!()
	}

	fn build_trimmed(&self, buffer: &mut String) {
		todo!()
	}
}
