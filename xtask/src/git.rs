use anyhow::Result;
use xshell::{Shell, cmd};

pub fn check_clean_state(sh: &Shell) -> Result<()> {
	if !cmd!(sh, "git status --porcelain").read()?.trim().is_empty() {
		anyhow::bail!("Working directory is not clean.");
	}
	Ok(())
}

pub fn checkout_branch(sh: &Shell, branch: &str) -> Result<()> {
	cmd!(sh, "git checkout -q {branch}").quiet().run()?;
	Ok(())
}

pub fn fetch(sh: &Shell) -> Result<()> {
	cmd!(sh, "git fetch -q").quiet().run()?;
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

pub fn list_files(sh: &Shell, rev: &str, dir: &str) -> Result<Vec<String>> {
	Ok(cmd!(sh, "git ls-tree -r --name-only {rev} -- {dir}")
		.read()?
		.lines()
		.map(String::from)
		.collect())
}

pub fn show(sh: &Shell, rev: &str, path: &str) -> Result<String> {
	let object = format!("{rev}:{path}");
	Ok(cmd!(sh, "git show {object}").read()?)
}
