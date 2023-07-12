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

use ::proc_macro::TokenStream;
use ::quote::{quote, quote_spanned};
use ::syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Message)]
pub fn derive_puroro_message(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ::syn::Data::Struct(data_struct) = &input.data else {
        return quote! {
            compile_error!("#[derive(Message)] must be attached to struct");
        }.into()
    };
    quote! {
        #input
    }
    .into()
}
