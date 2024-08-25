use combine::{self as cmb, Parser, Stream};

pub fn white_space<Input: Stream<Token = char>>() -> impl Parser<Input, Output = ()> {
	let tmp = cmb::satisfy(|c: char| match c {
		' ' => true,
		'\t' => true,
		'\n' => true,
		'\r' => true,
		_ => false,
	});

	cmb::many(tmp).map(|_: String| ())
}

#[cfg(test)]
mod tests {
	use super::*;
	use combine::error::StringStreamError;

	fn assert_result(actual: Result<((), &str), StringStreamError>, remain: &str) {
		let Ok((_, rem)) = actual else { unreachable!() };
		assert_eq!(rem, remain)
	}
	#[test]
	fn ws() {
		let mut parser = white_space();
		assert_result(parser.parse(" "), "");
		assert_result(parser.parse("\t"), "");
		assert_result(parser.parse("\n"), "");
		assert_result(parser.parse("\r"), "");
		assert_result(parser.parse("\r\n"), "");

		assert_result(parser.parse(" \t  \n\r\r\n        "), "");
		assert_result(parser.parse(""), "");
	}

	#[test]
	fn invalid_ws() {
		#[should_panic]
		fn f(input: &str) {
			let mut parser = white_space();
			parser.parse(input).unwrap();
		}

		f("a");
		f("　");
	}
}
