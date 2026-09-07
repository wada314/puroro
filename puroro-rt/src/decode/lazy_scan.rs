//! Shared leftover + `_wire` ingest used by handwritten lazy messages.

use ::allocator_api2::alloc::{Allocator, Global};
use ::allocator_api2::vec::Vec as AllocVec;
use ::bytes::BufMut;
use ::core::cell::Cell;
use ::core::ptr;
use ::puroro::DecodeError;
use ::std::vec::Vec;

use super::record::{RecordScanner, ScannedRecord, WireSpan};
use super::shared_wire::SharedWire;

/// Accumulates input bytes and yields complete records with `_wire` origins.
///
/// A nested child may sit in an **unparsed** state: [`regions`](Self::regions)
/// name complete bodies on the island buffer, but the record scanner has not
/// walked them yet. Field getters call [`LazyMessage::ensure_scanned`] and
/// parse then. Encode / `into_eager` may use the regions without parsing.
pub struct LazyScan<A: Allocator = Global> {
    scanner: RecordScanner<A>,
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
        } else if self.scanner.needs_more() {
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
            scanner: RecordScanner::new_in(alloc.clone()),
            finished: true,
            applied: Cell::new(true),
            wire: SharedWire::empty(alloc.clone()),
            regions: AllocVec::new_in(alloc),
        }
    }

    /// Append `chunk` to the island buffer and parse complete records.
    ///
    /// Returns the origin of `leftover || chunk` in `_wire`, plus the records.
    pub fn push(
        &mut self,
        chunk: &[u8],
    ) -> Result<(usize, AllocVec<ScannedRecord<A>, A>), DecodeError> {
        self.finished = false;
        debug_assert!(self.wire.as_bytes().ends_with(self.scanner.leftover()));
        let origin = self.wire.len() - self.scanner.leftover().len();
        self.wire.append(chunk);
        let records = self.scanner.push(chunk)?;
        debug_assert!(self.wire.as_bytes().ends_with(self.scanner.leftover()));
        Ok((origin, records))
    }

    pub fn finish(&mut self) -> Result<(), DecodeError> {
        self.scanner.finish()?;
        self.finished = true;
        Ok(())
    }

    /// Point this scan at an existing island-root buffer (no payload copy).
    pub fn adopt(&mut self, root: &SharedWire<A>) {
        self.wire = root.clone_handle();
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
    ) -> Result<AllocVec<ScannedRecord<A>, A>, DecodeError> {
        self.finished = false;
        let payload = span.slice(self.wire.as_bytes())?;
        let records = self.scanner.push(payload)?;
        self.scanner.finish()?;
        self.finished = true;
        Ok(records)
    }

    pub fn clone_in(&self, alloc: A) -> Self {
        Self {
            scanner: self.scanner.clone(),
            finished: self.finished,
            applied: Cell::new(self.applied.get()),
            wire: self.wire.clone_in(alloc.clone()),
            regions: self.regions.clone(),
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
    fn apply_record(&mut self, rec: &ScannedRecord<A>, origin: usize) -> Result<(), DecodeError>;

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
