use std::file;
use std::fs::File;
use std::env;
use std::io::Read;
use combine::Parser;
use ndjson::parser::value::macro_expand::value as value_parser;

fn main() {
	let mut file = File::open("./artifact/sample0.json").unwrap();
	let mut str = String::new();
	_ = file.read_to_string(&mut str).unwrap();


	let str = r#"      10        "#;
	let mut parser = value_parser::<&str>();
	let result = parser.parse(str);

	println!("{}", result.is_err())
}
