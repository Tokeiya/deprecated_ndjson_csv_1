use super::with_raw_text::WithRawText;
use crate::elements::parse_number_error::ParseNumberError;

pub type NumberValue = WithRawText<Result<Number, ParseNumberError>>;

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

pub fn from_i128(value: i128, raw_text: String) -> NumberValue {
	NumberValue::new(Ok(Number::Integer(value)), raw_text)
}

pub fn from_f64(value: f64, raw_text: String) -> NumberValue {
	NumberValue::new(Ok(Number::from(value)), raw_text)
}

pub fn from_error(err: ParseNumberError, raw_text: String) -> NumberValue {
	NumberValue::new(Err(err), raw_text)
}

#[cfg(test)]
pub mod test_helper {
	use super::*;

	pub fn is_integer(actual: &NumberValue, expected: i128) {
		let Ok(num) = actual.value() else {
			unreachable!()
		};
		num.is_integer(expected)
	}

	pub fn is_float(actual: &NumberValue, expected: f64) {
		let Ok(num) = actual.value() else {
			unreachable!()
		};
		num.is_float(expected)
	}

	pub fn is_int_error(actual: &NumberValue) {
		let Err(e) = actual.value() else {
			unreachable!()
		};
		assert!(matches!(e, ParseNumberError::Integer(_)))
	}

	pub fn is_float_error(actual: &NumberValue) {
		let Err(e) = actual.value() else {
			unreachable!()
		};
		assert!(matches!(e, ParseNumberError::Float(_)))
	}
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
	use super::super::value::test_helper::add_spaces;
	use super::test_helper as num;
	use super::*;
	use crate::elements::value;
	#[test]
	fn from_int_i128() {
		let fixture = Number::from(42);
		fixture.is_integer(42)
	}

	#[test]
	fn from_ext_i128() {
		let raw = add_spaces("42");
		let fixture = from_i128(42, raw.clone());

		num::is_integer(&fixture, 42);
	}

	#[test]
	fn from_ext_f64() {
		let raw = add_spaces("42.195");
		let fixture = from_f64(42.195, raw.clone());

		num::is_float(&fixture, 42.195);
	}

	#[test]
	fn from_int_error() {
		let err = "999999999999999999999999999999999999999999999999999"
			.parse::<i128>()
			.err()
			.unwrap();
		let raw = add_spaces("999999999999999999999999999999999999999999999999999");
		let fixture = from_error(ParseNumberError::Integer(err), raw.clone());

		num::is_int_error(&fixture);
	}

	#[test]
	fn from_float_error() {
		let err = "a".parse::<f64>().err().unwrap();
		let raw = add_spaces("a");
		let fixture = from_error(ParseNumberError::Float(err), raw.clone());

		num::is_float_error(&fixture);
	}

	#[test]
	fn from_int_f64() {
		let fixture = Number::from(std::f64::consts::PI);
		fixture.is_float(std::f64::consts::PI);
	}
}
