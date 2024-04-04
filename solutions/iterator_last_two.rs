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

#[test]
fn iterator() {
	let mut i = Fibonacci.into_iter();

	assert_eq!(i.next(), Some(0));
	assert_eq!(i.next(), Some(1));
	assert_eq!(i.next(), Some(1));
	assert_eq!(i.next(), Some(2));
	assert_eq!(i.next(), Some(3));
	assert_eq!(i.next(), Some(5));
	assert_eq!(i.next(), Some(8));
	assert_eq!(i.next(), Some(13));
	assert_eq!(i.next(), Some(21));
	assert_eq!(i.next(), Some(34));
	assert_eq!(i.next(), Some(55));
	assert_eq!(i.next(), Some(89));
	assert_eq!(i.next(), Some(144));
	assert_eq!(i.next(), Some(233));
	assert_eq!(i.next(), Some(377));
	assert_eq!(i.next(), Some(610));
	assert_eq!(i.next(), Some(987));
	assert_eq!(i.next(), Some(1597));
	assert_eq!(i.next(), Some(2584));
	assert_eq!(i.next(), Some(4181));
}
