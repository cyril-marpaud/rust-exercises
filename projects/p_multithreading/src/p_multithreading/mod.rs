pub mod image;

pub fn cpx_to_escape() {}
pub fn parse_pair() {}
pub fn parse_cpx() {}

// #[test]
// #[rustfmt::skip]
// fn cpx_to_escape_times() {
// 	use num::Complex;
//
// 	assert_eq!(cpx_to_escape(Complex { re: -2.5, im: 1.0 }), 1);
// 	assert_eq!(cpx_to_escape(Complex { re: 0.79, im: 0.78 }), 2);
// 	assert_eq!(cpx_to_escape(Complex { re: -1.485, im: 1.0 }), 3);
// 	assert_eq!(cpx_to_escape(Complex { re: 0.125, im: 0.94 }), 4);
// 	assert_eq!(cpx_to_escape(Complex { re: 0.125, im: 0.92 }), 5);
// 	assert_eq!(cpx_to_escape(Complex { re: 0.125, im: 0.8 }), 6);
// 	assert_eq!(cpx_to_escape(Complex { re: 0.125, im: 0.74 }), 7);
// 	assert_eq!(cpx_to_escape(Complex { re: 0.125, im: 0.7 }), 8);
// 	assert_eq!(cpx_to_escape(Complex { re: -0.645, im: 0.52 }), 9);
// 	assert_eq!(cpx_to_escape(Complex { re: -1.24, im: 0.4 }), 10);
// 	assert_eq!(cpx_to_escape(Complex { re: -1.24, im: 0.38 }), 11);
// 	assert_eq!(cpx_to_escape(Complex { re: -0.855, im: 0.26 }), 14);
// 	assert_eq!(cpx_to_escape(Complex { re: -0.96, im: 0.28 }), 16);
// 	assert_eq!(cpx_to_escape(Complex { re: -0.645, im: 0.4 }), 23);
// 	assert_eq!(cpx_to_escape(Complex { re: -1.03, im: 0.28 }), 28);
// 	assert_eq!(cpx_to_escape(Complex { re: -0.645, im: 0.38 }), 40);
// 	assert_eq!(cpx_to_escape(Complex { re: -0.925, im: 0.26 }), 63);
// 	assert_eq!(cpx_to_escape(Complex { re: -1.24, im: 0.12 }), 74);
// 	assert_eq!(cpx_to_escape(Complex { re: -0.645, im: 0.36 }), 234);
// 	assert_eq!(cpx_to_escape(Complex { re: 0.125, im: 0.62 }), 255);
// }

// #[test]
// fn pair_parsing() {
// 	assert_eq!(parse_pair::<i32>("-42,13", ','), Some((-42, 13)));
// 	assert_eq!(parse_pair::<usize>("1920x1080", 'x'), Some((1920, 1080)));
// 	assert_eq!(parse_pair::<u8>("5:10", ':'), Some((5, 10)));
// 	assert_eq!(parse_pair::<f32>("3.15,2.71", ','), Some((3.15, 2.71)));
// 	assert_eq!(parse_pair::<f64>("-2.5,1.0", ','), Some((-2.5, 1.0)));
// 	assert_eq!(parse_pair::<f64>("0.0,0.0", ','), Some((0.0, 0.0)));
// 	assert_eq!(parse_pair::<i32>("42", ','), None);
// 	assert_eq!(parse_pair::<f64>("3.14", ','), None);
// 	assert_eq!(parse_pair::<usize>("1920", 'x'), None);
// 	assert_eq!(parse_pair::<i32>("abc,123", ','), None);
// 	assert_eq!(parse_pair::<i32>("123,def", ','), None);
// 	assert_eq!(parse_pair::<f64>("3.14,pi", ','), None);
// 	assert_eq!(parse_pair::<usize>("-5,10", ','), None);
// 	assert_eq!(parse_pair::<i32>("1,2,3", ','), None);
// 	assert_eq!(parse_pair::<f64>("1.0,2.0,3.0", ','), None);
// 	assert_eq!(parse_pair::<i32>(",13", ','), None);
// 	assert_eq!(parse_pair::<i32>("42,", ','), None);
// 	assert_eq!(parse_pair::<i32>(",", ','), None);
// }

// #[test]
// fn cpx_parsing() {
// 	use num::Complex;
//
// 	assert_eq!(parse_cpx("1.0,2.0"), Some(Complex { re: 1.0, im: 2.0 }));
// 	assert_eq!(parse_cpx("-2.5,1.0"), Some(Complex { re: -2.5, im: 1.0 }));
// 	assert_eq!(parse_cpx("0.0,0.0"), Some(Complex { re: 0.0, im: 0.0 }));
// 	assert_eq!(parse_cpx("3.1,-2.71"), Some(Complex { re: 3.1, im: -2.71 }));
// 	assert_eq!(parse_cpx("-1.2,0.35"), Some(Complex { re: -1.2, im: 0.35 }));
// 	assert_eq!(parse_cpx("42.0,0.0"), Some(Complex { re: 42.0, im: 0.0 }));
// 	assert_eq!(parse_cpx("0.0,-13.3"), Some(Complex { re: 0.0, im: -13.3 }));
// 	assert_eq!(parse_cpx("1.0"), None);
// 	assert_eq!(parse_cpx("1.0 2.0"), None);
// 	assert_eq!(parse_cpx("1.0;2.0"), None);
// 	assert_eq!(parse_cpx("abc,def"), None);
// 	assert_eq!(parse_cpx("1.0,xyz"), None);
// 	assert_eq!(parse_cpx("pi,2.0"), None);
// 	assert_eq!(parse_cpx(",2.0"), None);
// 	assert_eq!(parse_cpx("1.0,"), None);
// 	assert_eq!(parse_cpx(","), None);
// 	assert_eq!(parse_cpx("1.0,2.0,3.0"), None);
// 	assert_eq!(parse_cpx(""), None);
// }
