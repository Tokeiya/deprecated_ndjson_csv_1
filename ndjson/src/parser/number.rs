use super::super::elements::number_value::{from_error, from_f64, from_i128};
use super::super::elements::parse_number_error::ParseNumberError;
use super::super::elements::Value;
use super::white_space::ws;
use combine as cmb;
use combine::parser::char as chr;
use combine::{Parser, Stream};

enum Output {
	Char(char),
	String(String),
}

fn int_string_parser<I: Stream<Token = char>>() -> impl Parser<I, Output = String> {
	let zero = chr::char::<I>('0')
		.skip(cmb::not_followed_by(chr::digit()))
		.map(|c| c.to_string());

	let one_nine = cmb::satisfy::<I, _>(|c| match c {
		'1' => true,
		'2' => true,
		'3' => true,
		'4' => true,
		'5' => true,
		'6' => true,
		'7' => true,
		'8' => true,
		'9' => true,
		_ => false,
	})
	.map(|c| Output::Char(c));

	let digits = cmb::many::<String, I, _>(chr::digit()).map(|s| Output::String(s));

	let multi = (one_nine, digits).map(|(a, b)| {
		let Output::Char(a) = a else { unreachable!() };
		let Output::String(b) = b else { unreachable!() };

		let mut buff = a.to_string();
		buff.push_str(&b);
		buff
	});

	let signed = (cmb::optional::<I, _>(chr::char('-')), zero.or(multi)).map(|(s, v)| {
		let mut buff = if let Some(c) = s {
			c.to_string()
		} else {
			"".to_string()
		};

		buff.push_str(&v);
		buff
	});

	signed
}

pub fn number<I: Stream<Token = char>>() -> impl Parser<I, Output = Value> {
	let integer = int_string_parser::<I>();

	let frac = (
		chr::char::<I>('.'),
		cmb::many1::<String, I, _>(chr::digit()),
	)
		.map(|(_, v)| {
			let mut buff = ".".to_string();
			buff.push_str(&v);
			buff
		});

	let sign = chr::char::<I>('-').or(chr::char('+'));
	let exp_sign = chr::char::<I>('e').or(chr::char('E'));

	let exp = (
		exp_sign,
		cmb::optional(sign),
		cmb::many1::<String, I, _>(chr::digit()),
	)
		.map(|(e, s, v)| {
			let mut buff = e.to_string();

			if let Some(s) = s {
				buff.push(s)
			};

			buff.push_str(&v);
			buff
		});

	(
		ws(),
		cmb::optional(chr::char('-')),
		integer,
		cmb::optional(frac),
		cmb::optional(exp),
		ws(),
	)
		.map(|(l, s, i, f, e, r)| {
			let mut buff = String::from(l);

			if let Some(s) = s {
				buff.push(s);
			};

			buff.push_str(&i);

			let mut flg = false;

			if let Some(f) = f {
				flg = true;
				buff.push_str(&f);
			};

			if let Some(e) = e {
				flg = true;
				buff.push_str(&e);
			};

			buff.push_str(&r);

			let num = if flg {
				let value = buff.trim().parse::<f64>();

				match value {
					Ok(num) => from_f64(num, buff),
					Err(err) => from_error(ParseNumberError::Float(err), buff),
				}
			} else {
				let value = buff.trim().parse::<i128>();

				match value {
					Ok(num) => from_i128(num, buff),
					Err(err) => from_error(ParseNumberError::Integer(err), buff),
				}
			};

			Value::from(num)
		})
}

#[cfg(test)]
mod tests {
	use super::super::trimmed_output::test_helper::add_ws;
	use super::*;

	#[test]
	#[test]
	fn number() {
		fn assert_f(input: &str, expected: f64) {
			let mut parser = super::number::<&str>();
			let (a, r) = parser.parse(input).unwrap();

			assert_eq!(r, "");
			let tmp = a.extract_number();

			assert_eq!(tmp.raw_text(), input);
			let Ok(num) = tmp.value() else { unreachable!() };
			num.is_float(expected)
		}

		fn assert_i(input: &str, expected: i128) {
			let mut parser = super::number::<&str>();
			let (a, r) = parser.parse(input).unwrap();

			assert_eq!(r, "");
			let tmp = a.extract_number();

			assert_eq!(tmp.raw_text(), input);
			let Ok(num) = tmp.value() else { unreachable!() };
			num.is_integer(expected)
		}

		fn assert_parse_err(input: &str) {
			let mut parser = super::number::<&str>();
			assert!(parser.parse(input).is_err());
		}

		assert_f("1.0", 1.0);
		assert_f(
			&add_ws(&std::f64::consts::PI.to_string()),
			std::f64::consts::PI,
		);

		assert_f("-2.25", -2.25);
		assert_f("15.255e42", 15.255e42);
		assert_f("15.255E42", 15.255E42);
		assert_f("15.255e-42", 15.255e-42);
		assert_f("15.255E-42", 15.255E-42);
		assert_f("-15.255e42", -15.255e42);
		assert_f("-15.255E42", -15.255E42);
		assert_f(&add_ws("-15.255E42"), -15.255E42);

		assert_i(
			&add_ws("170141183460469231731687303715884105727"),
			170141183460469231731687303715884105727,
		);
		assert_i(
			"-170141183460469231731687303715884105728",
			-170141183460469231731687303715884105728,
		);

		assert_i(&add_ws("42"), 42);
		assert_i(&add_ws("0"), 0);
		assert_i(&add_ws("-42"), -42);

		assert_parse_err("00");
		assert_parse_err("01");
		assert_parse_err("+1");

		assert_parse_err("+2.25");

		assert_parse_err("15.");
	}
}
