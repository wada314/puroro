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

mod message;
mod numerical;
mod r#unsized;

pub use self::message::{RepeatedMessageField, SingularHeapMessageField};
pub use self::numerical::{OptionalNumericalField, RepeatedNumericalField, SingularNumericalField};
pub use self::r#unsized::{OptionalUnsizedField, RepeatedUnsizedField, SingularUnsizedField};

use crate::internal::bitvec::BitSlice;
use crate::internal::ser::{FieldData, ScopedIter, WireType};
use crate::internal::variant::Variant;
use crate::{PuroroError, Result};
use ::std::io::{Result as IoResult, Write};
use ::std::ops::Index;

pub trait FieldType {
    // Deserialization methods

    fn deser_from_field_data<'a, I: Iterator<Item = IoResult<u8>>, B: BitSlice>(
        &mut self,
        bitvec: &mut B,
        field_data: FieldData<ScopedIter<'a, I>>,
    ) -> Result<()> {
        match field_data {
            FieldData::Variant(v) => self.deser_from_variant(bitvec, v.clone()),
            FieldData::LengthDelimited(mut iter) => {
                self.deser_from_ld_scoped_iter(bitvec, &mut iter)?;
                iter.drop_and_check_scope_completed()?;
                Ok(())
            }
            FieldData::Bits32(bits) => self.deser_from_bits32(bitvec, bits.clone()),
            FieldData::Bits64(bits) => self.deser_from_bits64(bitvec, bits.clone()),
        }
    }
    fn deser_from_variant<B: BitSlice>(
        &mut self,
        #[allow(unused)] bitvec: &mut B,
        #[allow(unused)] variant: Variant,
    ) -> Result<()> {
        Err(PuroroError::InvalidWireType(WireType::Variant as u32))?
    }
    fn deser_from_bits32<B: BitSlice>(
        &mut self,
        #[allow(unused)] bitvec: &mut B,
        #[allow(unused)] bits: [u8; 4],
    ) -> Result<()> {
        Err(PuroroError::InvalidWireType(WireType::Bits32 as u32))?
    }
    fn deser_from_bits64<B: BitSlice>(
        &mut self,
        #[allow(unused)] bitvec: &mut B,
        #[allow(unused)] bits: [u8; 8],
    ) -> Result<()> {
        Err(PuroroError::InvalidWireType(WireType::Bits64 as u32))?
    }
    fn deser_from_ld_scoped_iter<'a, I: Iterator<Item = IoResult<u8>>, B: BitSlice>(
        &mut self,
        #[allow(unused)] bitvec: &mut B,
        #[allow(unused)] iter: &mut ScopedIter<'a, I>,
    ) -> Result<()> {
        Err(PuroroError::InvalidWireType(
            WireType::LengthDelimited as u32,
        ))?
    }

    // Serialization methods

    fn ser_to_write<W: Write, B: BitSlice>(
        &self,
        #[allow(unused)] bitvec: &B,
        #[allow(unused)] number: i32,
        #[allow(unused)] out: &mut W,
    ) -> Result<()>;
}

pub trait NonRepeatedFieldType: FieldType {
    /// An optional getter type, which is used by the message struct's
    /// getter methods.
    /// int32 => Option<i32>
    /// String => Option<&'a str>
    /// Message => Option<&'a Message>
    type GetterOptType<'a>
    where
        Self: 'a;

    /// A default field type which can be defined in proto2.
    /// int32 => i32
    /// String => &'static str
    /// Message => unreachable!()
    type DefaultValueType;

    /// A getter type, which overrides `Self::GetterOptType`'s `None` case
    /// by the `Self::DefaultValueType`. Exceptionally, message type cannot get
    /// this benefit so it's still an optional type.
    /// int32 => i32
    /// String => &'a str
    /// Message => Option<&'a Message>
    type GetterOrElseType<'a>
    where
        Self: 'a;

    /// A mutable getter type.
    /// int32 => &'a mut i32
    /// String => &'a mut String
    /// Message => &'a mut Message
    type GetterMutType<'a>
    where
        Self: 'a;

    fn get_field_or_else<'a, B: BitSlice, F: FnOnce() -> Self::DefaultValueType>(
        &'a self,
        bitvec: &B,
        default: F,
    ) -> Self::GetterOrElseType<'a>;
    fn get_field_opt<B: BitSlice>(&self, bitvec: &B) -> Self::GetterOptType<'_>;
    fn get_field_mut<'a, B: BitSlice, F: FnOnce() -> Self::DefaultValueType>(
        &'a mut self,
        bitvec: &mut B,
        default: F,
    ) -> Self::GetterMutType<'a>;
    fn clear<B: BitSlice>(&mut self, bitvec: &mut B);
}

pub trait RepeatedFieldType: FieldType {
    type ScalarType;
    fn get_field<B: BitSlice>(&self, bitvec: &B) -> &[Self::ScalarType];
    type ItemType<'a>
    where
        Self: 'a;
    type RepeatedRustType<'a>: IntoIterator<Item = Self::ItemType<'a>>
        + Index<usize, Output = Self::ItemType<'a>>
    where
        Self: 'a;
    fn get_field2<B: BitSlice>(&self, bitvec: &B) -> Self::RepeatedRustType<'_>;
    type ContainerType;
    fn get_field_mut<B: BitSlice>(&mut self, bitvec: &mut B) -> &mut Self::ContainerType;
    fn clear<B: BitSlice>(&mut self, bitvec: &mut B);
}
