pub fn fibonacci(n: u32) -> u64 {
	// assert!(n <= i32::MAX as u32);

	let phi = (1.0 + 5_f64.sqrt()) / 2.0;
	let psi = 1.0 - phi;

	((phi.powi(n as i32) - psi.powi(n as i32)) / 5_f64.sqrt()).round() as u64
}
