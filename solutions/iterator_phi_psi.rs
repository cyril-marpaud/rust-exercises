pub struct Fibonacci;

pub struct FibIter {
	phi_n: f64,
	psi_n: f64,
}

impl IntoIterator for Fibonacci {
	type Item = <FibIter as Iterator>::Item;

	type IntoIter = FibIter;

	fn into_iter(self) -> Self::IntoIter {
		FibIter {
			phi_n: 1.,
			psi_n: 1.,
		}
	}
}

impl Iterator for FibIter {
	type Item = u64;

	fn next(&mut self) -> Option<Self::Item> {
		let res = ((self.phi_n - self.psi_n) / 5f64.sqrt()).round();

		self.phi_n *= (1. + 5f64.sqrt()) / 2.;
		self.psi_n *= (1. - 5f64.sqrt()) / 2.;

		Some(res as u64)
	}
}
