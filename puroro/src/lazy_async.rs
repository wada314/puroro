//! Async/streaming lazy parsing infrastructure.
//!
//! This module provides the building blocks for a poll-based lazy parser that reads from an
//! async reader and supports random-access getters by caching decoded values and/or raw bytes.
//!
//! Notes:
//! - The input is assumed to be **exactly one message**: either the caller provides a reader that
//!   ends at the message boundary (EOF), or the caller knows the total length and passes it in as
//!   a limit.
//! - Unknown fields are skipped (dropped) for now.

use crate::error::Error;
use ::bytes::{Buf, Bytes, BytesMut};
use ::futures_io::AsyncRead;
use ::std::collections::VecDeque;
use ::std::pin::Pin;
use ::std::task::{Context, Poll};

use ::protobuf_core::{Field, FieldValue};
use ::protobuf_core::{Tag, Varint, WireType};

/// Maximum allowed length-delimited size (2 GiB), matching the protobuf wire format limits.
const MAX_LEN_DELIMITED_SIZE: usize = 2 * 1024 * 1024 * 1024;
/// Maximum varint length in bytes.
const MAX_VARINT_BYTES_LOCAL: usize = 10;
use ::std::cell::RefCell;
use ::std::rc::Rc;

/// An in-memory async reader over a `Bytes` buffer.
///
/// This is useful for nested (length-delimited) message fields: the parent can store the
/// payload as `Bytes` and create a child parser over it without requiring a separate IO source.
pub struct BytesReader {
    bytes: Bytes,
    pos: usize,
}

impl BytesReader {
    pub fn new(bytes: Bytes) -> Self {
        Self { bytes, pos: 0 }
    }
}

impl AsyncRead for BytesReader {
    fn poll_read(
        mut self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        out: &mut [u8],
    ) -> Poll<Result<usize, ::std::io::Error>> {
        let remaining = &self.bytes.as_ref()[self.pos..];
        if remaining.is_empty() {
            return Poll::Ready(Ok(0));
        }
        let n = remaining.len().min(out.len());
        out[..n].copy_from_slice(&remaining[..n]);
        self.pos += n;
        Poll::Ready(Ok(n))
    }
}

/// Default chunk size to read from the underlying reader.
const DEFAULT_READ_CHUNK_SIZE: usize = 8 * 1024;

/// Buffered async input that supports partial reads and cheap slicing.
///
/// The buffer is stored as multiple `Bytes` segments to allow zero-copy subslices when a field's
/// bytes are fully contained in a single segment. When a field spans multiple segments, callers
/// can coalesce it into a single owned buffer.
pub(crate) struct AsyncInput<R> {
    reader: R,
    /// Remaining bytes allowed to read from the reader (message boundary), if known.
    remaining_limit: Option<usize>,
    eof: bool,
    read_chunk_size: usize,
    segments: VecDeque<Bytes>,
}

impl<R> AsyncInput<R>
where
    R: AsyncRead + Unpin,
{
    /// Create a new input that reads until EOF.
    pub(crate) fn new(reader: R) -> Self {
        Self {
            reader,
            remaining_limit: None,
            eof: false,
            read_chunk_size: DEFAULT_READ_CHUNK_SIZE,
            segments: VecDeque::new(),
        }
    }

    /// Create a new input that reads at most `limit` bytes.
    pub(crate) fn with_limit(reader: R, limit: usize) -> Self {
        Self {
            reader,
            remaining_limit: Some(limit),
            eof: limit == 0,
            read_chunk_size: DEFAULT_READ_CHUNK_SIZE,
            segments: VecDeque::new(),
        }
    }

    /// Set the internal read chunk size.
    pub(crate) fn set_read_chunk_size(&mut self, bytes: usize) {
        self.read_chunk_size = bytes.max(1);
    }

    /// Returns `true` if the buffered input has no remaining bytes and will never produce more.
    pub(crate) fn is_eof(&self) -> bool {
        self.eof && self.segments.is_empty()
    }

    /// Returns the number of bytes currently buffered (not yet consumed).
    pub(crate) fn buffered_len(&self) -> usize {
        self.segments.iter().map(|b| b.remaining()).sum()
    }

    /// Ensure at least `needed` bytes are buffered, reading from the underlying reader if needed.
    pub(crate) fn poll_ensure(&mut self, cx: &mut Context<'_>, needed: usize) -> Poll<Result<(), Error>> {
        while self.buffered_len() < needed {
            if self.eof {
                break;
            }
            let read = match self.poll_read_more(cx)? {
                Poll::Ready(n) => n,
                Poll::Pending => return Poll::Pending,
            };
            if read == 0 {
                break;
            }
        }
        Poll::Ready(Ok(()))
    }

    /// Returns the first contiguous chunk of buffered bytes (may be empty).
    pub(crate) fn peek_chunk(&self) -> &[u8] {
        self.segments.front().map(|b| b.chunk()).unwrap_or(&[])
    }

    /// Consume `n` bytes from the buffer.
    pub(crate) fn advance(&mut self, mut n: usize) {
        while n > 0 {
            let Some(mut front) = self.segments.pop_front() else {
                break;
            };
            let avail = front.remaining();
            if n < avail {
                front.advance(n);
                self.segments.push_front(front);
                return;
            }
            // Consume whole segment.
            n -= avail;
        }
    }

    /// Consume and return exactly `len` bytes as a `Bytes`.
    ///
    /// - If the bytes are fully contained in the first segment, this returns a zero-copy `Bytes`.
    /// - Otherwise, this allocates and coalesces the bytes into a single buffer.
    pub(crate) fn poll_take_bytes(
        &mut self,
        cx: &mut Context<'_>,
        len: usize,
    ) -> Poll<Result<Bytes, Error>> {
        if len == 0 {
            return Poll::Ready(Ok(Bytes::new()));
        }

        // Ensure we have enough buffered.
        let _ = match self.poll_ensure(cx, len)? {
            Poll::Ready(()) => (),
            Poll::Pending => return Poll::Pending,
        };

        if self.buffered_len() < len {
            // True EOF / limit reached before we could collect enough bytes.
            return Poll::Ready(Err(Error::InvalidWireFormat(
                "Unexpected EOF while reading length-delimited field".to_string(),
            )));
        }

        // Fast path: the first segment contains the entire range.
        if let Some(mut front) = self.segments.pop_front() {
            if front.remaining() >= len {
                let taken = front.split_to(len);
                if front.remaining() > 0 {
                    self.segments.push_front(front);
                }
                return Poll::Ready(Ok(taken));
            }
            // Put it back and fall through to coalesce.
            self.segments.push_front(front);
        }

        // Slow path: coalesce across segments.
        let mut out = BytesMut::with_capacity(len);
        let mut remaining = len;
        while remaining > 0 {
            let mut front = self.segments.pop_front().expect("buffered_len checked");
            let chunk = front.chunk();
            let take = remaining.min(chunk.len());
            out.extend_from_slice(&chunk[..take]);
            front.advance(take);
            remaining -= take;
            if front.remaining() > 0 {
                self.segments.push_front(front);
            }
        }
        Poll::Ready(Ok(out.freeze()))
    }

    fn poll_read_more(&mut self, cx: &mut Context<'_>) -> Poll<Result<usize, Error>> {
        if self.eof {
            return Poll::Ready(Ok(0));
        }

        let to_read = match self.remaining_limit {
            Some(0) => {
                self.eof = true;
                return Poll::Ready(Ok(0));
            }
            Some(rem) => rem.min(self.read_chunk_size),
            None => self.read_chunk_size,
        };

        let mut chunk = BytesMut::new();
        chunk.resize(to_read, 0);

        let n = match Pin::new(&mut self.reader).poll_read(cx, &mut chunk[..]) {
            Poll::Ready(Ok(n)) => n,
            Poll::Ready(Err(e)) => return Poll::Ready(Err(Error::Io(e))),
            Poll::Pending => return Poll::Pending,
        };

        if n == 0 {
            self.eof = true;
            return Poll::Ready(Ok(0));
        }

        if let Some(rem) = self.remaining_limit.as_mut() {
            *rem = rem.saturating_sub(n);
            if *rem == 0 {
                self.eof = true;
            }
        }

        chunk.truncate(n);
        self.segments.push_back(chunk.freeze());
        Poll::Ready(Ok(n))
    }
}

/// A streaming protobuf field reader backed by [`AsyncInput`].
///
/// This is a low-level building block used by async/streaming lazy message implementations.
/// It parses the wire format sequentially and yields raw fields (`Field<Bytes>`).
pub struct AsyncFieldReader<R> {
    input: AsyncInput<R>,
    terminated: bool,
}

/// Shared, poll-driven parser state for a single message.
///
/// This is the async/streaming counterpart of `lazy_slice_parser::MessageParserStateRef`,
/// but is designed for a single message boundary (either EOF or an explicit byte limit).
///
/// The parser yields fields sequentially and invokes a user-provided callback for the fields
/// selected by `field_filter`. Fields not selected by the filter are skipped without allocating.
pub struct AsyncMessageParserStateRef<R> {
    state: Rc<RefCell<AsyncMessageParserStateInner<R>>>,
}

impl<R> Clone for AsyncMessageParserStateRef<R> {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
        }
    }
}

struct AsyncMessageParserStateInner<R> {
    field_reader: AsyncFieldReader<R>,
    field_filter: Rc<dyn Fn(u32) -> bool>,
    field_update_callback: Rc<dyn Fn(Field<Bytes>) -> Result<(), Error>>,
}

impl<R> AsyncMessageParserStateRef<R>
where
    R: AsyncRead + Unpin,
{
    /// Create a new parser state from an async reader.
    ///
    /// `field_filter` decides which fields are kept (decoded into `Field<Bytes>` and passed to
    /// the callback). Fields not kept are skipped efficiently.
    pub fn create<F, G>(reader: R, limit: Option<usize>, field_filter: G, field_update_callback: F) -> Self
    where
        F: Fn(Field<Bytes>) -> Result<(), Error> + 'static,
        G: Fn(u32) -> bool + 'static,
    {
        let field_reader = match limit {
            Some(limit) => AsyncFieldReader::with_limit(reader, limit),
            None => AsyncFieldReader::new(reader),
        };
        let inner = AsyncMessageParserStateInner {
            field_reader,
            field_filter: Rc::new(field_filter),
            field_update_callback: Rc::new(field_update_callback),
        };
        Self {
            state: Rc::new(RefCell::new(inner)),
        }
    }

    /// Parse and process exactly one field (if available), then return whether progress was made.
    ///
    /// - `Ok(true)`: one kept field was parsed and the update callback was invoked.
    /// - `Ok(false)`: end-of-message reached (no more fields available).
    /// - `Pending`: more input is needed.
    pub fn poll_parse_one_field_with_callback(
        &self,
        cx: &mut Context<'_>,
    ) -> Poll<Result<bool, Error>> {
        // Poll the next field with only a short mutable borrow.
        let next = {
            let mut state = self.state.borrow_mut();
            let filter = state.field_filter.clone();
            state
                .field_reader
                .poll_next_field_filtered(cx, move |n| (filter)(n))
        };

        let next = match next {
            Poll::Ready(r) => r,
            Poll::Pending => return Poll::Pending,
        }?;

        let Some(field) = next else {
            return Poll::Ready(Ok(false));
        };

        // Invoke the callback without holding a mutable borrow of the state.
        let callback = {
            let state = self.state.borrow();
            state.field_update_callback.clone()
        };
        (callback)(field)?;
        Poll::Ready(Ok(true))
    }

    /// Parse fields until `condition()` becomes true, or end-of-message is reached.
    pub fn poll_parse_until_with_callback<C>(
        &self,
        cx: &mut Context<'_>,
        mut condition: C,
    ) -> Poll<Result<(), Error>>
    where
        C: FnMut() -> bool,
    {
        while !condition() {
            let progressed = match self.poll_parse_one_field_with_callback(cx) {
                Poll::Ready(r) => r?,
                Poll::Pending => return Poll::Pending,
            };
            if !progressed {
                break;
            }
        }
        Poll::Ready(Ok(()))
    }
}


impl<R> AsyncFieldReader<R>
where
    R: AsyncRead + Unpin,
{
    /// Read fields until EOF (message boundary is the reader's EOF).
    pub fn new(reader: R) -> Self {
        Self {
            input: AsyncInput::new(reader),
            terminated: false,
        }
    }

    /// Read fields up to `limit` bytes.
    pub fn with_limit(reader: R, limit: usize) -> Self {
        Self {
            input: AsyncInput::with_limit(reader, limit),
            terminated: limit == 0,
        }
    }

    /// Returns `true` if the message is fully terminated and no more fields are available.
    pub fn is_terminated(&self) -> bool {
        self.terminated
    }

    /// Poll for the next field.
    ///
    /// - `Poll::Pending` if more input is needed.
    /// - `Poll::Ready(Ok(Some(field)))` when a field is decoded.
    /// - `Poll::Ready(Ok(None))` on end-of-message.
    pub fn poll_next_field(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<Result<Option<Field<Bytes>>, Error>> {
        self.poll_next_field_filtered(cx, |_field_number| true)
    }

    /// Poll for the next field, allowing the caller to skip (drop) fields without allocating.
    ///
    /// The `keep` predicate is called with the decoded field number. If it returns `false`,
    /// the field is skipped:
    /// - varint/fixed values are read and discarded
    /// - length-delimited values are advanced over without coalescing into an owned buffer
    pub fn poll_next_field_filtered<F>(
        &mut self,
        cx: &mut Context<'_>,
        mut keep: F,
    ) -> Poll<Result<Option<Field<Bytes>>, Error>>
    where
        F: FnMut(u32) -> bool,
    {
        loop {
            if self.terminated {
                return Poll::Ready(Ok(None));
            }

            let tag_varint = match self.poll_read_varint(cx)? {
                Poll::Ready(Some(v)) => v,
                Poll::Ready(None) => {
                    self.terminated = true;
                    return Poll::Ready(Ok(None));
                }
                Poll::Pending => return Poll::Pending,
            };

            let tag = Tag::from_encoded(tag_varint)?;
            let field_number = tag.field_number;

            if !keep(field_number.as_u32()) {
                // Skip without allocating.
                match tag.wire_type {
                    WireType::Varint => {
                        let _ = match self.poll_read_required_varint(cx)? {
                            Poll::Ready(v) => v,
                            Poll::Pending => return Poll::Pending,
                        };
                    }
                    WireType::Int32 => {
                        let _ = match self.input.poll_ensure(cx, 4)? {
                            Poll::Ready(()) => (),
                            Poll::Pending => return Poll::Pending,
                        };
                        self.input.advance(4);
                    }
                    WireType::Int64 => {
                        let _ = match self.input.poll_ensure(cx, 8)? {
                            Poll::Ready(()) => (),
                            Poll::Pending => return Poll::Pending,
                        };
                        self.input.advance(8);
                    }
                    WireType::Len => {
                        let len_varint = match self.poll_read_required_varint(cx)? {
                            Poll::Ready(v) => v,
                            Poll::Pending => return Poll::Pending,
                        };
                        let len_u64 = len_varint.to_uint64();
                        let len: usize = len_u64.try_into().map_err(|_| {
                            Error::InvalidWireFormat("Length-delimited size out of range".to_string())
                        })?;
                        if len > MAX_LEN_DELIMITED_SIZE {
                            return Poll::Ready(Err(Error::InvalidWireFormat(
                                "Length-delimited size exceeds maximum allowed".to_string(),
                            )));
                        }
                        let _ = match self.input.poll_ensure(cx, len)? {
                            Poll::Ready(()) => (),
                            Poll::Pending => return Poll::Pending,
                        };
                        self.input.advance(len);
                    }
                    WireType::SGroup | WireType::EGroup => {
                        return Poll::Ready(Err(Error::InvalidWireFormat(
                            "Group wire types are not supported".to_string(),
                        )));
                    }
                }
                continue;
            }

            // Keep: read the value and return it.
            let value = match tag.wire_type {
                WireType::Varint => {
                    let v = match self.poll_read_required_varint(cx)? {
                        Poll::Ready(v) => v,
                        Poll::Pending => return Poll::Pending,
                    };
                    FieldValue::Varint(v)
                }
                WireType::Int32 => {
                    let bytes = match self.input.poll_take_bytes(cx, 4)? {
                        Poll::Ready(b) => b,
                        Poll::Pending => return Poll::Pending,
                    };
                    let mut arr = [0u8; 4];
                    arr.copy_from_slice(bytes.as_ref());
                    FieldValue::I32(arr)
                }
                WireType::Int64 => {
                    let bytes = match self.input.poll_take_bytes(cx, 8)? {
                        Poll::Ready(b) => b,
                        Poll::Pending => return Poll::Pending,
                    };
                    let mut arr = [0u8; 8];
                    arr.copy_from_slice(bytes.as_ref());
                    FieldValue::I64(arr)
                }
                WireType::Len => {
                    let len_varint = match self.poll_read_required_varint(cx)? {
                        Poll::Ready(v) => v,
                        Poll::Pending => return Poll::Pending,
                    };
                    let len_u64 = len_varint.to_uint64();
                    let len: usize = len_u64.try_into().map_err(|_| {
                        Error::InvalidWireFormat("Length-delimited size out of range".to_string())
                    })?;
                    if len > MAX_LEN_DELIMITED_SIZE {
                        return Poll::Ready(Err(Error::InvalidWireFormat(
                            "Length-delimited size exceeds maximum allowed".to_string(),
                        )));
                    }
                    let bytes = match self.input.poll_take_bytes(cx, len)? {
                        Poll::Ready(b) => b,
                        Poll::Pending => return Poll::Pending,
                    };
                    FieldValue::Len(bytes)
                }
                WireType::SGroup | WireType::EGroup => {
                    return Poll::Ready(Err(Error::InvalidWireFormat(
                        "Group wire types are not supported".to_string(),
                    )));
                }
            };

            return Poll::Ready(Ok(Some(Field::new(field_number, value))));
        }
    }

    fn poll_read_required_varint(&mut self, cx: &mut Context<'_>) -> Poll<Result<Varint, Error>> {
        match self.poll_read_varint(cx)? {
            Poll::Ready(Some(v)) => Poll::Ready(Ok(v)),
            Poll::Ready(None) => Poll::Ready(Err(Error::InvalidWireFormat(
                "Unexpected EOF while reading varint".to_string(),
            ))),
            Poll::Pending => Poll::Pending,
        }
    }

    /// Read a varint from the input.
    ///
    /// Returns `Ok(None)` if no bytes are available and the message has ended.
    fn poll_read_varint(&mut self, cx: &mut Context<'_>) -> Poll<Result<Option<Varint>, Error>> {
        let mut value: u64 = 0;
        let mut shift: u32 = 0;
        let mut read_any = false;

        for _ in 0..MAX_VARINT_BYTES_LOCAL {
            // Ensure at least one byte is available (or we hit EOF).
            let _ = match self.input.poll_ensure(cx, 1)? {
                Poll::Ready(()) => (),
                Poll::Pending => return Poll::Pending,
            };

            let chunk = self.input.peek_chunk();
            if chunk.is_empty() {
                if read_any {
                    return Poll::Ready(Err(Error::InvalidWireFormat(
                        "Unexpected EOF while reading varint".to_string(),
                    )));
                }
                return Poll::Ready(Ok(None));
            }

            read_any = true;
            let byte = chunk[0];
            self.input.advance(1);

            value |= ((byte & 0x7F) as u64) << shift;

            if (byte & 0x80) == 0 {
                return Poll::Ready(Ok(Some(Varint::from_uint64(value))));
            }

            shift = shift.saturating_add(7);
        }

        Poll::Ready(Err(Error::InvalidWireFormat(
            "Varint exceeds maximum length of 10 bytes".to_string(),
        )))
    }
}

