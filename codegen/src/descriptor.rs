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
use crate::Result;
use ::itertools::{Either, Itertools};
use ::std::cell::OnceCell;
use ::std::collections::HashMap;
use ::std::fmt::Debug;

#[derive(Debug)]
pub struct RootContext<'a> {
    files: Vec<(FileDescriptorBase, OnceCell<FileDescriptor<'a>>)>,
    cache: RootContextCache<'a>,
}

#[derive(Default, Debug)]
struct RootContextCache<'a> {
    package_to_files: OnceCell<HashMap<ProtoPathBuf, Vec<&'a FileDescriptor<'a>>>>,
}

impl From<FileDescriptorBase> for RootContext<'_> {
    fn from(f: FileDescriptorBase) -> Self {
        Self {
            files: vec![(f, OnceCell::default())],
            cache: Default::default(),
        }
    }
}
impl<T: IntoIterator<Item = FileDescriptorBase>> From<T> for RootContext<'_> {
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
    pub fn files(&'a self) -> impl Iterator<Item = &'a FileDescriptor> {
        self.files
            .iter()
            .map(|(f, c)| c.get_or_init(|| FileDescriptor::new(self, f)))
    }
    pub fn file_from_name(&'a self, name: &str) -> Result<&'a FileDescriptor<'a>> {
        Ok(self
            .files()
            .into_iter()
            .find(|f| f.name() == name)
            .ok_or_else(|| format!("No such file: {}", name))?)
    }
    pub fn package_to_files(
        &'a self,
        package: impl AsRef<ProtoPath>,
    ) -> Result<impl Iterator<Item = &'a FileDescriptor>> {
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
    ) -> Result<MessageOrEnum<&Descriptor, &EnumDescriptor>> {
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
    ) -> Result<MessageOrEnum<&Descriptor, &EnumDescriptor>> {
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
    ) -> Result<Option<MessageOrEnum<&Descriptor, &EnumDescriptor>>> {
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

#[derive(Debug, Clone)]
pub enum MessageOrEnum<M, E> {
    Message(M),
    Enum(E),
}
impl<'a> MessageOrEnum<&'a Descriptor<'a>, &'a EnumDescriptor<'a>> {
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

#[derive(Debug, Clone)]
pub enum FilesOrMessage<F, M> {
    Files(Vec<F>),
    Message(M),
}
impl<'a> FilesOrMessage<&'a FileDescriptor<'a>, &'a Descriptor<'a>> {
    pub fn direct_messages(&self) -> impl Iterator<Item = Result<&Descriptor<'a>>> {
        match self {
            FilesOrMessage::Files(files) => todo!()
            FilesOrMessage::Message(m) => todo!(),
        };
        ::std::iter::once(todo!())
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

#[allow(unused)]
trait UninterpretedOptionExt {
    fn name_as_string(&self) -> Result<String>;
}
impl<'a> UninterpretedOptionExt for ::puroro::google::protobuf::UninterpretedOptionProto<'a> {
    fn name_as_string(&self) -> Result<String> {
        let parts = self
            .name()
            .map(|n| -> Result<_> {
                let n = n?;
                Ok(if n.is_extension()?.unwrap_or_default() {
                    format!("({})", n.name_part()?.unwrap_or_default())
                } else {
                    n.name_part()?.unwrap_or_default().to_string()
                })
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(parts.join("."))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_path() {
        fn make_fd(name: &str, package: &str) -> FileDescriptorBase {
            type FD<'a> = DebugFileDescriptor<'a>;
            type MD<'a> = DebugDescriptor<'a>;
            type ED<'a> = DebugEnumDescriptor<'a>;
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
                        enum_types: vec![ED {
                            name: "F",
                            ..Default::default()
                        }],
                        ..Default::default()
                    },
                    MD {
                        name: "A2",
                        ..Default::default()
                    },
                ],
                enum_types: vec![ED {
                    name: "E",
                    ..Default::default()
                }],
                ..Default::default()
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
            assert_eq!(m.name().unwrap(), name);
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
