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

use crate::utils::{Fqtn, Package};
use crate::{ErrorKind, Result};
use ::once_cell::unsync::OnceCell;
#[allow(unused)]
use ::puroro_protobuf_compiled::google::protobuf::{
    DescriptorProto, EnumDescriptorProto, FieldDescriptorProto, FileDescriptorProto,
    OneofDescriptorProto,
};
use ::std::ops::Deref;

#[derive(Debug, Clone)]
pub struct File<'a> {
    proto: &'a FileDescriptorProto,
    messages: OnceCell<Box<[Message<'a>]>>,
    all_messages: OnceCell<Box<[&'a Message<'a>]>>,
    enums: OnceCell<Box<[Enum<'a>]>>,
    all_enums: OnceCell<Box<[&'a Enum<'a>]>>,
}
impl<'a> File<'a> {
    pub fn new(proto: &'a FileDescriptorProto) -> Self {
        Self {
            proto,
            messages: OnceCell::default(),
            all_messages: OnceCell::default(),
            enums: OnceCell::default(),
            all_enums: OnceCell::default(),
        }
    }
    pub fn proto(&'a self) -> &FileDescriptorProto {
        &self.proto
    }
    pub fn messages(&'a self) -> &[Message<'_>] {
        self.messages.get_or_init(|| {
            self.proto()
                .message_type()
                .into_iter()
                .map(|m| Message::new(m, FileOrMessageRef::File(self)))
                .collect::<Vec<_>>()
                .into_boxed_slice()
        })
    }
    pub fn all_messages(&'a self) -> &[&Message<'_>] {
        self.all_messages.get_or_init(|| {
            let mut queue = self.messages().into_iter().collect::<Vec<_>>();
            let mut result = Vec::with_capacity(queue.len());
            while let Some(m) = queue.pop() {
                queue.extend(m.messages().into_iter());
                result.push(m);
            }
            result.into_boxed_slice()
        })
    }
    pub fn enums(&'a self) -> &[Enum<'_>] {
        self.enums.get_or_init(|| {
            self.proto()
                .enum_type()
                .into_iter()
                .map(|e| Enum::new(e, FileOrMessageRef::File(self)))
                .collect::<Vec<_>>()
                .into_boxed_slice()
        })
    }
    pub fn all_enums(&'a self) -> &[&Enum<'_>] {
        self.all_enums.get_or_init(|| {
            let direct = self.enums().into_iter();
            let indirect = self
                .all_messages()
                .into_iter()
                .flat_map(|m| m.enums().into_iter());
            direct
                .chain(indirect)
                .collect::<Vec<_>>()
                .into_boxed_slice()
        })
    }
    pub fn try_syntax(&'a self) -> Result<Syntax> {
        Ok(match self.proto().syntax() {
            "proto2" | "" => Syntax::Proto2,
            "proto3" => Syntax::Proto3,
            e => Err(ErrorKind::UnknownProtoSyntax {
                name: e.to_string(),
            })?,
        })
    }
    pub fn package(&'a self) -> Package<&'a str> {
        Package::new(self.proto().package())
    }
}
impl Deref for File<'_> {
    type Target = FileDescriptorProto;
    fn deref(&self) -> &Self::Target {
        &self.proto
    }
}

#[derive(Debug, Clone)]
pub struct Message<'a> {
    proto: &'a DescriptorProto,
    parent: FileOrMessageRef<'a>,
    fields: OnceCell<Box<[Field<'a>]>>,
    oneofs: OnceCell<Box<[Oneof<'a>]>>,
    messages: OnceCell<Box<[Message<'a>]>>,
    enums: OnceCell<Box<[Enum<'a>]>>,
    fqtn: OnceCell<Fqtn<String>>,
    file: OnceCell<&'a File<'a>>,
}
impl<'a> Message<'a> {
    pub fn new(proto: &'a DescriptorProto, parent: FileOrMessageRef<'a>) -> Self {
        Self {
            proto,
            parent,
            fields: OnceCell::default(),
            oneofs: OnceCell::default(),
            messages: OnceCell::default(),
            enums: OnceCell::default(),
            fqtn: OnceCell::default(),
            file: OnceCell::default(),
        }
    }
    pub fn proto(&'a self) -> &DescriptorProto {
        &self.proto
    }
    pub fn parent(&'a self) -> FileOrMessageRef<'_> {
        self.parent.clone()
    }
    pub fn fields(&'a self) -> &[Field<'_>] {
        self.fields.get_or_init(|| {
            self.proto()
                .field()
                .into_iter()
                .filter(|f| !f.proto3_optional())
                .map(|f| Field::new(f, self))
                .collect::<Vec<_>>()
                .into_boxed_slice()
        })
    }
    pub fn field(&'a self) -> &[Field<'_>] {
        self.fields()
    }
    pub fn oneofs(&'a self) -> &[Oneof<'_>] {
        self.oneofs.get_or_init(|| {
            let max_oneof_index = self
                .proto()
                .field()
                .iter()
                .filter_map(|f| {
                    (!f.proto3_optional() && f.has_oneof_index()).then_some(f.oneof_index())
                })
                .max();
            self.proto()
                .oneof_decl()
                .iter()
                .take(max_oneof_index.map_or(0, |i| i + 1) as usize)
                .enumerate()
                .map(|(i, o)| Oneof::new(o, self, i as i32))
                .collect::<Vec<_>>()
                .into_boxed_slice()
        })
    }
    pub fn messages(&'a self) -> &[Message<'_>] {
        self.messages.get_or_init(|| {
            self.proto()
                .nested_type()
                .into_iter()
                .map(|m| Message::new(m, FileOrMessageRef::Message(self)))
                .collect::<Vec<_>>()
                .into_boxed_slice()
        })
    }
    pub fn enums(&'a self) -> &[Enum<'_>] {
        self.enums.get_or_init(|| {
            self.proto()
                .enum_type()
                .into_iter()
                .map(|e| Enum::new(e, FileOrMessageRef::Message(self)))
                .collect::<Vec<_>>()
                .into_boxed_slice()
        })
    }
    pub fn fqtn(&'a self) -> Fqtn<&str> {
        <Self as MessageOrEnumExt>::fqtn(self)
    }
    pub fn file(&'a self) -> &File<'_> {
        self.file.get_or_init(|| {
            let mut m = self;
            loop {
                match m.parent() {
                    FileOrMessageRef::File(f) => break f,
                    FileOrMessageRef::Message(pm) => m = pm,
                }
            }
        })
    }
}
impl Deref for Message<'_> {
    type Target = DescriptorProto;
    fn deref(&self) -> &Self::Target {
        &self.proto
    }
}

#[derive(Debug, Clone)]
pub struct Enum<'a> {
    proto: &'a EnumDescriptorProto,
    parent: FileOrMessageRef<'a>,
    fqtn: OnceCell<Fqtn<String>>,
}
impl<'a> Enum<'a> {
    pub fn new(proto: &'a EnumDescriptorProto, parent: FileOrMessageRef<'a>) -> Self {
        Self {
            proto,
            parent,
            fqtn: OnceCell::default(),
        }
    }
    pub fn proto(&'a self) -> &EnumDescriptorProto {
        &self.proto
    }
    pub fn parent(&'a self) -> FileOrMessageRef<'_> {
        self.parent.clone()
    }
    pub fn fqtn(&'a self) -> Fqtn<&str> {
        <Self as MessageOrEnumExt>::fqtn(self)
    }
}
impl Deref for Enum<'_> {
    type Target = EnumDescriptorProto;
    fn deref(&self) -> &Self::Target {
        &self.proto
    }
}

#[derive(Debug, Clone)]
pub struct Field<'a> {
    proto: &'a FieldDescriptorProto,
    parent: &'a Message<'a>,
    fqtn: OnceCell<Option<Fqtn<String>>>,
}
impl<'a> Field<'a> {
    pub fn new(proto: &'a FieldDescriptorProto, parent: &'a Message<'a>) -> Self {
        Self {
            proto,
            parent,
            fqtn: OnceCell::new(),
        }
    }
    pub fn proto(&self) -> &FieldDescriptorProto {
        &self.proto
    }
    pub fn parent(&'a self) -> &Message<'_> {
        self.parent
    }
    pub fn fqtn(&'a self) -> &Option<Fqtn<String>> {
        self.fqtn.get_or_init(|| {
            self.proto()
                .type_name_opt()
                .map(|tn| Fqtn::new(tn).to_owned())
        })
    }
}
impl Deref for Field<'_> {
    type Target = FieldDescriptorProto;
    fn deref(&self) -> &Self::Target {
        &self.proto
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum FileOrMessageRef<'a> {
    File(&'a File<'a>),
    Message(&'a Message<'a>),
}

#[derive(Debug, Clone)]
pub enum MessageOrEnumRef<'a> {
    Message(&'a Message<'a>),
    Enum(&'a Enum<'a>),
}

trait MessageOrEnumExt<'a> {
    fn fqtn_once_cell(&'a self) -> &OnceCell<Fqtn<String>>;
    fn name(&'a self) -> &str;
    fn parent(&'a self) -> FileOrMessageRef<'_>;

    fn fqtn(&'a self) -> Fqtn<&str> {
        self.fqtn_once_cell()
            .get_or_init(|| {
                Fqtn::new(match self.parent() {
                    FileOrMessageRef::File(f) => format!(".{}.{}", f.package(), self.name()),
                    FileOrMessageRef::Message(m) => {
                        format!(".{}.{}", m.fqtn(), self.name())
                    }
                })
            })
            .as_ref()
    }
}
impl<'a> MessageOrEnumExt<'a> for Message<'a> {
    fn fqtn_once_cell(&'a self) -> &OnceCell<Fqtn<String>> {
        &self.fqtn
    }
    fn name(&'a self) -> &str {
        self.proto().name()
    }
    fn parent(&'a self) -> FileOrMessageRef<'_> {
        <Message>::parent(self)
    }
}
impl<'a> MessageOrEnumExt<'a> for Enum<'a> {
    fn fqtn_once_cell(&'a self) -> &OnceCell<Fqtn<String>> {
        &self.fqtn
    }
    fn name(&'a self) -> &str {
        self.proto().name()
    }
    fn parent(&'a self) -> FileOrMessageRef<'_> {
        <Enum>::parent(self)
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, Copy)]
pub enum Syntax {
    Proto2,
    Proto3,
}
