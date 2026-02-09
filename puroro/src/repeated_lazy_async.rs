//! Async/streaming lazy wrapper for repeated fields.
//!
//! This module mirrors `repeated_lazy_slice`, but is driven by the async/streaming
//! parser state using async methods.

use crate::error::Error;
use crate::lazy_async::AsyncMessageParserStateRef;
use crate::repeated::Repeated;
use ::allocator_extras::Allocator;
use ::futures_io::AsyncRead;
use ::once_list2::OnceListWithTailLen as OnceList;

/// An async, on-demand parsing adapter over a repeated field.
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
    pub fn new(
        parent_parser_state: AsyncMessageParserStateRef<R>,
        list: &'message OnceList<T, A>,
    ) -> Self {
        Self {
            parent_parser_state,
            list,
        }
    }

    /// Async length of the repeated field (requires fully parsing the message).
    pub async fn len_async(&self) -> Result<usize, Error> {
        self.parent_parser_state
            .parse_until_with_callback(|| false)
            .await?;
        Ok(self.list.len())
    }

    /// Async check whether the repeated field is empty (parses just enough to know).
    pub async fn is_empty_async(&self) -> Result<bool, Error> {
        while self.list.len() < 1 {
            let progressed = self.parent_parser_state.parse_one_field_with_callback().await?;
            if !progressed {
                break;
            }
        }
        Ok(self.list.first().is_none())
    }

    /// Async get an element by index, parsing on-demand until it is available or EOF is reached.
    pub async fn get_async(&self, index: usize) -> Result<Option<T>, Error> {
        let needed = index.saturating_add(1);
        while self.list.len() < needed {
            let progressed = self.parent_parser_state.parse_one_field_with_callback().await?;
            if !progressed {
                break;
            }
        }
        Ok(self.list.iter().nth(index).cloned())
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
        // Note: this iterator does not drive parsing. Use async APIs (len_async, get_async) for on-demand parsing.
        let boxed = ::allocator_api2::boxed::Box::new(self.list.iter().cloned());
        let boxed_dyn: ::allocator_api2::boxed::Box<dyn Iterator<Item = T> + 'message> =
            ::allocator_api2::unsize_box!(boxed);
        boxed_dyn
    }
}
