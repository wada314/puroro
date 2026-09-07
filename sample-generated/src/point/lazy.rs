//! Read-oriented `Point`. Both fields are implicit int32 numericals.

use ::allocator_api2::alloc::{Allocator, Global};
use ::bitvec::array::BitArray;
use ::bitvec::order::Lsb0;
use ::bytes::{Buf, BufMut};
use ::core::ops::ControlFlow;
use ::puroro::{DecodeError, Message};
use ::puroro_rt::decode::{LazyMessage, LazyScan, ScannedRecord, merge_scanned_field};
use ::puroro_rt::{
    FieldDeallocVisitor, FieldVisitorMut, Implicit, MessageCommon, ProtoInt32, SingularField,
};
use ::std::vec::Vec;

use crate::Point;
use crate::point::{FIELD_X, FIELD_Y};

/// Lazy `Point` with catalog numerical slots.
pub struct PointLazy<A: Allocator = Global> {
    scan: LazyScan<A>,
    _common: MessageCommon<BitArray<[u8; 1], Lsb0>, A>,
    x: SingularField<ProtoInt32, Implicit, { FIELD_X }, A>,
    y: SingularField<ProtoInt32, Implicit, { FIELD_Y }, A>,
}

impl<A: Allocator + Clone + Default> Default for PointLazy<A> {
    fn default() -> Self {
        Self::new_in(A::default())
    }
}

impl PointLazy<Global> {
    pub fn new() -> Self {
        Self::new_in(Global)
    }

    pub fn decode<B: Buf>(mut buf: B) -> Result<Self, DecodeError> {
        let mut msg = Self::new();
        msg.merge_from(&mut buf)?;
        Ok(msg)
    }
}

impl<A: Allocator> PointLazy<A> {
    pub fn x(&self) -> Result<i32, DecodeError>
    where
        A: Clone,
    {
        self.ensure_scanned()?;
        Ok(self.x.bind(&self._common).value())
    }

    pub fn y(&self) -> Result<i32, DecodeError>
    where
        A: Clone,
    {
        self.ensure_scanned()?;
        Ok(self.y.bind(&self._common).value())
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

    fn visit_fields_mut<V: FieldVisitorMut<MessageCommon<BitArray<[u8; 1], Lsb0>, A>>>(
        &mut self,
        v: &mut V,
    ) -> ControlFlow<V::Break> {
        v.visit("x", &mut self.x)?;
        v.visit("y", &mut self.y)?;
        ControlFlow::Continue(())
    }
}

impl<A: Allocator + Clone> PointLazy<A> {
    pub fn new_in(alloc: A) -> Self {
        Self {
            scan: LazyScan::new_in(alloc.clone()),
            _common: MessageCommon::new_in(BitArray::ZERO, alloc.clone()),
            x: SingularField::new_in(alloc.clone()),
            y: SingularField::new_in(alloc),
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

    /// Decode every field from `_wire` into an eager [`Point`].
    pub fn into_eager(self) -> Result<Point<A>, DecodeError> {
        self.scan.require_finished()?;
        let mut eager = Point::new_in(self._common.alloc.clone());
        self.scan.for_each_body(|chunk| {
            let mut buf = chunk;
            Message::merge_from(&mut eager, &mut buf)
        })?;
        Ok(eager)
    }
}

impl<A: Allocator + Clone> LazyMessage<A> for PointLazy<A> {
    fn scan(&self) -> &LazyScan<A> {
        &self.scan
    }

    fn scan_mut(&mut self) -> &mut LazyScan<A> {
        &mut self.scan
    }

    fn apply_record(&mut self, rec: &ScannedRecord<A>, _origin: usize) -> Result<(), DecodeError> {
        match rec.field_number.as_u32() {
            FIELD_X => merge_scanned_field(&rec.field, |wire_type, buf| {
                self.x.bind_mut(&mut self._common).merge(wire_type, buf, 0)
            }),
            FIELD_Y => merge_scanned_field(&rec.field, |wire_type, buf| {
                self.y.bind_mut(&mut self._common).merge(wire_type, buf, 0)
            }),
            _ => Ok(()),
        }
    }
}

impl<A: Allocator> Drop for PointLazy<A> {
    fn drop(&mut self) {
        let mut v = FieldDeallocVisitor::new(&self._common);
        let _ = self.visit_fields_mut(&mut v);
        self._common.deallocate();
    }
}
