use anyhow::Result;
use chrono::{DateTime, Local};
use std::{
	fs::read_to_string,
	io::{self, Write},
	path::{Path, PathBuf},
	process::{Command, Stdio},
	time::SystemTime,
};
use toml::Value;
use uuid::Uuid;
use xshell::{Shell, cmd};

use crate::git;

const JUNIT_SRC: &str = "target/nextest/default/junit.xml";

pub fn reports() -> Result<()> {
	let sh = Shell::new()?;

	print!("Checking dependencies... ");
	io::stdout().flush().ok();
	check_dependencies(&sh)?;
	println!("ok");

	print!("Checking working directory... ");
	io::stdout().flush().ok();
	git::check_clean_state(&sh)?;
	println!("ok");

	print!("Fetching remote branches... ");
	io::stdout().flush().ok();
	git::fetch(&sh)?;
	println!("ok");

	let original_branch = git::get_current_branch(&sh)?;
	let branches = git::get_learner_branches(&sh)?;
	let crates = get_default_members()?;

	let total = branches.len();
	if total == 0 {
		println!("No learner branches found.");
		return Ok(());
	}
	println!("Found {total} learner branch(es): {}", branches.join(", "));

	branches.iter().enumerate().for_each(|(i, branch)| {
		println!("\n[{}/{total}] {branch}", i + 1);
		if let Err(e) = make_branch_reports(&sh, branch, &crates) {
			eprintln!("\tError: {e}");
		}
	});

	check_reports(&branches, &crates);

	print!("\nCreating reports/reports.zip... ");
	io::stdout().flush().ok();
	if let Err(e) = create_zip(&sh) {
		println!("failed");
		eprintln!("\t{e}");
	} else {
		println!("ok");
	}

	git::checkout_branch(&sh, &original_branch).map_err(|e| {
		anyhow::anyhow!(
			"Failed to return to branch '{original_branch}' — run `git checkout {original_branch}` manually: {e}"
		)
	})
}

fn check_dependencies(sh: &Shell) -> Result<()> {
	cmd!(sh, "git --version")
		.read()
		.map_err(|_| anyhow::anyhow!("'git' is not installed — run: sudo apt install git"))?;
	cmd!(sh, "cargo nextest --version").read().map_err(|_| {
		anyhow::anyhow!("'cargo-nextest' is not installed — run: cargo install cargo-nextest")
	})?;
	cmd!(sh, "zip -v")
		.read()
		.map_err(|_| anyhow::anyhow!("'zip' is not installed — run: sudo apt install zip"))?;
	Ok(())
}

fn get_default_members() -> Result<Vec<String>> {
	let workspace_toml = read_to_string("Cargo.toml")?;

	let default_members = toml::from_str::<Value>(&workspace_toml)?
		.get("workspace")
		.ok_or(anyhow::anyhow!("[workspace] parsing error"))?
		.get("default-members")
		.and_then(Value::as_array)
		.ok_or(anyhow::anyhow!("[default-members] parsing error"))?
		.iter()
		.filter_map(|item| {
			item.as_str()
				.map(|c| c.rsplit_once('/').map(|(_, c)| c).unwrap_or(c).to_string())
		})
		.collect();

	Ok(default_members)
}

fn make_branch_reports(sh: &Shell, branch: &str, crates: &[String]) -> Result<()> {
	git::checkout_branch(sh, branch)?;
	let report_dir = PathBuf::from("reports").join(branch);
	std::fs::create_dir_all(&report_dir)?;
	crates.iter().for_each(|crate_name| {
		if let Err(e) = make_crate_report(crate_name, &report_dir) {
			eprintln!("\tError: {e}");
		}
	});
	Ok(())
}

fn make_crate_report(crate_name: &str, report_dir: &Path) -> Result<()> {
	print!("\t{crate_name}... ");
	io::stdout().flush().ok();

	let junit_src = PathBuf::from(JUNIT_SRC);
	let junit_dst = report_dir.join(format!("{crate_name}.xml"));

	let status = Command::new("cargo")
		.args(["nextest", "run", "--no-fail-fast", "--package", crate_name])
		.stdout(Stdio::null())
		.stderr(Stdio::null())
		.status()
		.map_err(|e| anyhow::anyhow!("Failed to run cargo nextest for '{crate_name}': {e}"))?;

	if junit_src.exists() {
		std::fs::rename(&junit_src, &junit_dst)?;
		let msg = parse_test_counts(&junit_dst)
			.map(|(total, failed)| {
				let passed = total - failed;
				if failed == 0 {
					format!("ok ({passed}/{total})")
				} else {
					format!("tests failed ({passed}/{total})")
				}
			})
			.unwrap_or_else(|| {
				if status.success() {
					"ok".into()
				} else {
					"tests failed".into()
				}
			});
		println!("{msg}");
		return Ok(());
	}

	println!("build failed");
	std::fs::write(&junit_dst, make_failure_report(crate_name))?;
	Ok(())
}

fn parse_test_counts(path: &Path) -> Option<(u32, u32)> {
	let xml = std::fs::read_to_string(path).ok()?;
	let doc = roxmltree::Document::parse(&xml).ok()?;
	let root = doc.root_element();
	let total: u32 = root.attribute("tests")?.parse().ok()?;
	let failed: u32 = root.attribute("failures")?.parse().ok()?;
	Some((total, failed))
}

fn make_failure_report(crate_name: &str) -> String {
	let uuid = Uuid::new_v4();
	let timestamp = DateTime::<Local>::from(SystemTime::now())
		.to_rfc3339_opts(chrono::SecondsFormat::Millis, false);
	format!(
		r#"<?xml version="1.0" encoding="UTF-8"?>
<testsuites name="nextest-run" tests="1" failures="0" errors="1" uuid="{uuid}" timestamp="{timestamp}" time="0.0">
    <testsuite name="{crate_name}" tests="1" disabled="0" errors="1" failures="0">
        <testcase name="build_and_test" classname="{crate_name}" timestamp="{timestamp}" time="0.0">
            <error message="Build failed for {crate_name}" />
        </testcase>
    </testsuite>
</testsuites>"#
	)
}

fn check_reports(branches: &[String], crates: &[String]) {
	println!("\nVerifying reports...");
	let missing = branches
		.iter()
		.flat_map(|branch| {
			let dir = PathBuf::from("reports").join(branch);
			if !dir.exists() {
				vec![format!("Missing folder: {branch}")]
			} else {
				crates
					.iter()
					.filter(|crate_name| !dir.join(format!("{crate_name}.xml")).exists())
					.map(|crate_name| format!("Missing file: {branch}/{crate_name}.xml"))
					.collect::<Vec<_>>()
			}
		})
		.collect::<Vec<_>>();
	if missing.is_empty() {
		println!("\tAll reports present.");
	} else {
		missing.iter().for_each(|msg| eprintln!("\t{msg}"));
	}
}

fn create_zip(sh: &Shell) -> Result<()> {
	cmd!(sh, "zip -q -r reports.zip reports/").run()?;
	std::fs::rename("reports.zip", "reports/reports.zip")?;
	Ok(())
}
