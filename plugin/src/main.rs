#![cfg_attr(feature = "nightly", feature(backtrace))]

#[macro_use]
mod macros;
mod error;
mod generators;
mod plugin;
#[cfg(test)]
mod test;

use ::puroro::{Deserializable, Serializable};

use error::{ErrorKind, GeneratorError};
type Result<T> = std::result::Result<T, GeneratorError>;

use std::io::Read;
use std::io::{stdin, stdout};

use plugin::*;

fn main() -> Result<()> {
    let cgreq = CodeGeneratorRequest::from_bytes(stdin().bytes()).unwrap();
    let mut cgres = CodeGeneratorResponse::default();
    let mut file = CodeGeneratorResponse_File::default();
    file.name = "test.rs".to_string();
    file.content = generators::simple::generate_simple(&cgreq)?;
    cgres.file.push(file);
    cgres.serialize(&mut stdout())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
