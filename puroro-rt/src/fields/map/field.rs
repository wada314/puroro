//! Catalog wrapper for a protobuf `map<K, V>` field.
//!
//! - `K: `[`MapKey`] — key marker (integral / `bool` / `string`)
//! - `V: `[`RepeatedElement`] — value marker (same element types as repeated)
//!
//! On the wire each entry is a LEN message with `key = 1` and `value = 2`.
//!
//! [`MapReady`] (the default `L`) stores a `HashMap` as entries arrive.
//! [`MapSpans`] stores entry-LEN offsets and builds that `HashMap` on first get.

use ::core::borrow::Borrow;
use ::core::cell::UnsafeCell;
use ::core::fmt::{Debug, Formatter, Result as FmtResult};
use ::core::hash::Hash;
use ::core::mem;

use ::allocator_api2::alloc::Allocator;
use ::allocator_api2::vec::Vec as AllocVec;
use ::bytes::{Buf, BufMut};
use ::hashbrown::hash_map::Iter as HashMapIter;
use ::hashbrown::{DefaultHashBuilder, Equivalent, HashMap};
use ::puroro::{DecodeBuf, DecodeError, MapMut, MapRef, ScopedBuf, WireType};
use ::unmanaged::{CloneIn, ToOwnedIn};

use super::MapKey;
use crate::decode::{self, WireSpan};
use crate::encode::{self, field_number_const};
use crate::fields::shared::field_inspect::{FieldCloneIn, FieldDebug, FieldEncode, FieldPartialEq};
use crate::fields::shared::{FieldDeallocate, MessageCommon, MessageCommonAlloc};
use crate::fields::wire::repeated_element::{
    RepeatedElement, RepeatedElementMerge, RepeatedElementMut,
};
use crate::message_encode::EncodeCtx;

use super::entry::{decode_map_entry, encode_map_entry, entry_payload_len};

/// How a [`MapField`] holds entries.
///
/// [`MapReady`] is a `HashMap`. [`MapSpans`] is an offset list plus an optional
/// materialised map. Unlike oneof's ingest `L`, this parameter changes the
/// field's own storage.
pub trait MapLayout<K: MapKey, V: RepeatedElement, const FIELD: u32, A: Allocator> {
    /// Payload stored on [`MapField`].
    type Storage;

    /// Empty storage using `alloc` (HashMap hasher or the span vec).
    fn new_storage(alloc: A) -> Self::Storage;
}

/// Eager map: the `HashMap` is the field. Entries land during `merge`.
pub struct MapReady;

/// Lazy map: entry-LEN offsets during the parent scan; `HashMap` on first get.
pub struct MapSpans;

impl<K, V, const FIELD: u32, A> MapLayout<K, V, FIELD, A> for MapReady
where
    K: MapKey,
    V: RepeatedElement,
    A: Allocator,
{
    type Storage = HashMap<K::Element<A>, V::Element<A>, DefaultHashBuilder, A>;

    #[inline]
    fn new_storage(alloc: A) -> Self::Storage {
        HashMap::with_hasher_in(DefaultHashBuilder::default(), alloc)
    }
}

/// Offset list plus an optional materialised [`MapField`] (`L = `[`MapReady`]).
#[doc(hidden)]
pub struct MapSpanStorage<K, V, const FIELD: u32, A>
where
    K: MapKey,
    V: RepeatedElement,
    A: Allocator,
{
    spans: AllocVec<WireSpan, A>,
    ready: UnsafeCell<Option<MapField<K, V, FIELD, A>>>,
}

impl<K, V, const FIELD: u32, A> MapLayout<K, V, FIELD, A> for MapSpans
where
    K: MapKey,
    V: RepeatedElement,
    A: Allocator,
{
    type Storage = MapSpanStorage<K, V, FIELD, A>;

    #[inline]
    fn new_storage(alloc: A) -> Self::Storage {
        MapSpanStorage {
            spans: AllocVec::new_in(alloc),
            ready: UnsafeCell::new(None),
        }
    }
}

/// Map field: key marker `K`, value marker `V`, field number `FIELD`, allocator `A`.
///
/// `L` is the field layout: [`MapReady`] (default) or [`MapSpans`]. Eager
/// messages keep a `HashMap` of elements (unlike `UnmanagedVec` fields). Wire
/// order is unspecified. Lazy messages keep entry-LEN offsets until the first
/// getter.
pub struct MapField<K, V, const FIELD: u32, A, L = MapReady>
where
    K: MapKey,
    V: RepeatedElement,
    A: Allocator,
    L: MapLayout<K, V, FIELD, A>,
{
    storage: L::Storage,
}

impl<K, V, const FIELD: u32, A, L> MapField<K, V, FIELD, A, L>
where
    K: MapKey,
    V: RepeatedElement,
    A: Allocator,
    L: MapLayout<K, V, FIELD, A>,
{
    /// Creates an empty map using `alloc` for the layout's storage.
    pub fn new_in(alloc: A) -> Self {
        Self {
            storage: L::new_storage(alloc),
        }
    }
}

impl<K, V, const FIELD: u32, A> MapField<K, V, FIELD, A>
where
    K: MapKey,
    V: RepeatedElement,
    A: Allocator,
{
    /// Number of entries.
    #[inline]
    pub fn len(&self) -> usize {
        self.storage.len()
    }

    /// `true` when the map has no entries.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }

    /// Shared bound view (pairs this field with message common state).
    #[inline]
    pub fn bind<'a, Cx: MessageCommonAlloc<Alloc = A>>(
        &'a self,
        common: &'a Cx,
    ) -> MapFieldRef<'a, K, V, FIELD, A, Cx> {
        MapFieldRef::new(self, common)
    }

    /// Mutable bound view (pairs this field with message common state).
    #[inline]
    pub fn bind_mut<'f, 'c, Cx: MessageCommonAlloc<Alloc = A>>(
        &'f mut self,
        common: &'c mut Cx,
    ) -> MapFieldMut<'f, 'c, K, V, FIELD, A, Cx> {
        MapFieldMut::new(self, common)
    }

    /// Inserts with last-wins. Replaced value and discarded key are released.
    ///
    /// On key collision, keeps the stored key and frees the incoming key —
    /// [`HashMap::insert`] would drop the incoming key, which is unsafe for
    /// `Unmanaged*` payloads.
    pub fn insert_last_wins(&mut self, key: K::Element<A>, value: V::Element<A>, alloc: &A)
    where
        K::Element<A>: Eq + Hash,
    {
        let discarded = if let Some(slot) = self.storage.get_mut(&key) {
            let previous = mem::replace(slot, value);
            Some((key, previous))
        } else {
            self.storage.insert(key, value);
            None
        };
        if let Some((discarded_key, old_value)) = discarded {
            // SAFETY: message allocator owns discarded key / replaced value.
            unsafe {
                K::deallocate_element(discarded_key, alloc);
                V::deallocate_element(old_value, alloc);
            }
        }
    }

    /// Decode one map-entry message body (no length prefix) and insert last-wins.
    pub fn merge_entry_body<B: Buf>(
        &mut self,
        buf: &mut B,
        alloc: A,
        depth: usize,
    ) -> Result<(), DecodeError>
    where
        K: RepeatedElementMerge<A>,
        V: RepeatedElementMerge<A>,
        A: Clone,
        K::Element<A>: Eq + Hash,
    {
        let mut scoped = ScopedBuf::new(buf);
        let (key, value) = decode_map_entry::<K, V, A, _>(&mut scoped, alloc.clone(), depth)?;
        self.insert_last_wins(key, value, &alloc);
        Ok(())
    }
}

impl<K, V, const FIELD: u32, A> MapField<K, V, FIELD, A, MapSpans>
where
    K: MapKey,
    V: RepeatedElement,
    A: Allocator,
{
    /// Record one map-entry LEN. If already materialised, merge it now.
    pub fn store_span<Cx: MessageCommonAlloc<Alloc = A>>(
        &mut self,
        span: WireSpan,
        wire: &[u8],
        common: &Cx,
    ) -> Result<(), DecodeError>
    where
        A: Clone,
        K: RepeatedElementMerge<A>,
        V: RepeatedElementMerge<A>,
        K::Element<A>: Eq + Hash,
    {
        self.storage.spans.push(span);
        if let Some(map) = self.storage.ready.get_mut() {
            let payload = span.slice(wire)?;
            map.merge_entry_body(&mut &payload[..], common.clone_alloc(), 0)?;
        }
        Ok(())
    }

    /// Materialise on first call, then return the catalog map view.
    pub fn bind<'a, Cx: MessageCommonAlloc<Alloc = A>>(
        &'a self,
        wire: &'a [u8],
        common: &'a Cx,
    ) -> Result<MapFieldRef<'a, K, V, FIELD, A, Cx>, DecodeError>
    where
        A: Clone,
        K: RepeatedElementMerge<A>,
        V: RepeatedElementMerge<A>,
        K::Element<A>: Eq + Hash,
    {
        // SAFETY: callers only `bind` after the parent stream is finished, and
        // never overlap it with `store_span` (`&mut self`).
        let ready = unsafe { &mut *self.storage.ready.get() };
        if ready.is_none() {
            let mut map = MapField::new_in(common.clone_alloc());
            for &span in &self.storage.spans {
                let payload = match span.slice(wire) {
                    Ok(p) => p,
                    Err(e) => {
                        FieldDeallocate::deallocate(&mut map, common);
                        return Err(e);
                    }
                };
                if let Err(e) = map.merge_entry_body(&mut &payload[..], common.clone_alloc(), 0) {
                    FieldDeallocate::deallocate(&mut map, common);
                    return Err(e);
                }
            }
            *ready = Some(map);
        }
        Ok(ready.as_ref().expect("map materialised").bind(common))
    }
}

impl<K, V, const FIELD: u32, A, C> FieldDeallocate<C> for MapField<K, V, FIELD, A, MapSpans>
where
    K: MapKey,
    V: RepeatedElement,
    A: Allocator,
    C: MessageCommonAlloc<Alloc = A>,
{
    /// Release a materialised [`MapField`]. Spans need no teardown.
    #[inline]
    fn deallocate(&mut self, common: &C) {
        if let Some(mut map) = self.storage.ready.get_mut().take() {
            FieldDeallocate::deallocate(&mut map, common);
        }
    }
}

impl<K, V, const FIELD: u32, A, C> FieldDeallocate<C> for MapField<K, V, FIELD, A>
where
    K: MapKey,
    V: RepeatedElement,
    A: Allocator,
    C: MessageCommonAlloc<Alloc = A>,
{
    #[inline]
    fn deallocate(&mut self, common: &C) {
        let alloc = common.alloc();
        for (k, v) in self.storage.drain() {
            // SAFETY: message allocator owns key / value payloads.
            unsafe {
                K::deallocate_element(k, alloc);
                V::deallocate_element(v, alloc);
            }
        }
    }
}

impl<K, V, const FIELD: u32, A, P> FieldPartialEq<MessageCommon<P, A>> for MapField<K, V, FIELD, A>
where
    K: MapKey,
    V: RepeatedElement,
    A: Allocator,
    K::Element<A>: Eq + Hash,
    V::Element<A>: PartialEq,
{
    #[inline]
    fn field_eq(
        &self,
        common: &MessageCommon<P, A>,
        other: &Self,
        other_common: &MessageCommon<P, A>,
    ) -> bool {
        if self.len() != other.len() {
            return false;
        }
        self.bind(common)
            .iter()
            .all(|(k, v)| other.bind(other_common).get(k) == Some(v))
    }
}

impl<K, V, const FIELD: u32, A, P> FieldDebug<MessageCommon<P, A>> for MapField<K, V, FIELD, A>
where
    K: MapKey,
    V: RepeatedElement,
    A: Allocator,
    K::Element<A>: Eq + Hash + Debug,
    V::Element<A>: Debug,
{
    #[inline]
    fn fmt_debug(&self, _common: &MessageCommon<P, A>, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_map().entries(&self.storage).finish()
    }
}

impl<K, V, const FIELD: u32, A, P> FieldEncode<MessageCommon<P, A>> for MapField<K, V, FIELD, A>
where
    K: MapKey,
    V: RepeatedElement,
    A: Allocator,
    K::Element<A>: Eq + Hash,
{
    fn encoded_len(&self, _common: &MessageCommon<P, A>, ctx: &mut EncodeCtx) -> usize {
        let mut n = 0;
        for (key, value) in &self.storage {
            let payload = entry_payload_len::<K, V, A>(key, value, ctx);
            n += encode::encoded_len_len_field(field_number_const::<FIELD>(), payload);
        }
        n
    }

    fn encode_raw<B: BufMut>(
        &self,
        _common: &MessageCommon<P, A>,
        ctx: &mut EncodeCtx,
        buf: &mut B,
    ) {
        for (key, value) in &self.storage {
            encode_map_entry::<K, V, A, B>(field_number_const::<FIELD>(), key, value, ctx, buf);
        }
    }
}

impl<K, V, const FIELD: u32, A, P> FieldCloneIn<MessageCommon<P, A>> for MapField<K, V, FIELD, A>
where
    K: MapKey,
    V: RepeatedElement,
    A: Allocator + Clone,
    K::Element<A>: CloneIn<A> + Eq + Hash,
    V::Element<A>: CloneIn<A>,
{
    fn clone_field(&self, _common: &MessageCommon<P, A>, alloc: A) -> Self {
        let mut out = HashMap::with_capacity_and_hasher_in(
            self.storage.len(),
            DefaultHashBuilder::default(),
            alloc.clone(),
        );
        out.extend(
            self.storage
                .iter()
                .map(|(k, v)| (k.clone_in(alloc.clone()), v.clone_in(alloc.clone()))),
        );
        Self { storage: out }
    }
}

/// Short-lived shared binding of a map field to its message common state.
pub struct MapFieldRef<'a, K, V, const FIELD: u32, A, Cx>
where
    K: MapKey,
    V: RepeatedElement,
    A: Allocator,
{
    field: &'a MapField<K, V, FIELD, A>,
    #[allow(dead_code)]
    common: &'a Cx,
}

impl<'a, K, V, const FIELD: u32, A, Cx> MapFieldRef<'a, K, V, FIELD, A, Cx>
where
    K: MapKey,
    V: RepeatedElement,
    A: Allocator,
{
    #[inline]
    fn new(field: &'a MapField<K, V, FIELD, A>, common: &'a Cx) -> Self {
        Self { field, common }
    }

    /// Shared reference to the stored value for `key`, if present.
    #[inline]
    pub fn get<Q>(self, key: &Q) -> Option<&'a V::Element<A>>
    where
        K::Element<A>: Eq + Hash,
        Q: ?Sized + Hash + Equivalent<K::Element<A>>,
    {
        self.field.storage.get(key)
    }

    /// Iterator over stored `(key, value)` element pairs.
    #[inline]
    pub fn iter(self) -> HashMapIter<'a, K::Element<A>, V::Element<A>> {
        self.field.storage.iter()
    }
}

/// Short-lived binding of a map field to its message common state.
pub struct MapFieldMut<'f, 'c, K, V, const FIELD: u32, A, Cx: MessageCommonAlloc<Alloc = A>>
where
    K: MapKey,
    V: RepeatedElement,
    A: Allocator,
{
    field: &'f mut MapField<K, V, FIELD, A>,
    common: &'c mut Cx,
}

impl<'f, 'c, K, V, const FIELD: u32, A, Cx> MapFieldMut<'f, 'c, K, V, FIELD, A, Cx>
where
    K: MapKey,
    V: RepeatedElement,
    A: Allocator,
    Cx: MessageCommonAlloc<Alloc = A>,
{
    #[inline]
    fn new(field: &'f mut MapField<K, V, FIELD, A>, common: &'c mut Cx) -> Self {
        Self { field, common }
    }

    /// Mutable element handle (string/bytes → guard; scalars/messages → `&mut`).
    #[inline]
    pub fn get_element_mut<Q>(&mut self, key: &Q) -> Option<V::ElementMut<'_, A>>
    where
        V: RepeatedElementMut,
        A: Clone,
        K::Element<A>: Eq + Hash,
        Q: ?Sized + Hash + Equivalent<K::Element<A>>,
    {
        let alloc = self.common.clone_alloc();
        self.field
            .storage
            .get_mut(key)
            // SAFETY: message allocator owns heap-backed elements.
            .map(|elem| unsafe { V::element_mut(elem, alloc) })
    }

    /// Ensures `key` exists (type-default value if vacant), then returns a value mut handle.
    pub fn entry_element_mut_view(&mut self, key: &K::RefView) -> V::ElementMut<'_, A>
    where
        V: RepeatedElementMut + RepeatedElementMerge<A>,
        A: Clone,
        K::RefView: Hash + Eq + ToOwnedIn<A, Owned = K::Element<A>>,
        K::Element<A>: Eq + Hash + Borrow<K::RefView>,
    {
        if self.field.storage.get(key).is_none() {
            let owned_key = key.to_owned_in(self.common.clone_alloc());
            let value = V::default_element(self.common.clone_alloc());
            self.insert(owned_key, value);
        }
        self.get_element_mut(key)
            .expect("map entry present after insert")
    }

    /// Inserts with last-wins. Replaced value and discarded key are released.
    ///
    /// On key collision, keeps the stored key and frees the incoming key —
    /// [`HashMap::insert`] would drop the incoming key, which is unsafe for
    /// `Unmanaged*` payloads.
    pub fn insert(&mut self, key: K::Element<A>, value: V::Element<A>)
    where
        K::Element<A>: Eq + Hash,
    {
        self.field.insert_last_wins(key, value, self.common.alloc());
    }

    pub fn remove<Q>(&mut self, key: &Q)
    where
        K::Element<A>: Eq + Hash,
        Q: ?Sized + Hash + Equivalent<K::Element<A>>,
    {
        if let Some((old_key, old_value)) = self.field.storage.remove_entry(key) {
            let alloc = self.common.alloc();
            // SAFETY: message allocator owns removed key / value payloads.
            unsafe {
                K::deallocate_element(old_key, alloc);
                V::deallocate_element(old_value, alloc);
            }
        }
    }

    pub fn clear(&mut self) {
        let alloc = self.common.alloc();
        for (k, v) in self.field.storage.drain() {
            // SAFETY: message allocator owns key / value payloads.
            unsafe {
                K::deallocate_element(k, alloc);
                V::deallocate_element(v, alloc);
            }
        }
    }

    /// Merges one map-entry LEN occurrence (last-wins on duplicate keys).
    pub fn merge<B: DecodeBuf>(
        &mut self,
        wire_type: WireType,
        buf: &mut B,
        depth: usize,
    ) -> Result<(), DecodeError>
    where
        K: RepeatedElementMerge<A>,
        V: RepeatedElementMerge<A>,
        A: Clone,
        K::Element<A>: Eq + Hash,
    {
        if wire_type != WireType::Len {
            return Err(DecodeError::InvalidTag);
        }
        let len = decode::decode_varint(buf)? as usize;
        let mut guard = buf.push_limit_guard(len)?;
        let alloc = self.common.clone_alloc();
        let (key, value) = decode_map_entry::<K, V, A, _>(&mut *guard, alloc, depth)?;
        self.insert(key, value);
        Ok(())
    }
}

// Blanket `puroro::{MapRef, MapMut}` over catalog bind views (`RefView` for key
// and value markers).

impl<'a, K, V, const FIELD: u32, A, Cx> MapRef<K::RefView, V::RefView>
    for MapFieldRef<'a, K, V, FIELD, A, Cx>
where
    K: MapKey,
    V: RepeatedElement,
    A: Allocator,
    K::RefView: Hash + Eq,
    K::Element<A>: Hash + Eq + Borrow<K::RefView>,
{
    #[inline]
    fn len(&self) -> usize {
        self.field.len()
    }

    #[inline]
    fn get(&self, key: impl Borrow<K::RefView>) -> Option<&V::RefView> {
        self.field.storage.get(key.borrow()).map(V::as_ref_view)
    }
}

impl<'f, 'c, K, V, const FIELD: u32, A, Cx> MapMut<K::RefView, V::RefView>
    for MapFieldMut<'f, 'c, K, V, FIELD, A, Cx>
where
    K: MapKey,
    V: RepeatedElementMut + RepeatedElementMerge<A>,
    A: Allocator + Clone,
    Cx: MessageCommonAlloc<Alloc = A>,
    K::RefView: Hash + Eq + ToOwnedIn<A, Owned = K::Element<A>>,
    K::Element<A>: Hash + Eq + Borrow<K::RefView>,
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
    fn get(&self, key: impl Borrow<K::RefView>) -> Option<&V::RefView> {
        self.field.storage.get(key.borrow()).map(V::as_ref_view)
    }

    #[inline]
    fn get_mut(&mut self, key: impl Borrow<K::RefView>) -> Option<Self::Mut<'_>> {
        self.get_element_mut(key.borrow())
    }

    #[inline]
    fn entry_mut(&mut self, key: impl Borrow<K::RefView>) -> Self::Mut<'_> {
        self.entry_element_mut_view(key.borrow())
    }

    #[inline]
    fn remove(&mut self, key: impl Borrow<K::RefView>) {
        MapFieldMut::remove(self, key.borrow());
    }

    #[inline]
    fn clear(&mut self) {
        MapFieldMut::clear(self);
    }
}

#[cfg(test)]
mod tests {
    use super::super::entry::encode_map_entry;
    use super::MapField;
    use crate::decode::{decode_tag, decode_varint};
    use crate::encode::{encode_tag, encode_varint, encode_varint_field, field_number_const};
    use crate::fields::shared::field_inspect::FieldEncode;
    use crate::fields::shared::{FieldDeallocate, MessageCommon};
    use crate::fields::wire::{ProtoInt32, ProtoString};
    use crate::message_encode::EncodeCtx;
    use ::allocator_api2::alloc::Global;
    use ::bitvec::array::BitArray;
    use ::bitvec::order::Lsb0;
    use ::bytes::BytesMut;
    use ::protobuf_core::Varint;
    use ::puroro::{ScopedBuf, WireType};
    use ::unmanaged::UnmanagedString;

    #[test]
    fn map_field_int_markers() {
        let mut common =
            MessageCommon::<BitArray<[u8; 1], Lsb0>, Global>::new_in(BitArray::ZERO, Global);
        let mut field = MapField::<ProtoInt32, ProtoInt32, 1, _>::new_in(Global);
        field.bind_mut(&mut common).insert(1, 10);
        field.bind_mut(&mut common).insert(1, 11);
        assert_eq!(field.bind(&common).get(&1), Some(&11));
        assert_eq!(field.len(), 1);
        field.bind_mut(&mut common).clear();
        assert_eq!(field.len(), 0);
        field.deallocate(&common);
    }

    #[test]
    fn map_int_roundtrip_and_last_wins() {
        let mut common =
            MessageCommon::<BitArray<[u8; 1], Lsb0>, Global>::new_in(BitArray::ZERO, Global);
        let mut field = MapField::<ProtoInt32, ProtoInt32, 7, _>::new_in(Global);
        field.bind_mut(&mut common).insert(1, 10);
        field.bind_mut(&mut common).insert(2, 20);

        let mut buf = BytesMut::new();
        let mut ctx = EncodeCtx::new();
        field.encode_raw(&common, &mut ctx, &mut buf);
        assert_eq!(field.encoded_len(&common, &mut EncodeCtx::new()), buf.len());

        let mut decoded = MapField::<ProtoInt32, ProtoInt32, 7, _>::new_in(Global);
        let mut rest = buf.as_ref();
        while !rest.is_empty() {
            let (field_number, wire_type) = decode_tag(&mut rest).unwrap();
            assert_eq!(field_number.as_u32(), 7);
            let mut scoped = ScopedBuf::new(&mut rest);
            decoded
                .bind_mut(&mut common)
                .merge(wire_type, &mut scoped, 0)
                .unwrap();
        }
        assert_eq!(decoded.bind(&common).get(&1), Some(&10));
        assert_eq!(decoded.bind(&common).get(&2), Some(&20));

        // Second merge of key=1 overwrites.
        let mut one = BytesMut::new();
        encode_map_entry::<ProtoInt32, ProtoInt32, Global, _>(
            field_number_const::<7>(),
            &1,
            &99,
            &mut EncodeCtx::new(),
            &mut one,
        );
        let mut rest = one.as_ref();
        let (_, wt) = decode_tag(&mut rest).unwrap();
        let mut scoped = ScopedBuf::new(&mut rest);
        decoded
            .bind_mut(&mut common)
            .merge(wt, &mut scoped, 0)
            .unwrap();
        assert_eq!(decoded.bind(&common).get(&1), Some(&99));
        assert_eq!(decoded.len(), 2);

        field.deallocate(&common);
        decoded.deallocate(&common);
    }

    #[test]
    fn map_entry_missing_key_defaults_to_zero() {
        let mut common =
            MessageCommon::<BitArray<[u8; 1], Lsb0>, Global>::new_in(BitArray::ZERO, Global);
        let mut field = MapField::<ProtoInt32, ProtoInt32, 1, _>::new_in(Global);

        // Entry body: only value=2 with 42 (tag 0x10, varint 42).
        let mut entry_body = BytesMut::new();
        encode_varint_field(
            field_number_const::<2>(),
            Varint::from_uint64(42),
            &mut entry_body,
        );

        let mut framed = BytesMut::new();
        encode_tag(field_number_const::<1>(), WireType::Len, &mut framed);
        encode_varint(Varint::from_uint64(entry_body.len() as u64), &mut framed);
        framed.extend_from_slice(&entry_body);

        let mut rest = framed.as_ref();
        let (_, wt) = decode_tag(&mut rest).unwrap();
        let mut scoped = ScopedBuf::new(&mut rest);
        field
            .bind_mut(&mut common)
            .merge(wt, &mut scoped, 0)
            .unwrap();
        assert_eq!(field.bind(&common).get(&0), Some(&42));
        field.deallocate(&common);
    }

    fn unmanaged_str(s: &str) -> UnmanagedString<Global> {
        UnmanagedString::from_string(::unmanaged::String::from_str_in(s, Global))
    }

    #[test]
    fn map_string_key_roundtrip() {
        let mut common =
            MessageCommon::<BitArray<[u8; 1], Lsb0>, Global>::new_in(BitArray::ZERO, Global);
        let mut field = MapField::<ProtoString, ProtoInt32, 3, _>::new_in(Global);
        field.bind_mut(&mut common).insert(unmanaged_str("ab"), 7);

        let mut buf = BytesMut::new();
        field.encode_raw(&common, &mut EncodeCtx::new(), &mut buf);

        let mut decoded = MapField::<ProtoString, ProtoInt32, 3, _>::new_in(Global);
        let mut rest = buf.as_ref();
        let (_, wt) = decode_tag(&mut rest).unwrap();
        let mut scoped = ScopedBuf::new(&mut rest);
        decoded
            .bind_mut(&mut common)
            .merge(wt, &mut scoped, 0)
            .unwrap();
        assert_eq!(decoded.bind(&common).get("ab"), Some(&7));

        field.deallocate(&common);
        decoded.deallocate(&common);
    }

    /// `HashMap::insert` would Drop the colliding incoming key — fatal for
    /// `UnmanagedString`. Collision must keep the stored key and `deallocate`
    /// the incoming one (and the replaced value).
    #[test]
    fn insert_duplicate_string_key_releases_incoming_key() {
        let mut common =
            MessageCommon::<BitArray<[u8; 1], Lsb0>, Global>::new_in(BitArray::ZERO, Global);
        let mut field = MapField::<ProtoString, ProtoInt32, 1, _>::new_in(Global);

        field.bind_mut(&mut common).insert(unmanaged_str("k"), 1);
        // Second insert with an equal key: would panic if the incoming
        // `UnmanagedString` were dropped by `HashMap::insert`.
        field.bind_mut(&mut common).insert(unmanaged_str("k"), 2);

        assert_eq!(field.len(), 1);
        assert_eq!(field.bind(&common).get("k"), Some(&2));
        field.deallocate(&common);
    }

    /// Same collision path via wire merge (decode builds fresh `UnmanagedString` keys).
    #[test]
    fn merge_duplicate_string_key_releases_incoming_key() {
        let mut common =
            MessageCommon::<BitArray<[u8; 1], Lsb0>, Global>::new_in(BitArray::ZERO, Global);

        let encode_entry =
            |common: &mut MessageCommon<BitArray<[u8; 1], Lsb0>, Global>, value: i32| -> BytesMut {
                let mut src = MapField::<ProtoString, ProtoInt32, 1, _>::new_in(Global);
                src.bind_mut(common).insert(unmanaged_str("k"), value);
                let mut buf = BytesMut::new();
                src.encode_raw(common, &mut EncodeCtx::new(), &mut buf);
                src.deallocate(common);
                buf
            };
        let first = encode_entry(&mut common, 1);
        let second = encode_entry(&mut common, 9);

        let mut field = MapField::<ProtoString, ProtoInt32, 1, _>::new_in(Global);
        for framed in [first, second] {
            let mut rest = framed.as_ref();
            let (_, wt) = decode_tag(&mut rest).unwrap();
            let mut scoped = ScopedBuf::new(&mut rest);
            field
                .bind_mut(&mut common)
                .merge(wt, &mut scoped, 0)
                .unwrap();
        }

        assert_eq!(field.len(), 1);
        assert_eq!(field.bind(&common).get("k"), Some(&9));
        field.deallocate(&common);
    }

    /// Collision must also release the previous `UnmanagedString` value (not only the key).
    #[test]
    fn insert_duplicate_string_key_releases_previous_string_value() {
        let mut common =
            MessageCommon::<BitArray<[u8; 1], Lsb0>, Global>::new_in(BitArray::ZERO, Global);
        let mut field = MapField::<ProtoString, ProtoString, 1, _>::new_in(Global);

        field
            .bind_mut(&mut common)
            .insert(unmanaged_str("k"), unmanaged_str("old"));
        field
            .bind_mut(&mut common)
            .insert(unmanaged_str("k"), unmanaged_str("new"));

        assert_eq!(field.len(), 1);
        assert_eq!(field.bind(&common).get("k").map(|s| &**s), Some("new"));
        field.deallocate(&common);
    }

    fn encode_map_entry_body(common: &mut TestCommon, key: i32, value: i32) -> BytesMut {
        let mut src = MapField::<ProtoInt32, ProtoInt32, 7, _>::new_in(Global);
        src.bind_mut(common).insert(key, value);
        let mut framed = BytesMut::new();
        src.encode_raw(common, &mut EncodeCtx::new(), &mut framed);
        src.deallocate(common);
        // Drop the field tag + length prefix; store_span wants the entry body.
        let mut rest = framed.as_ref();
        let _ = decode_tag(&mut rest).unwrap();
        let len = decode_varint(&mut rest).unwrap() as usize;
        BytesMut::from(&rest[..len])
    }

    type TestCommon = MessageCommon<BitArray<[u8; 1], Lsb0>, Global>;

    #[test]
    fn map_spans_materialise_last_wins_and_later_store() {
        use super::MapSpans;
        use crate::decode::WireSpan;

        let mut common = TestCommon::new_in(BitArray::ZERO, Global);
        let mut field = MapField::<ProtoInt32, ProtoInt32, 7, _, MapSpans>::new_in(Global);

        let first = encode_map_entry_body(&mut common, 1, 10);
        let second = encode_map_entry_body(&mut common, 1, 11);
        let other = encode_map_entry_body(&mut common, 2, 20);
        let mut wire = BytesMut::new();
        wire.extend_from_slice(&first);
        let first_span = WireSpan {
            offset: 0,
            len: first.len(),
        };
        wire.extend_from_slice(&second);
        let second_span = WireSpan {
            offset: first.len(),
            len: second.len(),
        };
        wire.extend_from_slice(&other);
        let other_span = WireSpan {
            offset: first.len() + second.len(),
            len: other.len(),
        };

        field.store_span(first_span, &wire, &common).unwrap();
        field.store_span(second_span, &wire, &common).unwrap();
        field.store_span(other_span, &wire, &common).unwrap();

        assert_eq!(field.bind(&wire, &common).unwrap().get(&1), Some(&11));
        assert_eq!(field.bind(&wire, &common).unwrap().get(&2), Some(&20));

        let late = encode_map_entry_body(&mut common, 2, 99);
        let late_off = wire.len();
        wire.extend_from_slice(&late);
        field
            .store_span(
                WireSpan {
                    offset: late_off,
                    len: late.len(),
                },
                &wire,
                &common,
            )
            .unwrap();
        assert_eq!(field.bind(&wire, &common).unwrap().get(&2), Some(&99));

        field.deallocate(&common);
    }
}
