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
use crate::Result;
use ::puroro::google::protobuf;
use ::std::cell::OnceCell;
use ::std::fmt::Debug;

use super::*;

// region: FileDescriptor

#[derive(Debug)]
pub struct FileDescriptorExt<'a> {
    root: &'a RootContext<'a>,
    base: &'a protobuf::FileDescriptorProto,
    cache: FileDescriptorCache<'a>,
}

#[derive(Default, Debug)]
pub struct FileDescriptorCache<'a> {
    dependencies: OnceCell<Vec<&'a FileDescriptorExt<'a>>>,
    messages: OnceCell<Vec<DescriptorExt<'a>>>,
    enums: OnceCell<Vec<EnumDescriptorExt<'a>>>,
    package: OnceCell<Option<ProtoPathBuf>>,
    absolute_package: OnceCell<ProtoPathBuf>,

    field_presence: OnceCell<Option<protobuf::feature_set::FieldPresence>>,
}

impl<'a> FileDescriptorExt<'a> {
    pub fn new(root: &'a RootContext<'a>, base: &'a protobuf::FileDescriptorProto) -> Self {
        Self {
            root,
            base,
            cache: Default::default(),
        }
    }
    pub fn root(&self) -> &'a RootContext<'a> {
        self.root
    }
    pub fn name(&self) -> &str {
        debug_assert!(self.base.name().is_some() && !self.base.name().unwrap().is_empty());
        self.base.name().unwrap_or_default()
    }
    pub fn package(&self) -> Option<&ProtoPath> {
        self.cache
            .package
            .get_or_init(|| self.base.package().map(Into::into))
            .as_deref()
    }
    pub fn absolute_package(&self) -> &ProtoPath {
        self.cache.absolute_package.get_or_init(|| {
            let mut package = self
                .package()
                .map_or_else(ProtoPathBuf::new, |p| p.to_owned());
            if package.is_relative() {
                let mut new_package = Into::<ProtoPathBuf>::into(".");
                new_package.push(&package);
                package = new_package;
            }
            package
        })
    }
    pub fn dependencies(&'a self) -> Result<impl Iterator<Item = &'a FileDescriptorExt<'a>>> {
        Ok(self
            .cache
            .dependencies
            .get_or_try_init(|| {
                self.base
                    .dependency()
                    .map(|name| self.root.file_from_name(name))
                    .collect::<Result<Vec<_>>>()
            })?
            .iter()
            .copied())
    }
    pub fn messages(&'a self) -> impl Iterator<Item = &'a DescriptorExt<'a>> {
        self.cache
            .messages
            .get_or_init(|| {
                self.base
                    .message_type()
                    .map(|m| DescriptorExt::new(self, None, m))
                    .collect()
            })
            .iter()
    }
    pub fn enums(&'a self) -> impl Iterator<Item = &'a EnumDescriptorExt<'a>> {
        self.cache
            .enums
            .get_or_init(|| {
                self.base
                    .enum_type()
                    .map(|e| EnumDescriptorExt::new(self, None, e))
                    .collect()
            })
            .iter()
    }
    pub fn all_messages_or_enums(
        &'a self,
    ) -> impl Iterator<Item = MessageOrEnum<&'a DescriptorExt<'a>, &'a EnumDescriptorExt<'a>>> {
        self.all_messages()
            .map(MessageOrEnum::Message)
            .chain(self.all_enums().map(MessageOrEnum::Enum))
    }
    pub fn all_messages(&'a self) -> impl Iterator<Item = &'a DescriptorExt<'a>> {
        let direct_messages = self.messages();
        let indirect_messages = self
            .messages()
            .flat_map(|child| child.all_descendant_messages())
            .collect::<Vec<_>>();
        Box::new(direct_messages.chain(indirect_messages)) as Box<dyn Iterator<Item = _>>
    }
    pub fn all_enums(&'a self) -> impl Iterator<Item = &'a EnumDescriptorExt<'a>> {
        let direct_enums = self.enums();
        let indirect_enums = self
            .messages()
            .flat_map(|child| child.all_descendant_enums())
            .collect::<Vec<_>>();
        Box::new(direct_enums.chain(indirect_enums)) as Box<dyn Iterator<Item = _>>
    }
    pub fn field_presence(&'a self) -> Option<protobuf::feature_set::FieldPresence> {
        self.cache
            .field_presence
            .get_or_init(|| {
                use protobuf::feature_set::FieldPresence;
                use protobuf::Edition;
                if let Some(options) = self.base.options() {
                    if let Some(features) = options.features() {
                        if let Some(field_presence) = features.field_presence() {
                            return Some(field_presence);
                        }
                    }
                }
                match self.base.edition() {
                    Edition::EditionProto2 => {
                        return Some(FieldPresence::Explicit);
                    }
                    Edition::EditionProto3 => {
                        return Some(FieldPresence::Implicit);
                    }
                    Edition::Edition2023 => {
                        return Some(FieldPresence::Explicit);
                    }
                    _ => (),
                }
                match self.base.syntax() {
                    Some("proto2") => Some(FieldPresence::Explicit),
                    Some("proto3") => Some(FieldPresence::Implicit),
                    _ => None,
                }
            })
            .clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use protobuf::FileDescriptorProto;

    #[test]
    fn test_package_to_files() {
        fn make_fd(name: &str, package: &str) -> FileDescriptorProto {
            let mut fd = FileDescriptorProto::default();
            fd.field_mut(1).push_string(name);
            fd.field_mut(2).push_string(package);
            fd
        }
        let fd0 = make_fd("fd0.proto", "");
        let fd1 = make_fd("fd1.proto", "a");
        let fd2 = make_fd("fd2.proto", "a.b");
        let fd3 = make_fd("fd3.proto", "a.b");
        let fd4 = make_fd("fd4.proto", "a.b.c");
        let root = RootContext::from(vec![fd0, fd1, fd2, fd3, fd4]);

        let root_package_files = root.package_to_files("").unwrap().collect::<Vec<_>>();
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
