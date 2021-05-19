mod iter;
mod slice;

pub use iter::{DeserializableMessageFromIter, LdIter, Variants};
pub use slice::{DeserializableMessageFromSlice, FromIterToFromSlice, LdSlice};
