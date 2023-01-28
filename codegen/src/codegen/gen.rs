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

mod r#enum;
mod field;
mod field_base;
mod field_or_oneof;
mod field_type;
mod message;
mod oneof;
mod oneof_field;
mod package_or_message;

use self::field_base::FieldBaseExt;
use self::field_or_oneof::FieldOrOneofExt;
pub use self::package_or_message::PackageOrMessageExt;

use super::data::*;

// Global generator constants
// Q: Is there any good way to implement this using something like `Lazy` generic type?
// I couldn't find a good way for this case, where the target type
// like `Ident` or `Path` are `!Sync`...
//
// e.g. This does not compile:
// ```
// use once_cell::sync::Lazy;
// static FOO: Lazy<Ident> = Lazy::new(|| format_ident!("foo"));
// ```

use ::quote::{format_ident, quote, ToTokens};

macro_rules! gen_global_constants {
    () => {};
    (const $id:ident: $ty:ident = $expr:expr ; $($rest:tt)*) => {
        const $id: $ty = $ty;
        struct $ty;
        impl ToTokens for $ty {
            fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                ($expr).to_tokens(tokens)
            }
        }
        gen_global_constants!($($rest)*);
    };
}

gen_global_constants! {
    const PURORO_INTERNAL_IDENT: PuroroInternalIdent = format_ident!("_pinternal");
    const PURORO_INTERNAL: PuroroInternal = quote! { self :: #PURORO_INTERNAL_IDENT };
    const PURORO_ROOT_IDENT: PuroroRootIdent = format_ident!("_root");
    const PURORO_ROOT: PuroroRoot = quote! { self :: #PURORO_ROOT_IDENT };
    const PURORO_LIB_IDENT: PuroroLibIdent = format_ident!("_puroro");
    const PURORO_LIB: PuroroLib = quote! { self :: #PURORO_LIB_IDENT };
    const SUBMODULE_HEADER: SubmoduleHeader = quote! {
        mod #PURORO_ROOT_IDENT {
            #[allow(unused)]
            pub use super::super::#PURORO_ROOT_IDENT::*;
        }
        mod #PURORO_LIB_IDENT {
            #[allow(unused)]
            pub use ::puroro::*;
        }
        mod #PURORO_INTERNAL_IDENT {
            #[allow(unused)]
            pub use ::puroro::internal::*;
        }
    };
}
