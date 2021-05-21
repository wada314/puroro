use std::marker::PhantomData;

use crate::tags;
use crate::types::{FieldData, SliceViewField};
use crate::variant::VariantTypeTag;
use crate::InternalDataForSliceViewStruct;
use crate::{ErrorKind, Result, ResultHelper};
use ::itertools::{Either, Itertools};
use ::puroro::RepeatedField;

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
    pub fn iter_impl(
        &self,
    ) -> impl '_ + Iterator<Item = <tags::Int32 as VariantTypeTag>::NativeType> {
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
                .into_iter()) // Result<Iterator<Item=Result<<tags::Int32 as VariantTypeTag>::NativeType>>>
            })
            .map(|rrval| rrval.flatten())
            .flatten_ok()
            .map(|rrval| rrval.flatten())
            .map(|result| result.unwrap())
    }
}

impl<'slice, 'p> RepeatedField<'p, <tags::Int32 as VariantTypeTag>::NativeType>
    for RepeatedSliceViewField<'slice, 'p, tags::Int32>
{
    fn for_each<F>(&self, f: F)
    where
        F: FnMut(<tags::Int32 as VariantTypeTag>::NativeType),
    {
        self.iter_impl().for_each(f)
    }
    fn boxed_iter(
        &self,
    ) -> Box<dyn '_ + Iterator<Item = <tags::Int32 as VariantTypeTag>::NativeType>> {
        Box::new(self.iter_impl())
    }

    type Iter = impl Iterator<Item = <tags::Int32 as VariantTypeTag>::NativeType>;
    fn iter(&'p self) -> Self::Iter {
        self.iter_impl()
    }
}
