use super::number::Number;
use super::text_presentation::TextPresentation;

pub struct NumberValue {
	number: Number,
	raw_string: String,
}

impl NumberValue {
	pub fn new(number: Number, raw_string: String) -> Self {
		todo!()
	}

	pub fn value(&self) -> &Number {
		todo!()
	}
}

impl TextPresentation for NumberValue {
	fn build_raw(&self, buffer: &mut String) {
		todo!()
	}

	fn build_trimmed(&self, buffer: &mut String) {
		todo!()
	}
}

#[cfg(test)]
mod test {}
