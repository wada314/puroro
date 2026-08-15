//! `protoc-gen-puroro` — library entry points for the puroro code generator.
//!
//! The binary is a thin stdin/stdout wrapper around [`run_plugin`]. Tests and
//! future `build.rs` / CLI wrappers should call [`generate_from_bytes`] (or the
//! descriptor / resolved helpers) directly.

pub mod case;
pub mod default_value;
pub mod descriptor;
pub mod emit;
pub mod error;
pub mod field_kind;
pub mod module_tree;
pub mod plugin_io;
pub mod resolved;

pub use crate::descriptor::{CodegenMeta, CodegenRequest, ProtoFqn};
pub use crate::error::{Error, Result};
pub use crate::plugin_io::{CodeGeneratorResponse, ResponseFile};

use crate::emit::emit;
use crate::plugin_io::decode_request;
use ::std::io::{self, Read, Write};

/// Decode a `CodeGeneratorRequest`, emit files, and return response wire bytes.
pub fn generate_from_bytes(request_bytes: &[u8]) -> Result<Vec<u8>> {
    let request = decode_request(request_bytes)?;
    let response = match emit(&request) {
        Ok(response) => response,
        Err(e) => CodeGeneratorResponse::from_error(e.to_string()),
    };
    response.encode()
}

/// Read a `CodeGeneratorRequest` from `stdin` and write the response to `stdout`.
pub fn run_plugin() -> Result<()> {
    let mut stdin = io::stdin().lock();
    let mut request_bytes = Vec::new();
    stdin.read_to_end(&mut request_bytes)?;

    let response_bytes = generate_from_bytes(&request_bytes)?;

    let mut stdout = io::stdout().lock();
    stdout.write_all(&response_bytes)?;
    stdout.flush()?;
    Ok(())
}
