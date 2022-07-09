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

use crate::descriptor_ext::*;
use crate::descriptor_resolver::{DescriptorResolver, PackageContents};
use crate::utils::{get_keyword_safe_ident, to_camel_case};
use crate::Result;
use ::askama::Template;
use ::itertools::Itertools;

#[derive(Template)]
#[template(path = "module.rs.txt")]
pub struct Module {
    pub ident: String,
    pub is_root_package: bool,
    pub full_package: String,
    pub submodules: Vec<Module>,
    pub messages: Vec<Message>,
    pub enums: Vec<Enum>,
}
impl Module {
    pub fn try_new(p: &PackageContents, resolver: &DescriptorResolver) -> Result<Self> {
        let ident = get_keyword_safe_ident(&to_camel_case(&p.name)).into();
        let is_root_package = p.name.is_empty();
        let full_package = p.full_package.clone();
        let subpackages = p
            .subpackages
            .iter()
            .map(|sp| {
                let new_package = if is_root_package {
                    sp.clone()
                } else {
                    format!("{}.{}", full_package, sp)
                };
                resolver.package_contents_or_err(&new_package)
            })
            .collect::<Result<Vec<_>>>()?;
        let submodules = subpackages
            .iter()
            .map(|p| Module::try_new(*p, resolver))
            .collect::<Result<Vec<_>>>()?;
        let messages = p
            .input_files
            .iter()
            .map(|f| f.message_type().iter())
            .flatten()
            .map(|m| Message::try_new(m, resolver))
            .collect::<Result<Vec<_>>>()?;
        let enums = p
            .input_files
            .iter()
            .map(|f| f.enum_type().iter())
            .flatten()
            .map(|e| Enum::try_new(e, resolver))
            .collect::<Result<Vec<_>>>()?;
        Ok(Module {
            ident,
            is_root_package,
            full_package,
            submodules,
            messages,
            enums,
        })
    }
}

#[derive(Template)]
#[template(path = "message.rs.txt")]
pub struct Message {
    pub ident: String,
    pub submodule_ident: String,
}

impl Message {
    #[allow(unused)]
    pub fn try_new(m: &DescriptorExt, resolver: &DescriptorResolver) -> Result<Self> {
        todo!()
    }
}

#[derive(Template)]
#[template(path = "enum.rs.txt")]
pub struct Enum {
    pub ident: String,
}

impl Enum {
    #[allow(unused)]
    pub fn try_new(e: &EnumDescriptorExt, resolver: &DescriptorResolver) -> Result<Self> {
        todo!()
    }
}
