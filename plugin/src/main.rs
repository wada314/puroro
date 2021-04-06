#[macro_use]
mod macros;
mod plugin;
mod traits_generator;
use ::puroro::Result;
use ::puroro::{Deserializable, Serializable};

use std::io::Read;
use std::io::{stdin, stdout};

use plugin::*;

fn main() -> Result<()> {
    let cgreq = CodeGeneratorRequest::from_bytes(stdin().bytes()).unwrap();
    dbg!(&cgreq);
    let mut cgres = CodeGeneratorResponse::default();
    let mut file = CodeGeneratorResponse_File::default();
    file.name = "test.rs".to_string();
    file.content = traits_generator::generate_traits(&cgreq).unwrap();
    cgres.file.push(file);
    cgres.serialize(&mut stdout())?;
    Ok(())
}
