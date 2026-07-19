//! Protobuf `map<K, V>` field catalog.
//!
//! | Submodule | Contents |
//! |---|---|
//! | [`entries`] | [`MapEntries`] — `cached_pair::Pair` of `Vec` ↔ `HashMap` |
//! | [`field`] | [`MapField`] — catalog wrapper + bound views |

pub(crate) mod entries;
pub(crate) mod field;

pub use entries::{MapEntries, MapEntriesConverter, MapEntriesIter};
pub use field::{MapField, MapFieldMut, MapFieldRef};
