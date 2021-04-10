#![cfg_attr(feature = "nightly", feature(backtrace))]

#[macro_use]
mod macros;
mod error;
mod generators;
mod plugin;

use ::puroro::{Deserializable, Serializable};
use generators::shared::Context;

use error::{ErrorKind, GeneratorError};
type Result<T> = std::result::Result<T, GeneratorError>;

use std::io::Read;
use std::io::{stdin, stdout};

use plugin::*;

#[cfg(test)]
mod google;

fn main() -> Result<()> {
    let cgreq = CodeGeneratorRequest::from_bytes(stdin().bytes()).unwrap();
    let mut context = Context::new(&cgreq)?;
    let filename_and_content = generators::simple::generate_simple(&mut context)?;
    let mut cgres = CodeGeneratorResponse::default();
    cgres.file = filename_and_content
        .into_iter()
        .map(|(filename, content)| {
            let mut file = CodeGeneratorResponse_File::default();
            file.name = filename;
            file.content = content;
            file
        })
        .collect();
    cgres.serialize(&mut stdout())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
