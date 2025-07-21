use std::fs::File;

use mandelbrot::{
	image::{Image, ImageCfg},
	parse_cpx, parse_pair, write_img,
};
use poloto::build::plot;

pub const FILENAME_SEQ: &str = "sequential.png";
pub const FILENAME_LINES: &str = "lines.png";
pub const FILENAME_PIXELS: &str = "pixels.png";
pub const FILENAME_SCOPED: &str = "scoped.png";
pub const FILENAME_PLOT: &str = "plot.svg";

pub const MAX_THREADS: usize = 25;

fn main() {
	let mut args = std::env::args().skip(1);

	let (width, height) = parse_pair::<usize>(&args.next().unwrap(), 'x').unwrap();
	let top_left = parse_cpx(&args.next().unwrap()).unwrap();
	let bot_right = parse_cpx(&args.next().unwrap()).unwrap();

	let mut img = Image::new(ImageCfg {
		width,
		height,
		top_left,
		bot_right,
	});

	let duration_seq = img.render().as_secs_f64();
	println!("Sequential duration: {duration_seq}");
	write_img(FILENAME_SEQ, &img).unwrap();

	let duration = img.render_parallel_lines().as_secs_f64();
	let ratio = duration_seq / duration;
	println!("Parallel (lines) duration: {duration}");
	println!("Parallel (lines) ratio: {ratio}");
	write_img(FILENAME_LINES, &img).unwrap();

	let duration = img.render_parallel_pixels().as_secs_f64();
	let ratio = duration_seq / duration;
	println!("Parallel (pixels) duration: {duration}");
	println!("Parallel (pixels) ratio: {ratio}");
	write_img(FILENAME_PIXELS, &img).unwrap();

	let mut durations = Vec::with_capacity(MAX_THREADS);
	let mut ratios = Vec::with_capacity(MAX_THREADS);
	(1..MAX_THREADS).for_each(|threads| {
		let duration = img.render_parallel_lines_scoped(threads).as_secs_f64();
		let ratio = duration_seq / duration;
		println!("Parallel (scoped, t={threads}) duration: {duration}");
		println!("Parallel (scoped, t={threads}) ratio: {ratio}");
		durations.push([threads as f64, duration]);
		ratios.push([threads as f64, ratio]);
	});
	write_img(FILENAME_SCOPED, &img).unwrap();

	let first_bissector = plot("Linear ratio").line((0..MAX_THREADS).map(|i| [i as f64, i as f64]));
	let durations = plot("Duration").line(durations);
	let ratios = plot("Ratio").line(ratios);

	let plot_file = File::create(FILENAME_PLOT).unwrap();
	poloto::frame_build()
		.data([ratios, durations, first_bissector])
		.build_and_label(("Mandelbrot performance", "Threads", "Ratio"))
		.append_to(poloto::header().light_theme())
		.render_io_write(plot_file)
		.unwrap();
}
