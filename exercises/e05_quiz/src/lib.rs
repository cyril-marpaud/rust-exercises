//! # e05_1_quiz - Error Handling
//!
//! ## Overview
//! This exercise enhances the quiz application by incorporating structured error handling, ensuring that the application behaves predictably under various error conditions and avoids program panics.
//!
//! ## Task
//! You are tasked with refactoring the existing quiz application to use robust error handling practices. This involves using the `thiserror` crate to define a custom error type and modifying all functions to return `Result` instead of using panics.
//!
//! ## Instructions
//! - **Error Handling Setup:**
//!   - Add the [`thiserror`](https://docs.rs/thiserror) crate to handle custom errors.
//!
//!     <details>
//!     <summary>Add <code>thiserror</code> to your Cargo.toml</summary>
//!
//!     ```bash
//!     cargo add thiserror
//!     ```
//!     </details>
//!
//!   - Define a comprehensive error type using `thiserror` to encapsulate potential failure points.
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
//!     </details>
//!
//! - **Refactoring Functions:**
//!   - Update all functions to return `Result`.
//!   - Replace `unwrap()` with `map_err()` to convert various errors into your own type.
//!   - Use the propagation operator to unwrap or propagate those `Result`s.
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
//!     </details>
//!
//! - **Implement Robust Array Element Access:**
//!   - Replace direct array indexing with the `.get()` method to safely access elements.
//!   - Combine it with `ok_or` to convert the `Option` into a `Result`.
//!   - Propagate the error using the `?` operator as well.
//!
//! - **Implement Robust Operations:**
//!   - Replace each arithmetic operation with a safe method and propagate the errors as well.
//!
//! - **User Interaction Errors:**
//!   - Ensure that all user input functions handle errors gracefully, providing feedback and retry mechanisms where appropriate.
//!
//!     <details>
//!     <summary>Handling user input errors</summary>
//!
//!     ```ignore
//!     fn request_response(question: &Question) -> Result<usize, QuizError> {
//!         println!("{}", question.statement);
//!         for (index, choice) in question.choices.iter().enumerate() {
//!             println!("{}: {}", index + 1, choice);
//!         }
//!         let mut response = String::new();
//!         io::stdin().read_line(&mut response).map_err(QuizError::FileRead)?;
//!         response.trim().parse().map_err(|e| QuizError::ParseError(response, e))
//!     }
//!     ```
//!     </details>

// pub mod e05_1_quiz;
