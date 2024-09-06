use crate::value::Value;
use crate::with_raw_value::WithRawValue;
use combine::parser::char as chr;
use combine::{Parser, Stream};

pub fn object<I: Stream<Token = char>>() -> impl Parser<I, Output = WithRawValue<Value>> {
	chr::char('a').map(|_| panic!())
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::number::Number;

	#[test]
	fn singole_object() {
		let mut parser = object::<&str>();

		let (act, rem) = parser.parse("{key:42}").unwrap();

		assert_eq!(rem, "");
		assert_eq!(act.raw(), "{key:42}");

		let Value::Object(obj) = act.value() else {
			unreachable!()
		};
		assert_eq!(obj.len(), 1);

		let (key, value) = obj.iter().next().unwrap();
		assert_eq!(key, "key");

		let Value::Number(num) = value.value() else {
			unreachable!()
		};
		assert!(matches!(num,Number::Integer(i) if i==&42));
	}
}
