// Copyright 2021 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub mod bool;
pub mod de;
pub mod fixed_bits;
pub mod impls;
pub mod se;
pub mod types;
pub mod variant;

use crate::desc::{FieldDescriptor, MessageDescriptor};
use crate::once_cell::sync::Lazy;
use ::bitvec::order::BitOrder;
use ::bitvec::slice::BitSlice;
use ::bitvec::store::BitStore;

pub fn get_bitvec_bit<O, T>(slice: &BitSlice<O, T>, index: usize) -> bool
where
    O: BitOrder,
    T: BitStore,
{
    *slice.get(index).expect("bitvec index out of bound.")
}

// Limitation: BITS_LEN <= 29, the return value is always positive or 0.
// Can express a field number index (<= 2^29-1).
pub fn get_bitvec_range<O, T>(slice: &BitSlice<O, T>, start_bit: usize, bits_len: usize) -> i32
where
    O: BitOrder,
    T: BitStore,
{
    debug_assert!(bits_len <= 29);
    let mut result = 0i32;
    // I don't want to optimize this by hand...
    for i in 0..bits_len {
        if slice.get(start_bit + i).map_or(false, |b| *b) {
            result |= 1 << i;
        }
    }
    result
}
pub fn set_bitvec_range<O, T>(
    slice: &mut BitSlice<O, T>,
    start_bit: usize,
    bits_len: usize,
    val: i32,
) where
    O: BitOrder,
    T: BitStore,
{
    debug_assert!(bits_len <= 29);
    debug_assert!(val >= 0);
    debug_assert!(
        32 >= i32::leading_zeros(val) as usize + bits_len,
        "Too short BITS_LEN({}) for the input val({})",
        bits_len,
        val
    );
    for i in 0..bits_len {
        slice.set(start_bit + i, (val & (1 << i)) != 0);
    }
}

pub struct MessageDescriptorInitializer {
    pub name: &'static str,
    pub lazy_fields: Lazy<&'static [FieldDescriptor]>,
}

pub fn init_message_descriptor(init: MessageDescriptorInitializer) -> MessageDescriptor {
    MessageDescriptor {
        name: init.name,
        lazy_fields: init.lazy_fields,
    }
}

pub struct FieldDescriptorInitializer {
    pub name: &'static str,
    pub number: i32,
    pub lazy_containing_type: Lazy<&'static MessageDescriptor>,
}

pub fn init_field_descriptor(init: FieldDescriptorInitializer) -> FieldDescriptor {
    FieldDescriptor {
        name: init.name,
        number: init.number,
        lazy_containing_type: init.lazy_containing_type,
    }
}

#[test]
fn test_get_bitvec_range() {
    use crate::bitvec::array::BitArray;
    use crate::bitvec::order::Lsb0;

    let all_1: BitArray<Lsb0, [u8; 4]> = [0xffu8; 4].into();
    let bytes_33: BitArray<Lsb0, [u8; 4]> = [0x33u8; 4].into();
    let bytes_cc: BitArray<Lsb0, [u8; 4]> = [0xccu8; 4].into();

    // within 1 byte range
    assert_eq!(0b_0000_0001, get_bitvec_range(&all_1, 0, 1));
    assert_eq!(0b_0000_0011, get_bitvec_range(&all_1, 0, 2));
    assert_eq!(0b_0111_1111, get_bitvec_range(&all_1, 0, 7));
    assert_eq!(0b_1111_1111, get_bitvec_range(&all_1, 0, 8));
    assert_eq!(0b_0000_0001, get_bitvec_range(&all_1, 1, 1));
    assert_eq!(0b_0000_0011, get_bitvec_range(&all_1, 1, 2));
    assert_eq!(0b_0011_1111, get_bitvec_range(&all_1, 1, 6));
    assert_eq!(0b_0111_1111, get_bitvec_range(&all_1, 1, 7));
    assert_eq!(0b_0000_0001, get_bitvec_range(&all_1, 6, 1));
    assert_eq!(0b_0000_0011, get_bitvec_range(&all_1, 6, 2));
    assert_eq!(0b_0000_0001, get_bitvec_range(&all_1, 7, 1));

    assert_eq!(0b_0000_0001, get_bitvec_range(&bytes_33, 0, 1));
    assert_eq!(0b_0000_0011, get_bitvec_range(&bytes_33, 0, 2));
    assert_eq!(0b_0011_0011, get_bitvec_range(&bytes_33, 0, 7));
    assert_eq!(0b_0011_0011, get_bitvec_range(&bytes_33, 0, 8));
    assert_eq!(0b_0000_0001, get_bitvec_range(&bytes_33, 1, 1));
    assert_eq!(0b_0000_0001, get_bitvec_range(&bytes_33, 1, 2));
    assert_eq!(0b_0001_1001, get_bitvec_range(&bytes_33, 1, 6));
    assert_eq!(0b_0001_1001, get_bitvec_range(&bytes_33, 1, 7));
    assert_eq!(0b_0000_0000, get_bitvec_range(&bytes_33, 6, 1));
    assert_eq!(0b_0000_0000, get_bitvec_range(&bytes_33, 6, 2));
    assert_eq!(0b_0000_0000, get_bitvec_range(&bytes_33, 7, 1));

    assert_eq!(0b_0000_0000, get_bitvec_range(&bytes_cc, 0, 1));
    assert_eq!(0b_0000_0000, get_bitvec_range(&bytes_cc, 0, 2));
    assert_eq!(0b_0100_1100, get_bitvec_range(&bytes_cc, 0, 7));
    assert_eq!(0b_1100_1100, get_bitvec_range(&bytes_cc, 0, 8));
    assert_eq!(0b_0000_0000, get_bitvec_range(&bytes_cc, 1, 1));
    assert_eq!(0b_0000_0010, get_bitvec_range(&bytes_cc, 1, 2));
    assert_eq!(0b_0010_0110, get_bitvec_range(&bytes_cc, 1, 6));
    assert_eq!(0b_0110_0110, get_bitvec_range(&bytes_cc, 1, 7));
    assert_eq!(0b_0000_0001, get_bitvec_range(&bytes_cc, 6, 1));
    assert_eq!(0b_0000_0011, get_bitvec_range(&bytes_cc, 6, 2));
    assert_eq!(0b_0000_0001, get_bitvec_range(&bytes_cc, 7, 1));

    // among multiple bytes
    assert_eq!(0b_1111_1111, get_bitvec_range(&all_1, 1, 8));
    assert_eq!(0b_1111_1111, get_bitvec_range(&all_1, 2, 8));
    assert_eq!(0b_1111_1111, get_bitvec_range(&all_1, 6, 8));
    assert_eq!(0b_1111_1111, get_bitvec_range(&all_1, 7, 8));
    assert_eq!(0b_0011_1111, get_bitvec_range(&all_1, 3, 6));
    assert_eq!(0b_0011_1111, get_bitvec_range(&all_1, 4, 6));
    assert_eq!(0b_0011_1111, get_bitvec_range(&all_1, 6, 6));
    assert_eq!(0b_0011_1111, get_bitvec_range(&all_1, 7, 6));
}
