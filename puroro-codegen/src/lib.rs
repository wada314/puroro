// Copyright 2021 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Puroro Codegen
//!
//! Code generation library for Puroro - converts `.proto` files into Rust code.
//!
//! This crate provides both:
//! - A library API for programmatic code generation
//! - A protoc plugin binary (`protoc-gen-puroro`)

#![deny(missing_docs)]

/// Error types for code generation.
pub mod error {
    use thiserror::Error;

    /// Errors that can occur during code generation.
    #[derive(Error, Debug)]
    pub enum Error {
        /// Error from protobuf parsing.
        #[error("Protobuf error: {0}")]
        Protobuf(String),

        /// Error during code generation.
        #[error("Code generation error: {0}")]
        CodeGen(String),

        /// I/O error.
        #[error("I/O error: {0}")]
        Io(#[from] std::io::Error),
    }
}

/// Main code generator.
pub mod generator {
    use super::error::Error;

    /// Generates Rust code from a Protocol Buffers CodeGeneratorRequest.
    pub fn generate(_request: &[u8]) -> Result<Vec<u8>, Error> {
        // TODO: Implement code generation
        todo!("Code generation not yet implemented")
    }
}
