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

use super::super::desc::{FieldDescriptorExt, MessageDescriptor, Usize, UsizeAdd, UsizeValue};
use super::field_type::{OptionalOwnedField, ScalarMessageOwnedField};
use super::{OwnedField, OwnedMessage};
use ::metako::*;
use ::std::marker::PhantomData;

pub struct MdFdIntoMessageFieldFunctor;
impl<MD, FD> Func<(MD, FD)> for MdFdIntoMessageFieldFunctor
where
    FD: FieldDescriptorExt,
{
    type Type = ScalarMessageOwnedField<super::OwnedMessage<FD::MaybeFieldMessageDescriptor>, 999>;
}

pub struct IncrementNumber<const N: usize>;
impl<X: UsizeValue, const N: usize> Func<X> for IncrementNumber<N> {
    type Type = UsizeAdd<Usize<N>, X>;
}

mod preds {
    use super::{FieldDescriptorExt, MessageDescriptor};
    use crate::tags;
    use ::metako::{list, make_list, AllOf, AnyOf, Func, IsNumberEqual, Not, Number};

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
                    <list::Map<
                        IsNumberEqual<FD::TypeId>,
                    > as Func<make_list![tags::UInt32Id, tags::Fixed32Id]>>::Type,
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
    (preds::IsUnit, (Const<()>, IncrementNumber<0>)),
    (
        preds::IsU32,
        (OptionalFieldTypeGen<u32>, IncrementNumber<1>)
    ),
    (
        preds::IsString,
        (OptionalFieldTypeGen<String>, IncrementNumber<1>)
    ),
    (
        preds::IsOptBoxedMessage,
        (ScalarMessageFieldTypeGen, IncrementNumber<0>)
    ),
    // default
    (Const<B1>, (Const<()>, IncrementNumber<0>)),
];

pub struct OptionalFieldTypeGen<T>(PhantomData<T>);
impl<T, MD, FD, const BITFEILD_INDEX: usize> Func<(MD, FD, [(); BITFEILD_INDEX])>
    for OptionalFieldTypeGen<T>
{
    type Type = OptionalOwnedField<T, BITFEILD_INDEX>;
}
pub struct ScalarMessageFieldTypeGen;
impl<MD, FD, const BITFEILD_INDEX: usize> Func<(MD, FD, [(); BITFEILD_INDEX])>
    for ScalarMessageFieldTypeGen
where
    FD: FieldDescriptorExt,
{
    type Type =
        ScalarMessageOwnedField<OwnedMessage<FD::MaybeFieldMessageDescriptor>, BITFEILD_INDEX>;
}

pub struct FdIntoOwnedTypeFunctor<MD>(PhantomData<MD>);
impl<MD, FD, IntoOwnedType, OwnedType, NextBitfieldIndex, const BITFIELD_INDEX: usize>
    Func<(Usize<BITFIELD_INDEX>, FD)> for FdIntoOwnedTypeFunctor<MD>
where
    Switch<MdFdIntoOwnedTypeSwitch>: Func<(MD, FD), Type = (IntoOwnedType, NextBitfieldIndex)>,
    IntoOwnedType: Func<(MD, FD, Usize<BITFIELD_INDEX>), Type = OwnedType>,
    // OwnedType: OwnedField,
    NextBitfieldIndex: Func<Usize<BITFIELD_INDEX>>,
{
    type Type = (
        <NextBitfieldIndex as Func<Usize<BITFIELD_INDEX>>>::Type,
        OwnedType,
    );
}
