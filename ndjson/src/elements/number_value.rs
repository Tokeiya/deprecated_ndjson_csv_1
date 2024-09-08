use super::with_raw_text::WithRawText;

pub type NumberValue = WithRawText<Number>;

pub enum Number {
	Integer(i128),
	Float(f64),
}

impl From<i128> for Number {
	fn from(value: i128) -> Self {
		Number::Integer(value)
	}
}

impl From<f64> for Number {
	fn from(value: f64) -> Self {
		Number::Float(value)
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

#[cfg(test)]
mod tests {
	use super::test_helper;
	use super::*;
	#[test]
	fn from_i128() {
		let fixture = Number::from(42);
		fixture.is_integer(42)
	}

	#[test]
	fn from_f64() {
		let fixture = Number::from(std::f64::consts::PI);
		fixture.is_float(std::f64::consts::PI);
	}
}
