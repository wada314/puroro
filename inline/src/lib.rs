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
use ::std::process::Command;

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

    let protoc_status = Command::new(protoc_bin_path().unwrap()).status().unwrap();
    if !protoc_status.success() {
        panic!("Failed to run `protoc` command.");
    }

    quote! { "Hello!" }.into()
}
