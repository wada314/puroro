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

use crate::proto_path::{ProtoPath, ProtoPathBuf};
use crate::{ErrorKind, Result};
use ::itertools::Itertools;
use ::puroro::google::protobuf::{Edition as EditionProto, FileDescriptorProto};
use ::puroro::Result as PResult;
use ::std::cell::OnceCell;
use ::std::fmt::Debug;

use super::*;

// region: FileDescriptor

#[derive(Debug, Clone)]
pub struct FileDescriptorBase {
    name: String,
    dependencies: Vec<String>,
    package: Option<ProtoPathBuf>,
    message_types: Vec<DescriptorBase>,
    enum_types: Vec<EnumDescriptorBase>,
    #[allow(unused)]
    syntax: Option<String>,
    #[allow(unused)]
    edition: Option<Edition>,
}

impl<'a> TryFrom<FileDescriptorProto<'a>> for FileDescriptorBase {
    type Error = ErrorKind;
    fn try_from(proto: FileDescriptorProto) -> Result<Self> {
        Ok(Self {
            name: proto.name()?.try_into_string("No FileDescriptor name")?,
            dependencies: proto
                .dependency()
                .into_iter()
                .map_ok(str::to_string)
                .collect::<PResult<_>>()?,
            package: proto.package()?.map(|s| s.to_string().into()),
            message_types: proto
                .message_type()
                .into_iter()
                .map_ok(DescriptorBase::try_from)
                .collect::<PResult<Result<Vec<_>>>>()??,
            enum_types: proto
                .enum_type()
                .into_iter()
                .map_ok(EnumDescriptorBase::try_from)
                .collect::<PResult<Result<Vec<_>>>>()??,
            syntax: proto.syntax()?.map(str::to_string),
            edition: proto.edition()?.map(EditionProto::try_into).transpose()?,
        })
    }
}

#[cfg(test)]
#[derive(Default)]
pub struct DebugFileDescriptor<'a> {
    pub name: &'a str,
    pub package: Option<&'a str>,
    pub message_types: Vec<DebugDescriptor<'a>>,
    pub enum_types: Vec<DebugEnumDescriptor<'a>>,
}

#[cfg(test)]
impl From<DebugFileDescriptor<'_>> for FileDescriptorBase {
    fn from(debug: DebugFileDescriptor) -> Self {
        Self {
            name: debug.name.to_string(),
            dependencies: vec![],
            package: debug.package.map(ProtoPathBuf::from),
            message_types: debug.message_types.into_iter().map(Into::into).collect(),
            enum_types: debug.enum_types.into_iter().map(Into::into).collect(),
            syntax: None,
            edition: None,
        }
    }
}

#[derive(Debug)]
pub struct FileDescriptor<'a> {
    root: &'a RootContext<'a>,
    base: &'a FileDescriptorBase,
    cache: FileDescriptorCache<'a>,
}

#[derive(Default, Debug)]
pub struct FileDescriptorCache<'a> {
    dependencies: OnceCell<Vec<&'a FileDescriptor<'a>>>,
    messages: OnceCell<Vec<Descriptor<'a>>>,
    enums: OnceCell<Vec<EnumDescriptor<'a>>>,
}

impl<'a> FileDescriptor<'a> {
    pub fn new(root: &'a RootContext<'a>, base: &'a FileDescriptorBase) -> Self {
        Self {
            root,
            base,
            cache: Default::default(),
        }
    }
    pub fn root(&self) -> &'a RootContext {
        self.root
    }
    pub fn name(&self) -> &str {
        &self.base.name
    }
    pub fn package(&self) -> Option<&ProtoPath> {
        self.base.package.as_deref()
    }
    pub fn absolute_package(&self) -> Result<ProtoPathBuf> {
        let mut package = self
            .package()
            .map_or_else(ProtoPathBuf::new, |p| p.to_owned());
        if package.is_relative() {
            let mut new_package = Into::<ProtoPathBuf>::into(".");
            new_package.push(&package);
            package = new_package;
        }
        Ok(package)
    }
    pub fn dependencies(&'a self) -> Result<impl Iterator<Item = &FileDescriptor>> {
        self.cache
            .dependencies
            .get_or_try_init(|| {
                Ok(self
                    .base
                    .dependencies
                    .iter()
                    .map(|name| self.root.file_from_name(name))
                    .collect::<Result<Vec<_>>>()?)
            })
            .map(|v| v.into_iter().map(|f| *f))
    }
    pub fn messages(&'a self) -> Result<impl Iterator<Item = &Descriptor>> {
        Ok(self
            .cache
            .messages
            .get_or_init(|| {
                self.base
                    .message_types
                    .iter()
                    .map(|m| Descriptor::new(self, None, m))
                    .collect()
            })
            .iter())
    }
    pub fn enums(&'a self) -> Result<impl Iterator<Item = &EnumDescriptor>> {
        Ok(self
            .cache
            .enums
            .get_or_init(|| {
                self.base
                    .enum_types
                    .iter()
                    .map(|e| EnumDescriptor::new(self, None, e))
                    .collect()
            })
            .iter())
    }
    pub fn all_messages_or_enums(
        &'a self,
    ) -> Result<impl Iterator<Item = MessageOrEnum<&Descriptor, &EnumDescriptor>>> {
        Ok(self
            .all_messages()?
            .into_iter()
            .map(MessageOrEnum::Message)
            .chain(self.all_enums()?.into_iter().map(MessageOrEnum::Enum)))
    }
    pub fn all_messages(&'a self) -> Result<impl Iterator<Item = &Descriptor>> {
        let direct_messages = self.messages()?.into_iter();
        let indirect_messages_vec = self
            .messages()?
            .into_iter()
            .map(|child| child.all_messages())
            .collect::<Result<Vec<_>>>()?;
        let indirect_messages = indirect_messages_vec
            .into_iter()
            .flat_map(|v| v.into_iter());
        let boxed: Box<dyn Iterator<Item = _>> = Box::new(direct_messages.chain(indirect_messages));
        Ok(boxed)
    }
    pub fn all_enums(&'a self) -> Result<impl Iterator<Item = &EnumDescriptor>> {
        let direct_enums = self.enums()?.into_iter();
        let indirect_enums_vec = self
            .messages()?
            .into_iter()
            .map(|child| child.all_enums())
            .collect::<Result<Vec<_>>>()?;
        let indirect_enums = indirect_enums_vec.into_iter().flat_map(|v| v.into_iter());
        let boxed: Box<dyn Iterator<Item = _>> = Box::new(direct_enums.chain(indirect_enums));
        Ok(boxed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FD_DEFAULT: FileDescriptorBase = FileDescriptorBase {
        name: String::new(),
        dependencies: vec![],
        package: None,
        message_types: vec![],
        enum_types: vec![],
        syntax: None,
        edition: None,
    };

    #[test]
    fn test_package_to_files() {
        fn make_fd(name: &str, package: &str) -> FileDescriptorBase {
            FileDescriptorBase {
                name: name.to_string(),
                package: Some(package.into()),
                ..FD_DEFAULT
            }
        }
        let fd0 = make_fd("fd0.proto", "");
        let fd1 = make_fd("fd1.proto", "a");
        let fd2 = make_fd("fd2.proto", "a.b");
        let fd3 = make_fd("fd3.proto", "a.b");
        let fd4 = make_fd("fd4.proto", "a.b.c");
        let root = RootContext::from(vec![fd0, fd1, fd2, fd3, fd4]);

        let root_package_files = root.package_to_files("").unwrap().into_iter().collect_vec();
        let package_a_files = root
            .package_to_files("a")
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>();
        let package_a_b_files = root
            .package_to_files("a.b")
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>();
        let package_a_b_c_files = root
            .package_to_files("a.b.c")
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>();

        assert_eq!(1, root_package_files.len());
        assert_eq!(1, package_a_files.len());
        assert_eq!(2, package_a_b_files.len());
        assert_eq!(1, package_a_b_c_files.len());
        assert!(root_package_files.iter().any(|f| f.name() == "fd0.proto"));
        assert!(package_a_files.iter().any(|f| f.name() == "fd1.proto"));
        assert!(package_a_b_files.iter().any(|f| f.name() == "fd2.proto"));
        assert!(package_a_b_files.iter().any(|f| f.name() == "fd3.proto"));
        assert!(package_a_b_c_files.iter().any(|f| f.name() == "fd4.proto"));
    }
}
