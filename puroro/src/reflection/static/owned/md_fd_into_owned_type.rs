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

use super::super::desc::{FieldDescriptor, FieldDescriptorExt, MessageDescriptor};
use ::metako::*;

pub struct MdFdIntoOptBoxOwnedMessageFunctor;
impl<MD, FD> Functor<(MD, FD)> for MdFdIntoOptBoxOwnedMessageFunctor
where
    FD: FieldDescriptorExt,
{
    type Type = Option<Box<super::OwnedMessage<FD::MaybeFieldMessageDescriptor>>>;
}

mod preds {
    use super::{FieldDescriptor, MessageDescriptor};
    use crate::tags;
    use ::metako::{list, make_list, AllOf, AnyOf, Functor, IsNumberEqualFunctor, Not, Number};

    pub struct IsUnit;
    impl<MD: MessageDescriptor, FD: FieldDescriptor> Functor<(MD, FD)> for IsUnit {
        // if fd.has_oneof_index() && !fd.proto3_optional()
        type Type = AllOf<
            make_list![
                FD::HasOneofIndex,
                Not<<FD as FieldDescriptor>::IsProto3Optional>,
            ],
        >;
    }
    pub struct IsU32;
    impl<MD: MessageDescriptor, FD: FieldDescriptor, TypeId> Functor<(MD, FD)> for IsU32
    where
        FD::Type: tags::FieldTypeTag<Id = TypeId>,
        TypeId: Number,
    {
        type Type = AllOf<
            make_list![
                AnyOf<
                    list::Map2<
                        make_list![tags::UInt32Id, tags::Fixed32Id],
                        IsNumberEqualFunctor<TypeId>,
                    >,
                >,
                <<FD::Label as tags::FieldLabelTag>::Id as Number>::Neq<tags::RepeatedId>,
            ],
        >;
    }
    pub struct IsString;
    impl<MD: MessageDescriptor, FD: FieldDescriptor> Functor<(MD, FD)> for IsString {
        type Type = AllOf<
            make_list![
                <<FD::Type as tags::FieldTypeTag>::Id as Number>::Eq<tags::StringId>,
                <<FD::Label as tags::FieldLabelTag>::Id as Number>::Neq<tags::RepeatedId>,
            ],
        >;
    }
    pub struct IsOptBoxedMessage;
    impl<MD: MessageDescriptor, FD: FieldDescriptor> Functor<(MD, FD)> for IsOptBoxedMessage {
        type Type = AllOf<
            make_list![
                <<FD::Type as tags::FieldTypeTag>::Id as Number>::Eq<tags::MessageId>,
                <<FD::Label as tags::FieldLabelTag>::Id as Number>::Neq<tags::RepeatedId>,
            ],
        >;
    }
}
type MdFdIntoOwnedTypeSwitch = make_list![
    (preds::IsUnit, Const<()>),
    (preds::IsU32, Const<u32>),
    (preds::IsString, Const<String>),
    (preds::IsOptBoxedMessage, MdFdIntoOptBoxOwnedMessageFunctor),
    // default
    (Const<B1>, Const<()>),
];
pub trait MdFdIntoOwnedType {
    type Type;
}
impl<MdFd> MdFdIntoOwnedType for MdFd
where
    Self: Switch<MdFdIntoOwnedTypeSwitch>,
    <Self as Switch<MdFdIntoOwnedTypeSwitch>>::Type: Functor<Self>,
{
    type Type = <<Self as Switch<MdFdIntoOwnedTypeSwitch>>::Type as Functor<Self>>::Type;
}

pub struct MdFdIntoOwnedTypeFunctor;
impl<MdFd> Functor<MdFd> for MdFdIntoOwnedTypeFunctor
where
    MdFd: MdFdIntoOwnedType,
{
    type Type = <MdFd as MdFdIntoOwnedType>::Type;
}
