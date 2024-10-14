//! # e03_1_temperature - Temperature Conversion
//!
//! ## Overview
//! Implement a temperature conversion library that handles conversions between Celsius (C), Fahrenheit (F), and Kelvin (K).
//!
//! ## Components
//! - `Temp` struct: Holds a temperature value along with its unit.
//! - `TempUnit` enum: Defines the units of temperature, such as Celsius, Fahrenheit, and Kelvin.
//!
//! ## Methods
//! - `new`: Creates a new `Temp` instance with specified temperature and unit.
//!   ```ignore
//!   let temp = Temp::new(25.0, TempUnit::Celsius);
//!   ```
//! - `convert`: Modifies the `Temp` instance to convert its temperature to a specified unit.
//!   ```ignore
//!   temp.convert(TempUnit::Fahrenheit);
//!   ```
//!
//! ## Conversion Formulas
//! | From | To   | Formula                           |
//! |:----:|:----:|:---------------------------------:|
//! | C    | F    | F = 9 ⨉ C ÷ 5 + 32                |
//! | C    | K    | K = C + 273.15                    |
//! | F    | C    | C = 5 ⨉ (F - 32) ÷ 9              |
//! | F    | K    | K = 5 ⨉ (F - 32) ÷ 9 + 273.15     |
//! | K    | C    | C = K - 273.15                    |
//! | K    | F    | F = 9 ⨉ (K - 273.15) ÷ 5 + 32     |

// mod e03_1_temperature;
