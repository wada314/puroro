//! One shell for every generated message.
//!
//! Generated code defines the field struct and implements [`RecordFields`].
//! [`Record`] owns [`MessageCommon`] plus that struct, and implements the
//! message-wide traits (`Drop`, `Clone`, [`puroro::Message`], …) once.

use ::allocator_api2::alloc::Allocator;
use ::bytes::{Buf, BufMut};
use ::core::fmt::{self, Debug};
use ::core::iter;
use ::core::mem;
use ::core::ops::ControlFlow;
use ::puroro::{DecodeBuf, DecodeError, Message, RECURSION_LIMIT, WireType};

use crate::decode::{
    Eager, Lazy, LazyMessage, LazyScan, MessageScan, ScannedRecord, decode_tag,
    skip_field_and_save_in,
};
use crate::fields::shared::{
    BitStorage, CloneBound, DeallocateBound, DebugStructVisitor, EncodeRawVisitor,
    EncodedLenVisitor, FieldDeallocVisitor, FieldEqVisitor, FieldPairVisitor, FieldVisitor,
    FieldVisitorMut, MessageBindingMut, MessageCommon, UnknownStore,
};
use crate::fields::slot_index::{BindField, BindFieldMut, HasField};
use crate::message_encode::{EncodeCtx, MessageEncode, encode_message, encode_message_to_vec};
use crate::message_merge::{MessageMerge, merge_message};
use crate::{CloneIn, DeallocateIn, DefaultIn};

/// Field payload of one generated message.
///
/// `L` is [`Eager`] or [`Lazy`]. The generated impl is generic over `L` so the
/// same field list serves both aliases.
pub trait RecordFields<A, L>: Sized
where
    A: Allocator + Clone,
    L: MessageScan<A>,
{
    /// Common-bit storage for this message (`BitArray` or interior bits).
    type Bits: BitStorage + Clone;
    /// Preserve, discard, or reject. Baked into the message alias.
    type Unknown: UnknownStore<A>;
    /// Eager alias of this message. Lazy `PartialEq` / `Debug` build one.
    type EagerMessage: PartialEq + Debug + Message<Alloc = A>;

    /// Debug name of the eager alias.
    const EAGER_NAME: &'static str;
    /// Debug name of the lazy alias.
    const LAZY_NAME: &'static str;
    /// `true` when the message declares a `LEGACY_REQUIRED` field.
    ///
    /// Lazy [`Message::validate`] scans stored
    /// regions only when this is `true`. A message with no required fields
    /// keeps the trait default and does not apply unparsed regions.
    const CHECKS_REQUIRED: bool = false;

    /// Empty common bits for `L` ([`Eager`] or [`Lazy`]).
    fn empty_bits() -> Self::Bits;

    /// Teardown walk (`Drop`).
    fn visit_mut<V: FieldVisitorMut<MessageCommon<Self::Bits, A, Self::Unknown, L>>>(
        &mut self,
        v: &mut V,
    ) -> ControlFlow<V::Break>;

    /// `LEGACY_REQUIRED` presence. Default succeeds.
    fn validate(
        &self,
        common: &MessageCommon<Self::Bits, A, Self::Unknown, L>,
    ) -> Result<(), DecodeError> {
        let _ = common;
        Ok(())
    }
}

/// Build the field slots.
///
/// Not part of [`RecordFields`]: `SingularField::new_in` needs a concrete
/// layout, and a `Layout` associated type does not carry that bound.
pub trait ConstructRecordFields<A, L>: RecordFields<A, L>
where
    A: Allocator + Clone,
    L: MessageScan<A>,
{
    /// Field slots only. [`Record::new_in`] builds the common state.
    fn new(alloc: A) -> Self;
}

/// Clone field slots.
///
/// Separate from [`RecordFields`] because [`FieldCloneIn`](crate::FieldCloneIn)
/// is implemented for a concrete layout (`InlineOrHeap`, [`crate::WireOrSso`]),
/// not for the `Layout` associated-type projection.
pub trait CloneRecordFields<A, L>: RecordFields<A, L>
where
    A: Allocator + Clone,
    L: MessageScan<A>,
{
    /// Replace `dst`'s slots with clones of `self`. `dst` comes from [`ConstructRecordFields::new`].
    fn clone_fields(
        &self,
        dst: &mut Self,
        src_common: &MessageCommon<Self::Bits, A, Self::Unknown, L>,
        dst_common: &MessageCommon<Self::Bits, A, Self::Unknown, L>,
        alloc: A,
    );
}

/// Eager encode, equality, debug, and merge.
///
/// Lazy aliases do not implement these walks: their LEN slots are
/// [`crate::WireOrSso`] spans, and wire encode copies the stored body.
pub trait EagerRecordFields<A>: RecordFields<A, Eager>
where
    A: Allocator + Clone,
{
    /// Immutable field walk (encode, `Debug`).
    fn visit_fields<V: FieldVisitor<MessageCommon<Self::Bits, A, Self::Unknown, Eager>>>(
        &self,
        v: &mut V,
    ) -> ControlFlow<V::Break>;

    /// Pair walk (`PartialEq`).
    fn visit_pairs<V: FieldPairVisitor<MessageCommon<Self::Bits, A, Self::Unknown, Eager>>>(
        &self,
        other: &Self,
        v: &mut V,
    ) -> ControlFlow<V::Break>;

    /// Merge one known field.
    ///
    /// Returns `false` when `field_number` is not a field of this message.
    /// The shell then preserves, discards, or rejects it.
    fn merge_known<B: DecodeBuf>(
        &mut self,
        common: &mut MessageCommon<Self::Bits, A, Self::Unknown, Eager>,
        field_number: u32,
        wire_type: WireType,
        buf: &mut B,
        depth: usize,
    ) -> Result<bool, DecodeError>;
}

/// Lazy ingest of one scanned record.
///
/// Separate from [`RecordFields`] because the body names `Lazy::scan`, which
/// the eager alias does not have.
pub trait LazyRecordApply<A>: RecordFields<A, Lazy<A>>
where
    A: Allocator + Clone,
{
    /// Apply one scanned record into the field slots.
    fn apply_record(
        &mut self,
        common: &mut MessageCommon<Self::Bits, A, Self::Unknown, Lazy<A>>,
        rec: &ScannedRecord,
        origin: usize,
    ) -> Result<(), DecodeError>;
}

/// Generated message: common state plus [`RecordFields`].
///
/// `common` and `fields` are visible to the generated crate. [`Self::field`]
/// binds a field number to `common` for getters. Decode still receives `fields`
/// and `common` apart. User code should call the generated accessors.
pub struct Record<F, A, L, U>
where
    A: Allocator + Clone,
    L: MessageScan<A>,
    U: UnknownStore<A>,
    F: RecordFields<A, L, Unknown = U>,
{
    #[doc(hidden)]
    pub common: MessageCommon<F::Bits, A, U, L>,
    #[doc(hidden)]
    pub fields: F,
}

impl<F, A, L, U> Record<F, A, L, U>
where
    A: Allocator + Clone,
    L: MessageScan<A>,
    U: UnknownStore<A>,
    F: RecordFields<A, L, Unknown = U>,
{
    /// Field `N` already bound to [`Self::common`](Self::common).
    ///
    /// The result is the same view `*Fields`'s projection returns from `bind`.
    /// Callers then use that view's inherent method (`optional`, `value`,
    /// `get`, `as_slice`, …).
    #[inline]
    pub fn field<'a, const N: u32>(
        &'a self,
    ) -> <F::Ref<'a> as BindField<'a, MessageCommon<F::Bits, A, U, L>>>::Bound
    where
        F: HasField<N>,
        F::Ref<'a>: BindField<'a, MessageCommon<F::Bits, A, U, L>>,
    {
        self.fields.field().bind_field(&self.common)
    }

    /// Mutable field `N` already bound to [`Self::common`](Self::common).
    #[inline]
    pub fn field_mut<'a, const N: u32>(
        &'a mut self,
    ) -> <F::Mut<'a> as BindFieldMut<'a, 'a, MessageCommon<F::Bits, A, U, L>>>::Bound
    where
        F: HasField<N>,
        F::Mut<'a>: BindFieldMut<'a, 'a, MessageCommon<F::Bits, A, U, L>>,
    {
        self.fields.field_mut().bind_field_mut(&mut self.common)
    }
}

impl<F, A, L, U> Record<F, A, L, U>
where
    A: Allocator + Clone,
    L: MessageScan<A>,
    U: UnknownStore<A>,
    F: RecordFields<A, L, Unknown = U> + ConstructRecordFields<A, L>,
    F::Bits: Clone,
{
    /// Empty message using `alloc` for every slot.
    #[inline]
    pub fn new_in(alloc: A) -> Self {
        Self::build_in(alloc)
    }

    fn build_in(alloc: A) -> Self {
        Self {
            fields: F::new(alloc.clone()),
            common: MessageCommon::new_in(F::empty_bits(), alloc),
        }
    }
}

impl<F, A, U> Record<F, A, Eager, U>
where
    A: Allocator + Clone,
    U: UnknownStore<A>,
    F: RecordFields<A, Eager, Unknown = U> + EagerRecordFields<A> + ConstructRecordFields<A, Eager>,
    F::Bits: Clone,
{
    /// Encode `self` and decode that body as the lazy alias `Z`.
    #[doc(hidden)]
    pub fn encode_lazy<Z>(self) -> Z
    where
        Z: Message<Alloc = A>,
    {
        let alloc = self.common.alloc.clone();
        let bytes = Message::encode_to_vec(&self);
        let mut lazy = Z::new_in(alloc);
        Message::merge_from(&mut lazy, &mut bytes.as_slice())
            .expect("self-encoded message is a valid body");
        lazy
    }
}

impl<F, A, U> Record<F, A, Lazy<A>, U>
where
    A: Allocator + Clone,
    U: UnknownStore<A>,
    F: RecordFields<A, Lazy<A>, Unknown = U>,
    F::Bits: Clone,
{
    /// Replay ingested bytes into an eager message `Z`.
    #[doc(hidden)]
    pub fn decode_eager<Z>(self) -> Result<Z, DecodeError>
    where
        Z: Message<Alloc = A>,
    {
        self.common.lazy.scan.require_finished()?;
        let mut eager = Z::new_in(self.common.alloc.clone());
        self.common.lazy.scan.for_each_body(|chunk| {
            let mut buf = chunk;
            Message::merge_from(&mut eager, &mut buf)
        })?;
        Ok(eager)
    }
}

impl<F, A, L, U> Drop for Record<F, A, L, U>
where
    A: Allocator + Clone,
    L: MessageScan<A>,
    U: UnknownStore<A>,
    F: RecordFields<A, L, Unknown = U>,
{
    fn drop(&mut self) {
        let mut v = FieldDeallocVisitor::new(&self.common);
        let _ = self.fields.visit_mut(&mut v);
        self.common.deallocate();
    }
}

impl<F, A, L, U> CloneIn<A> for Record<F, A, L, U>
where
    A: Allocator + Clone,
    L: MessageScan<A>,
    U: UnknownStore<A>,
    F: RecordFields<A, L, Unknown = U> + CloneRecordFields<A, L> + ConstructRecordFields<A, L>,
    F::Bits: Clone,
{
    fn clone_in(&self, alloc: A) -> Self {
        let mut dst = Self::build_in(alloc.clone());
        self.fields
            .clone_fields(&mut dst.fields, &self.common, &dst.common, alloc.clone());
        let mut old = mem::replace(&mut dst.common, self.common.clone_in(alloc));
        old.deallocate();
        dst
    }
}

impl<F, A, L, U> Clone for Record<F, A, L, U>
where
    A: Allocator + Clone,
    L: MessageScan<A>,
    U: UnknownStore<A>,
    F: RecordFields<A, L, Unknown = U> + CloneRecordFields<A, L> + ConstructRecordFields<A, L>,
    F::Bits: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        CloneIn::clone_in(self, self.common.alloc.clone())
    }
}

impl<F, A, L, U> DefaultIn<A> for Record<F, A, L, U>
where
    A: Allocator + Clone,
    L: MessageScan<A>,
    U: UnknownStore<A>,
    F: RecordFields<A, L, Unknown = U> + ConstructRecordFields<A, L>,
    F::Bits: Clone,
{
    #[inline]
    fn default_in(alloc: A) -> Self {
        Self::build_in(alloc)
    }
}

impl<F, A, L, U> Default for Record<F, A, L, U>
where
    A: Allocator + Clone + Default,
    L: MessageScan<A>,
    U: UnknownStore<A>,
    F: RecordFields<A, L, Unknown = U> + ConstructRecordFields<A, L>,
    F::Bits: Clone,
{
    #[inline]
    fn default() -> Self {
        Self::build_in(A::default())
    }
}

impl<F, A, L, U> DeallocateIn<A> for Record<F, A, L, U>
where
    A: Allocator + Clone,
    L: MessageScan<A>,
    U: UnknownStore<A>,
    F: RecordFields<A, L, Unknown = U>,
{
    #[inline]
    unsafe fn deallocate_in(self, _alloc: &A) {
        drop(self);
    }
}

impl<F, A, L, U> DeallocateBound<A> for Record<F, A, L, U>
where
    A: Allocator + Clone,
    L: MessageScan<A>,
    U: UnknownStore<A>,
    F: RecordFields<A, L, Unknown = U>,
{
    #[inline]
    fn deallocate_bound<Cx: MessageBindingMut<A>>(self, common: &Cx) {
        unsafe { DeallocateIn::deallocate_in(self, common.alloc()) }
    }
}

impl<F, A, L, U> CloneBound<A> for Record<F, A, L, U>
where
    A: Allocator + Clone,
    L: MessageScan<A>,
    U: UnknownStore<A>,
    F: RecordFields<A, L, Unknown = U> + CloneRecordFields<A, L> + ConstructRecordFields<A, L>,
    F::Bits: Clone,
{
    #[inline]
    fn clone_bound<Cx: MessageBindingMut<A>>(&self, _common: &Cx, alloc: A) -> Self {
        CloneIn::clone_in(self, alloc)
    }
}

impl<F, A, U> PartialEq for Record<F, A, Eager, U>
where
    A: Allocator + Clone,
    U: UnknownStore<A>,
    F: RecordFields<A, Eager, Unknown = U> + EagerRecordFields<A>,
    F::Bits: Clone,
{
    fn eq(&self, other: &Self) -> bool {
        matches!(
            self.fields.visit_pairs(
                &other.fields,
                &mut FieldEqVisitor::new(&self.common, &other.common),
            ),
            ControlFlow::Continue(())
        ) && self.common.unknown_fields_eq(&other.common)
    }
}

impl<F, A, U> PartialEq for Record<F, A, Lazy<A>, U>
where
    A: Allocator + Clone,
    U: UnknownStore<A>,
    F: RecordFields<A, Lazy<A>, Unknown = U>
        + LazyRecordApply<A>
        + CloneRecordFields<A, Lazy<A>>
        + ConstructRecordFields<A, Lazy<A>>,
    F::Bits: Clone,
{
    fn eq(&self, other: &Self) -> bool {
        LazyMessage::eq_with_eager(self, other, |msg| {
            msg.clone().decode_eager::<F::EagerMessage>()
        })
    }
}

impl<F, A, U> Debug for Record<F, A, Eager, U>
where
    A: Allocator + Clone,
    U: UnknownStore<A>,
    F: RecordFields<A, Eager, Unknown = U> + EagerRecordFields<A>,
    F::Bits: Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut v = DebugStructVisitor::new(f.debug_struct(F::EAGER_NAME), &self.common);
        let _ = self.fields.visit_fields(&mut v);
        v.finish()
    }
}

impl<F, A, U> Debug for Record<F, A, Lazy<A>, U>
where
    A: Allocator + Clone,
    U: UnknownStore<A>,
    F: RecordFields<A, Lazy<A>, Unknown = U>
        + LazyRecordApply<A>
        + CloneRecordFields<A, Lazy<A>>
        + ConstructRecordFields<A, Lazy<A>>,
    F::Bits: Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if LazyMessage::ensure_scanned(self).is_err() {
            return f.debug_struct(F::LAZY_NAME).finish_non_exhaustive();
        }
        match self.clone().decode_eager::<F::EagerMessage>() {
            Ok(eager) => Debug::fmt(&eager, f),
            Err(_) => f.debug_struct(F::LAZY_NAME).finish_non_exhaustive(),
        }
    }
}

impl<F, A, U> MessageEncode for Record<F, A, Eager, U>
where
    A: Allocator + Clone,
    U: UnknownStore<A>,
    F: RecordFields<A, Eager, Unknown = U> + EagerRecordFields<A>,
    F::Bits: Clone,
{
    fn encoded_len(&self, ctx: &mut EncodeCtx) -> usize {
        let mut v = EncodedLenVisitor::new(&self.common, ctx);
        let _ = self.fields.visit_fields(&mut v);
        v.len + UnknownStore::as_bytes(&*self.common.unknown_fields).len()
    }

    fn encode_raw<B: BufMut>(&self, ctx: &mut EncodeCtx, buf: &mut B) {
        let _ = self
            .fields
            .visit_fields(&mut EncodeRawVisitor::new(&self.common, ctx, buf));
        let unknown: &[u8] = UnknownStore::as_bytes(&*self.common.unknown_fields);
        buf.put_slice(unknown);
    }
}

impl<F, A, U> MessageEncode for Record<F, A, Lazy<A>, U>
where
    A: Allocator + Clone,
    U: UnknownStore<A>,
    F: RecordFields<A, Lazy<A>, Unknown = U>,
    F::Bits: Clone,
{
    fn encoded_len(&self, _ctx: &mut EncodeCtx) -> usize {
        self.common.lazy.scan.body_len()
    }

    fn encode_raw<B: BufMut>(&self, _ctx: &mut EncodeCtx, buf: &mut B) {
        self.common
            .lazy
            .scan
            .write_bodies(buf)
            .expect("lazy wire span out of range");
    }
}

impl<F, A, U> MessageMerge for Record<F, A, Eager, U>
where
    A: Allocator + Clone,
    U: UnknownStore<A>,
    F: RecordFields<A, Eager, Unknown = U> + EagerRecordFields<A>,
    F::Bits: Clone,
{
    fn merge_from_with_depth<B: DecodeBuf>(
        &mut self,
        buf: &mut B,
        depth: usize,
    ) -> Result<(), DecodeError> {
        if depth >= RECURSION_LIMIT {
            return Err(DecodeError::RecursionLimitExceeded);
        }
        while buf.has_remaining() {
            let (field_number, wire_type) = decode_tag(buf)?;
            if !self.fields.merge_known(
                &mut self.common,
                field_number.as_u32(),
                wire_type,
                buf,
                depth,
            )? {
                skip_field_and_save_in(
                    field_number,
                    wire_type,
                    buf,
                    &mut *self.common.unknown_fields,
                    self.common.alloc.clone(),
                )?;
            }
        }
        Ok(())
    }
}

impl<F, A, U> MessageMerge for Record<F, A, Lazy<A>, U>
where
    A: Allocator + Clone,
    U: UnknownStore<A>,
    F: RecordFields<A, Lazy<A>, Unknown = U> + LazyRecordApply<A>,
    F::Bits: Clone,
{
    fn merge_from_with_depth<B: DecodeBuf>(
        &mut self,
        buf: &mut B,
        depth: usize,
    ) -> Result<(), DecodeError> {
        if depth >= RECURSION_LIMIT {
            return Err(DecodeError::RecursionLimitExceeded);
        }
        LazyMessage::ingest_buf(self, buf)
    }
}

impl<F, A, U> LazyMessage<A> for Record<F, A, Lazy<A>, U>
where
    A: Allocator + Clone,
    U: UnknownStore<A>,
    F: RecordFields<A, Lazy<A>, Unknown = U> + LazyRecordApply<A>,
    F::Bits: Clone,
{
    fn scan(&self) -> &LazyScan<A> {
        &self.common.lazy.scan
    }

    fn scan_mut(&mut self) -> &mut LazyScan<A> {
        &mut self.common.lazy.scan
    }

    fn apply_record(&mut self, rec: &ScannedRecord, origin: usize) -> Result<(), DecodeError> {
        self.fields.apply_record(&mut self.common, rec, origin)
    }
}

fn message_new_in<F, A, L, U>(alloc: A) -> Record<F, A, L, U>
where
    A: Allocator + Clone,
    L: MessageScan<A>,
    U: UnknownStore<A>,
    F: RecordFields<A, L, Unknown = U> + ConstructRecordFields<A, L>,
    F::Bits: Clone,
{
    Record::build_in(alloc)
}

fn message_encode<F, A, L, U, B>(message: &Record<F, A, L, U>, buf: &mut B)
where
    A: Allocator + Clone,
    L: MessageScan<A>,
    U: UnknownStore<A>,
    F: RecordFields<A, L, Unknown = U>,
    F::Bits: Clone,
    B: BufMut,
    Record<F, A, L, U>: MessageEncode,
{
    encode_message(message, buf);
}

fn message_encode_vec<F, A, L, U>(message: &Record<F, A, L, U>) -> Vec<u8>
where
    A: Allocator + Clone,
    L: MessageScan<A>,
    U: UnknownStore<A>,
    F: RecordFields<A, L, Unknown = U>,
    F::Bits: Clone,
    Record<F, A, L, U>: MessageEncode,
{
    encode_message_to_vec(message)
}

impl<F, A, U> Message for Record<F, A, Eager, U>
where
    A: Allocator + Clone,
    U: UnknownStore<A>,
    F: RecordFields<A, Eager, Unknown = U> + EagerRecordFields<A> + ConstructRecordFields<A, Eager>,
    F::Bits: Clone,
{
    type Alloc = A;

    fn new_in(alloc: A) -> Self
    where
        A: Clone,
    {
        message_new_in(alloc)
    }

    fn encode<B: BufMut>(&self, buf: &mut B) {
        message_encode(self, buf);
    }

    fn encode_to_vec(&self) -> Vec<u8> {
        message_encode_vec(self)
    }

    fn merge_from<B: Buf>(&mut self, buf: &mut B) -> Result<(), DecodeError>
    where
        A: Clone,
    {
        merge_message(self, buf)
    }

    fn unknown_fields(&self) -> impl Iterator<Item = ::puroro::UnknownField<'_>> + '_ {
        self.common.iter_unknown_fields()
    }

    fn validate(&self) -> Result<(), DecodeError> {
        self.fields.validate(&self.common)
    }
}

impl<F, A, U> Message for Record<F, A, Lazy<A>, U>
where
    A: Allocator + Clone,
    U: UnknownStore<A>,
    F: RecordFields<A, Lazy<A>, Unknown = U>
        + LazyRecordApply<A>
        + ConstructRecordFields<A, Lazy<A>>,
    F::Bits: Clone,
{
    type Alloc = A;

    fn new_in(alloc: A) -> Self
    where
        A: Clone,
    {
        message_new_in(alloc)
    }

    fn encode<B: BufMut>(&self, buf: &mut B) {
        message_encode(self, buf);
    }

    fn encode_to_vec(&self) -> Vec<u8> {
        message_encode_vec(self)
    }

    fn merge_from<B: Buf>(&mut self, buf: &mut B) -> Result<(), DecodeError>
    where
        A: Clone,
    {
        LazyMessage::ingest_buf(self, buf)
    }

    fn unknown_fields(&self) -> impl Iterator<Item = ::puroro::UnknownField<'_>> + '_ {
        iter::empty()
    }

    fn validate(&self) -> Result<(), DecodeError> {
        // Required fields are the only reason to scan here. An unfinished or
        // unapplied lazy message with no `LEGACY_REQUIRED` field stays `Ok(())`,
        // matching `Message::validate`'s default.
        if F::CHECKS_REQUIRED {
            LazyMessage::ensure_scanned(self)?;
        }
        self.fields.validate(&self.common)
    }
}

/// Forward shell traits from a generated newtype onto its [`Record`] field.
///
/// The newtype is local to the generated crate, so field getters can be
/// inherent methods. [`Record`] is not, so those getters cannot be inherent
/// impls on it. The newtype is:
///
/// ```ignore
/// pub struct Task<A: Allocator + Clone = Global> {
///     record: Record<TaskFields<A, Eager>, A, Eager, UnknownFields<A>>,
/// }
/// ```
///
/// `eager` / `lazy` pick the [`Record`] impls (`LazyMessage` is lazy-only).
#[macro_export]
macro_rules! forward_record {
    (eager $name:ident) => {
        $crate::forward_record!(@shared $name);
    };
    (lazy $name:ident) => {
        $crate::forward_record!(@shared $name);
        $crate::forward_record!(@lazy $name);
    };
    (@shared $name:ident) => {
        impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> $name<A> {
            /// Empty message using `alloc` for every slot.
            #[inline]
            pub fn new_in(alloc: A) -> Self {
                Self {
                    record: $crate::Record::new_in(alloc),
                }
            }
        }

        impl $name<::allocator_api2::alloc::Global> {
            /// Empty message using [`allocator_api2::alloc::Global`].
            #[inline]
            pub fn new() -> Self {
                Self {
                    record: $crate::Record::new_in(::allocator_api2::alloc::Global),
                }
            }
        }

        impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> ::core::clone::Clone
            for $name<A>
        {
            #[inline]
            fn clone(&self) -> Self {
                Self {
                    record: ::core::clone::Clone::clone(&self.record),
                }
            }
        }

        impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> ::core::cmp::PartialEq
            for $name<A>
        {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.record == other.record
            }
        }

        impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> ::core::fmt::Debug
            for $name<A>
        {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Debug::fmt(&self.record, f)
            }
        }

        impl<
            A: ::allocator_api2::alloc::Allocator
                + ::core::clone::Clone
                + ::core::default::Default,
        > ::core::default::Default for $name<A>
        {
            #[inline]
            fn default() -> Self {
                Self {
                    record: ::core::default::Default::default(),
                }
            }
        }

        impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> $crate::CloneIn<A>
            for $name<A>
        {
            #[inline]
            fn clone_in(&self, alloc: A) -> Self {
                Self {
                    record: $crate::CloneIn::clone_in(&self.record, alloc),
                }
            }
        }

        impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> $crate::DefaultIn<A>
            for $name<A>
        {
            #[inline]
            fn default_in(alloc: A) -> Self {
                Self {
                    record: $crate::DefaultIn::default_in(alloc),
                }
            }
        }

        impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> $crate::DeallocateIn<A>
            for $name<A>
        {
            #[inline]
            unsafe fn deallocate_in(self, alloc: &A) {
                unsafe { $crate::DeallocateIn::deallocate_in(self.record, alloc) }
            }
        }

        impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> $crate::DeallocateBound<A>
            for $name<A>
        {
            #[inline]
            fn deallocate_bound<Cx: $crate::MessageBindingMut<A>>(self, common: &Cx) {
                unsafe { $crate::DeallocateIn::deallocate_in(self, common.alloc()) }
            }
        }

        impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> $crate::CloneBound<A>
            for $name<A>
        {
            #[inline]
            fn clone_bound<Cx: $crate::MessageBindingMut<A>>(
                &self,
                _common: &Cx,
                alloc: A,
            ) -> Self {
                $crate::CloneIn::clone_in(self, alloc)
            }
        }

        impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> $crate::MessageEncode
            for $name<A>
        {
            #[inline]
            fn encoded_len(&self, ctx: &mut $crate::EncodeCtx) -> usize {
                $crate::MessageEncode::encoded_len(&self.record, ctx)
            }

            #[inline]
            fn encode_raw<B: ::bytes::BufMut>(&self, ctx: &mut $crate::EncodeCtx, buf: &mut B) {
                $crate::MessageEncode::encode_raw(&self.record, ctx, buf)
            }
        }

        impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> $crate::MessageMerge
            for $name<A>
        {
            #[inline]
            fn merge_from_with_depth<B: ::puroro::DecodeBuf>(
                &mut self,
                buf: &mut B,
                depth: usize,
            ) -> ::core::result::Result<(), ::puroro::DecodeError> {
                $crate::MessageMerge::merge_from_with_depth(&mut self.record, buf, depth)
            }
        }

        impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> ::puroro::Message
            for $name<A>
        {
            type Alloc = A;

            #[inline]
            fn new_in(alloc: A) -> Self
            where
                A: ::core::clone::Clone,
            {
                Self {
                    record: $crate::Record::new_in(alloc),
                }
            }

            #[inline]
            fn encode<B: ::bytes::BufMut>(&self, buf: &mut B) {
                ::puroro::Message::encode(&self.record, buf)
            }

            #[inline]
            fn encode_to_vec(&self) -> ::std::vec::Vec<u8> {
                ::puroro::Message::encode_to_vec(&self.record)
            }

            #[inline]
            fn merge_from<B: ::bytes::Buf>(
                &mut self,
                buf: &mut B,
            ) -> ::core::result::Result<(), ::puroro::DecodeError>
            where
                A: ::core::clone::Clone,
            {
                ::puroro::Message::merge_from(&mut self.record, buf)
            }

            #[inline]
            fn unknown_fields(
                &self,
            ) -> impl ::core::iter::Iterator<Item = ::puroro::UnknownField<'_>> + '_ {
                ::puroro::Message::unknown_fields(&self.record)
            }

            #[inline]
            fn validate(&self) -> ::core::result::Result<(), ::puroro::DecodeError> {
                ::puroro::Message::validate(&self.record)
            }
        }
    };
    (@lazy $name:ident) => {
        impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> $crate::decode::LazyMessage<A>
            for $name<A>
        {
            #[inline]
            fn scan(&self) -> &$crate::decode::LazyScan<A> {
                $crate::decode::LazyMessage::scan(&self.record)
            }

            #[inline]
            fn scan_mut(&mut self) -> &mut $crate::decode::LazyScan<A> {
                $crate::decode::LazyMessage::scan_mut(&mut self.record)
            }

            #[inline]
            fn apply_record(
                &mut self,
                rec: &$crate::decode::ScannedRecord,
                origin: usize,
            ) -> ::core::result::Result<(), ::puroro::DecodeError> {
                $crate::decode::LazyMessage::apply_record(&mut self.record, rec, origin)
            }
        }
    };
}
