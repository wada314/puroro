mod iter;
mod slice;

pub use iter::{BytesIter, DeserializableFromIter, DeserializableMessageFromIter};
pub use slice::{BytesSlice, DeserializableFromSlice, DeserializableMessageFromSlice};
