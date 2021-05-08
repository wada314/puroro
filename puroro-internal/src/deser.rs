mod iter;
mod slice;

pub use iter::{BytesIter, DeserializableMessageFromIter, FromIterToFromSlice};
pub use slice::{BytesSlice, DeserializableMessageFromSlice};
