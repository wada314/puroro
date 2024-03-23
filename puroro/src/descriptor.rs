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

/// Public traits for the descriptor types.

pub trait FileDescriptor {
    fn name(&self) -> &str;
    fn package(&self) -> &str;
    fn dependencies(&self) -> &[&dyn FileDescriptor];
    fn public_dependencies(&self) -> &[&dyn FileDescriptor];
    fn message_types(&self) -> &[&dyn Descriptor];
    fn enum_types(&self) -> &[&dyn EnumDescriptor];
}

pub enum Edition {
    Unknown,
    Proto2,
    Proto3,
    Edition2023,
    Edition2024,
}

pub trait Descriptor {
    fn name(&self) -> &str;
    fn full_name(&self) -> &str;
    fn index(&self) -> usize;
    fn file(&self) -> &dyn FileDescriptor;
    fn containing_type(&self) -> Option<&dyn Descriptor>;
    fn fields(&self) -> &[&dyn FieldDescriptor];
    fn oneof_decls(&self) -> &[&dyn OneofDescriptor];
    fn real_oneof_decls(&self) -> &[&dyn OneofDescriptor];
    fn nested_types(&self) -> &[&dyn Descriptor];
    fn enum_types(&self) -> &[&dyn EnumDescriptor];
}

pub trait FieldDescriptor {
    fn name(&self) -> &str;
    fn full_name(&self) -> &str;
    fn file(&self) -> &dyn FileDescriptor;
    fn is_extension(&self) -> bool;
    fn number(&self) -> i32;
    fn type_(&self) -> self::field_descriptor::Type;
    fn type_name(&self) -> &str;
    fn label(&self) -> self::field_descriptor::Label;
    fn index(&self) -> usize;
    fn containing_type(&self) -> &dyn Descriptor;
    fn index_in_oneof(&self) -> usize;
    fn containing_oneof(&self) -> Option<&dyn OneofDescriptor>;
    fn real_containing_oneof(&self) -> Option<&dyn OneofDescriptor>;
    fn message_type(&self) -> Option<&dyn Descriptor>;
    fn enum_type(&self) -> Option<&dyn EnumDescriptor>;

    fn is_required(&self) -> bool {
        self.label() == self::field_descriptor::Label::REQUIRED
    }
    fn is_optional(&self) -> bool {
        self.label() == self::field_descriptor::Label::OPTIONAL
    }
    fn is_repeated(&self) -> bool {
        self.label() == self::field_descriptor::Label::REPEATED
    }
    fn is_packable(&self) -> bool {
        self.is_repeated() && todo!()
    }
    fn is_packed(&self) -> bool {
        todo!()
    }
    fn is_map(&self) -> bool;
    fn has_optional_keyword(&self) -> bool;
    fn has_presense(&self) -> bool;
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

pub trait EnumDescriptor {
    fn name(&self) -> &str;
    fn full_name(&self) -> &str;
    fn index(&self) -> usize;
    fn file(&self) -> &dyn FileDescriptor;
    fn values(&self) -> &[&dyn EnumValueDescriptor];
    fn containing_type(&self) -> &dyn Descriptor;
}

pub trait EnumValueDescriptor {
    fn name(&self) -> &str;
    fn full_name(&self) -> &str;
    fn index(&self) -> usize;
    fn number(&self) -> i32;
    fn file(&self) -> &dyn FileDescriptor;
    fn type_(&self) -> &dyn EnumDescriptor;
}

pub trait OneofDescriptor {
    fn name(&self) -> &str;
    fn full_name(&self) -> &str;
    fn index(&self) -> usize;
    fn is_synthetic(&self) -> bool;
    fn file(&self) -> &dyn FileDescriptor;
    fn containing_type(&self) -> &dyn Descriptor;
    fn fields(&self) -> &[&dyn FieldDescriptor];
}

/// Structs for the each descriptor types.
/// These structs are strictly read-only and only knows about its children, not parent.
/// These structs does not directly impl the traits above.

pub struct FileDescriptorStruct {
    pub name: Cow<'static, str>,
    pub package: Cow<'static, str>,
    pub dependency_indices: Vec<usize>,
    pub message_types: Vec<DescriptorStruct>,
    pub enum_types: Vec<EnumDescriptorStruct>,
    pub syntax: Cow<'static, str>,
    pub edition: Edition,
}

pub struct DescriptorStruct {
    pub name: Cow<'static, str>,
    pub full_name: Cow<'static, str>,
    pub fields: Vec<FieldDescriptorStruct>,
    pub oneof_decls: Vec<OneofDescriptorStruct>,
    pub nested_types: Vec<DescriptorStruct>,
    pub enum_types: Vec<EnumDescriptorStruct>,
}

pub struct FieldDescriptorStruct {
    pub name: Cow<'static, str>,
    pub number: i32,
    pub type_: self::field_descriptor::Type,
    pub type_name: Cow<'static, str>,
    pub label: self::field_descriptor::Label,
}

pub struct EnumDescriptorStruct {
    pub name: Cow<'static, str>,
    pub values: Vec<EnumValueDescriptorStruct>,
}

pub struct EnumValueDescriptorStruct {
    pub name: Cow<'static, str>,
    pub number: i32,
}

pub struct OneofDescriptorStruct {
    pub name: Cow<'static, str>,
    pub field_indices: Vec<usize>,
}

/// The struct types with the "context".
/// The context here means the path from the root of the descriptor tree to the current node.
/// These contexted structs implements the descriptor traits above,
/// and they can be converted from the raw struct types above.

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
    fn files(&self) -> impl '_ + IntoIterator<Item = FileDescriptorWithContext> {
        (0..(self.files.len())).map(move |i| self.file_from_index(i))
    }
    fn file_from_index(&self, index: usize) -> FileDescriptorWithContext {
        FileDescriptorWithContext {
            root: self,
            index_in_root: index,
            file: &self.files[index],
            dependencies: Default::default(),
        }
    }
}

pub struct FileDescriptorWithContext<'a> {
    root: &'a Context,
    index_in_root: usize,
    file: &'a FileDescriptorStruct,
    dependencies: OnceCell<Vec<&'a FileDescriptorWithContext<'a>>>,
}

impl FileDescriptor for FileDescriptorWithContext<'_> {
    fn name(&self) -> &str {
        &self.file.name
    }
    fn package(&self) -> &str {
        &self.file.package
    }
    fn dependencies(&self) -> &[&dyn FileDescriptor] {
        self.dependencies.get_or_init(|| {
            self.file
                .dependency_indices
                .iter()
                .map(|&i| self.root.file_from_index(i))
                .collect()
        })
    }
    fn public_dependencies(&self) -> &[&dyn FileDescriptor] {
        todo!()
    }
    fn message_types(&self) -> &[&dyn Descriptor] {
        todo!()
    }
    fn enum_types(&self) -> &[&dyn EnumDescriptor] {
        todo!()
    }
}
