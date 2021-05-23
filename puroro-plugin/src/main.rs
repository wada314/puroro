#![cfg_attr(feature = "puroro-nightly", feature(backtrace))]
#![cfg_attr(feature = "puroro-nightly", feature(generic_associated_types))]
#![cfg_attr(feature = "puroro-nightly", feature(min_type_alias_impl_trait))]

mod context;
mod error;
mod generators;
mod protos;
mod utils;
mod wrappers;

use ::puroro::DeserializableFromIter;
use ::puroro::Serializable;
use context::Context;

use error::{ErrorKind, GeneratorError};
type Result<T> = std::result::Result<T, GeneratorError>;

use std::io::Read;
use std::io::{stdin, stdout};

pub use protos::google;
use protos::google::protobuf::compiler::{
    code_generator_response, CodeGeneratorRequest, CodeGeneratorResponse,
};

fn main() -> Result<()> {
    let mut cgreq = CodeGeneratorRequest::default();
    cgreq.deser_from_iter(&mut stdin().bytes()).unwrap();
    let context = Context::new(
        &cgreq,
        context::ImplType::Default,
        context::AllocatorType::Default,
    );
    let filename_and_content = generators::do_generate(&context)?;
    let mut cgres = CodeGeneratorResponse::default();
    cgres.file = filename_and_content
        .into_iter()
        .map(|(filename, content)| {
            let mut file = code_generator_response::File::default();
            file.name = Some(filename);
            file.content = Some(content);
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
