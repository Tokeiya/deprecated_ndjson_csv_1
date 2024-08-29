use super::with_raw_value::WithRawValue;
use combine as cmb;
use combine::parser::char as chr;
use combine::{Parser, Stream};

// fn dummy<I: Stream<Token = char>, O>() -> impl Parser<I, Output = O> {
// 	chr::char('a').map(|_| panic!())
// }
fn unescaped<I: Stream<Token = char>>() -> impl Parser<I, Output = WithRawValue<char>> {
	cmb::satisfy::<I, _>(|c| {
		c == '\u{000020}'
			|| c == '\u{000021}'
			|| ('\u{000023}'..='\u{00005B}').contains(&c)
			|| ('\u{00005D}'..='\u{10FFFF}').contains(&c)
	})
	.map(|c| WithRawValue::new_from_string(c.to_string(), c))
}

fn escape_chars<I: Stream<Token = char>>() -> impl Parser<I, Output = WithRawValue<char>> {
	let esc = cmb::satisfy::<I, _>(|c| matches!(c, '"' | '\\' | '/' | 'b' | 'f' | 'n' | 'r' | 't'));

	esc.map(|c| {
		let esc_char = match c {
			'"' => '"',
			'\\' => '\\',
			'/' => '/',
			'b' => '\u{0008}',
			'f' => '\u{000C}',
			'n' => '\u{000A}',
			'r' => '\u{000D}',
			't' => '\u{0009}',
			_ => unreachable!(),
		};

		WithRawValue::new_from_string(format!(r#"{c}"#), esc_char)
	})
}

fn unicode_literal<I: Stream<Token = char>>() -> impl Parser<I, Output = WithRawValue<char>> {
	let digits = cmb::parser::repeat::count_min_max::<String, I, _>(4, 4, chr::hex_digit());

	(chr::char('u'), digits).map(|(_, d)| {
		let code_point = u32::from_str_radix(&d, 16).unwrap();

		//TODO:invalid care.
		let character = char::from_u32(code_point).unwrap();
		WithRawValue::new_from_string(format!("u{}", &d), character)
	})
}

fn escape<I: Stream<Token = char>>() -> impl Parser<I, Output = WithRawValue<char>> {
	(chr::char('\\'), escape_chars().or(unicode_literal())).map(|(_, dta)| {
		let value = *dta.value();
		let raw = format!(r#"\{}"#, dta.raw());
		WithRawValue::new_from_string(raw, value)
	})
}

pub fn string<I: Stream<Token = char>>() -> impl Parser<I, Output = WithRawValue<String>> {
	let tmp = unescaped::<I>().or(escape::<I>());

	let tmp = cmb::many::<Vec<WithRawValue<char>>, I, _>(tmp).map(|v| {
		let mut raw = String::new();
		let mut value = String::new();

		for elem in v {
			raw.push_str(elem.raw());
			value.push(*elem.value())
		}

		WithRawValue::new_from_string(raw, value)
	});

	(chr::char('"'), tmp, chr::char('"')).map(|(_, v, _)| {
		WithRawValue::new_from_string(format!(r#""{}""#, v.raw()), v.value().to_string())
	})
}

#[cfg(test)]
mod tests {
	use super::*;

	fn assert_result<T: PartialEq + std::fmt::Debug + std::fmt::Display, E>(
		actual: &Result<(WithRawValue<T>, &str), E>,
		expected: &T,
		expected_raw: &str,
	) {
		let Ok((a, r)) = actual else { unreachable!() };

		println!("RawValue:{}", a.raw());
		println!("Value:{}", a.value());

		assert_eq!(*r, "");
		assert_eq!(a.raw(), expected_raw);
		assert_eq!(a.value(), expected);
	}

	#[test]
	fn unescaped() {
		let mut parser = super::unescaped::<&str>();

		assert!(parser.parse("\u{000019}").is_err());
		assert!(parser.parse("\u{000022}").is_err());

		assert!(parser.parse("\u{00005C}").is_err());

		assert_result(
			&parser.parse("\u{000020}"),
			&'\u{000020}',
			&'\u{000020}'.to_string(),
		);
		assert_result(
			&parser.parse("\u{000021}"),
			&'\u{000021}',
			&'\u{000021}'.to_string(),
		);

		assert_result(
			&parser.parse("\u{000023}"),
			&'\u{000023}',
			&'\u{000023}'.to_string(),
		);
		assert_result(
			&parser.parse("\u{00005B}"),
			&'\u{00005B}',
			&'\u{00005B}'.to_string(),
		);

		assert_result(
			&parser.parse("\u{00005D}"),
			&'\u{00005D}',
			&'\u{00005D}'.to_string(),
		);
		assert_result(
			&parser.parse("\u{10FFFF}"),
			&'\u{10FFFF}',
			&'\u{10FFFF}'.to_string(),
		);
	}

	#[test]
	fn escape_chars() {
		let mut parser = super::escape_chars::<&str>();

		assert_result(&parser.parse("\""), &'"', "\"");
		assert_result(&parser.parse(r#"/"#), &'/', r#"/"#);
		assert_result(&parser.parse(r#"\"#), &'\\', r#"\"#);
		assert_result(&parser.parse(r#"b"#), &'\u{000008}', r#"b"#);
		assert_result(&parser.parse(r#"f"#), &'\u{00000C}', r#"f"#);
		assert_result(&parser.parse(r#"n"#), &'\u{00000A}', r#"n"#);
		assert_result(&parser.parse(r#"r"#), &'\u{00000D}', r#"r"#);
		assert_result(&parser.parse(r#"t"#), &'\u{000009}', r#"t"#);
	}

	#[test]
	fn unicode_literal() {
		let mut parser = super::unicode_literal::<&str>();

		assert_result(&parser.parse(r#"u0023"#), &'#', "u0023");
		assert_result(&parser.parse(r#"u005B"#), &'[', r#"u005B"#);
		assert_result(&parser.parse(r#"u005D"#), &']', r#"u005D"#);
		assert_result(&parser.parse(r#"u10FF"#), &'\u{10FF}', r#"u10FF"#);
	}

	#[test]
	fn escape() {
		let mut parser = super::escape::<&str>();

		assert_result(&parser.parse(r#"\\"#), &'\\', "\\\\");
		assert_result(&parser.parse(r#"\u0023"#), &'#', r#"\u0023"#);
	}

	#[test]
	fn invalid_escape() {
		let mut parser = super::escape::<&str>();

		assert!(parser.parse(r#"\a"#).is_err());
		assert!(parser.parse(r#"\z"#).is_err());

		assert!(parser.parse(r#"\u000"#).is_err());
	}

	#[test]
	fn string() {
		fn test(input: &str, expected: &str) {
			let quoted = &format!(r#""{input}""#);
			let mut parser = super::string::<&str>();
			let Ok((act, rem)) = parser.parse(quoted.as_str()) else {
				panic!()
			};
			assert_eq!(rem, "");

			println!("act.value():{}", act.value());
			println!("act.raw():{}", act.raw());

			assert_eq!(act.value(), expected);

			assert_eq!(act.raw(), quoted)
		}

		test("hello world", "hello world");
		test("\\n", "\n");

		test(r#"\u0026\u0027"#, "&'");

		test("hello\\tworld", "hello\tworld");
	}
}
