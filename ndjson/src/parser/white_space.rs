use combine as cmb;
use combine::{Parser, Stream};

pub fn is_ws(c: &char) -> bool {
	match c {
		'\u{20}' => true,
		'\u{09}' => true,
		'\u{0A}' => true,
		'\u{0D}' => true,
		_ => false,
	}
}

pub fn ws<I: Stream<Token=char>>() -> impl Parser<I, Output=String> {
	let tmp = cmb::satisfy::<I, _>(|c| is_ws(&c));
	cmb::many::<String, I, _>(tmp)
}

#[cfg(test)]
mod test {
	use super::super::trimmed_output::test_helper::WS;
	use super::*;

	#[test]
	fn ws() {
		let mut p = super::ws::<&str>();
		let (p, r) = p.parse(WS.as_str()).unwrap();

		assert_eq!(&p, WS.as_str());
		assert_eq!(r, "");

		let mut p = super::ws::<&str>();
		let (p, r) = p.parse("aaa").unwrap();
		assert_eq!(p, "");
		assert_eq!(r, "aaa");
	}
}
