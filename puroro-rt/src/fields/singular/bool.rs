//! Bit-packed singular `bool` field — value lives in [`MessageCommon`] presence bitvec.
//!
//! Unlike [`SingularField`](super::field::SingularField), this wrapper is a ZST: the
//! boolean payload is stored at `VALUE_BIT` inside `_common.presence`. Explicit /
//! legacy-required presence still uses [`FieldPresence`]'s presence bit; Implicit
//! omit-on-encode treats a clear value bit as absent; Oneof presence stays on the
//! enclosing [`OneofSlot`](crate::fields::OneofSlot).

use ::core::marker::PhantomData;
use ::core::ops::DerefMut;

use ::allocator_api2::alloc::Allocator;
use ::bytes::{Buf, BufMut};
use ::puroro::DecodeError;
use ::puroro::WireType;
use ::puroro::{HasDefault, Optional};

use crate::defaults::ProtoDefault;
use crate::encode;
use crate::fields::shared::{
    Bindable, BindableMut, FieldDeallocate, MessageCommon, PresenceBits,
    field_presence::{
        Explicit, FieldPresence, Implicit, LegacyRequired, Oneof, RequiredFieldPresence,
    },
    slot_init::{SlotInitMut, SlotInitView},
};
use crate::fields::wire::varint::{self, ProtoBool, VarintProtoType};

/// Singular `bool` field whose value is packed into the message bitvec at `VALUE_BIT`.
///
/// `P` selects presence policy ([`Implicit`] / [`Explicit`](crate::fields::Explicit) /
/// [`LegacyRequired`] / [`Oneof`]). The struct itself holds no payload.
///
/// `Clone` / `Copy` / `Debug` / `Default` are hand-written rather than `#[derive]`'d:
/// derive would require `P` / `D` to satisfy those traits even though they only appear
/// in [`PhantomData`], and custom default markers (e.g. `WebhookIdDefault`) need not
/// implement them. A ZST catalog wrapper should stay unconditionally `Copy`.
pub struct BoolField<P: FieldPresence, const VALUE_BIT: usize, const FIELD: u32, D = ProtoDefault> {
    _marker: PhantomData<(P, D)>,
}

impl<P: FieldPresence, const VALUE_BIT: usize, const FIELD: u32, D> Clone
    for BoolField<P, VALUE_BIT, FIELD, D>
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<P: FieldPresence, const VALUE_BIT: usize, const FIELD: u32, D> Copy
    for BoolField<P, VALUE_BIT, FIELD, D>
{
}

impl<P: FieldPresence, const VALUE_BIT: usize, const FIELD: u32, D> ::core::fmt::Debug
    for BoolField<P, VALUE_BIT, FIELD, D>
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BoolField").finish_non_exhaustive()
    }
}

impl<P: FieldPresence, const VALUE_BIT: usize, const FIELD: u32, D> Default
    for BoolField<P, VALUE_BIT, FIELD, D>
{
    fn default() -> Self {
        Self::new_in(())
    }
}

impl<P: FieldPresence, const VALUE_BIT: usize, const FIELD: u32, D>
    BoolField<P, VALUE_BIT, FIELD, D>
{
    /// Creates an empty field wrapper (value bits start cleared with the message).
    #[inline]
    pub fn new_in<A>(_alloc: A) -> Self {
        Self {
            _marker: PhantomData,
        }
    }

    #[inline]
    fn read_value<Pb: PresenceBits, A: Allocator>(common: &MessageCommon<Pb, A>) -> bool {
        common.is_bit_set(VALUE_BIT)
    }

    #[inline]
    fn write_value<Pb: PresenceBits, A: Allocator>(common: &mut MessageCommon<Pb, A>, value: bool) {
        common.set_bit(VALUE_BIT, value);
    }

    /// `true` when the stored value is protobuf empty (`false`).
    #[inline]
    fn value_is_empty<Pb: PresenceBits, A: Allocator>(common: &MessageCommon<Pb, A>) -> bool {
        !Self::read_value(common)
    }

    pub fn encoded_len<Pb, A>(&self, common: &MessageCommon<Pb, A>) -> usize
    where
        Pb: PresenceBits,
        A: Allocator,
    {
        if P::should_emit(common, || Self::value_is_empty(common)) {
            encode::encoded_len_varint_field(FIELD, ProtoBool::encode_wire(Self::read_value(common)))
        } else {
            0
        }
    }

    pub fn encode_raw<Pb, A, B: BufMut>(&self, common: &MessageCommon<Pb, A>, buf: &mut B)
    where
        Pb: PresenceBits,
        A: Allocator,
    {
        if P::should_emit(common, || Self::value_is_empty(common)) {
            encode::encode_varint_field(
                FIELD,
                ProtoBool::encode_wire(Self::read_value(common)),
                buf,
            );
        }
    }

    /// No-op — bool values live in the bitvec and need no allocator release.
    ///
    /// Present so message `Drop` / oneof teardown can call [`FieldDeallocate`]
    /// uniformly on every field.
    #[inline]
    pub fn deallocate<Pb, A>(&mut self, _common: &MessageCommon<Pb, A>)
    where
        Pb: PresenceBits,
        A: Allocator,
    {
    }
}

impl<P: FieldPresence, const VALUE_BIT: usize, const FIELD: u32, D, Pb: PresenceBits, A: Allocator>
    FieldDeallocate<Pb, A> for BoolField<P, VALUE_BIT, FIELD, D>
{
    #[inline]
    fn deallocate(&mut self, common: &MessageCommon<Pb, A>) {
        BoolField::deallocate(self, common);
    }
}

impl<P: FieldPresence, const VALUE_BIT: usize, const FIELD: u32, D>
    BoolField<P, VALUE_BIT, FIELD, D>
{
    /// Validates LEGACY_REQUIRED presence when `P: RequiredFieldPresence`.
    pub fn validate_required<Pb, A>(&self, common: &MessageCommon<Pb, A>) -> Result<(), DecodeError>
    where
        P: RequiredFieldPresence,
        Pb: PresenceBits,
        A: Allocator,
    {
        P::validate_present(common, FIELD, || Self::value_is_empty(common))
    }
}

// ---------------------------------------------------------------------------
// Read view
// ---------------------------------------------------------------------------

/// Short-lived shared binding of a [`BoolField`] to its message common state.
pub struct BoolFieldRef<
    'a,
    P: FieldPresence,
    const VALUE_BIT: usize,
    const FIELD: u32,
    D,
    Pb: PresenceBits,
    A: Allocator,
> {
    #[allow(dead_code)] // retained for API symmetry with other field views
    field: &'a BoolField<P, VALUE_BIT, FIELD, D>,
    common: &'a MessageCommon<Pb, A>,
}

impl<
    'a,
    P: FieldPresence,
    const VALUE_BIT: usize,
    const FIELD: u32,
    D,
    Pb: PresenceBits,
    A: Allocator,
> BoolFieldRef<'a, P, VALUE_BIT, FIELD, D, Pb, A>
{
    #[inline]
    fn new(
        field: &'a BoolField<P, VALUE_BIT, FIELD, D>,
        common: &'a MessageCommon<Pb, A>,
    ) -> Self {
        Self { field, common }
    }
}

impl<
    'a,
    P: FieldPresence,
    const VALUE_BIT: usize,
    const FIELD: u32,
    D: HasDefault<bool>,
    Pb: PresenceBits,
    A: Allocator,
> BoolFieldRef<'a, P, VALUE_BIT, FIELD, D, Pb, A>
{
    pub fn optional(self) -> Optional<bool, D> {
        let v = if P::is_set(self.common, || {
            BoolField::<P, VALUE_BIT, FIELD, D>::value_is_empty(self.common)
        }) {
            Some(BoolField::<P, VALUE_BIT, FIELD, D>::read_value(self.common))
        } else {
            None
        };
        Optional::new(v)
    }
}

impl<'a, const VALUE_BIT: usize, const FIELD: u32, D, Pb: PresenceBits, A: Allocator>
    BoolFieldRef<'a, Implicit, VALUE_BIT, FIELD, D, Pb, A>
{
    #[inline]
    pub fn value(self) -> bool {
        BoolField::<Implicit, VALUE_BIT, FIELD, D>::read_value(self.common)
    }
}

impl<'a, const VALUE_BIT: usize, const FIELD: u32, D, Pb: PresenceBits, A: Allocator>
    BoolFieldRef<'a, Oneof, VALUE_BIT, FIELD, D, Pb, A>
{
    #[inline]
    pub fn value(self) -> bool {
        BoolField::<Oneof, VALUE_BIT, FIELD, D>::read_value(self.common)
    }
}

impl<
    'a,
    P: FieldPresence,
    const VALUE_BIT: usize,
    const FIELD: u32,
    D,
    A: Allocator + Clone,
    Pb: PresenceBits,
> Bindable<&'a MessageCommon<Pb, A>> for &'a BoolField<P, VALUE_BIT, FIELD, D>
{
    type Bound = BoolFieldRef<'a, P, VALUE_BIT, FIELD, D, Pb, A>;

    #[inline]
    fn bind(self, common: &'a MessageCommon<Pb, A>) -> Self::Bound {
        BoolFieldRef::new(self, common)
    }
}

impl<
    'f,
    'c,
    P: FieldPresence,
    const VALUE_BIT: usize,
    const FIELD: u32,
    D,
    A: Allocator + Clone,
    Pb: PresenceBits,
> BindableMut<&'c mut MessageCommon<Pb, A>> for &'f mut BoolField<P, VALUE_BIT, FIELD, D>
{
    type BoundMut = BoolFieldMut<'f, 'c, P, VALUE_BIT, FIELD, D, Pb, A>;

    #[inline]
    fn bind_mut(self, common: &'c mut MessageCommon<Pb, A>) -> Self::BoundMut {
        BoolFieldMut::new(self, common)
    }
}

// ---------------------------------------------------------------------------
// Mutation view
// ---------------------------------------------------------------------------

/// Short-lived binding of a [`BoolField`] to its message common state.
pub struct BoolFieldMut<
    'f,
    'c,
    P: FieldPresence,
    const VALUE_BIT: usize,
    const FIELD: u32,
    D,
    Pb: PresenceBits,
    A: Allocator,
> {
    #[allow(dead_code)] // retained for API symmetry with other field views
    field: &'f mut BoolField<P, VALUE_BIT, FIELD, D>,
    common: &'c mut MessageCommon<Pb, A>,
}

impl<
    'f,
    'c,
    P: FieldPresence,
    const VALUE_BIT: usize,
    const FIELD: u32,
    D,
    Pb: PresenceBits,
    A: Allocator,
> BoolFieldMut<'f, 'c, P, VALUE_BIT, FIELD, D, Pb, A>
{
    #[inline]
    fn new(
        field: &'f mut BoolField<P, VALUE_BIT, FIELD, D>,
        common: &'c mut MessageCommon<Pb, A>,
    ) -> Self {
        Self { field, common }
    }

    /// Ensures presence (when applicable) and returns a mutable handle to the value bit.
    ///
    /// Typically bitvec's `BitRef<'_, Mut, …>` from [`MessageCommon::bit_mut`].
    #[inline]
    pub fn value_mut(self) -> impl DerefMut<Target = bool> + 'c {
        if !P::slot_init_view(self.common).is_initialized() {
            BoolField::<P, VALUE_BIT, FIELD, D>::write_value(self.common, false);
            P::slot_init_mut(self.common).set_initialized(true);
        }
        self.common.bit_mut(VALUE_BIT)
    }

    #[inline]
    pub fn set(self, v: bool) {
        BoolField::<P, VALUE_BIT, FIELD, D>::write_value(self.common, v);
        P::slot_init_mut(self.common).set_initialized(true);
    }

    pub fn merge<B: Buf>(self, wire_type: WireType, buf: &mut B) -> Result<(), DecodeError> {
        if wire_type != varint::WIRE_TYPE {
            return Err(DecodeError::InvalidTag);
        }
        let raw = crate::decode::decode_varint(buf)?;
        let v = ProtoBool::decode_wire(raw)?;
        BoolField::<P, VALUE_BIT, FIELD, D>::write_value(self.common, v);
        P::slot_init_mut(self.common).set_initialized(true);
        Ok(())
    }

    /// Clears the value bit and explicit presence when applicable.
    pub fn clear(self) {
        BoolField::<P, VALUE_BIT, FIELD, D>::write_value(self.common, false);
        P::slot_init_mut(self.common).set_initialized(false);
    }
}

// ---------------------------------------------------------------------------
// Type aliases
// ---------------------------------------------------------------------------

pub type ImplicitBoolField<const VALUE_BIT: usize, const FIELD: u32> =
    BoolField<Implicit, VALUE_BIT, FIELD>;
pub type OneofBoolField<const VALUE_BIT: usize, const FIELD: u32, D = ProtoDefault> =
    BoolField<Oneof, VALUE_BIT, FIELD, D>;
pub type ExplicitBoolField<
    const PRESENCE_BIT: usize,
    const VALUE_BIT: usize,
    const FIELD: u32,
    D = ProtoDefault,
> = BoolField<Explicit<PRESENCE_BIT>, VALUE_BIT, FIELD, D>;
pub type LegacyRequiredBoolField<
    const PRESENCE_BIT: usize,
    const VALUE_BIT: usize,
    const FIELD: u32,
    D = ProtoDefault,
> = BoolField<LegacyRequired<PRESENCE_BIT>, VALUE_BIT, FIELD, D>;
