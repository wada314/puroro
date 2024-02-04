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

use self::record::{Record, SliceExtReadRecord};
use crate::internal::freezing_mut::{FreezeStatus, FrozenMut, UnfrozenMut};
use crate::{ErrorKind, Result};

#[derive(Default)]
struct Stack<T> {
    vec: Vec<T>,
}
impl<T> Stack<T> {
    fn new() -> Self {
        Self { vec: Vec::new() }
    }
    fn push(&mut self, elem: T) {
        self.vec.push(elem);
    }
    fn try_pop(&mut self) -> Result<()> {
        self.vec.pop().ok_or(ErrorKind::DeserError)?;
        Ok(())
    }
    fn len(&self) -> usize {
        self.vec.len()
    }
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn check_last_mut_and_maybe_push<'a>(
        &mut self,
        f: impl FnOnce(&'a mut T) -> Result<Option<T>>,
    ) -> Result<()>
    where
        T: 'a,
    {
        // Grabbing the mut borrow of the last element in the stack,
        // without grabbing the mut borrow of the stack itself.
        let last = unsafe { &mut *(self as *mut Self) }
            .vec
            .last_mut()
            .ok_or(ErrorKind::DeserError)?;
        if let Some(child) = (f)(last)? {
            self.vec.push(child);
        }
        Ok(())
    }
}

pub trait DeseringMessage {
    // fn try_parse_slice_record<'slice>(
    //     &mut self,
    //     record: Record<&'slice [u8]>,
    // ) -> Result<Option<(&mut dyn DeseringMessage, &'slice [u8])>>;
    fn parse_slice_record_or_alloc_child(
        &mut self,
        record: &Record<&[u8]>,
    ) -> Result<Option<&mut dyn DeseringMessage>>;
}

pub fn deser_from_slice(root: &mut dyn DeseringMessage, mut input: &[u8]) -> Result<()> {
    let mut msg = UnfrozenMut::new(root);
    let mut stack = Vec::new();
    while !(input.is_empty() && stack.is_empty()) {
        if !input.is_empty() {
            let record = input.read_record()?;
            match msg.try_work(|msg| msg.parse_slice_record_or_alloc_child(&record))? {
                FreezeStatus::Unfrozen(same) => {
                    msg = same;
                }
                FreezeStatus::Frozen(prev, new_msg) => {
                    stack.push((input, prev));
                    msg = new_msg;
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

// pub fn deser_from_slice<'a>(root: &'a mut dyn DeseringMessage, input: &'a [u8]) -> Result<()> {
//     let mut stack = Stack::new();
//     stack.push((root, input));
//     while !stack.is_empty() {
//         let mut input_is_empty = false;
//         stack.check_last_mut_and_maybe_push(|(msg, input)| {
//             if input.is_empty() {
//                 input_is_empty = true;
//                 Ok(None)
//             } else {
//                 let record = input.read_record()?;
//                 Ok(msg.try_parse_slice_record(record)?)
//             }
//         })?;
//         if input_is_empty {
//             stack.try_pop()?;
//         }
//     }
//     Ok(())
// }

#[cfg(test)]
mod test {
    use super::*;
    use crate::internal::deser::record::{Payload, Record};
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
        fn parse_slice_record_or_alloc_child(
            &mut self,
            record: &Record<&[u8]>,
        ) -> Result<Option<&mut dyn DeseringMessage>> {
            let num = record.number;
            match record.payload {
                Payload::Variant(val) => self.variants.push(Field { num, val }),
                Payload::I32(b4) => self.i32s.push(Field {
                    num,
                    val: u32::from_le_bytes(b4),
                }),
                Payload::I64(b8) => self.i64s.push(Field {
                    num,
                    val: u64::from_le_bytes(b8),
                }),
                Payload::Len(slice) => {
                    if num % 2 == 0 {
                        self.strings.push(Field {
                            num,
                            val: String::from_utf8_lossy(slice).into_owned(),
                        })
                    } else {
                        self.children.push(Field {
                            num,
                            val: Box::new(Default::default()),
                        });
                        return Ok(Some(self.children.last_mut().unwrap().val.as_mut()));
                    }
                }
            }
            Ok(None)
        }

        // fn try_parse_slice_record<'slice>(
        //     &mut self,
        //     record: Record<&'slice [u8]>,
        // ) -> Result<Option<(&mut dyn DeseringMessage, &'slice [u8])>> {
        //     let num = record.number;
        //     match record.payload {
        //         Payload::Variant(val) => self.variants.push(Field { num, val }),
        //         Payload::I32(b4) => self.i32s.push(Field {
        //             num,
        //             val: u32::from_le_bytes(b4),
        //         }),
        //         Payload::I64(b8) => self.i64s.push(Field {
        //             num,
        //             val: u64::from_le_bytes(b8),
        //         }),
        //         Payload::Len(slice) => {
        //             if num % 2 == 0 {
        //                 self.strings.push(Field {
        //                     num,
        //                     val: String::from_utf8_lossy(slice).into_owned(),
        //                 })
        //             } else {
        //                 self.children.push(Field {
        //                     num,
        //                     val: Box::new(Default::default()),
        //                 });
        //                 return Ok(Some((
        //                     self.children.last_mut().unwrap().val.as_mut(),
        //                     slice,
        //                 )));
        //             }
        //         }
        //     }
        //     Ok(None)
        // }
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
