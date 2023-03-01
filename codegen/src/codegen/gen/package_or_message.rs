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

use super::super::util::*;
use super::{
    PackageOrMessage, PURORO_INTERNAL_IDENT, PURORO_LIB_IDENT, PURORO_ROOT, PURORO_ROOT_IDENT,
    SUBMODULE_HEADER,
};
use crate::syn::{parse2, parse_str, File, Item, Path};
use crate::Result;
use ::itertools::Itertools;
use ::once_cell::unsync::OnceCell;
use ::proc_macro2::TokenStream;
use ::quote::{format_ident, quote};
use ::std::fmt::Debug;
use ::std::rc::Rc;

pub(crate) trait PackageOrMessageExt {
    fn module_name(&self) -> Result<&str>;
    fn module_file_path(
        &self,
        root_module_name: Option<&str>,
        root_file_name: Option<&str>,
    ) -> Result<&str>;
    fn module_file_dir(&self, root_module_name: Option<&str>) -> Result<&str>;
    fn gen_rust_module_path(&self) -> Result<Rc<Path>>;
    fn gen_module_file(&self, puroro_library_path: Option<&str>) -> Result<File>;
    fn gen_inline_code(&self, puroro_library_path: Option<&str>) -> Result<TokenStream>;
}

#[derive(Debug, Default)]
struct Cache {
    module_file_dir: OnceCell<String>,
    rust_module_path: OnceCell<Rc<Path>>,
    module_name: OnceCell<String>,
    module_file_path: OnceCell<String>,
}

impl<T: ?Sized + PackageOrMessage> PackageOrMessageExt for T {
    fn module_name(&self) -> Result<&str> {
        self.cache()
            .get::<Cache>()?
            .module_name
            .get_or_try_init(|| {
                Ok(if self.is_root()? {
                    "".to_string()
                } else {
                    self.name()?
                        .to_lower_snake_case()
                        .escape_rust_keywords()
                        .to_string()
                })
            })
            .map(|s| s.as_str())
    }
    fn module_file_path(
        &self,
        root_module_name: Option<&str>,
        root_file_name: Option<&str>,
    ) -> Result<&str> {
        self.cache()
            .get::<Cache>()?
            .module_file_path
            .get_or_try_init(|| {
                Ok(if let Some(parent) = self.parent()? {
                    format!(
                        "{}{}.rs",
                        parent.module_file_dir(root_module_name)?,
                        self.name()?.to_lower_snake_case()
                    )
                } else {
                    if let Some(root_module_name) = root_module_name {
                        format!("{}.rs", root_module_name)
                    } else {
                        root_file_name.unwrap_or("lib.rs").to_string()
                    }
                })
            })
            .map(|s| s.as_str())
    }

    fn module_file_dir(&self, root_module_name: Option<&str>) -> Result<&str> {
        self.cache()
            .get::<Cache>()?
            .module_file_dir
            .get_or_try_init(|| {
                if let Some(parent) = self.parent()? {
                    Ok(format!(
                        "{}{}/",
                        parent.module_file_dir(root_module_name)?,
                        self.name()?.to_lower_snake_case()
                    ))
                } else {
                    if let Some(root_module_name) = root_module_name {
                        Ok(root_module_name.to_string())
                    } else {
                        Ok("".to_string())
                    }
                }
            })
            .map(|s| s.as_str())
    }

    fn gen_rust_module_path(&self) -> Result<Rc<Path>> {
        self.cache()
            .get::<Cache>()?
            .rust_module_path
            .get_or_try_init(|| {
                let ts = if let Some(parent) = self.parent()? {
                    let parent_path = parent.gen_rust_module_path()?;
                    let ident = format_ident!(
                        "{}",
                        self.name()?.to_lower_snake_case().escape_rust_keywords()
                    );
                    quote! { #parent_path :: #ident }
                } else {
                    quote! { #PURORO_ROOT }
                };
                Ok(Rc::new(parse2(ts)?))
            })
            .cloned()
    }

    fn gen_module_file(&self, puroro_library_path: Option<&str>) -> Result<File> {
        let puroro_library_path: Path = if let Some(puroro_library_path) = puroro_library_path {
            parse_str(puroro_library_path)?
        } else {
            parse2(quote! { ::puroro })?
        };
        let header = if self.is_root()? {
            quote! {
                mod #PURORO_ROOT_IDENT {
                    #[allow(unused)]
                    pub(crate) use super::*;
                }
                mod #PURORO_LIB_IDENT {
                    #[allow(unused)]
                    pub use #puroro_library_path::*;
                }
                mod #PURORO_INTERNAL_IDENT {
                    #[allow(unused)]
                    pub(crate) use #puroro_library_path::internal::*;
                }
                /// Re-exporting puroro
                pub mod puroro {
                    pub use #puroro_library_path::*;
                }
            }
        } else {
            quote! { #SUBMODULE_HEADER }
        };

        let submodule_sources = self
            .subpackages()?
            .map(|p| Ok(Rc::clone(p) as Rc<dyn PackageOrMessage>))
            .chain(
                self.messages()?
                    .filter_map(|m| match m.should_generate_module_file() {
                        Ok(true) => Some(Ok(Rc::clone(m) as Rc<dyn PackageOrMessage>)),
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

        let content_items = gen_messages_enums_oneofs_in_module(self)?;
        Ok(parse2(quote! {
            #header
            #(pub mod #submodule_idents;)*
            #(#content_items)*
        })?)
    }

    fn gen_inline_code(&self, puroro_library_path: Option<&str>) -> Result<TokenStream> {
        let submodule_sources = self
            .subpackages()?
            .map(|p| Ok(Rc::clone(p) as Rc<dyn PackageOrMessage>))
            .chain(
                self.messages()?
                    .filter_map(|m| match m.should_generate_module_file() {
                        Ok(true) => Some(Ok(Rc::clone(m) as Rc<dyn PackageOrMessage>)),
                        Ok(false) => None,
                        Err(e) => Some(Err(e)),
                    }),
            );
        let submodule_names_and_contents = {
            let mut unsorted = submodule_sources
                .map(|rp| {
                    let p = rp?;
                    let name = format_ident!("{}", p.module_name()?);
                    let contents = p.gen_inline_code(puroro_library_path)?;
                    Ok((name, contents))
                })
                .collect::<Result<Vec<_>>>()?;
            unsorted.sort_by_key(|(name, _)| name.clone());
            unsorted
        };
        let submodule_idents = submodule_names_and_contents
            .iter()
            .map(|(name, _)| name.clone())
            .collect::<Vec<_>>();
        let submodule_contents = submodule_names_and_contents
            .into_iter()
            .map(|(_, contents)| contents)
            .collect::<Vec<_>>();

        let content_items = gen_messages_enums_oneofs_in_module(self)?;

        let puroro_library_path: Path = if let Some(puroro_library_path) = puroro_library_path {
            parse_str(puroro_library_path)?
        } else {
            parse2(quote! { ::puroro })?
        };
        let header = if self.is_root()? {
            quote! {
                mod #PURORO_ROOT_IDENT {
                    #[allow(unused)]
                    pub(crate) use super::*;
                }
                mod #PURORO_LIB_IDENT {
                    #[allow(unused)]
                    pub use #puroro_library_path::*;
                }
                mod #PURORO_INTERNAL_IDENT {
                    #[allow(unused)]
                    pub(crate) use #puroro_library_path::internal::*;
                }
                /// Re-exporting puroro
                pub mod puroro {
                    pub use #puroro_library_path::*;
                }
            }
        } else {
            quote! { #SUBMODULE_HEADER }
        };

        Ok(quote! {
            #header
            #(pub mod #submodule_idents {
                #submodule_contents
            })*
            #(#content_items)*
        })
    }
}

fn gen_messages_enums_oneofs_in_module(
    this: &(impl ?Sized + PackageOrMessage),
) -> Result<Vec<Item>> {
    let message_struct_items = this
        .messages()?
        .map(|m| Ok(m.gen_message_struct_items()?.into_iter()))
        .flatten_ok();
    let enum_items = this
        .enums()?
        .map(|e| Ok(e.gen_enum_items()?.into_iter()))
        .flatten_ok();
    let oneof_union_items = this
        .oneofs()?
        .map(|o| Ok(o.gen_oneof_union_items()?.into_iter()))
        .flatten_ok();

    // Message view structs are imported into separte module to avoid name conflict.
    let view_struct_in_module_items = this
        .messages()?
        .map(|m| Ok(m.gen_view_struct_items()?.into_iter()))
        .flatten_ok()
        .collect::<Result<Vec<_>>>()?;
    let view_struct_items = (!view_struct_in_module_items.is_empty())
        .then(|| -> Result<_> {
            Ok([
                Ok(parse2::<Item>(quote! {
                    #[doc(hidden)]
                    pub mod _view {
                        #SUBMODULE_HEADER
                        #(#view_struct_in_module_items)*
                    }
                })?),
                Ok(parse2::<Item>(quote! {
                    #[doc(inline)]
                    pub use self::_view::*;
                })?),
            ]
            .into_iter())
        })
        .transpose()?
        .into_iter()
        .flatten();

    // Message fields structs are imported into separte module to avoid name conflict.
    let fields_struct_in_module_items = this
        .messages()?
        .map(|m| Ok(m.gen_fields_struct_items()?.into_iter()))
        .flatten_ok()
        .collect::<Result<Vec<_>>>()?;
    let fields_struct_items = (!fields_struct_in_module_items.is_empty())
        .then(|| -> Result<_> {
            Ok([
                Ok(parse2::<Item>(quote! {
                    #[doc(hidden)]
                    pub mod _fields {
                        #SUBMODULE_HEADER
                        #(#fields_struct_in_module_items)*
                    }
                })?),
                Ok(parse2::<Item>(quote! {
                    #[doc(hidden)]
                    pub use self::_fields::*;
                })?),
            ]
            .into_iter())
        })
        .transpose()?
        .into_iter()
        .flatten();

    // Oneof cases are imported into separte module to avoid name conflict.
    let oneof_case_in_module_items = this
        .oneofs()?
        .map(|o| Ok(o.gen_oneof_case_items()?.into_iter()))
        .flatten_ok()
        .collect::<Result<Vec<_>>>()?;
    let oneof_case_items = (!oneof_case_in_module_items.is_empty())
        .then(|| -> Result<_> {
            Ok([
                Ok(parse2::<Item>(quote! {
                    pub mod _case {
                        #SUBMODULE_HEADER
                        #(#oneof_case_in_module_items)*
                    }
                })?),
                Ok(parse2::<Item>(quote! {
                    pub use self::_case::*;
                })?),
            ]
            .into_iter())
        })
        .transpose()?
        .into_iter()
        .flatten();

    message_struct_items
        .chain(view_struct_items)
        .chain(fields_struct_items)
        .chain(enum_items)
        .chain(oneof_union_items)
        .chain(oneof_case_items)
        .collect::<Result<Vec<_>>>()
}
