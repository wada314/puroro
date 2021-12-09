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

use crate::bumpalo::collections::Vec;
use crate::bumpalo::Bump;
use crate::internal::de::from_iter::{deser_from_scoped_iter, ScopedIter, Variants};
use crate::internal::de::DeserMessageFromBytesIter;
use crate::internal::fixed_bits::{Bits32TypeTag, Bits64TypeTag};
use crate::internal::types::FieldData;
use crate::internal::variant::VariantTypeTag;
use crate::internal::{NoAllocBumpString, NoAllocBumpVec, Bare};
use crate::BumpaloMessage;
use crate::ErrorKind;
use crate::{tags, Result};
use ::std::marker::PhantomData;

pub trait VecOrOptionOrBare<T> {
    fn push_in(&mut self, val: T, bump: &Bump);
    fn get_or_insert_with_in<F>(&mut self, f: F, bump: &Bump) -> &mut T
    where
        F: FnOnce() -> T;
    type Iter<'a>: Iterator<Item = &'a T>
    where
        Self: 'a,
        T: 'a;
    fn iter(&self) -> Self::Iter<'_>;
}
impl<T> VecOrOptionOrBare<T> for Option<T> {
    fn push_in(&mut self, val: T, _: &Bump) {
        *self = Some(val);
    }
    fn get_or_insert_with_in<F>(&mut self, f: F, _: &Bump) -> &mut T
    where
        F: FnOnce() -> T,
    {
        <Option<T>>::get_or_insert_with(self, f)
    }
    type Iter<'a>
    where
        T: 'a,
    = ::std::option::Iter<'a, T>;
    fn iter(&self) -> Self::Iter<'_> {
        Option::iter(self)
    }
}
impl<T> VecOrOptionOrBare<T> for NoAllocBumpVec<T> {
    fn push_in(&mut self, val: T, bump: &Bump) {
        let mut mut_vec = unsafe { self.as_mut_vec_in(bump) };
        mut_vec.push(val);
    }
    fn get_or_insert_with_in<F>(&mut self, f: F, bump: &Bump) -> &mut T
    where
        F: FnOnce() -> T,
    {
        {
            let mut mut_vec = unsafe { self.as_mut_vec_in(bump) };
            mut_vec.push((f)());
        }
        self.last_mut().unwrap()
    }
    type Iter<'a>
    where
        T: 'a,
    = ::std::slice::Iter<'a, T>;
    fn iter(&self) -> <Self as VecOrOptionOrBare<T>>::Iter<'_> {
        <[T]>::iter(self)
    }
}
impl<T> VecOrOptionOrBare<T> for Bare<T> {
    fn push_in(&mut self, val: T, _: &Bump) {
        **self = val;
    }
    fn get_or_insert_with_in<F>(&mut self, _: F, _: &Bump) -> &mut T
    where
        F: FnOnce() -> T,
    {
        self
    }
    type Iter<'a>
    where
        T: 'a,
    = ::std::iter::Once<&'a T>;
    fn iter(&self) -> Self::Iter<'_> {
        ::std::iter::once(self)
    }
}

// deser from iter methods

pub struct DeserFieldFromBytesIter<L, V>(PhantomData<(L, V)>);

impl<L, V> DeserFieldFromBytesIter<L, tags::Variant<V>>
where
    L: tags::FieldLabelTag,
    tags::Variant<V>: VariantTypeTag,
{
    pub fn deser_field<'bump, FieldType, I>(
        field: &mut FieldType,
        input: FieldData<&mut ScopedIter<I>>,
        bump: &'bump Bump,
    ) -> Result<()>
    where
        FieldType: VecOrOptionOrBare<<tags::Variant<V> as tags::NumericalTypeTag>::NativeType>,
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        match input {
            FieldData::Variant(v) => {
                let native_value = v.to_native::<tags::Variant<V>>()?;
                if !L::DO_DEFAULT_CHECK || native_value != Default::default() {
                    field.push_in(native_value, bump);
                }
            }
            FieldData::LengthDelimited(iter) => {
                let variants = Variants::new(iter);
                let values = variants.map(|rv| rv.and_then(|v| v.to_native::<tags::Variant<V>>()));
                for rvalue in values {
                    let native_value = rvalue?;
                    if !L::DO_DEFAULT_CHECK || native_value != Default::default() {
                        field.push_in(native_value, bump);
                    }
                }
            }
            _ => Err(ErrorKind::UnexpectedWireType)?,
        }
        Ok(())
    }
}

impl<L, V> DeserFieldFromBytesIter<L, tags::Bits32<V>>
where
    L: tags::FieldLabelTag,
    tags::Bits32<V>: Bits32TypeTag,
{
    pub fn deser_field<'bump, FieldType, I>(
        field: &mut FieldType,
        input: FieldData<&mut ScopedIter<I>>,
        bump: &'bump Bump,
    ) -> Result<()>
    where
        FieldType: VecOrOptionOrBare<<tags::Bits32<V> as tags::NumericalTypeTag>::NativeType>,
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        if let FieldData::Bits32(bytes) = input {
            let native_value = tags::Bits32::<V>::from_array(bytes);
            if !L::DO_DEFAULT_CHECK || native_value != Default::default() {
                field.push_in(native_value, bump);
            }
        } else {
            Err(ErrorKind::UnexpectedWireType)?;
        }
        Ok(())
    }
}

impl<L, V> DeserFieldFromBytesIter<L, tags::Bits64<V>>
where
    L: tags::FieldLabelTag,
    tags::Bits64<V>: Bits64TypeTag,
{
    pub fn deser_field<'bump, FieldType, I>(
        field: &mut FieldType,
        input: FieldData<&mut ScopedIter<I>>,
        bump: &'bump Bump,
    ) -> Result<()>
    where
        FieldType: VecOrOptionOrBare<<tags::Bits64<V> as tags::NumericalTypeTag>::NativeType>,
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        if let FieldData::Bits64(bytes) = input {
            let native_value = tags::Bits64::<V>::from_array(bytes);
            if !L::DO_DEFAULT_CHECK || native_value != Default::default() {
                field.push_in(native_value, bump);
            }
        } else {
            Err(ErrorKind::UnexpectedWireType)?;
        }
        Ok(())
    }
}

impl<L> DeserFieldFromBytesIter<L, tags::String>
where
    L: tags::FieldLabelTag,
{
    pub fn deser_field<'bump, FieldType, I>(
        field: &mut FieldType,
        input: FieldData<&mut ScopedIter<I>>,
        bump: &'bump Bump,
    ) -> Result<()>
    where
        FieldType: VecOrOptionOrBare<NoAllocBumpString>,
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        if let FieldData::LengthDelimited(iter) = input {
            let mut bytes = NoAllocBumpVec::new_in(bump);
            {
                let mut mut_bytes = unsafe { bytes.as_mut_vec_in(bump) };
                if let Some(expected_size) = iter.size_hint().1 {
                    mut_bytes.reserve_exact(expected_size);
                }
                for rbyte in iter {
                    <Vec<u8>>::push(&mut mut_bytes, rbyte?);
                }
            }
            let string =
                NoAllocBumpString::from_utf8(bytes).map_err(|_| ErrorKind::InvalidUtf8Bumpalo())?;
            if !L::DO_DEFAULT_CHECK || !string.is_empty() {
                field.push_in(string, bump);
            }
        } else {
            Err(ErrorKind::UnexpectedWireType)?;
        }
        Ok(())
    }
}

impl<L> DeserFieldFromBytesIter<L, tags::Bytes>
where
    L: tags::FieldLabelTag,
{
    pub fn deser_field<'bump, FieldType, I>(
        field: &mut FieldType,
        input: FieldData<&mut ScopedIter<I>>,
        bump: &'bump Bump,
    ) -> Result<()>
    where
        FieldType: VecOrOptionOrBare<NoAllocBumpVec<u8>>,
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        if let FieldData::LengthDelimited(iter) = input {
            let mut bytes = NoAllocBumpVec::new_in(bump);
            {
                let mut mut_bytes = unsafe { bytes.as_mut_vec_in(bump) };
                if let Some(expected_size) = iter.size_hint().1 {
                    mut_bytes.reserve_exact(expected_size);
                }
                for rbyte in iter {
                    <Vec<u8>>::push(&mut mut_bytes, rbyte?);
                }
                // drop mut_bytes
            }
            if !L::DO_DEFAULT_CHECK || !bytes.is_empty() {
                field.push_in(bytes, bump);
            }
        } else {
            Err(ErrorKind::UnexpectedWireType)?;
        }
        Ok(())
    }
}

impl<L, M> DeserFieldFromBytesIter<L, tags::Message<M>>
where
    L: tags::FieldLabelTag,
    M: DeserMessageFromBytesIter,
{
    pub fn deser_field<'bump, FieldType, I>(
        field: &mut FieldType,
        input: FieldData<&mut ScopedIter<I>>,
        bump: &'bump Bump,
    ) -> Result<()>
    where
        FieldType: VecOrOptionOrBare<M>,
        I: Iterator<Item = ::std::io::Result<u8>>,
        M: BumpaloMessage<'bump>,
    {
        if let FieldData::LengthDelimited(mut iter) = input {
            let msg = field.get_or_insert_with_in(|| BumpaloMessage::new_in(bump), bump);
            deser_from_scoped_iter(msg, &mut iter)?;
        } else {
            Err(ErrorKind::UnexpectedWireType)?;
        }
        Ok(())
    }
}
