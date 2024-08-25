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
