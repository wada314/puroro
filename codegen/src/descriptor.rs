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

//! A wrapped descriptor protos. Compared to the original descriptor protos,
//! this module provides:
//!
//! - A non-tree data link traversing: Specifically, the field's type can be
//!   resolved to the message or enum type.
//! - Some field's `Option<T>` will be unwrapped if it's a trivial information.
//! - Renaming some fields. Especially, the repeated fields are renamed to plural.

mod r#enum;
mod features;
mod field;
mod file;
mod message;
mod subtypes;
pub use features::*;
pub use field::*;
pub use file::*;
pub use message::*;
pub use r#enum::*;
pub use subtypes::*;

use crate::proto_path::{ProtoPath, ProtoPathBuf};
use crate::Result;
use ::puroro::google::protobuf;
use ::std::cell::OnceCell;
use ::std::collections::HashMap;
use ::std::fmt::Debug;

#[derive(Debug)]
pub struct RootContext<'a> {
    files: Vec<(
        protobuf::FileDescriptorProto,
        OnceCell<FileDescriptorExt<'a>>,
    )>,
    cache: RootContextCache<'a>,
}

#[derive(Default, Debug)]
struct RootContextCache<'a> {
    package_to_files: OnceCell<HashMap<ProtoPathBuf, Vec<&'a FileDescriptorExt<'a>>>>,
}

impl<T: IntoIterator<Item = protobuf::FileDescriptorProto>> From<T> for RootContext<'_> {
    fn from(files: T) -> Self {
        Self {
            files: files
                .into_iter()
                .map(|f| (f, OnceCell::default()))
                .collect(),
            cache: Default::default(),
        }
    }
}
impl<'a> RootContext<'a> {
    pub fn files(&'a self) -> impl Iterator<Item = &'a FileDescriptorExt<'a>> {
        self.files
            .iter()
            .map(|(f, c)| c.get_or_init(|| FileDescriptorExt::new(self, f)))
    }
    pub fn file_from_name(&'a self, name: &str) -> Result<&'a FileDescriptorExt<'a>> {
        Ok(self
            .files()
            .into_iter()
            .find(|f| f.name() == name)
            .ok_or_else(|| format!("No such file: {}", name))?)
    }
    pub fn package_to_files(
        &'a self,
        package: impl AsRef<ProtoPath>,
    ) -> Result<impl Iterator<Item = &'a FileDescriptorExt<'a>>> {
        let package = if package.as_ref().is_relative() {
            // This method is a root method, so the relative path should be converted
            // to the absolute path by just adding '.' at the beginning.
            format!(".{}", package.as_ref()).into()
        } else {
            package.as_ref().to_owned()
        };
        debug_assert!(package.is_absolute());
        let map = self
            .cache
            .package_to_files
            .get_or_try_init(|| -> Result<_> {
                let mut map = HashMap::new();
                for fd in self.files() {
                    let package = fd.absolute_package().to_owned();
                    map.entry(package.clone()).or_insert_with(Vec::new).push(fd);
                }
                Ok(map)
            })?;
        Ok(map
            .get(&package)
            .map_or(Default::default(), Vec::as_slice)
            .into_iter()
            .map(|f| *f))
    }
    pub fn resolve_path(
        &'a self,
        path: impl AsRef<ProtoPath>,
    ) -> Result<MessageOrEnum<&'a DescriptorExt<'a>, &'a EnumDescriptorExt<'a>>> {
        let path = path.as_ref();
        let path = if path.is_relative() {
            ProtoPathBuf::from(format!(".{}", path))
        } else {
            path.to_owned()
        };
        return Ok(self
            .resolve_absolute_path(&path)?
            .ok_or_else(|| format!("Path not found: {}", path))?);
    }
    pub fn resolve_relative_path(
        &'a self,
        path: impl AsRef<ProtoPath>,
        cur: impl AsRef<ProtoPath>,
    ) -> Result<MessageOrEnum<&'a DescriptorExt<'a>, &'a EnumDescriptorExt<'a>>> {
        let path = path.as_ref();
        let cur = cur.as_ref();
        if path.is_absolute() {
            return self.resolve_path(path);
        }
        if !cur.is_absolute() {
            Err(format!("Path not found: \"{cur}\" / \"{path}\""))?;
        }
        let cur = cur.to_owned();
        for cur2 in cur.ancestors() {
            let full_path = {
                let mut full_path = cur2.to_owned();
                full_path.push(path);
                full_path
            };
            if let Some(result) = self.resolve_absolute_path(&full_path)? {
                return Ok(result);
            }
        }
        Err(format!("Path not found: \"{cur}\" / \"{path}\""))?
    }

    fn resolve_absolute_path(
        &'a self,
        path: &ProtoPath,
    ) -> Result<Option<MessageOrEnum<&'a DescriptorExt<'a>, &'a EnumDescriptorExt<'a>>>> {
        debug_assert!(path.is_absolute());
        // Can improve the complexity here. Maybe later.
        for package_path in path.ancestors() {
            for file in self.package_to_files(package_path)? {
                for message_or_enum in file.all_messages_or_enums() {
                    if message_or_enum.full_path() == path {
                        return Ok(Some(message_or_enum));
                    }
                }
            }
        }
        Ok(None)
    }
}

#[derive(Debug, Clone)]
pub enum MessageOrEnum<M, E> {
    Message(M),
    Enum(E),
}
impl<'a> MessageOrEnum<&'a DescriptorExt<'a>, &'a EnumDescriptorExt<'a>> {
    pub fn full_path(&self) -> &ProtoPath {
        match self {
            MessageOrEnum::Message(m) => m.full_path(),
            MessageOrEnum::Enum(e) => e.full_path(),
        }
    }
}
impl<M, E> MessageOrEnum<M, E> {
    pub fn maybe_message(self) -> Option<M> {
        match self {
            MessageOrEnum::Message(m) => Some(m),
            MessageOrEnum::Enum(_) => None,
        }
    }
    pub fn maybe_enum(self) -> Option<E> {
        match self {
            MessageOrEnum::Message(_) => None,
            MessageOrEnum::Enum(e) => Some(e),
        }
    }
}

#[derive(Debug, Clone)]
pub enum FilesOrMessage<F, M> {
    Files(Vec<F>),
    Message(M),
}
impl<'a> FilesOrMessage<&'a FileDescriptorExt<'a>, &'a DescriptorExt<'a>> {
    pub fn messages(&'a self) -> impl Iterator<Item = &'a DescriptorExt<'a>> {
        match self {
            FilesOrMessage::Files(files) => {
                Box::new(files.iter().flat_map(|f| f.messages())) as Box<dyn Iterator<Item = _>>
            }
            FilesOrMessage::Message(m) => Box::new(m.nested_types()) as Box<dyn Iterator<Item = _>>,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct DebugFileDescriptor {
        name: &'static str,
        package: Option<&'static str>,
        message_types: Vec<DebugDescriptor>,
        enum_types: Vec<DebugEnumDescriptor>,
    }
    #[derive(Default)]
    struct DebugDescriptor {
        name: &'static str,
        nested_types: Vec<DebugDescriptor>,
        enum_types: Vec<DebugEnumDescriptor>,
    }
    #[derive(Default)]
    struct DebugEnumDescriptor {
        name: &'static str,
    }

    impl From<DebugEnumDescriptor> for protobuf::EnumDescriptorProto {
        fn from(d: DebugEnumDescriptor) -> Self {
            let mut e = protobuf::EnumDescriptorProto::default();
            e.field_mut(1).push_string(d.name);
            e
        }
    }
    impl From<DebugDescriptor> for protobuf::DescriptorProto {
        fn from(d: DebugDescriptor) -> Self {
            let mut m = protobuf::DescriptorProto::default();
            m.field_mut(1).push_string(d.name);
            for n in d.nested_types {
                m.field_mut(3)
                    .push_message(Into::<protobuf::DescriptorProto>::into(n).into());
            }
            for e in d.enum_types {
                m.field_mut(4)
                    .push_message(Into::<protobuf::EnumDescriptorProto>::into(e).into());
            }
            m
        }
    }
    impl From<DebugFileDescriptor> for protobuf::FileDescriptorProto {
        fn from(d: DebugFileDescriptor) -> Self {
            let mut fd = protobuf::FileDescriptorProto::default();
            fd.field_mut(1).push_string(d.name);
            if let Some(p) = d.package {
                fd.field_mut(2).push_string(p);
            }
            for m in d.message_types {
                fd.field_mut(4)
                    .push_message(Into::<protobuf::DescriptorProto>::into(m).into());
            }
            for e in d.enum_types {
                fd.field_mut(5)
                    .push_message(Into::<protobuf::EnumDescriptorProto>::into(e).into());
            }
            fd
        }
    }

    #[test]
    fn test_resolve_path() {
        fn make_fd(name: &'static str, package: &'static str) -> protobuf::FileDescriptorProto {
            type FD = DebugFileDescriptor;
            type MD = DebugDescriptor;
            type ED = DebugEnumDescriptor;
            FD {
                name,
                package: Some(package),
                message_types: vec![
                    MD {
                        name: "A",
                        nested_types: vec![
                            MD {
                                name: "B",
                                nested_types: vec![MD {
                                    name: "C",
                                    ..Default::default()
                                }],
                                ..Default::default()
                            },
                            MD {
                                name: "B2",
                                ..Default::default()
                            },
                        ],
                        enum_types: vec![ED { name: "F" }],
                    },
                    MD {
                        name: "A2",
                        ..Default::default()
                    },
                ],
                enum_types: vec![ED { name: "E" }],
            }
            .into()
        }
        let fd0 = make_fd("fd0.proto", "");
        let fd1 = make_fd("fd1.proto", "a");
        let fd2 = make_fd("fd2.proto", "a.b");
        let fd3 = make_fd("fd3.proto", "a.b2");
        let fd4 = make_fd("fd4.proto", "a.b.c");

        let root = RootContext::from(vec![fd0, fd1, fd2, fd3, fd4]);

        let assert_is_message = |path: &str, name: &str| {
            let result = root.resolve_path(path).unwrap();
            let MessageOrEnum::Message(m) = result else {
                panic!("Expected a message: {}", path);
            };
            assert_eq!(m.name(), name);
        };
        let assert_is_enum = |path: &str, name: &str| {
            let result = root.resolve_path(path).unwrap();
            let MessageOrEnum::Enum(e) = result else {
                panic!("Expected an enum: {}", path);
            };
            assert_eq!(e.name(), name);
        };
        assert_is_message("A", "A");
        assert_is_message("A.B", "B");
        assert_is_message("A.B2", "B2");
        assert_is_message("A.B.C", "C");
        assert_is_message("A2", "A2");
        assert_is_message(".A", "A");
        assert_is_message(".A.B", "B");
        assert_is_message(".A.B2", "B2");
        assert_is_message(".A.B.C", "C");
        assert_is_message(".A2", "A2");
        assert_is_message("a.A", "A");
        assert_is_message("a.A.B", "B");
        assert_is_message("a.A.B2", "B2");
        assert_is_message("a.A.B.C", "C");
        assert_is_message("a.A2", "A2");
        assert_is_message(".a.A", "A");
        assert_is_message(".a.A.B", "B");
        assert_is_message(".a.A.B2", "B2");
        assert_is_message(".a.A.B.C", "C");
        assert_is_message(".a.A2", "A2");
        assert_is_message("a.b.A", "A");
        assert_is_message("a.b.A.B", "B");
        assert_is_message("a.b.A.B2", "B2");
        assert_is_message("a.b.A.B.C", "C");
        assert_is_message("a.b.A2", "A2");
        assert_is_message(".a.b.A", "A");
        assert_is_message(".a.b.A.B", "B");
        assert_is_message(".a.b.A.B2", "B2");
        assert_is_message(".a.b.A.B.C", "C");
        assert_is_message(".a.b.A2", "A2");
        assert_is_message("a.b.c.A", "A");
        assert_is_message("a.b.c.A.B", "B");
        assert_is_message("a.b.c.A.B2", "B2");
        assert_is_message("a.b.c.A.B.C", "C");
        assert_is_message("a.b.c.A2", "A2");
        assert_is_message(".a.b.c.A", "A");
        assert_is_message(".a.b.c.A.B", "B");
        assert_is_message(".a.b.c.A.B2", "B2");
        assert_is_message(".a.b.c.A.B.C", "C");
        assert_is_message(".a.b.c.A2", "A2");
        assert_is_message("a.b2.A", "A");
        assert_is_message("a.b2.A.B", "B");
        assert_is_message("a.b2.A.B2", "B2");
        assert_is_message("a.b2.A.B.C", "C");
        assert_is_message("a.b2.A2", "A2");
        assert_is_message(".a.b2.A", "A");
        assert_is_message(".a.b2.A.B", "B");
        assert_is_message(".a.b2.A.B2", "B2");
        assert_is_message(".a.b2.A.B.C", "C");
        assert_is_message(".a.b2.A2", "A2");
        assert_is_enum("E", "E");
        assert_is_enum("A.F", "F");
        assert_is_enum(".E", "E");
        assert_is_enum(".A.F", "F");
        assert_is_enum("a.E", "E");
        assert_is_enum("a.A.F", "F");
        assert_is_enum(".a.E", "E");
        assert_is_enum(".a.A.F", "F");
        assert_is_enum("a.b.E", "E");
        assert_is_enum("a.b.A.F", "F");
        assert_is_enum(".a.b.E", "E");
        assert_is_enum(".a.b.A.F", "F");
        assert_is_enum("a.b.c.E", "E");
        assert_is_enum("a.b.c.A.F", "F");
        assert_is_enum(".a.b.c.E", "E");
        assert_is_enum(".a.b.c.A.F", "F");
        assert_is_enum("a.b2.E", "E");
        assert_is_enum("a.b2.A.F", "F");
        assert_is_enum(".a.b2.E", "E");
        assert_is_enum(".a.b2.A.F", "F");
    }
}
