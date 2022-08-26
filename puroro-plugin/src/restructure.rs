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

use ::once_cell::unsync::OnceCell;
#[allow(unused)]
use ::puroro_protobuf_compiled::google::protobuf::{
    DescriptorProto, EnumDescriptorProto, FieldDescriptorProto, FileDescriptorProto,
    OneofDescriptorProto,
};
use ::std::ops::Deref;

pub struct Message<'a> {
    proto: &'a DescriptorProto,
    fields: OnceCell<Box<[Field<'a>]>>,
    oneofs: OnceCell<Box<[Oneof<'a>]>>,
}
impl<'a> Message<'a> {
    pub fn new(proto: &'a DescriptorProto) -> Self {
        Self {
            proto,
            fields: OnceCell::default(),
            oneofs: OnceCell::default(),
        }
    }
    pub fn proto(&self) -> &DescriptorProto {
        &self.proto
    }
    pub fn fields(&self) -> &[Field<'a>] {
        todo!()
    }
    pub fn field(&self) -> &[Field<'a>] {
        self.fields()
    }
    pub fn oneofs(&self) -> &[Oneof<'a>] {
        todo!()
    }
}
impl Deref for Message<'_> {
    type Target = DescriptorProto;
    fn deref(&self) -> &Self::Target {
        &self.proto
    }
}

pub struct Field<'a> {
    proto: &'a FieldDescriptorProto,
    parent: &'a Message<'a>,
}
impl<'a> Field<'a> {
    pub fn new(proto: &'a FieldDescriptorProto, parent: &'a Message<'a>) -> Self {
        Self { proto, parent }
    }
    pub fn proto(&self) -> &FieldDescriptorProto {
        &self.proto
    }
}
impl Deref for Field<'_> {
    type Target = FieldDescriptorProto;
    fn deref(&self) -> &Self::Target {
        &self.proto
    }
}

pub struct Oneof<'a> {
    proto: &'a OneofDescriptorProto,
    parent: &'a Message<'a>,
    oneof_index: i32,
    fields: OnceCell<Box<[OneofField<'a>]>>,
}
impl<'a> Oneof<'a> {
    pub fn new(proto: &'a OneofDescriptorProto, parent: &'a Message<'a>, oneof_index: i32) -> Self {
        Self {
            proto,
            parent,
            oneof_index,
            fields: OnceCell::default(),
        }
    }
    pub fn proto(&self) -> &OneofDescriptorProto {
        &self.proto
    }
    pub fn fields(&'a self) -> &[OneofField<'a>] {
        let parent = self.parent;
        let oneof_index = self.oneof_index;
        self.fields.get_or_init(|| {
            let mut fields = Vec::new();
            for field_candidate in parent.proto().field() {
                if field_candidate.oneof_index_opt() == Some(oneof_index) {
                    fields.push(OneofField::new(field_candidate, self))
                }
            }
            fields.into_boxed_slice()
        })
    }
}
impl Deref for Oneof<'_> {
    type Target = OneofDescriptorProto;
    fn deref(&self) -> &Self::Target {
        &self.proto
    }
}

pub struct OneofField<'a> {
    proto: &'a FieldDescriptorProto,
    parent: &'a Oneof<'a>,
}
impl<'a> OneofField<'a> {
    pub fn new(proto: &'a FieldDescriptorProto, parent: &'a Oneof<'a>) -> Self {
        Self { proto, parent }
    }
    pub fn proto(&self) -> &FieldDescriptorProto {
        &self.proto
    }
}
impl Deref for OneofField<'_> {
    type Target = FieldDescriptorProto;
    fn deref(&self) -> &Self::Target {
        &self.proto
    }
}

enum FieldType {
    Int32,
    UInt32,
    SInt32,
    Int64,
    UInt64,
    SInt64,
    Enum2(),
    Enum3(),
    Fixed32,
    SFixed32,
    Fixed64,
    SFixed64,
    Float,
    Double,
    Bool,
    String,
    Bytes,
    Message(),
}
