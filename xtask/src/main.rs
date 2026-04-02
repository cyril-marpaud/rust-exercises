mod git;
mod reports;

use anyhow::Result;

fn main() -> Result<()> {
	match std::env::args().nth(1).as_deref() {
		Some("reports") => reports::reports(),
		Some(unknown) => anyhow::bail!("Unknown command: {unknown}"),
		None => anyhow::bail!("No command provided. Available: reports"),
	}
}
