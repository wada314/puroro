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

#![allow(unused)]

mod r#enum;
mod field;
mod file;
mod message;
mod subtypes;
pub use field::*;
pub use file::*;
pub use message::*;
pub use r#enum::*;
pub use subtypes::*;

use crate::proto_path::{ProtoPath, ProtoPathBuf};
use crate::{ErrorKind, Result};
use ::itertools::{Either, Itertools};
use ::puroro::google::protobuf::{
    field_descriptor_proto::Label as FieldLabelProto,
    field_descriptor_proto::Type as FieldTypeProto, DescriptorProto, Edition as EditionProto,
    EnumDescriptorProto, EnumValueDescriptorProto, FieldDescriptorProto, FileDescriptorProto,
    FileDescriptorSet, OneofDescriptorProto,
};
use ::puroro::Result as PResult;
use ::std::cell::OnceCell;
use ::std::collections::HashMap;
use ::std::fmt::Debug;
use ::std::ops::Deref;

/// The struct types with the "context".
/// The context here means the path from the root of the descriptor tree to the current node.

// region: RootContext

#[derive(Debug)]
pub struct RootContext<'a> {
    files: Vec<(FileDescriptor, OnceCell<FileDescriptorWithContext<'a>>)>,
    package_to_files: OnceCell<HashMap<ProtoPathBuf, Vec<&'a FileDescriptorWithContext<'a>>>>,
}
impl From<FileDescriptor> for RootContext<'_> {
    fn from(f: FileDescriptor) -> Self {
        Self {
            files: vec![(f, OnceCell::default())],
            package_to_files: Default::default(),
        }
    }
}
impl<T: IntoIterator<Item = FileDescriptor>> From<T> for RootContext<'_> {
    fn from(files: T) -> Self {
        Self {
            files: files
                .into_iter()
                .map(|f| (f, OnceCell::default()))
                .collect(),
            package_to_files: Default::default(),
        }
    }
}
impl<'a> RootContext<'a> {
    pub fn files(&'a self) -> impl 'a + IntoIterator<Item = &'a FileDescriptorWithContext<'a>> {
        self.files.iter().map(|(f, c)| {
            c.get_or_init(|| FileDescriptorWithContext {
                root: self,
                body: f,
                cache: Default::default(),
            })
        })
    }
    pub fn file_from_name(&'a self, name: &str) -> Result<&'a FileDescriptorWithContext<'a>> {
        Ok(self
            .files()
            .into_iter()
            .find(|f| f.name().is_ok_and(|n| n == name))
            .ok_or_else(|| format!("No such file: {}", name))?)
    }
    pub fn package_to_files(
        &'a self,
        package: impl AsRef<ProtoPath>,
    ) -> Result<impl 'a + IntoIterator<Item = &'a FileDescriptorWithContext<'a>>> {
        let package = if package.as_ref().is_relative() {
            // This method is a root method, so the relative path should be converted
            // to the absolute path by just adding '.' at the beginning.
            format!(".{}", package.as_ref()).into()
        } else {
            package.as_ref().to_owned()
        };
        debug_assert!(package.is_absolute());
        let map = self.package_to_files.get_or_try_init(|| -> Result<_> {
            let mut map = HashMap::new();
            for fd in self.files() {
                let package = fd.absolute_package()?.to_owned();
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
    ) -> Result<MessageOrEnum<&DescriptorWithContext, &EnumDescriptorWithContext>> {
        let path = path.as_ref();
        let path = if path.is_relative() {
            ProtoPathBuf::from(format!(".{}", path))
        } else {
            path.to_owned()
        };
        return Ok(self
            .resolve_absolute_path(&path)?
            .ok_or_else(|| format!("Path not found: {}", path))?);
        Err(format!("Path not found: {}", path))?
    }
    pub fn resolve_relative_path(
        &'a self,
        path: impl AsRef<ProtoPath>,
        cur: impl AsRef<ProtoPath>,
    ) -> Result<MessageOrEnum<&DescriptorWithContext, &EnumDescriptorWithContext>> {
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
    ) -> Result<Option<MessageOrEnum<&DescriptorWithContext, &EnumDescriptorWithContext>>> {
        debug_assert!(path.is_absolute());
        // Can improve the complexity here. Maybe later.
        for package_path in path.ancestors() {
            for file in self.package_to_files(package_path)? {
                for message_or_enum in file.all_messages_or_enums()? {
                    if message_or_enum.full_path()? == path {
                        return Ok(Some(message_or_enum));
                    }
                }
            }
        }
        Ok(None)
    }
}

// endregion:







// region: utils

#[derive(Debug, Clone)]
pub enum MessageOrEnum<M, E> {
    Message(M),
    Enum(E),
}
impl<'a> MessageOrEnum<&'a DescriptorWithContext<'a>, &'a EnumDescriptorWithContext<'a>> {
    pub fn full_path(&self) -> Result<&ProtoPath> {
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

trait TryIntoString {
    fn try_into_string(self, error_message: &str) -> Result<String>;
}
impl TryIntoString for Option<&str> {
    fn try_into_string(self, error_message: &str) -> Result<String> {
        Ok(self
            .ok_or_else(|| error_message.to_string())
            .map(str::to_string)?)
    }
}
trait TryIntoNumber<T> {
    fn try_into_number(self, error_message: &str) -> Result<T>;
}
impl<T> TryIntoNumber<T> for Option<T> {
    fn try_into_number(self, error_message: &str) -> Result<T> {
        Ok(self.ok_or_else(|| error_message.to_string())?)
    }
}

// endregion:

#[cfg(test)]
mod tests {
    use super::*;
    use ::std::assert_matches::assert_matches;

    const FD_DEFAULT: FileDescriptor = FileDescriptor {
        name: String::new(),
        dependencies: vec![],
        package: None,
        message_types: vec![],
        enum_types: vec![],
        syntax: None,
        edition: None,
    };
    const MD_DEFAULT: Descriptor = Descriptor {
        name: String::new(),
        fields: vec![],
        oneof_decls: vec![],
        nested_types: vec![],
        enum_types: vec![],
    };
    const ED_DEFAULT: EnumDescriptor = EnumDescriptor {
        name: String::new(),
        values: vec![],
    };

    #[test]
    fn test_package_to_files() {
        fn make_fd(name: &str, package: &str) -> FileDescriptor {
            FileDescriptor {
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
        assert!(root_package_files
            .iter()
            .any(|f| f.name().unwrap() == "fd0.proto"));
        assert!(package_a_files
            .iter()
            .any(|f| f.name().unwrap() == "fd1.proto"));
        assert!(package_a_b_files
            .iter()
            .any(|f| f.name().unwrap() == "fd2.proto"));
        assert!(package_a_b_files
            .iter()
            .any(|f| f.name().unwrap() == "fd3.proto"));
        assert!(package_a_b_c_files
            .iter()
            .any(|f| f.name().unwrap() == "fd4.proto"));
    }

    #[test]
    fn test_resolve_path() {
        fn make_fd(name: &str, package: &str) -> FileDescriptor {
            FileDescriptor {
                name: name.to_string(),
                package: Some(package.into()),
                message_types: vec![
                    Descriptor {
                        name: "A".to_string(),
                        nested_types: vec![
                            Descriptor {
                                name: "B".to_string(),
                                nested_types: vec![Descriptor {
                                    name: "C".to_string(),
                                    ..MD_DEFAULT
                                }],
                                ..MD_DEFAULT
                            },
                            Descriptor {
                                name: "B2".to_string(),
                                ..MD_DEFAULT
                            },
                        ],
                        enum_types: vec![EnumDescriptor {
                            name: "F".to_string(),
                            ..ED_DEFAULT
                        }],
                        ..MD_DEFAULT
                    },
                    Descriptor {
                        name: "A2".to_string(),
                        ..MD_DEFAULT
                    },
                ],
                enum_types: vec![EnumDescriptor {
                    name: "E".to_string(),
                    ..ED_DEFAULT
                }],
                ..FD_DEFAULT
            }
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
            assert_eq!(m.name().unwrap(), name);
        };
        let assert_is_enum = |path: &str, name: &str| {
            let result = root.resolve_path(path).unwrap();
            let MessageOrEnum::Enum(e) = result else {
                panic!("Expected an enum: {}", path);
            };
            assert_eq!(e.name().unwrap(), name);
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
