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

use self::record::{Payload, SliceExtReadRecord};
use crate::internal::freezing_mut::{FreezeStatus, UnfrozenMut};
use crate::internal::variant::Variant;
use crate::Result;
use ::std::io::{Read, Take};

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
        take: &mut Take<&mut dyn Read>,
    ) -> Result<Option<&mut dyn DeseringMessage>>;
}

pub fn deser_from_slice(root: &mut dyn DeseringMessage, mut input: &[u8]) -> Result<()> {
    let mut msg = UnfrozenMut::new(root);
    let mut stack = Vec::new();
    while !(input.is_empty() && stack.is_empty()) {
        if !input.is_empty() {
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
            let (prev_input, prev_msg) = stack.pop().unwrap();
            input = prev_input;
            msg = prev_msg.unfreeze(msg);
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::internal::variant::{Variant, WriteExtVariant};
    use crate::internal::WireType;

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
            take: &mut Take<&mut dyn Read>,
        ) -> Result<Option<&mut dyn DeseringMessage>> {
            if num % 2 == 0 {
                let mut val = String::with_capacity(take.limit() as usize);
                take.read_to_string(&mut val)?;
                debug_assert_eq!(0, take.limit());
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

    #[test]
    fn test_deser_variant_fields() {
        let input = [INPUT_FIELD_1_VARIANT_1, INPUT_FIELD_2_VARIANT_3]
            .into_iter()
            .flatten()
            .copied()
            .collect::<Vec<_>>();
        let mut msg1 = SampleMessage::default();

        deser_from_slice(&mut msg1, &input).unwrap();

        let mut expected_msg1 = SampleMessage::default();
        expected_msg1.variants.push(Field {
            num: 1,
            val: 1.into(),
        });
        expected_msg1.variants.push(Field {
            num: 2,
            val: 3.into(),
        });

        assert_eq!(expected_msg1, msg1);
    }

    #[test]
    fn test_deser_fixed_fields() {
        let input = [
            INPUT_FIELD_3_I32_1,
            INPUT_FIELD_4_I32_3,
            INPUT_FIELD_5_I64_1,
            INPUT_FIELD_6_I64_3,
        ]
        .into_iter()
        .flatten()
        .copied()
        .collect::<Vec<_>>();
        let mut msg1 = SampleMessage::default();

        deser_from_slice(&mut msg1, &input).unwrap();

        let mut expected_msg1 = SampleMessage::default();
        expected_msg1.i32s.push(Field { num: 3, val: 1 });
        expected_msg1.i32s.push(Field { num: 4, val: 3 });
        expected_msg1.i64s.push(Field { num: 5, val: 1 });
        expected_msg1.i64s.push(Field { num: 6, val: 3 });

        assert_eq!(expected_msg1, msg1);
    }

    #[test]
    fn test_deser_string_fields() {
        let input = [INPUT_FIELD_8_STRING_FOO, INPUT_FIELD_10_STRING_YO]
            .into_iter()
            .flatten()
            .copied()
            .collect::<Vec<_>>();
        let mut msg1 = SampleMessage::default();

        deser_from_slice(&mut msg1, &input).unwrap();

        let mut expected_msg1 = SampleMessage::default();
        expected_msg1.strings.push(Field {
            num: 8,
            val: "foo".to_string(),
        });
        expected_msg1.strings.push(Field {
            num: 10,
            val: "yo".to_string(),
        });
        assert_eq!(expected_msg1, msg1);
    }

    #[test]
    fn test_deser_complex_fields() {
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
        let mut msg1 = SampleMessage::default();

        deser_from_slice(&mut msg1, &msg1_bytes).unwrap();

        let mut expected_msg1 = SampleMessage::default();
        expected_msg1.i32s.push(Field { num: 3, val: 1 });
        expected_msg1.children.push(Field {
            num: 7,
            val: Box::new(SampleMessage::default()),
        });
        expected_msg1.children.push(Field {
            num: 5,
            val: Box::new(SampleMessage::default()),
        });
        expected_msg1.children[0].val.variants.push(Field {
            num: 2,
            val: 3.into(),
        });
        expected_msg1.children[0].val.children.push(Field {
            num: 3,
            val: Box::new(SampleMessage::default()),
        });
        expected_msg1.children[0].val.children[0]
            .val
            .variants
            .push(Field {
                num: 1,
                val: 1.into(),
            });
        expected_msg1.children[1]
            .val
            .i32s
            .push(Field { num: 4, val: 3 });
        assert_eq!(expected_msg1, msg1);
    }
}
