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

use super::*;
use crate::Result;
use ::once_cell::unsync::OnceCell;
use ::puroro_protobuf_compiled::google::protobuf::{DescriptorProto, FieldDescriptorProto};

pub trait MessageTrait: Sized {
    fn try_new(proto: &DescriptorProto) -> Result<Self>;
}

#[derive(Debug)]
pub struct MessageImpl<EnumType, OneofType> {
    submessages: Vec<Self>,
    enums: Vec<EnumType>,
    oneofs: Vec<OneofType>,
    fields_proto: Box<[FieldDescriptorProto]>,
    fields: OnceCell<Box<[Field]>>,
}

pub type Message = MessageImpl<Enum, Oneof>;

impl<EnumType: EnumTrait, OneofType: OneofTrait> MessageTrait for MessageImpl<EnumType, OneofType> {
    fn try_new(proto: &DescriptorProto) -> Result<Self> {
        Ok(MessageImpl {
            submessages: proto
                .nested_type()
                .into_iter()
                .map(|m| Self::try_new(m))
                .collect::<Result<Vec<_>>>()?,
            enums: proto
                .enum_type()
                .into_iter()
                .map(|e| EnumType::try_new(e))
                .collect::<Result<Vec<_>>>()?,
            oneofs: proto
                .oneof_decl()
                .into_iter()
                .map(|o| OneofType::try_new(o))
                .collect::<Result<Vec<_>>>()?,
            fields_proto: proto.field().to_vec().into_boxed_slice(),
            fields: OnceCell::new(),
        })
    }
}

impl<EnumType, OneofType> MessageImpl<EnumType, OneofType> {
    pub fn submessages(&self) -> Result<&[Self]> {
        Ok(&self.submessages)
    }
    pub fn enums(&self) -> Result<&[EnumType]> {
        Ok(&self.enums)
    }
    pub fn oneofs(&self) -> Result<&[OneofType]> {
        Ok(&self.oneofs)
    }
    pub fn fields(&self) -> Result<&[Field]> {
        Ok(self.fields.get_or_try_init(|| -> Result<_> {
            Ok(self
                .fields_proto
                .into_iter()
                .map(|f| Field::try_new(f))
                .collect::<Result<Vec<_>>>()?
                .into_boxed_slice())
        })?)
    }
}
