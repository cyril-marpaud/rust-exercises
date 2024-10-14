//! # e01_1_recursive - Recursive Fibonacci Sequence
//!
//! ## Overview
//! Implement the Fibonacci sequence recursively according to the following definitions:
//! - `fib(0) = 0`
//! - `fib(1) = 1`
//! - For `n > 1`, `fib(n) = fib(n-1) + fib(n-2)`
//!
//! ## Task
//! Create a recursive function `fib` that computes the n-th number of the Fibonacci sequence.
//! Use `u64` for both the function parameter and return type.
//!
//! # e01_2_closed_form - Closed-form Fibonacci Sequence
//!
//! ## Overview
//! The recursive Fibonacci function is inefficient for large `n` due to its exponential time complexity.
//! Use the closed-form (Binet's formula) to compute Fibonacci numbers more efficiently.
//!
//! ## Reference
//! The closed-form expression for the Fibonacci sequence, also known as Binet's formula, is detailed here:
//! <https://en.wikipedia.org/wiki/Fibonacci_sequence#Closed-form_expression>
//!
//! It utilizes the formula: F(n) = round((phi^n - psi^n) / sqrt(5)), where phi = (1 + sqrt(5)) / 2 and psi = (1 - sqrt(5)) / 2.
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

// mod e01_1_recursive;
// mod e01_2_closed_form;
