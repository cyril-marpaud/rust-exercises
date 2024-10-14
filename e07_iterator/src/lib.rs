//! # e07_1_iterator - Iterators
//!
//! ## Overview
//! This exercise involves implementing the `Iterator` trait for the `Fibonacci` struct. You will learn how iterators work in Rust and explore different methods provided by the `Iterator` trait.
//!
//! ## Task
//! The `Fibonacci` struct is initially empty, and your task is to transform it into a functional iterator that can generate Fibonacci numbers:
//! - Create a struct, `FibIter`, to hold the iteration state. You have two choices for managing the state:
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

// pub mod e07_1_iterator;
