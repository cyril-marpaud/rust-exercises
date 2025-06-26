pub fn fizzbuzz() {}

#[test]
fn zero() {
	assert_eq!(fizzbuzz(0), "FizzBuzz");
}

#[test]
fn one() {
	assert_eq!(fizzbuzz(1), "1");
}

#[test]
fn three() {
	assert_eq!(fizzbuzz(3), "Fizz");
}

#[test]
fn five() {
	assert_eq!(fizzbuzz(5), "Buzz");
}

#[test]
fn seven() {
	assert_eq!(fizzbuzz(7), "7");
}

#[test]
fn twelve() {
	assert_eq!(fizzbuzz(12), "Fizz");
}

#[test]
fn fifteen() {
	assert_eq!(fizzbuzz(15), "FizzBuzz");
}

#[test]
fn seventeen() {
	assert_eq!(fizzbuzz(17), "17");
}

#[test]
fn twenty() {
	assert_eq!(fizzbuzz(20), "Buzz");
}

#[test]
fn thirty() {
	assert_eq!(fizzbuzz(30), "FizzBuzz");
}
