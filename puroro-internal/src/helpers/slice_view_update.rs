use crate::types::{FieldData, SliceViewRepeatedField, SliceViewScalarField};
use crate::{ErrorKind, Result};

pub trait SliceViewUpdate {
    fn update(&mut self, field: FieldData<&[u8]>) -> Result<()>;
}


