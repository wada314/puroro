//! Read-oriented `Address`. Numericals apply during scan; strings use WireOrSso.

use ::allocator_api2::alloc::{Allocator, Global};
use ::bytes::{Buf, BufMut};
use ::core::fmt::{self, Debug, Formatter};
use ::core::iter;
use ::core::ops::ControlFlow;
use ::puroro::{DecodeBuf, DecodeError, HasDefault, Message, Optional, RECURSION_LIMIT};
use ::puroro_rt::decode::{
    LazyScan, ScannedRecord, SharedWire, WireSpan, merge_scanned_field, scanned_len_span,
};
use ::puroro_rt::{
    CloneIn, DeallocateIn, DefaultIn, EncodeCtx, Explicit, FieldCloneIn, FieldDeallocVisitor,
    FieldVisitorMut, InteriorBitArray, LazyStringSlot, MessageCommon, MessageEncode, MessageMerge,
    ProtoDefault, ProtoDouble, ProtoFixed32, SingularField, WireOrSsoArm,
};
use ::std::vec::Vec;

use crate::Address;
use crate::address::{
    BIT_CITY, BIT_CITY_ARM1, BIT_CITY_SSO, BIT_LATITUDE, BIT_POSTAL_CODE, BIT_STREET,
    BIT_STREET_ARM1, BIT_STREET_SSO, FIELD_CITY, FIELD_LATITUDE, FIELD_POSTAL_CODE, FIELD_STREET,
};

/// Lazy `Address` with catalog numericals and `WireOrSso` strings.
pub struct AddressLazy<A: Allocator = Global> {
    scan: LazyScan<A>,
    _common: MessageCommon<InteriorBitArray<1>, A>,
    street: LazyStringSlot<A>,
    city: LazyStringSlot<A>,
    postal_code:
        SingularField<ProtoFixed32, Explicit<{ BIT_POSTAL_CODE }>, { FIELD_POSTAL_CODE }, A>,
    latitude: SingularField<ProtoDouble, Explicit<{ BIT_LATITUDE }>, { FIELD_LATITUDE }, A>,
}

impl<A: Allocator + Clone + Default> Default for AddressLazy<A> {
    fn default() -> Self {
        Self::new_in(A::default())
    }
}

impl AddressLazy<Global> {
    pub fn new() -> Self {
        Self::new_in(Global)
    }

    pub fn decode<B: Buf>(mut buf: B) -> Result<Self, DecodeError> {
        let mut msg = Self::new();
        msg.merge_from(&mut buf)?;
        Ok(msg)
    }
}

impl<A: Allocator> AddressLazy<A> {
    pub fn street(&self) -> Result<Optional<&str, impl HasDefault<&str>>, DecodeError>
    where
        A: Clone,
    {
        self.scan.require_finished()?;
        if !self._common.is_bit_set(BIT_STREET) {
            return Ok(Optional::<&str, ProtoDefault>::new(None));
        }
        let s = self.street.get_str(
            self.arm(BIT_STREET_SSO, BIT_STREET_ARM1),
            self.scan.wire(),
            self._common.alloc.clone(),
            |arm| self.set_arm(BIT_STREET_SSO, BIT_STREET_ARM1, arm),
        )?;
        Ok(Optional::new(Some(s)))
    }

    pub fn has_street(&self) -> Result<bool, DecodeError> {
        self.scan.require_finished()?;
        Ok(self._common.is_bit_set(BIT_STREET))
    }

    pub fn city(&self) -> Result<Optional<&str, impl HasDefault<&str>>, DecodeError>
    where
        A: Clone,
    {
        self.scan.require_finished()?;
        if !self._common.is_bit_set(BIT_CITY) {
            return Ok(Optional::<&str, ProtoDefault>::new(None));
        }
        let s = self.city.get_str(
            self.arm(BIT_CITY_SSO, BIT_CITY_ARM1),
            self.scan.wire(),
            self._common.alloc.clone(),
            |arm| self.set_arm(BIT_CITY_SSO, BIT_CITY_ARM1, arm),
        )?;
        Ok(Optional::new(Some(s)))
    }

    pub fn has_city(&self) -> Result<bool, DecodeError> {
        self.scan.require_finished()?;
        Ok(self._common.is_bit_set(BIT_CITY))
    }

    pub fn postal_code(&self) -> Result<Optional<u32, impl HasDefault<u32>>, DecodeError> {
        self.scan.require_finished()?;
        Ok(self.postal_code.bind(&self._common).optional())
    }

    pub fn latitude(&self) -> Result<Optional<f64, impl HasDefault<f64>>, DecodeError> {
        self.scan.require_finished()?;
        Ok(self.latitude.bind(&self._common).optional())
    }

    pub fn encoded_len(&self) -> Result<usize, DecodeError> {
        self.scan.encoded_len()
    }

    pub fn encode<B: BufMut>(&self, buf: &mut B) -> Result<(), DecodeError> {
        self.scan.encode(buf)
    }

    pub fn encode_to_vec(&self) -> Result<Vec<u8>, DecodeError> {
        let mut out = Vec::with_capacity(self.encoded_len()?);
        self.encode(&mut out)?;
        Ok(out)
    }

    fn arm(&self, arm0: usize, arm1: usize) -> WireOrSsoArm {
        WireOrSsoArm::from_bits(
            self._common.bits.is_set(arm0),
            self._common.bits.is_set(arm1),
        )
    }

    fn set_arm(&self, arm0: usize, arm1: usize, arm: WireOrSsoArm) {
        let (b0, b1) = arm.bits();
        self._common.bits.set_shared(arm0, b0);
        self._common.bits.set_shared(arm1, b1);
    }

    fn visit_fields_mut<V: FieldVisitorMut<MessageCommon<InteriorBitArray<1>, A>>>(
        &mut self,
        v: &mut V,
    ) -> ControlFlow<V::Break> {
        v.visit("postal_code", &mut self.postal_code)?;
        v.visit("latitude", &mut self.latitude)?;
        ControlFlow::Continue(())
    }
}

impl<A: Allocator + Clone> AddressLazy<A> {
    pub fn new_in(alloc: A) -> Self {
        Self {
            scan: LazyScan::new_in(alloc.clone()),
            _common: MessageCommon::new_in(InteriorBitArray::zero(), alloc.clone()),
            street: LazyStringSlot::empty(),
            city: LazyStringSlot::empty(),
            postal_code: SingularField::new_in(alloc.clone()),
            latitude: SingularField::new_in(alloc),
        }
    }

    pub fn push(&mut self, chunk: &[u8]) -> Result<(), DecodeError> {
        let (origin, records) = self.scan.push(chunk)?;
        for rec in records {
            self.apply_record(&rec, origin)?;
        }
        Ok(())
    }

    pub fn finish(&mut self) -> Result<(), DecodeError> {
        self.scan.finish()
    }

    pub fn merge_from<B: Buf>(&mut self, buf: &mut B) -> Result<(), DecodeError> {
        self.ingest_buf(buf)
    }

    fn ingest_buf<B: Buf>(&mut self, buf: &mut B) -> Result<(), DecodeError> {
        while buf.has_remaining() {
            let n = buf.chunk().len();
            if n == 0 {
                let rest = buf.copy_to_bytes(buf.remaining());
                self.push(&rest)?;
                break;
            }
            self.push(buf.chunk())?;
            buf.advance(n);
        }
        self.finish()
    }

    /// Decode every field from `_wire` into an eager [`Address`].
    pub fn into_eager(self) -> Result<Address<A>, DecodeError> {
        self.scan.require_finished()?;
        let mut eager = Address::new_in(self._common.alloc.clone());
        self.scan.for_each_body(|chunk| {
            let mut buf = chunk;
            Message::merge_from(&mut eager, &mut buf)
        })?;
        Ok(eager)
    }

    /// Merge one complete Address body that already lives in `root`.
    pub(crate) fn merge_shared(
        &mut self,
        root: &SharedWire<A>,
        span: WireSpan,
    ) -> Result<(), DecodeError> {
        self.scan.adopt(root);
        self.scan.record_region(span);
        let payload = span.slice(root.as_bytes())?;
        let records = self.scan.scan_complete(payload)?;
        for rec in records {
            self.apply_record(&rec, span.offset)?;
        }
        Ok(())
    }

    fn apply_record(&mut self, rec: &ScannedRecord<A>, origin: usize) -> Result<(), DecodeError> {
        match rec.field_number.as_u32() {
            FIELD_STREET => {
                let span = scanned_len_span(origin, rec)?;
                let old = self.arm(BIT_STREET_SSO, BIT_STREET_ARM1);
                self.street.store_wire(span, old, &self._common.alloc);
                self._common.set_bit(BIT_STREET, true);
                self.set_arm(BIT_STREET_SSO, BIT_STREET_ARM1, WireOrSsoArm::Wire);
                Ok(())
            }
            FIELD_CITY => {
                let span = scanned_len_span(origin, rec)?;
                let old = self.arm(BIT_CITY_SSO, BIT_CITY_ARM1);
                self.city.store_wire(span, old, &self._common.alloc);
                self._common.set_bit(BIT_CITY, true);
                self.set_arm(BIT_CITY_SSO, BIT_CITY_ARM1, WireOrSsoArm::Wire);
                Ok(())
            }
            FIELD_POSTAL_CODE => merge_scanned_field(&rec.field, |wire_type, buf| {
                self.postal_code
                    .bind_mut(&mut self._common)
                    .merge(wire_type, buf, 0)
            }),
            FIELD_LATITUDE => merge_scanned_field(&rec.field, |wire_type, buf| {
                self.latitude
                    .bind_mut(&mut self._common)
                    .merge(wire_type, buf, 0)
            }),
            _ => Ok(()),
        }
    }
}

impl<A: Allocator + Clone> CloneIn<A> for AddressLazy<A> {
    fn clone_in(&self, alloc: A) -> Self {
        let street_arm = self.arm(BIT_STREET_SSO, BIT_STREET_ARM1);
        let city_arm = self.arm(BIT_CITY_SSO, BIT_CITY_ARM1);
        Self {
            scan: self.scan.clone_in(alloc.clone()),
            _common: self._common.clone_in(alloc.clone()),
            street: self.street.clone_in(street_arm, alloc.clone()),
            city: self.city.clone_in(city_arm, alloc.clone()),
            postal_code: self.postal_code.clone_field(&self._common, alloc.clone()),
            latitude: self.latitude.clone_field(&self._common, alloc),
        }
    }
}

impl<A: Allocator + Clone> Clone for AddressLazy<A> {
    fn clone(&self) -> Self {
        self.clone_in(self._common.alloc.clone())
    }
}

impl<A: Allocator> PartialEq for AddressLazy<A> {
    fn eq(&self, other: &Self) -> bool {
        let mut left = Vec::new();
        let mut right = Vec::new();
        if self.scan.write_bodies(&mut left).is_err() {
            return false;
        }
        if other.scan.write_bodies(&mut right).is_err() {
            return false;
        }
        left == right
    }
}

impl<A: Allocator> Debug for AddressLazy<A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("AddressLazy").finish_non_exhaustive()
    }
}

impl<A: Allocator> DeallocateIn<A> for AddressLazy<A> {
    #[inline]
    unsafe fn deallocate_in(self, _alloc: &A) {
        drop(self);
    }
}

::puroro_rt::impl_owned_slot_bounds!(AddressLazy);

impl<A: Allocator> MessageEncode for AddressLazy<A> {
    fn encoded_len(&self, _ctx: &mut EncodeCtx) -> usize {
        self.scan.body_len()
    }

    fn encode_raw<B: BufMut>(&self, _ctx: &mut EncodeCtx, buf: &mut B) {
        let _ = self.scan.write_bodies(buf);
    }
}

impl<A: Allocator + Clone> MessageMerge for AddressLazy<A> {
    fn merge_from_with_depth<B: DecodeBuf>(
        &mut self,
        buf: &mut B,
        depth: usize,
    ) -> Result<(), DecodeError> {
        if depth >= RECURSION_LIMIT {
            return Err(DecodeError::RecursionLimitExceeded);
        }
        self.ingest_buf(buf)
    }
}

impl<A: Allocator + Clone> DefaultIn<A> for AddressLazy<A> {
    #[inline]
    fn default_in(alloc: A) -> Self {
        Self::new_in(alloc)
    }
}

impl<A: Allocator> Message for AddressLazy<A> {
    type Alloc = A;

    fn new_in(alloc: A) -> Self
    where
        A: Clone,
    {
        Self::new_in(alloc)
    }

    fn encode<B: BufMut>(&self, buf: &mut B) {
        ::puroro_rt::encode_message(self, buf)
    }

    fn encode_to_vec(&self) -> Vec<u8> {
        ::puroro_rt::encode_message_to_vec(self)
    }

    fn merge_from<B: Buf>(&mut self, buf: &mut B) -> Result<(), DecodeError>
    where
        A: Clone,
    {
        self.ingest_buf(buf)
    }

    fn unknown_fields(&self) -> impl Iterator<Item = ::puroro::UnknownField<'_>> + '_ {
        iter::empty()
    }

    fn validate(&self) -> Result<(), DecodeError> {
        Ok(())
    }
}

impl<A: Allocator> Drop for AddressLazy<A> {
    fn drop(&mut self) {
        self.street.deallocate(
            self.arm(BIT_STREET_SSO, BIT_STREET_ARM1),
            &self._common.alloc,
        );
        self.city
            .deallocate(self.arm(BIT_CITY_SSO, BIT_CITY_ARM1), &self._common.alloc);
        let mut v = FieldDeallocVisitor::new(&self._common);
        let _ = self.visit_fields_mut(&mut v);
        self._common.deallocate();
    }
}
