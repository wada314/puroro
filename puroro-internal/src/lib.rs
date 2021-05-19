// For ResultHelper::flatten.
#![allow(unstable_name_collisions)]
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
};

pub use helpers::{
    FieldClone, FieldDeserFromIter, FieldDeserFromSlice, FieldNew, FieldSer, FieldTakeOrInit,
    MapEntry,
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
