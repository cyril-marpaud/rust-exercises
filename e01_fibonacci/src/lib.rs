//! - Clone or download the exercise repo
//!   - [`git clone https://gitlab.com/cmp-trainings/rust/rust-exercises.git`](https://gitlab.com/cmp-trainings/rust/rust-exercises)
//! - In `lib.rs`, uncomment `fibonacci_recursive`
//! - In `fibonacci_recursive.rs`, implement the function
//!   - `fib(0) = 0`, `fib(1) = 1`, `fib(n) = fib(n-1) + fib(n-2)`
//! - The exercise is OK when the tests pass
//!   - `cargo test -- fibonacci_recursive`
//!
//! # TP#2: Fibonacci... Faster
//! - Uncomment the module `fibonacci_closed_form`
//! - Implement the function
//! - Run the tests
//!
//! - $fib(n) = \frac{\phi^n - \psi^n}{\sqrt{5}}$
//!   - $\phi = \frac{1 + \sqrt{5}}{2}$
//!   - $\psi = \frac{1 - \sqrt{5}}{2} = 1 - \phi$
//!
//! - tools:
//!   - `4f64.sqrt() = 2.0`
//!   - `2f64.powi(2) = 4.0`
//!   - `4.5f64.round() = 5.0`
//!   - `2 as f64 = 2.0`

//! <!--
//! Show configuration for:
//! - automatic linting
//! - automatic formatting
//! - documenting
//! -->

// mod e01_1_recursive;
// mod e01_2_closed_form;
