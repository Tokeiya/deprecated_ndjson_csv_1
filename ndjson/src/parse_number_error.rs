use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::num::{ParseFloatError, ParseIntError};

pub enum ParseNumberError {
	Integer(ParseIntError),
	Float(ParseFloatError),
}

impl ParseNumberError {
	fn format(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			ParseNumberError::Integer(e) => write!(f, "Integer:{}", e.to_string()),
			ParseNumberError::Float(e) => write!(f, "Float:{}", e.to_string()),
		}
	}
}

impl Debug for ParseNumberError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.format(f)
	}
}

impl Display for ParseNumberError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.format(f)
	}
}

impl Error for ParseNumberError {}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn integer() {
		let err = "asdf".parse::<i32>().err().unwrap();
		let err = ParseNumberError::Integer(err);

		assert_eq!(format!("{}", err), "Integer:invalid digit found in string");
		assert_eq!(
			format!("{:?}", err),
			"Integer:invalid digit found in string"
		);
	}

	#[test]
	fn real() {
		let err = "asdf".parse::<f64>().err().unwrap();
		let err = ParseNumberError::Float(err);

		assert_eq!(format!("{}", err), "Float:invalid float literal");
		assert_eq!(format!("{:?}", err), "Float:invalid float literal");
	}
}
