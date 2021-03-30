use ::itertools::Either;
use ::puroro::{Deserializable, Mergeable, Message, PuroroError, RepeatedFieldHandler};
use ::puroro_serializer::deserializer::stream::*;
use ::puroro_serializer::variant;
use ::puroro_serializer::variant::{Variant, VariantType};
use std::collections::HashMap;

type Result<T> = std::result::Result<T, PuroroError>;

#[derive(Debug, Clone)]
enum Field {
    Variant(Variant),
    Value32([u8; 4]),
    Value64([u8; 8]),
    LengthDelimited(DelayedLengthDelimitedDeserializer),
}

#[derive(Debug, Clone)]
pub struct UnknownMessage {
    fields: HashMap<usize, Vec<Field>>,
}

impl UnknownMessage {
    fn new() -> Self {
        Self {
            fields: HashMap::new(),
        }
    }

    fn get_last_variant_field(&self, field_number: usize) -> Result<Variant> {
        if let Some(fields) = self.fields.get(&field_number) {
            if let Some(last_field) = fields.last() {
                match last_field {
                    Field::Variant(ref variant) => {
                        return Ok(variant.clone());
                    }
                    Field::LengthDelimited(ref dldd) => {
                        let mut variants = Vec::new();
                        dldd.deserialize_as_variants(|vs| {
                            variants = vs;
                            Ok(())
                        })?;
                        if let Some(variant) = variants.last() {
                            return Ok(variant.clone());
                        }
                    }
                    _ => {
                        // found fixed32 or fixed64 field
                        return Err(PuroroError::UnexpectedWireType);
                    }
                }
            }
        }
        Ok(Variant::default())
    }

    fn get_variant_field_as<T: VariantType>(&self, field_number: usize) -> Result<T::NativeType> {
        self.get_last_variant_field(field_number)?.to_native::<T>()
    }

    fn get_variant_field_iterator(
        &self,
        field_number: usize,
    ) -> impl Iterator<Item = Result<Variant>> + '_ {
        let fields_iter = if let Some(fields) = self.fields.get(&field_number) {
            Either::Left(fields.iter())
        } else {
            Either::Right(std::iter::empty::<&Field>())
        };
        fields_iter.flat_map(|field| match field {
            Field::Variant(ref variant) => Either::Left(std::iter::once(Ok(variant.clone()))),
            Field::LengthDelimited(ref dldd) => {
                let mut variants = Vec::new();
                if let Err(e) = dldd.deserialize_as_variants(|vs: Vec<Variant>| {
                    variants = vs.iter().map(|v| Ok(v.clone())).collect::<Vec<_>>();
                    Ok(())
                }) {
                    Either::Left(std::iter::once(Err(e)))
                } else {
                    Either::Right(variants.into_iter())
                }
            }
            _ => Either::Left(std::iter::once(Err(PuroroError::UnexpectedWireType))),
        })
    }

    fn handle_repeated_varient_field<T, H>(
        &self,
        field_number: usize,
        handler: H,
    ) -> Result<H::Output>
    where
        T: VariantType,
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

    fn deserialized_variant(&mut self, field_number: usize, variant: Variant) -> Result<()> {
        self.fields
            .entry(field_number)
            .or_insert(Vec::new())
            .push(Field::Variant(variant));
        Ok(())
    }

    fn deserialized_32bits(&mut self, field_number: usize, value: [u8; 4]) -> Result<()> {
        self.fields
            .entry(field_number)
            .or_insert(Vec::new())
            .push(Field::Value32(value));
        Ok(())
    }

    fn deserialized_64bits(&mut self, field_number: usize, value: [u8; 8]) -> Result<()> {
        self.fields
            .entry(field_number)
            .or_insert(Vec::new())
            .push(Field::Value64(value));
        Ok(())
    }

    fn deserialize_length_delimited_field<'a, D: LengthDelimitedDeserializer<'a>>(
        &mut self,
        deserializer: D,
        field_number: usize,
    ) -> Result<()> {
        self.fields
            .entry(field_number)
            .or_insert(Vec::new())
            .push(Field::LengthDelimited(deserializer.leave_as_unknown()?));
        Ok(())
    }
}

macro_rules! define_variant_methods {
    ($vtype:ty, $singular_get:ident, $repeated_handle:ident) => {
        fn $singular_get(
            &self,
            field_number: usize,
        ) -> Result<<$vtype as VariantType>::NativeType> {
            self.get_variant_field_as::<$vtype>(field_number)
        }

        fn $repeated_handle<H>(&self, field_number: usize, handler: H) -> Result<H::Output>
        where
            H: puroro::RepeatedFieldHandler<Item = <$vtype as VariantType>::NativeType>,
        {
            self.handle_repeated_varient_field::<$vtype, H>(field_number, handler)
        }
    };
}

impl Message for UnknownMessage {
    define_variant_methods!(
        variant::Int32,
        get_field_as_i32,
        handle_field_as_repeated_i32
    );
    define_variant_methods!(
        variant::Int64,
        get_field_as_i64,
        handle_field_as_repeated_i64
    );
    define_variant_methods!(
        variant::UInt32,
        get_field_as_u32,
        handle_field_as_repeated_u32
    );
    define_variant_methods!(
        variant::UInt64,
        get_field_as_u64,
        handle_field_as_repeated_u64
    );
    define_variant_methods!(
        variant::SInt32,
        get_field_as_si32,
        handle_field_as_repeated_si32
    );
    define_variant_methods!(
        variant::SInt64,
        get_field_as_si64,
        handle_field_as_repeated_si64
    );
    define_variant_methods!(
        variant::Bool,
        get_field_as_bool,
        handle_field_as_repeated_bool
    );

    fn collect_field_as_str<S: std::iter::FromIterator<char>>(
        &self,
        field_number: usize,
    ) -> Result<S> {
        if let Some(fields) = self.fields.get(&field_number) {
            if let Some(last_field) = fields.last() {
                match last_field {
                    Field::LengthDelimited(ref dldd) => {
                        let mut string = String::new();
                        dldd.deserialize_as_string(|s| {
                            string = s;
                            Ok(())
                        })?;
                        return Ok(string.chars().collect::<S>());
                    }
                    _ => {
                        return Err(PuroroError::InvalidWireType);
                    }
                }
            }
        }
        Ok(std::iter::empty().collect::<S>())
    }

    fn collect_field_as_repeated_str<
        S: std::iter::FromIterator<char>,
        T: std::iter::FromIterator<S>,
    >(
        &self,
        field_number: usize,
    ) -> Result<T> {
        if let Some(fields) = self.fields.get(&field_number) {
            return fields
                .iter()
                .map(|field| match field {
                    Field::LengthDelimited(ref dldd) => {
                        let mut string = String::new();
                        dldd.deserialize_as_string(|s| {
                            string = s;
                            Ok(())
                        })?;
                        return Ok(string.chars().collect::<S>());
                    }
                    _ => {
                        return Err(PuroroError::InvalidWireType);
                    }
                })
                .collect::<Result<T>>();
        }
        Ok(std::iter::empty().collect::<T>())
    }

    fn get_field_as_message<T>(&self, field_number: usize) -> Result<Option<T>>
    where
        T: Mergeable<MergedType = T> + Deserializable,
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
        deserializer_from_bytes(iter).deserialize(UnknownMessage::new())
    }
}
impl Mergeable for UnknownMessage {
    type MergedType = UnknownMessage;
    fn merge(&self, latter: &Self) -> Result<UnknownMessage> {
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
