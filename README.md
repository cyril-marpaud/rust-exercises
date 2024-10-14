# Rust Exercises

Each exercise requires one to uncomment a specific module in `lib.rs`. **Don't forget to do this!**

- E - Basic exercises: Easy exercises related to basic Rust concepts.
- C - Crate-specific exercises: Medium exercises related to specific crates.
- P - Projects: Advanced exercises involving multiple crates.

## Generating the guide

Run `cargo guide` in any exercise folder to generate and open its documentation in a browser, or run it in the root directory to document them all.

This is a cargo alias that has been set up in `.cargo/config.toml`.

## Checking your solutions

Most exercises have a test suite that you can run with `cargo test`.

Run it with the `-- --nocapture` flag to see the output of your code.

## Todo

- integrate solutions into their dedicated crate
- auto-publish documentation to gitlab pages
   - update workspace.package.documentation
- add more exercises/crates/projects
   - [slint](docs.rs/slint)
   - [neon](docs.rs/neon)
   - [Impl Snake for Micro:bit](https://gitlab.com/cyril-marpaud/impl_snake_for_microbit)
