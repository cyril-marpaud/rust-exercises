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
