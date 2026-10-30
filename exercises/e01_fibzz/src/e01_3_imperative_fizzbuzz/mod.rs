pub fn fizzbuzz() {}

#[test]
fn bonus_zero() {
	assert_eq!(fizzbuzz(0), "FizzBuzz");
}

#[test]
fn bonus_one() {
	assert_eq!(fizzbuzz(1), "1");
}

#[test]
fn bonus_three() {
	assert_eq!(fizzbuzz(3), "Fizz");
}

#[test]
fn bonus_five() {
	assert_eq!(fizzbuzz(5), "Buzz");
}

#[test]
fn bonus_seven() {
	assert_eq!(fizzbuzz(7), "7");
}

#[test]
fn bonus_twelve() {
	assert_eq!(fizzbuzz(12), "Fizz");
}

#[test]
fn bonus_fifteen() {
	assert_eq!(fizzbuzz(15), "FizzBuzz");
}

#[test]
fn bonus_seventeen() {
	assert_eq!(fizzbuzz(17), "17");
}

#[test]
fn bonus_twenty() {
	assert_eq!(fizzbuzz(20), "Buzz");
}

#[test]
fn bonus_thirty() {
	assert_eq!(fizzbuzz(30), "FizzBuzz");
}
