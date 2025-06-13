use std::borrow::Cow;

pub fn fizzbuzz(n: u32) -> Cow<'static, str> {
	match (n % 3, n % 5) {
		(0, 0) => Cow::Borrowed("FizzBuzz"),
		(0, _) => Cow::Borrowed("Fizz"),
		(_, 0) => Cow::Borrowed("Buzz"),
		_ => Cow::Owned(n.to_string()),
	}
}
