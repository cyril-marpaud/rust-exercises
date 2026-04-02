use anyhow::Result;
use xshell::{Shell, cmd};

pub fn check_clean_state(sh: &Shell) -> Result<()> {
	if !cmd!(sh, "git status --porcelain").read()?.trim().is_empty() {
		anyhow::bail!("Working directory is not clean.");
	}
	Ok(())
}

pub fn checkout_branch(sh: &Shell, branch: &str) -> Result<()> {
	cmd!(sh, "git checkout -q {branch}").run()?;
	Ok(())
}

pub fn fetch(sh: &Shell) -> Result<()> {
	cmd!(sh, "git fetch -q").run()?;
	Ok(())
}

pub fn get_current_branch(sh: &Shell) -> Result<String> {
	Ok(cmd!(sh, "git rev-parse --abbrev-ref HEAD")
		.read()?
		.trim()
		.to_string())
}

pub fn get_learner_branches(sh: &Shell) -> Result<Vec<String>> {
	let output = cmd!(
		sh,
		"git for-each-ref refs/remotes/origin/ --format=%(refname:lstrip=3) --exclude=refs/remotes/origin/HEAD --exclude=refs/remotes/origin/main"
	)
	.read()?;
	Ok(output
		.lines()
		.map(str::trim)
		.filter(|b| !b.is_empty())
		.map(String::from)
		.collect())
}
