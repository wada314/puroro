//! Catalog wrapper for a protobuf `map<K, V>` field.
//!
//! - `K: `[`MapKey`] — key marker (integral / `bool` / `string`)
//! - `V: `[`RepeatedElement`] — value marker (same element types as repeated)
//!
//! On the wire each entry is a LEN message with `key = 1` and `value = 2`.

use ::core::fmt::{Debug, Formatter, Result as FmtResult};
use ::core::hash::Hash;

use ::allocator_api2::alloc::Allocator;
use ::bytes::{Buf, BufMut};
use ::hashbrown::Equivalent;
use ::unmanaged::CloneIn;

use ::puroro::{DecodeError, WireType};

use crate::decode;
use crate::encode;
use crate::fields::shared::field_inspect::{FieldCloneIn, FieldDebug, FieldEncode, FieldPartialEq};
use crate::fields::shared::{FieldDeallocate, MessageCommon, PresenceBits};
use crate::fields::wire::map_element::MapKey;
use crate::fields::wire::repeated_element::{
    RepeatedElement, RepeatedElementMerge, RepeatedElementMut, RepeatedSlicePush,
};

use super::entries::MapEntries;
use super::entry::{decode_map_entry, encode_map_entry, entry_payload_len};

mod user_traits;

/// Map field: key marker `K`, value marker `V`, field number `FIELD`, allocator `A`.
///
/// Stores `HashMap<K::Element<A>, V::Element<A>>`.
pub struct MapField<K, V, const FIELD: u32, A>
where
    K: MapKey,
    V: RepeatedElement,
    A: Allocator + Clone,
{
    entries: MapEntries<K::Element<A>, V::Element<A>, A>,
}

impl<K, V, const FIELD: u32, A> MapField<K, V, FIELD, A>
where
    K: MapKey,
    V: RepeatedElement,
    A: Allocator + Clone,
{
    pub fn new_in(alloc: A) -> Self {
        Self {
            entries: MapEntries::new_in(alloc),
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

    /// Total wire length of all map entries (each as a LEN field `FIELD`).
    pub fn encoded_len<Pb>(&self, _common: &MessageCommon<Pb, A>) -> usize
    where
        Pb: PresenceBits,
        K::Element<A>: Eq + Hash,
    {
        let mut n = 0;
        for (key, value) in self.entries.iter() {
            let payload = entry_payload_len::<K, V, A>(key, value);
            n += encode::encoded_len_len_field(FIELD, payload);
        }
        n
    }

    /// Encodes all map entries (order unspecified).
    pub fn encode_raw<Pb, B: BufMut>(&self, _common: &MessageCommon<Pb, A>, buf: &mut B)
    where
        Pb: PresenceBits,
        K::Element<A>: Eq + Hash,
    {
        for (key, value) in self.entries.iter() {
            encode_map_entry::<K, V, A, B>(FIELD, key, value, buf);
        }
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
        K::Element<A>: CloneIn<A> + Eq + Hash,
        V::Element<A>: CloneIn<A>,
    {
        Self {
            entries: self.entries.clone_in(alloc),
        }
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
        f.debug_map().entries(self.entries.iter()).finish()
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
    #[inline]
    fn wire_encoded_len(&self, common: &MessageCommon<Pb, A>) -> usize {
        self.encoded_len(common)
    }

    #[inline]
    fn wire_encode_raw<B: BufMut>(&self, common: &MessageCommon<Pb, A>, buf: &mut B) {
        self.encode_raw(common, buf);
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
    #[inline]
    fn clone_field(&self, common: &MessageCommon<Pb, A>, alloc: A) -> Self {
        self.clone_in(common, alloc)
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
    pub fn len(self) -> usize {
        self.field.len()
    }

    #[inline]
    pub fn is_empty(self) -> bool {
        self.field.is_empty()
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
    pub fn iter(self) -> super::entries::MapEntriesIter<'a, K::Element<A>, V::Element<A>> {
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

    #[inline]
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V::Element<A>>
    where
        K::Element<A>: Eq + Hash,
        Q: ?Sized + Hash + Equivalent<K::Element<A>>,
    {
        self.field.entries.get_mut(key)
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

    /// Ensures `key` exists (type-default value if vacant). Sized keys must be `Copy`.
    pub fn entry_element_mut(&mut self, key: K::Element<A>) -> V::ElementMut<'_, A>
    where
        V: RepeatedElementMut + RepeatedElementMerge<A>,
        K::Element<A>: Copy + Eq + Hash,
    {
        if self.field.entries.get(&key).is_none() {
            let value = V::default_element(self.common.alloc.clone());
            let _ = self.field.entries.insert(key, value);
        }
        self.get_element_mut(&key)
            .expect("map entry present after insert")
    }

    /// Ensures a string key exists and returns a value mut handle.
    pub fn entry_element_mut_str(&mut self, key: &str) -> V::ElementMut<'_, A>
    where
        K: RepeatedSlicePush,
        V: RepeatedElementMut + RepeatedElementMerge<A>,
        K::Element<A>: Eq + Hash,
        str: Equivalent<K::Element<A>>,
    {
        if self.field.entries.get(key).is_none() {
            // Key is already UTF-8; `element_from_slice` only fails on invalid UTF-8.
            let owned_key = K::element_from_slice(key.as_bytes(), self.common.alloc.clone())
                .expect("str is valid UTF-8");
            let value = V::default_element(self.common.alloc.clone());
            self.insert(owned_key, value);
        }
        self.get_element_mut(key)
            .expect("map entry present after insert")
    }

    /// Inserts with last-wins. Replaced value and discarded key are released.
    pub fn insert(&mut self, key: K::Element<A>, value: V::Element<A>)
    where
        K::Element<A>: Eq + Hash,
    {
        if let Some((discarded_key, old_value)) = self.field.entries.insert(key, value) {
            let alloc = self.common.alloc.clone();
            // SAFETY: message allocator owns discarded key / replaced value.
            unsafe {
                K::deallocate_element(discarded_key, alloc.clone());
                V::deallocate_element(old_value, alloc);
            }
        }
    }

    /// Builds a `map<string, …>` key from `&str` and inserts.
    pub fn insert_str(&mut self, key: &str, value: V::Element<A>)
    where
        K: RepeatedSlicePush,
        K::Element<A>: Eq + Hash,
    {
        let owned_key = K::element_from_slice(key.as_bytes(), self.common.alloc.clone())
            .expect("str is valid UTF-8");
        self.insert(owned_key, value);
    }

    pub fn remove<Q>(&mut self, key: &Q)
    where
        K::Element<A>: Eq + Hash,
        Q: ?Sized + Hash + Equivalent<K::Element<A>>,
    {
        if let Some((old_key, old_value)) = self.field.entries.remove(key) {
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

    #[inline]
    pub fn len(&self) -> usize {
        self.field.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.field.is_empty()
    }

    #[inline]
    pub fn get<Q>(&self, key: &Q) -> Option<&V::Element<A>>
    where
        K::Element<A>: Eq + Hash,
        Q: ?Sized + Hash + Equivalent<K::Element<A>>,
    {
        self.field.entries.get(key)
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

#[cfg(test)]
mod tests {
    use super::super::entry::encode_map_entry;
    use super::MapField;
    use crate::decode::decode_tag;
    use crate::encode::{encode_tag, encode_varint, encode_varint_field};
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
        assert!(field.is_empty());
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
        field.encode_raw(&common, &mut buf);
        assert_eq!(field.encoded_len(&common), buf.len());

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

    #[test]
    fn map_string_key_roundtrip() {
        let mut common =
            MessageCommon::<BitArray<[u8; 1], Lsb0>, _>::new_in(BitArray::ZERO, Global);
        let mut field = MapField::<ProtoString, ProtoInt32, 3, _>::new_in(Global);
        let key = UnmanagedString::from_string(::unmanaged::String::from_str_in("ab", Global));
        field.bind_mut(&mut common).insert(key, 7);

        let mut buf = BytesMut::new();
        field.encode_raw(&common, &mut buf);

        let mut decoded = MapField::<ProtoString, ProtoInt32, 3, _>::new_in(Global);
        let mut rest = buf.as_ref();
        let (_, wt) = decode_tag(&mut rest).unwrap();
        decoded
            .bind_mut(&mut common)
            .merge(wt, &mut rest, 0)
            .unwrap();
        assert_eq!(decoded.bind(&common).get("ab"), Some(&7));

        // Duplicate string key: must not panic on discarded UnmanagedString key.
        let key2 = UnmanagedString::from_string(::unmanaged::String::from_str_in("ab", Global));
        decoded.bind_mut(&mut common).insert(key2, 8);
        assert_eq!(decoded.bind(&common).get("ab"), Some(&8));

        field.deallocate(&common);
        decoded.deallocate(&common);
    }
}
