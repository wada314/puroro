use ::itertools::Either;
use ::puroro::tags;
use ::puroro::{Deserializable, Mergeable, Message, PuroroError, RepeatedFieldHandler};
use ::puroro_serializer::deserializer::stream::{
    DelayedLengthDelimitedDeserializer, Deserializer, Field, LengthDelimitedDeserializer,
    MessageHandler,
};
use ::puroro_serializer::variant;
use ::puroro_serializer::variant::Variant;
use std::collections::HashMap;

type Result<T> = std::result::Result<T, PuroroError>;

#[derive(Debug, Clone)]
pub struct UnknownMessage {
    fields: HashMap<usize, Vec<Field<DelayedLengthDelimitedDeserializer>>>,
}

impl UnknownMessage {
    fn new() -> Self {
        Self {
            fields: HashMap::new(),
        }
    }

    fn get_last_variant_field(&self, field_number: usize) -> Result<Option<Variant>> {
        self.fields
            .get(&field_number)
            .into_iter()
            .flatten()
            .last()
            .map(|field| {
                match field {
                    Field::Variant(variant) => Ok(variant.clone()),
                    Field::LengthDelimited(ref dldd) => dldd
                        .deserialize_as_variants()
                        .last()
                        .unwrap_or(Err(PuroroError::ZeroLengthPackedField)),
                    _ => {
                        // found fixed32 or fixed64 field
                        Err(PuroroError::UnexpectedWireType)
                    }
                }
            })
            .transpose()
    }

    fn get_variant_field_as_or<T: variant::VariantType>(
        &self,
        field_number: usize,
        default: T::NativeType,
    ) -> Result<T::NativeType> {
        self.get_last_variant_field(field_number)
            .and_then(|optv| optv.map_or(Ok(default), |variant| variant.to_native::<T>()))
    }

    fn get_variant_field_iterator(
        &self,
        field_number: usize,
    ) -> impl Iterator<Item = Result<Variant>> + '_ {
        self.fields
            .get(&field_number)
            .into_iter()
            .flatten()
            .flat_map(|field| match field {
                Field::Variant(ref variant) => Either::Left(std::iter::once(Ok(variant.clone()))),
                Field::LengthDelimited(ref dldd) => Either::Right(dldd.deserialize_as_variants()),
                _ => Either::Left(std::iter::once(Err(PuroroError::UnexpectedWireType))),
            })
    }

    fn handle_repeated_varient_field<T, H>(
        &self,
        field_number: usize,
        handler: &H,
    ) -> Result<H::Output>
    where
        T: variant::VariantType,
        H: RepeatedFieldHandler<Item = T::NativeType>,
    {
        let iter = self
            .get_variant_field_iterator(field_number)
            .map(|r| r.and_then(|variant| variant.to_native::<T>()));
        handler.handle(iter)
    }
}

impl MessageHandler for UnknownMessage {
    type Target = Self;

    fn finish(self) -> Result<Self::Target> {
        Ok(self)
    }

    fn met_field<T: LengthDelimitedDeserializer>(
        &mut self,
        field: Field<T>,
        field_number: usize,
    ) -> puroro::Result<()> {
        self.fields
            .entry(field_number)
            .or_insert(Vec::new())
            .push(try_into_persistent_field(field)?);
        Ok(())
    }
}

macro_rules! define_variant_methods {
    ($vtype:ty, $singular_get_or:ident, $repeated_handle:ident) => {
        fn $singular_get_or(
            &self,
            field_number: usize,
            default: <$vtype as variant::VariantType>::NativeType,
        ) -> Result<<$vtype as variant::VariantType>::NativeType> {
            self.get_variant_field_as_or::<$vtype>(field_number, default)
        }

        fn $repeated_handle<H>(&self, field_number: usize, handler: &H) -> Result<H::Output>
        where
            H: puroro::RepeatedFieldHandler<Item = <$vtype as variant::VariantType>::NativeType>,
        {
            self.handle_repeated_varient_field::<$vtype, H>(field_number, handler)
        }
    };
}

impl Message for UnknownMessage {
    define_variant_methods!(
        tags::Int32,
        get_field_as_i32_or,
        handle_field_as_repeated_i32
    );
    define_variant_methods!(
        tags::Int64,
        get_field_as_i64_or,
        handle_field_as_repeated_i64
    );
    define_variant_methods!(
        tags::UInt32,
        get_field_as_u32_or,
        handle_field_as_repeated_u32
    );
    define_variant_methods!(
        tags::UInt64,
        get_field_as_u64_or,
        handle_field_as_repeated_u64
    );
    define_variant_methods!(
        tags::SInt32,
        get_field_as_si32_or,
        handle_field_as_repeated_si32
    );
    define_variant_methods!(
        tags::SInt64,
        get_field_as_si64_or,
        handle_field_as_repeated_si64
    );
    define_variant_methods!(
        tags::Bool,
        get_field_as_bool_or,
        handle_field_as_repeated_bool
    );

    fn handle_field_as_str<H: RepeatedFieldHandler<Item = char>>(
        &self,
        field_number: usize,
        handler: &H,
    ) -> puroro::Result<H::Output> {
        let iter = self
            .fields
            .get(&field_number)
            .into_iter()
            .flatten()
            .last()
            .map(|field| {
                if let Field::LengthDelimited(ref dldd) = field {
                    Ok(dldd.deserialize_as_chars())
                } else {
                    Err(PuroroError::UnexpectedWireType)
                }
            })
            .transpose()?
            .into_iter()
            .flatten();
        handler.handle(iter)
    }

    fn handle_field_as_repeated_str<
        H: RepeatedFieldHandler<Item = char>,
        G: RepeatedFieldHandler<Item = H::Output>,
    >(
        &self,
        field_number: usize,
        string_handler: &H,
        strings_handler: &G,
    ) -> puroro::Result<G::Output> {
        let strings_iter = self
            .fields
            .get(&field_number)
            .into_iter()
            .flatten()
            .map(|field| {
                if let Field::LengthDelimited(dldd) = field {
                    let chars_iter = dldd.deserialize_as_chars();
                    string_handler.handle(chars_iter)
                } else {
                    Err(PuroroError::UnexpectedWireType)
                }
            });
        strings_handler.handle(strings_iter)
    }

    fn get_field_as_message<T>(&self, field_number: usize) -> Result<Option<T>>
    where
        T: Mergeable + Deserializable,
    {
        if let Some(fields) = self.fields.get(&field_number) {
            return fields
                .iter()
                .map(|field| match field {
                    Field::LengthDelimited(ref dldd) => T::from_bytes(dldd.bytes()),
                    _ => Err(PuroroError::UnexpectedWireType),
                })
                .try_fold(None, |acc: Option<T>, r| match acc {
                    None => Ok(Some(r?)),
                    Some(acc_msg) => Ok(Some(acc_msg.merge(&r?)?)),
                });
        }
        Ok(None)
    }

    fn collect_field_as_repeated_message<T: Deserializable, U: std::iter::FromIterator<T>>(
        &self,
        field_number: usize,
    ) -> Result<U> {
        if let Some(fields) = self.fields.get(&field_number) {
            return fields
                .iter()
                .map(|field| match field {
                    Field::LengthDelimited(ref dldd) => T::from_bytes(dldd.bytes()),
                    _ => Err(PuroroError::InvalidWireType),
                })
                .collect::<Result<U>>();
        }
        Ok(std::iter::empty().collect::<U>())
    }
}
impl Deserializable for UnknownMessage {
    fn from_bytes<I: Iterator<Item = std::io::Result<u8>>>(iter: I) -> Result<Self> {
        ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter)
            .deserialize(UnknownMessage::new())
    }
}
impl Mergeable for UnknownMessage {
    fn merge(&self, latter: &Self) -> Result<Self> {
        let mut new_message = self.clone();
        for (k, v) in &latter.fields {
            new_message
                .fields
                .entry(*k)
                .and_modify(|fields| fields.extend(v.iter().cloned()))
                .or_insert_with(|| v.clone());
        }
        Ok(new_message)
    }
}

fn try_into_persistent_field<T: LengthDelimitedDeserializer>(
    from: Field<T>,
) -> Result<Field<DelayedLengthDelimitedDeserializer>> {
    Ok(match from {
        Field::LengthDelimited(ldd) => Field::LengthDelimited(ldd.leave_as_unknown()?),
        Field::Variant(v) => Field::Variant(v),
        Field::Bytes32(v) => Field::Bytes32(v),
        Field::Bytes64(v) => Field::Bytes64(v),
    })
}
