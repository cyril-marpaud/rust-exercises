pub mod image_cfg;

use image_cfg::ImageCfg;

pub struct Image {
	pub buf: Vec<u8>,
	pub cfg: ImageCfg,
}

impl Image {
	pub fn new() {}

	pub fn render() {}
	pub fn render_parallel_lines() {}
	pub fn render_parallel_pixels() {}
	pub fn render_parallel_lines_scoped() {}

	pub fn write() {}
}
