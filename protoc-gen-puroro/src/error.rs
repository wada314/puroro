//! Error types for the puroro protoc plugin.

use ::protobuf_core::ProtobufError;
use ::std::io;
use ::std::result::Result as StdResult;
use ::std::string::FromUtf8Error;
use ::thiserror::Error;

/// Errors produced while decoding plugin input, building IR, or emitting code.
#[derive(Debug, Error)]
pub enum Error {
    /// Underlying wire-format / I/O failure from `protobuf-core`.
    #[error("protobuf wire error: {0}")]
    Protobuf(#[from] ProtobufError),
    /// A field had an unexpected wire type or payload shape.
    #[error("unexpected field {field_number} while decoding {message}")]
    UnexpectedField {
        message: &'static str,
        field_number: u32,
    },
    /// A length-delimited string was not valid UTF-8.
    #[error("invalid UTF-8 in protobuf string: {0}")]
    InvalidUtf8(#[from] FromUtf8Error),
    /// Code generation failed for a logical reason (missing file, bad option, …).
    #[error("codegen error: {0}")]
    Codegen(String),
    /// Stdin / stdout / filesystem I/O.
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
}

/// Result alias for this crate's typed errors.
pub type Result<T> = StdResult<T, Error>;
