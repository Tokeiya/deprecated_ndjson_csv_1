use super::super::elements::value::Value as ElemValue;
use combine::parser::char as chr;
use combine::{self as cmb, Parser, Stream};
pub fn value<I: Stream<Token = char>>() -> impl Parser<I, Output = ElemValue> {
	chr::char('a').map(|_| todo!())
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::elements::number_value::test_helper::{is_float, is_integer};
	use crate::elements::StringValue;

	#[test]
	fn bool() {
		let mut parser = value::<&str>();
		let (b, _) = parser.parse("true").unwrap();
		assert!(b.extract_bool().value())
	}

	#[test]
	fn null() {
		let mut parser = value::<&str>();
		let (n, _) = parser.parse("null").unwrap();
		_ = n.extract_null();
	}

	#[test]
	fn string() {
		let mut parser = value::<&str>();
		let (s, _) = parser.parse(r#""hello""#).unwrap();
		assert_eq!(s.extract_string().value(), "hello")
	}

	#[test]
	fn number() {
		let mut parser = value::<&str>();
		let (n, _) = parser.parse("42").unwrap();
		is_integer(n.extract_number(), 42);

		let (n, _) = parser.parse("42.0").unwrap();
		is_float(n.extract_number(), 42.0);
	}

	#[test]
	fn array() {
		let mut parser = value::<&str>();
		let (a, _) = parser.parse(r#"[1,2,3]"#).unwrap();
		let a = a.extract_array().contents();

		is_integer(a[0].extract_number(), 1);
		is_integer(a[1].extract_number(), 2);
		is_integer(a[2].extract_number(), 3);
	}

	#[test]
	fn object() {
		todo!()
	}
}
