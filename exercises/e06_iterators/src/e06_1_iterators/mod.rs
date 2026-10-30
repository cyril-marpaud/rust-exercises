pub struct Fibonacci;

#[test]
fn iterator() {
	let mut i = Fibonacci.into_iter();

	assert_eq!(i.next(), Some(0));
	assert_eq!(i.next(), Some(1));
	assert_eq!(i.next(), Some(1));
	assert_eq!(i.next(), Some(2));
	assert_eq!(i.next(), Some(3));
	assert_eq!(i.next(), Some(5));
	assert_eq!(i.next(), Some(8));
	assert_eq!(i.next(), Some(13));
	assert_eq!(i.next(), Some(21));
	assert_eq!(i.next(), Some(34));
	assert_eq!(i.next(), Some(55));
	assert_eq!(i.next(), Some(89));
	assert_eq!(i.next(), Some(144));
	assert_eq!(i.next(), Some(233));
	assert_eq!(i.next(), Some(377));
	assert_eq!(i.next(), Some(610));
	assert_eq!(i.next(), Some(987));
	assert_eq!(i.next(), Some(1597));
	assert_eq!(i.next(), Some(2584));
	assert_eq!(i.next(), Some(4181));
}

// #[test]
// fn bonus_overflow() {
// 	let mut i = Fibonacci.into_iter();
//
// 	assert_eq!(i.nth(186), Some(332825110087067562321196029789634457848));
// 	assert_eq!(i.next(), None);
// }
