//! # e05_1_quiz - Error Handling
//!
//! This exercise focuses on error handling. We will refactor the quiz library so that it's
//! functions return a proper `Result` instead of panicking.
//!
//! We will use the `thiserror` crate to create a single error type for all errors:
//!
//! - Add `thiserror` to the dependencies
//! - Create a single error type for all errors
//! - Make all functions return a `Result`
//! - Replace each call to `unwrap` with [`map_err`](https://doc.rust-lang.org/std/result/enum.Result.html#method.map_err)
//! - Test various error cases

pub mod e05_1_quiz;
