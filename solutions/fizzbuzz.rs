pub fn fizzbuzz(n: u32) -> String {
	match (n % 3, n % 5) {
		(0, 0) => String::from("fizzbuzz"),
		(0, _) => String::from("fizz"),
		(_, 0) => String::from("buzz"),
		_ => format!("{n}"),
	}
}
