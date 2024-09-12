use super::super::elements::with_raw_text::WithRawText;
use crate::elements::Value;
use combine as cmb;
use combine::parser::char as chr;
use combine::{Parser, Stream};

/*
	 string = quotation-mark *char quotation-mark

	 char = unescaped /
		 escape (
			 %x22 /          ; "    quotation mark  U+0022
			 %x5C /          ; \    reverse solidus U+005C
			 %x2F /          ; /    solidus         U+002F
			 %x62 /          ; b    backspace       U+0008
			 %x66 /          ; f    form feed       U+000C
			 %x6E /          ; n    line feed       U+000A
			 %x72 /          ; r    carriage return U+000D
			 %x74 /          ; t    tab             U+0009
			 %x75 4HEXDIG )  ; uXXXX                U+XXXX

	 escape = %x5C              ; \

	 quotation-mark = %x22      ; "

	 unescaped = %x20-21 / %x23-5B / %x5D-10FFFF
*/
fn escape<I: Stream<Token = char>>() -> impl Parser<I, Output = WithRawText<char>> {
	let tmp = cmb::satisfy::<I, _>(|c| match c {
		'"' => true,
		'\\' => true,
		'/' => true,
		'b' => true,
		'f' => true,
		'n' => true,
		'r' => true,
		't' => true,
		_ => false,
	})
	.map(|c| match c {
		'"' => '"',
		'\\' => '\\',
		'/' => '/',
		'b' => '\u{0008}',
		'f' => '\u{000C}',
		'n' => '\n',
		'r' => '\r',
		't' => '\t',
		_ => unreachable!(),
	});

	let esc = chr::char('\\')
		.and(tmp)
		.map(|(a, b)| WithRawText::new(b, format!("\\{a}")));

	let unicode =
		cmb::parser::repeat::count_min_max::<String, I, _>(4, 4, chr::hex_digit()).map(|str| {
			let code_point = u32::from_str_radix(&str, 16).unwrap();
			WithRawText::new(std::char::from_u32(code_point).unwrap(), str)
		});

	let unicode = (chr::string::<I>(r#"\u"#), unicode)
		.map(|(a, b)| WithRawText::new(*b.value(), format!("{a}{}", b.raw_text())));

	esc.or(unicode)
}

fn unescaped<I: Stream<Token = char>>() -> impl Parser<I, Output = char> {
	chr::char::<I>('a').map(|_| todo!())
}

fn character<I: Stream<Token = char>>() -> impl Parser<I, Output = WithRawText<char>> {
	chr::char('a').map(|_| todo!())
}

pub fn string<I: Stream<Token = char>>() -> impl Parser<I, Output = Value> {
	chr::char('a').map(|_| todo!())
}

#[cfg(test)]
mod test {
	use super::super::trimmed_output::test_helper::add_ws;
	use super::*;

	fn assert_raw<E>(actual: Result<(WithRawText<char>, &str), E>, c: char, r: &str) {
		let Ok((act, rem)) = actual else {
			unreachable!()
		};
		assert_eq!(rem, "");
		assert_eq!(act.value(), &c);
		assert_eq!(act.raw_text(), r);
	}

	fn assert_char<E>(actual: Result<(char, &str), E>, c: char) {
		let Ok((act, rem)) = actual else {
			unreachable!()
		};
		assert_eq!(rem, "");
		assert_eq!(act, c);
	}

	fn assert_value<E>(actual: Result<(Value, &str), E>, expected: &str, expected_raw: &str) {
		let Ok((act, rem)) = actual else {
			unreachable!()
		};
		assert_eq!(rem, "");

		let act = act.extract_string();

		assert_eq!(act.value(), expected);
		assert_eq!(act.raw_text(), expected_raw);
	}

	#[test]
	fn string() {
		let mut parser = super::string::<&str>();

		//escaped
		assert_value(parser.parse(r#""\u0061""#), "a", r#""\u0061""#);
		assert_value(parser.parse(r#""\u0000""#), "\u{0000}", r#""\u0000""#);
		assert_value(parser.parse(r#""\"""#), "\"", r#""\"""#);
		assert_value(parser.parse(r#""\\""#), "\\", r#""\\""#);
		assert_value(parser.parse(r#""\/""#), "/", r#""\/""#);
		assert_value(parser.parse(r#""\b""#), "\u{0008}", r#""\b""#);
		assert_value(parser.parse(r#""\f""#), "\u{000C}", r#""\f""#);
		assert_value(parser.parse(r#""\n""#), "\n", r#""\n""#);
		assert_value(parser.parse(r#""\r""#), "\r", r#""\r""#);
		assert_value(parser.parse(r#""\t""#), "\t", r#""\t""#);

		//unescaped
		assert_value(parser.parse(r#""\u0020""#), " ", r#""\u0020""#);
		assert_value(parser.parse(r#""\u0021""#), "!", r#""\u0021""#);
		assert_value(parser.parse(r#""\u0023""#), "#", r#""\u0023""#);
		assert_value(parser.parse(r#""\u005B""#), "[", r#""\u005B""#);
		assert_value(parser.parse(r#""\u005D""#), "]", r#""\u005D""#);
		assert_value(parser.parse(r#""\u10FFFF""#), "\u{10FFFF}", r#""\u10FFFF""#);

		//ws
		let input = add_ws(r#""\u0061""#);
		let mut parser = super::string::<&str>();
		assert_value(parser.parse(&input), "a", &input);

		let input = add_ws(r#""\u0000""#);
		let mut parser = super::string::<&str>();
		assert_value(parser.parse(&input), "\u{0000}", &input);

		let input = add_ws(r#""\"""#);
		let mut parser = super::string::<&str>();
		assert_value(parser.parse(&input), "\"", &input);

		let input = add_ws(r#""\\""#);
		let mut parser = super::string::<&str>();
		assert_value(parser.parse(&input), "\\", &input);

		let input = add_ws(r#""\/""#);
		let mut parser = super::string::<&str>();
		assert_value(parser.parse(&input), "/", &input);

		let input = add_ws(r#""\b""#);
		let mut parser = super::string::<&str>();
		assert_value(parser.parse(&input), "\u{0008}", &input);

		let input = add_ws(r#""\f""#);
		let mut parser = super::string::<&str>();
		assert_value(parser.parse(&input), "\u{000C}", &input);

		let input = add_ws(r#""\n""#);
		let mut parser = super::string::<&str>();
		assert_value(parser.parse(&input), "\n", &input);

		let input = add_ws(r#""\r""#);
		let mut parser = super::string::<&str>();
		assert_value(parser.parse(&input), "\r", &input);

		let input = add_ws(r#""\t""#);
		let mut parser = super::string::<&str>();
		assert_value(parser.parse(&input), "\t", &input);

		let input = add_ws(r#""\u0020""#);
		let mut parser = super::string::<&str>();
		assert_value(parser.parse(&input), " ", &input);

		let input = add_ws(r#""\u0021""#);
		let mut parser = super::string::<&str>();
		assert_value(parser.parse(&input), "!", &input);

		let input = add_ws(r#""\u0023""#);
		let mut parser = super::string::<&str>();
		assert_value(parser.parse(&input), "#", &input);

		let input = add_ws(r#""\u005B""#);
		let mut parser = super::string::<&str>();
		assert_value(parser.parse(&input), "[", &input);

		let input = add_ws(r#""\u005D""#);
		let mut parser = super::string::<&str>();
		assert_value(parser.parse(&input), "]", &input);

		let input = add_ws(r#""\u10FFFF""#);
		let mut parser = super::string::<&str>();
		assert_value(parser.parse(&input), "\u{10FFFF}", &input);

		let input = add_ws(r#""   hello\tworld   \r\n""#);
		let mut parser = super::string::<&str>();
		assert_value(parser.parse(&input), "   hello\tworld   \r\n", &input);

		//err
		let mut parser = super::string::<&str>();
		assert!(parser.parse(r#""\u006""#).is_err());
		assert!(parser.parse(r#""\a""#).is_err());
	}
	#[test]
	fn escape() {
		let mut parser = super::escape::<&str>();

		assert_raw(parser.parse(r#"\""#), '"', r#"\""#);
		assert_raw(parser.parse(r#"\\"#), '\\', r#"\\"#);
		assert_raw(parser.parse(r#"\/"#), '/', r#"\/"#);
		assert_raw(parser.parse(r#"\b"#), '\u{0008}', r#"\b"#);
		assert_raw(parser.parse(r#"\f"#), '\u{000C}', r#"\f"#);
		assert_raw(parser.parse(r#"\n"#), '\n', r#"\n"#);
		assert_raw(parser.parse(r#"\r"#), '\r', r#"\r"#);
		assert_raw(parser.parse(r#"\t"#), '\t', r#"\t"#);

		assert_raw(parser.parse(r#"\u0000"#), '\u{0000}', r#"\u0000"#);
		assert_raw(parser.parse(r#"\u0061"#), 'a', r#"\u0061"#);
	}

	#[test]
	fn unescaped() {
		let input = '\u{20}'.to_string();
		let mut parser = super::unescaped::<&str>();
		assert_char(parser.parse(&input), '\u{20}');

		let input = '\u{21}'.to_string();
		let mut parser = super::unescaped::<&str>();
		assert_char(parser.parse(&input), '\u{21}');

		let input = '\u{23}'.to_string();
		let mut parser = super::unescaped::<&str>();
		assert_char(parser.parse(&input), '\u{23}');

		let input = '\u{5B}'.to_string();
		let mut parser = super::unescaped::<&str>();
		assert_char(parser.parse(&input), '\u{5B}');

		let input = '\u{5D}'.to_string();
		let mut parser = super::unescaped::<&str>();
		assert_char(parser.parse(&input), '\u{5D}');

		let input = '\u{10FFFF}'.to_string();
		let mut parser = super::unescaped::<&str>();
		assert_char(parser.parse(&input), '\u{10FFFF}');
	}
}
