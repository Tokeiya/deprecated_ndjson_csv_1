use combine::parser::char as chr;
use combine::{Parser, Stream};
use crate::member::Member;

pub fn member<I: Stream<Token=char>>() -> impl Parser<I, Output=Member> {
	chr::char('a').map(|_| panic!())
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::value::Value;
	use crate::number::Number;

	#[test]
	fn member() {
		let mut parser = super::member::<&str>();

		let (act, rem) = parser.parse("foo:42").unwrap();

		assert_eq!(rem, "");
		assert_eq!(act.name(), "foo");
		assert_eq!(act.value().raw(), "42");
		assert!(matches!(act.value().value(),Value::Number(num) if matches!(num,Number::Integer(i) if i==&42)));

		let (act, rem) = parser.parse("          foo\t:\n42\r\n").unwrap();

		assert_eq!(rem, "");
		assert_eq!(act.name(), "foo");
		assert_eq!(act.value().raw(), "42");
		assert!(matches!(act.value().value(),Value::Number(num) if matches!(num,Number::Integer(i) if i==&42)))
	}
}
