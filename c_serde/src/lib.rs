//! # c_serde - Deserialization
//!
//! This exercise guides you through the process of deserializing JSON data into Rust structures
//! using `serde` and `serde_json`.
//!
//! Documentation can be found at <serde.rs>, <docs.rs/serde> and <docs.rs/serde_json>.
//!
//! ## Step 1: Setting Up `serde`
//! - Import the [`serde`](https://docs.rs/serde/latest/serde/) crate with the `derive` feature
//!   enabled: `cargo add serde --features derive`
//! - [Implement `Deserialize`](https://serde.rs/derive.html) for the `Temp` struct. Don't implement
//!   it yourself, it can be derived
//! - Use the appropriate [attributes](https://serde.rs/attributes.html) on the fields of `Temp` and
//!   `TempUnit` to describe how the JSON data should be deserialized
//!
//! ## Step 2: Using `serde_json` to Deserialize Data
//! - Import the [`serde_json`](https://docs.rs/serde_json/latest/serde_json/) crate:
//!   `cargo add serde_json`
//! - Create `src/bin/deserialize.rs`
//! - [Open](https://doc.rust-lang.org/std/fs/struct.File.html#method.open) the file
//!   `data/temperatures.json`
//! - Parse the JSON file into a vector of `Temp` structs using
//!   [`serde_json::from_reader`](https://docs.rs/serde_json/latest/serde_json/fn.from_reader.html)
//!
//! ## Step 3: Expand to a City Structure
//! - Define a `City` structure with fields for name and temperature
//! - Annotate the `City` fields with appropriate `serde` attributes for deserialization
//! - Use `serde_json` to deserialize data into a vector of `City` instances
//!
//! ## Step 4: Implementing Ord and Displaying Results
//! - Implement `Ord` (and therefore `Eq`) for `Temp` using the
//!   [`total_cmp`](https://doc.rust-lang.org/std/primitive.f64.html#method.total_cmp) method on
//!   floating point numbers
//! - Utilize
//!   [`min_by_key()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.min_by_key) and
//!   [`max_by_key()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.max_by_key) to
//!   find and display the names and temperatures of the hottest and coldest cities
//! - Update the tests to check for the correct results

// mod deserialize;
