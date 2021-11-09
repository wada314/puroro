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
