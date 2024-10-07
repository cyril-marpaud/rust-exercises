//! # e07_1_iterator - Iterators
//!
//! In this exercise, we will implement the `Iterator` trait for the `Fibonacci` struct. This will
//! allow us to understand how iterators work and use all the methods provided by the `Iterator`
//! trait on our types.
//!
//! The Fibonacci struct is empty, we thus have to transform it into another struct that can hold
//! the state of the iteration (last two values, index, etc.) first. That is the purpose of the
//! `IntoIterator` trait.
//!
//! Once this is done, we can then implement the `Iterator` trait that does the actual iteration
//! using that new struct's fields to produce the next value. Each call to `next` should thus return
//! the next Fibonacci number and update the state accordingly.
//!
//! - Create a struct that can hold the state of the iteration (last two values, index, etc.)
//! - Implement the necessary traits:
//!   - [`IntoIterator`](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html) for `Fibonacci`
//!   - [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) for the new type

// mod e07_1_iterator;
