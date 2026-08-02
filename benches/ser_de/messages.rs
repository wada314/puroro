//! Minimal `Message` types for ser/de microbenchmarks.

use ::allocator_api2::alloc::{Allocator, Global};
use ::bitvec::array::BitArray;
use ::bitvec::order::Lsb0;
use ::bytes::{Buf, BufMut};
use ::puroro::{DecodeBuf, DecodeError, Message};
use ::puroro_rt::decode::{decode_tag, skip_field_and_save};
use ::puroro_rt::{
    EncodeCtx, Explicit, FieldDeallocate, FieldEncode, Implicit, Message as MessagePresence,
    MessageCommon, MessageEncode, MessageMerge, Packed, ProtoBytes, ProtoInt32, ProtoMessage,
    ProtoString, ProtoUInt64, RepeatedField, SingularField,
};

macro_rules! drop_fields {
    ($self:ident, $($field:ident),+ $(,)?) => {{
        $($self.$field.deallocate(&$self._common);)+
        $self._common.deallocate();
    }};
}

// ---------------------------------------------------------------------------
// Flat scalars — many tags / varints
// ---------------------------------------------------------------------------

/// Eight implicit integer fields (varint-heavy).
pub struct FlatScalars<A: Allocator + Clone = Global> {
    _common: MessageCommon<BitArray<[u8; 1], Lsb0>, A>,
    a: SingularField<ProtoInt32, Implicit, 1, A>,
    b: SingularField<ProtoInt32, Implicit, 2, A>,
    c: SingularField<ProtoInt32, Implicit, 3, A>,
    d: SingularField<ProtoInt32, Implicit, 4, A>,
    e: SingularField<ProtoUInt64, Implicit, 5, A>,
    f: SingularField<ProtoUInt64, Implicit, 6, A>,
    g: SingularField<ProtoInt32, Implicit, 7, A>,
    h: SingularField<ProtoInt32, Implicit, 8, A>,
}

impl FlatScalars<Global> {
    pub fn new() -> Self {
        Self::new_in(Global)
    }

    /// Fills every field with non-default values.
    pub fn sample() -> Self {
        let mut m = Self::new();
        *m.a.bind_mut(&mut m._common).value_mut() = 1;
        *m.b.bind_mut(&mut m._common).value_mut() = 22;
        *m.c.bind_mut(&mut m._common).value_mut() = 333;
        *m.d.bind_mut(&mut m._common).value_mut() = 4444;
        *m.e.bind_mut(&mut m._common).value_mut() = 1 << 20;
        *m.f.bind_mut(&mut m._common).value_mut() = u64::from(u32::MAX) + 99;
        *m.g.bind_mut(&mut m._common).value_mut() = -7;
        *m.h.bind_mut(&mut m._common).value_mut() = 42;
        m
    }
}

impl Default for FlatScalars<Global> {
    fn default() -> Self {
        Self::new()
    }
}

impl<A: Allocator + Clone> FlatScalars<A> {
    pub fn new_in(alloc: A) -> Self {
        Self {
            _common: MessageCommon::new_in(BitArray::ZERO, alloc.clone()),
            a: SingularField::new_in(alloc.clone()),
            b: SingularField::new_in(alloc.clone()),
            c: SingularField::new_in(alloc.clone()),
            d: SingularField::new_in(alloc.clone()),
            e: SingularField::new_in(alloc.clone()),
            f: SingularField::new_in(alloc.clone()),
            g: SingularField::new_in(alloc.clone()),
            h: SingularField::new_in(alloc),
        }
    }
}

impl<A: Allocator + Clone> Drop for FlatScalars<A> {
    fn drop(&mut self) {
        drop_fields!(self, a, b, c, d, e, f, g, h);
    }
}

impl<A: Allocator + Clone> ::unmanaged::DeallocateIn<A> for FlatScalars<A> {
    #[inline]
    unsafe fn deallocate_in(self, _alloc: A) {
        drop(self);
    }
}

impl<A: Allocator + Clone> MessageEncode for FlatScalars<A> {
    fn encoded_len(&self, ctx: &mut EncodeCtx) -> usize {
        let c = &self._common;
        self.a.encoded_len(c, ctx)
            + self.b.encoded_len(c, ctx)
            + self.c.encoded_len(c, ctx)
            + self.d.encoded_len(c, ctx)
            + self.e.encoded_len(c, ctx)
            + self.f.encoded_len(c, ctx)
            + self.g.encoded_len(c, ctx)
            + self.h.encoded_len(c, ctx)
            + c.unknown_fields.len()
    }

    fn encode_raw<B: BufMut>(&self, ctx: &mut EncodeCtx, buf: &mut B) {
        let c = &self._common;
        self.a.encode_raw(c, ctx, buf);
        self.b.encode_raw(c, ctx, buf);
        self.c.encode_raw(c, ctx, buf);
        self.d.encode_raw(c, ctx, buf);
        self.e.encode_raw(c, ctx, buf);
        self.f.encode_raw(c, ctx, buf);
        self.g.encode_raw(c, ctx, buf);
        self.h.encode_raw(c, ctx, buf);
        buf.put_slice(&c.unknown_fields);
    }
}

impl<A: Allocator + Clone> MessageMerge for FlatScalars<A> {
    fn merge_from_with_depth<B: DecodeBuf>(
        &mut self,
        buf: &mut B,
        depth: usize,
    ) -> Result<(), DecodeError> {
        if depth >= ::puroro::RECURSION_LIMIT {
            return Err(DecodeError::RecursionLimitExceeded);
        }
        while buf.has_remaining() {
            let (field_number, wire_type) = decode_tag(buf)?;
            match field_number {
                1 => self
                    .a
                    .bind_mut(&mut self._common)
                    .merge(wire_type, buf, depth)?,
                2 => self
                    .b
                    .bind_mut(&mut self._common)
                    .merge(wire_type, buf, depth)?,
                3 => self
                    .c
                    .bind_mut(&mut self._common)
                    .merge(wire_type, buf, depth)?,
                4 => self
                    .d
                    .bind_mut(&mut self._common)
                    .merge(wire_type, buf, depth)?,
                5 => self
                    .e
                    .bind_mut(&mut self._common)
                    .merge(wire_type, buf, depth)?,
                6 => self
                    .f
                    .bind_mut(&mut self._common)
                    .merge(wire_type, buf, depth)?,
                7 => self
                    .g
                    .bind_mut(&mut self._common)
                    .merge(wire_type, buf, depth)?,
                8 => self
                    .h
                    .bind_mut(&mut self._common)
                    .merge(wire_type, buf, depth)?,
                _ => skip_field_and_save(
                    field_number,
                    wire_type,
                    buf,
                    &mut self._common.unknown_fields,
                    self._common.alloc.clone(),
                )?,
            }
        }
        Ok(())
    }
}

impl<A: Allocator + Clone> Message for FlatScalars<A> {
    type Alloc = A;

    fn new_in(alloc: A) -> Self {
        Self::new_in(alloc)
    }

    fn encode<B: BufMut>(&self, buf: &mut B) {
        ::puroro_rt::encode_message(self, buf)
    }

    fn encode_to_vec(&self) -> Vec<u8> {
        ::puroro_rt::encode_message_to_vec(self)
    }

    fn merge_from<B: Buf>(&mut self, buf: &mut B) -> Result<(), DecodeError> {
        ::puroro_rt::merge_message(self, buf)
    }

    fn unknown_fields(&self) -> impl Iterator<Item = ::puroro::UnknownField<'_>> + '_ {
        self._common.iter_unknown_fields()
    }

    fn validate(&self) -> Result<(), DecodeError> {
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Nested messages — ScopedBuf path
// ---------------------------------------------------------------------------

/// Self-referential optional child (field 1) plus a leaf `value`.
pub struct Nest<A: Allocator + Clone = Global> {
    _common: MessageCommon<BitArray<[u8; 1], Lsb0>, A>,
    child: SingularField<ProtoMessage<Nest<A>>, MessagePresence, 1, A>,
    value: SingularField<ProtoInt32, Implicit, 2, A>,
}

impl Nest<Global> {
    pub fn new() -> Self {
        Self::new_in(Global)
    }

    /// Chain of `depth` nested children; each level sets `value = depth`.
    pub fn sample(depth: usize) -> Self {
        let mut root = Self::new();
        *root.value.bind_mut(&mut root._common).value_mut() = depth as i32;
        if depth > 0 {
            *root.child.bind_mut(&mut root._common).get_mut() = Self::sample(depth - 1);
        }
        root
    }
}

impl Default for Nest<Global> {
    fn default() -> Self {
        Self::new()
    }
}

impl<A: Allocator + Clone> Nest<A> {
    pub fn new_in(alloc: A) -> Self {
        Self {
            _common: MessageCommon::new_in(BitArray::ZERO, alloc.clone()),
            child: SingularField::new_in(alloc.clone()),
            value: SingularField::new_in(alloc),
        }
    }
}

impl<A: Allocator + Clone> Drop for Nest<A> {
    fn drop(&mut self) {
        drop_fields!(self, child, value);
    }
}

impl<A: Allocator + Clone> ::unmanaged::DeallocateIn<A> for Nest<A> {
    #[inline]
    unsafe fn deallocate_in(self, _alloc: A) {
        drop(self);
    }
}

impl<A: Allocator + Clone> MessageEncode for Nest<A> {
    fn encoded_len(&self, ctx: &mut EncodeCtx) -> usize {
        let c = &self._common;
        self.child.encoded_len(c, ctx) + self.value.encoded_len(c, ctx) + c.unknown_fields.len()
    }

    fn encode_raw<B: BufMut>(&self, ctx: &mut EncodeCtx, buf: &mut B) {
        let c = &self._common;
        self.child.encode_raw(c, ctx, buf);
        self.value.encode_raw(c, ctx, buf);
        buf.put_slice(&c.unknown_fields);
    }
}

impl<A: Allocator + Clone> MessageMerge for Nest<A> {
    fn merge_from_with_depth<B: DecodeBuf>(
        &mut self,
        buf: &mut B,
        depth: usize,
    ) -> Result<(), DecodeError> {
        if depth >= ::puroro::RECURSION_LIMIT {
            return Err(DecodeError::RecursionLimitExceeded);
        }
        while buf.has_remaining() {
            let (field_number, wire_type) = decode_tag(buf)?;
            match field_number {
                1 => self
                    .child
                    .bind_mut(&mut self._common)
                    .merge(wire_type, buf, depth)?,
                2 => self
                    .value
                    .bind_mut(&mut self._common)
                    .merge(wire_type, buf, depth)?,
                _ => skip_field_and_save(
                    field_number,
                    wire_type,
                    buf,
                    &mut self._common.unknown_fields,
                    self._common.alloc.clone(),
                )?,
            }
        }
        Ok(())
    }
}

impl<A: Allocator + Clone> Message for Nest<A> {
    type Alloc = A;

    fn new_in(alloc: A) -> Self {
        Self::new_in(alloc)
    }

    fn encode<B: BufMut>(&self, buf: &mut B) {
        ::puroro_rt::encode_message(self, buf)
    }

    fn encode_to_vec(&self) -> Vec<u8> {
        ::puroro_rt::encode_message_to_vec(self)
    }

    fn merge_from<B: Buf>(&mut self, buf: &mut B) -> Result<(), DecodeError> {
        ::puroro_rt::merge_message(self, buf)
    }

    fn unknown_fields(&self) -> impl Iterator<Item = ::puroro::UnknownField<'_>> + '_ {
        self._common.iter_unknown_fields()
    }

    fn validate(&self) -> Result<(), DecodeError> {
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Packed repeated — large packed varint payload
// ---------------------------------------------------------------------------

pub struct PackedInts<A: Allocator + Clone = Global> {
    _common: MessageCommon<BitArray<[u8; 1], Lsb0>, A>,
    values: RepeatedField<ProtoInt32, Packed, 1, A>,
}

impl PackedInts<Global> {
    pub fn new() -> Self {
        Self::new_in(Global)
    }

    pub fn sample(n: usize) -> Self {
        let mut m = Self::new();
        {
            let mut g = m.values.bind_mut(&mut m._common).values_mut();
            for i in 0..n {
                g.push(i as i32);
            }
        }
        m
    }
}

impl Default for PackedInts<Global> {
    fn default() -> Self {
        Self::new()
    }
}

impl<A: Allocator + Clone> PackedInts<A> {
    pub fn new_in(alloc: A) -> Self {
        Self {
            _common: MessageCommon::new_in(BitArray::ZERO, alloc.clone()),
            values: RepeatedField::new_in(alloc),
        }
    }
}

impl<A: Allocator + Clone> Drop for PackedInts<A> {
    fn drop(&mut self) {
        drop_fields!(self, values);
    }
}

impl<A: Allocator + Clone> ::unmanaged::DeallocateIn<A> for PackedInts<A> {
    #[inline]
    unsafe fn deallocate_in(self, _alloc: A) {
        drop(self);
    }
}

impl<A: Allocator + Clone> MessageEncode for PackedInts<A> {
    fn encoded_len(&self, ctx: &mut EncodeCtx) -> usize {
        let c = &self._common;
        self.values.encoded_len(c, ctx) + c.unknown_fields.len()
    }

    fn encode_raw<B: BufMut>(&self, ctx: &mut EncodeCtx, buf: &mut B) {
        let c = &self._common;
        self.values.encode_raw(c, ctx, buf);
        buf.put_slice(&c.unknown_fields);
    }
}

impl<A: Allocator + Clone> MessageMerge for PackedInts<A> {
    fn merge_from_with_depth<B: DecodeBuf>(
        &mut self,
        buf: &mut B,
        depth: usize,
    ) -> Result<(), DecodeError> {
        if depth >= ::puroro::RECURSION_LIMIT {
            return Err(DecodeError::RecursionLimitExceeded);
        }
        while buf.has_remaining() {
            let (field_number, wire_type) = decode_tag(buf)?;
            match field_number {
                1 => self
                    .values
                    .bind_mut(&mut self._common)
                    .merge(wire_type, buf, depth)?,
                _ => skip_field_and_save(
                    field_number,
                    wire_type,
                    buf,
                    &mut self._common.unknown_fields,
                    self._common.alloc.clone(),
                )?,
            }
        }
        Ok(())
    }
}

impl<A: Allocator + Clone> Message for PackedInts<A> {
    type Alloc = A;

    fn new_in(alloc: A) -> Self {
        Self::new_in(alloc)
    }

    fn encode<B: BufMut>(&self, buf: &mut B) {
        ::puroro_rt::encode_message(self, buf)
    }

    fn encode_to_vec(&self) -> Vec<u8> {
        ::puroro_rt::encode_message_to_vec(self)
    }

    fn merge_from<B: Buf>(&mut self, buf: &mut B) -> Result<(), DecodeError> {
        ::puroro_rt::merge_message(self, buf)
    }

    fn unknown_fields(&self) -> impl Iterator<Item = ::puroro::UnknownField<'_>> + '_ {
        self._common.iter_unknown_fields()
    }

    fn validate(&self) -> Result<(), DecodeError> {
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Strings / bytes — LEN copy + UTF-8
// ---------------------------------------------------------------------------

pub struct StringHeavy<A: Allocator + Clone = Global> {
    _common: MessageCommon<BitArray<[u8; 1], Lsb0>, A>,
    s0: SingularField<ProtoString, Explicit<0>, 1, A>,
    s1: SingularField<ProtoString, Explicit<1>, 2, A>,
    s2: SingularField<ProtoString, Explicit<2>, 3, A>,
    s3: SingularField<ProtoString, Explicit<3>, 4, A>,
    blob: SingularField<ProtoBytes, Explicit<4>, 5, A>,
}

impl StringHeavy<Global> {
    pub fn new() -> Self {
        Self::new_in(Global)
    }

    pub fn sample() -> Self {
        let mut m = Self::new();
        m.s0.bind_mut(&mut m._common)
            .value_mut()
            .push_str("alpha-benchmark-string-000");
        m.s1.bind_mut(&mut m._common)
            .value_mut()
            .push_str("bravo-benchmark-string-111");
        m.s2.bind_mut(&mut m._common)
            .value_mut()
            .push_str("charlie-benchmark-string-222");
        m.s3.bind_mut(&mut m._common)
            .value_mut()
            .push_str("delta-benchmark-string-333");
        m.blob
            .bind_mut(&mut m._common)
            .value_mut()
            .extend_from_slice(&[0u8; 64]);
        m
    }
}

impl Default for StringHeavy<Global> {
    fn default() -> Self {
        Self::new()
    }
}

impl<A: Allocator + Clone> StringHeavy<A> {
    pub fn new_in(alloc: A) -> Self {
        Self {
            _common: MessageCommon::new_in(BitArray::ZERO, alloc.clone()),
            s0: SingularField::new_in(alloc.clone()),
            s1: SingularField::new_in(alloc.clone()),
            s2: SingularField::new_in(alloc.clone()),
            s3: SingularField::new_in(alloc.clone()),
            blob: SingularField::new_in(alloc),
        }
    }
}

impl<A: Allocator + Clone> Drop for StringHeavy<A> {
    fn drop(&mut self) {
        drop_fields!(self, s0, s1, s2, s3, blob);
    }
}

impl<A: Allocator + Clone> ::unmanaged::DeallocateIn<A> for StringHeavy<A> {
    #[inline]
    unsafe fn deallocate_in(self, _alloc: A) {
        drop(self);
    }
}

impl<A: Allocator + Clone> MessageEncode for StringHeavy<A> {
    fn encoded_len(&self, ctx: &mut EncodeCtx) -> usize {
        let c = &self._common;
        self.s0.encoded_len(c, ctx)
            + self.s1.encoded_len(c, ctx)
            + self.s2.encoded_len(c, ctx)
            + self.s3.encoded_len(c, ctx)
            + self.blob.encoded_len(c, ctx)
            + c.unknown_fields.len()
    }

    fn encode_raw<B: BufMut>(&self, ctx: &mut EncodeCtx, buf: &mut B) {
        let c = &self._common;
        self.s0.encode_raw(c, ctx, buf);
        self.s1.encode_raw(c, ctx, buf);
        self.s2.encode_raw(c, ctx, buf);
        self.s3.encode_raw(c, ctx, buf);
        self.blob.encode_raw(c, ctx, buf);
        buf.put_slice(&c.unknown_fields);
    }
}

impl<A: Allocator + Clone> MessageMerge for StringHeavy<A> {
    fn merge_from_with_depth<B: DecodeBuf>(
        &mut self,
        buf: &mut B,
        depth: usize,
    ) -> Result<(), DecodeError> {
        if depth >= ::puroro::RECURSION_LIMIT {
            return Err(DecodeError::RecursionLimitExceeded);
        }
        while buf.has_remaining() {
            let (field_number, wire_type) = decode_tag(buf)?;
            match field_number {
                1 => self
                    .s0
                    .bind_mut(&mut self._common)
                    .merge(wire_type, buf, depth)?,
                2 => self
                    .s1
                    .bind_mut(&mut self._common)
                    .merge(wire_type, buf, depth)?,
                3 => self
                    .s2
                    .bind_mut(&mut self._common)
                    .merge(wire_type, buf, depth)?,
                4 => self
                    .s3
                    .bind_mut(&mut self._common)
                    .merge(wire_type, buf, depth)?,
                5 => self
                    .blob
                    .bind_mut(&mut self._common)
                    .merge(wire_type, buf, depth)?,
                _ => skip_field_and_save(
                    field_number,
                    wire_type,
                    buf,
                    &mut self._common.unknown_fields,
                    self._common.alloc.clone(),
                )?,
            }
        }
        Ok(())
    }
}

impl<A: Allocator + Clone> Message for StringHeavy<A> {
    type Alloc = A;

    fn new_in(alloc: A) -> Self {
        Self::new_in(alloc)
    }

    fn encode<B: BufMut>(&self, buf: &mut B) {
        ::puroro_rt::encode_message(self, buf)
    }

    fn encode_to_vec(&self) -> Vec<u8> {
        ::puroro_rt::encode_message_to_vec(self)
    }

    fn merge_from<B: Buf>(&mut self, buf: &mut B) -> Result<(), DecodeError> {
        ::puroro_rt::merge_message(self, buf)
    }

    fn unknown_fields(&self) -> impl Iterator<Item = ::puroro::UnknownField<'_>> + '_ {
        self._common.iter_unknown_fields()
    }

    fn validate(&self) -> Result<(), DecodeError> {
        Ok(())
    }
}
