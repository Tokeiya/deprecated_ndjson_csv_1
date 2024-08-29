use crate::parser::with_raw_value::WithRawValue;
use combine::parser::char::string;
use combine::{Parser, Stream};

pub struct Null;

pub fn null<I: Stream<Token = char>>() -> impl Parser<I, Output = WithRawValue<Null>> {
	string::<I>("null").map(|_| WithRawValue::new_from_str("null", Null))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn null_test() {
		let mut parser = null::<&str>();

		let (act, rem) = parser.parse("null").unwrap();
		assert_eq!(rem, "");
		assert_eq!(act.raw(), "null");
	}

	#[test]
	fn invalid_null() {
		let mut parser = null::<&str>();
		assert!(parser.parse("Null").is_err());
		assert!(parser.parse("NULL").is_err());
	}
}
