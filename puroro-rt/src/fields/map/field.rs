//! Catalog wrapper for a protobuf `map<K, V>` field.
//!
//! - `K: `[`MapKey`] — key marker (integral / `bool` / `string`)
//! - `V: `[`RepeatedElement`] — value marker (same element types as repeated)
//!
//! On the wire each entry is a LEN message with `key = 1` and `value = 2`.

use ::core::borrow::Borrow;
use ::core::fmt::{Debug, Formatter, Result as FmtResult};
use ::core::hash::Hash;
use ::core::mem;

use ::allocator_api2::alloc::Allocator;
use ::bytes::{Buf, BufMut};
use ::hashbrown::hash_map::Iter as HashMapIter;
use ::hashbrown::{DefaultHashBuilder, Equivalent, HashMap};
use ::unmanaged::CloneIn;

use ::puroro::{DecodeError, MapMut, MapRef, WireType};

use crate::decode;
use crate::encode;
use crate::fields::shared::field_inspect::{FieldCloneIn, FieldDebug, FieldEncode, FieldPartialEq};
use crate::fields::shared::{FieldDeallocate, MessageCommon, PresenceBits};
use crate::fields::wire::map_element::{MapKey, MapValueView};
use crate::fields::wire::repeated_element::{
    RepeatedElement, RepeatedElementMerge, RepeatedElementMut,
};

use super::entry::{decode_map_entry, encode_map_entry, entry_payload_len};

/// Map field: key marker `K`, value marker `V`, field number `FIELD`, allocator `A`.
///
/// Stores an allocator-owning `HashMap<K::Element<A>, V::Element<A>>` (unlike
/// `UnmanagedVec` repeated fields). Wire order is unspecified.
pub struct MapField<K, V, const FIELD: u32, A>
where
    K: MapKey,
    V: RepeatedElement,
    A: Allocator + Clone,
{
    entries: HashMap<K::Element<A>, V::Element<A>, DefaultHashBuilder, A>,
}

impl<K, V, const FIELD: u32, A> MapField<K, V, FIELD, A>
where
    K: MapKey,
    V: RepeatedElement,
    A: Allocator + Clone,
{
    pub fn new_in(alloc: A) -> Self {
        Self {
            entries: HashMap::with_hasher_in(DefaultHashBuilder::default(), alloc),
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
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
}

impl<K, V, const FIELD: u32, A, Pb> FieldDeallocate<Pb, A> for MapField<K, V, FIELD, A>
where
    K: MapKey,
    V: RepeatedElement,
    A: Allocator + Clone,
    Pb: PresenceBits,
{
    #[inline]
    fn deallocate(&mut self, common: &MessageCommon<Pb, A>) {
        let alloc = common.alloc.clone();
        for (k, v) in self.entries.drain() {
            // SAFETY: message allocator owns key / value payloads.
            unsafe {
                K::deallocate_element(k, alloc.clone());
                V::deallocate_element(v, alloc.clone());
            }
        }
    }
}

impl<K, V, const FIELD: u32, A, Pb> FieldPartialEq<Pb, A> for MapField<K, V, FIELD, A>
where
    K: MapKey,
    V: RepeatedElement,
    A: Allocator + Clone,
    Pb: PresenceBits,
    K::Element<A>: Eq + Hash,
    V::Element<A>: PartialEq,
{
    #[inline]
    fn field_eq(
        &self,
        common: &MessageCommon<Pb, A>,
        other: &Self,
        other_common: &MessageCommon<Pb, A>,
    ) -> bool {
        if self.len() != other.len() {
            return false;
        }
        self.bind(common)
            .iter()
            .all(|(k, v)| other.bind(other_common).get(k) == Some(v))
    }
}

impl<K, V, const FIELD: u32, A, Pb> FieldDebug<Pb, A> for MapField<K, V, FIELD, A>
where
    K: MapKey,
    V: RepeatedElement,
    A: Allocator + Clone,
    Pb: PresenceBits,
    K::Element<A>: Eq + Hash + Debug,
    V::Element<A>: Debug,
{
    #[inline]
    fn fmt_debug(&self, _common: &MessageCommon<Pb, A>, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_map().entries(&self.entries).finish()
    }
}

impl<K, V, const FIELD: u32, A, Pb> FieldEncode<Pb, A> for MapField<K, V, FIELD, A>
where
    K: MapKey,
    V: RepeatedElement,
    A: Allocator + Clone,
    Pb: PresenceBits,
    K::Element<A>: Eq + Hash,
{
    fn wire_encoded_len(&self, _common: &MessageCommon<Pb, A>) -> usize {
        let mut n = 0;
        for (key, value) in &self.entries {
            let payload = entry_payload_len::<K, V, A>(key, value);
            n += encode::encoded_len_len_field(FIELD, payload);
        }
        n
    }

    fn wire_encode_raw<B: BufMut>(&self, _common: &MessageCommon<Pb, A>, buf: &mut B) {
        for (key, value) in &self.entries {
            encode_map_entry::<K, V, A, B>(FIELD, key, value, buf);
        }
    }
}

impl<K, V, const FIELD: u32, A, Pb> FieldCloneIn<Pb, A> for MapField<K, V, FIELD, A>
where
    K: MapKey,
    V: RepeatedElement,
    A: Allocator + Clone,
    Pb: PresenceBits,
    K::Element<A>: CloneIn<A> + Eq + Hash,
    V::Element<A>: CloneIn<A>,
{
    fn clone_field(&self, _common: &MessageCommon<Pb, A>, alloc: A) -> Self {
        let mut out = HashMap::with_capacity_and_hasher_in(
            self.entries.len(),
            DefaultHashBuilder::default(),
            alloc.clone(),
        );
        out.extend(
            self.entries
                .iter()
                .map(|(k, v)| (k.clone_in(alloc.clone()), v.clone_in(alloc.clone()))),
        );
        Self { entries: out }
    }
}

/// Short-lived shared binding of a map field to its message common state.
pub struct MapFieldRef<'a, K, V, const FIELD: u32, A, Pb>
where
    K: MapKey,
    V: RepeatedElement,
    A: Allocator + Clone,
    Pb: PresenceBits,
{
    field: &'a MapField<K, V, FIELD, A>,
    #[allow(dead_code)]
    common: &'a MessageCommon<Pb, A>,
}

impl<'a, K, V, const FIELD: u32, A, Pb> MapFieldRef<'a, K, V, FIELD, A, Pb>
where
    K: MapKey,
    V: RepeatedElement,
    A: Allocator + Clone,
    Pb: PresenceBits,
{
    #[inline]
    fn new(field: &'a MapField<K, V, FIELD, A>, common: &'a MessageCommon<Pb, A>) -> Self {
        Self { field, common }
    }

    #[inline]
    pub fn get<Q>(self, key: &Q) -> Option<&'a V::Element<A>>
    where
        K::Element<A>: Eq + Hash,
        Q: ?Sized + Hash + Equivalent<K::Element<A>>,
    {
        self.field.entries.get(key)
    }

    #[inline]
    pub fn iter(self) -> HashMapIter<'a, K::Element<A>, V::Element<A>> {
        self.field.entries.iter()
    }
}

/// Short-lived binding of a map field to its message common state.
pub struct MapFieldMut<'f, 'c, K, V, const FIELD: u32, A, Pb>
where
    K: MapKey,
    V: RepeatedElement,
    A: Allocator + Clone,
    Pb: PresenceBits,
{
    field: &'f mut MapField<K, V, FIELD, A>,
    common: &'c mut MessageCommon<Pb, A>,
}

impl<'f, 'c, K, V, const FIELD: u32, A, Pb> MapFieldMut<'f, 'c, K, V, FIELD, A, Pb>
where
    K: MapKey,
    V: RepeatedElement,
    A: Allocator + Clone,
    Pb: PresenceBits,
{
    #[inline]
    fn new(field: &'f mut MapField<K, V, FIELD, A>, common: &'c mut MessageCommon<Pb, A>) -> Self {
        Self { field, common }
    }

    /// Mutable element handle (string/bytes → guard; scalars/messages → `&mut`).
    #[inline]
    pub fn get_element_mut<Q>(&mut self, key: &Q) -> Option<V::ElementMut<'_, A>>
    where
        V: RepeatedElementMut,
        K::Element<A>: Eq + Hash,
        Q: ?Sized + Hash + Equivalent<K::Element<A>>,
    {
        let alloc = self.common.alloc.clone();
        self.field
            .entries
            .get_mut(key)
            // SAFETY: message allocator owns heap-backed elements.
            .map(|elem| unsafe { V::element_mut(elem, alloc) })
    }

    /// Ensures `key` exists (type-default value if vacant), then returns a value mut handle.
    pub fn entry_element_mut_view(&mut self, key: &K::KeyView) -> V::ElementMut<'_, A>
    where
        V: RepeatedElementMut + RepeatedElementMerge<A>,
        K::Element<A>: Eq + Hash + Borrow<K::KeyView>,
    {
        if self.field.entries.get(key).is_none() {
            let owned_key = K::key_from_view(key, self.common.alloc.clone());
            let value = V::default_element(self.common.alloc.clone());
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
        let discarded = if let Some(slot) = self.field.entries.get_mut(&key) {
            let previous = mem::replace(slot, value);
            Some((key, previous))
        } else {
            self.field.entries.insert(key, value);
            None
        };
        if let Some((discarded_key, old_value)) = discarded {
            let alloc = self.common.alloc.clone();
            // SAFETY: message allocator owns discarded key / replaced value.
            unsafe {
                K::deallocate_element(discarded_key, alloc.clone());
                V::deallocate_element(old_value, alloc);
            }
        }
    }

    pub fn remove<Q>(&mut self, key: &Q)
    where
        K::Element<A>: Eq + Hash,
        Q: ?Sized + Hash + Equivalent<K::Element<A>>,
    {
        if let Some((old_key, old_value)) = self.field.entries.remove_entry(key) {
            let alloc = self.common.alloc.clone();
            // SAFETY: message allocator owns removed key / value payloads.
            unsafe {
                K::deallocate_element(old_key, alloc.clone());
                V::deallocate_element(old_value, alloc);
            }
        }
    }

    pub fn clear(&mut self) {
        let alloc = self.common.alloc.clone();
        for (k, v) in self.field.entries.drain() {
            // SAFETY: message allocator owns key / value payloads.
            unsafe {
                K::deallocate_element(k, alloc.clone());
                V::deallocate_element(v, alloc.clone());
            }
        }
    }

    /// Merges one map-entry LEN occurrence (last-wins on duplicate keys).
    pub fn merge<B: Buf>(
        &mut self,
        wire_type: WireType,
        buf: &mut B,
        depth: usize,
    ) -> Result<(), DecodeError>
    where
        K: RepeatedElementMerge<A>,
        V: RepeatedElementMerge<A>,
        K::Element<A>: Eq + Hash,
    {
        if wire_type != WireType::Len {
            return Err(DecodeError::InvalidTag);
        }
        let len = decode::decode_varint(buf)? as usize;
        if buf.remaining() < len {
            return Err(DecodeError::TruncatedMessage);
        }
        // Concrete `&[u8]` avoids `Take<…>` monomorphization blow-up (same as
        // nested `ProtoMessage` merge).
        let payload = buf.copy_to_bytes(len);
        let mut sub: &[u8] = payload.as_ref();
        let alloc = self.common.alloc.clone();
        let (key, value) = decode_map_entry::<K, V, A, _>(&mut sub, alloc, depth)?;
        self.insert(key, value);
        Ok(())
    }
}

// Blanket `puroro::{MapRef, MapMut}` over catalog bind views (view types from
// `MapKey::KeyView` / `MapValueView::View`).

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
    K: MapKey,
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

#[cfg(test)]
mod tests {
    use super::super::entry::encode_map_entry;
    use super::MapField;
    use crate::decode::decode_tag;
    use crate::encode::{encode_tag, encode_varint, encode_varint_field};
    use crate::fields::shared::field_inspect::FieldEncode;
    use crate::fields::shared::{FieldDeallocate, MessageCommon};
    use crate::fields::wire::{ProtoInt32, ProtoString};
    use ::allocator_api2::alloc::Global;
    use ::bitvec::array::BitArray;
    use ::bitvec::order::Lsb0;
    use ::bytes::BytesMut;
    use ::puroro::WireType;
    use ::unmanaged::UnmanagedString;

    #[test]
    fn map_field_int_markers() {
        let mut common =
            MessageCommon::<BitArray<[u8; 1], Lsb0>, _>::new_in(BitArray::ZERO, Global);
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
            MessageCommon::<BitArray<[u8; 1], Lsb0>, _>::new_in(BitArray::ZERO, Global);
        let mut field = MapField::<ProtoInt32, ProtoInt32, 7, _>::new_in(Global);
        field.bind_mut(&mut common).insert(1, 10);
        field.bind_mut(&mut common).insert(2, 20);

        let mut buf = BytesMut::new();
        field.wire_encode_raw(&common, &mut buf);
        assert_eq!(field.wire_encoded_len(&common), buf.len());

        let mut decoded = MapField::<ProtoInt32, ProtoInt32, 7, _>::new_in(Global);
        let mut rest = buf.as_ref();
        while !rest.is_empty() {
            let (field_number, wire_type) = decode_tag(&mut rest).unwrap();
            assert_eq!(field_number, 7);
            decoded
                .bind_mut(&mut common)
                .merge(wire_type, &mut rest, 0)
                .unwrap();
        }
        assert_eq!(decoded.bind(&common).get(&1), Some(&10));
        assert_eq!(decoded.bind(&common).get(&2), Some(&20));

        // Second merge of key=1 overwrites.
        let mut one = BytesMut::new();
        encode_map_entry::<ProtoInt32, ProtoInt32, Global, _>(7, &1, &99, &mut one);
        let mut rest = one.as_ref();
        let (_, wt) = decode_tag(&mut rest).unwrap();
        decoded
            .bind_mut(&mut common)
            .merge(wt, &mut rest, 0)
            .unwrap();
        assert_eq!(decoded.bind(&common).get(&1), Some(&99));
        assert_eq!(decoded.len(), 2);

        field.deallocate(&common);
        decoded.deallocate(&common);
    }

    #[test]
    fn map_entry_missing_key_defaults_to_zero() {
        let mut common =
            MessageCommon::<BitArray<[u8; 1], Lsb0>, _>::new_in(BitArray::ZERO, Global);
        let mut field = MapField::<ProtoInt32, ProtoInt32, 1, _>::new_in(Global);

        // Entry body: only value=2 with 42 (tag 0x10, varint 42).
        let mut entry_body = BytesMut::new();
        encode_varint_field(2, 42, &mut entry_body);

        let mut framed = BytesMut::new();
        encode_tag(1, WireType::Len, &mut framed);
        encode_varint(entry_body.len() as u64, &mut framed);
        framed.extend_from_slice(&entry_body);

        let mut rest = framed.as_ref();
        let (_, wt) = decode_tag(&mut rest).unwrap();
        field.bind_mut(&mut common).merge(wt, &mut rest, 0).unwrap();
        assert_eq!(field.bind(&common).get(&0), Some(&42));
        field.deallocate(&common);
    }

    fn unmanaged_str(s: &str) -> UnmanagedString<Global> {
        UnmanagedString::from_string(::unmanaged::String::from_str_in(s, Global))
    }

    #[test]
    fn map_string_key_roundtrip() {
        let mut common =
            MessageCommon::<BitArray<[u8; 1], Lsb0>, _>::new_in(BitArray::ZERO, Global);
        let mut field = MapField::<ProtoString, ProtoInt32, 3, _>::new_in(Global);
        field.bind_mut(&mut common).insert(unmanaged_str("ab"), 7);

        let mut buf = BytesMut::new();
        field.wire_encode_raw(&common, &mut buf);

        let mut decoded = MapField::<ProtoString, ProtoInt32, 3, _>::new_in(Global);
        let mut rest = buf.as_ref();
        let (_, wt) = decode_tag(&mut rest).unwrap();
        decoded
            .bind_mut(&mut common)
            .merge(wt, &mut rest, 0)
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
            MessageCommon::<BitArray<[u8; 1], Lsb0>, _>::new_in(BitArray::ZERO, Global);
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
            MessageCommon::<BitArray<[u8; 1], Lsb0>, _>::new_in(BitArray::ZERO, Global);

        let encode_entry =
            |common: &mut MessageCommon<BitArray<[u8; 1], Lsb0>, Global>, value: i32| -> BytesMut {
                let mut src = MapField::<ProtoString, ProtoInt32, 1, _>::new_in(Global);
                src.bind_mut(common).insert(unmanaged_str("k"), value);
                let mut buf = BytesMut::new();
                src.wire_encode_raw(common, &mut buf);
                src.deallocate(common);
                buf
            };
        let first = encode_entry(&mut common, 1);
        let second = encode_entry(&mut common, 9);

        let mut field = MapField::<ProtoString, ProtoInt32, 1, _>::new_in(Global);
        for framed in [first, second] {
            let mut rest = framed.as_ref();
            let (_, wt) = decode_tag(&mut rest).unwrap();
            field.bind_mut(&mut common).merge(wt, &mut rest, 0).unwrap();
        }

        assert_eq!(field.len(), 1);
        assert_eq!(field.bind(&common).get("k"), Some(&9));
        field.deallocate(&common);
    }

    /// Collision must also release the previous `UnmanagedString` value (not only the key).
    #[test]
    fn insert_duplicate_string_key_releases_previous_string_value() {
        let mut common =
            MessageCommon::<BitArray<[u8; 1], Lsb0>, _>::new_in(BitArray::ZERO, Global);
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
}
