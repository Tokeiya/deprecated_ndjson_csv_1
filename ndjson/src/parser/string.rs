use super::text_presentation::TextPresentation;
use super::with_raw_value::WithRawValue;
use combine as cmb;
use combine::parser::char as chr;
use combine::{Parser, Stream};

fn dummy<I: Stream<Token = char>, O>() -> impl Parser<I, Output = O> {
	chr::char('a').map(|_| panic!())
}
fn unescaped<I: Stream<Token = char>>() -> impl Parser<I, Output = char> {
	cmb::satisfy::<I, _>(|c| {
		if c == '\u{000020}' || c == '\u{000021}' {
			true
		} else if c >= '\u{000023}' && c <= '\u{00005B}' {
			true
		} else if c >= '\u{00005D}' && c <= '\u{10FFFF}' {
			true
		} else {
			false
		}
	})
}

fn escaped<I: Stream<Token = char>>() -> impl Parser<I, Output = WithRawValue<char>> {
	// let body = cmb::satisfy::<I, _>(|c| {
	// 	match c {
	// 		'"' => true,
	// 		'\\' => true,
	// 		'/' => true,
	// 		'b' => true,
	// 		'f' => true,
	// 		'n' => true,
	// 		'r' => true,
	// 		't' => true,
	// 		_ => false
	// 	}
	// }).map(|c| {
	// 	match c {
	// 		'"' => WithRawValue::new("\\\"", '"'),
	// 		'\\' => WithRawValue::new("\\\\", '\\'),
	// 		'/' => WithRawValue::new("\\/", '/'),
	// 		'b' => WithRawValue::new("\\b", '\u{08}'),
	// 		'f' => WithRawValue::new("\\f", '\u{0C}'),
	// 		'n' => WithRawValue::new("\\n", '\n'),
	// 		'r' => WithRawValue::new("\\r", '\r'),
	// 		't' => WithRawValue::new("\\t", '\t'),
	// 		_ => unreachable!()
	// 	}
	// });
	//
	// let literal = (chr::char('u'),
	//                cmb::parser::repeat::count_min_max::<String, I, _>(4, 4, chr::hex_digit())).map(|(_, x)| {
	// 	WithRawValue::new(&format!("u{x}"), u32::from_str_radix(x, 16))
	// });

	dummy()
}

fn escape<I: Stream<Token = char>>() -> impl Parser<I, Output = char> {
	dummy()
}

fn string<I: Stream<Token = char>>() -> impl Parser<I, Output = TextPresentation> {
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
