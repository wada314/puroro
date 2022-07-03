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
    type Fields: GetOwnedFields<Self> + FieldsIntoCountBits;
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
    type CountBits;
}
// Implementation note: Do not introduce any additional bounds except
// `MD: MessageDescriptor`!
impl<MD: MessageDescriptor> MessageDescriptorExt for MD {
    type Fields = MD::Fields;
    type Syntax = MD::Syntax;
    type OwnedFields = <MD::Fields as GetOwnedFields<MD>>::Type;
    type CountBits = <MD::Fields as FieldsIntoCountBits>::Type;
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
    list::Map<FdIntoOwnedTypeFunctor<MD>>: Func<Fields, Type = OwnedFields>,
{
    type Type = OwnedFields;
}

pub trait CountBits {
    const NUM_BITS: usize;
}

pub struct ConstBits<const NUM_BITS: usize>;
impl<const NUM_BITS: usize> CountBits for ConstBits<NUM_BITS> {
    const NUM_BITS: usize = NUM_BITS;
}

pub struct AddBits<T, U>(::std::marker::PhantomData<(T, U)>);
impl<T: CountBits, U: CountBits> CountBits for AddBits<T, U> {
    const NUM_BITS: usize = T::NUM_BITS + U::NUM_BITS;
}
pub struct AddBitsFunc;
impl<T: CountBits, U: CountBits> Func<(T, U)> for AddBitsFunc {
    type Type = AddBits<T, U>;
}

pub struct IntoBits1Func;
impl<T> Func<T> for IntoBits1Func {
    type Type = ConstBits<1>;
}

pub struct FieldsIntoCountBitsFunc;
impl<Fields, ListOf1, Sum> Func<Fields> for FieldsIntoCountBitsFunc
where
    list::Map<Const<ConstBits<1>>>: Func<Fields, Type = ListOf1>,
    list::Fold<ConstBits<0>, AddBitsFunc>: Func<ListOf1, Type=Sum>,
{
    type Type = Sum;
}

pub trait FieldsIntoCountBits {
    type Type;
}
impl<Fields> FieldsIntoCountBits for Fields
where
    FieldsIntoCountBitsFunc: Func<Fields>,
{
    type Type = <FieldsIntoCountBitsFunc as Func<Fields>>::Type;
}
