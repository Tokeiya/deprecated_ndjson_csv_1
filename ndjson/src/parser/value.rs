use crate::value::Value;
use crate::with_raw_value::WithRawValue;
use combine::parser::char as chr;
use combine::{Parser, Stream};

pub fn value<I: Stream<Token = char>>() -> impl Parser<I, Output = WithRawValue<Value>> {
	chr::char('a').map(|_| panic!())
}
