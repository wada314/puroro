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

use super::super::desc::{FieldDescriptorExt, MessageDescriptor};
use ::metako::*;
use ::std::marker::PhantomData;

pub struct MdFdIntoOptBoxOwnedMessageFunctor;
impl<MD, FD> Func<(MD, FD)> for MdFdIntoOptBoxOwnedMessageFunctor
where
    FD: FieldDescriptorExt,
{
    type Type = Option<Box<super::OwnedMessage<FD::MaybeFieldMessageDescriptor>>>;
}

mod preds {
    use super::{FieldDescriptorExt, MessageDescriptor};
    use crate::tags;
    use ::metako::{list, make_list, AllOf, AnyOf, Func, IsNumberEqualFunctor, Not, Number};

    pub struct IsUnit;
    impl<MD: MessageDescriptor, FD: FieldDescriptorExt> Func<(MD, FD)> for IsUnit {
        // if fd.has_oneof_index() && !fd.proto3_optional()
        type Type = AllOf<make_list![FD::HasOneofIndex, Not<FD::IsProto3Optional>,]>;
    }
    pub struct IsU32;
    impl<MD: MessageDescriptor, FD: FieldDescriptorExt> Func<(MD, FD)> for IsU32 {
        type Type = AllOf<
            make_list![
                AnyOf<
                    list::Map<
                        make_list![tags::UInt32Id, tags::Fixed32Id],
                        IsNumberEqualFunctor<FD::TypeId>,
                    >,
                >,
                Not<<FD::LabelId as Number>::Eq<tags::RepeatedId>>,
            ],
        >;
    }
    pub struct IsString;
    impl<MD: MessageDescriptor, FD: FieldDescriptorExt> Func<(MD, FD)> for IsString {
        type Type = AllOf<
            make_list![
                <FD::TypeId as Number>::Eq<tags::StringId>,
                Not<<FD::LabelId as Number>::Eq<tags::RepeatedId>>,
            ],
        >;
    }
    pub struct IsOptBoxedMessage;
    impl<MD: MessageDescriptor, FD: FieldDescriptorExt> Func<(MD, FD)> for IsOptBoxedMessage {
        type Type = AllOf<
            make_list![
                <FD::TypeId as Number>::Eq<tags::MessageId>,
                Not<<FD::LabelId as Number>::Eq<tags::RepeatedId>>,
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

pub struct FdIntoOwnedTypeFunctor<MD>(PhantomData<MD>);
impl<MD, FD, IntoOwnedType, OwnedType> Func<FD> for FdIntoOwnedTypeFunctor<MD>
where
    SwitchFunctor<MdFdIntoOwnedTypeSwitch>: Func<(MD, FD), Type = IntoOwnedType>,
    IntoOwnedType: Func<(MD, FD), Type = OwnedType>,
{
    type Type = OwnedType;
}
