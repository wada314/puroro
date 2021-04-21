mod bytes;
mod iters;
mod slice;
use iters::{BytesIterator, CharsIterator, VariantsIterator};

pub use bytes::{BytesIter, DeserializableFromBytes, DeserializeMessageFromBytesEventHandler};
pub use slice::{BytesSlice, DeserializableFromSlice, DeserializeMessageFromSliceEventHandler};
