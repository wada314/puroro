//! Protobuf `map<K, V>` field catalog.
//!
//! | Submodule | Contents |
//! |---|---|
//! | [`entry`] | map-entry wire encode / decode (`key=1`, `value=2`) |
//! | [`field`] | [`MapField<K, V, FIELD, A>`] — `K: MapKey`, `V: RepeatedElement` |

pub(crate) mod entry;
pub(crate) mod field;

pub use field::{MapField, MapFieldMut, MapFieldRef};
