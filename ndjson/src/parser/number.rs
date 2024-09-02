use super::text_presentation::TextPresentation;
use combine as cmb;
use combine::parser::char as chr;
use combine::{Parser, Stream};

use crate::with_raw_value::WithRawValue;
use crate::number::Number;
use crate::parse_number_error::ParseNumberError;

use crate::typed_value::TypedValue;

fn zero<I: Stream<Token = char>>() -> impl Parser<I, Output = TextPresentation> {
	chr::char('0').map(TextPresentation::from)
}

fn plus<I: Stream<Token = char>>() -> impl Parser<I, Output = TextPresentation> {
	chr::char('+').map(TextPresentation::from)
}

fn minus<I: Stream<Token = char>>() -> impl Parser<I, Output = TextPresentation> {
	chr::char('-').map(TextPresentation::from)
}

fn integer<I: Stream<Token = char>>() -> impl Parser<I, Output = TextPresentation> {
	let tmp = (
		digit1_9(),
		cmb::many::<String, I, _>(chr::digit()).map(TextPresentation::from),
	)
		.map(|(f, s)| {
			let mut buff = f.to_string();
			s.write_to_buffer(&mut buff);
			TextPresentation::from(buff)
		});

	zero().or(tmp)
}

fn frac<I: Stream<Token = char>>() -> impl Parser<I, Output = TextPresentation> {
	(
		chr::char::<I>('.'),
		cmb::many1::<String, I, _>(chr::digit()),
	)
		.map(|(dp, digits)| {
			let mut buff = String::from(dp);
			buff.push_str(&digits);
			TextPresentation::from(buff)
		})
}

fn exponent<I: Stream<Token = char>>() -> impl Parser<I, Output = TextPresentation> {
	let e_sign = chr::char::<I>('e').or(chr::char::<I>('E'));
	let opt_sign = cmb::optional(plus::<I>().or(minus::<I>()))
		.map(|c| c.unwrap_or_else(|| TextPresentation::Empty));
	let digits = cmb::many1::<String, I, _>(chr::digit::<I>());

	(e_sign, opt_sign, digits).map(|(s, o, d)| {
		let mut buff = String::from(s);
		o.write_to_buffer(&mut buff);
		buff.push_str(&d);

		TextPresentation::from(buff)
	})
}

fn digit1_9<I: Stream<Token = char>>() -> impl Parser<I, Output = TextPresentation> {
	cmb::satisfy(|c| match c {
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
	.map(TextPresentation::from)
}

fn number_str<I: Stream<Token = char>>() -> impl Parser<I, Output = (TextPresentation, bool)> {
	let opt_minus = cmb::optional(minus()).map(|c| c.unwrap_or_else(|| TextPresentation::Empty));
	let opt_frac = cmb::optional(frac()).map(|c| c.unwrap_or_else(|| TextPresentation::Empty));
	let opt_exp = cmb::optional(exponent()).map(|c| c.unwrap_or_else(|| TextPresentation::Empty));

	(opt_minus, integer(), opt_frac, opt_exp).map(|(m, i, f, e)| {
		let mut buff = m.to_string();
		i.write_to_buffer(&mut buff);
		f.write_to_buffer(&mut buff);
		e.write_to_buffer(&mut buff);

		(
			TextPresentation::from(buff),
			matches!(f, TextPresentation::Empty) && matches!(e, TextPresentation::Empty),
		)
	})
}

pub fn number<I: Stream<Token = char>>(
) -> impl Parser<I, Output = WithRawValue<Result<TypedValue, ParseNumberError>>> {
	number_str().map(|(txt, flg)| {
		let TextPresentation::Text(t) = txt else {
			unreachable!()
		};
		let t = t.as_str();

		if flg {
			match t.parse::<i128>() {
				Ok(i) => WithRawValue::new_from_str(t, Ok(TypedValue::from(Number::from(i)))),
				Err(e) => WithRawValue::new_from_str(t, Err(ParseNumberError::Integer(e))),
			}
		} else {
			match t.parse::<f64>() {
				Ok(f) => WithRawValue::new_from_str(t, Ok(TypedValue::from(Number::from(f)))),
				Err(e) => WithRawValue::new_from_str(t, Err(ParseNumberError::Float(e))),
			}
		}
	})
}

#[cfg(test)]
mod tests {
	use super::*;

	#[should_panic]
	fn should_panic<'a, T>(mut parser: impl Parser<&'a str, Output = T>, input: &'a str) {
		_ = parser.parse(input)
	}

	#[test]
	fn number() {
		fn a_int(
			(act, rem): (WithRawValue<Result<TypedValue, ParseNumberError>>, &str),
			expected: i128,
			expected_raw: &str,
		) {
			assert_eq!(rem, "");

			let Ok(TypedValue::Number(Number::Integer(i))) = act.value() else {
				unreachable!()
			};
			assert_eq!(i, &expected);
			assert_eq!(act.raw(), expected_raw)
		}

		fn a_flt(
			(act, rem): (WithRawValue<Result<TypedValue, ParseNumberError>>, &str),
			expected: f64,
			expected_raw: &str,
		) {
			assert_eq!(rem, "");

			let Ok(TypedValue::Number(Number::Float(f))) = act.value() else {
				unreachable!()
			};
			assert_eq!(f, &expected);
			assert_eq!(act.raw(), expected_raw)
		}

		fn a_err(
			act: (WithRawValue<Result<TypedValue, ParseNumberError>>, &str),
			is_int: bool,
			expected: &str,
		) {
			assert_eq!(act.1, expected);

			if is_int {
				assert!(matches!(act.0.value(), Err(ParseNumberError::Integer(_))))
			} else {
				assert!(matches!(act.0.value(), Err(ParseNumberError::Float(_))))
			}
		}

		let mut parser = super::number();

		println!("{}", i128::MAX);
		println!("{}", i128::MIN);

		a_err(
			parser
				.parse("170141183460469231731687303715884105728")
				.unwrap(),
			true,
			"",
		);
		a_err(
			parser
				.parse("-170141183460469231731687303715884105729")
				.unwrap(),
			true,
			"",
		);

		a_int(parser.parse("0").unwrap(), 0, "0");
		a_int(parser.parse("-0").unwrap(), 0, "-0");

		a_int(
			parser
				.parse("170141183460469231731687303715884105727")
				.unwrap(),
			170141183460469231731687303715884105727,
			"170141183460469231731687303715884105727",
		);
		a_int(
			parser
				.parse("-170141183460469231731687303715884105728")
				.unwrap(),
			-170141183460469231731687303715884105728,
			"-170141183460469231731687303715884105728",
		);

		a_flt(parser.parse("0.0").unwrap(), 0f64, "0.0");
		a_flt(parser.parse("-0.0").unwrap(), 0f64, "-0.0");

		a_flt(parser.parse("0.5").unwrap(), 0.5, "0.5");
		a_flt(parser.parse("-0.5").unwrap(), -0.5, "-0.5");

		a_flt(parser.parse("5e-1").unwrap(), 0.5, "5e-1");
		a_flt(parser.parse("-5e-1").unwrap(), -0.5, "-5e-1");
	}

	#[test]
	fn zero() {
		let mut parser = super::zero();
		let act = parser.parse("0");

		assert!(act.is_ok());
		let Ok((act, rem)) = act else { unreachable!() };

		assert_eq!(rem, "");
		assert!(matches!(act,super::TextPresentation::Character(c) if c=='0'));

		should_panic(super::zero(), "1");
	}

	#[test]
	fn plus() {
		let mut parser = super::plus();

		let Ok((act, rem)) = parser.parse("+") else {
			unreachable!()
		};
		assert_eq!(rem, "");

		assert!(matches!(act,TextPresentation::Character(c) if c=='+'));
		should_panic(super::plus(), "a");
	}

	#[test]
	fn minus() {
		let mut parser = super::minus();

		let Ok((act, rem)) = parser.parse("-") else {
			unreachable!()
		};
		assert_eq!(rem, "");

		assert!(matches!(act,TextPresentation::Character(c) if c=='-'));
		should_panic(super::minus(), "a");
	}

	#[test]
	fn integer() {
		let mut parser = super::integer();

		let Ok((act, rem)) = parser.parse("123") else {
			unreachable!()
		};
		assert_eq!(rem, "");
		assert!(matches!(act,TextPresentation::Text(c) if c=="123"));

		let Ok((act, rem)) = parser.parse("0") else {
			unreachable!()
		};
		assert_eq!(rem, "");
		assert!(matches!(act,TextPresentation::Character(c) if c=='0'));

		let Ok((act, rem)) = parser.parse("123456789") else {
			unreachable!()
		};
		assert_eq!(rem, "");
		assert!(matches!(act,TextPresentation::Text(c) if c=="123456789"));

		let Ok((act, rem)) = parser.parse("10") else {
			unreachable!()
		};
		assert_eq!(rem, "");
		assert!(matches!(act,TextPresentation::Text(c) if c=="10"));

		should_panic(super::integer(), "a");
		should_panic(super::integer(), "01");
		should_panic(super::integer(), "+1");
		should_panic(super::integer(), "-1");
	}

	#[test]
	fn frac() {
		let mut parser = super::frac();

		let Ok((act, rem)) = parser.parse(".141592") else {
			unreachable!()
		};
		assert_eq!(rem, "");
		assert!(matches!(act,TextPresentation::Text(c) if c==".141592"));

		let Ok((act, rem)) = parser.parse(".0") else {
			unreachable!()
		};
		assert_eq!(rem, "");
		assert!(matches!(act,TextPresentation::Text(c) if c==".0"));

		should_panic(super::frac(), ".a");
		should_panic(super::frac(), ".");
	}

	#[test]
	fn exponent() {
		let mut parser = super::exponent();

		let Ok((act, rem)) = parser.parse("e+10") else {
			unreachable!()
		};
		assert_eq!(rem, "");
		assert!(matches!(act,TextPresentation::Text(c) if c=="e+10"));

		let Ok((act, rem)) = parser.parse("E+42") else {
			unreachable!()
		};
		assert_eq!(rem, "");
		assert!(matches!(act,TextPresentation::Text(c) if c=="E+42"));

		let Ok((act, rem)) = parser.parse("e-10") else {
			unreachable!()
		};
		assert_eq!(rem, "");
		assert!(matches!(act,TextPresentation::Text(c) if c=="e-10"));

		let Ok((act, rem)) = parser.parse("E-42") else {
			unreachable!()
		};
		assert_eq!(rem, "");
		assert!(matches!(act,TextPresentation::Text(c) if c=="E-42"));

		let Ok((act, rem)) = parser.parse("e10") else {
			unreachable!()
		};
		assert_eq!(rem, "");
		assert!(matches!(act,TextPresentation::Text(c) if c=="e10"));

		let Ok((act, rem)) = parser.parse("E42") else {
			unreachable!()
		};
		assert_eq!(rem, "");
		assert!(matches!(act,TextPresentation::Text(c) if c=="E42"));

		should_panic(super::exponent(), "e");
		should_panic(super::exponent(), "e+");
		should_panic(super::exponent(), "e-");
		should_panic(super::exponent(), "e+a");
		should_panic(super::exponent(), "e-a");
		should_panic(super::exponent(), "e+a");

		should_panic(super::exponent(), "E");
		should_panic(super::exponent(), "E+");
		should_panic(super::exponent(), "E-");
		should_panic(super::exponent(), "E+a");
		should_panic(super::exponent(), "E-a");
	}

	#[test]
	fn digit1_9() {
		for c in "123456789".chars() {
			let input = c.to_string();
			let mut parser = super::digit1_9();
			let Ok((act, rem)) = parser.parse(input.as_str()) else {
				unreachable!()
			};
			assert_eq!(rem, "");
			assert!(matches!(act,TextPresentation::Character(c) if c==c));
		}

		should_panic(super::digit1_9(), "0");
		should_panic(super::digit1_9(), "a");
	}

	#[test]
	fn number_str() {
		//integer
		let mut parser = super::number_str();

		let Ok((act, rem)) = parser.parse("1234567890") else {
			unreachable!()
		};
		assert_eq!(rem, "");
		assert!(matches!(act,(TextPresentation::Text(c),f) if (c.as_str(),f)==("1234567890",true)));

		let Ok((act, rem)) = parser.parse("0") else {
			unreachable!()
		};
		assert_eq!(rem, "");
		assert!(matches!(act,(TextPresentation::Text(c),f) if (c.as_str(),f)==("0",true)));

		let Ok((act, rem)) = parser.parse("-1234567890") else {
			unreachable!()
		};
		assert_eq!(rem, "");
		assert!(
			matches!(act,(TextPresentation::Text(c),f) if (c.as_str(),f)==("-1234567890",true))
		);

		let Ok((act, rem)) = parser.parse("-0") else {
			unreachable!()
		};
		assert_eq!(rem, "");
		assert!(matches!(act,(TextPresentation::Text(c),f) if (c.as_str(),f)==("-0",true)));

		should_panic(super::number_str(), "01");
		should_panic(super::number_str(), "+1");

		//frac
		let Ok((act, rem)) = parser.parse("0.141592") else {
			unreachable!()
		};
		assert_eq!(rem, "");
		assert!(matches!(act,(TextPresentation::Text(c),f) if (c.as_str(),f)==("0.141592",false)));

		let Ok((act, rem)) = parser.parse("3.141592") else {
			unreachable!()
		};
		assert_eq!(rem, "");
		assert!(matches!(act,(TextPresentation::Text(c),f) if (c.as_str(),f)==("3.141592",false)));

		let Ok((act, rem)) = parser.parse("-3.0123") else {
			unreachable!()
		};
		assert_eq!(rem, "");
		assert!(matches!(act,(TextPresentation::Text(c),f) if (c.as_str(),f)==("-3.0123",false)));

		let Ok((act, rem)) = parser.parse("-0.0123") else {
			unreachable!()
		};
		assert_eq!(rem, "");
		assert!(matches!(act,(TextPresentation::Text(c),f) if (c.as_str(),f)==("-0.0123",false)));

		should_panic(super::number_str(), ".1");
		should_panic(super::number_str(), "1.");
		should_panic(super::number_str(), "+1.0");

		//exponent
		let Ok((act, rem)) = parser.parse("0e+10") else {
			unreachable!()
		};
		assert_eq!(rem, "");
		assert!(matches!(act,(TextPresentation::Text(c),f) if (c.as_str(),f)==("0e+10",false)));

		let Ok((act, rem)) = parser.parse("0E+10") else {
			unreachable!()
		};
		assert_eq!(rem, "");
		assert!(matches!(act,(TextPresentation::Text(c),f) if (c.as_str(),f)==("0E+10",false)));

		let Ok((act, rem)) = parser.parse("30e+10") else {
			unreachable!()
		};
		assert_eq!(rem, "");
		assert!(matches!(act,(TextPresentation::Text(c),f) if (c.as_str(),f)==("30e+10",false)));

		let Ok((act, rem)) = parser.parse("30E+10") else {
			unreachable!()
		};
		assert_eq!(rem, "");
		assert!(matches!(act,(TextPresentation::Text(c),f) if (c.as_str(),f)==("30E+10",false)));

		let Ok((act, rem)) = parser.parse("30e-10") else {
			unreachable!()
		};
		assert_eq!(rem, "");
		assert!(matches!(act,(TextPresentation::Text(c),f) if (c.as_str(),f)==("30e-10",false)));

		let Ok((act, rem)) = parser.parse("30E-10") else {
			unreachable!()
		};
		assert_eq!(rem, "");
		assert!(matches!(act,(TextPresentation::Text(c),f) if (c.as_str(),f)==("30E-10",false)));

		let Ok((act, rem)) = parser.parse("30e10") else {
			unreachable!()
		};
		assert_eq!(rem, "");
		assert!(matches!(act,(TextPresentation::Text(c),f) if (c.as_str(),f)==("30e10",false)));

		let Ok((act, rem)) = parser.parse("30E10") else {
			unreachable!()
		};
		assert_eq!(rem, "");
		assert!(matches!(act,(TextPresentation::Text(c),f) if (c.as_str(),f)==("30E10",f)));

		should_panic(super::number_str(), "e");
		should_panic(super::number_str(), "e+");
		should_panic(super::number_str(), "e-");
	}
}
