//! protoc plugin binary for Puroro.
//!
//! This binary reads a CodeGeneratorRequest from stdin and writes a CodeGeneratorResponse to stdout,
//! following the protoc plugin protocol.

use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    // Read CodeGeneratorRequest from stdin
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input)?;

    // Generate code
    let output = puroro_codegen::generator::generate(&input)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    // Write CodeGeneratorResponse to stdout
    io::stdout().write_all(&output)?;

    Ok(())
}
