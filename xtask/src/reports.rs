use std::{
	collections::HashMap,
	fmt::{Display, Formatter},
	fs::write,
	io::{Write, stdout},
};

use anyhow::Result;
use toml::Value;
use xshell::{Shell, cmd};

use crate::git;

const REFERENCE: &str = "origin/main";
const REPORT: &str = "reports.txt";
const TITLE_WIDTH: usize = 76;

/// A package of the course, with the tests it holds on the reference branch.
struct Package {
	name: String,
	tests: Vec<Test>,
}

/// A reference test, keyed `module::fn` so that it is still found after being moved within its module.
#[derive(Clone)]
struct Test {
	bonus: bool,
	key: String,
}

impl Test {
	fn new(module: &str, name: &str) -> Test {
		Test {
			bonus: name.starts_with("bonus_"),
			key: format!("{module}::{name}"),
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Outcome {
	BuildFailed,
	Failed,
	Missing,
	Passed,
}

impl Display for Outcome {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.write_str(match self {
			Outcome::BuildFailed => "build failed",
			Outcome::Failed => "failed",
			Outcome::Missing => "missing",
			Outcome::Passed => "passed",
		})
	}
}

/// The outcome of every reference test of one crate on one learner branch.
struct PackageReport {
	name: String,
	results: Vec<(Test, Outcome)>,
}

impl PackageReport {
	/// `run` is `None` when the crate does not build. Tests absent from `run` are missing, and tests absent from the reference are ignored.
	fn new(package: &Package, run: Option<&HashMap<String, Outcome>>) -> PackageReport {
		let results = package
			.tests
			.iter()
			.map(|test| {
				let outcome = run.map_or(Outcome::BuildFailed, |run| {
					run.get(&test.key).copied().unwrap_or(Outcome::Missing)
				});
				(test.clone(), outcome)
			})
			.collect();
		PackageReport {
			name: package.name.clone(),
			results,
		}
	}

	fn built(&self) -> bool {
		!self
			.results
			.iter()
			.any(|(_, outcome)| *outcome == Outcome::BuildFailed)
	}

	/// The `(test, result)` rows of the Failures section. A bonus left untouched is not a failure.
	fn failures(&self) -> Vec<(&str, Outcome)> {
		if !self.built() {
			return vec![("all tests", Outcome::BuildFailed)];
		}
		self.results
			.iter()
			.filter(|(test, outcome)| match outcome {
				Outcome::Failed => true,
				Outcome::Missing => !test.bonus,
				Outcome::BuildFailed | Outcome::Passed => false,
			})
			.map(|(test, outcome)| (test.key.as_str(), *outcome))
			.collect()
	}

	/// `(passed, total)` over the bonus tests, or over the required ones.
	fn score(&self, bonus: bool) -> (usize, usize) {
		self.results
			.iter()
			.filter(|(test, _)| test.bonus == bonus)
			.fold((0, 0), |(passed, total), (_, outcome)| {
				(passed + usize::from(*outcome == Outcome::Passed), total + 1)
			})
	}
}

enum Row {
	Cells(Vec<String>),
	Rule,
}

pub fn reports() -> Result<()> {
	let sh = Shell::new()?;

	step("Checking working directory", || git::check_clean_state(&sh))?;
	step("Checking current branch", || {
		let current = git::get_current_branch(&sh)?;
		if current != "main" {
			anyhow::bail!(
				"on '{current}', reports must run from main — run: git checkout main && cargo reports"
			);
		}
		Ok(())
	})?;
	step("Fetching remote branches", || git::fetch(&sh))?;

	let branches = git::get_learner_branches(&sh)?;
	let total = branches.len();
	println!("Found {total} learner branch(es): {}", branches.join(", "));

	let packages = step("Reading reference tests", || load_packages(&sh))?;

	let blocks = branches
		.iter()
		.enumerate()
		.map(|(i, branch)| {
			println!("\n[{}/{total}] {branch}", i + 1);
			learner_report(&sh, branch, &packages).unwrap_or_else(|e| {
				eprintln!("\tError: {e}");
				format!("{}\n\n  Error: {e}\n", title(branch))
			})
		})
		.collect::<Vec<_>>();

	git::checkout_branch(&sh, "main").map_err(|e| {
		anyhow::anyhow!("Failed to return to main — run `git checkout main` manually: {e}")
	})?;

	write(REPORT, blocks.join("\n\n"))?;
	println!("\nReport written to {REPORT}");
	Ok(())
}

fn announce(step: &str) {
	print!("{step}... ");
	stdout().flush().ok();
}

fn fn_name(line: &str) -> Option<&str> {
	line.strip_prefix("fn ")?
		.split_once('(')
		.map(|(name, _)| name.trim())
}

fn learner_report(sh: &Shell, branch: &str, packages: &[Package]) -> Result<String> {
	git::checkout_branch(sh, branch)?;
	let reports = packages
		.iter()
		.map(|package| {
			announce(&format!("\t{}", package.name));
			let run = run_tests(sh, &package.name)?;
			let report = PackageReport::new(package, run.as_ref());
			if report.built() {
				let ((passed, required), (bonus_passed, bonus)) =
					(report.score(false), report.score(true));
				println!("{passed}/{required}, bonus {bonus_passed}/{bonus}");
			} else {
				println!("build failed");
			}
			Ok(report)
		})
		.collect::<Result<Vec<_>>>()?;
	Ok(render_learner(branch, &reports))
}

fn load_package(sh: &Shell, path: &str) -> Result<Package> {
	let src = format!("{path}/src");
	let mut tests = Vec::new();
	for file in git::list_files(sh, REFERENCE, &src)? {
		if let Some(module) = module_of(&file, &src).filter(|_| file.ends_with(".rs")) {
			let source = git::show(sh, REFERENCE, &file)?;
			tests.extend(
				test_names(&source)
					.into_iter()
					.map(|name| Test::new(module, name)),
			);
		}
	}
	Ok(Package {
		name: path.rsplit('/').next().unwrap_or(path).to_string(),
		tests,
	})
}

/// The packages of `default-members` on the reference branch, in their declared order.
fn load_packages(sh: &Shell) -> Result<Vec<Package>> {
	toml::from_str::<Value>(&git::show(sh, REFERENCE, "Cargo.toml")?)?
		.get("workspace")
		.and_then(|workspace| workspace.get("default-members"))
		.and_then(Value::as_array)
		.ok_or(anyhow::anyhow!(
			"[workspace] default-members is missing from {REFERENCE}:Cargo.toml"
		))?
		.iter()
		.map(|member| {
			let path = member.as_str().ok_or(anyhow::anyhow!(
				"default-members entry {member} is not a path"
			))?;
			load_package(sh, path)
		})
		.collect()
}

/// The top-level module a source file belongs to, which is the first segment of its tests' paths.
fn module_of<'a>(file: &'a str, src: &str) -> Option<&'a str> {
	let first = file
		.strip_prefix(src)?
		.strip_prefix('/')?
		.split('/')
		.next()?;
	Some(first.strip_suffix(".rs").unwrap_or(first))
}

/// Outcomes of a `cargo test` run keyed like [`Test`], or `None` when nothing ran because the crate does not build.
fn parse_libtest(stdout: &str) -> Option<HashMap<String, Outcome>> {
	stdout
		.lines()
		.any(|line| line.starts_with("running "))
		.then(|| {
			stdout
				.lines()
				.filter_map(|line| {
					let (name, result) = line.strip_prefix("test ")?.rsplit_once(" ... ")?;
					let outcome = match result {
						"ok" => Outcome::Passed,
						"FAILED" => Outcome::Failed,
						_ => return None,
					};
					// Drops suffixes such as ` - should panic`.
					let path = name.split(' ').next()?;
					let (module, _) = path.split_once("::")?;
					let (_, function) = path.rsplit_once("::")?;
					Some((format!("{module}::{function}"), outcome))
				})
				.collect()
		})
}

fn render_learner(branch: &str, reports: &[PackageReport]) -> String {
	if reports
		.iter()
		.all(|report| report.score(false).0 + report.score(true).0 == 0)
	{
		return format!(
			"{}\n\n  No test passed: nothing to assess.\n",
			title(branch)
		);
	}
	let score = |(passed, total)| match total {
		0 => "-".to_string(),
		_ => format!("{passed}/{total}"),
	};
	let total = |bonus| {
		reports
			.iter()
			.map(|report| report.score(bonus))
			.fold((0, 0), |(passed, total), (p, t)| (passed + p, total + t))
	};
	let header = |cells: [&str; 3]| Row::Cells(cells.map(String::from).to_vec());

	let summary = [header(["Crate", "Required", "Bonus"]), Row::Rule]
		.into_iter()
		.chain(reports.iter().map(|report| {
			Row::Cells(vec![
				report.name.clone(),
				score(report.score(false)),
				score(report.score(true)),
			])
		}))
		.chain([
			Row::Rule,
			Row::Cells(vec![
				"Total".into(),
				score(total(false)),
				score(total(true)),
			]),
		])
		.collect::<Vec<_>>();

	let failures = reports
		.iter()
		.flat_map(|report| {
			report.failures().into_iter().map(|(test, result)| {
				Row::Cells(vec![report.name.clone(), test.into(), result.to_string()])
			})
		})
		.collect::<Vec<_>>();
	let failures = if failures.is_empty() {
		"    None.\n".to_string()
	} else {
		table(
			&[header(["Crate", "Test", "Result"]), Row::Rule]
				.into_iter()
				.chain(failures)
				.collect::<Vec<_>>(),
			&[false, false, false],
		)
	};

	format!(
		"{}\n\n  Summary\n\n{}\n  Failures\n\n{failures}",
		title(branch),
		table(&summary, &[false, true, true])
	)
}

/// Runs the unit tests only: doctests are examples, not exercises.
fn run_tests(sh: &Shell, name: &str) -> Result<Option<HashMap<String, Outcome>>> {
	let output = cmd!(sh, "cargo test --package {name} --lib --no-fail-fast")
		.ignore_status()
		.quiet()
		.output()?;
	Ok(parse_libtest(&String::from_utf8_lossy(&output.stdout)))
}

fn step<T>(name: &str, run: impl FnOnce() -> Result<T>) -> Result<T> {
	announce(name);
	let value = run()?;
	println!("ok");
	Ok(value)
}

fn table(rows: &[Row], right_aligned: &[bool]) -> String {
	let widths = (0..right_aligned.len())
		.map(|i| {
			rows.iter()
				.filter_map(|row| match row {
					Row::Cells(cells) => cells.get(i).map(String::len),
					Row::Rule => None,
				})
				.max()
				.unwrap_or(0)
		})
		.collect::<Vec<_>>();
	rows.iter()
		.map(|row| {
			let cells = match row {
				Row::Cells(cells) => cells
					.iter()
					.zip(&widths)
					.zip(right_aligned)
					.map(|((cell, &width), &right)| {
						if right {
							format!("{cell:>width$}")
						} else {
							format!("{cell:<width$}")
						}
					})
					.collect::<Vec<_>>(),
				Row::Rule => widths.iter().map(|&width| "-".repeat(width)).collect(),
			};
			format!("    {}\n", cells.join("   ").trim_end())
		})
		.collect()
}

/// Test function names in a source file, commented-out ones included: on the reference branch, a bonus test may wait commented out for the learner to enable it.
fn test_names(source: &str) -> Vec<&str> {
	let mut lines = source.lines().map(uncomment);
	let mut names = Vec::new();
	while let Some(line) = lines.next() {
		if line == "#[test]"
			&& let Some(name) = lines.find(|line| !line.starts_with("#[")).and_then(fn_name)
		{
			names.push(name);
		}
	}
	names
}

fn title(branch: &str) -> String {
	let head = format!("== {branch} ");
	let fill = "=".repeat(TITLE_WIDTH.saturating_sub(head.len()));
	format!("{head}{fill}")
}

/// Strips a leading `//`, so that a test commented out on the reference branch still counts.
fn uncomment(line: &str) -> &str {
	let line = line.trim();
	line.strip_prefix("//").map_or(line, str::trim_start)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn package_report() {
		let package = Package {
			name: "e00".into(),
			tests: ["a", "b", "bonus_c", "bonus_d"]
				.map(|name| Test::new("m", name))
				.into(),
		};
		let cases = [
			// Build failed: one row, every test lost.
			(
				None,
				(0, 2),
				(0, 2),
				vec![("all tests", Outcome::BuildFailed)],
			),
			// Nothing ran: required tests are missing, bonus ones neutral.
			(
				Some(vec![]),
				(0, 2),
				(0, 2),
				vec![("m::a", Outcome::Missing), ("m::b", Outcome::Missing)],
			),
			// A failed bonus is listed, an untouched one is not.
			(
				Some(vec![
					("m::a", Outcome::Passed),
					("m::b", Outcome::Failed),
					("m::bonus_c", Outcome::Failed),
				]),
				(1, 2),
				(0, 2),
				vec![("m::b", Outcome::Failed), ("m::bonus_c", Outcome::Failed)],
			),
			// A test outside the reference is ignored.
			(
				Some(vec![
					("m::a", Outcome::Passed),
					("m::b", Outcome::Passed),
					("m::extra", Outcome::Failed),
				]),
				(2, 2),
				(0, 2),
				vec![],
			),
			// Everything passed, bonus included.
			(
				Some(vec![
					("m::a", Outcome::Passed),
					("m::b", Outcome::Passed),
					("m::bonus_c", Outcome::Passed),
					("m::bonus_d", Outcome::Passed),
				]),
				(2, 2),
				(2, 2),
				vec![],
			),
		];
		cases
			.into_iter()
			.for_each(|(run, required, bonus, failures)| {
				let run = run.map(|run| {
					run.into_iter()
						.map(|(key, outcome)| (key.to_string(), outcome))
						.collect::<HashMap<_, _>>()
				});
				let report = PackageReport::new(&package, run.as_ref());
				assert_eq!(report.score(false), required, "{run:?}");
				assert_eq!(report.score(true), bonus, "{run:?}");
				assert_eq!(report.failures(), failures, "{run:?}");
			});
	}

	#[test]
	fn module_of() {
		let cases = [
			("e00/src/e00_1_x/mod.rs", Some("e00_1_x")),
			("e00/src/e00_1_x/nested/file.rs", Some("e00_1_x")),
			("e00/src/e00_2_y.rs", Some("e00_2_y")),
			("e00/src/lib.rs", Some("lib")),
			("e00/srcs/lib.rs", None),
			("e01/src/lib.rs", None),
		];
		cases.into_iter().for_each(|(file, expected)| {
			assert_eq!(super::module_of(file, "e00/src"), expected, "{file}");
		});
	}

	#[test]
	fn parse_libtest() {
		let cases = [
			// No `running` line: the crate did not build.
			("", None),
			("error[E0425]: cannot find value `x`\n", None),
			// Built, no module enabled.
			("running 0 tests\n", Some(vec![])),
			(
				"running 2 tests\ntest m::tests::a ... ok\ntest m::tests::b ... FAILED\n",
				Some(vec![("m::a", Outcome::Passed), ("m::b", Outcome::Failed)]),
			),
			// Moved deeper within its module.
			(
				"running 1 test\ntest m::inner::tests::a ... ok\n",
				Some(vec![("m::a", Outcome::Passed)]),
			),
			(
				"running 1 test\ntest m::tests::a - should panic ... ok\n",
				Some(vec![("m::a", Outcome::Passed)]),
			),
			// Ignored tests and the summary line are not outcomes.
			(
				"running 1 test\ntest m::tests::a ... ignored\n\ntest result: ok. 0 passed; 0 failed; 1 \
				 ignored\n",
				Some(vec![]),
			),
		];
		cases.into_iter().for_each(|(stdout, expected)| {
			let expected = expected.map(|expected| {
				expected
					.into_iter()
					.map(|(key, outcome)| (key.to_string(), outcome))
					.collect::<HashMap<_, _>>()
			});
			assert_eq!(super::parse_libtest(stdout), expected, "{stdout}");
		});
	}

	#[test]
	fn test_names() {
		let cases = [
			("#[test]\nfn a() {\n}\n", vec!["a"]),
			("\t#[test]\n\tfn a() {\n", vec!["a"]),
			// Commented out on the reference branch, waiting for the learner.
			("// #[test]\n// fn bonus_a() {\n", vec!["bonus_a"]),
			("\t// #[test]\n\t// fn bonus_a() {\n", vec!["bonus_a"]),
			("#[test]\n#[should_panic]\nfn a() {\n", vec!["a"]),
			("#[test]\nfn a() {}\n\n#[test]\nfn b() {}\n", vec!["a", "b"]),
			("fn helper() {\n", vec![]),
			// Doc examples are not tests.
			("//! #[test]\n//! fn a() {\n", vec![]),
			("#[test]\n", vec![]),
		];
		cases.into_iter().for_each(|(source, expected)| {
			assert_eq!(super::test_names(source), expected, "{source}");
		});
	}
}
