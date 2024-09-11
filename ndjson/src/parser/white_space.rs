use super::trimmed_output::TrimmedOutput;
use combine as cmb;
use combine::parser::char as chr;
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

// pub fn trim_left<I: Stream<Token = char>>(
// 	parser: impl Parser<I, Output = String>,
// ) -> impl Parser<I, Output = TrimmedOutput> {
// 	(ws(), parser).map(|(w, p)| TrimmedOutput::new(format!("{w}{p}")))
// }
//
// pub fn trim_right<I: Stream<Token = char>>(
// 	parser: impl Parser<I, Output = String>,
// ) -> impl Parser<I, Output = TrimmedOutput> {
// 	(parser, ws()).map(|(p, w)| TrimmedOutput::new(format!("{p}{w}")))
// }
//
// pub fn trim<I: Stream<Token = char>>(
// 	parser: impl Parser<I, Output = String>,
// ) -> impl Parser<I, Output = TrimmedOutput> {
// 	(ws(), parser, ws()).map(|(l, v, r)| TrimmedOutput::new(format!("{}{}{}", l, v, r)))
// }

#[cfg(test)]
mod test {
	use super::super::trimmed_output::test_helper::{assert, WS};
	use super::*;
	use combine::parser::char as chr;

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

	// #[test]
	// fn trim_left() {
	// 	let input = format!("{}value{}", WS.as_str(), WS.as_str());
	// 	let p = chr::string::<&str>("value").map(|x| x.to_string());
	// 	let mut p = super::trim_left(p);
	// 	let fixture = p.parse(input.as_str()).unwrap();
	//
	// 	let (f, r) = p.parse(input.as_str()).unwrap();
	//
	// 	assert(&fixture, Some(&WS), &format!("{}value", WS.as_str()));
	// }
	//
	// #[test]
	// fn trim_right() {
	// 	let input = format!("{}value{}", WS.as_str(), WS.as_str());
	// 	let p = chr::string::<&str>("value").map(|x| x.to_string());
	// 	let mut p = super::trim_right(p);
	//
	// 	assert!(p.parse(&input).is_err());
	//
	// 	let input = format!("value{}", WS.as_str());
	//
	// 	let p = chr::string::<&str>("value").map(|x| x.to_string());
	// 	let mut p = super::trim_right(p);
	//
	// 	let fixture = p.parse(&input).unwrap();
	// 	assert(&fixture, None, &input);
	// }
	//
	// #[test]
	// fn trim() {
	// 	let input = format!("{}{}value{}", WS.as_str(), WS.as_str(), WS.as_str());
	// 	let p = chr::string::<&str>("value").map(|x| x.to_string());
	// 	let mut p = super::trim(p);
	// 	let fixture = p.parse(input.as_str()).unwrap();
	// 	assert(&fixture, None, &input);
	// }
}
