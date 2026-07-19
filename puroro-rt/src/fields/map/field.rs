//! Catalog wrapper for a protobuf `map<K, V>` field.
//!
//! Wire encode / merge will land in a follow-up. Storage is [`MapEntries`]
//! (`cached-pair` with implicit list ↔ map conversion).

use ::core::hash::Hash;

use ::allocator_api2::alloc::Allocator;
use ::hashbrown::Equivalent;
use ::unmanaged::{CloneIn, DeallocateIn};

use crate::fields::shared::{FieldDeallocate, MessageCommon, PresenceBits};

use super::entries::MapEntries;

/// Map field parametrised by key / value element types, field number, and
/// allocator `A`.
pub struct MapField<K, V, const FIELD: u32, A>
where
    A: Allocator + Clone,
{
    entries: MapEntries<K, V, A>,
}

impl<K, V, const FIELD: u32, A> MapField<K, V, FIELD, A>
where
    A: Allocator + Clone,
{
    pub fn new_in(alloc: A) -> Self
    where
        K: CloneIn<A> + Eq + Hash,
        V: CloneIn<A>,
    {
        Self {
            entries: MapEntries::new_in(alloc),
        }
    }

    #[inline]
    pub fn len(&self) -> usize
    where
        K: CloneIn<A> + Eq + Hash,
        V: CloneIn<A>,
    {
        self.entries.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool
    where
        K: CloneIn<A> + Eq + Hash,
        V: CloneIn<A>,
    {
        self.entries.is_empty()
    }

    #[inline]
    pub fn bind<'a, Pb: PresenceBits>(
        &'a self,
        common: &'a MessageCommon<Pb, A>,
    ) -> MapFieldRef<'a, K, V, FIELD, A, Pb> {
        MapFieldRef::new(self, common)
    }

    #[inline]
    pub fn bind_mut<'f, 'c, Pb: PresenceBits>(
        &'f mut self,
        common: &'c mut MessageCommon<Pb, A>,
    ) -> MapFieldMut<'f, 'c, K, V, FIELD, A, Pb> {
        MapFieldMut::new(self, common)
    }

    #[inline]
    pub fn clone_in<Pb>(&self, _common: &MessageCommon<Pb, A>, alloc: A) -> Self
    where
        Pb: PresenceBits,
        K: CloneIn<A> + Eq + Hash,
        V: CloneIn<A>,
    {
        Self {
            entries: self.entries.clone_in(alloc),
        }
    }
}

impl<K, V, const FIELD: u32, A, Pb> FieldDeallocate<Pb, A> for MapField<K, V, FIELD, A>
where
    K: CloneIn<A> + Eq + Hash + DeallocateIn<A>,
    V: CloneIn<A> + DeallocateIn<A>,
    A: Allocator + Clone,
    Pb: PresenceBits,
{
    #[inline]
    fn deallocate(&mut self, _common: &MessageCommon<Pb, A>) {
        self.entries.clear();
    }
}

/// Short-lived shared binding of a map field to its message common state.
pub struct MapFieldRef<'a, K, V, const FIELD: u32, A, Pb>
where
    A: Allocator + Clone,
    Pb: PresenceBits,
{
    field: &'a MapField<K, V, FIELD, A>,
    #[allow(dead_code)]
    common: &'a MessageCommon<Pb, A>,
}

impl<'a, K, V, const FIELD: u32, A, Pb> MapFieldRef<'a, K, V, FIELD, A, Pb>
where
    A: Allocator + Clone,
    Pb: PresenceBits,
{
    #[inline]
    fn new(field: &'a MapField<K, V, FIELD, A>, common: &'a MessageCommon<Pb, A>) -> Self {
        Self { field, common }
    }

    #[inline]
    pub fn len(self) -> usize
    where
        K: CloneIn<A> + Eq + Hash,
        V: CloneIn<A>,
    {
        self.field.len()
    }

    #[inline]
    pub fn is_empty(self) -> bool
    where
        K: CloneIn<A> + Eq + Hash,
        V: CloneIn<A>,
    {
        self.field.is_empty()
    }

    #[inline]
    pub fn get<Q>(self, key: &Q) -> Option<&'a V>
    where
        K: CloneIn<A> + Eq + Hash,
        V: CloneIn<A>,
        Q: ?Sized + Hash + Equivalent<K>,
    {
        self.field.entries.get(key)
    }

    #[inline]
    pub fn iter(self) -> super::entries::MapEntriesIter<'a, K, V>
    where
        K: CloneIn<A> + Eq + Hash,
        V: CloneIn<A>,
    {
        self.field.entries.iter()
    }
}

/// Short-lived binding of a map field to its message common state.
pub struct MapFieldMut<'f, 'c, K, V, const FIELD: u32, A, Pb>
where
    A: Allocator + Clone,
    Pb: PresenceBits,
{
    field: &'f mut MapField<K, V, FIELD, A>,
    #[allow(dead_code)]
    common: &'c mut MessageCommon<Pb, A>,
}

impl<'f, 'c, K, V, const FIELD: u32, A, Pb> MapFieldMut<'f, 'c, K, V, FIELD, A, Pb>
where
    A: Allocator + Clone,
    Pb: PresenceBits,
{
    #[inline]
    fn new(field: &'f mut MapField<K, V, FIELD, A>, common: &'c mut MessageCommon<Pb, A>) -> Self {
        Self { field, common }
    }

    pub fn push(self, key: K, value: V)
    where
        K: CloneIn<A> + Eq + Hash,
        V: CloneIn<A>,
    {
        self.field.entries.push(key, value);
    }

    pub fn insert(self, key: K, value: V)
    where
        K: CloneIn<A> + Eq + Hash,
        V: CloneIn<A> + DeallocateIn<A>,
    {
        if let Some(old) = self.field.entries.insert(key, value) {
            let alloc = self.common.alloc.clone();
            // SAFETY: message allocator owns replaced value payloads.
            unsafe { old.deallocate_in(alloc) };
        }
    }

    pub fn remove<Q>(self, key: &Q)
    where
        K: CloneIn<A> + Eq + Hash,
        V: CloneIn<A> + DeallocateIn<A>,
        Q: ?Sized + Hash + Equivalent<K>,
    {
        if let Some(old) = self.field.entries.remove(key) {
            let alloc = self.common.alloc.clone();
            // SAFETY: message allocator owns removed value payloads.
            unsafe { old.deallocate_in(alloc) };
        }
    }

    pub fn clear(self)
    where
        K: CloneIn<A> + Eq + Hash + DeallocateIn<A>,
        V: CloneIn<A> + DeallocateIn<A>,
    {
        self.field.entries.clear();
    }
}
