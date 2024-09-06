mod poc;
mod sample;

fn main() {
	let a = Result::<i32, i32>::Ok(100i32);

	for i in a {
		print!("{i}")
	}
}
