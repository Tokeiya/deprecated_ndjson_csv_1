use crate::parser::with_raw_value::WithRawValue;
use crate::typed_value::TypedValue;
use combine::parser::char as chr;
use combine::{Parser, Stream};
pub fn boolean<I: Stream<Token = char>>() -> impl Parser<I, Output = WithRawValue<TypedValue>> {
	let t = chr::string("true").map(|_| WithRawValue::new_from_str("true", TypedValue::from(true)));
	let f =
		chr::string("false").map(|_| WithRawValue::new_from_str("false", TypedValue::from(false)));

	t.or(f)
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn true_test() {
		let mut parser = boolean::<&str>();

		let (act, rem) = parser.parse("true").unwrap();

		assert_eq!(rem, "");
		assert_eq!(act.raw(), "true");

		matches!(act.value(), TypedValue::Boolean(b) if b==&true);
	}

	#[test]
	fn false_test() {
		let mut parser = boolean::<&str>();

		let (act, rem) = parser.parse("false").unwrap();
		assert_eq!(rem, "");
		assert_eq!(act.raw(), "false");

		matches!(act.value(), TypedValue::Boolean(b) if b==&false);
	}

	#[test]
	fn invalid() {
		let mut parser = boolean::<&str>();

		assert!(parser.parse("True").is_err());
		assert!(parser.parse("False").is_err());

		assert!(parser.parse("TRUE").is_err());
		assert!(parser.parse("FALSE").is_err());
	}
}
