//! # e04_1_quiz - Error Handling
//!
//! ## Overview
//! This exercise enhances the quiz application by incorporating structured error handling, ensuring that the application behaves predictably under various error conditions and avoids program panics.
//!
//! ## Step 0: Try the Quiz Application
//! - Create a binary entry point at `src/bin/main.rs`.
//! - Inside it, call the `play()` function from the quiz module.
//! - Run the program and experiment:
//!   - Provide valid answers.
//!   - Try triggering errors: give unexpected input, interrupt the quiz, or simulate file access errors.
//!
//! This will help you understand how the quiz works and identify where panic-worthy behavior must be replaced by proper error handling.
//!
//! ## Error Handling Setup
//! - Add the [`thiserror`](https://docs.rs/thiserror) crate to handle custom errors.
//!
//!     <details>
//!     <summary>Add <code>thiserror</code> to your Cargo.toml</summary>
//!
//!     ```bash
//!     cargo add thiserror
//!     ```
//!
//!     </details>
//!
//! - Define a comprehensive error type using `thiserror` to encapsulate potential failure points.
//!
//!     <details>
//!     <summary>Defining the <code>QuizError</code> type</summary>
//!
//!     ```ignore
//!     #[derive(Error, Debug)]
//!     pub enum QuizError {
//!         #[error("file open error: {0} ({1})")]
//!         FileOpen(String, #[source] io::Error),
//!         ...
//!     }
//!     ```
//!
//!     </details>
//!
//! ## Refactoring Functions
//! - Update all functions to return `Result`.
//! - Replace `unwrap()` with `map_err()` to convert various errors into your own type.
//! - Use the propagation operator to unwrap or propagate those `Result`s.
//!
//!     <details>
//!     <summary>Example of refactoring a function to return Result</summary>
//!
//!     ```ignore
//!     fn read_questions(p: &str) -> Result<Vec<Question>, QuizError> {
//!         let file = File::open(p).map_err(|e| QuizError::FileOpen(p.to_string(), e))?;
//!         ...
//!     }
//!     ```
//!
//!     </details>
//!
//! ## Bonus: Implement Robust Array Element Access
//! - Replace direct array indexing with the `.get()` method to safely access elements.
//! - Combine it with `ok_or` to convert the `Option` into a `Result`.
//! - Propagate the error using the `?` operator as well.
//!
//! ## Bonus: Implement Robust Operations
//! - Replace each arithmetic operation with a safe method and propagate the errors as well.

// pub mod e04_1_quiz;
