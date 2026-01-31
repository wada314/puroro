//! Async/streaming lazy wrapper for repeated fields.
//!
//! This module will mirror `repeated_lazy_slice`, but will be driven by the async/streaming
//! parser state and use poll-based progress.

use crate::error::Error;
use crate::lazy_async::AsyncMessageParserStateRef;
use crate::repeated::Repeated;
use ::allocator_extras::Allocator;
use ::futures_io::AsyncRead;
use ::once_list2::OnceListWithTailLen as OnceList;
use ::std::task::{Context, Poll};

/// A poll-driven, on-demand parsing adapter over a repeated field.
///
/// This is intended to be returned by async-lazy message repeated field getters.
pub struct LazyRepeatedAsync<'message, T, A, R>
where
    T: Clone + 'message,
    A: Allocator + Clone + 'message,
{
    parent_parser_state: AsyncMessageParserStateRef<R>,
    list: &'message OnceList<T, A>,
}

impl<'message, T, A, R> LazyRepeatedAsync<'message, T, A, R>
where
    T: Clone + 'message,
    A: Allocator + Clone + 'message,
    R: AsyncRead + Unpin,
{
    /// Create a new `LazyRepeatedAsync` adapter.
    #[inline]
    pub fn new(parent_parser_state: AsyncMessageParserStateRef<R>, list: &'message OnceList<T, A>) -> Self {
        Self {
            parent_parser_state,
            list,
        }
    }

    fn poll_ensure_at_least(
        &self,
        cx: &mut Context<'_>,
        needed: usize,
    ) -> Poll<Result<(), Error>> {
        while self.list.len() < needed {
            let progressed = match self.parent_parser_state.poll_parse_one_field_with_callback(cx) {
                Poll::Ready(r) => r?,
                Poll::Pending => return Poll::Pending,
            };
            if !progressed {
                break;
            }
        }
        Poll::Ready(Ok(()))
    }

    fn poll_ensure_fully_parsed(&self, cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        self.parent_parser_state
            .poll_parse_until_with_callback(cx, || false)
    }

    /// Poll the length of the repeated field (requires fully parsing the message).
    pub fn poll_len(&self, cx: &mut Context<'_>) -> Poll<Result<usize, Error>> {
        let _ = match self.poll_ensure_fully_parsed(cx)? {
            Poll::Ready(()) => (),
            Poll::Pending => return Poll::Pending,
        };
        Poll::Ready(Ok(self.list.len()))
    }

    /// Poll whether the repeated field is empty (parses just enough to know).
    pub fn poll_is_empty(&self, cx: &mut Context<'_>) -> Poll<Result<bool, Error>> {
        let _ = match self.poll_ensure_at_least(cx, 1)? {
            Poll::Ready(()) => (),
            Poll::Pending => return Poll::Pending,
        };
        Poll::Ready(Ok(self.list.first().is_none()))
    }

    /// Poll and get an element by index, parsing on-demand until it is available or EOF is reached.
    pub fn poll_get(&self, cx: &mut Context<'_>, index: usize) -> Poll<Result<Option<T>, Error>> {
        let _ = match self.poll_ensure_at_least(cx, index.saturating_add(1))? {
            Poll::Ready(()) => (),
            Poll::Pending => return Poll::Pending,
        };
        Poll::Ready(Ok(self.list.iter().nth(index).cloned()))
    }
}

#[allow(missing_docs)]
impl<'message, T, A, R> Repeated<'message> for LazyRepeatedAsync<'message, T, A, R>
where
    T: Clone + 'message,
    A: Allocator + Clone + 'message,
{
    type Item = T;

    fn len(&self) -> usize {
        // Synchronous length is not available without a Context, so we conservatively return
        // what is currently available.
        self.list.len()
    }

    fn is_empty(&self) -> bool {
        self.list.first().is_none()
    }

    fn get(&self, index: usize) -> Option<Self::Item> {
        self.list.iter().nth(index).cloned()
    }

    fn iter_box(&self) -> ::allocator_api2::boxed::Box<dyn Iterator<Item = Self::Item> + 'message> {
        // Note: this iterator does not drive parsing. Use poll-based APIs for on-demand parsing.
        let boxed = ::allocator_api2::boxed::Box::new(self.list.iter().cloned());
        let boxed_dyn: ::allocator_api2::boxed::Box<dyn Iterator<Item = T> + 'message> =
            ::allocator_api2::unsize_box!(boxed);
        boxed_dyn
    }
}

