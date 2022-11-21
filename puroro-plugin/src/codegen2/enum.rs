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

use super::util::WeakExt;
use super::{InputFileTrait, MessageTrait, PackageOrMessage, PackageTrait};
use crate::codegen::utils::StrExt;
use crate::Result;
use ::once_cell::unsync::OnceCell;
use ::puroro_protobuf_compiled::google::protobuf::EnumDescriptorProto;
use ::std::fmt::Debug;
use ::std::rc::{Rc, Weak};

pub(super) trait EnumTrait: Debug {
    fn name(&self) -> &str;
    fn rust_struct_path(&self) -> Result<&str>;
}

#[derive(Debug)]
pub(super) struct Enum {
    name: String,
    input_file: Weak<dyn InputFileTrait>,
    parent: PackageOrMessage<Weak<dyn PackageTrait>, Weak<dyn MessageTrait>>,
    rust_struct_path: OnceCell<String>,
}

impl Enum {
    pub(super) fn new(
        proto: &EnumDescriptorProto,
        input_file: Weak<dyn InputFileTrait>,
        parent: PackageOrMessage<Weak<dyn PackageTrait>, Weak<dyn MessageTrait>>,
    ) -> Rc<Self> {
        Rc::new(Enum {
            name: proto.name().to_string(),
            input_file,
            parent,
            rust_struct_path: OnceCell::new(),
        })
    }

    fn parent(&self) -> Result<PackageOrMessage<Rc<dyn PackageTrait>, Rc<dyn MessageTrait>>> {
        Ok(match &self.parent {
            PackageOrMessage::Package(p) => PackageOrMessage::Package(p.try_upgrade()?),
            PackageOrMessage::Message(m) => PackageOrMessage::Message(m.try_upgrade()?),
        })
    }
}

impl EnumTrait for Enum {
    fn name(&self) -> &str {
        &self.name
    }

    fn rust_struct_path(&self) -> Result<&str> {
        self.rust_struct_path
            .get_or_try_init(|| {
                Ok(format!(
                    "{}::{}",
                    self.parent()?.rust_module_path()?,
                    self.name().to_camel_case().escape_rust_keywords(),
                ))
            })
            .map(|s| s.as_str())
    }
}

#[cfg(test)]
pub struct EnumFake;

#[cfg(test)]
impl EnumFake {
    fn try_new(proto: &EnumDescriptorProto) -> Result<Self> {
        Ok(EnumFake)
    }
}
