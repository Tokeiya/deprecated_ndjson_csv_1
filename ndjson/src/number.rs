use super::parse_number_error::ParseNumberError;
use std::num::{ParseFloatError, ParseIntError};
#[derive(Debug)]
pub enum Number {
	Integer(i128),
	Float(f64),
	Error(ParseNumberError),
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

impl From<ParseIntError> for Number {
	fn from(value: ParseIntError) -> Self {
		Number::Error(ParseNumberError::Integer(value))
	}
}

impl From<ParseFloatError> for Number {
	fn from(value: ParseFloatError) -> Self {
		Number::Error(ParseNumberError::Float(value))
	}
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn from_i128() {
		let fixture = Number::from(42);
		assert!(matches!(fixture,Number::Integer(i) if i==42));

		let fixture = Number::from(-42);
		assert!(matches!(fixture,Number::Integer(i) if i==-42));
	}

	#[test]
	fn from_f64() {
		let fixture = Number::from(std::f64::consts::PI);
		assert!(matches!(fixture, Number::Float(std::f64::consts::PI)));
	}

	#[test]
	fn from_parse_int_error() {
		let err = "a".parse::<i128>().err().unwrap();
		let fixture = Number::from(err);

		assert!(matches!(fixture,Number::Error(e) if matches!(&e,ParseNumberError::Integer(_))))
	}

	#[test]
	fn from_parse_float_error() {
		let err = "a".parse::<f64>().err().unwrap();
		let fixture = Number::from(err);

		assert!(matches!(fixture,Number::Error(e) if matches!(&e,ParseNumberError::Float(_))))
	}
}
