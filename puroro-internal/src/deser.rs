mod iter;
mod slice;

pub use iter::{BytesIter, DeserializableMessageFromIter};
pub use slice::{BytesSlice, DeserializableFromSlice, DeserializableMessageFromSlice};
