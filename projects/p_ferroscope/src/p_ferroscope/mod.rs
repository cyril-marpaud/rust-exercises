pub mod implementors;

use std::{
	fmt::Display,
	path::{Path, PathBuf},
};

pub trait FerroScope {
	const CONTEXT: usize = 30;
	const EXTENSION_FILTER: fn(&str) -> bool = |_| true;

	fn read_one(&self, path: &Path) -> String;

	fn find(&self, _path: &Path) -> Vec<PathBuf> {
		vec![]
	}

	fn search(&self, _path: &Path, _pattern: &str) -> Vec<FerroMatch> {
		vec![]
	}
}

pub struct FerroMatch {
	pub path: PathBuf,
	pub occs: Vec<Occurence>,
}

pub struct Occurence {
	pub line: usize,
	pub col: usize,
	pub occ: String,
}

impl Display for FerroMatch {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		writeln!(f, "Match !")
	}
}
