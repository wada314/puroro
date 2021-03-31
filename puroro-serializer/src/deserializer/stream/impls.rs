use ::either::Either;
use ::num_traits::FromPrimitive;
use std::io::{Bytes, Result as IoResult};
use utf8_decode::UnsafeDecoder;

use super::*;

pub(crate) struct DeserializerImpl<I>
where
    I: Iterator<Item = IoResult<u8>>,
{
    indexed_iter: IndexedIterator<I>,
}
impl<I> DeserializerImpl<I>
where
    I: Iterator<Item = IoResult<u8>>,
{
    pub(crate) fn new(iter: I) -> Self {
        Self {
            indexed_iter: IndexedIterator::new(iter),
        }
    }
}
impl<I> Deserializer for DeserializerImpl<I>
where
    I: Iterator<Item = IoResult<u8>>,
{
    fn deserialize<H: MessageHandler>(mut self, handler: H) -> Result<H::Target> {
        LengthDelimitedDeserializerImpl::new(&mut self.indexed_iter, None)
            .deserialize_as_message(handler)
    }
}

pub(crate) struct LengthDelimitedDeserializerImpl<'a, I>
where
    I: Iterator<Item = IoResult<u8>>,
{
    indexed_iter: &'a mut IndexedIterator<I>,
    bytes_len: Option<usize>,
    #[cfg(debug_assertions)]
    deserialized: bool,
    sub_deserializer_expected_finish_pos: Option<usize>,
}
impl<'a, I> LengthDelimitedDeserializerImpl<'a, I>
where
    I: Iterator<Item = IoResult<u8>>,
{
    pub(crate) fn new(indexed_iter: &'a mut IndexedIterator<I>, bytes_len: Option<usize>) -> Self {
        Self {
            indexed_iter,
            bytes_len,
            #[cfg(debug_assertions)]
            deserialized: false,
            #[cfg(debug_assertions)]
            sub_deserializer_expected_finish_pos: None,
        }
    }
    fn make_sub_deserializer<'b>(
        &'b mut self,
        new_length: usize,
    ) -> LengthDelimitedDeserializerImpl<'b, I>
    where
        'a: 'b,
    {
        assert!(
            self.sub_deserializer_expected_finish_pos.is_none(),
             "The previous field is not preccessed yet. i.e. The iterator is still in the previous field."
        );
        self.sub_deserializer_expected_finish_pos = Some(new_length + self.indexed_iter.index());
        LengthDelimitedDeserializerImpl {
            indexed_iter: self.indexed_iter,
            bytes_len: Some(new_length),
            #[cfg(debug_assertions)]
            deserialized: false,
            #[cfg(debug_assertions)]
            sub_deserializer_expected_finish_pos: None,
        }
    }

    // May expectedly fail if reached to the eof
    fn try_get_wire_type_and_field_number(&mut self) -> Result<Option<(WireType, usize)>> {
        let mut peekable = self.indexed_iter.by_ref().peekable();
        if let None = peekable.peek() {
            // Found EOF at first byte. Successfull failure.
            return Ok(None);
        }
        let key = Variant::decode_bytes(&mut peekable)?.to_usize()?;
        Ok(Some((
            WireType::from_usize(key & 0x07).ok_or(PuroroError::InvalidWireType)?,
            (key >> 3),
        )))
    }

    fn eat_one_byte(&mut self) -> Result<u8> {
        self.indexed_iter
            .next()
            .ok_or(PuroroError::UnexpectedInputTermination)
            .and_then(|r| r.map_err(|e| e.into()))
    }

    #[cfg(debug_assertions)]
    fn mark_deserialized(&mut self) {
        self.deserialized = true;
    }
    #[cfg(not(debug_assertions))]
    fn mark_deserialized(&mut self) {}
}

impl<'a, I> Iterator for LengthDelimitedDeserializerImpl<'a, I>
where
    I: Iterator<Item = IoResult<u8>>,
{
    type Item = <I as Iterator>::Item;
    #[cfg(debug_assertions)]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(expected_pos) = self.sub_deserializer_expected_finish_pos.take() {
            assert_eq!(
                expected_pos,
                self.indexed_iter.index(),
                concat!(
                    "The previous field is not finished yet! ",
                    "Since this deserializer uses an Iterator as input, ",
                    "you need to make sure the previous field has read the ",
                    "all its contents from the input Iterator."
                )
            );
        }
        self.indexed_iter.next()
    }
    #[cfg(not(debug_assertions))]
    fn next(&mut self) -> Option<Self::Item> {
        self.indexed_iter.next()
    }
}

impl<'a, I> LengthDelimitedDeserializer<'a> for LengthDelimitedDeserializerImpl<'a, I>
where
    I: Iterator<Item = IoResult<u8>>,
{
    fn deserialize_as_message<H: MessageHandler>(mut self, mut handler: H) -> Result<H::Target> {
        self.mark_deserialized();
        let start_pos = self.indexed_iter.index();
        loop {
            // Check message length if possible
            if let Some(message_length) = self.bytes_len {
                if start_pos + message_length <= self.indexed_iter.index() {
                    break;
                }
            }

            // get field number, wire type
            let (wire_type, field_number) = match self.try_get_wire_type_and_field_number() {
                Ok(Some(x)) => x,
                Ok(None) => {
                    break;
                } // This is ok. finish This message deserialization.
                Err(e) => {
                    return Err(e);
                }
            };

            match wire_type {
                WireType::Variant => {
                    let variant = Variant::decode_bytes(&mut self.indexed_iter)?;
                    handler.deserialized_variant(field_number, variant)?;
                }
                WireType::LengthDelimited => {
                    let field_length = Variant::decode_bytes(&mut self.indexed_iter)?.to_usize()?;
                    let deserializer_for_inner = self.make_sub_deserializer(field_length);
                    handler
                        .deserialize_length_delimited_field(deserializer_for_inner, field_number)?;
                }
                WireType::Bytes32 => {
                    let v0 = self.eat_one_byte()?;
                    let v1 = self.eat_one_byte()?;
                    let v2 = self.eat_one_byte()?;
                    let v3 = self.eat_one_byte()?;
                    handler.deserialized_32bits(field_number, [v0, v1, v2, v3])?;
                }
                WireType::Bytes64 => {
                    let v0 = self.eat_one_byte()?;
                    let v1 = self.eat_one_byte()?;
                    let v2 = self.eat_one_byte()?;
                    let v3 = self.eat_one_byte()?;
                    let v4 = self.eat_one_byte()?;
                    let v5 = self.eat_one_byte()?;
                    let v6 = self.eat_one_byte()?;
                    let v7 = self.eat_one_byte()?;
                    handler.deserialized_64bits(field_number, [v0, v1, v2, v3, v4, v5, v6, v7])?;
                }
                _ => {
                    unimplemented!("WIP for group support");
                }
            }
        }

        handler.finish()
    }

    fn deserialize_as_string<H>(mut self, handler: H) -> Result<H::Output>
    where
        H: RepeatedFieldHandler<Item = char>,
    {
        self.mark_deserialized();
        let start_pos = self.indexed_iter.index();
        let maybe_has_length_bytes_iter = if let Some(length) = self.bytes_len {
            Either::Left(self.indexed_iter.by_ref().take(length))
        } else {
            Either::Right(self.indexed_iter.by_ref())
        };
        let chars = CharsIterator::new(maybe_has_length_bytes_iter);
        let retval = handler.handle(chars)?;
        let end_pos = self.indexed_iter.index();
        if let Some(length) = self.bytes_len {
            if end_pos - start_pos == length {
                Ok(retval)
            } else {
                Err(PuroroError::InvalidFieldLength)
            }
        } else {
            Ok(retval)
        }
    }

    fn deserialize_as_bytes<H>(mut self, handler: H) -> Result<H::Output>
    where
        H: RepeatedFieldHandler<Item = u8>,
    {
        self.mark_deserialized();
        let start_pos = self.indexed_iter.index();
        let maybe_has_length_bytes_iter = if let Some(length) = self.bytes_len {
            Either::Left(self.indexed_iter.by_ref().take(length))
        } else {
            Either::Right(self.indexed_iter.by_ref())
        };
        let bytes_with_puroro_error = maybe_has_length_bytes_iter.map(|r| r.map_err(|e| e.into()));
        let retval = handler.handle(bytes_with_puroro_error)?;
        let end_pos = self.indexed_iter.index();
        if let Some(length) = self.bytes_len {
            if end_pos - start_pos == length {
                Ok(retval)
            } else {
                Err(PuroroError::InvalidFieldLength)
            }
        } else {
            Ok(retval)
        }
    }

    fn deserialize_as_variants<H>(mut self, handler: H) -> Result<H::Output>
    where
        H: RepeatedFieldHandler<Item = Variant>,
    {
        self.mark_deserialized();
        let start_pos = self.indexed_iter.index();
        let maybe_has_length_bytes_iter = if let Some(length) = self.bytes_len {
            Either::Left(self.indexed_iter.by_ref().take(length))
        } else {
            Either::Right(self.indexed_iter.by_ref())
        };
        let variants = VariantsIterator::new(maybe_has_length_bytes_iter);
        let retval = handler.handle(variants)?;
        let end_pos = self.indexed_iter.index();
        if let Some(length) = self.bytes_len {
            if end_pos - start_pos == length {
                Ok(retval)
            } else {
                Err(PuroroError::InvalidFieldLength)
            }
        } else {
            Ok(retval)
        }
    }

    fn leave_as_unknown(mut self) -> Result<DelayedLengthDelimitedDeserializer> {
        self.mark_deserialized();
        Ok(DelayedLengthDelimitedDeserializer::new(
            self.indexed_iter
                .collect::<IoResult<Vec<_>>>()
                .map_err(|e| PuroroError::from(e))?,
        ))
    }

    type BytesIterator = BytesIterator<'a, I>;
    fn deserialize_as_bytes_iter(self) -> Self::BytesIterator {
        BytesIterator::new(self)
    }

    type CharsIterator = CharsIterator2<'a, I>;
    fn deserialize_as_chars_iter(self) -> Self::CharsIterator {
        CharsIterator2::new(self)
    }

    type VariantsIterator = VariantsIterator2<'a, I>;
    fn deserialize_as_variants_iter(self) -> Self::VariantsIterator {
        VariantsIterator2::new(self)
    }
}
impl<'a, I> Drop for LengthDelimitedDeserializerImpl<'a, I>
where
    I: Iterator<Item = IoResult<u8>>,
{
    #[cfg(debug_assertions)]
    fn drop(&mut self) {
        if !self.deserialized {
            panic!("You MUST call one of the methods of LengthDelimitedDeserializer!!");
        }
    }
    #[cfg(not(debug_assertions))]
    fn drop(&mut self) {}
}

pub(crate) struct IndexedIterator<I> {
    index: usize,
    iter: I,
}
impl<I> Iterator for IndexedIterator<I>
where
    I: Iterator,
{
    type Item = <I as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        self.iter.next()
    }
}
impl<I> IndexedIterator<I> {
    pub(crate) fn new(iter: I) -> Self {
        IndexedIterator { index: 0, iter }
    }
    fn index(&self) -> usize {
        self.index
    }
}

pub struct BytesIterator<'a, I: Iterator<Item = IoResult<u8>>> {
    iter: LengthDelimitedDeserializerImpl<'a, I>,
}

impl<'a, I: Iterator<Item = IoResult<u8>>> BytesIterator<'a, I> {
    fn new(iter: LengthDelimitedDeserializerImpl<'a, I>) -> Self {
        Self { iter }
    }
}
impl<'a, I: Iterator<Item = IoResult<u8>>> Iterator for BytesIterator<'a, I> {
    type Item = Result<u8>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|ior| ior.map_err(|ioe| ioe.into()))
    }
}
pub struct CharsIterator2<'a, I: Iterator<Item = IoResult<u8>>> {
    iter: ::utf8_decode::UnsafeDecoder<LengthDelimitedDeserializerImpl<'a, I>>,
}
impl<'a, I: Iterator<Item = IoResult<u8>>> CharsIterator2<'a, I> {
    fn new(iter: LengthDelimitedDeserializerImpl<'a, I>) -> Self {
        Self {
            iter: ::utf8_decode::UnsafeDecoder::new(iter),
        }
    }
}
impl<'a, I: Iterator<Item = IoResult<u8>>> Iterator for CharsIterator2<'a, I> {
    type Item = Result<char>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|ior| ior.map_err(|ioe| ioe.into()))
    }
}
pub struct VariantsIterator2<'a, I: Iterator<Item = IoResult<u8>>> {
    iter: LengthDelimitedDeserializerImpl<'a, I>,
}
impl<'a, I: Iterator<Item = IoResult<u8>>> VariantsIterator2<'a, I> {
    fn new(iter: LengthDelimitedDeserializerImpl<'a, I>) -> Self {
        Self { iter }
    }
}
impl<'a, I: Iterator<Item = IoResult<u8>>> Iterator for VariantsIterator2<'a, I> {
    type Item = Result<Variant>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut peekable = self.iter.by_ref().peekable();
        if let None = peekable.peek() {
            return None;
        }
        Some(Variant::decode_bytes(&mut peekable))
    }
}

pub struct CharsIterator<T: Iterator<Item = IoResult<u8>>> {
    iter: ::utf8_decode::UnsafeDecoder<T>,
}
impl<T: Iterator<Item = IoResult<u8>>> CharsIterator<T> {
    pub fn new(iter: T) -> Self {
        Self {
            iter: ::utf8_decode::UnsafeDecoder::new(iter),
        }
    }
}
impl<T: Iterator<Item = IoResult<u8>>> Iterator for CharsIterator<T> {
    type Item = Result<char>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|r| r.map_err(|e| e.into()))
    }
}

pub struct VariantsIterator<I: Iterator<Item = IoResult<u8>>> {
    iter: I,
}
impl<I: Iterator<Item = IoResult<u8>>> VariantsIterator<I> {
    pub fn new(iter: I) -> Self {
        Self { iter }
    }
}
impl<I: Iterator<Item = IoResult<u8>>> Iterator for VariantsIterator<I> {
    type Item = Result<Variant>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut peekable = self.iter.by_ref().peekable();
        if let None = peekable.peek() {
            return None;
        }
        Some(Variant::decode_bytes(&mut peekable))
    }
}

#[cfg(test)]
mod tests {
    use puroro::RepeatedFieldCollector;

    use super::*;

    #[test]
    fn deserialize_samples_test1() {
        // https://developers.google.com/protocol-buffers/docs/encoding#simple
        // message Test1 {
        //   optional int32 a = 1;
        // }
        // a = 150
        let input: &[u8] = &[0x08, 0x96, 0x01];
        #[derive(Default, PartialEq)]
        struct Test1 {
            a: i32,
        }
        impl MessageHandler for Test1 {
            type Target = Self;
            fn finish(self) -> Result<Self::Target> {
                Ok(self)
            }
            fn deserialized_variant(
                &mut self,
                field_number: usize,
                variant: Variant,
            ) -> Result<()> {
                assert_eq!(1, field_number);
                self.a = variant.to_i32()?;
                Ok(())
            }
        }

        let handler = Test1::default();
        let deserializer = DeserializerImpl::<_>::new(input.bytes());
        let result = deserializer.deserialize(handler);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().a, 150);
    }

    #[test]
    fn deserialize_samples_test2() {
        // https://developers.google.com/protocol-buffers/docs/encoding#strings
        // message Test2 {
        //   optional string b = 2;
        // }
        // b = "testing"
        let input: &[u8] = &[0x12, 0x07, 0x74, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67];
        #[derive(Default, PartialEq)]
        struct Test2 {
            b: String,
        }
        impl MessageHandler for Test2 {
            type Target = Self;
            fn finish(self) -> Result<Self::Target> {
                Ok(self)
            }
            fn deserialize_length_delimited_field<'a, D: LengthDelimitedDeserializer<'a>>(
                &mut self,
                deserializer: D,
                field_number: usize,
            ) -> Result<()> {
                assert_eq!(field_number, 2);
                self.b = deserializer
                    .deserialize_as_string(RepeatedFieldCollector::<char, String>::new())?;
                Ok(())
            }
        }

        let handler = Test2::default();
        let deserializer = DeserializerImpl::<_>::new(input.bytes());
        let result = deserializer.deserialize(handler);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().b, "testing");
    }

    #[test]
    fn deserialize_samples_test3() {
        // https://developers.google.com/protocol-buffers/docs/encoding#embedded
        // message Test1 {
        //   optional int32 a = 1;
        // }
        // message Test3 {
        //   optional Test1 c = 3;
        // }
        // a = 150
        let input: &[u8] = &[0x1a, 0x03, 0x08, 0x96, 0x01];
        #[derive(Default, PartialEq)]
        struct Test1 {
            a: i32,
        }
        #[derive(Default, PartialEq)]
        struct Test3 {
            c: Test1,
        }

        impl MessageHandler for Test1 {
            type Target = Self;
            fn finish(self) -> Result<Self::Target> {
                Ok(self)
            }
            fn deserialized_variant(
                &mut self,
                field_number: usize,
                variant: Variant,
            ) -> Result<()> {
                assert_eq!(1, field_number);
                self.a = variant.to_i32()?;
                Ok(())
            }
        }
        impl MessageHandler for Test3 {
            type Target = Self;
            fn finish(self) -> Result<Self::Target> {
                Ok(self)
            }
            fn deserialize_length_delimited_field<'a, D: LengthDelimitedDeserializer<'a>>(
                &mut self,
                deserializer: D,
                field_number: usize,
            ) -> Result<()> {
                assert_eq!(3, field_number);
                self.c = deserializer.deserialize_as_message(Test1::default())?;
                Ok(())
            }
        }

        let handler = Test3::default();
        let deserializer = DeserializerImpl::<_>::new(input.bytes());
        let result = deserializer.deserialize(handler);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().c.a, 150);
    }

    #[test]
    fn deserialize_samples_test4() {
        // https://developers.google.com/protocol-buffers/docs/encoding#packed
        // message Test4 {
        //   repeated int32 d = 4 [packed=true];
        // }
        // d = [3, 270, 86942]
        let input: &[u8] = &[0x22, 0x06, 0x03, 0x8E, 0x02, 0x9E, 0xA7, 0x05];
        #[derive(Default, PartialEq)]
        struct Test4 {
            d: Vec<i32>,
        }
        impl MessageHandler for Test4 {
            type Target = Self;
            fn finish(self) -> Result<Self::Target> {
                Ok(self)
            }
            fn deserialize_length_delimited_field<'a, D: LengthDelimitedDeserializer<'a>>(
                &mut self,
                deserializer: D,
                field_number: usize,
            ) -> Result<()> {
                assert_eq!(4, field_number);
                self.d =
                    deserializer
                        .deserialize_as_variants(
                            RepeatedFieldCollector::<Variant, Vec<Variant>>::new(),
                        )?
                        .into_iter()
                        .map(|v| v.to_i32())
                        .collect::<Result<Vec<_>>>()?;
                Ok(())
            }
        }

        let handler = Test4::default();
        let deserializer = DeserializerImpl::<_>::new(input.bytes());
        let result = deserializer.deserialize(handler);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().d, [3, 270, 86942]);
    }
}
