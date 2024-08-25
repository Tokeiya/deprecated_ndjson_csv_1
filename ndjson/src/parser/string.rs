use combine as cmb;
use combine::parser::char as chr;
use combine::{Parser, Stream};
use super::text_presentation::TextPresentation;

fn dummy<I: Stream<Token=char>, O>() -> impl Parser<I, Output=O> {
	chr::char('a').map(|_| panic!())
}
fn unescaped<I: Stream<Token=char>>() -> impl Parser<I, Output=char> {
	dummy()
}

fn escape<I: Stream<Token=char>>() -> impl Parser<I, Output=char> {
	dummy()
}

fn string<I: Stream<Token=char>>() -> impl Parser<I, Output=TextPresentation> {
	dummy()
}


#[cfg(test)]
mod tests {
	use super::*;

	fn assert<T: PartialEq + std::fmt::Debug, E>(actual: &Result<(T, &str), E>, expected: &T) {
		let Ok((a, r)) = actual else { unreachable!() };
		assert_eq!(*r, "");
		assert_eq!(a, expected)
	}

	#[test]
	fn unescaped() {
		let mut parser = super::unescaped::<&str>();

		assert!(parser.parse("\u{000019}").is_err());
		assert!(parser.parse("\u{000022}").is_err());

		assert!(parser.parse("\u{00005C}").is_err());

		assert(&parser.parse("\u{000020}"), &'\u{000020}');
		assert(&parser.parse("\u{000021}"), &'\u{000021}');

		assert(&parser.parse("\u{000023}"), &'\u{000023}');
		assert(&parser.parse("\u{00005B}"), &'\u{00005B}');

		assert(&parser.parse("\u{00005D}"), &'\u{00005D}');
		assert(&parser.parse("\u{10FFFF}"), &'\u{10FFFF}');
	}

	#[test]
	fn escape() {
		let mut parser = super::escape::<&str>();


		todo!()
	}
}