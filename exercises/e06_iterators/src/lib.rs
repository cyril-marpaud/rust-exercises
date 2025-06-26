//! # e06_1_iterators - Iterators
//!
//! ## Overview
//! This exercise involves implementing the `Iterator` trait for the `Fibonacci` struct. You will learn how iterators work in Rust and explore different methods provided by the `Iterator` trait.
//!
//! ## Task
//! The `Fibonacci` struct is initially empty, and your task is to transform it into a functional iterator that can generate Fibonacci numbers:
//! - Create a struct, `FibIter`, to hold the iteration state. You have at least two choices for managing the state:
//!   - **Index-Based:** Store only the current index of the sequence and calculate each Fibonacci number using the closed-form expression known as Binet's formula.
//!   - **Last Two Elements:** Store the last two numbers of the sequence to compute the next number, which is more efficient than recalculating each number from scratch.
//! - Implement the necessary traits:
//!   - [`IntoIterator`](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html) for `Fibonacci` to return an instance of `FibIter`.
//!   - [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) for `FibIter` that utilizes the chosen state management strategy to generate the sequence.
//!
//! ## Instructions
//! 1. **Choose a State Management Strategy:** Decide whether to store the index or the last two Fibonacci numbers in `FibIter`.
//! 2. **Implement IntoIterator for Fibonacci:** This trait should initialize `FibIter` with the appropriate initial state.
//! 3. **Implement Iterator for FibIter:** Depending on your chosen strategy, implement the `next` method to calculate the next Fibonacci number and update the state accordingly.
//!
//! ## Tips
//! - **Index-Based Approach:** While simpler, this approach may involve more computation for higher indices due to repeated application of Binet's formula.
//! - **Last Two Elements Approach:** This method minimizes computation by maintaining a rolling state of the last two numbers, allowing for quick calculation of the next number in the sequence.
//! - Use Rust Analyzer to facilitate code writing, especially when implementing traits. It can suggest quick fixes and auto-generate boilerplate code for missing trait methods.
//!
//! # e06_2_functional_fizzbuzz - FizzBuzz, Functional Paradigm
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
//! The sequence for generating "Fizz" every third value is created by using `repeat_n` to generate empty strings for placeholders, `take` to limit it to two iterations, and `once` to insert "Fizz" after every two empty strings. This sequence is then made infinite with `cycle`.
//! ```ignore
//! let f = repeat_n("", 2).chain(once("Fizz")).cycle();
//! ```
//!
//! </details>
//!
//! ## Tools
//! - [`once`](https://doc.rust-lang.org/std/iter/fn.once.html): Creates an iterator that yields an item exactly once.
//! - [`repeat_n`](https://doc.rust-lang.org/std/iter/fn.repeat_n.html): Generates an iterator that repeats a given value `n` times.
//! - [`cycle`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.cycle): Repeats an iterator endlessly.
//! - [`take`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.take): Creates an iterator that yields its first `n` elements.
//! - [`chain`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.chain): Combines two iterators into one sequence.
//! - [`zip`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.zip): Pairs up elements from two iterators.
//! - [`enumerate`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.enumerate): Transforms an iterator into an iterator of tuples (index, element).
//! - [`map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map): Creates an iterator which calls a closure on each element.
//! - [`to_string`](https://doc.rust-lang.org/std/string/trait.ToString.html): Converts a type to its string representation.
//! - [`format!`](https://doc.rust-lang.org/std/macro.format.html): Writes formatted text to a string.
//! - [`println!`](https://doc.rust-lang.org/std/macro.println.html): Prints a string to the console, with a newline.

// pub mod e06_1_iterators;
// pub mod e06_2_functional_fizzbuzz;
