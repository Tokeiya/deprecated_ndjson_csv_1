use crate::values::value::Value;
use crate::values::with_raw_value::WithRawValue;
use combine::parser::char as chr;
use combine::{Parser, Stream};
pub fn boolean<I: Stream<Token=char>>() -> impl Parser<I, Output=Value> {
	chr::char('a').map(|_| panic!())
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn true_test() {
		todo!()
	}

	#[test]
	fn false_test() {
		todo!()
	}

	#[test]
	fn invalid() {
		todo!()
	}
}
