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

/// Repeated field runtime trait and adapters.
pub mod repeated;

pub use repeated::OnceListRepeatedMap;

/// Reference-separated vector: split API for holding element refs while appending.
pub mod split_vec;

/// Slice-backed lazy parser infrastructure for Protocol Buffer messages.
///
/// This is the original lazy parsing implementation that assumes input slices have
/// a stable lifetime (`&'slice [u8]`), allowing zero-copy field iteration.
///
/// Provides:
/// - FieldIterator: iterator over protobuf fields in slices
/// - MessageParserStateRef: parser state shared between message bodies and child messages
#[path = "lazy_parser.rs"]
pub mod lazy_slice_parser;

/// Slice-backed lazy wrapper for repeated fields enabling on-demand parsing.
#[path = "repeated_lazy.rs"]
pub mod repeated_lazy_slice;

/// Async/streaming lazy parsing implementation.
///
/// This module provides a poll-based lazy parser that can read from an async reader
/// and supports random-access getters by caching decoded values and/or raw bytes.
pub mod lazy_async;

/// Async/streaming lazy wrapper for repeated fields.
pub mod repeated_lazy_async;

/// Core message trait that all generated Protocol Buffer messages implement.
pub trait Message: Sized + Clone + PartialEq {
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
        A: Allocator + Clone;

    /// Serializes this message using the provided allocator.
    fn write_to_bytes_in<A>(&self, alloc: A) -> Result<AllocVec<u8, A>, crate::error::Error>
    where
        A: Allocator + Clone;

    /// Computes the serialized size of this message in bytes.
    fn compute_size(&self) -> usize;
}

/// Error types for Protocol Buffer operations.
pub mod error {
    use ::protobuf_core::FieldValue;
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

        /// Invalid UTF-8 in string field (borrowed view).
        ///
        /// This is used by lazy parsing paths that validate UTF-8 without allocating a `String`.
        #[error("Invalid UTF-8: {0}")]
        InvalidUtf8Str(#[from] std::str::Utf8Error),

        /// Attempted to add slice after message has been terminated.
        /// This occurs when a terminating getter (e.g., scalar field getter that checks all slices)
        /// has been called, making the message state immutable.
        #[error("Cannot add slice: message has been terminated by a terminating getter call")]
        MessageTerminated,

        /// Field has unexpected wire type.
        /// According to the .proto file definition, this field should always have a specific wire type,
        /// but the actual wire type in the data is different.
        ///
        /// Note: According to the Protocol Buffers specification, fields with unexpected wire types
        /// could be treated as unknown fields and preserved for forward compatibility. However,
        /// this implementation currently treats it as an error for strict validation.
        /// Future enhancement: Consider storing such fields in unknown_fields storage instead.
        #[error(
            "Field {field_number}: expected wire type {expected_wire_type:?} (Len), but found {found_wire_type:?}"
        )]
        UnexpectedWireType {
            /// Field number that has the unexpected wire type
            field_number: u32,
            /// Expected wire type according to .proto definition
            expected_wire_type: &'static str,
            /// Actual wire type found in the data
            found_wire_type: &'static str,
        },
    }

    impl From<protobuf_core::ProtobufError> for Error {
        fn from(err: protobuf_core::ProtobufError) -> Self {
            match err {
                protobuf_core::ProtobufError::FieldNumberOutOfRange { value } => {
                    Error::InvalidWireFormat(format!("Field number out of range: {}", value))
                }
                protobuf_core::ProtobufError::InvalidWireType { value } => {
                    Error::InvalidWireFormat(format!("Invalid wire type: {}", value))
                }
                protobuf_core::ProtobufError::VarintDowncastOutOfRange { value, target_type } => {
                    Error::InvalidWireFormat(format!(
                        "Varint value {} out of range for {}",
                        value, target_type
                    ))
                }
                protobuf_core::ProtobufError::VarintTooLong => Error::InvalidWireFormat(
                    "Varint exceeds maximum length of 10 bytes".to_string(),
                ),
                protobuf_core::ProtobufError::MalformedTag {
                    field_number,
                    wire_type,
                } => Error::InvalidWireFormat(format!(
                    "Malformed tag: field_number={}, wire_type={}",
                    field_number, wire_type
                )),
                protobuf_core::ProtobufError::UnexpectedEof => {
                    Error::InvalidWireFormat("Unexpected EOF while parsing field".to_string())
                }
                protobuf_core::ProtobufError::IoError(e) => Error::Io(e),
                protobuf_core::ProtobufError::FieldTypeDowncastError { expected_type } => {
                    Error::InvalidWireFormat(format!(
                        "Field type downcast error: {}",
                        expected_type
                    ))
                }
            }
        }
    }

    impl From<Error> for std::io::Error {
        fn from(e: Error) -> Self {
            std::io::Error::new(std::io::ErrorKind::InvalidData, e)
        }
    }

    impl Error {
        /// Returns true if this error is "unexpected EOF" (e.g. from `read_exact`-style operations).
        /// Callers can use this to treat EOF as a normal end-of-stream (e.g. return `Ok(None)`) instead of propagating an error.
        pub fn is_unexpected_eof(&self) -> bool {
            matches!(
                self,
                Error::Io(e) if e.kind() == std::io::ErrorKind::UnexpectedEof
            )
        }
    }

    /// Get the wire type name from a FieldValue.
    ///
    /// Returns a static string representation of the wire type.
    fn wire_type_name<L>(value: &FieldValue<L>) -> &'static str {
        match value {
            FieldValue::Varint(_) => "Varint",
            FieldValue::I32(_) => "Int32",
            FieldValue::I64(_) => "Int64",
            FieldValue::Len(_) => "Len",
        }
    }

    impl Error {
        /// Create an `UnexpectedWireType` error from a field and its actual value.
        ///
        /// This helper function extracts the wire type name from the actual `FieldValue`
        /// and creates an appropriate error. The caller only needs to provide the field number
        /// and the expected wire type name.
        ///
        /// # Behavior
        ///
        /// According to the Protocol Buffers specification, fields with unexpected wire types
        /// could be treated as unknown fields and preserved for forward compatibility.
        /// However, this implementation currently treats it as an error for strict validation.
        /// Future enhancement: Consider storing such fields in unknown_fields storage instead.
        ///
        /// # Arguments
        /// * `field_number` - The field number that has the unexpected wire type
        /// * `expected_wire_type` - The expected wire type name (e.g., "Len")
        /// * `actual_value` - The actual `FieldValue` that was found
        ///
        /// # Example
        /// ```ignore
        /// let FieldValue::Len(data) = field.value else {
        ///     return Err(Error::unexpected_wire_type(
        ///         field.field_number.as_u32(),
        ///         "Len",
        ///         &field.value,
        ///     ));
        /// };
        /// ```
        pub fn unexpected_wire_type<L>(
            field_number: u32,
            expected_wire_type: &'static str,
            actual_value: &FieldValue<L>,
        ) -> Self {
            Self::UnexpectedWireType {
                field_number,
                expected_wire_type,
                found_wire_type: wire_type_name(actual_value),
            }
        }
    }
}
