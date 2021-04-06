#[macro_use]
mod macros;
mod plugin;
use ::puroro::Deserializable;

use std::io::stdin;
use std::io::Read;

use plugin::*;

fn main() {
    let cgr = CodeGeneratorRequest::from_bytes(stdin().bytes()).unwrap();
    eprintln!("{:?}", cgr);
}
