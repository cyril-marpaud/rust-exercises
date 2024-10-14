pub fn fizz_buzz_functional() -> impl Iterator<Item = String> {}

#[test]
fn test_fizz_buzz_functional() {
	let mut fb = fizz_buzz_functional();

	assert_eq!(fb.next(), Some("1".to_string()));
	assert_eq!(fb.next(), Some("2".to_string()));
	assert_eq!(fb.next(), Some("Fizz".to_string()));
	assert_eq!(fb.next(), Some("4".to_string()));
	assert_eq!(fb.next(), Some("Buzz".to_string()));
	assert_eq!(fb.next(), Some("Fizz".to_string()));
	assert_eq!(fb.next(), Some("7".to_string()));
	assert_eq!(fb.next(), Some("8".to_string()));
	assert_eq!(fb.next(), Some("Fizz".to_string()));
	assert_eq!(fb.next(), Some("Buzz".to_string()));
	assert_eq!(fb.next(), Some("11".to_string()));
	assert_eq!(fb.next(), Some("Fizz".to_string()));
	assert_eq!(fb.next(), Some("13".to_string()));
	assert_eq!(fb.next(), Some("14".to_string()));
	assert_eq!(fb.next(), Some("FizzBuzz".to_string()));
}
