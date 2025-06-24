//! # e04_1_modularization - Modularization
//!
//! ## Overview
//! This exercise aims to organize and modularize the temperature conversion library to enhance code maintainability and separation of concerns.
//!
//! ## Task
//! - Move the `TempUnit` enum into its own module.
//! - Create a `prelude` module that exports `Temp`, `Celsius`, `Fahrenheit`, and `Kelvin`. These should be accessible directly, not as part of the enum.
//!
//! ## Instructions
//! 1. Define a new module named `temp_unit` and move the `TempUnit` enum into this module.
//!
//!    <details>
//!    <summary>Using the <code>mod</code> keyword</summary>
//!
//!    Declare the module with `mod` and create a new file in your library:
//!    ```ignore
//!    mod temp_unit;
//!    // The code shall be moved in `temp_unit.rs` or `temp_unit/mod.rs`
//!    ```
//!    This isolates the `TempUnit` enum in its own namespace, clarifying its scope and usage.
//!    </details>
//!
//! 2. Inside `lib.rs`, make the necessary adaptations
//!
//! 3. Create a `prelude` module which re-export `Temp` and `TempUnit`'s variants.
//!
//!    <details>
//!    <summary>Why use <code>pub</code> in a prelude module?</summary>
//!
//!    The `prelude` module is intended to simplify imports for the end users of your library by providing direct access to commonly used items:
//!    ```ignore
//!    pub mod prelude;
//!
//!    // prelude.rs
//!    pub use super::{...};
//!    pub use crate::{...};
//!    ```
//!    Marking these imports as `pub` ensures they can be easily included at the top level of consumer code, enhancing accessibility.
//!    </details>
//!
//! ## Using Rust Analyzer
//! Rust Analyzer can greatly simplify the process of refactoring and modularizing your code:
//! - **Extract to Module**: Select the code to extract, then use the "Extract to Module" quick fix. This can be done by clicking the yellow light bulb (💡) or using the **`ctrl`**+**`.`** shortcut.
//! - **Extract Module to File**: Place the cursor on the module name, then use the "Extract Module to File" quick fix to move the module to its own file.
//! - **Rename Module**: To rename a module, simply press **`F2`** when the cursor is on the module's name and enter the new name.
//!
//! Use these Rust Analyzer features as much as possible to automate these tasks, rather than doing them all manually, which enhances efficiency and reduces the potential for errors.

// pub mod e04_1_modularization;
