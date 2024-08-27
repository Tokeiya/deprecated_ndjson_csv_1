mod sample;

fn main() {
	let c = '\u{000020}';

	println!("{c}");
	println!("{}", c == '\u{000020}')
}
