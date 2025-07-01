use anyhow::Result;
use chrono::{DateTime, Local};
use std::{fs::read_to_string, path::PathBuf, time::SystemTime};
use toml::Value;
use uuid::Uuid;
use xshell::Shell;

fn main() -> Result<()> {
	let sh = Shell::new()?;
	let crates = get_default_members()?;

	const JUNIT_SRC: &str = "target/nextest/default/junit.xml";
	let junit_src = PathBuf::from(JUNIT_SRC);
	let mut junit_dst = PathBuf::from(JUNIT_SRC);

	crates.iter().for_each(|crate_name| {
		junit_dst.set_file_name(format!("{crate_name}.xml"));

		let test_cmd = sh
			.cmd("cargo")
			.arg("nextest")
			.arg("run")
			.arg("--no-fail-fast")
			.arg("--package")
			.arg(crate_name)
			.run();

		let junit_report = if test_cmd.is_ok() && PathBuf::from(&junit_src).exists() {
			std::fs::rename(&junit_src, &junit_dst)
		} else {
			let uuid = Uuid::new_v4();
			let timestamp = DateTime::<Local>::from(SystemTime::now())
				.to_rfc3339_opts(chrono::SecondsFormat::Millis, false);
			let xml = format!(
r#"<?xml version="1.0" encoding="UTF-8"?>
<testsuites name="nextest-run" tests="1" failures="1" errors="0" uuid="{uuid}" timestamp="{timestamp}" time="0.0">
    <testsuite name="{crate_name}" tests="1" disabled="0" errors="0" failures="1">
        <testcase name="build_and_test" classname="{crate_name}" timestamp="{timestamp}" time="0.0">
            <failure message="Build or test failed for {crate_name}" />
        </testcase>
    </testsuite>
</testsuites>"#
			);
			std::fs::write(&junit_dst, xml)
		};

		if let Err(e) = junit_report {
			println!("An error occured: {e}");
		}
	});

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
