//! - Uncomment the `deserialize` module
//! - Step 1
//!  - Import the [`serde`](https://docs.rs/serde/latest/serde/) crate **with the `derive` feature**
//!   - [Implement `Deserialize`](https://serde.rs/derive.html) for `Temp`
//!   - Use the adequate [attributes](https://serde.rs/attributes.html) on the fields of `Temp` and `TempUnit`
//! - Step 2
//!   - Import the [`serde_json`](https://docs.rs/serde_json/latest/serde_json/) crate
//!   - Create `src/bin/deserialize.rs`
//!   - [Open](https://doc.rust-lang.org/std/fs/struct.File.html#method.open) `data/temp_data.json`
//!   - Parse the data (`Vec<Temp>`) with [`serde_json::from_reader()`](https://docs.rs/serde_json/latest/serde_json/fn.from_reader.html)
//! - Step 3
//!   - Create a `City` structure (name + temperature)
//!   - Annotate its fields with the appropriate attributes
//!   - Use `serde_json` to parse a `Vec<City>`
//! - Step 4
//!   - Implement Ord (and therefore Eq) for `Temp` (use [`total_cmp`](https://doc.rust-lang.org/std/primitive.f64.html#method.total_cmp))
//!   - Use [`min_by_key()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.min_by_key) and [`max_by_key()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.max_by_key) to display the names and temperatures of the hottest and coldest cities

// mod deserialize;
