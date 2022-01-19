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

use crate::internal::bool::{False, True};
use ::std::marker::PhantomData;

/// A tag trait for types corresponding to the field's type.
/// e.g. Int32, Float, String, Message<M>
/// This type actually consist of two tags for generics specialization:
/// `wire_tag<value::value_tag>`.
pub trait FieldTypeTag {
    type DefaultValueType;
}

/// A `FieldTypeTag` which has wire type one of Variant, Bits32 or Bits64.
pub trait NumericalTypeTag {
    type NativeType: 'static + Default + PartialEq + Clone;
}

/// A `FieldTypeTag` which is either String or Bytes.
pub trait StringOrBytesTypeTag {
    type BorrowedType: ?Sized;
}

/// A tag trait for types corresponding to the field label.
/// e.g. Optional, Repeated, Required
pub trait FieldLabelTag {
    const DO_DEFAULT_CHECK: bool;
}

mod value {
    use super::{False, True};
    use ::std::marker::PhantomData;
    pub struct Int32;
    pub struct UInt32;
    pub struct SInt32;
    pub struct Int64;
    pub struct UInt64;
    pub struct SInt64;
    pub struct Bool;
    pub struct Enum2<E>(PhantomData<E>);
    pub struct Enum3<E>(PhantomData<E>);
    pub struct Float;
    pub struct Double;
    pub struct SFixed32;
    pub struct SFixed64;
    pub struct Fixed32;
    pub struct Fixed64;
    pub type Bytes = (False, (True, False));
    pub type String = (False, (False, True));
    pub type Message<M> = (True, PhantomData<M>);
}

type EmptyLd = (False, (False, False));
pub type Variant<V> = (EmptyLd, (True, PhantomData<V>));
pub type LengthDelimited<V> = (V, False);
pub type Bits32<V> = (EmptyLd, (False, (True, PhantomData<V>)));
pub type Bits64<V> = (EmptyLd, (False, (False, PhantomData<V>)));

pub type NonLdType<_1> = (EmptyLd, _1);
pub type NonMessageType<_1, _2> = ((False, _1), _2);
pub type StringOrBytesType<_1> = ((False, _1), False);

pub type Int32 = Variant<value::Int32>;
pub type SInt32 = Variant<value::SInt32>;
pub type UInt32 = Variant<value::UInt32>;
pub type Int64 = Variant<value::Int64>;
pub type SInt64 = Variant<value::SInt64>;
pub type UInt64 = Variant<value::UInt64>;
pub type Bool = Variant<value::Bool>;
pub type String = LengthDelimited<value::String>;
pub type Bytes = LengthDelimited<value::Bytes>;
pub type Float = Bits32<value::Float>;
pub type Fixed32 = Bits32<value::Fixed32>;
pub type SFixed32 = Bits32<value::SFixed32>;
pub type Double = Bits64<value::Double>;
pub type Fixed64 = Bits64<value::Fixed64>;
pub type SFixed64 = Bits64<value::SFixed64>;
pub type Enum2<E> = Variant<value::Enum2<E>>;
pub type Enum3<E> = Variant<value::Enum3<E>>;
pub type Message<M> = LengthDelimited<value::Message<M>>;

/// A repeated field, which is available in both proto2 and proto3.
pub type Repeated = (True, False);
/// Proto2 optional field || Proto3 explicitly optional marked field.
pub type Optional = (False, (False, (True, True)));
/// Only available in proto2.
pub type Required = (False, (False, (True, False)));
/// Proto3 unlabeled field.
pub type Unlabeled = (False, (True, False));
/// An item of oneof.
pub type OneofField = (False, (False, (False, False)));

pub type NonRepeatedLabel<_1> = (False, _1);
pub type NeedOptionalBitLabel<_1> = (False, (False, (True, _1)));

impl FieldTypeTag for Int32 {
    type DefaultValueType = i32;
}
impl FieldTypeTag for Int64 {
    type DefaultValueType = i64;
}
impl FieldTypeTag for UInt32 {
    type DefaultValueType = u32;
}
impl FieldTypeTag for UInt64 {
    type DefaultValueType = u64;
}
impl FieldTypeTag for SInt32 {
    type DefaultValueType = i32;
}
impl FieldTypeTag for SInt64 {
    type DefaultValueType = i64;
}
impl FieldTypeTag for Bool {
    type DefaultValueType = bool;
}
impl FieldTypeTag for Bytes {
    type DefaultValueType = &'static [u8];
}
impl FieldTypeTag for String {
    type DefaultValueType = &'static str;
}
impl<E> FieldTypeTag for Enum2<E> {
    type DefaultValueType = E;
}
impl<E> FieldTypeTag for Enum3<E> {
    type DefaultValueType = E;
}
impl<M> FieldTypeTag for Message<M> {
    type DefaultValueType = (); // Never be instanciated
}
impl FieldTypeTag for Float {
    type DefaultValueType = f32;
}
impl FieldTypeTag for Double {
    type DefaultValueType = f64;
}
impl FieldTypeTag for Fixed32 {
    type DefaultValueType = u32;
}
impl FieldTypeTag for Fixed64 {
    type DefaultValueType = u64;
}
impl FieldTypeTag for SFixed32 {
    type DefaultValueType = i32;
}
impl FieldTypeTag for SFixed64 {
    type DefaultValueType = i64;
}

impl NumericalTypeTag for Int32 {
    type NativeType = i32;
}
impl NumericalTypeTag for UInt32 {
    type NativeType = u32;
}
impl NumericalTypeTag for SInt32 {
    type NativeType = i32;
}
impl NumericalTypeTag for Fixed32 {
    type NativeType = u32;
}
impl NumericalTypeTag for SFixed32 {
    type NativeType = i32;
}
impl NumericalTypeTag for Float {
    type NativeType = f32;
}
impl NumericalTypeTag for Int64 {
    type NativeType = i64;
}
impl NumericalTypeTag for UInt64 {
    type NativeType = u64;
}
impl NumericalTypeTag for SInt64 {
    type NativeType = i64;
}
impl NumericalTypeTag for Fixed64 {
    type NativeType = u64;
}
impl NumericalTypeTag for SFixed64 {
    type NativeType = i64;
}
impl NumericalTypeTag for Double {
    type NativeType = f64;
}
impl NumericalTypeTag for Bool {
    type NativeType = bool;
}
impl<E: crate::Enum2> NumericalTypeTag for Enum2<E> {
    type NativeType = E;
}
impl<E: crate::Enum3> NumericalTypeTag for Enum3<E> {
    type NativeType = E;
}
impl StringOrBytesTypeTag for String {
    type BorrowedType = str;
}
impl StringOrBytesTypeTag for Bytes {
    type BorrowedType = [u8];
}

impl FieldLabelTag for Repeated {
    const DO_DEFAULT_CHECK: bool = false;
}
impl FieldLabelTag for Optional {
    const DO_DEFAULT_CHECK: bool = false;
}
impl FieldLabelTag for Unlabeled {
    const DO_DEFAULT_CHECK: bool = true;
}
impl FieldLabelTag for Required {
    const DO_DEFAULT_CHECK: bool = false;
}
impl FieldLabelTag for OneofField {
    const DO_DEFAULT_CHECK: bool = false;
}

pub struct SimpleImpl;
