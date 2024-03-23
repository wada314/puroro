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

pub trait FileDescriptor {
    fn name(&self) -> &str;
    fn package(&self) -> &str;
    fn dependencies(&self) -> &[&dyn FileDescriptor];
    fn public_dependencies(&self) -> &[&dyn FileDescriptor];
    fn weak_dependencies(&self) -> &[&dyn FileDescriptor];
    fn message_types(&self) -> &[&dyn Descriptor];
    fn enum_types(&self) -> &[&dyn EnumDescriptor];
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

pub struct FileDescriptorStruct {
    name: Cow<'static, str>,
    package: Cow<'static, str>,
    message_types: Vec<DescriptorStruct>,
    enum_types: Vec<EnumDescriptorStruct>,
}

pub struct DescriptorStruct {
    name: Cow<'static, str>,
    full_name: Cow<'static, str>,
    fields: Vec<FieldDescriptorStruct>,
    oneof_decls: Vec<OneofDescriptorStruct>,
    nested_types: Vec<DescriptorStruct>,
    enum_types: Vec<EnumDescriptorStruct>,
}

pub struct FieldDescriptorStruct {
    name: Cow<'static, str>,
    number: i32,
    type_: self::field_descriptor::Type,
    type_name: Cow<'static, str>,
    label: self::field_descriptor::Label,
}

pub struct EnumDescriptorStruct {
    name: Cow<'static, str>,
    values: Vec<EnumValueDescriptorStruct>,
}

pub struct EnumValueDescriptorStruct {
    name: Cow<'static, str>,
    number: i32,
}

pub struct OneofDescriptorStruct {
    name: Cow<'static, str>,
    fields: Vec<FieldDescriptorStruct>,
    is_synthetic: bool,
}
