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
use ::std::task::ready;
use ::std::task::{Context, Poll};

use ::protobuf_core::{DecodeOutcome, DecodeState, Field, FieldValue, ReadExtVarint};
use ::protobuf_core::{Tag, Varint, WireType};

/// Maximum allowed length-delimited size (2 GiB), matching the protobuf wire format limits.
const MAX_LEN_DELIMITED_SIZE: usize = 2 * 1024 * 1024 * 1024;
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
/// Internal to [`AsyncFieldReader`]. The buffer is stored as multiple `Bytes` segments to allow
/// zero-copy subslices when a field's bytes are fully contained in a single segment.
struct AsyncInput<R> {
    /// The underlying async I/O source (e.g. `BytesReader` for in-memory data, or a network stream).
    /// All bytes are read from this via `poll_read`.
    reader: R,

    /// When `Some(n)`, we stop reading from the reader after consuming `n` bytes total.
    /// Used for length-delimited message boundaries. Decremented as we read.
    /// `None` means read until the reader returns EOF.
    remaining_limit: Option<usize>,

    /// `true` when the underlying reader will produce no more data. Set when: (a) the reader returns
    /// 0 bytes (actual EOF), (b) `remaining_limit` reaches 0, or (c) constructed with `limit == 0`.
    /// Note: this does *not* mean "all buffered data consumed" — segments may still hold data.
    reader_eof: bool,

    /// Maximum bytes to request per `poll_read` call. Larger values reduce syscalls but use more
    /// memory; smaller values yield more granular progress.
    read_chunk_size: usize,

    /// Queue of buffered byte chunks already read from the reader but not yet consumed by the
    /// parser. Oldest segment at front. Enables zero-copy when a field fits entirely in one segment.
    segments: VecDeque<Bytes>,
}

impl<R> AsyncInput<R>
where
    R: AsyncRead + Unpin,
{
    fn new(reader: R) -> Self {
        Self {
            reader,
            remaining_limit: None,
            reader_eof: false,
            read_chunk_size: DEFAULT_READ_CHUNK_SIZE,
            segments: VecDeque::new(),
        }
    }

    fn with_limit(reader: R, limit: usize) -> Self {
        Self {
            reader,
            remaining_limit: Some(limit),
            reader_eof: limit == 0,
            read_chunk_size: DEFAULT_READ_CHUNK_SIZE,
            segments: VecDeque::new(),
        }
    }

    /// Returns the number of bytes currently buffered (not yet consumed).
    fn buffered_len(&self) -> usize {
        self.segments.iter().map(|b| b.remaining()).sum()
    }

    /// Ensure at least `needed` bytes are buffered, reading from the underlying reader if needed.
    ///
    /// Semantics match `std::io::Read::read_exact` / `futures::AsyncReadExt::read_exact` / `tokio::io::AsyncReadExt::read_exact`:
    /// returns `Ok(())` when at least `needed` bytes are available, or `Err` with `UnexpectedEof` when EOF is hit before that.
    fn poll_ensure(&mut self, cx: &mut Context<'_>, needed: usize) -> Poll<Result<(), Error>> {
        while self.buffered_len() < needed {
            if self.reader_eof {
                return Poll::Ready(Err(Error::Io(::std::io::Error::new(
                    ::std::io::ErrorKind::UnexpectedEof,
                    "failed to fill buffer",
                ))));
            }
            let read = ready!(self.poll_read_more(cx)?);
            if read == 0 {
                return Poll::Ready(Err(Error::Io(::std::io::Error::new(
                    ::std::io::ErrorKind::UnexpectedEof,
                    "failed to fill buffer",
                ))));
            }
        }
        Poll::Ready(Ok(()))
    }

    /// Returns the first contiguous chunk of buffered bytes (may be empty).
    fn peek_chunk(&self) -> &[u8] {
        self.segments.front().map(|b| b.chunk()).unwrap_or(&[])
    }

    /// Consume `n` bytes from the buffer.
    fn advance(&mut self, mut n: usize) {
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
    /// Zero-copy if the bytes fit in one segment; otherwise coalesces into a single buffer.
    fn poll_take_bytes(&mut self, cx: &mut Context<'_>, len: usize) -> Poll<Result<Bytes, Error>> {
        if len == 0 {
            return Poll::Ready(Ok(Bytes::new()));
        }

        // Ensure we have enough buffered (read_exact semantics: error if EOF before full).
        ready!(self.poll_ensure(cx, len)?);

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
        if self.reader_eof {
            return Poll::Ready(Ok(0));
        }

        let to_read = match self.remaining_limit {
            Some(0) => {
                self.reader_eof = true;
                return Poll::Ready(Ok(0));
            }
            Some(rem) => rem.min(self.read_chunk_size),
            None => self.read_chunk_size,
        };

        let mut chunk = BytesMut::new();
        chunk.resize(to_read, 0);

        let n = match ready!(Pin::new(&mut self.reader).poll_read(cx, &mut chunk[..])) {
            Ok(n) => n,
            Err(e) => return Poll::Ready(Err(Error::Io(e))),
        };

        if n == 0 {
            self.reader_eof = true;
            return Poll::Ready(Ok(0));
        }

        if let Some(rem) = self.remaining_limit.as_mut() {
            *rem = rem.saturating_sub(n);
            if *rem == 0 {
                self.reader_eof = true;
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
    /// When a varint spans segment boundaries, we get `DecodeOutcome::Incomplete(state)` from
    /// `read_varint_partial`. We store the state here and continue with `read_varint_resume` on
    /// the next poll when more bytes are available.
    varint_resume_state: Option<DecodeState>,
}

/// Shared, poll-driven parser state for a single message.
///
/// This is the async/streaming counterpart of `lazy_slice_parser::MessageParserStateRef`,
/// but is designed for a single message boundary (either EOF or an explicit byte limit).
///
/// The parser yields fields sequentially and invokes the callback for each field. The callback
/// (owned by the message struct) decides which fields to record and which to drop.
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
    field_update_callback: Rc<dyn Fn(Field<Bytes>) -> Result<(), Error>>,
}

impl<R> AsyncMessageParserStateRef<R>
where
    R: AsyncRead + Unpin,
{
    /// Create a new parser state from an async reader.
    pub fn create<F>(reader: R, limit: Option<usize>, field_update_callback: F) -> Self
    where
        F: Fn(Field<Bytes>) -> Result<(), Error> + 'static,
    {
        let field_reader = match limit {
            Some(limit) => AsyncFieldReader::with_limit(reader, limit),
            None => AsyncFieldReader::new(reader),
        };
        let inner = AsyncMessageParserStateInner {
            field_reader,
            field_update_callback: Rc::new(field_update_callback),
        };
        Self {
            state: Rc::new(RefCell::new(inner)),
        }
    }

    /// Parse and process exactly one field (if available), then return whether progress was made.
    ///
    /// - `Ok(true)`: one field was parsed and the update callback was invoked.
    /// - `Ok(false)`: end-of-message reached (no more fields available).
    /// - `Pending`: more input is needed.
    pub fn poll_parse_one_field_with_callback(
        &self,
        cx: &mut Context<'_>,
    ) -> Poll<Result<bool, Error>> {
        // Poll the next field with only a short mutable borrow.
        let next = {
            let mut state = self.state.borrow_mut();
            ready!(state.field_reader.poll_next_field(cx))?
        };

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
            let progressed = ready!(self.poll_parse_one_field_with_callback(cx))?;
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
            varint_resume_state: None,
        }
    }

    /// Read fields up to `limit` bytes.
    pub fn with_limit(reader: R, limit: usize) -> Self {
        Self {
            input: AsyncInput::with_limit(reader, limit),
            terminated: limit == 0,
            varint_resume_state: None,
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
        if self.terminated {
            return Poll::Ready(Ok(None));
        }

        let tag = match ready!(self.poll_read_tag(cx)?) {
            Some(t) => t,
            None => {
                self.terminated = true;
                return Poll::Ready(Ok(None));
            }
        };
        let field_number = tag.field_number;

        let value = match tag.wire_type {
            WireType::Varint => {
                let v = ready!(self.poll_read_required_varint(cx)?);
                FieldValue::Varint(v)
            }
            WireType::Int32 => {
                let bytes = ready!(self.input.poll_take_bytes(cx, 4)?);
                let mut arr = [0u8; 4];
                arr.copy_from_slice(bytes.as_ref());
                FieldValue::I32(arr)
            }
            WireType::Int64 => {
                let bytes = ready!(self.input.poll_take_bytes(cx, 8)?);
                let mut arr = [0u8; 8];
                arr.copy_from_slice(bytes.as_ref());
                FieldValue::I64(arr)
            }
            WireType::Len => {
                let len_varint = ready!(self.poll_read_required_varint(cx)?);
                let len_u64 = len_varint.to_uint64();
                let len: usize = len_u64.try_into().map_err(|_| {
                    Error::InvalidWireFormat("Length-delimited size out of range".to_string())
                })?;
                if len > MAX_LEN_DELIMITED_SIZE {
                    return Poll::Ready(Err(Error::InvalidWireFormat(
                        "Length-delimited size exceeds maximum allowed".to_string(),
                    )));
                }
                let bytes = ready!(self.input.poll_take_bytes(cx, len)?);
                FieldValue::Len(bytes)
            }
            WireType::SGroup | WireType::EGroup => {
                return Poll::Ready(Err(Error::InvalidWireFormat(
                    "Group wire types are not supported".to_string(),
                )));
            }
        };

        Poll::Ready(Ok(Some(Field::new(field_number, value))))
    }

    /// Poll for the next protobuf tag (field number + wire type).
    ///
    /// Returns `Ok(None)` when no more bytes are available (end-of-message).
    pub fn poll_read_tag(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<Result<Option<Tag>, Error>> {
        let varint_opt = ready!(self.poll_read_varint(cx)?);
        Poll::Ready(match varint_opt {
            Some(varint) => Tag::from_encoded(varint).map(Some).map_err(Error::from),
            None => Ok(None),
        })
    }

    fn poll_read_required_varint(&mut self, cx: &mut Context<'_>) -> Poll<Result<Varint, Error>> {
        match ready!(self.poll_read_varint(cx)?) {
            Some(v) => Poll::Ready(Ok(v)),
            None => Poll::Ready(Err(Error::InvalidWireFormat(
                "Unexpected EOF while reading varint".to_string(),
            ))),
        }
    }

    /// Read a varint from the input.
    ///
    /// Returns `Ok(None)` if no bytes are available and the message has ended.
    fn poll_read_varint(&mut self, cx: &mut Context<'_>) -> Poll<Result<Option<Varint>, Error>> {
        // Ensure at least 1 byte so we can decode or detect EOF. Cheap when already buffered (one bounds check).
        if let Err(e) = ready!(self.input.poll_ensure(cx, 1)) {
            // Empty input is legal only when we are in clean state (not resuming a partial varint).
            if e.is_unexpected_eof() && self.varint_resume_state.is_none() {
                return Poll::Ready(Ok(None));
            }
            return Poll::Ready(Err(e));
        }

        let state = self.varint_resume_state.take();
        let mut chunk = self.input.peek_chunk();
        let len_before = chunk.len();
        let outcome = match state {
            Some(s) => chunk.read_varint_resume(s).map_err(Error::from),
            None => chunk.read_varint_partial().map_err(Error::from),
        };
        let consumed = len_before - chunk.len();
        self.input.advance(consumed);

        match outcome {
            Ok(DecodeOutcome::Complete(varint)) => Poll::Ready(Ok(Some(varint))),
            Ok(DecodeOutcome::Incomplete(s)) => {
                self.varint_resume_state = Some(s);
                Poll::Pending
            }
            Ok(DecodeOutcome::Empty) => {
                unreachable!("we ensure at least 1 byte before decoding; Empty cannot occur")
            }
            Err(e) => Poll::Ready(Err(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::bytes::Bytes;
    use ::std::task::Poll;

    fn noop_waker() -> ::std::task::Waker {
        unsafe fn clone(_: *const ()) -> ::std::task::RawWaker {
            ::std::task::RawWaker::new(::std::ptr::null(), &VTABLE)
        }
        unsafe fn wake(_: *const ()) {}
        unsafe fn wake_by_ref(_: *const ()) {}
        unsafe fn drop(_: *const ()) {}
        static VTABLE: ::std::task::RawWakerVTable =
            ::std::task::RawWakerVTable::new(clone, wake, wake_by_ref, drop);
        unsafe { ::std::task::Waker::from_raw(::std::task::RawWaker::new(::std::ptr::null(), &VTABLE)) }
    }

    fn poll_until_ready<T>(
        mut f: impl FnMut(&mut Context<'_>) -> Poll<Result<T, Error>>,
    ) -> Result<T, Error> {
        let waker = noop_waker();
        let mut cx = Context::from_waker(&waker);
        loop {
            match f(&mut cx) {
                Poll::Ready(r) => return r,
                Poll::Pending => continue,
            }
        }
    }

    /// Yields one byte per poll for testing varint resume across chunk boundaries.
    struct OneByteReader {
        data: Vec<u8>,
        pos: usize,
    }
    impl OneByteReader {
        fn new(data: Vec<u8>) -> Self {
            Self { data, pos: 0 }
        }
    }
    impl AsyncRead for OneByteReader {
        fn poll_read(
            mut self: Pin<&mut Self>,
            _cx: &mut Context<'_>,
            out: &mut [u8],
        ) -> Poll<Result<usize, ::std::io::Error>> {
            if self.pos >= self.data.len() {
                return Poll::Ready(Ok(0));
            }
            let n = 1.min(out.len());
            out[..n].copy_from_slice(&self.data[self.pos..self.pos + n]);
            self.pos += n;
            Poll::Ready(Ok(n))
        }
    }

    #[test]
    fn test_poll_read_varint_empty_input_returns_none() {
        let reader = BytesReader::new(Bytes::new());
        let mut field_reader = AsyncFieldReader::new(reader);
        let result = poll_until_ready(|cx| field_reader.poll_read_varint(cx));
        assert!(matches!(result, Ok(None)));
    }

    #[test]
    fn test_poll_read_varint_single_byte_varint() {
        // Varint 0 is encoded as 0x00.
        let reader = BytesReader::new(Bytes::from_static(&[0x00]));
        let mut field_reader = AsyncFieldReader::new(reader);
        let result = poll_until_ready(|cx| field_reader.poll_read_varint(cx));
        let varint = result.unwrap().unwrap();
        assert_eq!(varint.to_uint64(), 0);
    }

    #[test]
    fn test_poll_read_varint_multi_byte_complete() {
        // Varint 150 = 0x96 0x01.
        let reader = BytesReader::new(Bytes::from_static(&[0x96, 0x01]));
        let mut field_reader = AsyncFieldReader::new(reader);
        let result = poll_until_ready(|cx| field_reader.poll_read_varint(cx));
        let varint = result.unwrap().unwrap();
        assert_eq!(varint.to_uint64(), 150);
    }

    #[test]
    fn test_poll_read_varint_incomplete_then_resume() {
        // Varint 150 = 0x96 0x01. Feed one byte per poll to force Incomplete then resume.
        let reader = OneByteReader::new(vec![0x96, 0x01]);
        let mut field_reader = AsyncFieldReader::new(reader);
        let waker = noop_waker();
        let mut cx = Context::from_waker(&waker);

        // First poll: only 0x96 is available, varint is incomplete -> Pending.
        let first = field_reader.poll_read_varint(&mut cx);
        assert!(matches!(first, Poll::Pending));

        // Second poll: 0x01 is available, resume completes -> Ready(Some(150)).
        let second = field_reader.poll_read_varint(&mut cx);
        let varint = match second {
            Poll::Ready(Ok(Some(v))) => v,
            other => panic!("expected Ready(Ok(Some(_))), got {:?}", other),
        };
        assert_eq!(varint.to_uint64(), 150);
    }
}
