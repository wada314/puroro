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
    ///
    /// `message` is filled by [`Error::while_decoding`] at the decode-function
    /// boundary when still [`None`].
    #[error("{}", fmt_unexpected_field(.message, *.field_number))]
    UnexpectedField {
        message: Option<&'static str>,
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

impl Error {
    /// Bare unexpected-field error (no protobuf message context yet).
    pub fn unexpected_field(field_number: u32) -> Self {
        Self::UnexpectedField {
            message: None,
            field_number,
        }
    }

    /// Attach the protobuf message type name when it is not set yet.
    ///
    /// Other variants (and already-contextualized nested decode errors) are
    /// returned unchanged.
    pub fn while_decoding(self, message: &'static str) -> Self {
        match self {
            Self::UnexpectedField {
                message: None,
                field_number,
            } => Self::UnexpectedField {
                message: Some(message),
                field_number,
            },
            other => other,
        }
    }
}

fn fmt_unexpected_field(message: &Option<&'static str>, field_number: u32) -> String {
    match message {
        Some(message) => {
            format!("unexpected field {field_number} while decoding {message}")
        }
        None => format!("unexpected wire payload for field {field_number}"),
    }
}

/// Result alias for this crate's typed errors.
pub type Result<T> = StdResult<T, Error>;
