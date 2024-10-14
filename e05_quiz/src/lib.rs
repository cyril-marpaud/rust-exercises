//! # e05_1_quiz - Error Handling
//!
//! ## Overview
//! This exercise enhances the quiz application by incorporating structured error handling. The aim is to ensure the application behaves predictably under various error conditions and avoids program panics.
//!
//! ## Step 1: Setting Up Error Handling
//! - Add the [`thiserror`](https://docs.rs/thiserror) crate to handle custom errors.
//!
//!   <details>
//!   <summary>How to add <code>thiserror</code> via Cargo</summary>
//!
//!   ```bash
//!   cargo add thiserror
//!   ```
//!   </details>
//!
//! - Define a comprehensive error type using `thiserror` to encapsulate potential failure points.
//!
//!   <details>
//!   <summary>Defining the <code>QuizError</code> type</summary>
//!
//!   ```ignore
//!   #[derive(Error, Debug)]
//!   pub enum QuizError {
//!       #[error("file open error: {0} ({1})")]
//!       FileOpen(String, #[source] io::Error),
//!       ...
//!   }
//!   ```
//!   </details>
//!
//! ## Step 2: Refactoring Functions to Return `Result`
//! - Update all functions to return `Result`, replacing `unwrap()` with proper error handling.
//!
//!   <details>
//!   <summary>Example of refactoring a function to return Result</summary>
//!
//!   ```ignore
//!   fn read_questions(p: &str) -> Result<Vec<Question>, QuizError> {
//!       let file = File::open(p).map_err(|e| QuizError::FileOpen(p.to_string(), e))?;
//!       ...
//!   }
//!   ```
//!   </details>
//!
//! ## Step 3: Implementing Error Handling in User Interactions
//! - Ensure that all user input functions handle errors gracefully, providing feedback and retry mechanisms where appropriate.
//!
//!   <details>
//!   <summary>Handling user input errors</summary>
//!
//!   ```ignore
//!   fn request_response(question: &Question) -> Result<usize, QuizError> {
//!       println!("{}", question.statement);
//!       for (index, choice) in question.choices.iter().enumerate() {
//!           println!("{}: {}", index + 1, choice);
//!       }
//!       let mut response = String::new();
//!       io::stdin().read_line(&mut response).map_err(QuizError::FileRead)?;
//!       response.trim().parse().map_err(|e| QuizError::ParseError(response, e))
//!   }
//!   ```
//!   </details>
//!
//! ## Bonus Step: Testing Error Handling
//! - Write tests to verify that the error handling works as expected, particularly that the application does not panic and provides meaningful error messages.
//!
//!   <details>
//!   <summary>Writing tests for error handling</summary>
//!
//!   ```ignore
//!   #[test]
//!   fn test_invalid_input() {
//!       // Example test to check handling of invalid user input
//!       let result = request_response(&invalid_question);
//!       assert!(result.is_err());
//!   }
//!   ```
//!   </details>

pub mod e05_1_quiz;
