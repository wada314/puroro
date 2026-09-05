//! Shared leftover + `_wire` ingest used by handwritten lazy messages.

use ::allocator_api2::alloc::{Allocator, Global};
use ::allocator_api2::vec::Vec as AllocVec;
use ::puroro::DecodeError;

use super::record::{RecordScanner, ScannedRecord};

/// Accumulates input bytes and yields complete records with `_wire` origins.
pub struct LazyScan<A: Allocator = Global> {
    scanner: RecordScanner<A>,
    finished: bool,
    wire: AllocVec<u8, A>,
}

impl<A: Allocator> LazyScan<A> {
    pub fn wire(&self) -> &[u8] {
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
}

impl<A: Allocator + Clone> LazyScan<A> {
    pub fn new_in(alloc: A) -> Self {
        Self {
            scanner: RecordScanner::new_in(alloc.clone()),
            finished: true,
            wire: AllocVec::new_in(alloc),
        }
    }

    /// Append `chunk` to `_wire` and parse complete records.
    ///
    /// Returns the origin of `leftover || chunk` in `_wire`, plus the records.
    pub fn push(
        &mut self,
        chunk: &[u8],
    ) -> Result<(usize, AllocVec<ScannedRecord<A>, A>), DecodeError> {
        self.finished = false;
        debug_assert!(self.wire.ends_with(self.scanner.leftover()));
        let origin = self.wire.len() - self.scanner.leftover().len();
        self.wire.extend_from_slice(chunk);
        let records = self.scanner.push(chunk)?;
        debug_assert!(self.wire.ends_with(self.scanner.leftover()));
        Ok((origin, records))
    }

    pub fn finish(&mut self) -> Result<(), DecodeError> {
        self.scanner.finish()?;
        self.finished = true;
        Ok(())
    }
}
