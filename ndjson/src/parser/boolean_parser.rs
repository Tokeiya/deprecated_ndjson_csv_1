use super::white_space::ws;
use crate::elements::{BooleanValue, Value};
use combine::parser::char as chr;
use combine::{Parser, Stream};
pub fn boolean<I: Stream<Token=char>>() -> impl Parser<I, Output=Value> {
	let t = chr::string::<I>("true").map(|_| (true, "true"));

	let f = chr::string::<I>("false").map(|_| (false, "false"));

	let b = t.or(f);

	(ws(), b, ws())
		.map(|(l, (v, t), r)| Value::from(BooleanValue::new(v, format!("{}{}{}", l, t, r))))
}

#[cfg(test)]
mod test {
	use super::super::trimmed_output::test_helper::WS;
	use super::*;
	use crate::elements::text_expression::TextExpression;
	#[test]
	fn bool() {
		let input = format!("{}true{}", WS.as_str(), WS.as_str());
		let mut parser = boolean::<&str>();
		let (v, _) = parser.parse(input.as_str()).unwrap();

		assert_eq!(&v.extract_bool().raw_text(), &input);
		assert_eq!(v.extract_bool().value(), &true);

		let input = format!("{}false{}", WS.as_str(), WS.as_str());
		let mut parser = boolean::<&str>();
		let (v, _) = parser.parse(input.as_str()).unwrap();

		assert_eq!(&v.extract_bool().raw_text(), &input);
		assert_eq!(v.extract_bool().value(), &false);
	}
}
