use super::super::elements::value::Value as ElemValue;
use super::string::string;
use super::value::macro_expand::value as value_parser;
use crate::elements::key_value::KeyValue;
use crate::elements::ObjectValue;
use crate::parser::white_space::ws;
use combine::parser::char as chr;
use combine::{self as cmb, Parser, Stream};
fn member<I: Stream<Token=char>>() -> impl Parser<I, Output=KeyValue> {
	(string::<I>(), chr::char(':'), value_parser::<I>()).map(|(k, _, v)| {
		let ElemValue::String(s) = k else {
			unreachable!()
		};
		KeyValue::new(s, v)
	})
}

pub fn object<I: Stream<Token=char>>() -> impl Parser<I, Output=ElemValue> {
	let begin = (ws::<I>(), chr::char::<I>('{'), ws::<I>()).map(|(l, b, r)| {
		let mut buff = String::new();
		buff.push_str(&l);
		buff.push(b);
		buff.push_str(&r);

		buff
	});

	let end = (chr::char::<I>('}'), ws::<I>()).map(|(b, r)| {
		let mut buff = b.to_string();
		buff.push_str(&r);

		buff
	});

	let tmp = (chr::char::<I>(','), member()).map(|(_, v)| v);

	let contents = (
		cmb::optional(member::<I>()),
		cmb::many::<Vec<KeyValue>, I, _>(tmp),
	)
		.map(|(o, v)| {
			let mut vec = Vec::new();

			if let Some(x) = o {
				vec.push(x)
			}

			for elem in v.into_iter() {
				vec.push(elem)
			}

			vec
		});

	(begin, contents, end).map(|(b, c, e)| ElemValue::from(ObjectValue::new(c, b, e)))
}

#[cfg(test)]
mod test {
	use super::super::trimmed_output::test_helper::WS;
	use super::*;
	use crate::elements::number_value::test_helper::is_integer;
	use crate::elements::text_expression::{TextExpression, test_helper::assert_text_expression};

	#[test]
	fn empty() {
		let input = format!("{}{{{}}}{}", WS.as_str(), WS.as_str(), WS.as_str());
		println!("input:{:?}", &input);

		let mut parser = object::<&str>();
		let (obj, rem) = parser.parse(&input).unwrap();

		assert_eq!(rem, "");

		let obj = obj.extract_object();
		println!("end:{}", obj.end());

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

		let (act, rem) = parser
			.parse(r#"{"key1":42,"key2":true,"key3":"value"}"#)
			.unwrap();
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
