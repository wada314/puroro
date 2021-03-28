use ::itertools::Either;
use ::puroro::Message;
use ::puroro::PuroroError;
use ::puroro_deserializer::stream::*;
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

impl Message for UnknownMessage {
    fn from_bytes<I: Iterator<Item = std::io::Result<u8>>>(iter: I) -> Result<Self> {
        deserializer_from_bytes(iter).deserialize(UnknownMessage::new())
    }
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

    fn get_field_as_i32(&self, field_number: usize) -> Result<i32> {
        self.get_last_variant_field(field_number)?.to_i32()
    }
    fn get_field_as_i64(&self, field_number: usize) -> Result<i64> {
        self.get_last_variant_field(field_number)?.to_i64()
    }
    fn get_field_as_si32(&self, field_number: usize) -> Result<i32> {
        self.get_last_variant_field(field_number)?.to_si32()
    }
    fn get_field_as_si64(&self, field_number: usize) -> Result<i64> {
        self.get_last_variant_field(field_number)?.to_si64()
    }
    fn get_field_as_u32(&self, field_number: usize) -> Result<u32> {
        self.get_last_variant_field(field_number)?.to_u32()
    }
    fn get_field_as_u64(&self, field_number: usize) -> Result<u64> {
        self.get_last_variant_field(field_number)?.to_u64()
    }

    fn collect_field_as_repeated_i32<T: std::iter::FromIterator<i32>>(
        &self,
        field_number: usize,
    ) -> Result<T> {
        self.get_variant_field_iterator(field_number)
            .map(|r| r.and_then(|v| v.to_i32()))
            .collect::<Result<T>>()
    }

    fn collect_field_as_repeated_i64<T: std::iter::FromIterator<i64>>(
        &self,
        field_number: usize,
    ) -> Result<T> {
        self.get_variant_field_iterator(field_number)
            .map(|r| r.and_then(|v| v.to_i64()))
            .collect::<Result<T>>()
    }

    fn collect_field_as_repeated_si32<T: std::iter::FromIterator<i32>>(
        &self,
        field_number: usize,
    ) -> Result<T> {
        self.get_variant_field_iterator(field_number)
            .map(|r| r.and_then(|v| v.to_si32()))
            .collect::<Result<T>>()
    }

    fn collect_field_as_repeated_si64<T: std::iter::FromIterator<i64>>(
        &self,
        field_number: usize,
    ) -> Result<T> {
        self.get_variant_field_iterator(field_number)
            .map(|r| r.and_then(|v| v.to_si64()))
            .collect::<Result<T>>()
    }

    fn collect_field_as_repeated_u32<T: std::iter::FromIterator<u32>>(
        &self,
        field_number: usize,
    ) -> Result<T> {
        self.get_variant_field_iterator(field_number)
            .map(|r| r.and_then(|v| v.to_u32()))
            .collect::<Result<T>>()
    }

    fn collect_field_as_repeated_u64<T: std::iter::FromIterator<u64>>(
        &self,
        field_number: usize,
    ) -> Result<T> {
        self.get_variant_field_iterator(field_number)
            .map(|r| r.and_then(|v| v.to_u64()))
            .collect::<Result<T>>()
    }

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
        T: Message<MergedType = T>,
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

    fn collect_field_as_repeated_message<T: Message, U: std::iter::FromIterator<T>>(
        &self,
        field_number: usize,
    ) -> Result<U> {
        todo!()
    }
}
