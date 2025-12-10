//! # e01_1_recursive_fibonacci - Recursive Fibonacci Sequence
//!
//! ## Overview
//! Implement the Fibonacci sequence recursively according to the following definitions:
//! - `fib(0) = 0`
//! - `fib(1) = 1`
//! - For `n > 1`, `fib(n) = fib(n-1) + fib(n-2)`
//!
//! ## Task
//! Create a recursive function `fibonacci` that computes the n-th number of the Fibonacci sequence.
//! You may use `u64` for both the function parameter and return type.
//!
//! # e01_2_closed_form_fibonacci - Closed-form Fibonacci Sequence
//!
//! ## Overview
//! The recursive Fibonacci function is inefficient for large `n` due to its exponential time complexity.
//! Use the closed-form (Binet's formula) to compute Fibonacci numbers more efficiently.
//!
//! ## Reference
//! The closed-form expression for the Fibonacci sequence, also known as Binet's formula, is defined as follows:
//!
//! $$
//!   F(n) = \mathrm{round}\left( \frac{\varphi^n - \psi^n}{\sqrt{5}} \right) \quad
//!   \text{où} \quad \varphi = \frac{1 + \sqrt{5}}{2} \quad et \quad \psi = \frac{1 - \sqrt{5}}{2}
//! $$
//!
//! ## Task
//! Implement a function `fib_closed_form` that calculates the n-th Fibonacci number using the closed-form expression.
//! Use rounding to the nearest integer and return the result as `u64`.
//!
//! ## Tools
//! - [`round`](https://doc.rust-lang.org/std/primitive.f64.html#method.round): Rounds the floating-point number to the nearest integer.
//! - [`sqrt`](https://doc.rust-lang.org/std/primitive.f64.html#method.sqrt): Calculates the square root of a number.
//! - [`powi`](https://doc.rust-lang.org/std/primitive.f64.html#method.powi): Raises a number to a power represented as an integer. Preferred over `powf` for maintaining precision with integers.
//! - [`as`](https://doc.rust-lang.org/rust-by-example/types/cast.html): Used to perform type casting between types, particularly from `f64` to `u64`.
//!
//! # Bonus: e01_3_imperative_fizzbuzz - FizzBuzz, Imperative Paradigm
//!
//! ## Overview
//! Implement the classic FizzBuzz problem for a single integer `n` using the imperative programming style.
//!
//! ## Task
//! Write a function `fizz_buzz_imperative` that takes an integer `n` and returns the FizzBuzz value for that specific number as a string.
//! - If `n` is a multiple of 3, return "Fizz".
//! - If `n` is a multiple of 5, return "Buzz".
//! - If `n` is a multiple of both 3 and 5, return "FizzBuzz".
//! - Otherwise, return the number as a string.
//!
//! <details>
//! <summary>Implementation Tip</summary>
//!
//! Use a `match` statement to check conditions. Construct a tuple `(n % 3 == 0, n % 5 == 0)` to test both divisions in one go.
//! ```ignore
//! match (n % 3 == 0, n % 5 == 0) {
//!     // Fill in the match arms here
//! }
//! ```
//!
//! </details>
//!
//! ## Tools
//! - [`match`](https://doc.rust-lang.org/book/ch06-02-match.html): Conditional structure to handle value patterns.
//! - [`to_string`](https://doc.rust-lang.org/std/string/trait.ToString.html): Converts a type to its string representation.

// pub mod e01_1_recursive_fibonacci;
// pub mod e01_2_closed_form_fibonacci;
// pub mod e01_3_imperative_fizzbuzz;
