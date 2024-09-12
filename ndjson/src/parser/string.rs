use super::super::elements::with_raw_text::WithRawText;
use crate::elements::Value;
use combine as cmb;
use combine::parser::char as chr;
use combine::{Parser, Stream};

fn escape<I: Stream<Token = char>>() -> impl Parser<I, Output = WithRawText<char>> {
	chr::char::<I>('a').map(|_| todo!())
}

fn unescape<I: Stream<Token = char>>() -> impl Parser<I, Output = char> {
	chr::char::<I>('a').map(|_| todo!())
}

fn character<I: Stream<Token = char>>() -> impl Parser<I, Output = WithRawText<char>> {
	chr::char('a').map(|_| todo!())
}

pub fn string<I: Stream<Token = char>>() -> impl Parser<I, Output = Value> {
	chr::char('a').map(|_| todo!())
}
