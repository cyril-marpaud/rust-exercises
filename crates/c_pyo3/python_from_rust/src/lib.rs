//! # c_pyo3 - PyO3
//!
//! ## Call Python from Rust
//!
//! Full documentation available at:
//! - <https://pyo3.rs>
//! - <https://pyo3.rs/latest/python-from-rust>
//!
//! ### Create the cargo project
//!
//! One can either use this repo's `python_from_rust` folder or create a new project:
//!
//! ```Bash
//! cargo new python_from_rust && cd "$_"
//! ```
//!
//! ### Add PyO3 as dependency
//!
//! ```Bash
//! cargo add pyo3 --features auto-initialize
//! ```
//!
//! ### Add source code in main.rs
//!
//! If your project is a library, create `src/bin/main.rs`, else just use `src/main.rs`.
//!
//! The [Getting Started example](https://pyo3.rs/v0.22.3/#using-python-from-rust) provides a good
//! starting point for `src/main.rs`:
//!
//! ```Rust
//! use pyo3::{prelude::*, types::IntoPyDict};
//!
//! fn main() -> PyResult<()> {
//!   Python::with_gil(|py| {
//!     let sys = py.import_bound("sys")?;
//!     let version: String = sys.getattr("version")?.extract()?;
//!
//!     let locals = [("os", py.import_bound("os")?)].into_py_dict_bound(py);
//!     let code = "os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'";
//!     let user: String = py.eval_bound(code, None, Some(&locals))?.extract()?;
//!
//!     println!("Hello {}, I'm Python {}", user, version);
//!     Ok(())
//!   })
//! }
//! ```
//!
//! ### Run the project
//!
//! ```Bash
//! cargo run
//! ```
//!
//! ### Go further
//!
//! See
//! <https://pyo3.rs/v0.22.3/python-from-rust/calling-existing-code#executing-existing-python-code>
//! to:
//! - import modules
//! - evaluate one or several expression
//! - run one or several files
//! - use context managers
