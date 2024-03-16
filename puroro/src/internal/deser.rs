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

use self::record::Payload;
use crate::internal::freezing_mut::{FreezeStatus, UnfrozenMut};
use crate::internal::variant::Variant;
use crate::Result;
use ::futures::io::{AsyncRead, Take as AsyncTake};
use ::futures::task::Poll;
use ::std::io::{Read, Take};
use ::std::pin::Pin;
use ::std::task::Context;

pub trait DeseringMessage {
    fn parse_variant(&mut self, num: u32, var: Variant) -> Result<()>;
    fn parse_i32(&mut self, num: u32, val: [u8; 4]) -> Result<()>;
    fn parse_i64(&mut self, num: u32, val: [u8; 8]) -> Result<()>;
    fn parse_len_slice_or_alloc_child(
        &mut self,
        num: u32,
        slice: &[u8],
    ) -> Result<Option<&mut dyn DeseringMessage>>;
    fn parse_len_read_or_alloc_child(
        &mut self,
        num: u32,
        read: &mut Take<&mut dyn Read>,
    ) -> Result<Option<&mut dyn DeseringMessage>>;
}

pub fn deser_from_slice(root: &mut dyn DeseringMessage, mut input: &[u8]) -> Result<()> {
    use self::record::SliceExtReadRecord;
    let mut msg = UnfrozenMut::new(root);
    let mut stack = Vec::new();
    loop {
        if !input.is_empty() {
            // Still have records to process.
            let record = input.read_record()?;
            match record.payload {
                Payload::Variant(val) => msg.parse_variant(record.number, val)?,
                Payload::I32(val) => msg.parse_i32(record.number, val)?,
                Payload::I64(val) => msg.parse_i64(record.number, val)?,
                Payload::Len(input_subpart) => {
                    match msg.try_work(|msg| {
                        msg.parse_len_slice_or_alloc_child(record.number, input_subpart)
                    })? {
                        FreezeStatus::Unfrozen(new_msg) => {
                            msg = new_msg;
                        }
                        FreezeStatus::Frozen(frozen_msg, new_msg) => {
                            stack.push((input, frozen_msg));
                            input = input_subpart;
                            msg = new_msg;
                        }
                    }
                }
            }
        } else {
            // Finished the current level of messages, pop the stack to go back to the parent.
            let Some((prev_input, prev_msg)) = stack.pop() else {
                // No more records and no more stack, we're done for the given slice.
                break;
            };
            input = prev_input;
            msg = prev_msg.unfreeze(msg);
        }
    }
    Ok(())
}

pub fn deser_from_bound_read(
    root: &mut dyn DeseringMessage,
    mut bound_read: Take<impl Read>,
) -> Result<()> {
    use self::record::ReadExtReadRecord;
    let mut msg = UnfrozenMut::new(root);
    let mut stack = Vec::new();
    loop {
        if let Some(record) = bound_read.read_record_or_eof()? {
            // Still have records to process.
            match record.payload {
                Payload::Variant(val) => msg.parse_variant(record.number, val)?,
                Payload::I32(val) => msg.parse_i32(record.number, val)?,
                Payload::I64(val) => msg.parse_i64(record.number, val)?,
                Payload::Len(mut child_read) => {
                    match msg.try_work(|msg| {
                        msg.parse_len_read_or_alloc_child(record.number, &mut child_read)
                    })? {
                        FreezeStatus::Unfrozen(new_msg) => {
                            msg = new_msg;
                        }
                        FreezeStatus::Frozen(frozen_msg, new_msg) => {
                            let child_read_remaining = child_read.limit();
                            let parent_read_remaining = bound_read.limit() - child_read_remaining;
                            bound_read.set_limit(child_read_remaining);
                            stack.push((parent_read_remaining, frozen_msg));
                            msg = new_msg;
                        }
                    }
                }
            }
        } else {
            // Finished the current level of messages, pop the stack to go back to the parent.
            let Some((parent_read_remaining, prev_msg)) = stack.pop() else {
                // No more records and no more stack, we're done for the given `Read` instance.
                break;
            };
            bound_read.set_limit(parent_read_remaining);
            msg = prev_msg.unfreeze(msg);
        }
    }
    Ok(())
}

pub fn deser_from_read(root: &mut dyn DeseringMessage, mut read: impl Read) -> Result<()> {
    use self::record::ReadExtReadRecord;
    while let Some(record) = read.read_record_or_eof()? {
        match record.payload {
            Payload::Variant(val) => root.parse_variant(record.number, val)?,
            Payload::I32(val) => root.parse_i32(record.number, val)?,
            Payload::I64(val) => root.parse_i64(record.number, val)?,
            Payload::Len(mut child_read) => {
                if let Some(child) =
                    root.parse_len_read_or_alloc_child(record.number, &mut child_read)?
                {
                    deser_from_bound_read(child, child_read)?;
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::internal::variant::WriteExtVariant;
    use crate::internal::WireType;
    use ::futures::io::AsyncReadExt;
    use ::std::future::Future;

    #[derive(Default, Debug, PartialEq)]
    struct Field<T> {
        num: u32,
        val: T,
    }

    #[derive(Default, Debug, PartialEq)]
    struct SampleMessage {
        variants: Vec<Field<Variant>>,
        i32s: Vec<Field<u32>>,
        i64s: Vec<Field<u64>>,
        strings: Vec<Field<String>>,
        children: Vec<Field<Box<SampleMessage>>>,
    }

    impl DeseringMessage for SampleMessage {
        fn parse_variant(&mut self, num: u32, var: Variant) -> Result<()> {
            self.variants.push(Field { num, val: var });
            Ok(())
        }

        fn parse_i32(&mut self, num: u32, val: [u8; 4]) -> Result<()> {
            self.i32s.push(Field {
                num,
                val: u32::from_le_bytes(val),
            });
            Ok(())
        }

        fn parse_i64(&mut self, num: u32, val: [u8; 8]) -> Result<()> {
            self.i64s.push(Field {
                num,
                val: u64::from_le_bytes(val),
            });
            Ok(())
        }

        fn parse_len_slice_or_alloc_child(
            &mut self,
            num: u32,
            slice: &[u8],
        ) -> Result<Option<&mut dyn DeseringMessage>> {
            if num % 2 == 0 {
                self.strings.push(Field {
                    num,
                    val: String::from_utf8_lossy(slice).into_owned(),
                })
            } else {
                self.children.push(Field {
                    num,
                    val: Box::new(SampleMessage::default()),
                });
                return Ok(Some(self.children.last_mut().unwrap().val.as_mut()));
            }
            Ok(None)
        }

        fn parse_len_read_or_alloc_child(
            &mut self,
            num: u32,
            read: &mut Take<&mut dyn Read>,
        ) -> Result<Option<&mut dyn DeseringMessage>> {
            if num % 2 == 0 {
                let mut val = String::with_capacity(read.limit() as usize);
                read.read_to_string(&mut val)?;
                debug_assert_eq!(0, read.limit());
                self.strings.push(Field { num, val });
            } else {
                self.children.push(Field {
                    num,
                    val: Box::new(SampleMessage::default()),
                });
                return Ok(Some(self.children.last_mut().unwrap().val.as_mut()));
            }
            Ok(None)
        }
    }

    impl SampleMessage {
        fn parse_len_async_read_or_alloc_child<'this: 'r, 'r>(
            &'this mut self,
            num: u32,
            read: &'r mut AsyncTake<impl AsyncRead + Unpin>,
        ) -> Box<dyn 'r + Future<Output = Result<Option<&'this mut dyn DeseringMessage>>>> {
            let boxed = Box::new(async move {
                if num % 2 == 0 {
                    let mut val = String::with_capacity(read.limit() as usize);
                    read.read_to_string(&mut val).await?;
                    self.strings.push(Field { num, val });
                    Result::<_>::Ok(None)
                } else {
                    self.children.push(Field {
                        num,
                        val: Box::new(SampleMessage::default()),
                    });
                    Result::<_>::Ok(Some(
                        self.children.last_mut().unwrap().val.as_mut() as &mut dyn DeseringMessage
                    ))
                }
            });
            boxed
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

    fn gen_submessage_bytes(num: u32, submessage_bytes: impl AsRef<[u8]>) -> Vec<u8> {
        let mut result = Vec::new();
        result
            .write_variant(Variant::from(((num as u64) << 3) | WireType::Len as u64))
            .unwrap();
        result
            .write_variant(Variant::try_from(submessage_bytes.as_ref().len()).unwrap())
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
            val: 1.into(),
        });
        expected.variants.push(Field {
            num: 2,
            val: 3.into(),
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
            val: 3.into(),
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
                val: 1.into(),
            });
        expected.children[1].val.i32s.push(Field { num: 4, val: 3 });
        (msg1_bytes, expected)
    }

    #[test]
    fn test_slice_deser_variant_fields() {
        let (input, expected) = test_case_variant_fields();
        let mut msg = SampleMessage::default();
        deser_from_slice(&mut msg, &input).unwrap();
        assert_eq!(expected, msg);
    }

    #[test]
    fn test_slice_deser_fixed_fields() {
        let (input, expected) = test_case_fixed_fields();
        let mut msg = SampleMessage::default();
        deser_from_slice(&mut msg, &input).unwrap();
        assert_eq!(expected, msg);
    }

    #[test]
    fn test_slice_deser_string_fields() {
        let (input, expected) = test_case_string_fields();
        let mut msg = SampleMessage::default();
        deser_from_slice(&mut msg, &input).unwrap();
        assert_eq!(expected, msg);
    }

    #[test]
    fn test_slice_deser_complex_fields() {
        let (input, expected) = test_case_complex_fields();
        let mut msg = SampleMessage::default();
        deser_from_slice(&mut msg, &input).unwrap();
        assert_eq!(expected, msg);
    }

    #[test]
    fn test_read_deser_variant_fields() {
        let (input, expected) = test_case_variant_fields();
        let mut msg = SampleMessage::default();
        deser_from_read(&mut msg, input.as_slice()).unwrap();
        assert_eq!(expected, msg);
    }

    #[test]
    fn test_read_deser_fixed_fields() {
        let (input, expected) = test_case_fixed_fields();
        let mut msg = SampleMessage::default();
        deser_from_read(&mut msg, input.as_slice()).unwrap();
        assert_eq!(expected, msg);
    }

    #[test]
    fn test_read_deser_string_fields() {
        let (input, expected) = test_case_string_fields();
        let mut msg = SampleMessage::default();
        deser_from_read(&mut msg, input.as_slice()).unwrap();
        assert_eq!(expected, msg);
    }

    #[test]
    fn test_read_deser_complex_fields() {
        let (input, expected) = test_case_complex_fields();
        let mut msg = SampleMessage::default();
        deser_from_read(&mut msg, input.as_slice()).unwrap();
        assert_eq!(expected, msg);
    }
}
