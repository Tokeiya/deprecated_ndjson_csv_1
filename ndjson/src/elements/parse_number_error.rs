use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::num::{ParseFloatError, ParseIntError};

pub enum ParseNumberError {
	Integer(ParseIntError),
	Float(ParseFloatError),
}

impl From<ParseFloatError> for ParseNumberError {
	fn from(value: ParseFloatError) -> Self {
		ParseNumberError::Float(value)
	}
}

impl From<ParseIntError> for ParseNumberError {
	fn from(value: ParseIntError) -> Self {
		ParseNumberError::Integer(value)
	}
}

impl Debug for ParseNumberError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			ParseNumberError::Integer(i) => write!(f, "{:?}", i),
			ParseNumberError::Float(fe) => write!(f, "{:?}", fe),
		}
	}
}

impl Display for ParseNumberError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			ParseNumberError::Integer(ie) => write!(f, "{}", ie),
			ParseNumberError::Float(fe) => write!(f, "{}", fe),
		}
	}
}

impl Error for ParseNumberError {}

#[cfg(test)]
pub mod test_helper {
	use super::*;

	impl ParseNumberError {
		pub fn is_integer(&self) {
			assert!(matches!(self, ParseNumberError::Integer(_)));
		}

		pub fn is_float(&self) {
			assert!(matches!(self, ParseNumberError::Float(_)));
		}
	}
}


#[cfg(test)]
mod test {
	use super::*;
	use super::test_helper;

	#[test]
	fn from_int_parse_error() {
		let err = "a".parse::<i128>().err().unwrap();
		let act = ParseNumberError::from(err);

		act.is_integer();
	}

	#[test]
	fn from_float_parse_error() {
		let err = "a".parse::<f64>().err().unwrap();
		let act = ParseNumberError::from(err);

		act.is_float();
	}

	#[test]
	fn debug() {
		let err = "a".parse::<i128>().err().unwrap();
		let act = ParseNumberError::from(err);
		assert_eq!(format!("{:?}", act), "ParseIntError { kind: InvalidDigit }");

		let err = "a".parse::<f64>().err().unwrap();
		let act = ParseNumberError::from(err);
		assert_eq!(format!("{:?}", act), "ParseFloatError { kind: Invalid }");
	}

	#[test]
	fn display() {
		let err = "a".parse::<i128>().err().unwrap();
		let act = ParseNumberError::from(err);
		assert_eq!(format!("{}", act), "invalid digit found in string");

		let err = "a".parse::<f64>().err().unwrap();
		let act = ParseNumberError::from(err);
		assert_eq!(format!("{}", act), "invalid float literal");
	}
}