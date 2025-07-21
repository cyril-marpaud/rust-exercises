pub mod image;

use std::{self, fs::File, io, str::FromStr};

use ::image::{ExtendedColorType::L8, ImageEncoder, codecs::png::PngEncoder};
use num::Complex;

use crate::image::{Image, ImageCfg};

pub type Pixel = (usize, usize);

pub fn escape_time(c: Complex<f64>) -> u8 {
	let mut z = Complex::<f64>::default();

	(0..=u8::MAX)
		.position(|_| {
			let escape = z.norm_sqr() > 4.0;
			z = z * z + c;
			escape
		})
		.unwrap_or(u8::MAX as usize) as u8
}

pub fn map_pxl_to_cpx((row, col): Pixel, cfg: &ImageCfg) -> Complex<f64> {
	let cpx_w = cfg.bot_right.re - cfg.top_left.re;
	let cpx_h = cfg.top_left.im - cfg.bot_right.im;

	let re = cfg.top_left.re + cpx_w * (col as f64 / cfg.width as f64);
	let im = cfg.top_left.im - cpx_h * (row as f64 / cfg.height as f64);

	Complex { re, im }
}

pub fn parse_pair<T: FromStr>(s: &str, sep: char) -> Option<(T, T)> {
	let pair = s
		.split_once(sep)
		.map(|(one, two)| -> Result<(T, T), <T as FromStr>::Err> {
			Ok((one.parse::<T>()?, two.parse::<T>()?))
		})?
		.ok()?;

	Some(pair)
}

pub fn parse_cpx(s: &str) -> Option<Complex<f64>> {
	parse_pair::<f64>(s, ',').map(|(re, im)| Complex { re, im })
}

pub fn pxl_to_escape(pxl: Pixel, cfg: &ImageCfg) -> u8 {
	let cpx = map_pxl_to_cpx(pxl, cfg);
	u8::MAX - escape_time(cpx)
}

pub fn write_img(filename: &str, img: &Image) -> Result<(), io::Error> {
	let file = File::create(filename)?;

	PngEncoder::new(file)
		.write_image(&img.buf, img.cfg.width as u32, img.cfg.height as u32, L8)
		.map_err(|e| io::Error::other(format!("Failed to write img: {e:?}")))
}
