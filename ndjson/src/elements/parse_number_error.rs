use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::num::{ParseFloatError, ParseIntError};

pub enum ParseNumberError {
	Integer(ParseIntError),
	Float(ParseFloatError),
}

impl Debug for ParseNumberError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

impl Display for ParseNumberError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		todo!()
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
