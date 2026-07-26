//! `puroro::{MapRef, MapEntryMut, MapMut, …}` impls for [`MapField`](super::MapField).

use ::core::ops::Deref;

use ::allocator_api2::alloc::Allocator;
use ::allocator_api2::vec::Vec as AllocVec;
use ::puroro::{MapEntryInsert, MapEntryMut, MapMut, MapRef, MapStrInsert, Message};

use crate::fields::shared::PresenceBits;
use crate::fields::wire::repeated_element::RepeatedElementMut;
use crate::fields::wire::{
    Closed, ClosedEnum, Open, OpenEnum, ProtoBool, ProtoBytes, ProtoDouble, ProtoEnum,
    ProtoFixed32, ProtoFixed64, ProtoFloat, ProtoInt32, ProtoInt64, ProtoMessage, ProtoSFixed32,
    ProtoSFixed64, ProtoSint32, ProtoSint64, ProtoString, ProtoUInt32, ProtoUInt64,
};

use super::{MapFieldMut, MapFieldRef};

// ---------------------------------------------------------------------------
// Copy scalar / bool values
// ---------------------------------------------------------------------------

macro_rules! impl_map_ref_copy {
    ($key_marker:ty, $key_view:ty, $val_marker:ty, $val_view:ty) => {
        impl<'a, const FIELD: u32, A, Pb> MapRef<$key_view, $val_view>
            for MapFieldRef<'a, $key_marker, $val_marker, FIELD, A, Pb>
        where
            A: Allocator + Clone,
            Pb: PresenceBits,
        {
            #[inline]
            fn len(&self) -> usize {
                self.field.len()
            }

            #[inline]
            fn get(&self, key: &$key_view) -> Option<&$val_view> {
                self.field.entries.get(key)
            }
        }
    };
}

macro_rules! impl_map_entry_copy {
    ($key_marker:ty, $key_view:ty, $val_marker:ty, $val_view:ty) => {
        impl<'f, 'c, const FIELD: u32, A, Pb> MapEntryMut<$key_view, $val_view>
            for MapFieldMut<'f, 'c, $key_marker, $val_marker, FIELD, A, Pb>
        where
            A: Allocator + Clone,
            Pb: PresenceBits,
        {
            type MutTarget = $val_view;
            type Mut<'a>
                = &'a mut $val_view
            where
                Self: 'a;

            #[inline]
            fn len(&self) -> usize {
                self.field.len()
            }

            #[inline]
            fn get(&self, key: &$key_view) -> Option<&$val_view> {
                self.field.entries.get(key)
            }

            #[inline]
            fn get_mut(&mut self, key: &$key_view) -> Option<Self::Mut<'_>> {
                self.get_element_mut(key)
            }

            #[inline]
            fn entry_mut(&mut self, key: $key_view) -> Self::Mut<'_> {
                self.entry_element_mut(key)
            }

            #[inline]
            fn remove(&mut self, key: &$key_view) {
                MapFieldMut::remove(self, key);
            }

            #[inline]
            fn clear(&mut self) {
                MapFieldMut::clear(self);
            }
        }

        impl<'f, 'c, const FIELD: u32, A, Pb> MapEntryInsert<$key_view, $val_view>
            for MapFieldMut<'f, 'c, $key_marker, $val_marker, FIELD, A, Pb>
        where
            A: Allocator + Clone,
            Pb: PresenceBits,
        {
            #[inline]
            fn insert(&mut self, key: $key_view, value: $val_view) {
                MapFieldMut::insert(self, key, value);
            }
        }
    };
}

macro_rules! impl_map_string_key_copy {
    ($val_marker:ty, $val_view:ty) => {
        impl_map_ref_copy!(ProtoString, str, $val_marker, $val_view);

        impl<'f, 'c, const FIELD: u32, A, Pb> MapMut<$val_view>
            for MapFieldMut<'f, 'c, ProtoString, $val_marker, FIELD, A, Pb>
        where
            A: Allocator + Clone,
            Pb: PresenceBits,
        {
            type MutTarget = $val_view;
            type Mut<'a>
                = &'a mut $val_view
            where
                Self: 'a;

            #[inline]
            fn len(&self) -> usize {
                self.field.len()
            }

            #[inline]
            fn get(&self, key: &str) -> Option<&$val_view> {
                self.field.entries.get(key)
            }

            #[inline]
            fn get_mut(&mut self, key: &str) -> Option<Self::Mut<'_>> {
                self.get_element_mut(key)
            }

            #[inline]
            fn entry_mut(&mut self, key: &str) -> Self::Mut<'_> {
                self.entry_element_mut_str(key)
            }

            #[inline]
            fn remove(&mut self, key: &str) {
                MapFieldMut::remove(self, key);
            }

            #[inline]
            fn clear(&mut self) {
                MapFieldMut::clear(self);
            }
        }

        impl<'f, 'c, const FIELD: u32, A, Pb> MapStrInsert<$val_view>
            for MapFieldMut<'f, 'c, ProtoString, $val_marker, FIELD, A, Pb>
        where
            A: Allocator + Clone,
            Pb: PresenceBits,
        {
            #[inline]
            fn insert_str(&mut self, key: &str, value: $val_view) {
                MapFieldMut::insert_str(self, key, value)
            }
        }
    };
}

macro_rules! for_each_copy_value {
    ($callback:ident $($fixed:tt)*) => {
        $callback!($($fixed)* ProtoDouble, f64);
        $callback!($($fixed)* ProtoFloat, f32);
        $callback!($($fixed)* ProtoInt64, i64);
        $callback!($($fixed)* ProtoUInt64, u64);
        $callback!($($fixed)* ProtoInt32, i32);
        $callback!($($fixed)* ProtoFixed64, u64);
        $callback!($($fixed)* ProtoFixed32, u32);
        $callback!($($fixed)* ProtoBool, bool);
        $callback!($($fixed)* ProtoUInt32, u32);
        $callback!($($fixed)* ProtoSFixed32, i32);
        $callback!($($fixed)* ProtoSFixed64, i64);
        $callback!($($fixed)* ProtoSint32, i32);
        $callback!($($fixed)* ProtoSint64, i64);
    };
}

macro_rules! impl_sized_key_copy_pair {
    ($key_marker:ty, $key_view:ty, $val_marker:ty, $val_view:ty) => {
        impl_map_ref_copy!($key_marker, $key_view, $val_marker, $val_view);
        impl_map_entry_copy!($key_marker, $key_view, $val_marker, $val_view);
    };
}

macro_rules! impl_all_copy_for_sized_key {
    ($key_marker:ty, $key_view:ty) => {
        for_each_copy_value!(impl_sized_key_copy_pair $key_marker, $key_view,);
    };
}

for_each_copy_value!(impl_map_string_key_copy);

impl_all_copy_for_sized_key!(ProtoInt32, i32);
impl_all_copy_for_sized_key!(ProtoInt64, i64);
impl_all_copy_for_sized_key!(ProtoUInt32, u32);
impl_all_copy_for_sized_key!(ProtoUInt64, u64);
impl_all_copy_for_sized_key!(ProtoSint32, i32);
impl_all_copy_for_sized_key!(ProtoSint64, i64);
impl_all_copy_for_sized_key!(ProtoFixed32, u32);
impl_all_copy_for_sized_key!(ProtoFixed64, u64);
impl_all_copy_for_sized_key!(ProtoSFixed32, i32);
impl_all_copy_for_sized_key!(ProtoSFixed64, i64);
impl_all_copy_for_sized_key!(ProtoBool, bool);

// ---------------------------------------------------------------------------
// string / bytes values
// ---------------------------------------------------------------------------

macro_rules! impl_map_ref_len {
    ($key_marker:ty, $key_view:ty, $val_marker:ty, $val_view:ty) => {
        impl<'a, const FIELD: u32, A, Pb> MapRef<$key_view, $val_view>
            for MapFieldRef<'a, $key_marker, $val_marker, FIELD, A, Pb>
        where
            A: Allocator + Clone,
            Pb: PresenceBits,
        {
            #[inline]
            fn len(&self) -> usize {
                self.field.len()
            }

            #[inline]
            fn get(&self, key: &$key_view) -> Option<&$val_view> {
                self.field.entries.get(key).map(|v| v.deref())
            }
        }
    };
}

macro_rules! impl_map_entry_string_val {
    ($key_marker:ty, $key_view:ty) => {
        impl_map_ref_len!($key_marker, $key_view, ProtoString, str);

        impl<'f, 'c, const FIELD: u32, A, Pb> MapEntryMut<$key_view, str>
            for MapFieldMut<'f, 'c, $key_marker, ProtoString, FIELD, A, Pb>
        where
            A: Allocator + Clone,
            Pb: PresenceBits,
        {
            type MutTarget = ::puroro::String<A>;
            type Mut<'a>
                = <ProtoString as RepeatedElementMut>::ElementMut<'a, A>
            where
                Self: 'a;

            #[inline]
            fn len(&self) -> usize {
                self.field.len()
            }

            #[inline]
            fn get(&self, key: &$key_view) -> Option<&str> {
                self.field.entries.get(key).map(|v| v.deref())
            }

            #[inline]
            fn get_mut(&mut self, key: &$key_view) -> Option<Self::Mut<'_>> {
                self.get_element_mut(key)
            }

            #[inline]
            fn entry_mut(&mut self, key: $key_view) -> Self::Mut<'_> {
                self.entry_element_mut(key)
            }

            #[inline]
            fn remove(&mut self, key: &$key_view) {
                MapFieldMut::remove(self, key);
            }

            #[inline]
            fn clear(&mut self) {
                MapFieldMut::clear(self);
            }
        }
    };
}

macro_rules! impl_map_entry_bytes_val {
    ($key_marker:ty, $key_view:ty) => {
        impl_map_ref_len!($key_marker, $key_view, ProtoBytes, [u8]);

        impl<'f, 'c, const FIELD: u32, A, Pb> MapEntryMut<$key_view, [u8]>
            for MapFieldMut<'f, 'c, $key_marker, ProtoBytes, FIELD, A, Pb>
        where
            A: Allocator + Clone,
            Pb: PresenceBits,
        {
            type MutTarget = AllocVec<u8, A>;
            type Mut<'a>
                = <ProtoBytes as RepeatedElementMut>::ElementMut<'a, A>
            where
                Self: 'a;

            #[inline]
            fn len(&self) -> usize {
                self.field.len()
            }

            #[inline]
            fn get(&self, key: &$key_view) -> Option<&[u8]> {
                self.field.entries.get(key).map(|v| v.deref())
            }

            #[inline]
            fn get_mut(&mut self, key: &$key_view) -> Option<Self::Mut<'_>> {
                self.get_element_mut(key)
            }

            #[inline]
            fn entry_mut(&mut self, key: $key_view) -> Self::Mut<'_> {
                self.entry_element_mut(key)
            }

            #[inline]
            fn remove(&mut self, key: &$key_view) {
                MapFieldMut::remove(self, key);
            }

            #[inline]
            fn clear(&mut self) {
                MapFieldMut::clear(self);
            }
        }
    };
}

macro_rules! for_each_sized_key {
    ($callback:ident) => {
        $callback!(ProtoInt32, i32);
        $callback!(ProtoInt64, i64);
        $callback!(ProtoUInt32, u32);
        $callback!(ProtoUInt64, u64);
        $callback!(ProtoSint32, i32);
        $callback!(ProtoSint64, i64);
        $callback!(ProtoFixed32, u32);
        $callback!(ProtoFixed64, u64);
        $callback!(ProtoSFixed32, i32);
        $callback!(ProtoSFixed64, i64);
        $callback!(ProtoBool, bool);
    };
}

for_each_sized_key!(impl_map_entry_string_val);
for_each_sized_key!(impl_map_entry_bytes_val);

impl_map_ref_len!(ProtoString, str, ProtoString, str);
impl_map_ref_len!(ProtoString, str, ProtoBytes, [u8]);

impl<'f, 'c, const FIELD: u32, A, Pb> MapMut<str>
    for MapFieldMut<'f, 'c, ProtoString, ProtoString, FIELD, A, Pb>
where
    A: Allocator + Clone,
    Pb: PresenceBits,
{
    type MutTarget = ::puroro::String<A>;
    type Mut<'a>
        = <ProtoString as RepeatedElementMut>::ElementMut<'a, A>
    where
        Self: 'a;

    #[inline]
    fn len(&self) -> usize {
        self.field.len()
    }

    #[inline]
    fn get(&self, key: &str) -> Option<&str> {
        self.field.entries.get(key).map(|v| v.deref())
    }

    #[inline]
    fn get_mut(&mut self, key: &str) -> Option<Self::Mut<'_>> {
        self.get_element_mut(key)
    }

    #[inline]
    fn entry_mut(&mut self, key: &str) -> Self::Mut<'_> {
        self.entry_element_mut_str(key)
    }

    #[inline]
    fn remove(&mut self, key: &str) {
        MapFieldMut::remove(self, key);
    }

    #[inline]
    fn clear(&mut self) {
        MapFieldMut::clear(self);
    }
}

impl<'f, 'c, const FIELD: u32, A, Pb> MapMut<[u8]>
    for MapFieldMut<'f, 'c, ProtoString, ProtoBytes, FIELD, A, Pb>
where
    A: Allocator + Clone,
    Pb: PresenceBits,
{
    type MutTarget = AllocVec<u8, A>;
    type Mut<'a>
        = <ProtoBytes as RepeatedElementMut>::ElementMut<'a, A>
    where
        Self: 'a;

    #[inline]
    fn len(&self) -> usize {
        self.field.len()
    }

    #[inline]
    fn get(&self, key: &str) -> Option<&[u8]> {
        self.field.entries.get(key).map(|v| v.deref())
    }

    #[inline]
    fn get_mut(&mut self, key: &str) -> Option<Self::Mut<'_>> {
        self.get_element_mut(key)
    }

    #[inline]
    fn entry_mut(&mut self, key: &str) -> Self::Mut<'_> {
        self.entry_element_mut_str(key)
    }

    #[inline]
    fn remove(&mut self, key: &str) {
        MapFieldMut::remove(self, key);
    }

    #[inline]
    fn clear(&mut self) {
        MapFieldMut::clear(self);
    }
}

// ---------------------------------------------------------------------------
// enum values
// ---------------------------------------------------------------------------

macro_rules! impl_map_entry_enum {
    ($key_marker:ty, $key_view:ty, $kind:ty, $bound:ident) => {
        impl<'a, E, const FIELD: u32, A, Pb> MapRef<$key_view, E>
            for MapFieldRef<'a, $key_marker, ProtoEnum<E, $kind>, FIELD, A, Pb>
        where
            E: $bound,
            A: Allocator + Clone,
            Pb: PresenceBits,
        {
            #[inline]
            fn len(&self) -> usize {
                self.field.len()
            }

            #[inline]
            fn get(&self, key: &$key_view) -> Option<&E> {
                self.field.entries.get(key)
            }
        }

        impl<'f, 'c, E, const FIELD: u32, A, Pb> MapEntryMut<$key_view, E>
            for MapFieldMut<'f, 'c, $key_marker, ProtoEnum<E, $kind>, FIELD, A, Pb>
        where
            E: $bound,
            A: Allocator + Clone,
            Pb: PresenceBits,
        {
            type MutTarget = E;
            type Mut<'a>
                = &'a mut E
            where
                Self: 'a;

            #[inline]
            fn len(&self) -> usize {
                self.field.len()
            }

            #[inline]
            fn get(&self, key: &$key_view) -> Option<&E> {
                self.field.entries.get(key)
            }

            #[inline]
            fn get_mut(&mut self, key: &$key_view) -> Option<Self::Mut<'_>> {
                self.get_element_mut(key)
            }

            #[inline]
            fn entry_mut(&mut self, key: $key_view) -> Self::Mut<'_> {
                self.entry_element_mut(key)
            }

            #[inline]
            fn remove(&mut self, key: &$key_view) {
                MapFieldMut::remove(self, key);
            }

            #[inline]
            fn clear(&mut self) {
                MapFieldMut::clear(self);
            }
        }

        impl<'f, 'c, E, const FIELD: u32, A, Pb> MapEntryInsert<$key_view, E>
            for MapFieldMut<'f, 'c, $key_marker, ProtoEnum<E, $kind>, FIELD, A, Pb>
        where
            E: $bound,
            A: Allocator + Clone,
            Pb: PresenceBits,
        {
            #[inline]
            fn insert(&mut self, key: $key_view, value: E) {
                MapFieldMut::insert(self, key, value);
            }
        }
    };
}

macro_rules! impl_map_string_key_enum {
    ($kind:ty, $bound:ident) => {
        impl<'a, E, const FIELD: u32, A, Pb> MapRef<str, E>
            for MapFieldRef<'a, ProtoString, ProtoEnum<E, $kind>, FIELD, A, Pb>
        where
            E: $bound,
            A: Allocator + Clone,
            Pb: PresenceBits,
        {
            #[inline]
            fn len(&self) -> usize {
                self.field.len()
            }

            #[inline]
            fn get(&self, key: &str) -> Option<&E> {
                self.field.entries.get(key)
            }
        }

        impl<'f, 'c, E, const FIELD: u32, A, Pb> MapMut<E>
            for MapFieldMut<'f, 'c, ProtoString, ProtoEnum<E, $kind>, FIELD, A, Pb>
        where
            E: $bound,
            A: Allocator + Clone,
            Pb: PresenceBits,
        {
            type MutTarget = E;
            type Mut<'a>
                = &'a mut E
            where
                Self: 'a;

            #[inline]
            fn len(&self) -> usize {
                self.field.len()
            }

            #[inline]
            fn get(&self, key: &str) -> Option<&E> {
                self.field.entries.get(key)
            }

            #[inline]
            fn get_mut(&mut self, key: &str) -> Option<Self::Mut<'_>> {
                self.get_element_mut(key)
            }

            #[inline]
            fn entry_mut(&mut self, key: &str) -> Self::Mut<'_> {
                self.entry_element_mut_str(key)
            }

            #[inline]
            fn remove(&mut self, key: &str) {
                MapFieldMut::remove(self, key);
            }

            #[inline]
            fn clear(&mut self) {
                MapFieldMut::clear(self);
            }
        }

        impl<'f, 'c, E, const FIELD: u32, A, Pb> MapStrInsert<E>
            for MapFieldMut<'f, 'c, ProtoString, ProtoEnum<E, $kind>, FIELD, A, Pb>
        where
            E: $bound,
            A: Allocator + Clone,
            Pb: PresenceBits,
        {
            #[inline]
            fn insert_str(&mut self, key: &str, value: E) {
                MapFieldMut::insert_str(self, key, value)
            }
        }
    };
}

macro_rules! impl_enum_for_sized_key {
    ($key_marker:ty, $key_view:ty) => {
        impl_map_entry_enum!($key_marker, $key_view, Open, OpenEnum);
        impl_map_entry_enum!($key_marker, $key_view, Closed, ClosedEnum);
    };
}

for_each_sized_key!(impl_enum_for_sized_key);
impl_map_string_key_enum!(Open, OpenEnum);
impl_map_string_key_enum!(Closed, ClosedEnum);

// ---------------------------------------------------------------------------
// message values
// ---------------------------------------------------------------------------

macro_rules! impl_map_entry_message {
    ($key_marker:ty, $key_view:ty) => {
        impl<'a, M, const FIELD: u32, A, Pb> MapRef<$key_view, M>
            for MapFieldRef<'a, $key_marker, ProtoMessage<M>, FIELD, A, Pb>
        where
            M: Message<Alloc = A>,
            A: Allocator + Clone,
            Pb: PresenceBits,
        {
            #[inline]
            fn len(&self) -> usize {
                self.field.len()
            }

            #[inline]
            fn get(&self, key: &$key_view) -> Option<&M> {
                self.field.entries.get(key)
            }
        }

        impl<'f, 'c, M, const FIELD: u32, A, Pb> MapEntryMut<$key_view, M>
            for MapFieldMut<'f, 'c, $key_marker, ProtoMessage<M>, FIELD, A, Pb>
        where
            M: Message<Alloc = A> + ::unmanaged::DeallocateIn<A>,
            A: Allocator + Clone,
            Pb: PresenceBits,
        {
            type MutTarget = M;
            type Mut<'a>
                = &'a mut M
            where
                Self: 'a;

            #[inline]
            fn len(&self) -> usize {
                self.field.len()
            }

            #[inline]
            fn get(&self, key: &$key_view) -> Option<&M> {
                self.field.entries.get(key)
            }

            #[inline]
            fn get_mut(&mut self, key: &$key_view) -> Option<Self::Mut<'_>> {
                self.get_element_mut(key)
            }

            #[inline]
            fn entry_mut(&mut self, key: $key_view) -> Self::Mut<'_> {
                self.entry_element_mut(key)
            }

            #[inline]
            fn remove(&mut self, key: &$key_view) {
                MapFieldMut::remove(self, key);
            }

            #[inline]
            fn clear(&mut self) {
                MapFieldMut::clear(self);
            }
        }

        impl<'f, 'c, M, const FIELD: u32, A, Pb> MapEntryInsert<$key_view, M>
            for MapFieldMut<'f, 'c, $key_marker, ProtoMessage<M>, FIELD, A, Pb>
        where
            M: Message<Alloc = A> + ::unmanaged::DeallocateIn<A>,
            A: Allocator + Clone,
            Pb: PresenceBits,
        {
            #[inline]
            fn insert(&mut self, key: $key_view, value: M) {
                MapFieldMut::insert(self, key, value);
            }
        }
    };
}

for_each_sized_key!(impl_map_entry_message);

impl<'a, M, const FIELD: u32, A, Pb> MapRef<str, M>
    for MapFieldRef<'a, ProtoString, ProtoMessage<M>, FIELD, A, Pb>
where
    M: Message<Alloc = A>,
    A: Allocator + Clone,
    Pb: PresenceBits,
{
    #[inline]
    fn len(&self) -> usize {
        self.field.len()
    }

    #[inline]
    fn get(&self, key: &str) -> Option<&M> {
        self.field.entries.get(key)
    }
}

impl<'f, 'c, M, const FIELD: u32, A, Pb> MapMut<M>
    for MapFieldMut<'f, 'c, ProtoString, ProtoMessage<M>, FIELD, A, Pb>
where
    M: Message<Alloc = A> + ::unmanaged::DeallocateIn<A>,
    A: Allocator + Clone,
    Pb: PresenceBits,
{
    type MutTarget = M;
    type Mut<'a>
        = &'a mut M
    where
        Self: 'a;

    #[inline]
    fn len(&self) -> usize {
        self.field.len()
    }

    #[inline]
    fn get(&self, key: &str) -> Option<&M> {
        self.field.entries.get(key)
    }

    #[inline]
    fn get_mut(&mut self, key: &str) -> Option<Self::Mut<'_>> {
        self.get_element_mut(key)
    }

    #[inline]
    fn entry_mut(&mut self, key: &str) -> Self::Mut<'_> {
        self.entry_element_mut_str(key)
    }

    #[inline]
    fn remove(&mut self, key: &str) {
        MapFieldMut::remove(self, key);
    }

    #[inline]
    fn clear(&mut self) {
        MapFieldMut::clear(self);
    }
}

impl<'f, 'c, M, const FIELD: u32, A, Pb> MapStrInsert<M>
    for MapFieldMut<'f, 'c, ProtoString, ProtoMessage<M>, FIELD, A, Pb>
where
    M: Message<Alloc = A> + ::unmanaged::DeallocateIn<A>,
    A: Allocator + Clone,
    Pb: PresenceBits,
{
    #[inline]
    fn insert_str(&mut self, key: &str, value: M) {
        MapFieldMut::insert_str(self, key, value)
    }
}
