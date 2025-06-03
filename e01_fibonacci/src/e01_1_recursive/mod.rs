pub fn fibonacci() {}

#[test]
fn zero() {
	assert_eq!(fibonacci(0), 0);
}

#[test]
fn one() {
	assert_eq!(fibonacci(1), 1);
}

#[test]
fn three() {
	assert_eq!(fibonacci(3), 2);
}

#[test]
fn five() {
	assert_eq!(fibonacci(5), 5);
}

#[test]
fn ten() {
	assert_eq!(fibonacci(10), 55);
}

#[test]
fn twenty() {
	assert_eq!(fibonacci(20), 6765);
}

#[test]
fn thirty() {
	assert_eq!(fibonacci(30), 832040);
}

#[test]
fn forty() {
	assert_eq!(fibonacci(40), 102334155);
}

#[test]
fn rand() {
	(0..10).for_each(|_| {
		let r = rand::random_range(2..30);
		assert_eq!(fibonacci(r), fibonacci(r - 1) + fibonacci(r - 2));
	});
}
