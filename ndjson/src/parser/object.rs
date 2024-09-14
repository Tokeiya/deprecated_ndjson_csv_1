use super::super::elements::value::Value as ElemValue;
use combine::parser::char as chr;
use combine::{self as cmb, Parser, Stream};
pub fn object<I: Stream<Token=char>>() -> impl Parser<I, Output=ElemValue> {
	chr::char('a').map(|_| todo!())
}

#[cfg(test)]
mod test {
	use super::super::trimmed_output::test_helper::WS;
	use super::*;
	use crate::elements::number_value::test_helper::is_integer;
	use crate::elements::{StringValue, Value};
	use crate::elements::key_value::KeyValue;


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

		let (act, rem) = parser.parse(r#"{"key":42}"#).unwrap();
		assert_eq!("", rem);

		let obj = act.extract_object();

		assert_eq!(obj.begin(), "{");
		assert_eq!(obj.end(), "}");

		let obj = obj.content();
		assert_eq!(obj.len(), 1);

		let act = &obj[0];
		assert_eq!("key", act.key().value());
		assert_eq!(r#""key""#, act.key().raw_text());

		let act = obj[0].value().extract_number();
		is_integer(act, 42);
	}

	#[test]
	fn many() {
		let mut parser = object::<&str>();

		let (act, rem) = parser.parse(r#"{"key1":42,"key2":true,"key3":"value"}"#).unwrap();
		assert_eq!("", rem);

		let obj = act.extract_object();

		assert_eq!(obj.begin(), "{");
		assert_eq!(obj.end(), "}");

		let obj = obj.content();
		assert_eq!(obj.len(), 3);


		let act = &obj[0];
		assert_eq!("key1", act.key().value());
		assert_eq!(r#""key1""#, act.key().raw_text());

		let act = obj[0].value().extract_number();
		is_integer(act, 42);
		assert_eq!("42", act.raw_text());

		let act = &obj[1];
		assert_eq!("key2", act.key().value());
		assert_eq!(r#""key2""#, act.key().raw_text());

		let act = obj[1].value().extract_bool();
		assert!(act.value());
		assert_eq!("true", act.raw_text());

		let act = &obj[2];
		assert_eq!("key3", act.key().value());
		assert_eq!(r#""key3""#, act.key().raw_text());

		let act = obj[2].value().extract_string();
		assert_eq!("value", act.value());
		assert_eq!(r#""value""#, act.raw_text());
	}
}
