use std::iter::{once, repeat_n};

pub fn fizz_buzz_functional() -> impl Iterator<Item = String> {
	let fizz = repeat_n("", 2).chain(once("Fizz")).cycle();
	let buzz = repeat_n("", 4).chain(once("Buzz")).cycle();

	fizz.zip(buzz).enumerate().map(|(i, (f, b))| {
		let i = i + 1;
		match i % 3 == 0 || i % 5 == 0 {
			true => format!("{f}{b}"),
			false => i.to_string(),
		}
	})
}
