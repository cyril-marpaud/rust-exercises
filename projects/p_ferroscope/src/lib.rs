//! # FerroScope: A Generic File Search Tool
//!
//! In this exercise, we'll implement a file search tool that can find text patterns across different file types. The tool uses a trait-based abstraction to handle various file formats uniformly, demonstrating Rust's powerful trait system and its application in real-world scenarios.
//!
//! ## Introduction: Project Overview
//!
//! FerroScope is a command-line application that searches for text patterns within files in a given directory. The key feature is its extensible architecture: by implementing a single trait (`FerroScope`), we can add support for new file types without modifying the core search logic.
//!
//! We'll implement search capabilities for:
//! - Text files (`.txt` and `.md`)
//! - PDF documents (`.pdf`)
//!
//! The program takes two arguments:
//! 1. A path to a directory to search in
//! 2. A text pattern to search for
//!
//! For each match found, the program displays:
//! - The file path
//! - The line number and column where the match starts
//! - The matched text with surrounding context (30 characters before and after by default)
//!
//! Example usage:
//! ```bash
//! cargo run --release -- ./assets sparks
//! ```
//!
//! The `assets` directory contains sample `.txt`, `.md`, and `.pdf` files for testing your implementation at each step.
//!
//! ## Part 1: Core Data Structures
//!
//! ### Understanding FerroMatch and Occurence (`mod.rs`)
//!
//! Before implementing the search logic, let's understand the data structures used to represent search results:
//!
//! - **`FerroMatch`**: Represents all matches found in a single file
//!   - `path: PathBuf` - The path to the file containing matches
//!   - `occs: Vec<Occurence>` - A vector of all occurrences found in this file
//!
//! - **`Occurence`**: Represents a single match within a file
//!   - `line: usize` - The line number (1-indexed) where the match was found
//!   - `col: usize` - The column number (0-indexed) where the match starts
//!   - `occ: String` - The actual matched text with `CONTEXT` characters of context on each side
//!
//! ## Part 2: The FerroScope Trait
//!
//! ### 1. Understanding the Trait (`mod.rs`)
//!
//! The `FerroScope` trait defines the interface for any file searcher. It includes:
//! - **`CONTEXT`**: A constant defining how many characters of context to include (default: 30)
//! - **`EXTENSION_FILTER`**: A function constant that filters files by extension
//! - **`read_one`**: Reads a single file and returns its entire content as a String. Each implementor defines how to extract text from its specific file format.
//! - **`find`**: Scans a directory and returns paths to all files matching the extension filter. This default implementation works for most cases but can be overridden if needed.
//! - **`search`**: Coordinates the complete search operation: calls `find` to list files, reads each with `read_one`, searches for the pattern within the text, and builds the result structure with matches and context.
//!
//! ### 2. Implementing `find` (`mod.rs`)
//!
//! Implement the `find` method that discovers all files to search within a given directory. It should:
//! - Use [`std::fs::read_dir`](https://doc.rust-lang.org/std/fs/fn.read_dir.html) to list directory contents
//! - Filter out any errors with [`filter_map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter_map) and [`Result::ok`](https://doc.rust-lang.org/std/result/enum.Result.html#method.ok)
//! - Extract the path from each [`DirEntry`](https://doc.rust-lang.org/std/fs/struct.DirEntry.html) using the [`path`](https://doc.rust-lang.org/std/fs/struct.DirEntry.html#method.path) method
//! - Filter files by extension using `EXTENSION_FILTER`:
//!   - Extract the extension with [`Path::extension`](https://doc.rust-lang.org/std/path/struct.Path.html#method.extension)
//!   - Convert it to `&str` with [`OsStr::to_str`](https://doc.rust-lang.org/std/ffi/struct.OsStr.html#method.to_str)
//!   - Apply the `EXTENSION_FILTER` function
//! - Return a `Vec<PathBuf>` of matching files
//!
//! <details>
//! <summary>💡 Hint: Method chaining</summary>
//!
//! ```ignore
//! std::fs::read_dir(path)
//!     .unwrap()
//!     .filter_map(Result::ok)
//!     .map(|e| e.path())
//!     .filter(|p| /* check extension with EXTENSION_FILTER */)
//!     .collect()
//! ```
//! </details>
//!
//! ### 3. Implementing `search` (`mod.rs`)
//!
//! Implement the `search` method that orchestrates the entire search process:
//! - Call `find` to get all files to search
//! - For each file:
//!   - Read its contents using `read_one`
//!   - Search for the pattern in each line using [`windows`](https://doc.rust-lang.org/std/primitive.slice.html#method.windows) to slide through characters and [`position`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.position) to find the first match
//!   - Convert matching character positions (not byte positions!) into `Occurence` objects with [`map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map)
//!   - Collect all occurrences into a `FerroMatch`
//! - Filter out files with no matches
//! - Return `Vec<FerroMatch>`
//!
//! <details>
//! <summary>⚠️ UTF-8 Safety</summary>
//!
//! When extracting context, work with character indices, not byte indices. Use [`chars`](https://doc.rust-lang.org/std/primitive.str.html#method.chars) to convert the line to a `Vec<char>` first:
//! ```ignore
//! let chars: Vec<char> = txt.chars().collect();
//! let pattern_chars: Vec<char> = pattern.chars().collect();
//! ```
//! </details>
//!
//! ### 4. Understanding `read_one`
//!
//! This is an abstract method that each file type implementation must provide. It takes a [`&Path`](https://doc.rust-lang.org/std/path/struct.Path.html) and returns the file's contents as a `String`. We'll implement this method twice later:
//! - Once for `FerroText` to handle text files
//! - Once for `FerroPdf` to handle PDF files
//!
//! Each implementation will use different techniques to extract text from its specific file format.
//!
//! ## Part 3: Command-Line Interface
//!
//! ### Setting up `main.rs`
//!
//! Implement the initial command-line interface. For now, focus only on argument parsing and validation:
//!
//! 1. **Retrieve command-line arguments** using [`std::env::args`](https://doc.rust-lang.org/std/env/fn.args.html)
//! 2. **Validate arguments**:
//!    - Check that exactly 3 arguments are provided (program name + 2 args)
//!    - Display usage message if incorrect: `Usage: <program> <path> <pattern>` and exit with [`std::process::exit(1)`](https://doc.rust-lang.org/std/process/fn.exit.html)
//! 3. **Validate the path**:
//!    - Convert the string to a [`Path`](https://doc.rust-lang.org/std/path/struct.Path.html) using [`Path::new`](https://doc.rust-lang.org/std/path/struct.Path.html#method.new)
//!    - Check it exists using [`Path::exists`](https://doc.rust-lang.org/std/path/struct.Path.html#method.exists)
//!    - Display an error message and exit if the path doesn't exist
//!
//! Don't implement the actual search logic yet, we'll add that after implementing the trait for specific file types.
//!
//! ## Part 4: Text File Support
//!
//! ### Implementing FerroText (`text.rs`)
//!
//! 1. **Create the struct**: Define an empty struct `FerroText`
//!
//! 2. **Implement the trait**: `impl FerroScope for FerroText`
//!    - You only need to implement `read_one`, the trait provides default implementations for `find` and `search`
//!
//! 3. **Override `EXTENSION_FILTER`**:
//!    - Modify the constant to accept only files with extensions "txt" or "md"
//!    - You can use boolean logic (`||`) or pattern matching with [`matches!`](https://doc.rust-lang.org/std/macro.matches.html)
//!
//! 4. **Implement `read_one`**:
//!    - Use [`std::fs::read_to_string`](https://doc.rust-lang.org/std/fs/fn.read_to_string.html) which reads the entire file content into a single `String`
//!    - Return `unwrap_or_default()` to handle errors gracefully (returns an empty string if the file can't be read)
//!
//! ## Part 5: PDF Support
//!
//! ### Implementing FerroPdf (`pdf.rs`)
//!
//! 1. **Add a PDF reading dependency**: You can use [`pdf-extract`](https://docs.rs/pdf-extract/) (`cargo add pdf-extract`), or choose any other PDF reading crate ([`lopdf`](https://docs.rs/lopdf/), [`pdf`](https://docs.rs/pdf/), etc.) as long as it can extract text from PDFs.
//!
//! 2. **Create the struct**: Define an empty struct `FerroPdf`
//!
//! 3. **Implement the trait**: `impl FerroScope for FerroPdf`
//!    - You only need to implement `read_one`, the default implementations of `find` and `search` will be used
//!
//! 4. **Override constants**:
//!    - Increase `CONTEXT` to 50 characters
//!    - Override `EXTENSION_FILTER` to accept only files with the "pdf" extension
//!
//! 5. **Implement `read_one`**:
//!    - If using `pdf-extract`: call [`pdf_extract::extract_text`](https://docs.rs/pdf-extract/latest/pdf_extract/fn.extract_text.html) which takes a `&Path` and returns `Result<String, PdfExtractError>`
//!    - For other crates, follow their documentation to extract text from the PDF
//!    - Use `unwrap_or_default()` for error handling
//!
//! ## Part 6: Display Implementation
//!
//! ### Implementing Display for FerroMatch (`mod.rs`)
//!
//! Implement the [`Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html) trait to format search results nicely:
//! - Display the file path on its own line
//! - For each occurrence, display:
//!   - Line number and column (e.g., `L10:C25`)
//!   - The matched text with context, surrounded by `...`
//!
//! Use [`writeln!`](https://doc.rust-lang.org/std/macro.writeln.html) for formatted output and [`try_for_each`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.try_for_each) to handle potential write errors.
//!
//! ## Part 7: Bringing It All Together
//!
//! ### Completing `main.rs`
//!
//! 1. **Create searcher instances**: Create an instance of both `FerroText` and `FerroPdf`
//!
//! 2. **Perform searches**:
//!    - Call `search` on each searcher
//!    - Combine results (e.g., using [`extend`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.extend))
//!
//! 3. **Display results**:
//!    - Check if results are empty and display appropriate message
//!    - Iterate and print each `FerroMatch`
//!
//! 4. **Test with different keywords**:
//!    - Try searching for "rust", "trait" or "impl" in the assets folder
//!    - Verify that context is displayed correctly
//!    - Ensure both text and PDF files are searched

// pub mod p_ferroscope;
