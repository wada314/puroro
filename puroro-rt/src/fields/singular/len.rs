//! Singular LEN field wrapper — generic over payload type and presence policy.

use ::core::marker::PhantomData;
use ::core::mem::ManuallyDrop;

use ::allocator_api2::alloc::Allocator;
use ::bytes::{Buf, BufMut};

use crate::defaults::ProtoDefault;
use crate::encode;
use ::puroro::DecodeError;
use ::puroro::WireType;
use ::puroro::{HasDefault, Optional};

use crate::fields::shared::{
    BindableMut, MessageCommon, PresenceBits,
    field_presence::{FieldPresence, LegacyRequired, RequiredFieldPresence},
    slot_init::SlotInitMut,
};
use crate::fields::wire::len::{self, LenProtoType};

/// Singular LEN field — parametrised by [`LenProtoType`] `T`, presence policy `P`,
/// proto field number `FIELD`, message allocator `A`, and compile-time default marker `D`.
pub struct SingularLenField<
    T: LenProtoType,
    P: FieldPresence,
    const FIELD: u32,
    A: Allocator,
    D = ProtoDefault,
> {
    value: ManuallyDrop<T::Storage>,
    _marker: PhantomData<(P, A, D)>,
}

impl<T: LenProtoType, P: FieldPresence, const FIELD: u32, A: Allocator, D>
    SingularLenField<T, P, FIELD, A, D>
{
    pub fn new_in(alloc: A) -> Self {
        Self {
            value: ManuallyDrop::new(T::new_in(alloc)),
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn value(&self) -> T::Ref<'_> {
        T::borrow(&self.value)
    }

    pub fn value_mut(&mut self, alloc: A) -> T::Mut<'_, A> {
        T::with_alloc(&mut self.value, alloc)
    }

    pub fn encoded_len<Pb>(&self, common: &MessageCommon<Pb, A>) -> usize
    where
        Pb: PresenceBits,
    {
        if P::should_emit(common, || T::is_empty(&self.value)) {
            encode::encoded_len_len_field(FIELD, T::as_bytes(&self.value).len())
        } else {
            0
        }
    }

    pub fn encode_raw<Pb, B: BufMut>(&self, common: &MessageCommon<Pb, A>, buf: &mut B)
    where
        Pb: PresenceBits,
    {
        if P::should_emit(common, || T::is_empty(&self.value)) {
            encode::encode_len_field(FIELD, T::as_bytes(&self.value), buf);
        }
    }

    pub fn deallocate(&mut self, alloc: A) {
        let old = unsafe { ManuallyDrop::take(&mut self.value) };
        unsafe { T::deallocate(old, alloc) };
    }

    #[inline]
    pub fn has<Pb>(&self, common: &MessageCommon<Pb, A>) -> bool
    where
        Pb: PresenceBits,
    {
        P::is_set(common, || T::is_empty(&self.value))
    }
}

impl<T: LenProtoType, P: FieldPresence, const FIELD: u32, A: Allocator, D>
    SingularLenField<T, P, FIELD, A, D>
where
    for<'a> T::Ref<'a>: Copy,
    D: for<'a> HasDefault<T::Ref<'a>>,
{
    pub fn optional<'a, Pb>(&'a self, common: &MessageCommon<Pb, A>) -> Optional<T::Ref<'a>, D>
    where
        Pb: PresenceBits,
    {
        let v = if P::is_set(common, || T::is_empty(&self.value)) {
            Some(T::borrow(&self.value))
        } else {
            None
        };
        Optional::new(v)
    }
}

impl<T: LenProtoType, const BIT: usize, const FIELD: u32, A: Allocator, D>
    SingularLenField<T, LegacyRequired<BIT>, FIELD, A, D>
{
    pub fn validate_required<Pb>(&self, common: &MessageCommon<Pb, A>) -> Result<(), DecodeError>
    where
        Pb: PresenceBits,
    {
        LegacyRequired::<BIT>::validate_present(common, FIELD, || T::is_empty(&self.value))
    }
}

// ---------------------------------------------------------------------------
// Mutation view
// ---------------------------------------------------------------------------

pub struct SingularLenFieldMut<
    'f,
    'c,
    T: LenProtoType,
    P: FieldPresence,
    const FIELD: u32,
    A: Allocator,
    D,
    Pb: PresenceBits,
> {
    field: &'f mut SingularLenField<T, P, FIELD, A, D>,
    common: &'c mut MessageCommon<Pb, A>,
}

impl<'f, 'c, T: LenProtoType, P: FieldPresence, const FIELD: u32, A: Allocator, D, Pb: PresenceBits>
    SingularLenFieldMut<'f, 'c, T, P, FIELD, A, D, Pb>
{
    #[inline]
    fn new(
        field: &'f mut SingularLenField<T, P, FIELD, A, D>,
        common: &'c mut MessageCommon<Pb, A>,
    ) -> Self {
        Self { field, common }
    }

    #[inline]
    pub fn value_mut(self) -> T::Mut<'f, A>
    where
        A: Clone,
    {
        {
            let mut init = P::slot_init_mut(self.common);
            init.set_initialized(true);
        }
        let alloc = self.common.alloc.clone();
        self.field.value_mut(alloc)
    }

    pub fn merge<B: Buf>(self, wire_type: WireType, buf: &mut B) -> Result<(), DecodeError>
    where
        A: Clone,
    {
        if wire_type != len::WIRE_TYPE {
            return Err(DecodeError::InvalidTag);
        }
        let new = T::decode(buf, self.common.alloc.clone())?;
        let old = unsafe { ManuallyDrop::take(&mut self.field.value) };
        unsafe { T::deallocate(old, self.common.alloc.clone()) };
        self.field.value = ManuallyDrop::new(new);
        {
            let mut init = P::slot_init_mut(self.common);
            init.set_initialized(true);
        }
        Ok(())
    }

    /// Resets payload to empty / type-zero and clears explicit presence when applicable.
    ///
    /// For [`Implicit`](crate::fields::shared::field_presence::Implicit) fields this omits the field on
    /// the wire.
    pub fn clear(self)
    where
        A: Clone,
    {
        {
            let mut init = P::slot_init_mut(self.common);
            init.set_initialized(false);
        }
        let old = unsafe { ManuallyDrop::take(&mut self.field.value) };
        unsafe { T::deallocate(old, self.common.alloc.clone()) };
        self.field.value = ManuallyDrop::new(T::new_in(self.common.alloc.clone()));
    }
}

impl<T: LenProtoType, P: FieldPresence, const FIELD: u32, A: Allocator + Clone, D, Pb: PresenceBits>
    BindableMut<MessageCommon<Pb, A>> for SingularLenField<T, P, FIELD, A, D>
{
    type BoundMut<'f, 'c> = SingularLenFieldMut<'f, 'c, T, P, FIELD, A, D, Pb>
    where
        Self: 'f,
        MessageCommon<Pb, A>: 'c;

    fn bind_mut<'f, 'c>(
        &'f mut self,
        common: &'c mut MessageCommon<Pb, A>,
    ) -> SingularLenFieldMut<'f, 'c, T, P, FIELD, A, D, Pb> {
        SingularLenFieldMut::new(self, common)
    }
}

// ---------------------------------------------------------------------------
// Type aliases
// ---------------------------------------------------------------------------

pub type SingularLen<T, P, const FIELD: u32, A, D = ProtoDefault> =
    SingularLenField<T, P, FIELD, A, D>;

pub type ImplicitLenField<T, const FIELD: u32, A> =
    SingularLenField<T, crate::fields::shared::field_presence::Implicit, FIELD, A>;
pub type OneofLenField<T, const FIELD: u32, A> =
    SingularLenField<T, crate::fields::shared::field_presence::Oneof, FIELD, A>;
pub type ExplicitLenField<T, const BIT: usize, const FIELD: u32, A, D = ProtoDefault> =
    SingularLenField<T, crate::fields::shared::field_presence::Explicit<BIT>, FIELD, A, D>;
pub type LegacyRequiredLenField<T, const BIT: usize, const FIELD: u32, A, D = ProtoDefault> =
    SingularLenField<T, crate::fields::shared::field_presence::LegacyRequired<BIT>, FIELD, A, D>;

pub type ImplicitString<const FIELD: u32, A> = ImplicitLenField<len::ProtoString, FIELD, A>;
pub type ExplicitString<const BIT: usize, const FIELD: u32, A, D = ProtoDefault> =
    ExplicitLenField<len::ProtoString, BIT, FIELD, A, D>;
pub type ImplicitBytes<const FIELD: u32, A> = ImplicitLenField<len::ProtoBytes, FIELD, A>;
pub type ExplicitBytes<const BIT: usize, const FIELD: u32, A, D = ProtoDefault> =
    ExplicitLenField<len::ProtoBytes, BIT, FIELD, A, D>;
