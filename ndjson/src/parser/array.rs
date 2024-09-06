use super::value::value;
use crate::value::Value;
use crate::with_raw_value::WithRawValue;
use combine::parser::char as chr;
use combine::{Parser, Stream};

pub fn array<I: Stream<Token = char>>() -> impl Parser<I, Output = WithRawValue<Value>> {
	chr::char('a').map(|_| panic!())
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::number::Number;
	#[test]
	fn empty() {
		let mut parser = super::array();
		let (act, rem) = parser.parse("[]").unwrap();

		assert_eq!(rem, "");
		assert!(matches!(act.value(),Value::Array(v) if v.len()==0))
	}

	#[test]
	fn array() {
		let mut parser = super::array();
		let (act, rem) = parser
			.parse(
				r#"[1, 1.2,  true,       false,
		"hello",null]"#,
			)
			.unwrap();

		assert_eq!(rem, "");
		let Value::Array(vec) = act.value() else {
			unreachable!()
		};
		assert_eq!(vec.len(), 6);

		assert!(
			matches!(vec[0].value(),Value::Number(num) if matches!(num,Number::Integer(i) if i==&1))
		);
		assert_eq!(vec[0].raw(), "1");

		assert!(
			matches!(vec[1].value(),Value::Number(num) if matches!(num,Number::Float(f) if f==&1.2))
		);
		assert_eq!(vec[1].raw(), "1.2");

		assert!(matches!(vec[2].value(),Value::Boolean(b) if b==&true));
		assert_eq!(vec[2].raw(), "true");

		assert!(matches!(vec[3].value(),Value::Boolean(b) if b==&false));
		assert_eq!(vec[3].raw(), "false");

		assert!(matches!(vec[4].value(),Value::String(s) if s=="hello"));
		assert_eq!(vec[4].raw(), "\"hello\"");

		assert!(matches!(vec[5].value(), Value::Null));
		assert_eq!(vec[5].raw(), "null");
	}

	#[test]
	fn complex() {
		let mut parser = super::array();
		let (act, rem) = parser.parse(r#"[1, [2, 3], [4, [5, 6], 7], 8]"#).unwrap();

		assert_eq!(rem, "");
		let Value::Array(vec) = act.value() else {
			unreachable!()
		};

		assert_eq!(vec.len(), 4);

		assert!(
			matches!(vec[0].value(),Value::Number(num) if matches!(num,Number::Integer(i) if i==&1))
		);
		assert_eq!(vec[0].raw(), "1");

		let Value::Array(inner) = vec[1].value() else {
			unreachable!()
		};
		assert_eq!(inner.len(), 2);
		assert!(
			matches!(inner[0].value(),Value::Number(num) if matches!(num,Number::Integer(i) if i==&2))
		);
		assert_eq!(inner[0].raw(), "2");

		assert!(
			matches!(inner[1].value(),Value::Number(num) if matches!(num,Number::Integer(i) if i==&3))
		);
		assert_eq!(inner[1].raw(), "3");

		let Value::Array(inner) = vec[2].value() else {
			unreachable!()
		};
		assert_eq!(inner.len(), 3);

		assert!(
			matches!(inner[0].value(),Value::Number(num) if matches!(num,Number::Integer(i) if i==&4))
		);
		assert_eq!(inner[0].raw(), "4");

		let Value::Array(tmp) = inner[1].value() else {
			unreachable!()
		};
		assert_eq!(tmp.len(), 2);

		assert!(
			matches!(tmp[0].value(),Value::Number(num) if matches!(num,Number::Integer(i) if i==&5))
		);
		assert_eq!(tmp[0].raw(), "5");

		assert!(
			matches!(tmp[1].value(),Value::Number(num) if matches!(num,Number::Integer(i) if i==&6))
		);
		assert_eq!(tmp[1].raw(), "6");

		assert!(
			matches!(inner[2].value(),Value::Number(num) if matches!(num,Number::Integer(i) if i==&7))
		);
		assert_eq!(inner[2].raw(), "7");

		assert!(
			matches!(vec[3].value(),Value::Number(num) if matches!(num,Number::Integer(i) if i==&8))
		);
		assert_eq!(vec[3].raw(), "8");
	}
}
