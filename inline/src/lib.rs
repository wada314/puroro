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

use ::litrs::StringLit;
use ::proc_macro::TokenStream;
use ::protoc_bin_vendored::protoc_bin_path;
use ::puroro_codegen::generate_tokens_for_inline;
use ::quote::quote;
use ::std::process::{Command, Stdio};
use std::io::Write;

// We need protoc to support stdin / stdout handling:
// https://github.com/protocolbuffers/protobuf/issues/4163

#[proc_macro]
pub fn inline(input: TokenStream) -> TokenStream {
    let mut input_iter = input.into_iter();
    let Some(token) = input_iter.next() else {
        panic!("No token inside the puroro inline macro.");
    };
    if input_iter.next().is_some() {
        panic!("There should be only 1 token inside the puroro inline macro.");
    }

    let string_lit = match StringLit::try_from(&token) {
        Ok(lit) => lit,
        Err(e) => return e.to_compile_error(),
    };

    let mut child = Command::new(protoc_bin_path().unwrap())
        .arg("/dev/stdin")
        .arg("--experimental_allow_proto3_optional")
        .arg("--descriptor_set_out=CON")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(string_lit.value().as_bytes()).unwrap();
    }

    let output = child.wait_with_output().unwrap();
    let string = format!("{:?}", &output.stdout);
    quote! { #string }.into()
}
