//! Shared leftover + `_wire` ingest used by handwritten lazy messages.

use ::allocator_api2::alloc::{Allocator, Global};
use ::allocator_api2::vec::Vec as AllocVec;
use ::bytes::{Buf, BufMut, Bytes};
use ::core::cell::Cell;
use ::core::cmp::PartialEq;
use ::core::ptr;
use ::puroro::{DecodeError, UnknownField};
use ::std::vec::Vec;

use super::iter_unknown_fields;
use super::record::{RecordScanner, ScannedRecord, WireSpan};
use super::wire_buf::{WireAppend, WireBuf};

/// Ingest layout stored in [`MessageCommon`](crate::MessageCommon)::`lazy`.
///
/// Implemented by [`Eager`] (ZST) and [`Lazy`].
pub trait MessageScan<A: Allocator>: Sized {
    fn new_scan(alloc: A) -> Self;
    fn clone_scan(&self, alloc: A) -> Self;

    /// Remember one lazy unknown record. Eager keeps unknowns in the store.
    fn record_policy_span(&mut self, span: WireSpan) {
        let _ = span;
    }
}

/// Eager ingest layout: no leftover, island buffer, or unparsed regions.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Eager;

/// Lazy ingest layout: owns the [`LazyScan`] machine.
pub struct Lazy<A: Allocator = Global, B: WireBuf = Bytes> {
    pub scan: LazyScan<A, B>,
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
pub struct LazyScan<A: Allocator = Global, B: WireBuf = Bytes> {
    scanner: RecordScanner<A>,
    finished: bool,
    /// `false` when [`regions`](Self::regions) are waiting for a first field get.
    applied: Cell<bool>,
    wire: B,
    /// Payload spans in [`wire`](Self::wire) when this node is not the island
    /// root. Empty means encode / replay uses the whole buffer.
    regions: AllocVec<WireSpan, A>,
    /// Unrecognized tags and closed-enum unknowns, as full records in `wire`.
    ///
    /// Preserve yields these from `unknown_fields()` and still encodes `_wire`.
    /// Discard and reject omit them on encode and do not yield them. The store
    /// is not also iterated, so a closed-enum tag is not returned twice.
    policy_spans: AllocVec<WireSpan, A>,
}

impl<A: Allocator, B: WireBuf> LazyScan<A, B> {
    pub fn wire(&self) -> &[u8] {
        self.wire.as_bytes()
    }

    /// Handle to the stored buffer (`Bytes`, `&[u8]`, …).
    pub fn wire_buf(&self) -> &B {
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
    pub fn encode<Out: BufMut>(&self, buf: &mut Out) -> Result<(), DecodeError> {
        self.require_finished()?;
        self.write_bodies(buf)
    }

    pub fn body_len(&self) -> usize {
        if self.regions.is_empty() {
            self.wire.as_bytes().len()
        } else {
            self.regions.iter().map(|s| s.len).sum()
        }
    }

    pub fn write_bodies<Out: BufMut>(&self, buf: &mut Out) -> Result<(), DecodeError> {
        if self.regions.is_empty() {
            buf.put_slice(self.wire.as_bytes());
            return Ok(());
        }
        for &span in &self.regions {
            buf.put_slice(span.slice(self.wire.as_bytes())?);
        }
        Ok(())
    }

    /// Byte equality of the ingested body (regions, or the whole island buffer).
    ///
    /// Lazy `PartialEq` uses this when an eager value cannot be built, so a
    /// message stays equal to itself. Does not require [`finish`](Self::finish).
    pub fn body_eq(&self, other: &Self) -> bool {
        if self.body_len() != other.body_len() {
            return false;
        }
        let mut left = Vec::new();
        let mut right = Vec::new();
        match (self.write_bodies(&mut left), other.write_bodies(&mut right)) {
            (Ok(()), Ok(())) => left == right,
            _ => false,
        }
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

impl<A: Allocator + Clone, B: WireAppend> LazyScan<A, B> {
    pub fn new_in(alloc: A) -> Self {
        Self {
            scanner: RecordScanner::new_in(alloc.clone()),
            finished: true,
            applied: Cell::new(true),
            wire: B::empty(),
            regions: AllocVec::new_in(alloc.clone()),
            policy_spans: AllocVec::new_in(alloc),
        }
    }

    /// Parse `chunk` and, on success, append it to the island buffer.
    ///
    /// A scan error leaves `_wire`, the leftover, and the finished flag as they
    /// were, so a rejected chunk cannot sit in the buffer ahead of the scanner.
    /// Returns the origin of `leftover || chunk` in `_wire`, plus the records.
    pub fn push(
        &mut self,
        chunk: &[u8],
    ) -> Result<(usize, AllocVec<ScannedRecord, A>), DecodeError> {
        debug_assert!(self.wire.as_bytes().ends_with(self.scanner.leftover()));
        let origin = self.wire.as_bytes().len() - self.scanner.leftover().len();
        let records = self.scanner.push(chunk)?;
        self.finished = false;
        self.wire.append(chunk);
        debug_assert!(self.wire.as_bytes().ends_with(self.scanner.leftover()));
        Ok((origin, records))
    }

    pub fn clone_in(&self, _alloc: A) -> Self {
        Self {
            scanner: self.scanner.clone(),
            finished: self.finished,
            applied: Cell::new(self.applied.get()),
            wire: self.wire.clone(),
            regions: self.regions.clone(),
            policy_spans: self.policy_spans.clone(),
        }
    }
}

impl<A: Allocator + Clone, B: WireBuf> LazyScan<A, B> {
    pub fn finish(&mut self) -> Result<(), DecodeError> {
        self.scanner.finish()?;
        self.finished = true;
        Ok(())
    }

    pub fn policy_spans(&self) -> &[WireSpan] {
        &self.policy_spans
    }

    pub fn push_policy_span(&mut self, span: WireSpan) {
        // A failed apply retries the same regions. Recording the span again
        // would make `unknown_fields()` yield it twice and `encoded_len`
        // subtract the bytes twice.
        if self.policy_spans.contains(&span) {
            return;
        }
        self.policy_spans.push(span);
    }

    /// Byte length of the body after removing `omit` records.
    pub fn body_len_omitting(&self, omit: &[WireSpan]) -> usize {
        let mut total = 0;
        self.for_each_body_range(|range| {
            total += retained_len(range, omit);
        });
        total
    }

    /// Writes the body, skipping `omit` records that sit inside it.
    pub fn write_bodies_omitting<Out: BufMut>(
        &self,
        buf: &mut Out,
        omit: &[WireSpan],
    ) -> Result<(), DecodeError> {
        let mut failed = Ok(());
        self.for_each_body_range(|range| {
            if failed.is_err() {
                return;
            }
            if let Err(err) = write_range_omitting(self.wire.as_bytes(), range, omit, buf) {
                failed = Err(err);
            }
        });
        failed
    }

    /// One unknown field per policy span, in ingest order.
    pub fn iter_policy_spans(&self) -> PolicySpanIter<'_> {
        PolicySpanIter {
            wire: self.wire.as_bytes(),
            spans: &self.policy_spans,
            index: 0,
        }
    }

    fn for_each_body_range(&self, mut f: impl FnMut(WireSpan)) {
        if self.regions.is_empty() {
            f(WireSpan {
                offset: 0,
                len: self.wire.as_bytes().len(),
            });
        } else {
            for &span in &self.regions {
                f(span);
            }
        }
    }

    /// Point this scan at an existing island-root buffer (no payload copy).
    pub fn adopt(&mut self, root: &B) {
        self.wire = root.clone();
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
    pub(crate) fn store_shared(&mut self, root: &B, span: WireSpan) -> Option<WireSpan> {
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
        let records = self.scanner.push(payload)?;
        self.scanner.finish()?;
        self.finished = true;
        Ok(records)
    }
}

impl<A: Allocator + Clone, B: WireAppend> MessageScan<A> for Lazy<A, B> {
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

    #[inline]
    fn record_policy_span(&mut self, span: WireSpan) {
        self.scan.push_policy_span(span);
    }
}

impl<A: Allocator + Clone, B: WireBuf> Lazy<A, B> {
    /// Remember one unknown record span in this message's scan.
    #[inline]
    pub fn record_policy_span(&mut self, span: WireSpan) {
        self.scan.push_policy_span(span);
    }
}

/// Generated lazy message: field matching stays here; scan deferral does not.
///
/// Implement [`scan`](Self::scan), [`scan_mut`](Self::scan_mut), and
/// [`apply_record`](Self::apply_record). Ingest, finish, and wire-passthrough
/// encode are provided. Getters call [`ensure_scanned`](Self::ensure_scanned);
/// a parent nested LEN calls [`merge_shared`](Self::merge_shared).
///
/// [`encode`](Self::encode) and [`encode_to_vec`](Self::encode_to_vec) return
/// [`Result`] and require a finished scan. [`puroro::Message::encode`] and
/// [`puroro::Message::encode_to_vec`] are infallible and write through
/// [`crate::MessageEncode`]. When both traits are in scope, call the
/// `Result` methods with UFCS. [`puroro::Message::decode`] is the complete
/// decode entry; generated `Message::merge_from` calls [`ingest_buf`](Self::ingest_buf).
pub trait LazyMessage<A: Allocator + Clone, B: WireBuf = Bytes> {
    fn scan(&self) -> &LazyScan<A, B>;
    fn scan_mut(&mut self) -> &mut LazyScan<A, B>;
    fn apply_record(
        &mut self,
        rec: &ScannedRecord,
        origin: usize,
        depth: usize,
    ) -> Result<(), DecodeError>;

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
    fn merge_shared(&mut self, root: &B, span: WireSpan) -> Result<(), DecodeError> {
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
            self.apply_record(&rec, span.offset, 0)?;
        }
        Ok(())
    }

    /// Byte length of the ingested body.
    ///
    /// Fails with [`DecodeError::UnfinishedMessage`] or
    /// [`DecodeError::TruncatedMessage`] when [`finish`](Self::finish) has not
    /// succeeded.
    #[inline]
    fn encoded_len(&self) -> Result<usize, DecodeError> {
        self.scan().encoded_len()
    }

    /// Writes the ingested body as-is.
    ///
    /// Same finish requirement as [`encoded_len`](Self::encoded_len). This is
    /// not a canonical field-by-field encode.
    #[inline]
    fn encode<Out: BufMut>(&self, buf: &mut Out) -> Result<(), DecodeError> {
        self.scan().encode(buf)
    }

    /// Encodes the ingested body into a new `Vec<u8>`.
    ///
    /// Same finish requirement as [`encoded_len`](Self::encoded_len).
    fn encode_to_vec(&self) -> Result<Vec<u8>, DecodeError> {
        let mut out = Vec::with_capacity(self.encoded_len()?);
        self.encode(&mut out)?;
        Ok(out)
    }

    /// Closes the current input stream.
    ///
    /// Non-empty leftover is [`DecodeError::TruncatedMessage`].
    #[inline]
    fn finish(&mut self) -> Result<(), DecodeError> {
        self.scan_mut().finish()
    }

    /// Parses `chunk`, appends it to the island buffer, and applies every
    /// complete record.
    fn push(&mut self, chunk: &[u8]) -> Result<(), DecodeError>
    where
        B: WireAppend,
    {
        self.push_with_depth(chunk, 0)
    }

    /// [`push`](Self::push) at nesting `depth` (0 is the top-level message).
    fn push_with_depth(&mut self, chunk: &[u8], depth: usize) -> Result<(), DecodeError>
    where
        B: WireAppend,
    {
        let (origin, records) = self.scan_mut().push(chunk)?;
        for rec in records {
            self.apply_record(&rec, origin, depth)?;
        }
        Ok(())
    }

    /// Ingests one complete message: [`push`](Self::push) each non-empty
    /// chunk, then [`finish`](Self::finish).
    ///
    /// A later call merges another message (scalars last-wins, repeated
    /// appends). It does not continue an unfinished chunk sequence; use
    /// [`push`](Self::push) for that.
    fn ingest_buf<In: Buf>(&mut self, buf: &mut In) -> Result<(), DecodeError>
    where
        B: WireAppend,
    {
        self.ingest_with_depth(buf, 0)
    }

    /// [`ingest_buf`](Self::ingest_buf) at nesting `depth`.
    fn ingest_with_depth<In: Buf>(&mut self, buf: &mut In, depth: usize) -> Result<(), DecodeError>
    where
        B: WireAppend,
    {
        while buf.has_remaining() {
            let n = buf.chunk().len();
            if n == 0 {
                let rest = buf.copy_to_bytes(buf.remaining());
                self.push_with_depth(&rest, depth)?;
                break;
            }
            self.push_with_depth(buf.chunk(), depth)?;
            buf.advance(n);
        }
        self.finish()
    }

    /// Equality for a lazy alias.
    ///
    /// When both sides have been scanned and both `to_eager` calls succeed,
    /// compare those eager values. When both sides fail the same way (the scan
    /// is unfinished, or building the eager message fails, including
    /// `DecodeError::InvalidUtf8`), compare ingested body bytes so equality
    /// stays reflexive. One success and one failure is not equal.
    fn eq_with_eager<E: PartialEq>(
        &self,
        other: &Self,
        to_eager: impl Fn(&Self) -> Result<E, DecodeError>,
    ) -> bool {
        match (self.ensure_scanned(), other.ensure_scanned()) {
            (Ok(()), Ok(())) => match (to_eager(self), to_eager(other)) {
                (Ok(left), Ok(right)) => left == right,
                (Err(_), Err(_)) => self.scan().body_eq(other.scan()),
                _ => false,
            },
            (Err(_), Err(_)) => self.scan().body_eq(other.scan()),
            _ => false,
        }
    }
}

/// Iterator of one unknown field per [`LazyScan`] policy span.
pub struct PolicySpanIter<'a> {
    wire: &'a [u8],
    spans: &'a [WireSpan],
    index: usize,
}

impl<'a> Iterator for PolicySpanIter<'a> {
    type Item = UnknownField<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.spans.len() {
            let span = self.spans[self.index];
            self.index += 1;
            let Ok(bytes) = span.slice(self.wire) else {
                continue;
            };
            if let Some(field) = iter_unknown_fields(bytes).next() {
                return Some(field);
            }
        }
        None
    }
}

/// Merged `[lo, hi)` cuts of `omit` that overlap `range`, in offset order.
///
/// Identical or overlapping spans count once, matching a retried apply that
/// recorded the same record twice.
fn omit_cuts(range: WireSpan, omit: &[WireSpan]) -> Vec<(usize, usize)> {
    let end = range.offset.saturating_add(range.len);
    let mut cuts: Vec<(usize, usize)> = Vec::new();
    for skip in omit {
        let lo = range.offset.max(skip.offset);
        let hi = end.min(skip.offset.saturating_add(skip.len));
        if hi > lo {
            cuts.push((lo, hi));
        }
    }
    cuts.sort_by_key(|cut| cut.0);
    let mut merged: Vec<(usize, usize)> = Vec::new();
    for (lo, hi) in cuts {
        if let Some(last) = merged.last_mut()
            && lo <= last.1
        {
            last.1 = last.1.max(hi);
            continue;
        }
        merged.push((lo, hi));
    }
    merged
}

fn retained_len(range: WireSpan, omit: &[WireSpan]) -> usize {
    let mut kept = range.len;
    for (lo, hi) in omit_cuts(range, omit) {
        kept = kept.saturating_sub(hi - lo);
    }
    kept
}

fn write_range_omitting<B: BufMut>(
    wire: &[u8],
    range: WireSpan,
    omit: &[WireSpan],
    buf: &mut B,
) -> Result<(), DecodeError> {
    let end = range
        .offset
        .checked_add(range.len)
        .ok_or(DecodeError::TruncatedMessage)?;
    let mut cursor = range.offset;
    for (lo, hi) in omit_cuts(range, omit) {
        if lo > cursor {
            buf.put_slice(wire.get(cursor..lo).ok_or(DecodeError::TruncatedMessage)?);
        }
        if hi > cursor {
            cursor = hi;
        }
    }
    if cursor < end {
        buf.put_slice(wire.get(cursor..end).ok_or(DecodeError::TruncatedMessage)?);
    }
    Ok(())
}

#[cfg(test)]
mod omit_tests {
    use super::{WireSpan, omit_cuts, retained_len, write_range_omitting};

    #[test]
    fn overlapping_omits_count_once() {
        let range = WireSpan { offset: 0, len: 10 };
        let omit = [
            WireSpan { offset: 2, len: 3 },
            WireSpan { offset: 2, len: 3 },
            WireSpan { offset: 4, len: 2 },
        ];
        assert_eq!(omit_cuts(range, &omit), vec![(2, 6)]);
        assert_eq!(retained_len(range, &omit), 6);
        let wire = [0_u8, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut out = Vec::new();
        write_range_omitting(&wire, range, &omit, &mut out).unwrap();
        assert_eq!(out, [0, 1, 6, 7, 8, 9]);
        assert_eq!(out.len(), retained_len(range, &omit));
    }
}
