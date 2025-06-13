//! # c_serde - Deserialization
//!
//! This exercise guides you through the process of deserializing JSON data into Rust structures
//! using `serde` and `serde_json`.
//!
//! Documentation can be found at [Serde.rs](https://serde.rs/), [docs.rs/serde](https://docs.rs/serde), and [docs.rs/serde_json](https://docs.rs/serde_json).
//!
//! ## Step 1: Setting Up `serde`
//! - Import the [`serde`](https://docs.rs/serde/latest/serde/) crate with the `derive` feature enabled.
//!
//!   <details>
//!   <summary>How to add serde to your Cargo.toml</summary>
//!
//!   ```bash
//!   cargo add serde --features derive
//!   ```
//!   </details>
//!
//! - Implement [`Deserialize`](https://serde.rs/derive.html) for the `Temp` struct. This can be derived.
//!
//! - Use the appropriate [attributes](https://serde.rs/attributes.html) to control deserialization of `Temp` and `TempUnit`.
//!
//! ## Step 2: Using `serde_json` to Deserialize Data
//! - Import the [`serde_json`](https://docs.rs/serde_json/latest/serde_json/) crate.
//! - Create `src/bin/deserialize.rs` and [open](https://doc.rust-lang.org/std/fs/struct.File.html#method.open) the JSON file.
//! - Use [from_reader](https://docs.rs/serde_json/latest/serde_json/fn.from_reader.html) to deserialize the JSON data into a vector of `Temp` instances.
//!
//!   <details>
//!   <summary>Solution code for deserializing into Temp structs</summary>
//!
//!   ```ignore
//!   let temps: Vec<Temp> = serde_json::from_reader(file).expect("error while reading json");
//!   ```
//!
//!   </details>
//!
//! ## Step 3: Expand to a City Structure
//! - Define a `City` structure with fields for name and temperature.
//!
//!   <details>
//!   <summary>Example of defining a City structure with serde attributes</summary>
//!
//!   ```ignore
//!   #[derive(Deserialize)]
//!   pub struct City {
//!       name: String,
//!       temperature: Temp,
//!   }
//!   ```
//!
//!   </details>
//!
//! - Use the appropriate [attributes](https://serde.rs/attributes.html) to control deserialization of `City`.
//! - Use `serde_json` to deserialize data into a vector of `City` instances.
//!
//! ## Step 4: Implementing Ord and Displaying Results
//! - Implement `Ord` for `Temp` using the [`total_cmp`](https://doc.rust-lang.org/std/primitive.f64.html#method.total_cmp) method on floating-point numbers.
//! - Utilize [`min_by_key()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.min_by_key) and [`max_by_key()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.max_by_key) to find and display the names and temperatures of the hottest and coldest cities
//!
//!   <details>
//!   <summary>Example of using min_by_key</summary>
//!
//!   ```ignore
//!       let cities = vec![...]; // Assume this vector is filled with City instances
//!       let coldest = cities.iter().min_by_key(|c| c.temperature.temp);
//!   ```
//!
//!   </details>
//!
//! - Update the tests to check for correct results

// pub mod deserialize;
