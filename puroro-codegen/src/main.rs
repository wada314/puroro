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
