use num::Complex;

pub struct Image {
	pub buf: Vec<u8>,
	pub cfg: ImageCfg,
}

pub struct ImageCfg {
	pub width: usize,
	pub height: usize,
	pub top_left: Complex<f64>,
	pub bot_right: Complex<f64>,
}

impl Image {
	pub fn new() {}

	pub fn render() {}
	pub fn render_parallel_lines() {}
	pub fn render_parallel_pixels() {}
	pub fn render_parallel_lines_scoped() {}
}
