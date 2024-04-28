// Copyright 2021 Google LLC
//
// Licensed under the Apache License, Version 2.num (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.num
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub mod record;

use crate::internal::WireType;
use crate::variant::{Int32, Variant, VariantIntegerType};
use crate::Result;
use ::std::io::{BufRead, Read};

pub struct ScopeRead<R> {
    read: R,
    limits: Vec<usize>,
}
impl<R> ScopeRead<R> {
    pub fn new(read: R) -> Self {
        Self {
            read,
            limits: Vec::new(),
        }
    }
    pub fn scope(&mut self, limit: usize) {
        if let Some(last_mut) = self.limits.last_mut() {
            *last_mut -= limit;
        }
        self.limits.push(limit);
    }
    pub fn unscope(&mut self) {
        self.limits.pop();
    }
    pub fn depth(&self) -> usize {
        self.limits.len()
    }
    pub fn limit(&self) -> Option<usize> {
        self.limits.last().copied()
    }
}
impl<R: Read> Read for ScopeRead<R> {
    fn read(&mut self, buf: &mut [u8]) -> ::std::io::Result<usize> {
        if let Some(limit) = self.limits.last_mut() {
            let read_len = if buf.len() > *limit {
                // Buffer is too long. Limit the buffer.
                self.read.read(&mut buf[..*limit])?
            } else {
                // Buffer is within the limit.
                self.read.read(buf)?
            };
            // Shrink the current scope's limit size.
            *limit -= read_len;
            Ok(read_len)
        } else {
            // No limit is set. Identical to the inner read.
            self.read.read(buf)
        }
    }
}
impl<R: BufRead> BufRead for ScopeRead<R> {
    fn fill_buf(&mut self) -> ::std::io::Result<&[u8]> {
        let buf = self.read.fill_buf()?;
        if let Some(limit) = self.limits.last() {
            if buf.len() > *limit {
                // Buffer is too long. Limit the buffer.
                return Ok(&buf[..*limit]);
            }
        }
        // Buffer is within the limit.
        Ok(buf)
    }
    fn consume(&mut self, amt: usize) {
        if let Some(last_mut) = self.limits.last_mut() {
            assert!(amt <= *last_mut);
            *last_mut -= amt;
        }
        self.read.consume(amt)
    }
}

pub trait DeserMessageHandler<LenBody> {
    fn parse_variant(&mut self, num: i32, var: Variant) -> Result<()>;
    fn parse_i32(&mut self, num: i32, val: [u8; 4]) -> Result<()>;
    fn parse_i64(&mut self, num: i32, val: [u8; 8]) -> Result<()>;
    fn parse_len(&mut self, num: i32, val: &mut LenBody) -> Result<()>;
    fn is_message_field(&self, num: i32) -> bool;
    fn start_message(&mut self, num: i32) -> Result<()>;
    fn end_message(&mut self) -> Result<()>;
}

trait ReadExt: Sized {
    fn read_wire_type_and_field_number(&mut self) -> Result<Option<(WireType, i32)>>;
}

impl<T: Read> ReadExt for T {
    fn read_wire_type_and_field_number(&mut self) -> Result<Option<(WireType, i32)>> {
        use crate::variant::ReadExtVariant;
        let Some(tag_var) = self.read_variant_or_eof()? else {
            return Ok(None);
        };
        let tag: u32 = tag_var.try_into()?;
        let wire_type: WireType = (tag & 0x7).try_into()?;
        // safe because the `tag >> 3` is less than 29 bits
        let field_number: i32 = (tag >> 3).try_into()?;
        Ok(Some((wire_type, field_number)))
    }
}

pub fn deser_from_read<R: BufRead, H: DeserMessageHandler<ScopeRead<R>>>(
    read: R,
    handler: &mut H,
) -> Result<()> {
    use crate::variant::BufReadExtVariant;
    let mut read = ScopeRead::new(read);
    loop {
        let Some((wire_type, field_number)) = read.read_wire_type_and_field_number()? else {
            if read.depth() == 0 {
                // No more records and no more stack, we're done for the given `Read` instance.
                break;
            } else {
                // Finished the current level of messages, pop the stack to go back to the parent.
                read.unscope();
                handler.end_message()?;
                continue;
            }
        };
        match wire_type {
            WireType::Variant => {
                handler.parse_variant(field_number, read.read_variant()?)?;
            }
            WireType::I32 => {
                let mut buf = [0; 4];
                read.read_exact(&mut buf)?;
                handler.parse_i32(field_number, buf)?;
            }
            WireType::I64 => {
                let mut buf = [0; 8];
                read.read_exact(&mut buf)?;
                handler.parse_i64(field_number, buf)?;
            }
            WireType::Len => {
                let len: usize = Int32::try_from_variant(read.read_variant()?)?.try_into()?;
                read.scope(len);
                if handler.is_message_field(field_number) {
                    handler.start_message(field_number)?;
                } else {
                    handler.parse_len(field_number, &mut read)?;
                    read.unscope();
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{variant::*, ErrorKind};

    #[derive(Default, Debug, PartialEq)]
    struct Field<T> {
        num: i32,
        val: T,
    }

    #[derive(Default, Debug, PartialEq)]
    struct SampleMessage {
        variants: Vec<Field<Variant>>,
        i32s: Vec<Field<u32>>,
        i64s: Vec<Field<u64>>,
        // even number fields are strings, odd number fields are submessages
        strings: Vec<Field<String>>,
        children: Vec<Field<Box<SampleMessage>>>,
    }

    struct SampleMessageHandler {
        cur: SampleMessage,
        // (message, field number of child)
        stack: Vec<(SampleMessage, i32)>,
    }
    impl SampleMessageHandler {
        fn new() -> Self {
            Self {
                cur: SampleMessage::default(),
                stack: Vec::new(),
            }
        }
        fn finish(self) -> SampleMessage {
            assert!(self.stack.is_empty());
            self.cur
        }
    }
    impl<R: BufRead> DeserMessageHandler<R> for SampleMessageHandler {
        fn parse_variant(&mut self, num: i32, var: Variant) -> Result<()> {
            self.cur.variants.push(Field { num, val: var });
            Ok(())
        }
        fn parse_i32(&mut self, num: i32, val: [u8; 4]) -> Result<()> {
            self.cur.i32s.push(Field {
                num,
                val: u32::from_le_bytes(val),
            });
            Ok(())
        }
        fn parse_i64(&mut self, num: i32, val: [u8; 8]) -> Result<()> {
            self.cur.i64s.push(Field {
                num,
                val: u64::from_le_bytes(val),
            });
            Ok(())
        }
        fn parse_len(&mut self, num: i32, read: &mut R) -> Result<()> {
            let mut val = String::new();
            read.read_to_string(&mut val)?;
            self.cur.strings.push(Field { num, val });
            Ok(())
        }
        fn is_message_field(&self, num: i32) -> bool {
            num % 2 == 1
        }
        fn start_message(&mut self, num: i32) -> Result<()> {
            self.stack.push((::std::mem::take(&mut self.cur), num));
            Ok(())
        }
        fn end_message(&mut self) -> Result<()> {
            let (parent, field_num) = self.stack.pop().ok_or(ErrorKind::DeserUnexpectedEof)?;
            let child = ::std::mem::replace(&mut self.cur, parent);
            self.cur.children.push(Field {
                num: field_num,
                val: Box::new(child),
            });
            Ok(())
        }
    }

    const INPUT_FIELD_1_VARIANT_1: &[u8] = &[(1 << 3) | WireType::Variant as u8, 0x01];
    const INPUT_FIELD_2_VARIANT_3: &[u8] = &[(2 << 3) | WireType::Variant as u8, 0x03];
    const INPUT_FIELD_3_I32_1: &[u8] = &[(3 << 3) | WireType::I32 as u8, 1, 0, 0, 0];
    const INPUT_FIELD_4_I32_3: &[u8] = &[(4 << 3) | WireType::I32 as u8, 3, 0, 0, 0];
    const INPUT_FIELD_5_I64_1: &[u8] = &[(5 << 3) | WireType::I64 as u8, 1, 0, 0, 0, 0, 0, 0, 0];
    const INPUT_FIELD_6_I64_3: &[u8] = &[(6 << 3) | WireType::I64 as u8, 3, 0, 0, 0, 0, 0, 0, 0];
    const INPUT_FIELD_8_STRING_FOO: &[u8] = &[(8 << 3) | WireType::Len as u8, 3, b'f', b'o', b'o'];
    const INPUT_FIELD_10_STRING_YO: &[u8] = &[(10 << 3) | WireType::Len as u8, 2, b'y', b'o'];

    fn gen_submessage_bytes(num: i32, submessage_bytes: impl AsRef<[u8]>) -> Vec<u8> {
        let mut result = Vec::new();
        result
            .write_variant(Variant::from(((num as u64) << 3) | WireType::Len as u64))
            .unwrap();
        result
            .write_variant(
                Int32::try_into_variant(submessage_bytes.as_ref().len().try_into().unwrap())
                    .unwrap(),
            )
            .unwrap();
        result.extend_from_slice(submessage_bytes.as_ref());
        result
    }

    fn test_case_variant_fields() -> (Vec<u8>, SampleMessage) {
        let vec = [INPUT_FIELD_1_VARIANT_1, INPUT_FIELD_2_VARIANT_3]
            .into_iter()
            .flatten()
            .copied()
            .collect::<Vec<_>>();
        let mut expected = SampleMessage::default();
        expected.variants.push(Field {
            num: 1,
            val: 1u32.into(),
        });
        expected.variants.push(Field {
            num: 2,
            val: 3u32.into(),
        });
        (vec, expected)
    }

    fn test_case_fixed_fields() -> (Vec<u8>, SampleMessage) {
        let vec = [
            INPUT_FIELD_3_I32_1,
            INPUT_FIELD_4_I32_3,
            INPUT_FIELD_5_I64_1,
            INPUT_FIELD_6_I64_3,
        ]
        .into_iter()
        .flatten()
        .copied()
        .collect::<Vec<_>>();
        let mut expected = SampleMessage::default();
        expected.i32s.push(Field { num: 3, val: 1 });
        expected.i32s.push(Field { num: 4, val: 3 });
        expected.i64s.push(Field { num: 5, val: 1 });
        expected.i64s.push(Field { num: 6, val: 3 });
        (vec, expected)
    }

    fn test_case_string_fields() -> (Vec<u8>, SampleMessage) {
        let vec = [INPUT_FIELD_8_STRING_FOO, INPUT_FIELD_10_STRING_YO]
            .into_iter()
            .flatten()
            .copied()
            .collect::<Vec<_>>();
        let mut expected = SampleMessage::default();
        expected.strings.push(Field {
            num: 8,
            val: "foo".to_string(),
        });
        expected.strings.push(Field {
            num: 10,
            val: "yo".to_string(),
        });
        (vec, expected)
    }

    fn test_case_complex_fields() -> (Vec<u8>, SampleMessage) {
        let subsubmsg1_bytes = INPUT_FIELD_1_VARIANT_1;
        let submsg1_bytes = [
            INPUT_FIELD_2_VARIANT_3.to_vec(),
            gen_submessage_bytes(3, subsubmsg1_bytes),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
        let msg1_bytes = [
            INPUT_FIELD_3_I32_1.to_vec(),
            gen_submessage_bytes(7, &submsg1_bytes),
            gen_submessage_bytes(5, INPUT_FIELD_4_I32_3),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
        let mut expected = SampleMessage::default();
        expected.i32s.push(Field { num: 3, val: 1 });
        expected.children.push(Field {
            num: 7,
            val: Box::new(SampleMessage::default()),
        });
        expected.children.push(Field {
            num: 5,
            val: Box::new(SampleMessage::default()),
        });
        expected.children[0].val.variants.push(Field {
            num: 2,
            val: 3u32.into(),
        });
        expected.children[0].val.children.push(Field {
            num: 3,
            val: Box::new(SampleMessage::default()),
        });
        expected.children[0].val.children[0]
            .val
            .variants
            .push(Field {
                num: 1,
                val: 1u32.into(),
            });
        expected.children[1].val.i32s.push(Field { num: 4, val: 3 });
        (msg1_bytes, expected)
    }

    #[test]
    fn test_deser_variant_fields() {
        let (input, expected) = test_case_variant_fields();
        let mut handler = SampleMessageHandler::new();
        deser_from_read(input.as_slice(), &mut handler).unwrap();
        let msg = handler.finish();
        assert_eq!(expected, msg);
    }

    #[test]
    fn test_deser_fixed_fields() {
        let (input, expected) = test_case_fixed_fields();
        let mut handler = SampleMessageHandler::new();
        deser_from_read(input.as_slice(), &mut handler).unwrap();
        let msg = handler.finish();
        assert_eq!(expected, msg);
    }

    #[test]
    fn test_deser_string_fields() {
        let (input, expected) = test_case_string_fields();
        let mut handler = SampleMessageHandler::new();
        deser_from_read(input.as_slice(), &mut handler).unwrap();
        let msg = handler.finish();
        assert_eq!(expected, msg);
    }

    #[test]
    fn test_deser_complex_fields() {
        let (input, expected) = test_case_complex_fields();
        let mut handler = SampleMessageHandler::new();
        deser_from_read(input.as_slice(), &mut handler).unwrap();
        let msg = handler.finish();
        assert_eq!(expected, msg);
    }
}
