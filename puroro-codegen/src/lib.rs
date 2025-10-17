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
