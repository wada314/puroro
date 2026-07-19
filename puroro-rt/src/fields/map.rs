//! Protobuf `map<K, V>` field catalog.
//!
//! | Submodule | Contents |
//! |---|---|
//! | [`entries`] | [`MapEntries`] — physical `HashMap` of elements |
//! | [`entry`] | map-entry wire encode / decode (`key=1`, `value=2`) |
//! | [`field`] | [`MapField<K, V, FIELD, A>`] — `K: MapKey`, `V: RepeatedElement` |

pub(crate) mod entries;
pub(crate) mod entry;
pub(crate) mod field;

pub use entries::{MapEntries, MapEntriesIter};
pub use field::{MapField, MapFieldMut, MapFieldRef};
