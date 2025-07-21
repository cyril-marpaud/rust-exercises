use num::Complex;

pub type Pixel = (usize, usize);

pub struct ImageCfg {
	pub width: usize,
	pub height: usize,
	pub top_left: Complex<f64>,
	pub bot_right: Complex<f64>,
}

impl ImageCfg {
	pub fn pxl_to_cpx() {}
	pub fn pxl_to_escape() {}
}

// #[test]
// fn pxl_to_cpx_mapping() {
// 	let cfg = ImageCfg {
// 		width: 5,
// 		height: 5,
// 		top_left: Complex { re: 0.0, im: 4.0 },
// 		bot_right: Complex { re: 4.0, im: 0.0 },
// 	};
//
// 	assert_eq!(cfg.pxl_to_cpx((0, 0)), cfg.top_left);
// 	assert_eq!(cfg.pxl_to_cpx((0, 1)), Complex { re: 0.0, im: 3.0 });
// 	assert_eq!(cfg.pxl_to_cpx((0, 2)), Complex { re: 0.0, im: 2.0 });
// 	assert_eq!(cfg.pxl_to_cpx((0, 3)), Complex { re: 0.0, im: 1.0 });
// 	assert_eq!(cfg.pxl_to_cpx((0, 4)), Complex { re: 0.0, im: 0.0 });
// 	assert_eq!(cfg.pxl_to_cpx((1, 0)), Complex { re: 1.0, im: 4.0 });
// 	assert_eq!(cfg.pxl_to_cpx((1, 1)), Complex { re: 1.0, im: 3.0 });
// 	assert_eq!(cfg.pxl_to_cpx((1, 2)), Complex { re: 1.0, im: 2.0 });
// 	assert_eq!(cfg.pxl_to_cpx((1, 3)), Complex { re: 1.0, im: 1.0 });
// 	assert_eq!(cfg.pxl_to_cpx((1, 4)), Complex { re: 1.0, im: 0.0 });
// 	assert_eq!(cfg.pxl_to_cpx((2, 0)), Complex { re: 2.0, im: 4.0 });
// 	assert_eq!(cfg.pxl_to_cpx((2, 1)), Complex { re: 2.0, im: 3.0 });
// 	assert_eq!(cfg.pxl_to_cpx((2, 2)), Complex { re: 2.0, im: 2.0 });
// 	assert_eq!(cfg.pxl_to_cpx((2, 3)), Complex { re: 2.0, im: 1.0 });
// 	assert_eq!(cfg.pxl_to_cpx((2, 4)), Complex { re: 2.0, im: 0.0 });
// 	assert_eq!(cfg.pxl_to_cpx((3, 0)), Complex { re: 3.0, im: 4.0 });
// 	assert_eq!(cfg.pxl_to_cpx((3, 1)), Complex { re: 3.0, im: 3.0 });
// 	assert_eq!(cfg.pxl_to_cpx((3, 2)), Complex { re: 3.0, im: 2.0 });
// 	assert_eq!(cfg.pxl_to_cpx((3, 3)), Complex { re: 3.0, im: 1.0 });
// 	assert_eq!(cfg.pxl_to_cpx((3, 4)), Complex { re: 3.0, im: 0.0 });
// 	assert_eq!(cfg.pxl_to_cpx((4, 0)), Complex { re: 4.0, im: 4.0 });
// 	assert_eq!(cfg.pxl_to_cpx((4, 1)), Complex { re: 4.0, im: 3.0 });
// 	assert_eq!(cfg.pxl_to_cpx((4, 2)), Complex { re: 4.0, im: 2.0 });
// 	assert_eq!(cfg.pxl_to_cpx((4, 3)), Complex { re: 4.0, im: 1.0 });
// 	assert_eq!(cfg.pxl_to_cpx((4, 4)), cfg.bot_right);
// }

// #[test]
// fn pxl_to_escape_mapping() {
// 	let cfg = ImageCfg {
// 		width: 100,
// 		height: 100,
// 		top_left: Complex { re: -2.5, im: 1.0 },
// 		bot_right: Complex { re: 1.0, im: -1.0 },
// 	};
//
// 	assert_eq!(cfg.pxl_to_escape((65, 9)), 146);
// 	assert_eq!(cfg.pxl_to_escape((70, 11)), 208);
// 	assert_eq!(cfg.pxl_to_escape((65, 16)), 169);
// 	assert_eq!(cfg.pxl_to_escape((64, 17)), 143);
// 	assert_eq!(cfg.pxl_to_escape((58, 22)), 180);
// 	assert_eq!(cfg.pxl_to_escape((36, 42)), 176);
// 	assert_eq!(cfg.pxl_to_escape((32, 49)), 178);
// 	assert_eq!(cfg.pxl_to_escape((81, 56)), 145);
// 	assert_eq!(cfg.pxl_to_escape((50, 60)), 46);
// 	assert_eq!(cfg.pxl_to_escape((52, 67)), 176);
// 	assert_eq!(cfg.pxl_to_escape((81, 75)), 241);
// 	assert_eq!(cfg.pxl_to_escape((62, 81)), 119);
// 	assert_eq!(cfg.pxl_to_escape((64, 86)), 234);
// 	assert_eq!(cfg.pxl_to_escape((66, 91)), 225);
// }
