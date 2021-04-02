use std::iter::FromIterator;

use crate::Result;
use ::puroro::tags;
use ::puroro_serializer::deserializer::stream::{
    DelayedLengthDelimitedDeserializer, Field, LengthDelimitedDeserializer,
};
use ::puroro_serializer::variant;
use puroro::{tags::FieldTypeTag, Deserializable, Mergeable, PuroroError};

trait FromFields {
    type Output;
    fn from_field(field: &Field<DelayedLengthDelimitedDeserializer>) -> Result<Self::Output>;
    // Empty fields are okay, return Ok(None) in that case.
    // Note this is a different behavior with the empty packed variants!
    fn from_fields<'a, I>(fields: I) -> Result<Option<Self::Output>>
    where
        I: Iterator<Item = &'a Field<DelayedLengthDelimitedDeserializer>>,
    {
        fields
            .last()
            .map(|field| Self::from_field(field))
            .transpose()
    }
}

// Whitelisting the type tags which can be convertible from a variant.
// We cannot reuse variant::VariantType trait impl. Instead,
// we need to define this trait IN THIS MODULE to avoid the compiler's
// impl conflict checker.
trait FromVariantField {}
impl FromVariantField for tags::SInt32 {}
impl FromVariantField for tags::SInt64 {}
impl FromVariantField for tags::Int32 {}
impl FromVariantField for tags::Int64 {}
impl FromVariantField for tags::UInt32 {}
impl FromVariantField for tags::UInt64 {}
impl FromVariantField for tags::Bool {}

impl<T> FromFields for T
where
    T: FromVariantField,
    T: FieldTypeTag,
    T: variant::VariantType<NativeType = <T as FieldTypeTag>::SingularRustType>,
{
    type Output = <Self as FieldTypeTag>::SingularRustType;

    fn from_field(field: &Field<DelayedLengthDelimitedDeserializer>) -> Result<Self::Output> {
        match field {
            Field::Variant(v) => Ok(v.to_native::<Self>()?),
            Field::LengthDelimited(dldd) => {
                // Extract the last field
                dldd.deserialize_as_variants()
                    .last()
                    .unwrap_or(Err(PuroroError::ZeroLengthPackedField))
                    .and_then(|v| v.to_native::<Self>())
            }
            _ => Err(PuroroError::InvalidWireType),
        }
    }
}

impl<T> FromFields for tags::String<T>
where
    T: FromIterator<char>,
{
    type Output = T;

    fn from_field(field: &Field<DelayedLengthDelimitedDeserializer>) -> Result<Self::Output> {
        if let Field::LengthDelimited(dldd) = field {
            dldd.deserialize_as_chars()
                .collect::<Result<Self::Output>>()
        } else {
            Err(PuroroError::InvalidWireType)
        }
    }
}

impl<T> FromFields for tags::Message<T>
where
    T: Deserializable + Mergeable,
{
    type Output = T;
    fn from_field(field: &Field<DelayedLengthDelimitedDeserializer>) -> Result<Self::Output> {
        if let Field::LengthDelimited(dldd) = field {
            T::from_bytes(dldd.into_iter())
        } else {
            Err(PuroroError::InvalidWireType)
        }
    }

    // Special implementation for Message type. We need to merge the all messages.
    fn from_fields<'a, I>(fields: I) -> Result<Option<Self::Output>>
    where
        I: Iterator<Item = &'a Field<DelayedLengthDelimitedDeserializer>>,
    {
        let mut results_iter = fields.map(Self::from_field);
        let opt_first = results_iter.next();
        opt_first
            .map(|first_result| {
                let mut merged = first_result?;
                for result in results_iter {
                    merged = merged.merge(&(result?))?;
                }
                Ok(merged)
            })
            .transpose()
    }
}
