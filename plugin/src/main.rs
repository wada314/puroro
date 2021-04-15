#![cfg_attr(feature = "nightly", feature(backtrace))]

mod context;
mod error;
mod generators;
#[cfg(feature = "stage1")]
mod stage1;
#[cfg(feature = "stage2")]
mod stage2;
#[cfg(feature = "stage3")]
mod stage3;
mod utils;
mod wrappers;

use ::puroro::{Deserializable, Serializable};
use context::Context;

use error::{ErrorKind, GeneratorError};
type Result<T> = std::result::Result<T, GeneratorError>;

use std::io::Read;
use std::io::{stdin, stdout};

mod protos {
    #[cfg(feature = "stage1")]
    pub use crate::stage1::*;
    #[cfg(feature = "stage2")]
    pub use crate::stage2::google;
    #[cfg(feature = "stage3")]
    pub use crate::stage3::google;
}
use google::protobuf::compiler::{
    code_generator_response, CodeGeneratorRequest, CodeGeneratorResponse,
};
pub use protos::google;

fn main() -> Result<()> {
    let cgreq = CodeGeneratorRequest::from_bytes(stdin().bytes()).unwrap();
    let context = Context::new(cgreq.clone())?;
    let filename_and_content = generators::do_generate(&context)?;
    let mut cgres = CodeGeneratorResponse::default();
    cgres.file = filename_and_content
        .into_iter()
        .map(|(filename, content)| {
            let mut file = code_generator_response::File::default();
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
