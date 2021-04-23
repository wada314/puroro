mod bytes;
mod iters;
mod slice;
use iters::{Bytes, Chars, Variants};

pub use bytes::{BytesIter, DeserializableFromBytes, DeserializeMessageFromBytesEventHandler};
pub use slice::{BytesSlice, DeserializableFromSlice, DeserializeMessageFromSliceEventHandler};
