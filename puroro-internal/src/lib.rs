#![allow(unstable_name_collisions)] // For ResultHelper::flatten.
#![cfg_attr(feature = "puroro-nightly", feature(generic_associated_types))]
#![cfg_attr(feature = "puroro-nightly", feature(min_type_alias_impl_trait))]
#![allow(incomplete_features)]

pub mod deser;
pub mod field_type_gen;
pub mod helpers;
pub mod internal_data;
pub mod ser;
pub mod types;
pub mod variant;

// Re-exporting library modules
pub use ::puroro::{bumpalo, hashbrown};

pub use ::puroro::{ErrorKind, PuroroError, Result};
pub use internal_data::{
    InternalDataForBumpaloStruct, InternalDataForNormalStruct, InternalDataForSliceViewStruct,
    SliceSource, SourceLdSlices,
};

pub use helpers::{
    FieldClone, FieldMergeFromIter, FieldMergeFromSlice, FieldNew, FieldSer, FieldTakeOrInit,
    MapEntryForNormalImpl, MapEntryForSliceViewImpl, RepeatedSliceViewField,
};
pub use types::SliceViewField;

trait ResultHelper<T, E> {
    fn flatten(self) -> std::result::Result<T, E>;
}
impl<T, E> ResultHelper<T, E> for std::result::Result<std::result::Result<T, E>, E> {
    fn flatten(self) -> std::result::Result<T, E> {
        self.and_then(|x| x)
    }
}

pub trait MapEntry {
    type KeyType;
    type ValueType;
    // Note: &mut self, not self. The implementor may need to use std::mem::replace.
    fn take_kv(&mut self) -> (Self::KeyType, Self::ValueType);
    fn from_kv(key: &Self::KeyType, value: &Self::ValueType) -> Self;
}
