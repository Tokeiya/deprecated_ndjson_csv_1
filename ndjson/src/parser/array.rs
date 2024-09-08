use super::value::value;
use crate::values::value::Value;
use crate::values::with_raw_value::WithRawValue;
use combine::parser::char as chr;
use combine::{Parser, Stream};

pub fn array<I: Stream<Token=char>>() -> impl Parser<I, Output=Value> {
	chr::char('a').map(|_| panic!())
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::values::number::Number;
	#[test]
	fn empty() {
		todo!()
	}

	#[test]
	fn array() {
		todo!()
	}

	#[test]
	fn complex() {
		todo!()
	}
}
