// pub fn fizzbuzz() {}

#[test]
fn zero() {
	assert_eq!(fizzbuzz(0), "fizzbuzz");
}

#[test]
fn one() {
	assert_eq!(fizzbuzz(1), "1");
}

#[test]
fn three() {
	assert_eq!(fizzbuzz(3), "fizz");
}

#[test]
fn five() {
	assert_eq!(fizzbuzz(5), "buzz");
}

#[test]
fn seven() {
	assert_eq!(fizzbuzz(7), "7");
}

#[test]
fn twelve() {
	assert_eq!(fizzbuzz(12), "fizz");
}

#[test]
fn fifteen() {
	assert_eq!(fizzbuzz(15), "fizzbuzz");
}

#[test]
fn seventeen() {
	assert_eq!(fizzbuzz(17), "17");
}

#[test]
fn twenty() {
	assert_eq!(fizzbuzz(20), "buzz");
}

#[test]
fn thirty() {
	assert_eq!(fizzbuzz(30), "fizzbuzz");
}
