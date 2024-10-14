//! # e02_1_imperative - FizzBuzz, Imperative Paradigm
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
//!
//! # e02_2_functional - FizzBuzz, Functional Paradigm
//!
//! ## Overview
//! Implement the FizzBuzz sequence for the first `n` numbers using the functional programming style, and display them.
//!
//! ## Task
//! Write an iterator `fizz_buzz_functional` that generates and displays the FizzBuzz sequence from 1 to `n`.
//! Each iteration should transform the number to "Fizz", "Buzz", "FizzBuzz", or the number itself, then print it.
//!
//! <details>
//! <summary>Sequence Generation Tip</summary>
//!
//! The sequence for generating "Fizz" every third value is created by using `repeat` to generate empty strings for placeholders, `take` to limit it to two iterations, and `once` to insert "Fizz" after every two empty strings. This sequence is then made infinite with `cycle`.
//! ```ignore
//! let f = repeat("").take(2).chain(once("Fizz")).cycle();
//! ```
//!
//! </details>
//!
//! ## Tools
//! - [`once`](https://doc.rust-lang.org/std/iter/fn.once.html): Creates an iterator that yields an item exactly once.
//! - [`repeat`](https://doc.rust-lang.org/std/iter/fn.repeat.html): Generates an infinite iterator that repeats a given value.
//! - [`cycle`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.cycle): Repeats an iterator endlessly.
//! - [`take`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.take): Creates an iterator that yields its first `n` elements.
//! - [`chain`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.chain): Combines two iterators into one sequence.
//! - [`zip`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.zip): Pairs up elements from two iterators.
//! - [`enumerate`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.enumerate): Transforms an iterator into an iterator of tuples (index, element).
//! - [`for_each`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.for_each): Calls a function on each element of an iterator.
//! - [`to_string`](https://doc.rust-lang.org/std/string/trait.ToString.html): Converts a type to its string representation.
//! - [`format!`](https://doc.rust-lang.org/std/macro.format.html): Writes formatted text to a string.
//! - [`println!`](https://doc.rust-lang.org/std/macro.println.html): Prints a string to the console, with a newline.

// mod e02_1_imperative;
pub mod e02_2_functional;
