//! Blanket `puroro::{MapRef, MapMut}` impls for [`MapField`](super::MapField).
//!
//! View types come from marker GATs ([`MapKey::KeyView`], [`MapValueView::View`]),
//! matching the scalar pattern (`ProtoType::{Ref, Mut}` → generic field surface).

use ::core::borrow::Borrow;
use ::core::hash::Hash;

use ::allocator_api2::alloc::Allocator;
use ::puroro::{MapMut, MapRef};

use crate::fields::shared::PresenceBits;
use crate::fields::wire::map_element::{MapKey, MapKeyInsert, MapValueView};
use crate::fields::wire::repeated_element::{RepeatedElementMerge, RepeatedElementMut};

use super::{MapFieldMut, MapFieldRef};

impl<'a, K, V, const FIELD: u32, A, Pb> MapRef<K::KeyView, V::View>
    for MapFieldRef<'a, K, V, FIELD, A, Pb>
where
    K: MapKey,
    V: MapValueView,
    A: Allocator + Clone,
    Pb: PresenceBits,
    K::Element<A>: Hash + Eq + Borrow<K::KeyView>,
{
    #[inline]
    fn len(&self) -> usize {
        self.field.len()
    }

    #[inline]
    fn get(&self, key: impl Borrow<K::KeyView>) -> Option<&V::View> {
        self.field.entries.get(key.borrow()).map(V::as_view)
    }
}

impl<'f, 'c, K, V, const FIELD: u32, A, Pb> MapMut<K::KeyView, V::View>
    for MapFieldMut<'f, 'c, K, V, FIELD, A, Pb>
where
    K: MapKeyInsert<A>,
    V: MapValueView + RepeatedElementMut + RepeatedElementMerge<A>,
    A: Allocator + Clone,
    Pb: PresenceBits,
    K::Element<A>: Hash + Eq + Borrow<K::KeyView>,
{
    type MutTarget = V::MutTarget<A>;

    type Mut<'a>
        = V::ElementMut<'a, A>
    where
        Self: 'a;

    #[inline]
    fn len(&self) -> usize {
        self.field.len()
    }

    #[inline]
    fn get(&self, key: impl Borrow<K::KeyView>) -> Option<&V::View> {
        self.field.entries.get(key.borrow()).map(V::as_view)
    }

    #[inline]
    fn get_mut(&mut self, key: impl Borrow<K::KeyView>) -> Option<Self::Mut<'_>> {
        self.get_element_mut(key.borrow())
    }

    #[inline]
    fn entry_mut(&mut self, key: impl Borrow<K::KeyView>) -> Self::Mut<'_> {
        self.entry_element_mut_view(key.borrow())
    }

    #[inline]
    fn remove(&mut self, key: impl Borrow<K::KeyView>) {
        MapFieldMut::remove(self, key.borrow());
    }

    #[inline]
    fn clear(&mut self) {
        MapFieldMut::clear(self);
    }
}
