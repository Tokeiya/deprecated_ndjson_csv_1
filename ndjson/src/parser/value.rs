use crate::value::Value;
use crate::with_raw_value::WithRawValue;
use combine::parser::char as chr;
use combine::{Parser, Stream};

pub fn value<I: Stream<Token=char>>() -> impl Parser<I, Output=WithRawValue<Value>> {
	chr::char('a').map(|_| panic!())
}


#[cfg(test)]
mod test {
	use crate::number::Number;
	use super::*;
	#[test]
	fn value() {
		let mut parser = super::value::<&str>();

		let (act, rem) = parser.parse("42").unwrap();

		assert_eq!(rem, "");
		assert_eq!(act.raw(), "42");

		matches!(act.value(),Value::Number(num) if matches!(num,Number::Integer(i) if i==&42));

		let (act, rem) = parser.parse("true").unwrap();

		assert_eq!(rem, "");
		assert_eq!(act.raw(), "true");

		matches!(act.value(),Value::Boolean(b) if b==&true);

		let (act, rem) = parser.parse("null").unwrap();

		assert_eq!(rem, "");
		assert_eq!(act.raw(), "null");

		matches!(act.value(),Value::Null);

		let (act, rem) = parser.parse(r#"[1,2,3,{key="value"}]"#).unwrap();

		assert_eq!(rem, "");
		assert_eq!(act.raw(), "[1,2,3,{key=\"value\"}]");

		let Value::Array(vec) = act.value() else { unreachable!() };
		assert_eq!(vec.len(), 4);

		assert!(matches!(vec[0].value(),Value::Number(num) if matches!(num,Number::Integer(i) if i==&1)));
		assert_eq!(vec[0].raw(), "1");

		assert!(matches!(vec[1].value(),Value::Number(num) if matches!(num,Number::Integer(i) if i==&2)));
		assert_eq!(vec[1].raw(), "2");

		assert!(matches!(vec[2].value(),Value::Number(num) if matches!(num,Number::Integer(i) if i==&3)));
		assert_eq!(vec[2].raw(), "3");

		let Value::Object(obj) = vec[3].value() else { unreachable!() };
		assert_eq!(obj.len(), 1);

		let (key, value) = obj.iter().next().unwrap();
		assert_eq!(key, "key");

		assert!(matches!(value.value(),Value::String(s) if s=="value"));
		assert_eq!(value.raw(), "\"value\"");
	}
}