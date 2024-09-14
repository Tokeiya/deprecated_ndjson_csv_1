use combine::parser::char as chr;
use combine::{self as cmb, Parser, Stream, StreamOnce};

pub struct Digits;

impl<I> Parser<I> for Digits
where
	I: Stream<Token = char>,
{
	type Output = String;
	type PartialState = ();

	fn parse(&mut self, input: I) -> Result<(Self::Output, I), <I as StreamOnce>::Error> {
		let mut a = chr::char::<I>('a').map(|x| x.to_string());
		a.parse(input)
	}
}

fn main() {
	let mut parser = cmb::many1::<String, &str, _>(chr::digit());
}
