//! Island-root ingest used by handwritten lazy messages.
//!
//! [`LazyScan::push`] copies each chunk onto [`SharedWire`] once, then scans
//! that instance. An unfinished tail is an offset (`unparsed`), not a second
//! `AllocVec`. [`LazyScan::push_bytes`] adopts a [`Bytes`] when the island is
//! empty.

use ::allocator_api2::alloc::{Allocator, Global};
use ::allocator_api2::vec::Vec as AllocVec;
use ::bytes::{BufMut, Bytes};
use ::core::cell::Cell;
use ::core::ptr;
use ::puroro::DecodeError;
use ::std::vec::Vec;

use super::record::{ScannedRecord, WireSpan, scan_slice};
use super::shared_wire::SharedWire;

/// Ingest layout stored in [`MessageCommon`](crate::MessageCommon)::`lazy`.
///
/// Implemented by [`Eager`] (ZST) and [`Lazy`].
pub trait MessageScan<A: Allocator>: Sized {
    fn new_scan(alloc: A) -> Self;
    fn clone_scan(&self, alloc: A) -> Self;
}

/// Eager ingest layout: no leftover, island buffer, or unparsed regions.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Eager;

/// Lazy ingest layout: owns the [`LazyScan`] machine.
pub struct Lazy<A: Allocator = Global> {
    pub scan: LazyScan<A>,
}

impl<A: Allocator> MessageScan<A> for Eager {
    #[inline]
    fn new_scan(_alloc: A) -> Self {
        Self
    }

    #[inline]
    fn clone_scan(&self, _alloc: A) -> Self {
        Self
    }
}

/// Accumulates input bytes and yields complete records with `_wire` origins.
///
/// A nested child may sit in an **unparsed** state: [`regions`](Self::regions)
/// name complete bodies on the island buffer, but the record scanner has not
/// walked them yet. Field getters call [`LazyMessage::ensure_scanned`] and
/// parse then. Encode / `into_eager` may use the regions without parsing.
pub struct LazyScan<A: Allocator = Global> {
    /// Start of the unfinished tail in [`wire`](Self::wire). Equal to
    /// `wire.len()` when every ingested byte is a complete record.
    unparsed: usize,
    finished: bool,
    /// `false` when [`regions`](Self::regions) are waiting for a first field get.
    applied: Cell<bool>,
    wire: SharedWire<A>,
    /// Payload spans in [`wire`](Self::wire) when this node is not the island
    /// root. Empty means encode / replay uses the whole buffer.
    regions: AllocVec<WireSpan, A>,
}

impl<A: Allocator> LazyScan<A> {
    pub fn wire(&self) -> &[u8] {
        self.wire.as_bytes()
    }

    pub fn shared(&self) -> &SharedWire<A> {
        &self.wire
    }

    pub fn require_finished(&self) -> Result<(), DecodeError> {
        if self.finished {
            Ok(())
        } else if self.unparsed < self.wire.len() {
            Err(DecodeError::TruncatedMessage)
        } else {
            Err(DecodeError::UnfinishedMessage)
        }
    }

    /// `true` when catalog slots match a completed parse (or an empty message).
    pub fn is_applied(&self) -> bool {
        self.applied.get()
    }

    pub fn mark_applied(&self) {
        self.applied.set(true);
    }

    pub fn mark_unapplied(&self) {
        self.applied.set(false);
    }

    pub fn regions(&self) -> &[WireSpan] {
        &self.regions
    }

    /// Byte length of the message body (regions, or the whole island buffer).
    pub fn encoded_len(&self) -> Result<usize, DecodeError> {
        self.require_finished()?;
        Ok(self.body_len())
    }

    /// Write the message body as-is. Requires a finished stream.
    pub fn encode<B: BufMut>(&self, buf: &mut B) -> Result<(), DecodeError> {
        self.require_finished()?;
        self.write_bodies(buf)
    }

    pub fn body_len(&self) -> usize {
        if self.regions.is_empty() {
            self.wire.len()
        } else {
            self.regions.iter().map(|s| s.len).sum()
        }
    }

    pub fn write_bodies<B: BufMut>(&self, buf: &mut B) -> Result<(), DecodeError> {
        if self.regions.is_empty() {
            buf.put_slice(self.wire.as_bytes());
            return Ok(());
        }
        for &span in &self.regions {
            buf.put_slice(span.slice(self.wire.as_bytes())?);
        }
        Ok(())
    }

    /// Replay each body region (or the whole buffer) into `f`.
    pub fn for_each_body(
        &self,
        mut f: impl FnMut(&[u8]) -> Result<(), DecodeError>,
    ) -> Result<(), DecodeError> {
        if self.regions.is_empty() {
            return f(self.wire.as_bytes());
        }
        for &span in &self.regions {
            f(span.slice(self.wire.as_bytes())?)?;
        }
        Ok(())
    }
}

impl<A: Allocator + Clone> LazyScan<A> {
    pub fn new_in(alloc: A) -> Self {
        Self {
            unparsed: 0,
            finished: true,
            applied: Cell::new(true),
            wire: SharedWire::empty(alloc.clone()),
            regions: AllocVec::new_in(alloc),
        }
    }

    /// Unfinished tail still on the island (`wire[unparsed..]`).
    pub fn leftover(&self) -> &[u8] {
        &self.wire.as_bytes()[self.unparsed..]
    }

    /// Copy `chunk` onto the island once, then scan that instance.
    ///
    /// A scan error truncates `_wire` back to its previous length so a rejected
    /// chunk cannot sit ahead of the scanner. Returns the origin of the
    /// scanned range in `_wire`, plus the records.
    pub fn push(
        &mut self,
        chunk: &[u8],
    ) -> Result<(usize, AllocVec<ScannedRecord, A>), DecodeError> {
        self.ingest(|wire| wire.append(chunk))
    }

    /// Same as [`push`](Self::push), but an empty island adopts `chunk`.
    pub fn push_bytes(
        &mut self,
        chunk: Bytes,
    ) -> Result<(usize, AllocVec<ScannedRecord, A>), DecodeError> {
        self.ingest(|wire| wire.append_bytes(chunk))
    }

    fn ingest(
        &mut self,
        append: impl FnOnce(&mut SharedWire<A>),
    ) -> Result<(usize, AllocVec<ScannedRecord, A>), DecodeError> {
        debug_assert!(self.unparsed <= self.wire.len());
        let old_len = self.wire.len();
        let origin = self.unparsed;
        append(&mut self.wire);
        match self.scan_from(origin) {
            Ok(records) => {
                self.finished = false;
                Ok((origin, records))
            }
            Err(e) => {
                self.wire.truncate(old_len);
                Err(e)
            }
        }
    }

    fn scan_from(&mut self, origin: usize) -> Result<AllocVec<ScannedRecord, A>, DecodeError> {
        let alloc = self.wire.allocator().clone();
        let (records, consumed) = scan_slice(&self.wire.as_bytes()[origin..], alloc)?;
        self.unparsed = origin + consumed;
        debug_assert!(self.unparsed <= self.wire.len());
        Ok(records)
    }

    pub fn finish(&mut self) -> Result<(), DecodeError> {
        if self.unparsed < self.wire.len() {
            return Err(DecodeError::TruncatedMessage);
        }
        self.finished = true;
        Ok(())
    }

    /// Point this scan at an existing island-root buffer (no payload copy).
    pub fn adopt(&mut self, root: &SharedWire<A>) {
        self.wire = root.clone_handle();
        self.unparsed = self.wire.len();
    }

    /// Remember a complete message-body LEN inside the adopted island buffer.
    pub fn record_region(&mut self, span: WireSpan) {
        self.regions.push(span);
    }

    /// Adopt `root` and record `span`.
    ///
    /// Returns [`Some`] when catalog slots are already applied and the caller
    /// must parse this occurrence immediately. The first region on an empty
    /// applied shell stays unparsed until the first field getter.
    pub(crate) fn store_shared(
        &mut self,
        root: &SharedWire<A>,
        span: WireSpan,
    ) -> Option<WireSpan> {
        self.adopt(root);
        let first_region = self.regions.is_empty();
        let was_applied = self.is_applied();
        self.record_region(span);
        if first_region && was_applied {
            self.mark_unapplied();
            None
        } else if was_applied {
            Some(span)
        } else {
            None
        }
    }

    /// Parse one already-recorded body without appending it to `_wire`.
    pub fn scan_region(
        &mut self,
        span: WireSpan,
    ) -> Result<AllocVec<ScannedRecord, A>, DecodeError> {
        self.finished = false;
        let payload = span.slice(self.wire.as_bytes())?;
        let alloc = self.wire.allocator().clone();
        let (records, consumed) = scan_slice(payload, alloc)?;
        if consumed < payload.len() {
            return Err(DecodeError::TruncatedMessage);
        }
        self.finished = true;
        Ok(records)
    }

    pub fn clone_in(&self, alloc: A) -> Self {
        Self {
            unparsed: self.unparsed,
            finished: self.finished,
            applied: Cell::new(self.applied.get()),
            wire: self.wire.clone_in(alloc.clone()),
            regions: self.regions.clone(),
        }
    }
}

impl<A: Allocator + Clone> MessageScan<A> for Lazy<A> {
    #[inline]
    fn new_scan(alloc: A) -> Self {
        Self {
            scan: LazyScan::new_in(alloc),
        }
    }

    #[inline]
    fn clone_scan(&self, alloc: A) -> Self {
        Self {
            scan: self.scan.clone_in(alloc),
        }
    }
}

/// Generated lazy message: field matching stays here; scan deferral does not.
///
/// Implement [`scan`](Self::scan), [`scan_mut`](Self::scan_mut), and
/// [`apply_record`](Self::apply_record). Getters call
/// [`ensure_scanned`](Self::ensure_scanned); a parent nested LEN calls
/// [`merge_shared`](Self::merge_shared).
pub trait LazyMessage<A: Allocator + Clone> {
    fn scan(&self) -> &LazyScan<A>;
    fn scan_mut(&mut self) -> &mut LazyScan<A>;
    fn apply_record(&mut self, rec: &ScannedRecord, origin: usize) -> Result<(), DecodeError>;

    /// Parse stored regions on the first field get.
    fn ensure_scanned(&self) -> Result<(), DecodeError> {
        self.scan().require_finished()?;
        if self.scan().is_applied() {
            return Ok(());
        }
        // SAFETY: field getters hold `&self` only; ingest uses `&mut self` and
        // does not overlap. The first get walks stored regions once.
        let this = ptr::from_ref(self).cast_mut();
        unsafe { (*this).apply_pending() }
    }

    /// Remember one complete message body on `root` without walking its tags.
    ///
    /// The first field getter parses every stored region (protobuf merge). If
    /// slots were already applied, this occurrence is parsed immediately.
    fn merge_shared(&mut self, root: &SharedWire<A>, span: WireSpan) -> Result<(), DecodeError> {
        if let Some(span) = self.scan_mut().store_shared(root, span) {
            self.apply_span(span)?;
        }
        Ok(())
    }

    fn apply_pending(&mut self) -> Result<(), DecodeError> {
        let regions: Vec<WireSpan> = self.scan().regions().to_vec();
        for span in regions {
            self.apply_span(span)?;
        }
        self.scan().mark_applied();
        Ok(())
    }

    fn apply_span(&mut self, span: WireSpan) -> Result<(), DecodeError> {
        let records = self.scan_mut().scan_region(span)?;
        for rec in records {
            self.apply_record(&rec, span.offset)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::allocator_api2::alloc::Global;
    use ::puroro::DecodeError;

    /// Field 1, varint 1 (`0x08 0x01`).
    const SCORE_ONE: &[u8] = &[0x08, 0x01];
    /// Field 1, wire type 7 — invalid tag.
    const BAD_TAG: &[u8] = &[0x0f];

    #[test]
    fn push_copies_once_then_scans_island() {
        let mut scan = LazyScan::new_in(Global);
        let (origin, recs) = scan.push(SCORE_ONE).unwrap();
        scan.finish().unwrap();
        assert_eq!(origin, 0);
        assert_eq!(recs.len(), 1);
        assert_eq!(scan.wire(), SCORE_ONE);
        assert!(scan.leftover().is_empty());
    }

    #[test]
    fn leftover_is_island_tail() {
        let mut scan = LazyScan::new_in(Global);
        let (origin, recs) = scan.push(&[0x08]).unwrap();
        assert_eq!(origin, 0);
        assert!(recs.is_empty());
        assert_eq!(scan.leftover(), &[0x08]);
        assert_eq!(scan.wire(), &[0x08]);
        let (origin, recs) = scan.push(&[0x01]).unwrap();
        scan.finish().unwrap();
        assert_eq!(origin, 0);
        assert_eq!(recs.len(), 1);
        assert_eq!(scan.wire(), SCORE_ONE);
        assert!(scan.leftover().is_empty());
    }

    #[test]
    fn rejected_chunk_does_not_stay_on_island() {
        let mut scan = LazyScan::new_in(Global);
        scan.push(SCORE_ONE).unwrap();
        scan.finish().unwrap();
        assert_eq!(scan.push(BAD_TAG).err(), Some(DecodeError::InvalidTag));
        assert_eq!(scan.wire(), SCORE_ONE);
        assert!(scan.leftover().is_empty());
    }

    #[test]
    fn push_bytes_adopts_when_empty() {
        let data = Bytes::from_static(SCORE_ONE);
        let ptr = data.as_ptr();
        let mut scan = LazyScan::new_in(Global);
        let (origin, recs) = scan.push_bytes(data).unwrap();
        scan.finish().unwrap();
        assert_eq!(origin, 0);
        assert_eq!(recs.len(), 1);
        assert_eq!(scan.wire().as_ptr(), ptr);
    }

    #[test]
    fn failed_push_bytes_does_not_adopt() {
        let data = Bytes::from_static(BAD_TAG);
        let mut scan = LazyScan::new_in(Global);
        assert_eq!(scan.push_bytes(data).err(), Some(DecodeError::InvalidTag));
        assert!(scan.wire().is_empty());
    }
}
