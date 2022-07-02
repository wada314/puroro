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

use super::owned::FdIntoOwnedTypeFunctor;
use crate::tags;
use ::metako::*;
use ::typenum;
use ::typenum::U0;

pub trait MessageDescriptor: Sized {
    type Fields: GetOwnedFields<Self>;
    type Syntax: tags::ProtoSyntaxTag;
}
impl MessageDescriptor for () {
    type Fields = ();
    type Syntax = ();
}
pub trait MessageDescriptorExt {
    type Fields;
    type Syntax: tags::ProtoSyntaxTag;
    type OwnedFields;
}
// Implementation note: Do not introduce any additional bounds except
// `MD: MessageDescriptor`!
impl<MD: MessageDescriptor> MessageDescriptorExt for MD {
    type Fields = MD::Fields;
    type Syntax = MD::Syntax;
    type OwnedFields = <MD::Fields as GetOwnedFields<MD>>::Type;
}

pub trait FieldDescriptor {
    type Number: typenum::ToInt<i32> + Number;
    type Label: tags::FieldLabelTag;
    type Type: tags::FieldTypeTag;
    type HasOneofIndex: Bool;
    type OneofIndex: Number;
    type IsProto3Optional: Bool;
}
impl FieldDescriptor for () {
    type Number = U0;
    type Label = ();
    type Type = ();
    type HasOneofIndex = B0;
    type OneofIndex = U0;
    type IsProto3Optional = B0;
}
pub trait FieldDescriptorExt {
    type Number: typenum::ToInt<i32> + Number;
    type Label: tags::FieldLabelTag;
    type Type: tags::FieldTypeTag;
    type HasOneofIndex: Bool;
    type OneofIndex: Number;
    type IsProto3Optional: Bool;
    type TypeId: Number;
    type LabelId: Number;
    type MaybeFieldMessageDescriptor: MessageDescriptorExt;
}
impl<FD: FieldDescriptor> FieldDescriptorExt for FD {
    type Number = FD::Number;
    type Label = FD::Label;
    type Type = FD::Type;
    type HasOneofIndex = FD::HasOneofIndex;
    type OneofIndex = FD::OneofIndex;
    type IsProto3Optional = FD::IsProto3Optional;
    type TypeId = <FD::Type as tags::FieldTypeTag>::Id;
    type LabelId = <FD::Label as tags::FieldLabelTag>::Id;
    type MaybeFieldMessageDescriptor = <FD::Type as tags::FieldTypeTag>::MessageDescriptor;
}

pub trait GetOwnedFields<MD> {
    type Type;
}
impl<MD, Fields, OwnedFields> GetOwnedFields<MD> for Fields
where
    list::IntoTupleList: Func<list::Map<Fields, FdIntoOwnedTypeFunctor<MD>>, Type = OwnedFields>,
{
    type Type = OwnedFields;
}
