#![cfg_attr(feature = "puroro-nightly", feature(backtrace))]
#![cfg_attr(feature = "puroro-nightly", feature(generic_associated_types))]
#![cfg_attr(feature = "puroro-nightly", feature(min_type_alias_impl_trait))]
#![allow(incomplete_features)]

mod error;
mod protos;
mod utils;
mod wrappers;

use ::puroro::Serializable;
use ::puroro_internal::deser::MergeableMessageFromIter;

use error::GeneratorError;
type Result<T> = std::result::Result<T, GeneratorError>;

use std::io::Read;
use std::io::{stdin, stdout};

pub use protos::simple::google;
use protos::simple::google::protobuf::compiler::code_generator_response::File;
use protos::simple::google::protobuf::compiler::{CodeGeneratorRequest, CodeGeneratorResponse};

use askama::Template;

fn main() -> Result<()> {
    let mut cgreq = CodeGeneratorRequest::default();
    cgreq.merge_from_iter(&mut stdin().bytes()).unwrap();
    let mut cgres = CodeGeneratorResponse::default();
    let mut mod_rs = File::default();
    mod_rs.name = Some("mod.rs".to_string());

    let t = wrappers::ModRs { hogee: "foo" };
    mod_rs.content = Some(t.render().unwrap());

    cgres.file.push(mod_rs);
    cgres.serialize(&mut stdout())?;
    Ok(())
}
