//! Field-number projection for generated messages.
//!
//! Generated `*Fields` structs implement [`HasSlot`] once per protobuf field
//! number. [`Record::get`](crate::Record::get) binds that slot to
//! [`MessageCommon`](crate::MessageCommon). User-facing `flag()` methods only
//! name the field and forward to that number.

use ::allocator_api2::alloc::Allocator;
use ::core::hash::Hash;
use ::core::marker::PhantomData;
use ::puroro::{DecodeError, HasDefault, Optional};
use ::unmanaged::vec::VecGuard;

use crate::WireOrSso;
use crate::decode::Lazy;
use crate::fields::map::{MapField, MapFieldMut, MapFieldRef, MapKey, MapReady, MapSpans};
use crate::fields::oneof::OneofSlot;
use crate::fields::oneof_variant::OneofVariant;
use crate::fields::repeated::container::RepeatedElementsMut;
use crate::fields::repeated::encoding::RepeatedEncoding;
use crate::fields::repeated::{DecodeLenBody, RepeatedField, RepeatedReady, RepeatedSpans};
use crate::fields::shared::field_presence::{
    Explicit, FieldPresence, Implicit, LegacyRequired, Message, Oneof,
};
use crate::fields::shared::value_layout::{ValueLayout, ValueLayoutGet, ValueLayoutMut};
use crate::fields::shared::value_slot::{AddressableSlot, ValueSlot};
use crate::fields::shared::{
    BitStorage, DefaultIn, MessageBindingMut, MessageCommon, MessageCommonAlloc, MessageCommonBits,
    MessageCommonSharedBits, UnknownStore,
};
use crate::fields::singular::SingularField;
use crate::fields::wire::encode_type::EncodeType;
use crate::fields::wire::len::{LenCodec, LenScalar};
use crate::fields::wire::numerical::{
    BoolCodec, Closed, ClosedEnum, DoubleCodec, EnumCodec, Fixed32Codec, Fixed64Codec, FloatCodec,
    Int32Codec, Int64Codec, Numerical, NumericalType, Open, OpenEnum, SFixed32Codec, SFixed64Codec,
    SInt32Codec, SInt64Codec, UInt32Codec, UInt64Codec,
};
use crate::fields::wire::proto_message::ProtoMessage;
use crate::fields::wire::repeated_element::{
    RepeatedElement, RepeatedElementMerge, RepeatedElementMut, RepeatedVecMut,
};
use crate::fields::wire::singular_type::SingularType;

/// Map protobuf field number `N` to the slot stored on a generated fields struct.
///
/// Several numbers may share one slot (oneof variants).
pub trait HasSlot<const N: u32> {
    /// Catalog wrapper for field `N` (`SingularField`, `RepeatedField`, …).
    type Slot;

    /// Shared borrow of the slot for field `N`.
    fn slot(&self) -> &Self::Slot;

    /// Mutable borrow of the slot for field `N`.
    fn slot_mut(&mut self) -> &mut Self::Slot;
}

/// Eager (infallible) read of a slot already bound to message common state.
pub trait SlotRead<Cx, const N: u32> {
    /// User-facing getter view (`i32`, `Optional<…>`, `&[T]`, …).
    type View<'a>
    where
        Self: 'a,
        Cx: 'a;

    /// Project this slot through `common`.
    fn read<'a>(&'a self, common: &'a Cx) -> Self::View<'a>;
}

/// Lazy read. The returned [`Result`] includes scan and materialise failures.
pub trait SlotReadLazy<Cx, const N: u32> {
    /// Success view. The [`Record`](crate::Record) lazy getter returns
    /// `Result<View, DecodeError>`.
    type View<'a>
    where
        Self: 'a,
        Cx: 'a;

    /// Project this slot after the parent scan has been applied.
    fn read_lazy<'a>(&'a self, common: &'a Cx) -> Result<Self::View<'a>, DecodeError>;
}

/// Eager mutable projection.
pub trait SlotMut<Cx, const N: u32> {
    /// User-facing `_mut` handle.
    type Mut<'a>
    where
        Self: 'a,
        Cx: 'a;

    /// Bind `common` and return the mutable handle.
    fn project_mut<'a>(&'a mut self, common: &'a mut Cx) -> Self::Mut<'a>;
}

/// Eager `clear_*`.
pub trait SlotClear<Cx> {
    /// Reset the slot through `common`.
    fn clear_slot(&mut self, common: &mut Cx);
}

/// Presence bit for `has_*` (lazy string / bytes).
pub trait SlotHas<Cx> {
    /// Whether the presence bit for this slot is set.
    fn is_present(&self, common: &Cx) -> bool;
}

/// How an implicit numerical field is read.
///
/// Scalars return the value. Enums return [`Optional`] even when presence is
/// implicit.
pub trait ImplicitRead: NumericalType {
    /// Getter view for `Implicit` presence.
    type Out<'a, D>
    where
        Self: 'a,
        D: HasDefault<Self::NativeType> + 'a;

    /// Read `field` through `common`.
    fn project<'a, A, L, D, Cx, const FIELD: u32>(
        field: &'a SingularField<Numerical<Self>, Implicit, FIELD, A, L, D>,
        common: &'a Cx,
    ) -> Self::Out<'a, D>
    where
        A: Allocator,
        L: ValueLayoutGet<Numerical<Self>, A>,
        Cx: MessageBindingMut<A>,
        L::Slot: AddressableSlot,
        <Implicit as FieldPresence>::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
        D: HasDefault<Self::NativeType> + 'a;
}

macro_rules! impl_scalar_implicit_read {
    ($($codec:ty),* $(,)?) => {$(
        impl ImplicitRead for $codec {
            type Out<'a, D>
                = <Self as NumericalType>::NativeType
            where
                Self: 'a,
                D: HasDefault<Self::NativeType> + 'a;

            #[inline]
            fn project<'a, A, L, D, Cx, const FIELD: u32>(
                field: &'a SingularField<Numerical<Self>, Implicit, FIELD, A, L, D>,
                common: &'a Cx,
            ) -> Self::Out<'a, D>
            where
                A: Allocator,
                L: ValueLayoutGet<Numerical<Self>, A>,
                Cx: MessageBindingMut<A>,
                L::Slot: AddressableSlot,
                <Implicit as FieldPresence>::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
                D: HasDefault<Self::NativeType> + 'a,
            {
                let _ = PhantomData::<&D>;
                field.bind(common).value()
            }
        }
    )*};
}

impl_scalar_implicit_read!(
    UInt32Codec,
    UInt64Codec,
    Int32Codec,
    Int64Codec,
    SInt32Codec,
    SInt64Codec,
    BoolCodec,
    Fixed32Codec,
    SFixed32Codec,
    FloatCodec,
    Fixed64Codec,
    SFixed64Codec,
    DoubleCodec
);

macro_rules! impl_enum_implicit_read {
    ($kind:ident, $bound:ident) => {
        impl<E: $bound> ImplicitRead for EnumCodec<E, $kind> {
            type Out<'a, D>
                = Optional<E, D>
            where
                Self: 'a,
                D: HasDefault<Self::NativeType> + 'a;

            #[inline]
            fn project<'a, A, L, D, Cx, const FIELD: u32>(
                field: &'a SingularField<Numerical<Self>, Implicit, FIELD, A, L, D>,
                common: &'a Cx,
            ) -> Self::Out<'a, D>
            where
                A: Allocator,
                L: ValueLayoutGet<Numerical<Self>, A>,
                Cx: MessageBindingMut<A>,
                L::Slot: AddressableSlot,
                <Implicit as FieldPresence>::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
                D: HasDefault<Self::NativeType> + 'a,
            {
                field.bind(common).optional()
            }
        }
    };
}

impl_enum_implicit_read!(Open, OpenEnum);
impl_enum_implicit_read!(Closed, ClosedEnum);

macro_rules! impl_singular_read_lazy {
    (
        $(#[$meta:meta])*
        impl[$($gen:tt)*] $ty:ty
        where $($bound:tt)*
    ) => {
        $(#[$meta])*
        impl<$($gen)*> SlotReadLazy<Cx, FIELD> for $ty
        where
            $($bound)*
            Self: SlotRead<Cx, FIELD>,
        {
            type View<'a> = <Self as SlotRead<Cx, FIELD>>::View<'a>
            where
                Self: 'a,
                Cx: 'a;

            #[inline]
            fn read_lazy<'a>(&'a self, common: &'a Cx) -> Result<Self::View<'a>, DecodeError> {
                Ok(SlotRead::<Cx, FIELD>::read(self, common))
            }
        }
    };
}

impl<C, const FIELD: u32, A, L, D, Cx> SlotRead<Cx, FIELD>
    for SingularField<Numerical<C>, Implicit, FIELD, A, L, D>
where
    C: ImplicitRead,
    A: Allocator,
    L: ValueLayoutGet<Numerical<C>, A>,
    Cx: MessageBindingMut<A>,
    L::Slot: AddressableSlot,
    <Implicit as FieldPresence>::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
    D: HasDefault<C::NativeType> + 'static,
{
    type View<'a>
        = C::Out<'a, D>
    where
        Self: 'a,
        Cx: 'a;

    #[inline]
    fn read<'a>(&'a self, common: &'a Cx) -> Self::View<'a> {
        C::project(self, common)
    }
}

impl_singular_read_lazy! {
    impl[C, const FIELD: u32, A, L, D, Cx]
    SingularField<Numerical<C>, Implicit, FIELD, A, L, D>
    where
        C: ImplicitRead,
        A: Allocator,
        L: ValueLayoutGet<Numerical<C>, A>,
        Cx: MessageBindingMut<A>,
        L::Slot: AddressableSlot,
        <Implicit as FieldPresence>::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
        D: HasDefault<C::NativeType> + 'static,
}

macro_rules! impl_numerical_optional_read {
    ($presence:ident, $($extra:ident)*) => {
        impl<C, $(const $extra: usize,)* const FIELD: u32, A, L, D, Cx> SlotRead<Cx, FIELD>
            for SingularField<Numerical<C>, $presence $(<$extra>)*, FIELD, A, L, D>
        where
            C: NumericalType,
            A: Allocator,
            L: ValueLayoutGet<Numerical<C>, A>,
            Cx: MessageBindingMut<A>,
            L::Slot: AddressableSlot,
            <$presence $(<$extra>)* as FieldPresence>::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
            D: 'static,
            D: HasDefault<C::NativeType>,
        {
            type View<'a>
                = Optional<<Numerical<C> as EncodeType>::View<'a, A>, D>
            where
                Self: 'a,
                Cx: 'a;

            #[inline]
            fn read<'a>(&'a self, common: &'a Cx) -> Self::View<'a> {
                self.bind(common).optional()
            }
        }

        impl_singular_read_lazy! {
            impl[C, $(const $extra: usize,)* const FIELD: u32, A, L, D, Cx]
            SingularField<Numerical<C>, $presence $(<$extra>)*, FIELD, A, L, D>
            where
                C: NumericalType,
                A: Allocator,
                L: ValueLayoutGet<Numerical<C>, A>,
                Cx: MessageBindingMut<A>,
                L::Slot: AddressableSlot,
                <$presence $(<$extra>)* as FieldPresence>::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
                D: 'static,
                D: HasDefault<C::NativeType>,
        }
    };
}

impl_numerical_optional_read!(Explicit, BIT);
impl_numerical_optional_read!(LegacyRequired, BIT);

macro_rules! impl_len_read {
    ($presence:ident, $($extra:ident)*) => {
        impl<C, $(const $extra: usize,)* const FIELD: u32, A, L, D, Cx> SlotRead<Cx, FIELD>
            for SingularField<LenScalar<C>, $presence $(<$extra>)*, FIELD, A, L, D>
        where
            C: LenCodec,
            A: Allocator,
            L: ValueLayoutGet<LenScalar<C>, A>,
            Cx: MessageBindingMut<A>,
            L::Slot: AddressableSlot,
            <$presence $(<$extra>)* as FieldPresence>::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
            D: 'static,
            C: 'static,
            for<'v> D: HasDefault<&'v C::RefView>,
        {
            type View<'a>
                = Optional<<LenScalar<C> as EncodeType>::View<'a, A>, D>
            where
                Self: 'a,
                Cx: 'a;

            #[inline]
            fn read<'a>(&'a self, common: &'a Cx) -> Self::View<'a> {
                self.bind(common).optional()
            }
        }
    };
}

// Implicit strings / bytes return the view itself (`&str` / `&[u8]`).
impl<C, const FIELD: u32, A, L, D, Cx> SlotRead<Cx, FIELD>
    for SingularField<LenScalar<C>, Implicit, FIELD, A, L, D>
where
    C: LenCodec,
    A: Allocator,
    L: ValueLayoutGet<LenScalar<C>, A>,
    Cx: MessageBindingMut<A>,
    L::Slot: AddressableSlot,
    <Implicit as FieldPresence>::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
{
    type View<'a>
        = <LenScalar<C> as EncodeType>::View<'a, A>
    where
        Self: 'a,
        Cx: 'a;

    #[inline]
    fn read<'a>(&'a self, common: &'a Cx) -> Self::View<'a> {
        self.bind(common).value()
    }
}

impl_len_read!(Explicit, BIT);
impl_len_read!(LegacyRequired, BIT);

macro_rules! impl_message_read {
    ($presence:ident, $($extra:ident)*) => {
        impl<M, $(const $extra: usize,)* const FIELD: u32, A, L, D, Cx> SlotRead<Cx, FIELD>
            for SingularField<ProtoMessage<M>, $presence $(<$extra>)*, FIELD, A, L, D>
        where
            M: crate::MessageEncode,
            A: Allocator,
            L: ValueLayoutGet<ProtoMessage<M>, A>,
            Cx: MessageBindingMut<A>,
            L::Slot: AddressableSlot,
            <$presence $(<$extra>)* as FieldPresence>::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
        {
            type View<'a>
                = Option<<ProtoMessage<M> as EncodeType>::View<'a, A>>
            where
                Self: 'a,
                Cx: 'a;

            #[inline]
            fn read<'a>(&'a self, common: &'a Cx) -> Self::View<'a> {
                self.bind(common).get()
            }
        }

        impl_singular_read_lazy! {
            impl[M, $(const $extra: usize,)* const FIELD: u32, A, L, D, Cx]
            SingularField<ProtoMessage<M>, $presence $(<$extra>)*, FIELD, A, L, D>
            where
                M: crate::MessageEncode,
                A: Allocator,
                L: ValueLayoutGet<ProtoMessage<M>, A>,
                Cx: MessageBindingMut<A>,
                L::Slot: AddressableSlot,
                <$presence $(<$extra>)* as FieldPresence>::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
        }
    };
}

impl_message_read!(Message,);
impl_message_read!(Explicit, BIT);
impl_message_read!(LegacyRequired, BIT);

macro_rules! impl_lazy_len_read {
    ($marker:ty, $view:ty, $call:ident) => {
        impl<P, const FIELD: u32, A, const KIND: usize, D, B, U>
            SlotReadLazy<MessageCommon<B, A, U, Lazy<A>>, FIELD>
            for SingularField<$marker, P, FIELD, A, WireOrSso<KIND>, D>
        where
            P: FieldPresence,
            A: Allocator + Clone,
            U: UnknownStore<A>,
            B: BitStorage,
            D: 'static,
            MessageCommon<B, A, U, Lazy<A>>: MessageBindingMut<A> + MessageCommonSharedBits,
            for<'v> D: HasDefault<&'v $view>,
            <WireOrSso<KIND> as ValueLayout<$marker, A>>::Slot: AddressableSlot,
            P::ValueSlot<<WireOrSso<KIND> as ValueLayout<$marker, A>>::Slot>:
                ValueSlot<<WireOrSso<KIND> as ValueLayout<$marker, A>>::Slot, A>,
        {
            type View<'a>
                = Optional<&'a $view, D>
            where
                Self: 'a,
                MessageCommon<B, A, U, Lazy<A>>: 'a;

            #[inline]
            fn read_lazy<'a>(
                &'a self,
                common: &'a MessageCommon<B, A, U, Lazy<A>>,
            ) -> Result<Self::View<'a>, DecodeError> {
                self.$call(common, common.lazy.scan.wire())
            }
        }
    };
}

impl_lazy_len_read!(crate::ProtoString, str, try_str);
impl_lazy_len_read!(crate::ProtoBytes, [u8], try_bytes);
impl_lazy_len_read!(crate::ProtoStringUnchecked, [u8], try_bytes);

impl<T, E, const FIELD: u32, A, Cx> SlotRead<Cx, FIELD>
    for RepeatedField<T, E, FIELD, A, RepeatedReady>
where
    T: RepeatedElement,
    E: RepeatedEncoding<T, A>,
    A: Allocator,
    Cx: MessageCommonAlloc<Alloc = A>,
{
    type View<'a>
        = &'a [T::Element<A>]
    where
        Self: 'a,
        Cx: 'a;

    #[inline]
    fn read<'a>(&'a self, common: &'a Cx) -> Self::View<'a> {
        self.bind(common).as_slice()
    }
}

impl<T, E, const FIELD: u32, A, Cx> SlotReadLazy<Cx, FIELD>
    for RepeatedField<T, E, FIELD, A, RepeatedReady>
where
    T: RepeatedElement,
    E: RepeatedEncoding<T, A>,
    A: Allocator,
    Cx: MessageCommonAlloc<Alloc = A>,
{
    type View<'a>
        = &'a [T::Element<A>]
    where
        Self: 'a,
        Cx: 'a;

    #[inline]
    fn read_lazy<'a>(&'a self, common: &'a Cx) -> Result<Self::View<'a>, DecodeError> {
        Ok(self.bind(common).as_slice())
    }
}

impl<T, E, const FIELD: u32, A, B, U> SlotReadLazy<MessageCommon<B, A, U, Lazy<A>>, FIELD>
    for RepeatedField<T, E, FIELD, A, RepeatedSpans>
where
    T: RepeatedElement + DecodeLenBody<A>,
    E: RepeatedEncoding<T, A>,
    A: Allocator + Clone,
    U: UnknownStore<A>,
    T::Element<A>: crate::DeallocateIn<A>,
{
    type View<'a>
        = &'a [T::Element<A>]
    where
        Self: 'a,
        MessageCommon<B, A, U, Lazy<A>>: 'a;

    #[inline]
    fn read_lazy<'a>(
        &'a self,
        common: &'a MessageCommon<B, A, U, Lazy<A>>,
    ) -> Result<Self::View<'a>, DecodeError> {
        Ok(self.bind(common.lazy.scan.wire(), common)?.as_slice())
    }
}

impl<K, V, const FIELD: u32, A, Cx> SlotRead<Cx, FIELD> for MapField<K, V, FIELD, A, MapReady>
where
    K: MapKey,
    V: RepeatedElement,
    A: Allocator,
    Cx: MessageCommonAlloc<Alloc = A>,
{
    type View<'a>
        = MapFieldRef<'a, K, V, FIELD, A, Cx>
    where
        Self: 'a,
        Cx: 'a;

    #[inline]
    fn read<'a>(&'a self, common: &'a Cx) -> Self::View<'a> {
        self.bind(common)
    }
}

impl<K, V, const FIELD: u32, A, Cx> SlotReadLazy<Cx, FIELD> for MapField<K, V, FIELD, A, MapReady>
where
    K: MapKey,
    V: RepeatedElement,
    A: Allocator,
    Cx: MessageCommonAlloc<Alloc = A>,
{
    type View<'a>
        = MapFieldRef<'a, K, V, FIELD, A, Cx>
    where
        Self: 'a,
        Cx: 'a;

    #[inline]
    fn read_lazy<'a>(&'a self, common: &'a Cx) -> Result<Self::View<'a>, DecodeError> {
        Ok(self.bind(common))
    }
}

impl<K, V, const FIELD: u32, A, B, U> SlotReadLazy<MessageCommon<B, A, U, Lazy<A>>, FIELD>
    for MapField<K, V, FIELD, A, MapSpans>
where
    K: MapKey + RepeatedElementMerge<A>,
    V: RepeatedElement + RepeatedElementMerge<A>,
    A: Allocator + Clone,
    U: UnknownStore<A>,
    K::Element<A>: Eq + Hash,
{
    type View<'a>
        = MapFieldRef<'a, K, V, FIELD, A, MessageCommon<B, A, U, Lazy<A>>>
    where
        Self: 'a,
        MessageCommon<B, A, U, Lazy<A>>: 'a;

    #[inline]
    fn read_lazy<'a>(
        &'a self,
        common: &'a MessageCommon<B, A, U, Lazy<A>>,
    ) -> Result<Self::View<'a>, DecodeError> {
        self.bind(common.lazy.scan.wire(), common)
    }
}

// ---------------------------------------------------------------------------
// Oneof variants: one slot, many field numbers.
// ---------------------------------------------------------------------------

/// Read one oneof member identified by `N` off the shared [`OneofSlot`].
pub trait VariantSlotRead<Cx, const N: u32> {
    /// Getter view for variant `N`.
    type View<'a>
    where
        Self: 'a,
        Cx: 'a;

    /// Project variant `N` from `slot`.
    fn read_from<'a, E>(slot: &'a OneofSlot<E>, common: &'a Cx) -> Self::View<'a>
    where
        Self: 'a,
        E: OneofVariant<N, Value = Self>;
}

/// Lazy read of one oneof member.
pub trait VariantSlotReadLazy<Cx, const N: u32> {
    /// Success view for variant `N`.
    type View<'a>
    where
        Self: 'a,
        Cx: 'a;

    /// Project variant `N` after the parent scan.
    fn read_from<'a, E>(
        slot: &'a OneofSlot<E>,
        common: &'a Cx,
    ) -> Result<Self::View<'a>, DecodeError>
    where
        Self: 'a,
        E: OneofVariant<N, Value = Self>;
}

/// Mutable projection of one oneof member.
pub trait VariantSlotMut<Pb, A, U, Ly, const N: u32>
where
    A: Allocator,
{
    /// `_mut` handle for variant `N`.
    type Mut<'a>
    where
        Self: 'a,
        MessageCommon<Pb, A, U, Ly>: 'a;

    /// Install variant `N` if needed and return its mutable handle.
    fn project_from<'a, E>(
        slot: &'a mut OneofSlot<E>,
        common: &'a mut MessageCommon<Pb, A, U, Ly>,
    ) -> Self::Mut<'a>
    where
        Self: 'a,
        E: OneofVariant<N, Value = Self>
            + crate::OneofGroup<Bits = Pb, Alloc = A, Unknown = U, Layout = Ly>,
        U: UnknownStore<A>;
}

impl<E, Cx, const N: u32> SlotRead<Cx, N> for OneofSlot<E>
where
    E: OneofVariant<N>,
    <E as OneofVariant<N>>::Value: VariantSlotRead<Cx, N>,
{
    type View<'a>
        = <<E as OneofVariant<N>>::Value as VariantSlotRead<Cx, N>>::View<'a>
    where
        Self: 'a,
        Cx: 'a;

    #[inline]
    fn read<'a>(&'a self, common: &'a Cx) -> Self::View<'a> {
        <<E as OneofVariant<N>>::Value as VariantSlotRead<Cx, N>>::read_from(self, common)
    }
}

impl<E, Cx, const N: u32> SlotReadLazy<Cx, N> for OneofSlot<E>
where
    E: OneofVariant<N>,
    <E as OneofVariant<N>>::Value: VariantSlotReadLazy<Cx, N>,
{
    type View<'a>
        = <<E as OneofVariant<N>>::Value as VariantSlotReadLazy<Cx, N>>::View<'a>
    where
        Self: 'a,
        Cx: 'a;

    #[inline]
    fn read_lazy<'a>(&'a self, common: &'a Cx) -> Result<Self::View<'a>, DecodeError> {
        <<E as OneofVariant<N>>::Value as VariantSlotReadLazy<Cx, N>>::read_from(self, common)
    }
}

impl<E, Pb, A, U, Ly, const N: u32> SlotMut<MessageCommon<Pb, A, U, Ly>, N> for OneofSlot<E>
where
    A: Allocator,
    U: UnknownStore<A>,
    E: OneofVariant<N> + crate::OneofGroup<Bits = Pb, Alloc = A, Unknown = U, Layout = Ly>,
    <E as OneofVariant<N>>::Value: VariantSlotMut<Pb, A, U, Ly, N>,
{
    type Mut<'a>
        = <<E as OneofVariant<N>>::Value as VariantSlotMut<Pb, A, U, Ly, N>>::Mut<'a>
    where
        Self: 'a,
        MessageCommon<Pb, A, U, Ly>: 'a;

    #[inline]
    fn project_mut<'a>(&'a mut self, common: &'a mut MessageCommon<Pb, A, U, Ly>) -> Self::Mut<'a> {
        <<E as OneofVariant<N>>::Value as VariantSlotMut<Pb, A, U, Ly, N>>::project_from(
            self, common,
        )
    }
}

fn oneof_optional<'a, T, const N: u32, A, L, D, Pb, U, Ly, E>(
    slot: &'a OneofSlot<E>,
    common: &'a MessageCommon<Pb, A, U, Ly>,
) -> Optional<T::View<'a, A>, D>
where
    T: SingularType,
    A: Allocator,
    L: ValueLayoutGet<T, A> + 'a,
    U: UnknownStore<A>,
    E: OneofVariant<N, Value = SingularField<T, Oneof, N, A, L, D>>,
    T::View<'a, A>: Copy,
    D: HasDefault<T::View<'a, A>> + 'a,
    MessageCommon<Pb, A, U, Ly>: MessageCommonBits,
    L::Slot: AddressableSlot,
    <Oneof as FieldPresence>::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
{
    slot.bind(common).variant_of::<N>().optional()
}

impl<C, const N: u32, A, L, D, Pb, U, Ly> VariantSlotRead<MessageCommon<Pb, A, U, Ly>, N>
    for SingularField<Numerical<C>, Oneof, N, A, L, D>
where
    C: NumericalType,
    A: Allocator,
    L: ValueLayoutGet<Numerical<C>, A>,
    U: UnknownStore<A>,
    D: 'static,
    D: HasDefault<C::NativeType>,
    MessageCommon<Pb, A, U, Ly>: MessageCommonBits,
    L::Slot: AddressableSlot,
    <Oneof as FieldPresence>::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
{
    type View<'a>
        = Optional<<Numerical<C> as EncodeType>::View<'a, A>, D>
    where
        Self: 'a,
        MessageCommon<Pb, A, U, Ly>: 'a;

    #[inline]
    fn read_from<'a, E>(
        slot: &'a OneofSlot<E>,
        common: &'a MessageCommon<Pb, A, U, Ly>,
    ) -> Self::View<'a>
    where
        Self: 'a,
        E: OneofVariant<N, Value = Self>,
    {
        oneof_optional::<Numerical<C>, N, A, L, D, Pb, U, Ly, E>(slot, common)
    }
}

impl<C, const N: u32, A, L, D, Pb, U, Ly> VariantSlotReadLazy<MessageCommon<Pb, A, U, Ly>, N>
    for SingularField<Numerical<C>, Oneof, N, A, L, D>
where
    C: NumericalType,
    A: Allocator,
    L: ValueLayoutGet<Numerical<C>, A>,
    U: UnknownStore<A>,
    D: 'static,
    D: HasDefault<C::NativeType>,
    MessageCommon<Pb, A, U, Ly>: MessageCommonBits,
    L::Slot: AddressableSlot,
    <Oneof as FieldPresence>::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
{
    type View<'a>
        = Optional<<Numerical<C> as EncodeType>::View<'a, A>, D>
    where
        Self: 'a,
        MessageCommon<Pb, A, U, Ly>: 'a;

    #[inline]
    fn read_from<'a, E>(
        slot: &'a OneofSlot<E>,
        common: &'a MessageCommon<Pb, A, U, Ly>,
    ) -> Result<Self::View<'a>, DecodeError>
    where
        Self: 'a,
        E: OneofVariant<N, Value = Self>,
    {
        Ok(oneof_optional::<Numerical<C>, N, A, L, D, Pb, U, Ly, E>(
            slot, common,
        ))
    }
}

impl<C, const N: u32, A, L, D, Pb, U, Ly> VariantSlotRead<MessageCommon<Pb, A, U, Ly>, N>
    for SingularField<LenScalar<C>, Oneof, N, A, L, D>
where
    C: LenCodec,
    A: Allocator,
    L: ValueLayoutGet<LenScalar<C>, A>,
    U: UnknownStore<A>,
    D: 'static,
    C: 'static,
    for<'v> D: HasDefault<&'v C::RefView>,
    MessageCommon<Pb, A, U, Ly>: MessageCommonBits,
    L::Slot: AddressableSlot,
    <Oneof as FieldPresence>::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
{
    type View<'a>
        = Optional<<LenScalar<C> as EncodeType>::View<'a, A>, D>
    where
        Self: 'a,
        MessageCommon<Pb, A, U, Ly>: 'a;

    #[inline]
    fn read_from<'a, E>(
        slot: &'a OneofSlot<E>,
        common: &'a MessageCommon<Pb, A, U, Ly>,
    ) -> Self::View<'a>
    where
        Self: 'a,
        E: OneofVariant<N, Value = Self>,
    {
        oneof_optional::<LenScalar<C>, N, A, L, D, Pb, U, Ly, E>(slot, common)
    }
}

impl<M, const N: u32, A, L, D, Pb, U, Ly> VariantSlotRead<MessageCommon<Pb, A, U, Ly>, N>
    for SingularField<ProtoMessage<M>, Oneof, N, A, L, D>
where
    M: crate::MessageEncode,
    A: Allocator,
    L: ValueLayoutGet<ProtoMessage<M>, A>,
    U: UnknownStore<A>,
    MessageCommon<Pb, A, U, Ly>: MessageCommonBits,
    L::Slot: AddressableSlot,
    <Oneof as FieldPresence>::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
{
    type View<'a>
        = Option<<ProtoMessage<M> as EncodeType>::View<'a, A>>
    where
        Self: 'a,
        MessageCommon<Pb, A, U, Ly>: 'a;

    #[inline]
    fn read_from<'a, E>(
        slot: &'a OneofSlot<E>,
        common: &'a MessageCommon<Pb, A, U, Ly>,
    ) -> Self::View<'a>
    where
        Self: 'a,
        E: OneofVariant<N, Value = Self>,
    {
        slot.bind(common).variant_of::<N>().get()
    }
}

impl<M, const N: u32, A, L, D, Pb, U, Ly> VariantSlotReadLazy<MessageCommon<Pb, A, U, Ly>, N>
    for SingularField<ProtoMessage<M>, Oneof, N, A, L, D>
where
    M: crate::MessageEncode,
    A: Allocator,
    L: ValueLayoutGet<ProtoMessage<M>, A>,
    U: UnknownStore<A>,
    MessageCommon<Pb, A, U, Ly>: MessageCommonBits,
    L::Slot: AddressableSlot,
    <Oneof as FieldPresence>::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
{
    type View<'a>
        = Option<<ProtoMessage<M> as EncodeType>::View<'a, A>>
    where
        Self: 'a,
        MessageCommon<Pb, A, U, Ly>: 'a;

    #[inline]
    fn read_from<'a, E>(
        slot: &'a OneofSlot<E>,
        common: &'a MessageCommon<Pb, A, U, Ly>,
    ) -> Result<Self::View<'a>, DecodeError>
    where
        Self: 'a,
        E: OneofVariant<N, Value = Self>,
    {
        Ok(slot.bind(common).variant_of::<N>().get())
    }
}

macro_rules! impl_lazy_oneof_len {
    ($marker:ty, $view:ty, $call:ident) => {
        impl<const N: u32, A, const KIND: usize, D, B, U>
            VariantSlotReadLazy<MessageCommon<B, A, U, Lazy<A>>, N>
            for SingularField<$marker, Oneof, N, A, WireOrSso<KIND>, D>
        where
            A: Allocator + Clone,
            U: UnknownStore<A>,
            B: BitStorage,
            D: 'static,
            MessageCommon<B, A, U, Lazy<A>>:
                MessageBindingMut<A> + MessageCommonSharedBits + MessageCommonBits,
            for<'v> D: HasDefault<&'v $view>,
            <WireOrSso<KIND> as ValueLayout<$marker, A>>::Slot: AddressableSlot,
            <Oneof as FieldPresence>::ValueSlot<<WireOrSso<KIND> as ValueLayout<$marker, A>>::Slot>:
                ValueSlot<<WireOrSso<KIND> as ValueLayout<$marker, A>>::Slot, A>,
        {
            type View<'a>
                = Optional<&'a $view, D>
            where
                Self: 'a,
                MessageCommon<B, A, U, Lazy<A>>: 'a;

            #[inline]
            fn read_from<'a, E>(
                slot: &'a OneofSlot<E>,
                common: &'a MessageCommon<B, A, U, Lazy<A>>,
            ) -> Result<Self::View<'a>, DecodeError>
            where
                Self: 'a,
                E: OneofVariant<N, Value = Self>,
            {
                match slot.bind(common).variant_of::<N>().as_field() {
                    Some(field) => field.$call(common, common.lazy.scan.wire()),
                    None => Ok(Optional::new(None)),
                }
            }
        }
    };
}

impl_lazy_oneof_len!(crate::ProtoString, str, try_str);
impl_lazy_oneof_len!(crate::ProtoBytes, [u8], try_bytes);
impl_lazy_oneof_len!(crate::ProtoStringUnchecked, [u8], try_bytes);

fn oneof_value_mut<'a, T, const N: u32, A, L, D: 'a, Pb, U, Ly, E>(
    slot: &'a mut OneofSlot<E>,
    common: &'a mut MessageCommon<Pb, A, U, Ly>,
) -> <L as ValueLayoutMut<T, A>>::Mut<'a>
where
    T: SingularType,
    A: Allocator + Clone,
    L: ValueLayoutMut<T, A>,
    U: UnknownStore<A>,
    L::Slot: AddressableSlot + DefaultIn<A>,
    <Oneof as FieldPresence>::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
    MessageCommon<Pb, A, U, Ly>: MessageBindingMut<A>,
    E: OneofVariant<N, Value = SingularField<T, Oneof, N, A, L, D>>
        + crate::OneofGroup<Bits = Pb, Alloc = A, Unknown = U, Layout = Ly>,
    SingularField<T, Oneof, N, A, L, D>: DefaultIn<A>,
{
    slot.bind_mut(common)
        .variant_mut::<N>()
        .bind_mut(common)
        .value_mut()
}

impl<T, const N: u32, A, L, D, Pb, U, Ly> VariantSlotMut<Pb, A, U, Ly, N>
    for SingularField<T, Oneof, N, A, L, D>
where
    T: SingularType,
    A: Allocator + Clone,
    L: ValueLayoutMut<T, A>,
    U: UnknownStore<A>,
    L::Slot: AddressableSlot + DefaultIn<A>,
    <Oneof as FieldPresence>::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
    MessageCommon<Pb, A, U, Ly>: MessageBindingMut<A>,
    Self: DefaultIn<A>,
{
    type Mut<'a>
        = <L as ValueLayoutMut<T, A>>::Mut<'a>
    where
        Self: 'a,
        MessageCommon<Pb, A, U, Ly>: 'a;

    #[inline]
    fn project_from<'a, E>(
        slot: &'a mut OneofSlot<E>,
        common: &'a mut MessageCommon<Pb, A, U, Ly>,
    ) -> Self::Mut<'a>
    where
        Self: 'a,
        E: OneofVariant<N, Value = Self>
            + crate::OneofGroup<Bits = Pb, Alloc = A, Unknown = U, Layout = Ly>,
        U: UnknownStore<A>,
    {
        oneof_value_mut::<T, N, A, L, D, Pb, U, Ly, E>(slot, common)
    }
}

// ---------------------------------------------------------------------------
// Mutation and clear for ordinary slots.
// ---------------------------------------------------------------------------

impl<T, P, const FIELD: u32, A, L, D, Cx> SlotMut<Cx, FIELD> for SingularField<T, P, FIELD, A, L, D>
where
    T: SingularType,
    P: FieldPresence,
    A: Allocator + Clone,
    L: ValueLayoutMut<T, A>,
    Cx: MessageBindingMut<A>,
    L::Slot: AddressableSlot + DefaultIn<A>,
    P::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
{
    type Mut<'a>
        = <L as ValueLayoutMut<T, A>>::Mut<'a>
    where
        Self: 'a,
        Cx: 'a;

    #[inline]
    fn project_mut<'a>(&'a mut self, common: &'a mut Cx) -> Self::Mut<'a> {
        self.bind_mut(common).value_mut()
    }
}

impl<T, P, const FIELD: u32, A, L, D, Cx> SlotClear<Cx> for SingularField<T, P, FIELD, A, L, D>
where
    T: SingularType,
    P: FieldPresence,
    A: Allocator + Clone,
    L: ValueLayout<T, A>,
    Cx: MessageBindingMut<A>,
    L::Slot: AddressableSlot + DefaultIn<A>,
    P::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
{
    #[inline]
    fn clear_slot(&mut self, common: &mut Cx) {
        self.bind_mut(common).clear();
    }
}

impl<T, const BIT: usize, const FIELD: u32, A, L, D, Cx> SlotHas<Cx>
    for SingularField<T, Explicit<BIT>, FIELD, A, L, D>
where
    T: SingularType,
    A: Allocator,
    L: ValueLayout<T, A>,
    Cx: MessageCommonBits,
    L::Slot: AddressableSlot,
    <Explicit<BIT> as FieldPresence>::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
{
    #[inline]
    fn is_present(&self, common: &Cx) -> bool {
        common.is_bit_set(BIT)
    }
}

impl<T, const BIT: usize, const FIELD: u32, A, L, D, Cx> SlotHas<Cx>
    for SingularField<T, LegacyRequired<BIT>, FIELD, A, L, D>
where
    T: SingularType,
    A: Allocator,
    L: ValueLayout<T, A>,
    Cx: MessageCommonBits,
    L::Slot: AddressableSlot,
    <LegacyRequired<BIT> as FieldPresence>::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
{
    #[inline]
    fn is_present(&self, common: &Cx) -> bool {
        common.is_bit_set(BIT)
    }
}

impl<C, E, const FIELD: u32, A, Cx> SlotMut<Cx, FIELD>
    for RepeatedField<LenScalar<C>, E, FIELD, A, RepeatedReady>
where
    C: LenCodec,
    E: RepeatedEncoding<LenScalar<C>, A>,
    A: Allocator + Clone,
    Cx: MessageCommonAlloc<Alloc = A>,
    LenScalar<C>: RepeatedElementMut + RepeatedElementMerge<A>,
{
    type Mut<'a>
        = RepeatedElementsMut<'a, LenScalar<C>, A>
    where
        Self: 'a,
        Cx: 'a;

    #[inline]
    fn project_mut<'a>(&'a mut self, common: &'a mut Cx) -> Self::Mut<'a> {
        self.bind_mut(common).container_mut()
    }
}

impl<C, E, const FIELD: u32, A, Cx> SlotMut<Cx, FIELD>
    for RepeatedField<Numerical<C>, E, FIELD, A, RepeatedReady>
where
    C: NumericalType,
    E: RepeatedEncoding<Numerical<C>, A>,
    A: Allocator + Clone,
    Cx: MessageCommonAlloc<Alloc = A>,
    Numerical<C>: RepeatedVecMut,
{
    type Mut<'a>
        = VecGuard<'a, <Numerical<C> as RepeatedElement>::Element<A>, A>
    where
        Self: 'a,
        Cx: 'a;

    #[inline]
    fn project_mut<'a>(&'a mut self, common: &'a mut Cx) -> Self::Mut<'a> {
        self.bind_mut(common).values_mut()
    }
}

impl<M, E, const FIELD: u32, A, Cx> SlotMut<Cx, FIELD>
    for RepeatedField<ProtoMessage<M>, E, FIELD, A, RepeatedReady>
where
    M: crate::MessageEncode,
    E: RepeatedEncoding<ProtoMessage<M>, A>,
    A: Allocator + Clone,
    Cx: MessageCommonAlloc<Alloc = A>,
    ProtoMessage<M>: RepeatedVecMut,
{
    type Mut<'a>
        = VecGuard<'a, <ProtoMessage<M> as RepeatedElement>::Element<A>, A>
    where
        Self: 'a,
        Cx: 'a;

    #[inline]
    fn project_mut<'a>(&'a mut self, common: &'a mut Cx) -> Self::Mut<'a> {
        self.bind_mut(common).values_mut()
    }
}

impl<T, E, const FIELD: u32, A, Cx> SlotClear<Cx> for RepeatedField<T, E, FIELD, A, RepeatedReady>
where
    T: RepeatedElement,
    E: RepeatedEncoding<T, A>,
    A: Allocator + Clone,
    Cx: MessageCommonAlloc<Alloc = A>,
{
    #[inline]
    fn clear_slot(&mut self, common: &mut Cx) {
        self.bind_mut(common).clear();
    }
}

impl<K, V, const FIELD: u32, A, Cx> SlotMut<Cx, FIELD> for MapField<K, V, FIELD, A, MapReady>
where
    K: MapKey,
    V: RepeatedElement,
    A: Allocator,
    Cx: MessageCommonAlloc<Alloc = A>,
{
    type Mut<'a>
        = MapFieldMut<'a, 'a, K, V, FIELD, A, Cx>
    where
        Self: 'a,
        Cx: 'a;

    #[inline]
    fn project_mut<'a>(&'a mut self, common: &'a mut Cx) -> Self::Mut<'a> {
        self.bind_mut(common)
    }
}

impl<K, V, const FIELD: u32, A, Cx> SlotClear<Cx> for MapField<K, V, FIELD, A, MapReady>
where
    K: MapKey,
    V: RepeatedElement,
    A: Allocator + Clone,
    Cx: MessageCommonAlloc<Alloc = A>,
{
    #[inline]
    fn clear_slot(&mut self, common: &mut Cx) {
        self.bind_mut(common).clear();
    }
}
