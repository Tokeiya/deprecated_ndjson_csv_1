use combine::{self as cmb, Stream, Parser};
use combine::parser::char as chr;
use super::super::elements::value::Value as ElemValue;
pub fn object<I: Stream<Token=char>>() -> impl Parser<I, Output=ElemValue> {
	chr::char('a').map(|_| todo!())
}


#[cfg(test)]
mod test {
	use crate::elements::StringValue;
	use super::*;
	use super::super::trimmed_output::test_helper::WS;
	use crate::elements::number_value::test_helper::is_integer;

	#[test]
	fn empty() {
		let input = format!("{}{{{}}}{}", WS.as_str(), WS.as_str(), WS.as_str());
		let mut parser = object::<&str>();
		let (obj, rem) = parser.parse(&input).unwrap();

		assert_eq!(rem, "");

		let obj = obj.extract_object();
		assert_eq!(obj.begin(), &format!("{}{{{}", WS.as_str(), WS.as_str()));
		assert_eq!(obj.end(), &format!("}}{}", WS.as_str()));

		assert_eq!(obj.content().len(), 0);
	}

	#[test]
	fn single() {
		let mut parser = object::<&str>();
		let (obj, rem) = parser.parse(r#"{"foo":true}"#).unwrap();

		assert_eq!("", rem);
		let obj = obj.extract_object().content();

		assert_eq!(obj.len(), 1);

		let act = &obj[&StringValue::new("foo".to_string(), "foo".to_string())];
		assert!(act.extract_single().extract_bool().value());

		let (obj, _) = parser.parse(r#"{"foo":false,"foo":null}"#).unwrap();
		let obj = obj.extract_object().content();

		assert_eq!(obj.len(), 1);
		let act = obj[&StringValue::new("foo".to_string(), "foo".to_string())].extract_multi();
		assert_eq!(act.len(), 2);

		assert!(!act[0].extract_bool().value());
		_ = act[1].extract_null();
	}

	#[test]
	fn multi() {
		let mut parser = object::<&str>();
		let (obj, _) = parser.parse(r#"{"foo":"hello","bar":42}"#).unwrap();
		let obj = obj.extract_object().content();

		assert_eq!(obj.len(), 2);

		let act = obj[&StringValue::new("foo".to_string(), "foo".to_string())].extract_single().extract_string().value();
		assert_eq!(act, "hello");

		let act = obj[&StringValue::new("bar".to_string(), "bar".to_string())].extract_single().extract_number();
		is_integer(act, 42);
	}
}