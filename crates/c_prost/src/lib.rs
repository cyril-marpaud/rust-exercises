//! # c_prost - Prost! - Protocol Buffers in Rust
//!
//! **Protocol Buffers** (protobuf) is a language-neutral method for serializing structured data,
//! designed by Google. The **prost** crate provides a Rust implementation of Protocol Buffers,
//! helping serialize and deserialize protobuf messages into Rust structs.
//!
//! - [Prost Documentation](https://docs.rs/prost/latest/prost/)
//! - [Prost GitHub Repository](https://github.com/tokio-rs/prost)
//!
//! ## Protoc compiler installation
//!
//! One might need to install the `protoc` compiler firsthand:
//! - [official instructions](https://github.com/protocolbuffers/protobuf?tab=readme-ov-file#protobuf-compiler-installation)
//! - [official releases](https://github.com/protocolbuffers/protobuf/releases/tag/v28.2)
//! - debian/ubuntu install: `sudo apt install protobuf-compiler`
//! - [SO windows install](https://stackoverflow.com/questions/13616033/install-protocol-buffers-on-windows/67946001#67946001)
//!
//! ## Cargo Dependencies
//!
//! We’re going to need these dependencies:
//!
//! - `prost`: The core crate that provides message encoding and decoding for Protocol Buffers.
//! - `prost-build`: A build-time dependency that generates Rust code from `.proto` files.
//!
//! **prost-build** is a build dependency because it is used during the compile process to generate
//! Rust code from `.proto` files.
//!
//! ```bash
//! cargo add prost
//! cargo add --build prost-build
//! ```
//!
//! ## `.proto` File
//!
//! The `.proto` file is available in the `data` folder. It defines the structure of the serialized
//! data, which Prost uses to generate the Rust structs. For example, it defines a message
//! `Artifact` with fields like `name`, `origin`, `category`, and `year_discovered`.
//!
//! ```proto
//! syntax = "proto3";
//!
//! message Artifact {
//!   string name = 1;
//!   string origin = 2;
//!   string category = 3;
//!   int32 year_discovered = 4;
//! }
//! ```
//!
//! ## Generate Rust Code
//!
//! In order to generate Rust code from the `.proto` file, you need to create a `build.rs` file in
//! the root of your project (the same directory as `Cargo.toml`). The `build.rs` file is a build
//! script that is executed at compile time. It tells `prost-build` to compile the `.proto` file and
//! generate Rust code based on the message definitions.
//!
//! Follow [this guide in Prost's documentation](https://docs.rs/prost-build/latest/prost_build/) to
//! configure your `build.rs` file and generate Rust code from the `.proto` file.
//!
//! <details>
//! <summary>Solution</summary>
//!
//! ```ignore
//! prost_build::compile_protos(&["data/artifact.proto"], &["data/"]).unwrap();
//! ```
//!
//! </details>
//!
//! This script will generate the Rust code in the `OUT_DIR` at compile time.
//!
//! ## Implement Deserialization
//!
//! The goal is to **deserialize the binary file `data/artifacts.bin` into a `Vec<Artifact>`**,
//! where each binary message represents an artifact. The generated Rust code (from the `.proto`
//! file) needs to be included in your project at compile time. Use the `include!` macro to pull in
//! the generated code from `OUT_DIR`:
//!
//! ```ignore
//! include!(concat!(env!("OUT_DIR"), "/artifacts.rs"));
//! ```
//!
//! To achieve deserialization, follow these steps:
//!
//! - **Open and read the file**: Use `BufReader` to open the file and `read_to_end` to read its
//!   contents into a `Vec<u8>` buffer.
//! - **Decoding with Prost**: Use the
//!   [`decode_length_delimited`](https://docs.rs/prost/latest/prost/trait.Message.html#method.decode_length_delimited)
//!   method to decode each serialized artifact from the buffer.
//! - **Using a loop**: Since there may be multiple artifacts in the file, you need to loop over the
//!   buffer and decode each artifact. After decoding, update the buffer using the
//!   [`encoded_len`](https://docs.rs/prost/latest/prost/trait.Message.html#method.encoded_len)
//!   method to process the next message.
//! - **Return the result**: Collect the deserialized artifacts into a `Vec<Artifact>`.
//!
//! <details>
//! <summary>Solution</summary>
//!
//! ```ignore
//! use prost::Message;
//! use std::{
//!   fs::File,
//!   io::{BufReader, Read},
//! };
//!
//! include!(concat!(env!("OUT_DIR"), "/artifacts.rs"));
//!
//! pub fn deserialize_artifacts(file_path: &str) -> Vec<Artifact> {
//!   let mut file = BufReader::new(File::open(file_path).expect("Failed to open file"));
//!   let mut buffer = Vec::new();
//!   file.read_to_end(&mut buffer).expect("Failed to read file");
//!
//!   let mut rem_buf = &buffer[..];
//!   let mut artifacts = Vec::new();
//!
//!   while !rem_buf.is_empty() {
//!     let artifact = Artifact::decode_length_delimited(rem_buf).expect("Failed to decode Artifact");
//!
//!     let len = Artifact::encoded_len(&artifact);
//!     rem_buf = &rem_buf[len + 1..]; // Update the remaining buffer
//!
//!     artifacts.push(artifact);
//!   }
//!
//!   artifacts
//! }
//! ```
//!
//! </details>
//!
//! ## Create a Binary Executable
//!
//! To display the deserialized data, create a binary executable in the `src/bin/` folder:
//!
//! - Create a new file in `src/bin/` (e.g., `src/bin/display_artifacts.rs`).
//! - Import and call the `deserialize_artifacts` function in this file to read the serialized file
//!   and print the deserialized artifacts.
//!
//! <details>
//! <summary>Solution</summary>
//!
//! ```ignore
//! use c_prost::c_prost_1_deserialize::deserialize_artifacts;
//! let artifacts = deserialize_artifacts("data/artifacts.bin");
//! println!("artifacts: {artifacts:#?}");
//! ```
//! </details>

pub mod c_prost_1_deserialize;
