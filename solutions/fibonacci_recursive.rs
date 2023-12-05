pub fn fibonacci(n: u32) -> u64 {
	match n {
		(0 | 1) => n as u64,
		n => fibonacci(n - 1) + fibonacci(n - 2),
	}
}
