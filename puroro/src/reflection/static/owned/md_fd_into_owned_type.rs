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

use super::super::desc::{FieldDescriptor, MessageDescriptorBase};
use super::boxed_message::BoxedMessage;
use crate::tags;
use ::metako::*;

pub struct MdFdIntoOptBoxOwnedMessage;
impl<MD, FD, FieldMD> Func<(MD, FD)> for MdFdIntoOptBoxOwnedMessage
where
    FD: FieldDescriptor,
    <FD as FieldDescriptor>::Type: tags::FieldTypeTag<MessageDescriptor = FieldMD>,
{
    type Type = Option<BoxedMessage<FieldMD>>;
}

mod preds {
    use super::{FieldDescriptor, MessageDescriptorBase};
    use crate::tags;
    use ::metako::{list, make_list, AllOf, AnyOf, Func, If, IsNumberEqual, Number};

    pub struct IsUnit;
    impl<MD: MessageDescriptorBase, FD: FieldDescriptor> Func<(MD, FD)> for IsUnit {
        // if fd.has_oneof_index() && !fd.proto3_optional()
        type Type = <AllOf as Func<
            make_list![
                FD::HasOneofIndex,
                <<FD as FieldDescriptor>::IsProto3Optional as If>::Not
            ],
        >>::Type;
    }
    pub struct IsU32;
    impl<MD: MessageDescriptorBase, FD: FieldDescriptor, TypeId> Func<(MD, FD)> for IsU32
    where
        FD::Type: tags::FieldTypeTag<Id = TypeId>,
        TypeId: Number,
    {
        type Type = <AllOf as Func<
            make_list![
                <AnyOf as Func<
                    <list::Map<IsNumberEqual<TypeId>> as Func<
                        make_list![tags::UInt32Id, tags::Fixed32Id],
                    >>::Type,
                >>::Type,
                <<FD::Label as tags::FieldLabelTag>::Id as Number>::Neq<tags::RepeatedId>,
            ],
        >>::Type;
    }
    pub struct IsString;
    impl<MD: MessageDescriptorBase, FD: FieldDescriptor> Func<(MD, FD)> for IsString {
        type Type = <AllOf as Func<
            make_list![
                <<FD::Type as tags::FieldTypeTag>::Id as Number>::Eq<tags::StringId>,
                <<FD::Label as tags::FieldLabelTag>::Id as Number>::Neq<tags::RepeatedId>,
            ],
        >>::Type;
    }
    pub struct IsOptBoxedMessage;
    impl<MD: MessageDescriptorBase, FD: FieldDescriptor> Func<(MD, FD)> for IsOptBoxedMessage {
        type Type = <AllOf as Func<
            make_list![
                <<FD::Type as tags::FieldTypeTag>::Id as Number>::Eq<tags::MessageId>,
                <<FD::Label as tags::FieldLabelTag>::Id as Number>::Neq<tags::RepeatedId>,
            ],
        >>::Type;
    }
}
pub struct MdFdIntoOwnedType;
type MdFdIntoOwnedTypeSwitch = make_list![
    (preds::IsUnit, Const<()>),
    (preds::IsU32, Const<u32>),
    (preds::IsString, Const<String>),
    (preds::IsOptBoxedMessage, MdFdIntoOptBoxOwnedMessage),
];
impl<MD, FD, Gen> Func<(MD, FD)> for MdFdIntoOwnedType
where
    Switch: Func<((MD, FD), MdFdIntoOwnedTypeSwitch), Type = Gen>,
    Gen: Func<(MD, FD)>,
{
    type Type = <Gen as Func<(MD, FD)>>::Type;
}
