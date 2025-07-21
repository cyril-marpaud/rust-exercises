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

// #[cfg(test)]
// mod tests {
// 	use super::*;
//
// 	const EXPECTED_HASH: &str = "2e83b91a08f9566839c9331459000645590b489262bf393af5c7f6337ae23b33";
//
// 	fn check_hash(img: &Image) {
// 		assert_eq!(blake3::hash(&img.buf).to_hex().as_str(), EXPECTED_HASH,);
// 	}
//
// 	fn make_img() -> Image {
// 		let cfg = ImageCfg {
// 			width: 480,
// 			height: 270,
// 			top_left: num::Complex { re: -2.5, im: 1.0 },
// 			bot_right: num::Complex { re: 1.0, im: -1.0 },
// 		};
// 		Image::new(cfg)
// 	}
//
// 	#[test]
// 	fn render_hash() {
// 		let mut img = make_img();
// 		img.render();
// 		check_hash(&img);
// 	}
//
// 	#[test]
// 	fn render_parallel_lines_hash() {
// 		let mut img = make_img();
// 		img.render_parallel_lines();
// 		check_hash(&img);
// 	}
//
// 	#[test]
// 	fn render_parallel_pixels_hash() {
// 		let mut img = make_img();
// 		img.render_parallel_pixels();
// 		check_hash(&img);
// 	}
//
// 	#[test]
// 	fn render_parallel_lines_scoped_hash() {
// 		let mut img = make_img();
// 		(5..10).for_each(|nb_threads| {
// 			img.render_parallel_lines_scoped(nb_threads);
// 			check_hash(&img);
// 		});
// 	}
// }
