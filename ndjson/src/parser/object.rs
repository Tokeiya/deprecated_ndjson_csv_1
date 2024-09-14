use super::super::elements::value::Value as ElemValue;
use combine::parser::char as chr;
use combine::{self as cmb, Parser, Stream};
pub fn object<I: Stream<Token = char>>() -> impl Parser<I, Output = ElemValue> {
	chr::char('a').map(|_| todo!())
}

#[cfg(test)]
mod test {
	use super::super::trimmed_output::test_helper::WS;
	use super::*;
	use crate::elements::number_value::test_helper::is_integer;
	use crate::elements::StringValue;

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
		todo!();
	}

	#[test]
	fn many() {
		todo!();
	}
}
