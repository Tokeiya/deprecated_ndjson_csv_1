use std::collections::HashMap;
use std::hash::{BuildHasher, DefaultHasher, Hash, Hasher};
use std::net::ToSocketAddrs;

mod poc;
mod sample;

pub struct Foo(String);

fn main() {
	let s = "hello".to_string();
	let mut h = DefaultHasher::new();
}
