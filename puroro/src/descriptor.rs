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

use ::std::borrow::Cow;
use ::std::cell::OnceCell;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Edition {
    Unknown,
    Proto2,
    Proto3,
    Edition2023,
    Edition2024,
}

pub mod field_descriptor {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum Type {
        DOUBLE,
        FLOAT,
        INT64,
        UINT64,
        INT32,
        FIXED64,
        FIXED32,
        BOOL,
        STRING,
        GROUP,
        MESSAGE,
        BYTES,
        UINT32,
        ENUM,
        SFIXED32,
        SFIXED64,
        SINT32,
        SINT64,
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum Label {
        OPTIONAL,
        REQUIRED,
        REPEATED,
    }
}

/// Structs for the each descriptor types.
/// These structs are strictly read-only and only knows about its children, not parent.

#[derive(Debug, Clone)]
pub struct FileDescriptorStruct {
    pub name: Cow<'static, str>,
    pub package: Cow<'static, str>,
    pub dependency_indices: Cow<'static, [usize]>,
    pub message_types: Cow<'static, [DescriptorStruct]>,
    pub enum_types: Cow<'static, [EnumDescriptorStruct]>,
    pub syntax: Cow<'static, str>,
    pub edition: Edition,
}

#[derive(Debug, Clone)]
pub struct DescriptorStruct {
    pub name: Cow<'static, str>,
    pub full_name: Cow<'static, str>,
    pub fields: Cow<'static, [FieldDescriptorStruct]>,
    pub oneof_decls: Cow<'static, [OneofDescriptorStruct]>,
    pub nested_types: Cow<'static, [DescriptorStruct]>,
    pub enum_types: Cow<'static, [EnumDescriptorStruct]>,
}

#[derive(Debug, Clone)]
pub struct FieldDescriptorStruct {
    pub name: Cow<'static, str>,
    pub number: i32,
    pub type_: self::field_descriptor::Type,
    pub type_name: Cow<'static, str>,
    pub label: self::field_descriptor::Label,
}

#[derive(Debug, Clone)]
pub struct EnumDescriptorStruct {
    pub name: Cow<'static, str>,
    pub values: Cow<'static, [EnumValueDescriptorStruct]>,
}

#[derive(Debug, Clone)]
pub struct EnumValueDescriptorStruct {
    pub name: Cow<'static, str>,
    pub number: i32,
}

#[derive(Debug, Clone)]
pub struct OneofDescriptorStruct {
    pub name: Cow<'static, str>,
    pub field_indices: Cow<'static, [usize]>,
}

/// The struct types with the "context".
/// The context here means the path from the root of the descriptor tree to the current node.

pub struct Context {
    files: Vec<FileDescriptorStruct>,
}
impl From<FileDescriptorStruct> for Context {
    fn from(f: FileDescriptorStruct) -> Self {
        Self { files: vec![f] }
    }
}
impl From<Vec<FileDescriptorStruct>> for Context {
    fn from(files: Vec<FileDescriptorStruct>) -> Self {
        Self { files }
    }
}
impl<const N: usize> From<[FileDescriptorStruct; N]> for Context {
    fn from(files: [FileDescriptorStruct; N]) -> Self {
        Self {
            files: files.into(),
        }
    }
}
impl Context {
    fn files(&self) -> impl IntoIterator<Item = FileDescriptorWithContext> {
        (0..(self.files.len())).map(move |i| self.file_from_index(i))
    }
    fn file_from_index(&self, index: usize) -> FileDescriptorWithContext {
        FileDescriptorWithContext {
            root: self,
            body: &self.files[index],
            dependencies: Default::default(),
        }
    }
}

pub struct FileDescriptorWithContext<'a> {
    root: &'a Context,
    body: &'a FileDescriptorStruct,
    dependencies: OnceCell<Vec<FileDescriptorWithContext<'a>>>,
    messages: OnceCell<Vec<DescriptorWithContext<'a>>>,
    enums: OnceCell<Vec<EnumDescriptorWithContext<'a>>>,
}

impl<'a> FileDescriptorWithContext<'a> {
    fn name(&self) -> &str {
        &self.body.name
    }
    fn package(&self) -> &str {
        &self.body.package
    }
    fn dependencies(&'a self) -> impl 'a + IntoIterator<Item = &FileDescriptorWithContext> {
        self.dependencies.get_or_init(|| {
            self.body
                .dependency_indices
                .iter()
                .map(|&i| self.root.file_from_index(i))
                .collect()
        })
    }
    fn messages(&'a self) -> impl 'a + IntoIterator<Item = &DescriptorWithContext<'a>> {
        self.messages.get_or_init(|| {
            self.body
                .message_types
                .iter()
                .map(|m| DescriptorWithContext {
                    file: self,
                    maybe_nested: None,
                    body: m,
                })
                .collect()
        })
    }
    fn enums(&'a self) -> impl 'a + IntoIterator<Item = &EnumDescriptorWithContext<'a>> {
        self.enums.get_or_init(|| {
            self.body
                .enum_types
                .iter()
                .map(|e| EnumDescriptorWithContext {
                    root: self.root,
                    file: self.body,
                    descriptor: e,
                })
                .collect()
        })
    }
}

pub struct DescriptorWithContext<'a> {
    file: &'a FileDescriptorWithContext<'a>,
    maybe_nested: Option<&'a DescriptorWithContext<'a>>,
    body: &'a DescriptorStruct,
}

pub struct EnumDescriptorWithContext<'a> {
    root: &'a Context,
    file: &'a FileDescriptorStruct,
    descriptor: &'a EnumDescriptorStruct,
}
