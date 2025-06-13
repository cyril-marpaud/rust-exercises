//! # c_pyo3 - PyO3
//!
//! ## Call Rust from Python
//!
//! Full documentation available at:
//! - <https://pyo3.rs>
//! - <https://pyo3.rs/latest/rust-from-python>
//!
//! To ensure all the commands below function correctly, make sure you are in the `c_pyo3` folder
//! first:
//!
//! ```Bash
//! cd rust-exercises/c_pyo3
//! ```
//!
//! ### Make and activate a Python virtual environment
//!
//! ```Bash
//! mkdir rfp && cd "$_"
//!
//! python -m venv .venv
//! source .venv/bin/activate
//! ```
//!
//! ### Install maturin (formerly pyo3-pack)
//!
//! ```Bash
//! pip install maturin
//! ```
//!
//! ### Init a project & build the Python package
//!
//! ```Bash
//! maturin init --bindings pyo3
//! ```
//!
//! In `src/lib.rs`, observe how:
//! - a function is defined with `#[pyfunction]`
//! - a module is defined with `#[pymodule]`
//!   - `add_function()` is used to add functions to a module
//!
//! ```Rust
//! use pyo3::prelude::*;
//!
//! #[pymodule]
//! fn rfp(m: &Bound<'_, PyModule>) -> PyResult<()> {
//!     m.add_function(wrap_pyfunction!(sum_as_string, m)?)
//! }
//!
//! #[pyfunction]
//! fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
//!     Ok((a + b).to_string())
//! }
//! ```
//!
//! Now, compile the python package:
//!
//! ```Bash
//! maturin develop
//! ```
//!
//! - In `target/debug/`, observe `librfp.so`.
//! - Call `maturin develop` after each modification to the code.
//!
//! ### Call the package from Python
//!
//! ```Bash
//! python
//! ```
//!
//! ```Python
//! import rfp
//! rfp.sum_as_string(10,20)
//! ```
//!
//! ### Deactivate the virtual environment
//!
//! ```Bash
//! deactivate
//! ```
