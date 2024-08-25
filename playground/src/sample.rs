use combine::parser::{self as psr, char as chr};
use combine::{parser, Parser, Stream};

fn expr_<I: Stream<Token = char>>() -> impl Parser<I, Output = String> {
	let d = chr::digit().map(|c| c.to_string());
	let e =
		(chr::char::<I>('('), expr(), chr::char::<I>(')')).map(|(l, e, r)| format!("{l} {e} {r}"));

	d.or(e)
}

parser! {
	pub fn expr[Input]()(Input)->String
	where [Input:Stream<Token = char>]{
		expr_()
	}
}
