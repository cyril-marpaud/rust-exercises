//! # e03_1_temperature - Temperature Conversion
//!
//! Implement a temperature conversion library that can convert between Celsius, Fahrenheit and
//! Kelvin.
//!
//! It should have a `Temp` struct that stores the temperature and a `TempUnit` enum that represents
//! the temperature unit with support for Celsius, Fahrenheit and Kelvin.
//!
//! The following methods shall be available:
//!
//! - `new` - Instantiates a new `Temp` struct
//! - `convert` - Converts the temperature to the specified unit
//!
//! ## Conversion formulas:
//!
//! | From | To | Formula |
//! |:----:|:--:|:--------|
//! | C | F | F = 9 ⨉ C ÷ 5 + 32 |
//! | C | K | K = C + 273.15 |
//! | F | C | C = 5 ⨉ (F - 32) ÷ 9 |
//! | F | K | K = 5 ⨉ (F - 32) ÷ 9 + 273.15 |
//! | K | C | C = K - 273.15 |
//! | K | F | F = 9 ⨉ (K - 273.15) ÷ 5 + 32 |

// mod e03_1_temperature;
