//! Shared leftover + `_wire` ingest used by handwritten lazy messages.

use ::allocator_api2::alloc::{Allocator, Global};
use ::allocator_api2::vec::Vec as AllocVec;
use ::bytes::BufMut;
use ::puroro::DecodeError;

use super::record::{RecordScanner, ScannedRecord, WireSpan};
use super::shared_wire::SharedWire;

/// Accumulates input bytes and yields complete records with `_wire` origins.
pub struct LazyScan<A: Allocator = Global> {
    scanner: RecordScanner<A>,
    finished: bool,
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

    fn body_len(&self) -> usize {
        if self.regions.is_empty() {
            self.wire.len()
        } else {
            self.regions.iter().map(|s| s.len).sum()
        }
    }

    fn write_bodies<B: BufMut>(&self, buf: &mut B) -> Result<(), DecodeError> {
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

    /// Parse a complete message body without appending it to `_wire`.
    pub fn scan_complete(
        &mut self,
        body: &[u8],
    ) -> Result<AllocVec<ScannedRecord<A>, A>, DecodeError> {
        self.finished = false;
        let records = self.scanner.push(body)?;
        self.scanner.finish()?;
        self.finished = true;
        Ok(records)
    }
}
