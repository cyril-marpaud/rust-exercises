pub struct Fibonacci;

pub struct FibIter {
	one: u64,
	two: u64,
}

impl IntoIterator for Fibonacci {
	type Item = <FibIter as Iterator>::Item;

	type IntoIter = FibIter;

	fn into_iter(self) -> Self::IntoIter {
		FibIter { one: 0, two: 1 }
	}
}

impl Iterator for FibIter {
	type Item = u64;

	fn next(&mut self) -> Option<Self::Item> {
		let current = self.one;
		self.one = self.two;
		self.two = current + self.one;
		Some(current)
	}
}
