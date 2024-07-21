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

use ::std::path::PathBuf;
use ::syn::{Ident, Path};

use super::gen_enum_items::GenEnumItems;
use super::gen_message_items::GenMessageItems;
use crate::descriptor::RootContext;
use crate::proto_path::ProtoPath;
use crate::Result;

pub struct Module {
    rust_name: Option<Ident>,
    rust_full_path: Path,
    file_path: PathBuf,
    submodules: Vec<Module>,
    gen_enums: Vec<GenEnumItems>,
    gen_messages: Vec<GenMessageItems>,
}
impl Module {
    pub fn try_new<'a>(package: &ProtoPath, desc_root: &'a RootContext<'a>) -> Result<Self> {
        todo!()
    }
}
