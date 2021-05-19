use std::marker::PhantomData;

use crate::deser::LdSlice;
use crate::tags;
use crate::types::{FieldData, SliceViewField};
use crate::InternalDataForSliceViewStruct;
use crate::{ErrorKind, Result, ResultHelper};
use ::itertools::Itertools;
use itertools::Either;

#[derive(Debug, Clone)]
pub struct RepeatedSliceViewField<'slice, 'p, TypeTag>
where
    TypeTag: tags::FieldTypeTag,
{
    field: &'p Option<SliceViewField<'slice>>,
    field_number: usize,
    internal_data: &'p InternalDataForSliceViewStruct<'slice, 'p>,
    phantom: PhantomData<TypeTag>,
}

impl<'slice, 'p> RepeatedSliceViewField<'slice, 'p, tags::Int32> {
    pub fn iter(&self) -> impl '_ + Iterator<Item = i32> {
        self.internal_data
            .field_data_iter(self.field_number, self.field)
            .map_ok(|field| -> Result<_ /* impl Iterator<Item=Result<i32>> */> {
                Ok(match field {
                    FieldData::Variant(variant) => {
                        Either::Left(std::iter::once(variant.to_native::<tags::Int32>()))
                    }
                    FieldData::LengthDelimited(ld_slice) => Either::Right(
                        ld_slice
                            .variants()
                            .map_ok(|variant| variant.to_native::<tags::Int32>())
                            .map(|rrval| rrval.flatten()),
                    ),
                    _ => Err(ErrorKind::UnexpectedWireType)?,
                }
                .into_iter()) // Result<Iterator<Item=Result<i32>>>
            })
            .map(|rrval| rrval.flatten())
            .flatten_ok()
            .map(|rrval| rrval.flatten())
            .map(|result| result.unwrap())
    }
}
