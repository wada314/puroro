//! Sample of the code puroro generates for message `example.Address`
//! (from `example.proto`). The public type is re-exported at the crate root so
//! it sits beside companion module [`crate::address`].
//!
//! [`AddressFields`] holds the slots. [`Address`] / [`AddressLazy`] are
//! newtypes over [`puroro_rt::Record`].

use ::allocator_api2::alloc::{Allocator, Global};
use ::core::marker::PhantomData;
use ::core::mem;
use ::core::ops::{ControlFlow, DerefMut};
use ::puroro::{DecodeBuf, DecodeError, WireType};
use ::puroro_rt::decode::{MessageScan, ScannedRecord, scanned_len_span};
use ::puroro_rt::{
    CloneRecordFields, ConstructRecordFields, Eager, EagerRecordFields, Explicit, FieldCloneIn,
    FieldDeallocate, FieldPairVisitor, FieldVisitor, FieldVisitorMut, HasField, Lazy,
    LazyRecordApply, MessageCommon, ProtoDouble, ProtoFixed32, ProtoString, Record, RecordFields,
    SingularField, UnknownFields,
};

use crate::address::{
    self, BIT_CITY, BIT_LATITUDE, BIT_POSTAL_CODE, BIT_STREET, FIELD_CITY, FIELD_LATITUDE,
    FIELD_POSTAL_CODE, FIELD_STREET,
};

pub(crate) struct AddressFields<A: Allocator + Clone = Global, L: address::Layout<A> = Eager> {
    street: SingularField<ProtoString, Explicit<{ BIT_STREET }>, { FIELD_STREET }, A, L::StreetLen>,
    city: SingularField<ProtoString, Explicit<{ BIT_CITY }>, { FIELD_CITY }, A, L::CityLen>,
    postal_code:
        SingularField<ProtoFixed32, Explicit<{ BIT_POSTAL_CODE }>, { FIELD_POSTAL_CODE }, A>,
    latitude: SingularField<ProtoDouble, Explicit<{ BIT_LATITUDE }>, { FIELD_LATITUDE }, A>,
    _layout: PhantomData<(A, L)>,
}

macro_rules! has_address_field {
    ($n:expr, $slot:ident, $ty:ty) => {
        impl<A: Allocator + Clone, L: address::Layout<A>> HasField<{ $n }> for AddressFields<A, L> {
            type Ref<'a>
                = &'a $ty
            where
                Self: 'a;
            type Mut<'a>
                = &'a mut $ty
            where
                Self: 'a;

            fn field<'a>(&'a self) -> Self::Ref<'a> {
                &self.$slot
            }

            fn field_mut<'a>(&'a mut self) -> Self::Mut<'a> {
                &mut self.$slot
            }
        }
    };
}

has_address_field!(
    FIELD_STREET,
    street,
    SingularField<ProtoString, Explicit<{ BIT_STREET }>, { FIELD_STREET }, A, L::StreetLen>
);
has_address_field!(
    FIELD_CITY,
    city,
    SingularField<ProtoString, Explicit<{ BIT_CITY }>, { FIELD_CITY }, A, L::CityLen>
);
has_address_field!(
    FIELD_POSTAL_CODE,
    postal_code,
    SingularField<ProtoFixed32, Explicit<{ BIT_POSTAL_CODE }>, { FIELD_POSTAL_CODE }, A>
);
has_address_field!(
    FIELD_LATITUDE,
    latitude,
    SingularField<ProtoDouble, Explicit<{ BIT_LATITUDE }>, { FIELD_LATITUDE }, A>
);

pub struct Address<A: Allocator + Clone = Global> {
    pub(crate) record: Record<AddressFields<A, Eager>, A, Eager, UnknownFields<A>>,
}

pub struct AddressLazy<A: Allocator + Clone = Global> {
    pub(crate) record: Record<AddressFields<A, Lazy<A>>, A, Lazy<A>, UnknownFields<A>>,
}

::puroro_rt::forward_record!(eager Address);
::puroro_rt::forward_record!(lazy AddressLazy);

impl<A: Allocator + Clone, L: address::Layout<A> + MessageScan<A>> RecordFields<A, L>
    for AddressFields<A, L>
{
    type Bits = <L as address::Layout<A>>::Bits;
    type Unknown = UnknownFields<A>;
    type EagerMessage = Address<A>;

    const EAGER_NAME: &'static str = "Address";
    const LAZY_NAME: &'static str = "AddressLazy";

    fn empty_bits() -> Self::Bits {
        <L as address::Layout<A>>::empty_bits()
    }

    fn visit_mut<V: FieldVisitorMut<MessageCommon<Self::Bits, A, Self::Unknown, L>>>(
        &mut self,
        v: &mut V,
    ) -> ControlFlow<V::Break> {
        v.visit("street", &mut self.street)?;
        v.visit("city", &mut self.city)?;
        v.visit("postal_code", &mut self.postal_code)?;
        v.visit("latitude", &mut self.latitude)?;
        ControlFlow::Continue(())
    }
}

macro_rules! impl_address_build {
    ($layout:ty) => {
        impl<A: Allocator + Clone> ConstructRecordFields<A, $layout> for AddressFields<A, $layout> {
            fn new(alloc: A) -> Self {
                Self {
                    street: SingularField::new_in(alloc.clone()),
                    city: SingularField::new_in(alloc.clone()),
                    postal_code: SingularField::new_in(alloc.clone()),
                    latitude: SingularField::new_in(alloc),
                    _layout: PhantomData,
                }
            }
        }

        impl<A: Allocator + Clone> CloneRecordFields<A, $layout> for AddressFields<A, $layout> {
            fn clone_fields(
                &self,
                dst: &mut Self,
                src_common: &MessageCommon<Self::Bits, A, Self::Unknown, $layout>,
                dst_common: &MessageCommon<Self::Bits, A, Self::Unknown, $layout>,
                alloc: A,
            ) {
                clone_address_slot!(self, dst, src_common, dst_common, alloc, street);
                clone_address_slot!(self, dst, src_common, dst_common, alloc, city);
                clone_address_slot!(self, dst, src_common, dst_common, alloc, postal_code);
                clone_address_slot!(self, dst, src_common, dst_common, alloc, latitude);
            }
        }
    };
}

macro_rules! clone_address_slot {
    ($self:ident, $dst:ident, $src_common:ident, $dst_common:ident, $alloc:ident, $name:ident) => {{
        let new = FieldCloneIn::clone_field(&$self.$name, $src_common, $alloc.clone());
        let mut old = mem::replace(&mut $dst.$name, new);
        FieldDeallocate::deallocate(&mut old, $dst_common);
    }};
}

impl_address_build!(Eager);
impl_address_build!(Lazy<A>);

impl<A: Allocator + Clone> EagerRecordFields<A> for AddressFields<A, Eager> {
    fn visit_fields<V: FieldVisitor<MessageCommon<Self::Bits, A, Self::Unknown, Eager>>>(
        &self,
        v: &mut V,
    ) -> ControlFlow<V::Break> {
        v.visit("street", &self.street)?;
        v.visit("city", &self.city)?;
        v.visit("postal_code", &self.postal_code)?;
        v.visit("latitude", &self.latitude)?;
        ControlFlow::Continue(())
    }

    fn visit_pairs<V: FieldPairVisitor<MessageCommon<Self::Bits, A, Self::Unknown, Eager>>>(
        &self,
        other: &Self,
        v: &mut V,
    ) -> ControlFlow<V::Break> {
        v.visit("street", &self.street, &other.street)?;
        v.visit("city", &self.city, &other.city)?;
        v.visit("postal_code", &self.postal_code, &other.postal_code)?;
        v.visit("latitude", &self.latitude, &other.latitude)?;
        ControlFlow::Continue(())
    }

    fn merge_known<B: DecodeBuf>(
        &mut self,
        common: &mut MessageCommon<Self::Bits, A, Self::Unknown, Eager>,
        field_number: u32,
        wire_type: WireType,
        buf: &mut B,
        depth: usize,
    ) -> Result<bool, DecodeError> {
        match field_number {
            FIELD_STREET => {
                <Self as HasField<{ FIELD_STREET }>>::field_mut(self)
                    .bind_mut(common)
                    .merge(wire_type, buf, depth)?;
                Ok(true)
            }
            FIELD_CITY => {
                <Self as HasField<{ FIELD_CITY }>>::field_mut(self)
                    .bind_mut(common)
                    .merge(wire_type, buf, depth)?;
                Ok(true)
            }
            FIELD_POSTAL_CODE => {
                <Self as HasField<{ FIELD_POSTAL_CODE }>>::field_mut(self)
                    .bind_mut(common)
                    .merge(wire_type, buf, depth)?;
                Ok(true)
            }
            FIELD_LATITUDE => {
                <Self as HasField<{ FIELD_LATITUDE }>>::field_mut(self)
                    .bind_mut(common)
                    .merge(wire_type, buf, depth)?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

impl<A: Allocator + Clone> LazyRecordApply<A> for AddressFields<A, Lazy<A>> {
    fn apply_record(
        &mut self,
        common: &mut MessageCommon<Self::Bits, A, Self::Unknown, Lazy<A>>,
        rec: &ScannedRecord,
        origin: usize,
    ) -> Result<(), DecodeError> {
        match rec.field_number.as_u32() {
            FIELD_STREET => {
                let span = scanned_len_span(origin, rec)?;
                <Self as HasField<{ FIELD_STREET }>>::field_mut(self).store_len_span(span, common);
                Ok(())
            }
            FIELD_CITY => {
                let span = scanned_len_span(origin, rec)?;
                <Self as HasField<{ FIELD_CITY }>>::field_mut(self).store_len_span(span, common);
                Ok(())
            }
            FIELD_POSTAL_CODE => <Self as HasField<{ FIELD_POSTAL_CODE }>>::field_mut(self)
                .bind_mut(common)
                .merge_scanned(&rec.value),
            FIELD_LATITUDE => <Self as HasField<{ FIELD_LATITUDE }>>::field_mut(self)
                .bind_mut(common)
                .merge_scanned(&rec.value),
            _ => common.on_unrecognized_tag(rec.field_number.as_u32()),
        }
    }
}

impl<A: Allocator + Clone> Address<A> {
    pub fn street<'a>(&'a self) -> ::puroro::Optional<&'a str, impl ::puroro::HasDefault<&'a str>>
    where
        A: 'a,
    {
        self.record.field::<{ FIELD_STREET }>().optional()
    }

    pub fn city<'a>(&'a self) -> ::puroro::Optional<&'a str, impl ::puroro::HasDefault<&'a str>>
    where
        A: 'a,
    {
        self.record.field::<{ FIELD_CITY }>().optional()
    }

    pub fn postal_code<'a>(&'a self) -> ::puroro::Optional<u32, impl ::puroro::HasDefault<u32>>
    where
        A: 'a,
    {
        self.record.field::<{ FIELD_POSTAL_CODE }>().optional()
    }

    pub fn latitude<'a>(&'a self) -> ::puroro::Optional<f64, impl ::puroro::HasDefault<f64>>
    where
        A: 'a,
    {
        self.record.field::<{ FIELD_LATITUDE }>().optional()
    }

    pub fn street_mut<'s>(&'s mut self) -> impl ::puroro::StringMut<A> + 's {
        self.record.field_mut::<{ FIELD_STREET }>().value_mut()
    }

    pub fn clear_street(&mut self) {
        self.record.field_mut::<{ FIELD_STREET }>().clear();
    }

    pub fn city_mut<'s>(&'s mut self) -> impl ::puroro::StringMut<A> + 's {
        self.record.field_mut::<{ FIELD_CITY }>().value_mut()
    }

    pub fn clear_city(&mut self) {
        self.record.field_mut::<{ FIELD_CITY }>().clear();
    }

    pub fn postal_code_mut<'s>(&'s mut self) -> impl DerefMut<Target = u32> + 's {
        self.record.field_mut::<{ FIELD_POSTAL_CODE }>().value_mut()
    }

    pub fn clear_postal_code(&mut self) {
        self.record.field_mut::<{ FIELD_POSTAL_CODE }>().clear();
    }

    pub fn latitude_mut<'s>(&'s mut self) -> impl DerefMut<Target = f64> + 's {
        self.record.field_mut::<{ FIELD_LATITUDE }>().value_mut()
    }

    pub fn clear_latitude(&mut self) {
        self.record.field_mut::<{ FIELD_LATITUDE }>().clear();
    }

    /// Encodes `self` and rebuilds it as the lazy alias.
    pub fn into_lazy(self) -> AddressLazy<A> {
        AddressLazy {
            record: Record::encode_lazy(self.record),
        }
    }
}
