use super::super::elements::value::Value as ElemValue;
use super::array::array;
use super::boolean_parser::boolean;
use super::null::null;
use super::number::number;
use super::object::object;
use super::string::string;
use combine::{Parser, Stream};

fn value_<I: Stream<Token = char>>() -> impl Parser<I, Output = ElemValue> {
	let bool = boolean();
	let null = null();
	let obj = object();
	let arr = array();
	let num = number();
	let str = string();

	bool.or(null).or(obj).or(arr).or(num).or(str)
}

pub mod macro_expand {
	use super::*;
	#[allow(non_camel_case_types)]
	#[doc(hidden)]
	pub struct value<Input>
	where
		<Input as ::combine::stream::StreamOnce>::Error: ::combine::error::ParseError<
			<Input as ::combine::stream::StreamOnce>::Token,
			<Input as ::combine::stream::StreamOnce>::Range,
			<Input as ::combine::stream::StreamOnce>::Position,
		>,
		Input: Stream,
		Input: Stream<Token = char>,
	{
		__marker: ::combine::lib::marker::PhantomData<fn(Input) -> ElemValue>,
	}
	#[allow(non_shorthand_field_patterns)]
	impl<Input> ::combine::Parser<Input> for value<Input>
	where
		<Input as ::combine::stream::StreamOnce>::Error: ::combine::error::ParseError<
			<Input as ::combine::stream::StreamOnce>::Token,
			<Input as ::combine::stream::StreamOnce>::Range,
			<Input as ::combine::stream::StreamOnce>::Position,
		>,
		Input: ::combine::stream::Stream,
		Input: Stream<Token = char>,
	{
		type Output = ElemValue;
		type PartialState = ();
		#[inline]
		fn parse_partial(
			&mut self,
			input: &mut Input,
			state: &mut Self::PartialState,
		) -> ::combine::error::ParseResult<Self::Output, <Input as ::combine::StreamOnce>::Error>
		{
			self.parse_mode(::combine::parser::PartialMode::default(), input, state)
		}
		#[inline]
		fn parse_first(
			&mut self,
			input: &mut Input,
			state: &mut Self::PartialState,
		) -> ::combine::error::ParseResult<Self::Output, <Input as ::combine::StreamOnce>::Error>
		{
			self.parse_mode(::combine::parser::FirstMode, input, state)
		}
		#[inline]
		fn parse_mode_impl<M>(
			&mut self,
			mode: M,
			input: &mut Input,
			state: &mut Self::PartialState,
		) -> ::combine::error::ParseResult<ElemValue, <Input as ::combine::stream::StreamOnce>::Error>
		where
			M: ::combine::parser::ParseMode,
		{
			let value { .. } = *self;
			{
				let _ = state;
				let mut state = Default::default();
				let state = &mut state;
				{ value_() }.parse_mode(mode, input, state)
			}
		}
		#[inline]
		fn add_error(
			&mut self,
			errors: &mut ::combine::error::Tracked<<Input as ::combine::stream::StreamOnce>::Error>,
		) {
			let value { .. } = *self;
			let mut parser = { value_() };
			{
				let _: &mut dyn ::combine::Parser<Input, Output = ElemValue, PartialState = _> =
					&mut parser;
			}
			parser.add_error(errors)
		}
		fn add_committed_expected_error(
			&mut self,
			errors: &mut ::combine::error::Tracked<<Input as ::combine::stream::StreamOnce>::Error>,
		) {
			let value { .. } = *self;
			let mut parser = { value_() };
			{
				let _: &mut dyn ::combine::Parser<Input, Output = ElemValue, PartialState = _> =
					&mut parser;
			}
			parser.add_committed_expected_error(errors)
		}
	}
	#[inline]
	pub fn value<Input>() -> value<Input>
	where
		<Input as ::combine::stream::StreamOnce>::Error: ::combine::error::ParseError<
			<Input as ::combine::stream::StreamOnce>::Token,
			<Input as ::combine::stream::StreamOnce>::Range,
			<Input as ::combine::stream::StreamOnce>::Position,
		>,
		Input: ::combine::stream::Stream,
		Input: Stream<Token = char>,
	{
		value {
			__marker: ::combine::lib::marker::PhantomData,
		}
	}
}

#[cfg(test)]
mod test {
	use super::macro_expand::value;
	use super::*;
	use crate::elements::number_value::test_helper::{is_float, is_integer};

	#[test]
	fn null() {
		let mut parser = value::<&str>();
		let (n, _) = parser.parse("null").unwrap();
		_ = n.extract_null();
	}

	#[test]
	fn string() {
		let mut parser = value::<&str>();
		let (s, _) = parser.parse(r#""hello""#).unwrap();
		assert_eq!(s.extract_string().value(), "hello")
	}

	#[test]
	fn number() {
		let mut parser = value::<&str>();
		let (n, _) = parser.parse("42").unwrap();
		is_integer(n.extract_number(), 42);

		let (n, _) = parser.parse("42.0").unwrap();
		is_float(n.extract_number(), 42.0);
	}

	#[test]
	fn array() {
		let mut parser = value::<&str>();
		let (a, _) = parser.parse(r#"[1,2,3]"#).unwrap();
		let a = a.extract_array().contents();

		is_integer(a[0].extract_number(), 1);
		is_integer(a[1].extract_number(), 2);
		is_integer(a[2].extract_number(), 3);
	}

	#[test]
	fn object() {
		let mut parser = value::<&str>();
		let (o, _) = parser.parse(r#"{"key":42}"#).unwrap();
		let o = o.extract_object().content();

		assert_eq!(o.len(), 1);
		assert_eq!(o[0].key().value(), "key");
		assert_eq!(o[0].key().raw_text(), r#""key""#);

		is_integer(o[0].value().extract_number(), 42);
		assert_eq!(o[0].value().extract_number().raw_text(), "42");
	}
}
