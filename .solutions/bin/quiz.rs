use anyhow::Result;

fn main() -> Result<()> {
	let _ = std::env::set_current_dir("e05_quiz");
	e05_quiz::e05_1_quiz::play()?;
	Ok(())
}
