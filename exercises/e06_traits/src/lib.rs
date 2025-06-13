//! # e06_1_traits - Traits
//!
//! ## Overview
//! Enhance the idiomatic use of Rust by implementing common traits for the `Temp` and `TempUnit` types.
//! Implementing these traits will ensure that our temperature conversion library adheres to Rust's design principles, enhancing utility and compatibility.
//!
//! ## Task
//! Implement several traits to improve the functionality and usability of the `Temp` and `TempUnit` types:
//! - For both `Temp` and `TempUnit`:
//!   - [`Debug`](https://doc.rust-lang.org/std/fmt/trait.Debug.html): Enables logging and debugging by formatting the output using the `{:?}` formatter. `Debug` is [derivable](https://doc.rust-lang.org/std/fmt/trait.Debug.html#examples).
//!
//!     <details>
//!     <summary>Example of deriving <code>Debug</code></summary>
//!
//!     Simply add `#[derive(Debug)]` above your struct or enum definition:
//!     ```ignore
//!     #[derive(Debug)]
//!     pub struct Temp {
//!         temp: f64,
//!         unit: TempUnit,
//!     }
//!     ```
//!
//!     </details>
//!
//!   - [`Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html): Allows custom string representation, making objects printable in a user-friendly format. Not derivable, must be manually implemented.
//!
//!     <details>
//!     <summary>Example of a manual implementation</summary>
//!
//!     ```ignore
//!     impl Display for TempUnit { ... }
//!     ```
//!
//!     </details>
//!
//!   - [`Default`](https://doc.rust-lang.org/std/default/trait.Default.html): Provides a mechanism to create default values of these types. `Default` is [derivable](https://doc.rust-lang.org/std/default/trait.Default.html#derivable).
//! - For `Temp` only:
//!   - [`PartialEq`](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html): Enables comparison for equality. `PartialEq` is [derivable](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html#derivable).
//!   - [`PartialOrd`](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html): Allows objects to be compared, which is essential for ordering. `PartialOrd` is [derivable](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html#derivable).
//!
//! ## Instructions
//! - **Step 1:** Uncomment each unit test one by one.
//! - **Step 2:** Implement the required traits to make the tests pass. Use Rust Analyzer's quick fix "Implement Missing Members" to automatically generate method prototypes.
//!
//! ## Tips
//! - Utilize Rust's documentation to understand traits' use and applicability.
//! - Use Rust Analyzer to speed up trait implementation:
//!   - Click the yellow light bulb (💡) or use **`ctrl`**+**`.`** when a trait is not fully implemented to bring up the quick fixes menu.

// pub mod e06_1_traits;
