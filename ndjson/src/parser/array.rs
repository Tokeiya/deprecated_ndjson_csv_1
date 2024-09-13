use combine::{self as cmb, Stream, Parser};
use combine::parser::char as chr;
use super::super::elements::value::Value as ElemValue;
use super::super::elements::array_value::ArrayValue;

pub fn array<I: Stream<Token=char>>() -> impl Parser<I, Output=ElemValue> {
	chr::char('a').map(|_| todo!())
}


#[cfg(test)]
mod test {
	use super::*;
	use super::super::trimmed_output::test_helper::{add_ws, WS};
	use super::super::super::elements::number_value::test_helper::{is_integer};
	#[test]
	fn empty() {
		let input = format!("{}[{}]{}", WS.as_str(), WS.as_str(), WS.as_str());
		let mut parser = array::<&str>();
		let (arr, rem) = parser.parse(&input).unwrap();

		assert_eq!(rem, "");

		let arr = arr.extract_array();
		assert_eq!(arr.beigin(), &format!("{}[{}", WS.as_str(), WS.as_str()));
		assert_eq!(arr.contents().len(), 0);
		assert_eq!(arr.end(), &format!("{}]", WS.as_str()));
	}

	#[test]
	fn single() {
		let mut parser = array::<&str>();
		let (arr, rem) = parser.parse("[1]").unwrap();

		assert_eq!(rem, "");
		let arr = arr.extract_array();

		let a = arr.contents()[0].extract_number();
		is_integer(a, 1);
	}

	#[test]
	fn multi() {
		let mut parser = array::<&str>();
		let (arr, rem) = parser.parse(r#"["hello",1,null,true]"#).unwrap();

		assert_eq!("", rem);

		let arr = arr.extract_array().contents();

		assert_eq!(arr[0].extract_string().value(), "hello");
		is_integer(arr[1].extract_number(), 1);
		_ = arr[2].extract_null();
		assert!(arr[3].extract_bool().value());
	}
}