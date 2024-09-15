use super::white_space::ws;
use crate::elements::*;
use combine::parser::char as chr;
use combine::{Parser, Stream};
pub fn null<I: Stream<Token=char>>() -> impl Parser<I, Output=Value> {
	let p = chr::string("null").map(|_| ());
	(ws(), p, ws()).map(|(l, _, r)| Value::from(NullValue::from(format!("{}null{}", l, r))))
}

#[cfg(test)]
mod tests {
	use super::super::trimmed_output::test_helper::add_ws;
	use super::*;

	#[test]
	fn null() {
		let input = add_ws("null");
		let mut parser = super::null::<&str>();

		let (f, rem) = parser.parse(&input).unwrap();
		assert_eq!(rem, "");
		_ = f.extract_null();
	}
}
