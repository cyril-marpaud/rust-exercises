struct Fibonacci;
struct FibIter(u32);

impl Fibonacci {
	pub fn fibonacci(n: u32) -> u64 {
		let phi = (1.0 + 5_f64.sqrt()) / 2.0;
		let psi = 1.0 - phi;

		((phi.powi(n as i32) - psi.powi(n as i32)) / 5_f64.sqrt()).round() as u64
	}
}

impl IntoIterator for Fibonacci {
	type Item = <FibIter as Iterator>::Item;
	type IntoIter = FibIter;

	fn into_iter(self) -> Self::IntoIter {
		FibIter(0)
	}
}

impl Iterator for FibIter {
	type Item = u64;

	fn next(&mut self) -> Option<Self::Item> {
		let next = Some(Fibonacci::fibonacci(self.0));
		self.0 += 1;
		next
	}
}
