#![allow(unstable_name_collisions)] // For ResultHelper::flatten.
#![cfg_attr(feature = "puroro-nightly", feature(generic_associated_types))]
#![cfg_attr(feature = "puroro-nightly", feature(min_type_alias_impl_trait))]
#![allow(incomplete_features)]

pub mod deser;
pub mod helpers;
pub mod internal_data;
pub mod ser;
pub mod tags;
pub mod types;
pub mod variant;

pub use ::puroro::{ErrorKind, PuroroError, Result};
pub use internal_data::{
    InternalDataForBumpaloStruct, InternalDataForNormalStruct, InternalDataForSliceViewStruct,
    SourceLdSlices,
};

pub use helpers::{
    FieldClone, FieldDeserFromIter, FieldDeserFromSlice, FieldNew, FieldSer, FieldTakeOrInit,
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
