//! Unified repeated field wrapper — generic over element marker and encode policy.
//!
//! [`RepeatedReady`] (the default `L`) stores elements in an allocator-less
//! [`UnmanagedVec`] wrapped in [`ManuallyDrop`]. Growth and release borrow the
//! message allocator. [`RepeatedSpans`] stores element-LEN offsets and builds
//! that vec on first get.

use ::core::cell::UnsafeCell;
use ::core::fmt::{Debug, Formatter, Result as FmtResult};
use ::core::marker::PhantomData;
use ::core::mem::ManuallyDrop;

use ::allocator_api2::alloc::Allocator;
use ::allocator_api2::vec::Vec as AllocVec;
use ::bytes::BufMut;
use ::puroro::{DecodeBuf, DecodeError, WireType};
use ::unmanaged::CloneIn;
use ::unmanaged::DeallocateIn;
use ::unmanaged::UnmanagedString;
use ::unmanaged::UnmanagedVec;
use ::unmanaged::vec::VecGuard;

use crate::decode::{LazyMessage, SharedWire, WireSpan};
use crate::encode::field_number_const;
use crate::fields::shared::field_inspect::{FieldCloneIn, FieldDebug, FieldEncode, FieldPartialEq};
use crate::fields::shared::{DefaultIn, FieldDeallocate, MessageCommon, MessageCommonAlloc};
use crate::fields::wire::len::{ProtoBytes, ProtoString, ProtoStringUnchecked};
use crate::fields::wire::proto_message::ProtoMessage;
use crate::fields::wire::repeated_element::{
    RepeatedElement, RepeatedElementMerge, RepeatedElementMut, RepeatedVecMut,
};
use crate::message_encode::{EncodeCtx, MessageEncode};

use super::container::RepeatedElementsMut;
use super::encoding::RepeatedEncoding;

/// How a [`RepeatedField`] holds elements.
///
/// [`RepeatedReady`] is the element vec. [`RepeatedSpans`] is an offset list
/// plus an optional materialised vec. Unlike oneof's ingest `L`, this
/// parameter changes the field's own storage.
pub trait RepeatedLayout<T, E, const FIELD: u32, A>
where
    T: RepeatedElement,
    E: RepeatedEncoding<T, A>,
    A: Allocator,
{
    /// Payload stored on [`RepeatedField`].
    type Storage;

    /// Empty storage using `alloc` (element vec or the span vec).
    fn new_storage(alloc: A) -> Self::Storage;

    /// Release `storage` through `common`'s allocator.
    fn deallocate_storage<C>(storage: &mut Self::Storage, common: &C)
    where
        C: MessageCommonAlloc<Alloc = A>,
        T::Element<A>: DeallocateIn<A>;
}

/// Eager repeated: the element vec is the field. Occurrences land during `merge`.
pub struct RepeatedReady;

/// Lazy repeated LEN: element-LEN offsets during the parent scan; vec on first get.
pub struct RepeatedSpans;

impl<T, E, const FIELD: u32, A> RepeatedLayout<T, E, FIELD, A> for RepeatedReady
where
    T: RepeatedElement,
    E: RepeatedEncoding<T, A>,
    A: Allocator,
{
    type Storage = ManuallyDrop<UnmanagedVec<T::Element<A>, A>>;

    #[inline]
    fn new_storage(alloc: A) -> Self::Storage {
        ManuallyDrop::new(UnmanagedVec::new(alloc))
    }

    #[inline]
    fn deallocate_storage<C>(storage: &mut Self::Storage, common: &C)
    where
        C: MessageCommonAlloc<Alloc = A>,
        T::Element<A>: DeallocateIn<A>,
    {
        // SAFETY: called once; `&common.alloc` is interchangeable with the
        // clones that grew the buffer. Reconstructs `Vec<T, &A>` for the free.
        let v = unsafe { ManuallyDrop::take(storage) };
        unsafe { v.deallocate(common.alloc()) };
    }
}

/// Offset list plus an optional materialised [`RepeatedField`] (`L = `[`RepeatedReady`]).
#[doc(hidden)]
pub struct RepeatedSpanStorage<T, E, const FIELD: u32, A>
where
    T: RepeatedElement,
    E: RepeatedEncoding<T, A>,
    A: Allocator,
{
    spans: AllocVec<WireSpan, A>,
    ready: UnsafeCell<Option<RepeatedField<T, E, FIELD, A>>>,
}

impl<T, E, const FIELD: u32, A> RepeatedLayout<T, E, FIELD, A> for RepeatedSpans
where
    T: RepeatedElement,
    E: RepeatedEncoding<T, A>,
    A: Allocator,
{
    type Storage = RepeatedSpanStorage<T, E, FIELD, A>;

    #[inline]
    fn new_storage(alloc: A) -> Self::Storage {
        RepeatedSpanStorage {
            spans: AllocVec::new_in(alloc),
            ready: UnsafeCell::new(None),
        }
    }

    #[inline]
    fn deallocate_storage<C>(storage: &mut Self::Storage, common: &C)
    where
        C: MessageCommonAlloc<Alloc = A>,
        T::Element<A>: DeallocateIn<A>,
    {
        if let Some(mut field) = storage.ready.get_mut().take() {
            FieldDeallocate::deallocate(&mut field, common);
        }
    }
}

/// Decode a scanned LEN **body** (length prefix already consumed).
pub trait DecodeLenBody<A: Allocator>: RepeatedElement {
    fn decode_len_body(bytes: &[u8], alloc: A) -> Result<Self::Element<A>, DecodeError>;
}

impl<A: Allocator + Clone> DecodeLenBody<A> for ProtoString {
    fn decode_len_body(bytes: &[u8], alloc: A) -> Result<UnmanagedString<A>, DecodeError> {
        let mut vec = AllocVec::with_capacity_in(bytes.len(), alloc.clone());
        vec.extend_from_slice(bytes);
        match UnmanagedString::from_utf8(UnmanagedVec::from_vec(vec)) {
            Ok(s) => Ok(s),
            Err(v) => {
                // SAFETY: `alloc` owns the buffer produced above.
                unsafe { v.deallocate(&alloc) };
                Err(DecodeError::InvalidUtf8)
            }
        }
    }
}

impl<A: Allocator + Clone> DecodeLenBody<A> for ProtoBytes {
    fn decode_len_body(bytes: &[u8], alloc: A) -> Result<UnmanagedVec<u8, A>, DecodeError> {
        let mut vec = AllocVec::with_capacity_in(bytes.len(), alloc);
        vec.extend_from_slice(bytes);
        Ok(UnmanagedVec::from_vec(vec))
    }
}

impl<A: Allocator + Clone> DecodeLenBody<A> for ProtoStringUnchecked {
    fn decode_len_body(bytes: &[u8], alloc: A) -> Result<UnmanagedVec<u8, A>, DecodeError> {
        ProtoBytes::decode_len_body(bytes, alloc)
    }
}

/// Repeated field parametrised by type marker `T`, encode policy `E`, and allocator `A`.
///
/// Parameter order: `T`, `E`, `FIELD`, `A`, `L`. `L` is [`RepeatedReady`]
/// (default) or [`RepeatedSpans`].
pub struct RepeatedField<T, E, const FIELD: u32, A, L = RepeatedReady>
where
    T: RepeatedElement,
    E: RepeatedEncoding<T, A>,
    A: Allocator,
    L: RepeatedLayout<T, E, FIELD, A>,
{
    storage: L::Storage,
    _encoding: PhantomData<E>,
}

impl<T, E, const FIELD: u32, A, L> RepeatedField<T, E, FIELD, A, L>
where
    T: RepeatedElement,
    E: RepeatedEncoding<T, A>,
    A: Allocator,
    L: RepeatedLayout<T, E, FIELD, A>,
{
    pub fn new_in(alloc: A) -> Self {
        Self {
            storage: L::new_storage(alloc),
            _encoding: PhantomData,
        }
    }
}

impl<T, E, const FIELD: u32, A> RepeatedField<T, E, FIELD, A>
where
    T: RepeatedElement,
    E: RepeatedEncoding<T, A>,
    A: Allocator,
{
    #[inline]
    pub fn as_slice(&self) -> &[T::Element<A>] {
        &self.storage
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }

    /// Binds this field to `common` for read access.
    #[inline]
    pub fn bind<'a, Cx: MessageCommonAlloc<Alloc = A>>(
        &'a self,
        common: &'a Cx,
    ) -> RepeatedFieldRef<'a, T, E, FIELD, A, Cx> {
        RepeatedFieldRef::new(self, common)
    }

    /// Binds this field to `common` for mutation.
    #[inline]
    pub fn bind_mut<'f, 'c, Cx: MessageCommonAlloc<Alloc = A>>(
        &'f mut self,
        common: &'c mut Cx,
    ) -> RepeatedFieldMut<'f, 'c, T, E, FIELD, A, Cx> {
        RepeatedFieldMut::new(self, common)
    }
}

impl<T, E, const FIELD: u32, A, L, C> FieldDeallocate<C> for RepeatedField<T, E, FIELD, A, L>
where
    T: RepeatedElement,
    E: RepeatedEncoding<T, A>,
    A: Allocator,
    L: RepeatedLayout<T, E, FIELD, A>,
    C: MessageCommonAlloc<Alloc = A>,
    T::Element<A>: DeallocateIn<A>,
{
    /// Releases every element (when heap-backed) and the backing buffer.
    #[inline]
    fn deallocate(&mut self, common: &C) {
        L::deallocate_storage(&mut self.storage, common);
    }
}

// ---------------------------------------------------------------------------
// Read view
// ---------------------------------------------------------------------------

/// Short-lived shared binding of a repeated field to its message common state.
pub struct RepeatedFieldRef<
    'a,
    T: RepeatedElement,
    E: RepeatedEncoding<T, A>,
    const FIELD: u32,
    A: Allocator,
    Cx,
> {
    field: &'a RepeatedField<T, E, FIELD, A>,
    /// Bound for symmetry with [`RepeatedFieldMut`]; unused by current getters.
    #[allow(dead_code)]
    common: &'a Cx,
    _encoding: PhantomData<E>,
}

impl<'a, T, E, const FIELD: u32, A, Cx> RepeatedFieldRef<'a, T, E, FIELD, A, Cx>
where
    T: RepeatedElement,
    E: RepeatedEncoding<T, A>,
    A: Allocator,
{
    #[inline]
    fn new(field: &'a RepeatedField<T, E, FIELD, A>, common: &'a Cx) -> Self {
        Self {
            field,
            common,
            _encoding: PhantomData,
        }
    }

    #[inline]
    pub fn as_slice(self) -> &'a [T::Element<A>] {
        self.field.as_slice()
    }

    #[inline]
    pub fn is_empty(self) -> bool {
        self.field.is_empty()
    }
}

// ---------------------------------------------------------------------------
// Mutation view
// ---------------------------------------------------------------------------

/// Short-lived binding of a repeated field to its message common state.
pub struct RepeatedFieldMut<
    'f,
    'c,
    T: RepeatedElement,
    E: RepeatedEncoding<T, A>,
    const FIELD: u32,
    A: Allocator,
    Cx: MessageCommonAlloc<Alloc = A>,
> {
    field: &'f mut RepeatedField<T, E, FIELD, A>,
    common: &'c mut Cx,
    _encoding: PhantomData<E>,
}

impl<'f, 'c, T, E, const FIELD: u32, A, Cx> RepeatedFieldMut<'f, 'c, T, E, FIELD, A, Cx>
where
    T: RepeatedElement,
    E: RepeatedEncoding<T, A>,
    A: Allocator,
    Cx: MessageCommonAlloc<Alloc = A>,
{
    #[inline]
    fn new(field: &'f mut RepeatedField<T, E, FIELD, A>, common: &'c mut Cx) -> Self {
        Self {
            field,
            common,
            _encoding: PhantomData,
        }
    }

    /// Growable handle over vec-mutable elements (`repeated int32`, message, …).
    pub fn values_mut(self) -> VecGuard<'f, T::Element<A>, A>
    where
        T: RepeatedVecMut,
        A: Clone,
    {
        let alloc = self.common.clone_alloc();
        // SAFETY: an owned clone of the message allocator owns this vector's
        // buffer.
        unsafe { self.field.storage.with_alloc(alloc) }
    }

    /// Minimal container mutator ([`RepeatedContainerMut`](super::container::RepeatedContainerMut)).
    ///
    /// Works for string / bytes as well as scalars and messages: [`push`](super::container::RepeatedContainerMut::push)
    /// appends a default element and returns a singular-style mut handle.
    pub fn container_mut(self) -> RepeatedElementsMut<'f, T, A>
    where
        T: RepeatedElementMut + RepeatedElementMerge<A>,
        A: Clone,
    {
        let alloc = self.common.clone_alloc();
        // SAFETY: an owned clone of the message allocator owns this vector's
        // buffer.
        RepeatedElementsMut::new(unsafe { self.field.storage.with_alloc(alloc) })
    }

    /// Empties the vector (keeps capacity). Heap elements are freed first.
    pub fn clear(self)
    where
        A: Clone,
    {
        let alloc = self.common.clone_alloc();
        // SAFETY: owned clones of the message allocator own this vector's buffer
        // and every element.
        let mut g = unsafe { self.field.storage.with_alloc(alloc.clone()) };
        while let Some(elem) = g.pop() {
            unsafe { T::deallocate_element(elem, &alloc) };
        }
    }

    /// Merges one wire occurrence — appends element(s).
    pub fn merge<B: DecodeBuf>(
        self,
        wire_type: WireType,
        buf: &mut B,
        depth: usize,
    ) -> Result<(), DecodeError>
    where
        T: RepeatedElementMerge<A>,
        A: Clone,
    {
        self.field
            .merge_from_wire(wire_type, buf, self.common.clone_alloc(), depth)
    }
}

impl<T, E, const FIELD: u32, A> RepeatedField<T, E, FIELD, A>
where
    T: RepeatedElement,
    E: RepeatedEncoding<T, A>,
    A: Allocator,
{
    /// Appends one wire occurrence using `alloc` (no [`MessageCommon`] borrow).
    pub fn merge_from_wire<B: DecodeBuf>(
        &mut self,
        wire_type: WireType,
        buf: &mut B,
        alloc: A,
        depth: usize,
    ) -> Result<(), DecodeError>
    where
        T: RepeatedElementMerge<A>,
        A: Clone,
    {
        // SAFETY: an owned clone of the message allocator owns this vector's
        // buffer.
        let mut g = unsafe { self.storage.with_alloc(alloc.clone()) };
        T::merge_occurrence(wire_type, buf, alloc, depth, |elem| {
            g.push(elem);
        })
    }

    /// Appends an already-decoded element using `alloc`.
    pub fn push_in(&mut self, elem: T::Element<A>, alloc: A)
    where
        A: Clone,
    {
        // SAFETY: an owned clone of the message allocator owns this vector's
        // buffer.
        let mut g = unsafe { self.storage.with_alloc(alloc) };
        g.push(elem);
    }
}

impl<T, E, const FIELD: u32, A> RepeatedField<T, E, FIELD, A, RepeatedSpans>
where
    T: RepeatedElement,
    E: RepeatedEncoding<T, A>,
    A: Allocator,
{
    /// Record one element LEN. If already materialised, append it now.
    pub fn store_span<Cx: MessageCommonAlloc<Alloc = A>>(
        &mut self,
        span: WireSpan,
        wire: &[u8],
        common: &Cx,
    ) -> Result<(), DecodeError>
    where
        A: Clone,
        T: DecodeLenBody<A>,
    {
        self.storage.spans.push(span);
        if let Some(field) = self.storage.ready.get_mut() {
            append_span(field, span, wire, common.clone_alloc())?;
        }
        Ok(())
    }

    /// Materialise on first call, then return the catalog repeated view.
    pub fn bind<'a, Cx: MessageCommonAlloc<Alloc = A>>(
        &'a self,
        wire: &'a [u8],
        common: &'a Cx,
    ) -> Result<RepeatedFieldRef<'a, T, E, FIELD, A, Cx>, DecodeError>
    where
        A: Clone,
        T: DecodeLenBody<A>,
        T::Element<A>: DeallocateIn<A>,
    {
        // SAFETY: callers only `bind` after the parent stream is finished, and
        // never overlap it with `store_span` (`&mut self`).
        let ready = unsafe { &mut *self.storage.ready.get() };
        if ready.is_none() {
            let mut field = RepeatedField::new_in(common.clone_alloc());
            for &span in &self.storage.spans {
                if let Err(e) = append_span(&mut field, span, wire, common.clone_alloc()) {
                    FieldDeallocate::deallocate(&mut field, common);
                    return Err(e);
                }
            }
            *ready = Some(field);
        }
        Ok(ready
            .as_ref()
            .expect("repeated field materialised")
            .bind(common))
    }
}

fn append_span<T, E, const FIELD: u32, A>(
    field: &mut RepeatedField<T, E, FIELD, A>,
    span: WireSpan,
    wire: &[u8],
    alloc: A,
) -> Result<(), DecodeError>
where
    T: DecodeLenBody<A>,
    E: RepeatedEncoding<T, A>,
    A: Allocator + Clone,
{
    let payload = span.slice(wire)?;
    let elem = T::decode_len_body(payload, alloc.clone())?;
    field.push_in(elem, alloc);
    Ok(())
}

impl<M, E, const FIELD: u32, A> RepeatedField<ProtoMessage<M>, E, FIELD, A>
where
    M: MessageEncode + LazyMessage<A> + DefaultIn<A>,
    E: RepeatedEncoding<ProtoMessage<M>, A>,
    A: Allocator + Clone,
{
    /// Append one still-lazy child that records `span` on the island-root buffer.
    ///
    /// Does not walk the child's tags. The child's first field getter parses
    /// stored regions. Distinct from singular last-wins: each occurrence is a
    /// new list element (same as eager repeated message).
    pub fn merge_shared<Cx: MessageCommonAlloc<Alloc = A>>(
        &mut self,
        root: &SharedWire<A>,
        span: WireSpan,
        common: &Cx,
    ) -> Result<(), DecodeError> {
        let mut child = M::default_in(common.clone_alloc());
        child.merge_shared(root, span)?;
        self.push_in(child, common.clone_alloc());
        Ok(())
    }
}

impl<T, E, const FIELD: u32, A, P> FieldPartialEq<MessageCommon<P, A>>
    for RepeatedField<T, E, FIELD, A>
where
    T: RepeatedElement,
    E: RepeatedEncoding<T, A>,
    A: Allocator,
    T::Element<A>: PartialEq,
{
    #[inline]
    fn field_eq(
        &self,
        _common: &MessageCommon<P, A>,
        other: &Self,
        _other_common: &MessageCommon<P, A>,
    ) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<T, E, const FIELD: u32, A, P> FieldDebug<MessageCommon<P, A>> for RepeatedField<T, E, FIELD, A>
where
    T: RepeatedElement,
    E: RepeatedEncoding<T, A>,
    A: Allocator,
    T::Element<A>: Debug,
{
    #[inline]
    fn fmt_debug(&self, _common: &MessageCommon<P, A>, f: &mut Formatter<'_>) -> FmtResult {
        Debug::fmt(self.as_slice(), f)
    }
}

impl<T, E, const FIELD: u32, A, P> FieldEncode<MessageCommon<P, A>>
    for RepeatedField<T, E, FIELD, A>
where
    T: RepeatedElement,
    E: RepeatedEncoding<T, A>,
    A: Allocator,
{
    fn encoded_len(&self, _common: &MessageCommon<P, A>, ctx: &mut EncodeCtx) -> usize {
        if self.storage.is_empty() {
            0
        } else {
            E::encoded_len(field_number_const::<FIELD>(), self.as_slice(), ctx)
        }
    }

    fn encode_raw<B: BufMut>(
        &self,
        _common: &MessageCommon<P, A>,
        ctx: &mut EncodeCtx,
        buf: &mut B,
    ) {
        if !self.storage.is_empty() {
            E::encode(field_number_const::<FIELD>(), self.as_slice(), ctx, buf);
        }
    }
}

impl<T, E, const FIELD: u32, A, P> FieldCloneIn<MessageCommon<P, A>>
    for RepeatedField<T, E, FIELD, A>
where
    T: RepeatedElement,
    E: RepeatedEncoding<T, A>,
    A: Allocator + Clone,
    T::Element<A>: CloneIn<A>,
{
    fn clone_field(&self, _common: &MessageCommon<P, A>, alloc: A) -> Self {
        Self {
            storage: ManuallyDrop::new(self.storage.clone_in(alloc)),
            _encoding: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::Expanded;
    use super::{RepeatedField, RepeatedSpans};
    use crate::decode::WireSpan;
    use crate::fields::shared::{FieldDeallocate, MessageCommon};
    use crate::fields::wire::ProtoString;
    use ::allocator_api2::alloc::Global;
    use ::bitvec::array::BitArray;
    use ::bitvec::order::Lsb0;
    use ::puroro::DecodeError;

    type TestCommon = MessageCommon<BitArray<[u8; 1], Lsb0>, Global>;

    #[test]
    fn repeated_spans_materialise_append_and_later_store() {
        let common = TestCommon::new_in(BitArray::ZERO, Global);
        let mut field = RepeatedField::<ProtoString, Expanded, 8, _, RepeatedSpans>::new_in(Global);

        let mut wire = b"urgentdocs".to_vec();
        field
            .store_span(WireSpan { offset: 0, len: 6 }, &wire, &common)
            .unwrap();
        field
            .store_span(WireSpan { offset: 6, len: 4 }, &wire, &common)
            .unwrap();

        let first = field.bind(&wire, &common).unwrap().as_slice();
        assert_eq!(first.len(), 2);
        assert_eq!(&*first[0], "urgent");
        assert_eq!(&*first[1], "docs");

        let late_off = wire.len();
        wire.extend_from_slice(b"more");
        field
            .store_span(
                WireSpan {
                    offset: late_off,
                    len: 4,
                },
                &wire,
                &common,
            )
            .unwrap();
        let after = field.bind(&wire, &common).unwrap().as_slice();
        assert_eq!(after.len(), 3);
        assert_eq!(&*after[2], "more");

        field.deallocate(&common);
    }

    #[test]
    fn repeated_spans_invalid_utf8_is_sticky() {
        let common = TestCommon::new_in(BitArray::ZERO, Global);
        let mut field = RepeatedField::<ProtoString, Expanded, 8, _, RepeatedSpans>::new_in(Global);
        let wire = [b'o', b'k', 0xff, 0xfe];
        field
            .store_span(WireSpan { offset: 0, len: 2 }, &wire, &common)
            .unwrap();
        field
            .store_span(WireSpan { offset: 2, len: 2 }, &wire, &common)
            .unwrap();
        assert_eq!(
            field.bind(&wire, &common).err(),
            Some(DecodeError::InvalidUtf8)
        );
        assert_eq!(
            field.bind(&wire, &common).err(),
            Some(DecodeError::InvalidUtf8)
        );
        field.deallocate(&common);
    }
}
