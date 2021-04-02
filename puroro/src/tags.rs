use std::{iter::FromIterator, marker::PhantomData};

use crate::{Deserializable, Mergeable};

pub trait FieldTypeTag {
    type SingularRustType;
    type RepeatedRustType: FromIterator<<Self as FieldTypeTag>::SingularRustType>;
}

pub struct Int32();
pub struct UInt32();
pub struct SInt32();
pub struct Int64();
pub struct UInt64();
pub struct SInt64();
pub struct Bool();
pub struct String<T>(PhantomData<T>)
where
    T: FromIterator<char>;
pub struct Message<T>(PhantomData<T>)
where
    T: Deserializable + Mergeable;
pub struct WithRepeatedType<T, R>(PhantomData<(T, R)>)
where
    T: FieldTypeTag,
    R: FromIterator<T::SingularRustType>;

impl FieldTypeTag for Int32 {
    type SingularRustType = i32;
    type RepeatedRustType = Vec<i32>;
}
impl FieldTypeTag for Int64 {
    type SingularRustType = i64;
    type RepeatedRustType = Vec<i64>;
}
impl FieldTypeTag for UInt32 {
    type SingularRustType = u32;
    type RepeatedRustType = Vec<u32>;
}
impl FieldTypeTag for UInt64 {
    type SingularRustType = u64;
    type RepeatedRustType = Vec<u64>;
}
impl FieldTypeTag for SInt32 {
    type SingularRustType = i32;
    type RepeatedRustType = Vec<i32>;
}
impl FieldTypeTag for SInt64 {
    type SingularRustType = i64;
    type RepeatedRustType = Vec<i64>;
}
impl FieldTypeTag for Bool {
    type SingularRustType = bool;
    type RepeatedRustType = Vec<bool>;
}
impl<T> FieldTypeTag for String<T>
where
    T: FromIterator<char>,
{
    type SingularRustType = T;
    type RepeatedRustType = Vec<T>;
}
impl<T> FieldTypeTag for Message<T>
where
    T: Deserializable + Mergeable,
{
    type SingularRustType = T;
    type RepeatedRustType = Vec<T>;
}
impl<T: FieldTypeTag, R: FromIterator<T::SingularRustType>> FieldTypeTag
    for WithRepeatedType<T, R>
{
    type SingularRustType = <T as FieldTypeTag>::SingularRustType;
    type RepeatedRustType = R;
}

pub trait WireTypeTag {}
pub struct Variant();
impl WireTypeTag for Variant {}
pub struct LengthDelimited();
impl WireTypeTag for LengthDelimited {}
pub struct Bits32();
impl WireTypeTag for Bits32 {}
pub struct Bits64();
impl WireTypeTag for Bits64 {}

pub trait FieldLabelTag {}
pub struct FieldLabelOptional();
impl FieldLabelTag for FieldLabelOptional {}
pub struct FieldLabelRepeated();
impl FieldLabelTag for FieldLabelRepeated {}
pub struct FieldLabelRequired();
impl FieldLabelTag for FieldLabelRequired {}
