use crate::text_presentation::TextPresentation;
use combine as cmb;
use combine::parser::char as chr;
use combine::{Parser, Stream};
use std::collections::HashMap;
use std::hash::{BuildHasher, DefaultHasher, Hash, Hasher};
use std::net::ToSocketAddrs;

mod poc;
mod sample;
mod text_presentation;

pub struct Foo(String);

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

fn main() {
	let mut zero = chr::char::<&str>('0').skip(cmb::not_followed_by(chr::char('0')));

	let ret = zero.parse("00");

	println!("{ret:?}")
}
