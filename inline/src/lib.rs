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

#![feature(proc_macro_span)]

use ::litrs::StringLit;
use ::proc_macro::TokenStream as TokenStream1;
use ::proc_macro2::{LineColumn, Span, TokenStream};
use ::protoc_bin_vendored::protoc_bin_path;
use ::puroro_codegen::puroro::Message;
use ::puroro_codegen::{
    generate_tokens_for_inline, CodegenOptions, FileDescriptorSet, GeneratorError,
};
use ::quote::{format_ident, quote};
use ::std::fs::File;
use ::std::io::{Read, Write};
use ::std::process::Command;
use ::tempdir::TempDir;

// I really want protoc to support stdin / stdout handling:
// https://github.com/protocolbuffers/protobuf/issues/4163

#[proc_macro]
pub fn puroro_inline(input: TokenStream1) -> TokenStream1 {
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

    gen_code(string_lit.value()).into()
}

#[proc_macro]
pub fn puroro_inline2(input: TokenStream1) -> TokenStream1 {
    gen_code(&input.to_string()).into()
}

fn gen_code(input_str: &str) -> TokenStream {
    let temp_dir = TempDir::new("puroro-inline").unwrap();
    let input_file_path = temp_dir.path().join("input.proto");
    let output_file_path = temp_dir.path().join("output.pb");

    {
        let mut f = File::create(&input_file_path).unwrap();
        f.write_all(input_str.as_bytes()).unwrap();
    }

    let status = Command::new(protoc_bin_path().unwrap())
        .arg(&input_file_path)
        .arg("--proto_path")
        .arg(temp_dir.path())
        .arg("--descriptor_set_out")
        .arg(&output_file_path)
        .arg("--experimental_allow_proto3_optional")
        .status()
        .unwrap();

    if !status.success() {
        panic!("protoc command failed.");
    }

    let fd_set = {
        let f = File::open(&output_file_path).unwrap();
        FileDescriptorSet::from_bytes_iter(f.bytes()).unwrap()
    };

    let options = CodegenOptions::default();

    let main_code = match generate_tokens_for_inline(fd_set.file().into_iter(), &options) {
        Ok(main_code) => main_code,
        Err(GeneratorError::FatalError { kind, backtrace }) => {
            let message = format!("{}\n{}", kind.to_string(), &backtrace);
            return quote! { compile_error!(#message); }.into();
        }
        Err(GeneratorError::RecoverableError { kind }) => {
            let message = format!("{}", kind.to_string());
            return quote! { compile_error!(#message); }.into();
        }
    };

    let LineColumn { line, column } = Span::call_site().start();
    let wrapping_mod = format_ident!("_puroro_inline_{}_{}", line, column);

    quote! {
        mod #wrapping_mod {
            #main_code
        }
        use self::#wrapping_mod::*;
    }
    .into()
}
