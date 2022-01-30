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

pub mod bool;
pub mod fixed_bits;
pub mod impls;
pub mod methods;
pub mod types;
pub mod utils;
pub mod variant;

pub use impls::bumpalo::AddBumpVecView;
pub use impls::bumpalo::NoAllocBox as NoAllocBumpBox;
pub use impls::bumpalo::NoAllocString as NoAllocBumpString;
pub use impls::bumpalo::NoAllocVec as NoAllocBumpVec;
pub use impls::bumpalo::RefMutString as RefMutBumpString;
pub use impls::bumpalo::RefMutVec as RefMutBumpVec;
pub use impls::simple::{SimpleFields, SimpleShared};

use crate::tags;
use crate::MessageImpl;
use ::bitvec::array::BitArray;
use ::bitvec::order::BitOrder;
use ::bitvec::slice::BitSlice;
use ::bitvec::view::BitViewSized;

pub trait Bitfield {
    fn get(&self, index: usize) -> bool;
    fn set(&mut self, index: usize, val: bool);
}
impl<O: BitOrder, V: BitViewSized> Bitfield for BitArray<O, V> {
    fn get(&self, index: usize) -> bool {
        <BitSlice<_, _>>::get(&**self, index).map_or(false, |b| *b)
    }
    fn set(&mut self, index: usize, val: bool) {
        <BitSlice<_, _>>::set(&mut **self, index, val);
    }
}

impl Bitfield for () {
    fn get(&self, _: usize) -> bool {
        false
    }
    fn set(&mut self, _: usize, _: bool) {
        unimplemented!()
    }
}

pub struct FlipBitOn<Base, const INDEX: usize>(Base);
impl<Base: Bitfield, const INDEX: usize> Bitfield for FlipBitOn<Base, INDEX> {
    fn get(&self, index: usize) -> bool {
        index == INDEX || self.0.get(index)
    }
    fn set(&mut self, _: usize, _: bool) {
        unimplemented!()
    }
}

pub trait MessageProperties {
    const BITFIELD_OPTIONAL_FIELD_COUNT: usize;
    type Fields<const NUMBER: i32>;
}
pub trait FieldProperties {
    type LabelTag: tags::FieldLabelTag;
    type TypeTag: tags::FieldTypeTag;
    const DEFAULT_VALUE: <Self::TypeTag as tags::FieldTypeTag>::DefaultValueType;
    const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
}

pub trait ImplProperties {
    type ImplTag;
    type FieldsType;
    type SharedType;
}

pub trait FieldsContainer {}

pub trait HasField<const NUMBER: i32>: FieldsContainer {
    type Type;
    fn get(&self) -> &Self::Type;
}

pub trait SharedBitfield {
    type BitfieldType: Bitfield;
    fn bitfield(&self) -> &Self::BitfieldType;
    fn bitfield_mut(&mut self) -> &mut Self::BitfieldType;
}
pub trait SharedAllocator {
    type AllocatorType;
    fn alloc(&self) -> &Self::AllocatorType;
}

#[derive(Default, Clone)]
pub struct EmptyFields;
impl FieldsContainer for EmptyFields {}
impl<const NUMBER: i32> HasField<NUMBER> for EmptyFields {
    type Type = ();
    fn get(&self) -> &Self::Type {
        &()
    }
}

#[macro_export]
macro_rules! define_fields_container {
    ($(#[$attr:meta])* struct $container:ident $(<$lt:lifetime>)? {
        $($name:ident: $ty:ty = $number:literal,)*
    }) => {
        $(#[$attr])*
        pub struct $container $(<$lt>)? {
            $($name: $ty,)*
            $(_phantom: ::std::marker::PhantomData<&$lt ()>,)?
        }
        impl $(<$lt>)? $crate::internal::FieldsContainer for self::$container $(<$lt>)? {}
        define_fields_container!(@impls $container, $($lt)?, $($name : $ty = $number,)*);
    };
    (@impls $container:ident, $($lt:lifetime)?, $name:ident: $ty:ty = $number:literal, $($rest:tt)*) => {
        impl$(<$lt>)? $crate::internal::HasField<$number> for self::$container $(<$lt>)? {
            type Type = $ty;
            fn get(&self) -> &Self::Type {
                &self.$name
            }
        }
        define_fields_container!(@impls $container, $($lt)?, $($rest)*);
    };
    (@impls $container:ident, $($lt:lifetime)?, ) => {};
}

#[macro_export]
macro_rules! define_getter {
    ($pub:vis fn $id:ident<$num:literal>(&$($lt:lifetime)? self)) => {
        $pub fn $id(&$($lt)*self) -> <<Self as $crate::AsMessageRef>::MessageType as GetFieldMethod<$($lt, )* $num>>::GetterType {
            <<Self as $crate::AsMessageRef>::MessageType as GetFieldMethod<$num>>::get(
                self.as_message_ref(),
            )
        }
    };
}

#[macro_export]
macro_rules! define_opt_getter {
    ($pub:vis fn $id:ident<$num:literal>(&$($lt:lifetime)? self)) => {
        $pub fn $id(&$($lt)*self) -> <<Self as $crate::AsMessageRef>::MessageType as GetOptFieldMethod<$($lt, )* $num>>::GetterType {
            <<Self as $crate::AsMessageRef>::MessageType as GetOptFieldMethod<$num>>::get_opt(
                self.as_message_ref(),
            )
        }
    };
}

#[macro_export]
macro_rules! impl_getter {
    ($struct:ident, $pub:vis fn $get:ident<$num:literal>(&$($lt:lifetime)? self)) => {
        impl<Impl> $struct<Impl>
        where
            Self: $crate::AsMessageRef,
            for <'a> <Self as $crate::AsMessageRef>::MessageType: GetFieldMethod<'a, $num>,
            Impl: $crate::internal::ImplProperties,
        {
            define_getter!($pub fn $get<$num>(&$($lt)*self));
        }
    };
}
#[macro_export]
macro_rules! impl_opt_getter {
    ($struct:ident, $pub:vis fn $get:ident<$num:literal>(&$($lt:lifetime)? self)) => {
        impl<Impl> $struct<Impl>
        where
            Self: $crate::AsMessageRef,
            for<'a> <Self as $crate::AsMessageRef>::MessageType: GetOptFieldMethod<'a, $num>,
            Impl: $crate::internal::ImplProperties,
        {
            define_opt_getter!($pub fn $get<$num>(&$($lt)*self));
        }
    };
}
#[macro_export]
macro_rules! impl_field_properties {
    ($fp:ty, $ltag_id:ident, $ttag_id:ident $(<$ttag_param:ty>)?, $default:expr, $opt_idx:expr) => {
        impl $crate::internal::FieldProperties for $fp {
            type LabelTag = $crate::tags::$ltag_id;
            type TypeTag = $crate::tags::$ttag_id$(<$ttag_param>)?;
            const DEFAULT_VALUE: <Self::TypeTag as $crate::tags::FieldTypeTag>::DefaultValueType =
                $default;
            const OPTIONAL_FIELD_BITFIELD_INDEX: usize = $opt_idx;
        }
    };
}
