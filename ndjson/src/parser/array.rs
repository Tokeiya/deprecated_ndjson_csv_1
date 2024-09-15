use super::super::elements::array_value::ArrayValue;
use super::super::elements::value::Value as ElemValue;
use super::value::macro_expand::value as value_parser;
use super::white_space::ws;
use combine::parser::char as chr;
use combine::{self as cmb, Parser, Stream};

fn begin<I: Stream<Token=char>>() -> impl Parser<I, Output=String> {
	(ws(), chr::char::<I>('['), ws()).map(|(l, b, r)| {
		let mut buff = String::new();
		buff.push_str(&l);
		buff.push(b);
		buff.push_str(&r);
		buff
	})
}

fn end<I: Stream<Token=char>>() -> impl Parser<I, Output=String> {
	(chr::char::<I>(']'), ws()).map(|(b, r)| {
		let mut buff = String::new();
		buff.push(b);
		buff.push_str(&r);
		buff
	})
}

pub fn array<I: Stream<Token=char>>() -> impl Parser<I, Output=ElemValue> {
	let opt_val = cmb::optional(value_parser::<I>());

	let elem = (chr::char::<I>(',').map(|_| println!("hit")), value_parser::<I>()).map(|(_, v)| v);

	let following = cmb::many::<Vec<ElemValue>, I, _>(elem);

	let vec = (opt_val, following).map(|(e, f)| {
		let mut vec = Vec::<ElemValue>::new();

		if let Some(x) = e {
			vec.push(x)
		}

		for elem in f.into_iter() {
			vec.push(elem)
		}

		vec
	});

	(begin(), vec, end()).map(|(b, v, e)| ElemValue::from(ArrayValue::new(v, b, e)))
}

#[cfg(test)]
mod test {
	use super::super::super::elements::number_value::test_helper::is_integer;
	use super::super::trimmed_output::test_helper::WS;
	use super::*;
	#[test]
	fn empty() {
		let input = format!("{}[{}]{}", WS.as_str(), WS.as_str(), WS.as_str());
		let mut parser = array::<&str>();
		let (arr, rem) = parser.parse(&input).unwrap();

		assert_eq!(rem, "");

		let arr = arr.extract_array();
		assert_eq!(arr.beigin(), &format!("{}[{}", WS.as_str(), WS.as_str()));
		assert_eq!(arr.contents().len(), 0);
		assert_eq!(arr.end(), &format!("]{}", WS.as_str()));
	}

	#[test]
	fn single() {
		let mut parser = array::<&str>();
		let (arr, rem) = parser.parse("[           1     , 2     ]").unwrap();

		assert_eq!(rem, "");
		let arr = arr.extract_array();

		let a = arr.contents()[0].extract_number();
		is_integer(a, 1);

		println!("{:?}", arr.contents()[0].raw_string())
	}

	#[test]
	fn multi() {
		let mut parser = array::<&str>();
		let (arr, rem) = parser.parse(r#"["hello", 1 , null ,true]"#).unwrap();

		assert_eq!("", rem);

		let arr = arr.extract_array().contents();

		assert_eq!(arr[0].extract_string().value(), "hello");
		is_integer(arr[1].extract_number(), 1);
		_ = arr[2].extract_null();
		assert!(arr[3].extract_bool().value());
	}
}
