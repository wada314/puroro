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

use super::{EnumTrait, MessageOrEnum, MessageTrait, PackageTrait, RootPackage};
use crate::codegen::utils::StrExt;
use crate::{ErrorKind, Result};
use ::proc_macro2::TokenStream;
use ::quote::{format_ident, quote};
use ::std::fmt::Debug;
use ::std::rc::Rc;
use ::std::borrow::Cow;

pub(super) trait PackageOrMessageTrait: Debug {
    fn messages(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<dyn MessageTrait>>>>;
    fn enums(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<dyn EnumTrait>>>>;
    fn subpackages(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<dyn PackageTrait>>>>;
    fn root_package(&self) -> Result<Rc<RootPackage>>;
    fn parent(&self) -> Result<Option<Rc<dyn PackageOrMessageTrait>>>;

    fn module_file_path(&self) -> Result<Cow<'_, str>>;
    fn module_file_dir(&self) -> Result<Cow<'_, str>>;

    fn gen_rust_module_path(&self) -> Result<Rc<TokenStream>>;

    fn gen_module_file(&self) -> Result<TokenStream> {
        let header = if self.parent()?.is_some() {
            quote! {
                pub mod _puroro_root {
                    pub use super::super::_puroro_root::*;
                }
            }
        } else {
            quote! {
                /// re-export puroro.
                pub use ::puroro;

                /// re-export the primitive types in puroro namespace.
                /// by using the "*", it can be hidden by the same typename explicitly defined in this file.
                pub use ::puroro::*;

                pub mod _puroro_root {
                    pub use super::*;
                }
            }
        };

        let submodule_decls = self
            .subpackages()?
            .map(|p| {
                let ident =
                    format_ident!("{}", p.name()?.to_lower_snake_case().escape_rust_keywords());
                Ok(quote! {
                    pub mod #ident;
                })
            })
            .collect::<Result<Vec<_>>>()?;
        let struct_decls = self
            .messages()?
            .map(|m| m.gen_struct())
            .collect::<Result<Vec<_>>>()?;
        let enum_decls = self
            .enums()?
            .map(|e| e.gen_enum())
            .collect::<Result<Vec<_>>>()?;
        Ok(quote! {
            #header
            pub mod _puroro {
                pub use ::puroro::*;
            }
            #(#submodule_decls)*
            #(#struct_decls)*
            #(#enum_decls)*
        })
    }

    fn resolve_type_name(
        &self,
        type_name: &str,
    ) -> Result<MessageOrEnum<Rc<dyn MessageTrait>, Rc<dyn EnumTrait>>> {
        if let Some(absolute_path) = type_name.strip_prefix('.') {
            return self.root_package()?.resolve_type_name(absolute_path);
        }

        if let Some((subcomponent, rest)) = type_name.split_once('.') {
            // 2 or more remaining components. The next component is either a message or a package.
            if let Some(m) = self
                .messages()?
                .try_find(|m| -> Result<_> { Ok(m.name()? == subcomponent) })?
            {
                return m.resolve_type_name(rest);
            } else if let Some(p) = self
                .subpackages()?
                .try_find(|p| -> Result<_> { Ok(p.name()? == subcomponent) })?
            {
                return p.resolve_type_name(rest);
            }
        } else {
            // Exactly 1 remaining component. Message or Enum. Return that item.
            if let Some(m) = self
                .messages()?
                .try_find(|m| -> Result<_> { Ok(m.name()? == type_name) })?
            {
                return Ok(MessageOrEnum::Message(m));
            } else if let Some(m) = self
                .enums()?
                .try_find(|e| -> Result<_> { Ok(e.name() == type_name) })?
            {
                return Ok(MessageOrEnum::Enum(m));
            }
        }
        Err(ErrorKind::UnknownTypeName {
            name: type_name.to_string(),
        })?
    }
}
