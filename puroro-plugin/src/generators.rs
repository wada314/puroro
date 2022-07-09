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
use crate::descriptor_ext::{DescriptorExt, EnumDescriptorExt, FileDescriptorExt};
use crate::descriptor_resolver::{DescriptorResolver, PackageContents};
use crate::utils::{
    convert_octal_escape_to_rust_style_escape, get_keyword_safe_ident, to_camel_case, upgrade,
};
use crate::{ErrorKind, Result};
use ::askama::Template;
use ::itertools::Itertools;
use ::std::borrow::Cow;
use ::std::rc::Rc;

#[derive(Template)]
#[template(path = "module.rs.txt")]
pub struct Module {
    pub ident: String,
    pub is_root_package: bool,
    pub full_package: String,
    pub submodules: Vec<Rc<Module>>,
    pub messages: Vec<Rc<Message>>,
    pub enums: Vec<Rc<Enum>>,
}
impl Module {
    pub fn try_new(p: &PackageContents) -> Result<Self> {
        todo!()
    }
}

#[derive(Template)]
#[template(path = "message.rs.txt")]
pub struct Message {
    pub ident: String,
    pub submodule_ident: String,
}

impl Message {
    pub fn try_new(m: &DescriptorExt) -> Result<Self> {
        todo!()
    }
}

#[derive(Template)]
#[template(path = "enum.rs.txt")]
pub struct Enum {
    pub ident: String,
}

impl Enum {
    pub fn try_new(e: &EnumDescriptorExt) -> Result<Self> {
        todo!()
    }
}
