use super::with_raw_text::WithRawText;

pub type NumberValue = WithRawText<Number>;

pub enum Number {
	Integer(i128),
	Float(f64),
}

impl From<i128> for Number {
	fn from(value: i128) -> Self {
		todo!()
	}
}

impl From<f64> for Number {
	fn from(value: f64) -> Self {
		todo!()
	}
}

#[cfg(test)]
mod test_helper {
	use super::*;

	impl Number {
		pub fn is_integer(&self, expected: i128) {
			assert!(matches!(self, Number::Integer(value) if value == &expected));
		}

		pub fn is_float(&self, expected: f64) {
			assert!(matches!(self, Number::Float(value) if value == &expected));
		}
	}
}
