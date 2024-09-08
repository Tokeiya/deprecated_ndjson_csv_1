use crate::values::value::Value;
use crate::values::with_raw_value::WithRawValue;
use combine::parser::char as chr;
use combine::{Parser, Stream};

pub fn value<I: Stream<Token=char>>() -> impl Parser<I, Output=WithRawValue<Value>> {
	chr::char('a').map(|_| panic!())
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::values::number::Number;
	#[test]
	fn value() {
		todo!()
	}
}
