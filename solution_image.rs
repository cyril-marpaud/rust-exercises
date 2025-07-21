use std::{
	thread::scope,
	time::{Duration, Instant},
};

use num::Complex;
use rayon::{
	iter::{IndexedParallelIterator, ParallelIterator},
	slice::ParallelSliceMut,
};

use super::pxl_to_escape;

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
	pub fn new(cfg: ImageCfg) -> Self {
		Self {
			buf: vec![0; cfg.width * cfg.height],
			cfg,
		}
	}

	pub fn render(&mut self) -> Duration {
		let instant = Instant::now();
		let mut i = 0;
		(0..self.cfg.height).for_each(|row| {
			(0..self.cfg.width).for_each(|col| {
				self.buf[i] = pxl_to_escape((row, col), &self.cfg);
				i += 1;
			});
		});
		instant.elapsed()
	}

	pub fn render_parallel_lines(&mut self) -> Duration {
		let instant = Instant::now();
		self.buf
			.par_chunks_exact_mut(self.cfg.width)
			.enumerate()
			.for_each(|(row, chunk)| {
				(0..self.cfg.width).for_each(|col| {
					chunk[col] = pxl_to_escape((row, col), &self.cfg);
				});
			});
		instant.elapsed()
	}

	pub fn render_parallel_pixels(&mut self) -> Duration {
		let instant = Instant::now();
		self.buf
			.par_chunks_exact_mut(1)
			.enumerate()
			.for_each(|(i, chunk)| {
				let (row, col) = (i / self.cfg.width, i % self.cfg.width);
				chunk[0] = pxl_to_escape((row, col), &self.cfg);
			});
		instant.elapsed()
	}

	pub fn render_parallel_lines_scoped(&mut self, nb_threads: usize) -> Duration {
		let instant = Instant::now();
		let chunks = self.buf.chunks_exact_mut(self.cfg.width).enumerate();
		let mutex = std::sync::Mutex::new(chunks);

		scope(|s| {
			(0..nb_threads).for_each(|_| {
				s.spawn(|| {
					while let Some((row, chunk)) = { mutex.lock().unwrap().next() } {
						(0..self.cfg.width).for_each(|col| {
							chunk[col] = pxl_to_escape((row, col), &self.cfg);
						});
					}
				});
			});
		});
		instant.elapsed()
	}
}
