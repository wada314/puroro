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
// limitations under the LicensMessageDescriptor;

use crate::reflection::r#static::desc::MessageDescriptorBase;
use ::metako::Number;
use ::std::marker::PhantomData;
use ::typenum::consts::*;

pub trait ProtoSyntaxTag {
    type Id: Number;
}

/// A tag trait for types corresponding to the field's type.
/// e.g. Int32, Float, String, Message<M>
/// This type actually consist of two tags for generics specialization:
/// `wire_tag<value::value_tag>`.
pub trait FieldTypeTag {
    type Id: Number;
    // If the type is message or enum, the `MessageDescriptor` or `EnumDescriptor`
    // for the type should also be provided. otherwise `()`.
    type MessageDescriptor: MessageDescriptorBase;
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
    type Id: Number;
}

pub struct Proto2;
pub struct Proto3;

impl ProtoSyntaxTag for () {
    type Id = U0;
}
impl ProtoSyntaxTag for Proto2 {
    type Id = U2;
}
impl ProtoSyntaxTag for Proto3 {
    type Id = U3;
}
pub type Proto2Id = <Proto2 as ProtoSyntaxTag>::Id;
pub type Proto3Id = <Proto3 as ProtoSyntaxTag>::Id;

mod value {
    use crate::reflection::r#static::desc::MessageDescriptorBase;
    use ::std::marker::PhantomData;

    pub struct Int32;
    pub struct UInt32;
    pub struct SInt32;
    pub struct Int64;
    pub struct UInt64;
    pub struct SInt64;
    pub struct Bool;
    pub struct Enum<E>(PhantomData<E>);
    pub struct Float;
    pub struct Double;
    pub struct SFixed32;
    pub struct SFixed64;
    pub struct Fixed32;
    pub struct Fixed64;
    pub struct Bytes;
    pub struct String;
    pub struct Message<M: MessageDescriptorBase>(PhantomData<M>);
}

pub struct Variant<V>(PhantomData<V>);
pub struct LengthDelimited<V>(PhantomData<V>);
pub struct Bits32<V>(PhantomData<V>);
pub struct Bits64<V>(PhantomData<V>);

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
pub type Enum<E> = Variant<value::Enum<E>>;
pub type Message<M> = LengthDelimited<value::Message<M>>;

/// A repeated field, which is available in both proto2 and proto3.
pub struct Repeated;
/// Only available in proto2.
pub struct Required;
/// Every fields except `Repeated` and `Required`.
pub struct Optional;

impl FieldTypeTag for () {
    type Id = U0;
    type MessageDescriptor = ();
}
impl FieldTypeTag for Int32 {
    type Id = U1;
    type MessageDescriptor = ();
}
impl FieldTypeTag for Int64 {
    type Id = U2;
    type MessageDescriptor = ();
}
impl FieldTypeTag for UInt32 {
    type Id = U3;
    type MessageDescriptor = ();
}
impl FieldTypeTag for UInt64 {
    type Id = U4;
    type MessageDescriptor = ();
}
impl FieldTypeTag for SInt32 {
    type Id = U5;
    type MessageDescriptor = ();
}
impl FieldTypeTag for SInt64 {
    type Id = U6;
    type MessageDescriptor = ();
}
impl FieldTypeTag for Bool {
    type Id = U7;
    type MessageDescriptor = ();
}
impl FieldTypeTag for Bytes {
    type Id = U8;
    type MessageDescriptor = ();
}
impl FieldTypeTag for String {
    type Id = U9;
    type MessageDescriptor = ();
}
impl<E> FieldTypeTag for Enum<E> {
    type Id = U10;
    type MessageDescriptor = ();
}
impl<M: MessageDescriptorBase> FieldTypeTag for Message<M> {
    type Id = U12;
    type MessageDescriptor = M;
}
impl FieldTypeTag for Float {
    type Id = U13;
    type MessageDescriptor = ();
}
impl FieldTypeTag for Double {
    type Id = U14;
    type MessageDescriptor = ();
}
impl FieldTypeTag for Fixed32 {
    type Id = U15;
    type MessageDescriptor = ();
}
impl FieldTypeTag for Fixed64 {
    type Id = U16;
    type MessageDescriptor = ();
}
impl FieldTypeTag for SFixed32 {
    type Id = U17;
    type MessageDescriptor = ();
}
impl FieldTypeTag for SFixed64 {
    type Id = U18;
    type MessageDescriptor = ();
}
pub type Int32Id = <Int32 as FieldTypeTag>::Id;
pub type SInt32Id = <SInt32 as FieldTypeTag>::Id;
pub type UInt32Id = <UInt32 as FieldTypeTag>::Id;
pub type Int64Id = <Int64 as FieldTypeTag>::Id;
pub type SInt64Id = <SInt64 as FieldTypeTag>::Id;
pub type UInt64Id = <UInt64 as FieldTypeTag>::Id;
pub type BoolId = <Bool as FieldTypeTag>::Id;
pub type StringId = <String as FieldTypeTag>::Id;
pub type BytesId = <Bytes as FieldTypeTag>::Id;
pub type FloatId = <Float as FieldTypeTag>::Id;
pub type Fixed32Id = <Fixed32 as FieldTypeTag>::Id;
pub type SFixed32Id = <SFixed32 as FieldTypeTag>::Id;
pub type DoubleId = <Double as FieldTypeTag>::Id;
pub type Fixed64Id = <Fixed64 as FieldTypeTag>::Id;
pub type SFixed64Id = <SFixed64 as FieldTypeTag>::Id;
pub type EnumId = <Enum<()> as FieldTypeTag>::Id;
pub type MessageId = <Message<()> as FieldTypeTag>::Id;

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
impl<E> NumericalTypeTag for Enum<E> {
    type NativeType = (); // TODO
}
impl StringOrBytesTypeTag for String {
    type BorrowedType = str;
}
impl StringOrBytesTypeTag for Bytes {
    type BorrowedType = [u8];
}

impl FieldLabelTag for () {
    type Id = U0;
}
impl FieldLabelTag for Repeated {
    type Id = U1;
}
impl FieldLabelTag for Optional {
    type Id = U2;
}
impl FieldLabelTag for Required {
    type Id = U3;
}
pub type RepeatedId = <Repeated as FieldLabelTag>::Id;
pub type OptionalId = <Optional as FieldLabelTag>::Id;
pub type RequiredId = <Required as FieldLabelTag>::Id;
