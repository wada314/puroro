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

use super::{Enum, Message, MessageOrEnum, Package, RootPackage};
use crate::{ErrorKind, Result};
use ::itertools::Itertools;
use ::proc_macro2::TokenStream;
use ::quote::{format_ident, quote};
use ::std::borrow::Cow;
use ::std::fmt::Debug;
use ::std::rc::Rc;

pub(super) trait PackageOrMessage: Debug {
    fn messages(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<dyn Message>>>>;
    fn enums(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<dyn Enum>>>>;
    fn subpackages(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<dyn Package>>>>;
    fn root_package(&self) -> Result<Rc<RootPackage>>;
    fn parent(&self) -> Result<Option<Rc<dyn PackageOrMessage>>>;

    fn all_messages(&self) -> Result<Vec<Rc<dyn Message>>> {
        let mut ret = self.messages()?.collect::<Vec<_>>();
        let messages_iter = self.messages()?.map(|m| m as Rc<dyn PackageOrMessage>);
        let packages_iter = self.subpackages()?.map(|p| p as Rc<dyn PackageOrMessage>);
        let mut stack = messages_iter.chain(packages_iter).collect::<Vec<_>>();
        while let Some(p) = stack.pop() {
            stack.extend(p.messages()?.map(|m| m as Rc<dyn PackageOrMessage>));
            stack.extend(p.subpackages()?.map(|p| p as Rc<dyn PackageOrMessage>));
            ret.extend(p.messages()?);
        }
        Ok(ret)
    }

    fn module_name(&self) -> Result<Cow<'_, str>>;
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

        let submodule_sources = self
            .subpackages()?
            .map(|p| Ok(p as Rc<dyn PackageOrMessage>))
            .chain(
                self.messages()?
                    .filter_map(|m| match m.should_generate_module_file() {
                        Ok(true) => Some(Ok(m as Rc<dyn PackageOrMessage>)),
                        Ok(false) => None,
                        Err(e) => Some(Err(e)),
                    }),
            );
        let submodule_idents = {
            let mut unsorted = submodule_sources
                .map(|p| Ok(format_ident!("{}", p?.module_name()?)))
                .collect::<Result<Vec<_>>>()?;
            unsorted.sort();
            unsorted
        };
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
            #(pub mod #submodule_idents;)*
            #(#struct_decls)*
            #(#enum_decls)*
        })
    }

    fn resolve_type_name(
        &self,
        type_name: &str,
    ) -> Result<MessageOrEnum<Rc<dyn Message>, Rc<dyn Enum>>> {
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
