// fn fibonacci() {}

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
fn fifty() {
	assert_eq!(fibonacci(50), 12586269025);
}

#[test]
fn seventy() {
	assert_eq!(fibonacci(70), 190392490709135);
}
