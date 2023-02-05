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

pub mod bitvec;
pub mod field_type;
pub mod message_internal;
pub mod oneof_field_type;
pub mod oneof_type;
pub mod ser;
pub mod shared;
pub mod tags;
pub mod unknown_fields;
pub mod utils;
pub mod variant;

pub use self::bitvec::{BitArray, BitSlice};
pub use self::field_type::{
    FieldType, NonRepeatedFieldType, OptionalNumericalField, OptionalUnsizedField,
    RepeatedFieldType, RepeatedMessageField, RepeatedNumericalField, RepeatedUnsizedField,
    SingularHeapMessageField, SingularNumericalField, SingularUnsizedField,
};
pub use self::message_internal::MessageInternal;
pub use self::oneof_field_type::{HeapMessageField, NumericalField, OneofFieldType, UnsizedField};
pub use self::oneof_type::{OneofCase, OneofUnion};
pub use self::ser::{PosIter, ScopedIter};
pub use self::shared::{SharedItems, SharedItemsImpl};
pub use self::unknown_fields::{UnknownFields, UnknownFieldsImpl};
