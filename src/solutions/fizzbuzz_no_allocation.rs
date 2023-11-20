use std::borrow::Cow;

pub fn fizzbuzz(n: u32) -> Cow<'static, str> {
	match (n % 3, n % 5) {
		(0, 0) => Cow::Borrowed("fizzbuzz"),
		(0, _) => Cow::Borrowed("fizz"),
		(_, 0) => Cow::Borrowed("buzz"),
		_ => Cow::Owned(format!("{n}")),
	}
}
