//! # c_reqwest - HTTP Client Adventure
//!
//! ## Overview
//! In this exercise, you will explore how to make HTTP requests using the [`reqwest`](https://docs.rs/reqwest) crate.
//! The goal is to navigate a sequence of challenges exposed via a web server by issuing the correct HTTP requests.
//!
//! You will:
//! - Practice sending different types of HTTP requests (GET, POST, PUT, DELETE).
//! - Use headers, request bodies (JSON, form), and basic authentication.
//! - Read and handle responses using the `reqwest` API.
//!
//! ## Requirements
//! Before starting:
//! - Launch the provided HTTP server in a separate terminal using:
//!   ```bash
//!   cargo c_reqwest_server
//!   ```
//! - You will need the `reqwest` crate with the `json` feature enabled, as well as the `serde_json` crate. It is up to you to import them.
//!
//! ## Instructions
//! Implement the client inside `main.rs`. The constant `BASE_URL` is already defined and points to the local server.
//!
//! ### Step 1 — Make a GET Request
//! Visit the server's root path (i.e., `/`) by sending a GET request to `BASE_URL`.
//!
//! Refer to the [`reqwest` documentation](https://docs.rs/reqwest/latest/reqwest/index.html#making-a-get-request) for help.
//!
//! <details>
//! <summary>Solution</summary>
//!
//! ```rust
//! let r = reqwest::get(format!("{BASE_URL}/")).await?.text().await?;
//! println!("{r}");
//! ```
//!
//! </details>
//!
//! ### Step 2 — Implement the Rest of the Flow
//!
//! At each step, the server responds with hints on how to proceed — so pay attention:
//! - Follow the instructions returned by the server
//! - Use the appropriate HTTP method and body format
//!
//! ### Response Handling
//! To work safely with the responses, use the following `reqwest` methods:
//!
//! - [`error_for_status`](https://docs.rs/reqwest/latest/reqwest/struct.Response.html#method.error_for_status): returns an error if the server responds with a 4xx or 5xx status.
//! - [`text`](https://docs.rs/reqwest/latest/reqwest/struct.Response.html#method.text): reads the response body as a `String`.
//! - [`status`](https://docs.rs/reqwest/latest/reqwest/struct.Response.html#method.status): retrieves the HTTP status code of the response.
//!
//! Good luck!
