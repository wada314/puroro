mod iter;
mod slice;

pub use iter::{LdIter, MergeableMessageFromIter, Variants};
pub use slice::{DeserializableMessageFromSlice, FromIterToFromSlice, LdSlice};
