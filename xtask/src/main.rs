use anyhow::{Context, Result};
use std::{fs::read_to_string, path::PathBuf};
use time::{OffsetDateTime, format_description};
use toml::Value;
use xshell::Shell;

fn main() -> Result<()> {
	let sh = Shell::new()?;
	let crates = get_default_members()?;

	const JUNIT_SRC: &str = "target/nextest/default/junit.xml";
	let junit_src = PathBuf::from(JUNIT_SRC);
	let mut junit_dst = PathBuf::from(JUNIT_SRC);

	for crate_name in &crates {
		junit_dst.set_file_name(format!("{crate_name}.xml"));

		let status = sh
			.cmd("cargo")
			.arg("nextest")
			.arg("run")
			.arg("--package")
			.arg(crate_name)
			.run();

		if status.is_ok() && PathBuf::from(&junit_src).exists() {
			std::fs::rename(&junit_src, &junit_dst)?;
		} else {
			let uuid = uuid::Uuid::new_v4();
			let timestamp = now_iso8601()?;
			let xml = format!(
				r#"<?xml version="1.0" encoding="UTF-8"?>
<testsuites name="nextest-run" tests="1" failures="1" errors="0" uuid="{uuid}" timestamp="{timestamp}" time="0.0">
    <testsuite name="{crate_name}" tests="1" disabled="0" errors="0" failures="1">
        <testcase name="build_and_test" classname="{crate_name}" timestamp="{timestamp}" time="0.0">
            <failure message="Build or test failed for {crate_name}" />
        </testcase>
    </testsuite>
</testsuites>"#,
			);
			std::fs::write(&junit_dst, xml)?;
		}
	}

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
		.filter_map(|item| item.as_str().map(str::to_string))
		.collect();

	Ok(default_members)
}

fn now_iso8601() -> Result<String> {
	let now = OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());
	let fmt = format_description::parse(
        "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:3][offset_hour sign:mandatory]:[offset_minute]"
    ).unwrap();
	now.format(&fmt).context("Time formatting error")
}
