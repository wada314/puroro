//! Field-number projection for generated messages.
//!
//! Generated `*Fields` structs implement [`HasField`] once per protobuf field
//! number. The associated type is the stored member for an ordinary field, and
//! a oneof member view when several numbers share one [`OneofSlot`](super::oneof::OneofSlot).
//! [`Record::field`](crate::Record::field) binds that member to `MessageCommon`
//! through [`BindField`] / [`BindFieldMut`]. Getters then call an inherent method
//! on the bound view (`optional`, `value`, `get`, `as_slice`, …).

use ::allocator_api2::alloc::Allocator;
use ::puroro::{DecodeError, HasDefault, Optional};

use super::map::{MapField, MapFieldMut, MapFieldRef, MapKey};
use super::oneof::{OneofGroup, OneofSlot, OneofVariantRef};
use super::oneof_variant::OneofVariant;
use super::repeated::encoding::RepeatedEncoding;
use super::repeated::{RepeatedField, RepeatedFieldMut, RepeatedFieldRef};
use super::shared::field_presence::FieldPresence;
use super::shared::value_slot::ValueSlot;
use super::shared::{
    AddressableSlot, BitStorage, DefaultIn, MessageBindingMut, MessageCommon, MessageCommonAlloc,
    MessageCommonSharedBits, UnknownStore, ValueLayout,
};
use super::singular::{SingularField, SingularFieldMut, SingularFieldRef};
use super::wire::len::{BytesLikeLenCodec, LenScalar};
use super::wire::repeated_element::RepeatedElement;
use super::wire::singular_type::SingularType;
use super::wire::wire_or_sso::WireOrSso;
use crate::ProtoString;

/// Map protobuf field number `N` to the value generated code reads and writes.
///
/// On `*Fields`, ordinary numbers return `&SingularField` / `&RepeatedField` /
/// `&MapField`, and oneof numbers return [`OneofFieldRef`] / [`OneofFieldMut`].
/// [`Record::field`](crate::Record::field) binds that projection to `MessageCommon`.
pub trait HasField<const N: u32> {
    /// Borrowed projection of field `N`.
    type Ref<'a>
    where
        Self: 'a;

    /// Mutable projection of field `N`.
    type Mut<'a>
    where
        Self: 'a;

    /// Borrow the projection for field `N`.
    fn field<'a>(&'a self) -> Self::Ref<'a>;

    /// Mutably borrow the projection for field `N`.
    fn field_mut<'a>(&'a mut self) -> Self::Mut<'a>;
}

/// Bind a field projection to message common state for reading.
///
/// Implemented for the projections [`HasField`] returns from `*Fields` when
/// `bind` takes only `common`. A wire buffer is a different method.
pub trait BindField<'a, Cx: ?Sized> {
    /// View produced by binding `self` to `common`.
    type Bound;

    /// Pair `self` with `common`.
    fn bind_field(self, common: &'a Cx) -> Self::Bound;
}

/// Bind a field projection to message common state for mutation.
pub trait BindFieldMut<'f, 'c, Cx: ?Sized> {
    /// Mutable view produced by binding `self` to `common`.
    type Bound;

    /// Pair `self` with `common`.
    fn bind_field_mut(self, common: &'c mut Cx) -> Self::Bound;
}

/// Read view of one oneof member, keyed by its protobuf field number.
pub struct OneofFieldRef<'a, E, const N: u32> {
    slot: &'a OneofSlot<E>,
}

impl<'a, E, const N: u32> OneofFieldRef<'a, E, N> {
    /// Wraps the shared oneof member that stores field `N`.
    #[inline]
    pub fn new(slot: &'a OneofSlot<E>) -> Self {
        Self { slot }
    }

    /// Bind `common` and project variant `N`.
    ///
    /// The result has the same inherent `optional` / `get` methods as a
    /// directly stored singular field's bound view.
    #[inline]
    pub fn bind<Pb, A, U, L>(
        self,
        common: &'a MessageCommon<Pb, A, U, L>,
    ) -> OneofVariantRef<'a, <E as OneofVariant<N>>::Value, Pb, A, U, L>
    where
        A: Allocator,
        U: UnknownStore<A>,
        E: OneofVariant<N>,
    {
        self.slot.bind(common).variant_of::<N>()
    }
}

impl<'a, E, const N: u32, A, const KIND: usize, D> OneofFieldRef<'a, E, N>
where
    A: Allocator + Clone,
    E: OneofVariant<
            N,
            Value = SingularField<
                ProtoString,
                super::shared::field_presence::Oneof,
                N,
                A,
                WireOrSso<KIND>,
                D,
            >,
        >,
{
    /// UTF-8 read of variant `N`. Absent when another variant is active.
    #[inline]
    pub fn try_str<Pb, U, L>(
        self,
        common: &'a MessageCommon<Pb, A, U, L>,
        wire: &'a [u8],
    ) -> Result<Optional<&'a str, D>, DecodeError>
    where
        U: UnknownStore<A>,
        Pb: BitStorage,
        D: HasDefault<&'a str> + 'a,
        MessageCommon<Pb, A, U, L>: MessageCommonSharedBits,
    {
        match self.bind(common).as_field() {
            Some(field) => field.try_str(common, wire),
            None => Ok(Optional::new(None)),
        }
    }
}

impl<'a, E, const N: u32, C, A, const KIND: usize, D> OneofFieldRef<'a, E, N>
where
    C: BytesLikeLenCodec,
    A: Allocator + Clone,
    E: OneofVariant<
            N,
            Value = SingularField<
                LenScalar<C>,
                super::shared::field_presence::Oneof,
                N,
                A,
                WireOrSso<KIND>,
                D,
            >,
        >,
{
    /// Bytes read of variant `N`. Absent when another variant is active.
    #[inline]
    pub fn try_bytes<Pb, U, L>(
        self,
        common: &'a MessageCommon<Pb, A, U, L>,
        wire: &'a [u8],
    ) -> Result<Optional<&'a [u8], D>, DecodeError>
    where
        U: UnknownStore<A>,
        Pb: BitStorage,
        C: 'a,
        D: HasDefault<&'a [u8]> + 'a,
        MessageCommon<Pb, A, U, L>: MessageCommonSharedBits,
    {
        match self.bind(common).as_field() {
            Some(field) => field.try_bytes(common, wire),
            None => Ok(Optional::new(None)),
        }
    }
}

/// Mutable view of one oneof member, keyed by its protobuf field number.
pub struct OneofFieldMut<'a, E, const N: u32> {
    slot: &'a mut OneofSlot<E>,
}

impl<'a, E, const N: u32> OneofFieldMut<'a, E, N> {
    /// Wraps the shared oneof member that stores field `N`.
    #[inline]
    pub fn new(slot: &'a mut OneofSlot<E>) -> Self {
        Self { slot }
    }

    /// Install variant `N` if needed and return its stored field.
    ///
    /// The result is the same `&mut SingularField` a directly stored field
    /// would be, so `store_len_span` / `bind_mut` resolve as inherent methods.
    #[inline]
    pub fn variant<'c, Pb, A, U, L>(
        self,
        common: &'c mut MessageCommon<Pb, A, U, L>,
    ) -> &'a mut <E as OneofVariant<N>>::Value
    where
        A: Allocator + Clone,
        U: UnknownStore<A>,
        E: OneofVariant<N> + OneofGroup<Bits = Pb, Alloc = A, Unknown = U, Layout = L>,
        <E as OneofVariant<N>>::Value: DefaultIn<A>,
    {
        self.slot.bind_mut(common).variant_mut::<N>()
    }

    /// Install variant `N` if needed and bind it for mutation.
    #[inline]
    pub fn bind_mut<'c, T, L, D, Pb, A, U, Ly>(
        self,
        common: &'c mut MessageCommon<Pb, A, U, Ly>,
    ) -> SingularFieldMut<
        'a,
        'c,
        T,
        super::shared::field_presence::Oneof,
        N,
        A,
        L,
        D,
        MessageCommon<Pb, A, U, Ly>,
    >
    where
        A: Allocator + Clone,
        U: UnknownStore<A>,
        T: super::wire::singular_type::SingularType,
        L: super::shared::value_layout::ValueLayout<T, A>,
        E: OneofVariant<
                N,
                Value = SingularField<T, super::shared::field_presence::Oneof, N, A, L, D>,
            > + OneofGroup<Bits = Pb, Alloc = A, Unknown = U, Layout = Ly>,
        SingularField<T, super::shared::field_presence::Oneof, N, A, L, D>: DefaultIn<A>,
        MessageCommon<Pb, A, U, Ly>: super::shared::MessageBindingMut<A>,
    {
        let field: &'a mut SingularField<T, super::shared::field_presence::Oneof, N, A, L, D> =
            self.variant(common);
        field.bind_mut(common)
    }
}

impl<'a, T, P, const FIELD: u32, A, L, D, Cx> BindField<'a, Cx>
    for &'a SingularField<T, P, FIELD, A, L, D>
where
    T: SingularType,
    P: FieldPresence,
    A: Allocator,
    L: ValueLayout<T, A>,
    L::Slot: AddressableSlot,
    P::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
    Cx: MessageBindingMut<A> + 'a,
{
    type Bound = SingularFieldRef<'a, T, P, FIELD, A, L, D, Cx>;

    #[inline]
    fn bind_field(self, common: &'a Cx) -> Self::Bound {
        SingularField::bind(self, common)
    }
}

impl<'f, 'c, T, P, const FIELD: u32, A, L, D, Cx> BindFieldMut<'f, 'c, Cx>
    for &'f mut SingularField<T, P, FIELD, A, L, D>
where
    T: SingularType,
    P: FieldPresence,
    A: Allocator,
    L: ValueLayout<T, A>,
    L::Slot: AddressableSlot,
    P::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
    Cx: MessageBindingMut<A> + 'c,
{
    type Bound = SingularFieldMut<'f, 'c, T, P, FIELD, A, L, D, Cx>;

    #[inline]
    fn bind_field_mut(self, common: &'c mut Cx) -> Self::Bound {
        SingularField::bind_mut(self, common)
    }
}

impl<'a, T, E, const FIELD: u32, A, Cx> BindField<'a, Cx> for &'a RepeatedField<T, E, FIELD, A>
where
    T: RepeatedElement,
    E: RepeatedEncoding<T, A>,
    A: Allocator,
    Cx: MessageCommonAlloc<Alloc = A> + 'a,
{
    type Bound = RepeatedFieldRef<'a, T, E, FIELD, A, Cx>;

    #[inline]
    fn bind_field(self, common: &'a Cx) -> Self::Bound {
        self.bind(common)
    }
}

impl<'f, 'c, T, E, const FIELD: u32, A, Cx> BindFieldMut<'f, 'c, Cx>
    for &'f mut RepeatedField<T, E, FIELD, A>
where
    T: RepeatedElement,
    E: RepeatedEncoding<T, A>,
    A: Allocator,
    Cx: MessageCommonAlloc<Alloc = A> + 'c,
{
    type Bound = RepeatedFieldMut<'f, 'c, T, E, FIELD, A, Cx>;

    #[inline]
    fn bind_field_mut(self, common: &'c mut Cx) -> Self::Bound {
        self.bind_mut(common)
    }
}

impl<'a, K, V, const FIELD: u32, A, Cx> BindField<'a, Cx> for &'a MapField<K, V, FIELD, A>
where
    K: MapKey,
    V: RepeatedElement,
    A: Allocator,
    Cx: MessageCommonAlloc<Alloc = A> + 'a,
{
    type Bound = MapFieldRef<'a, K, V, FIELD, A, Cx>;

    #[inline]
    fn bind_field(self, common: &'a Cx) -> Self::Bound {
        self.bind(common)
    }
}

impl<'f, 'c, K, V, const FIELD: u32, A, Cx> BindFieldMut<'f, 'c, Cx>
    for &'f mut MapField<K, V, FIELD, A>
where
    K: MapKey,
    V: RepeatedElement,
    A: Allocator,
    Cx: MessageCommonAlloc<Alloc = A> + 'c,
{
    type Bound = MapFieldMut<'f, 'c, K, V, FIELD, A, Cx>;

    #[inline]
    fn bind_field_mut(self, common: &'c mut Cx) -> Self::Bound {
        self.bind_mut(common)
    }
}

impl<'a, E, const N: u32, Pb, A, U, L> BindField<'a, MessageCommon<Pb, A, U, L>>
    for OneofFieldRef<'a, E, N>
where
    A: Allocator + 'a,
    U: UnknownStore<A> + 'a,
    Pb: 'a,
    L: 'a,
    E: OneofVariant<N>,
{
    type Bound = OneofVariantRef<'a, <E as OneofVariant<N>>::Value, Pb, A, U, L>;

    #[inline]
    fn bind_field(self, common: &'a MessageCommon<Pb, A, U, L>) -> Self::Bound {
        OneofFieldRef::bind(self, common)
    }
}

impl<'a, 'c, E, const N: u32, Pb, A, U, Ly> BindFieldMut<'a, 'c, MessageCommon<Pb, A, U, Ly>>
    for OneofFieldMut<'a, E, N>
where
    A: Allocator + Clone,
    U: UnknownStore<A>,
    E: OneofVariant<N> + OneofGroup<Bits = Pb, Alloc = A, Unknown = U, Layout = Ly>,
    <E as OneofVariant<N>>::Value: DefaultIn<A>,
    &'a mut <E as OneofVariant<N>>::Value: BindFieldMut<'a, 'c, MessageCommon<Pb, A, U, Ly>>,
{
    type Bound = <&'a mut <E as OneofVariant<N>>::Value as BindFieldMut<
        'a,
        'c,
        MessageCommon<Pb, A, U, Ly>,
    >>::Bound;

    #[inline]
    fn bind_field_mut(self, common: &'c mut MessageCommon<Pb, A, U, Ly>) -> Self::Bound {
        let field: &'a mut <E as OneofVariant<N>>::Value = self.variant(common);
        BindFieldMut::bind_field_mut(field, common)
    }
}
