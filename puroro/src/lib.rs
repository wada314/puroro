//! # Puroro
//!
//! A Rust-idiomatic implementation of Google Protocol Buffers.
//!
//! This crate provides the runtime library used by code generated from `.proto` files.
//! It focuses on:
//! - Precise Protocol Buffers specification support
//! - Rust-idiomatic APIs that leverage the type system
//! - Memory safety without sacrificing performance
//!
//! ## Design Philosophy
//!
//! Unlike some other implementations that are constrained by C/C++ interface compatibility,
//! Puroro is designed from the ground up to feel natural in Rust, using modern Rust patterns
//! and idioms throughout.

#![deny(missing_docs)]
#![deny(unsafe_code)]

// Re-export protobuf-core for use by generated code
pub use protobuf_core;

use ::allocator_api2::vec::Vec as AllocVec;
use ::allocator_extras::{Allocator, Global};

/// Generic field operations using trait-based dispatch.
///
/// This module provides a unified interface for all field types,
/// using generic type parameters to encode field attributes.
///
/// # Key Components
///
/// - `Field`: Type-level descriptor for protobuf fields
/// - `FieldType`: Concrete field implementation with data storage
/// - `FieldSet`, `FieldGet`, `FieldClear`: Split traits for field operations
pub mod field_ops;

/// Shared fields for message implementations.
///
/// Provides a wrapper type for fields shared across all message fields.
pub mod shared;

/// View types for Protocol Buffer message fields.
///
/// Provides types for returning flexible views of message fields
/// that support both borrowed references and owned values.
pub mod view;

/// Core message trait that all generated Protocol Buffer messages implement.
pub trait Message: Sized + Default + Clone + PartialEq {
    /// Parses a message from the given byte slice.
    fn parse_from_bytes(bytes: &[u8]) -> Result<Self, crate::error::Error> {
        Self::parse_from_bytes_in(bytes, Global)
    }

    /// Serializes this message to a byte vector.
    fn write_to_bytes(&self) -> Result<Vec<u8>, crate::error::Error> {
        let buffer = self.write_to_bytes_in(Global)?;
        Ok(buffer.into_iter().collect())
    }

    /// Parses a message using the provided allocator.
    fn parse_from_bytes_in<A>(bytes: &[u8], alloc: A) -> Result<Self, crate::error::Error>
    where
        A: Allocator + Clone + Default + 'static;

    /// Serializes this message using the provided allocator.
    fn write_to_bytes_in<A>(&self, alloc: A) -> Result<AllocVec<u8, A>, crate::error::Error>
    where
        A: Allocator + Clone + Default + 'static;

    /// Computes the serialized size of this message in bytes.
    fn compute_size(&self) -> usize;
}

/// Error types for Protocol Buffer operations.
pub mod error {
    use thiserror::Error;

    /// Errors that can occur during Protocol Buffer operations.
    #[derive(Error, Debug)]
    pub enum Error {
        /// I/O error occurred during reading or writing.
        #[error("I/O error: {0}")]
        Io(#[from] std::io::Error),

        /// Invalid wire format data.
        #[error("Invalid wire format: {0}")]
        InvalidWireFormat(String),

        /// Required field is missing.
        #[error("Required field missing: {0}")]
        RequiredFieldMissing(String),

        /// Invalid UTF-8 in string field.
        #[error("Invalid UTF-8: {0}")]
        InvalidUtf8(#[from] std::string::FromUtf8Error),
    }
}
