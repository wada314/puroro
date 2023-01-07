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

use super::{Enum, Message, MessageOrEnumCase, Syntax};
use crate::{ErrorKind, Result};
use ::puroro_protobuf_compiled::google::protobuf::field_descriptor_proto;
use ::std::rc::{Rc, Weak};

#[derive(Debug, Clone)]
pub enum FieldType {
    Variant(VariantType),
    LengthDelimited(LengthDelimitedType),
    Bits32(Bits32Type),
    Bits64(Bits64Type),
}
#[derive(Debug, Clone)]
pub enum VariantType {
    Int32,
    UInt32,
    SInt32,
    Int64,
    UInt64,
    SInt64,
    Bool,
    Enum2(Weak<dyn Enum>),
    Enum3(Weak<dyn Enum>),
}
#[derive(Debug, Clone)]
pub enum LengthDelimitedType {
    String,
    Bytes,
    Message(Weak<dyn Message>),
}
#[derive(Debug, Clone)]
pub enum Bits32Type {
    Fixed32,
    SFixed32,
    Float,
}
#[derive(Debug, Clone)]
pub enum Bits64Type {
    Fixed64,
    SFixed64,
    Double,
}

impl FieldType {
    pub fn try_new(
        type_opt: Option<field_descriptor_proto::Type>,
        type_name: &str,
        syntax: Syntax,
        message: Rc<dyn Message>,
    ) -> Result<Self> {
        use field_descriptor_proto::Type::*;
        use Bits32Type::*;
        use Bits64Type::*;
        use FieldType::*;
        use LengthDelimitedType::*;
        use VariantType::*;

        let maybe_m_or_e = (!type_name.is_empty())
            .then(|| -> Result<_> { Ok(message.resolve_type_name(type_name)?) })
            .transpose()?;

        if let Some(r#type) = type_opt {
            Ok(match r#type {
                TypeInt32 => Variant(Int32),
                TypeUint32 => Variant(UInt32),
                TypeSint32 => Variant(SInt32),
                TypeInt64 => Variant(Int64),
                TypeUint64 => Variant(UInt64),
                TypeSint64 => Variant(SInt64),
                TypeBool => Variant(Bool),
                TypeEnum => {
                    if let Some(MessageOrEnumCase::Enum(e)) = maybe_m_or_e {
                        match syntax {
                            Syntax::Proto2 => Variant(Enum2(Rc::downgrade(&e))),
                            Syntax::Proto3 => Variant(Enum3(Rc::downgrade(&e))),
                        }
                    } else {
                        Err(ErrorKind::UnknownTypeName {
                            name: type_name.to_string(),
                        })?
                    }
                }
                TypeFixed32 => Bits32(Fixed32),
                TypeSfixed32 => Bits32(SFixed32),
                TypeFloat => Bits32(Float),
                TypeFixed64 => Bits64(Fixed64),
                TypeSfixed64 => Bits64(SFixed64),
                TypeDouble => Bits64(Double),
                TypeString => LengthDelimited(String),
                TypeBytes => LengthDelimited(Bytes),
                TypeGroup => Err(ErrorKind::GroupNotSupported)?,
                TypeMessage => {
                    if let Some(MessageOrEnumCase::Message(m)) = maybe_m_or_e {
                        LengthDelimited(Message(Rc::downgrade(&m)))
                    } else {
                        Err(ErrorKind::UnknownTypeName {
                            name: type_name.to_string(),
                        })?
                    }
                }
            })
        } else if let Some(m_or_e) = maybe_m_or_e {
            Ok(match m_or_e {
                MessageOrEnumCase::Message(m) => LengthDelimited(Message(Rc::downgrade(&m))),
                MessageOrEnumCase::Enum(e) => match syntax {
                    Syntax::Proto2 => Variant(Enum2(Rc::downgrade(&e))),
                    Syntax::Proto3 => Variant(Enum3(Rc::downgrade(&e))),
                },
            })
        } else {
            Err(ErrorKind::MissingTypeName)?
        }
    }
}
