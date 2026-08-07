//! Message-body encode surface used by the field catalog and generated code.
//!
//! This is intentionally **not** part of the user-facing [`puroro::Message`](::puroro::Message)
//! trait. Library users call [`Message::encode`](::puroro::Message::encode) /
//! [`Message::encode_to_vec`](::puroro::Message::encode_to_vec); generated
//! impls forward those to [`encode_message`] / [`encode_message_to_vec`].

use ::bytes::BufMut;
use ::core::cell::OnceCell;
use ::core::ptr;
use ::std::collections::HashMap;

/// Scratch state for one root encode (`encode` / equivalent).
///
/// Nested LEN encode needs each child's [`MessageEncode::encoded_len`] both when
/// sizing a parent and when writing that child's length prefix. Passing this
/// context down the encode stack avoids O(depth²) re-walks without TLS / globals.
///
/// The body-length map is allocated lazily on first nested encode so flat
/// messages pay only for an empty [`OnceCell`].
#[derive(Debug, Default)]
pub struct EncodeCtx {
    /// `message as *const ()` → body wire length (no LEN prefix).
    body_lens: OnceCell<HashMap<*const (), usize>>,
}

impl EncodeCtx {
    /// Empty context; the length cache is created on first nested encode.
    #[inline]
    pub fn new() -> Self {
        Self {
            body_lens: OnceCell::new(),
        }
    }

    /// Cached `message.encoded_len(self)`, inserting on miss.
    #[inline]
    pub fn body_len_for<M: MessageEncode>(&mut self, message: &M) -> usize {
        let key = ptr::from_ref(message).cast::<()>();
        if let Some(&n) = self.body_lens.get().and_then(|m| m.get(&key)) {
            return n;
        }
        let n = message.encoded_len(self);
        self.body_lens.get_or_init(|| HashMap::with_capacity(32));
        self.body_lens.get_mut().unwrap().insert(key, n);
        n
    }
}

/// Wire encode for a message body (tag-free fields + unknown bytes).
///
/// Implemented by generated messages and hand-written test / bench messages.
/// Nested-message catalog code ([`ProtoMessage`](crate::ProtoMessage)) bounds on
/// this trait, not on [`puroro::Message`](::puroro::Message).
///
/// User-facing [`Message::encode`](::puroro::Message::encode) /
/// [`Message::encode_to_vec`](::puroro::Message::encode_to_vec) are thin
/// wrappers around [`encode_message`] / [`encode_message_to_vec`].
pub trait MessageEncode {
    /// Exact number of bytes this message body occupies on the wire.
    /// Must be consistent with [`encode_raw`](Self::encode_raw).
    fn encoded_len(&self, ctx: &mut EncodeCtx) -> usize;

    /// Writes the message body to `buf` without a framing length prefix.
    fn encode_raw<B: BufMut>(&self, ctx: &mut EncodeCtx, buf: &mut B);
}

/// Appends the encoded message body to `buf` (creates a fresh [`EncodeCtx`]).
///
/// Does not reserve capacity on `buf`; the caller owns allocation strategy.
/// Prefer [`encode_message_to_vec`] when allocating a fresh `Vec`.
#[inline]
pub fn encode_message<M: MessageEncode, B: BufMut>(message: &M, buf: &mut B) {
    let mut ctx = EncodeCtx::new();
    message.encode_raw(&mut ctx, buf);
}

/// Encodes `message` into a new `Vec<u8>`, sized with [`MessageEncode::encoded_len`].
///
/// The length walk fills [`EncodeCtx`] so nested LEN prefixes reuse cached body
/// lengths during [`MessageEncode::encode_raw`].
#[inline]
pub fn encode_message_to_vec<M: MessageEncode>(message: &M) -> Vec<u8> {
    let mut ctx = EncodeCtx::new();
    let mut v = Vec::with_capacity(message.encoded_len(&mut ctx));
    message.encode_raw(&mut ctx, &mut v);
    v
}
