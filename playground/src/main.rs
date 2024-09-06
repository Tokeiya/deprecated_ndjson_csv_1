mod poc;
mod sample;

fn main() {
	let foo = "  \t  hello \r \n  ";

	let a = foo.trim();

	println!("{foo}");
	println!("{a}:{}", a.len())
}
