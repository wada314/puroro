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

use super::ser::FieldData;
use ::std::collections::HashMap;
use ::std::fmt::{DebugStruct, Result as FmtResult};

pub trait UnknownFields {
    fn debug_struct_fields<'a, 'b>(&self, debug_struct: &mut DebugStruct<'a, 'b>) -> FmtResult;
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct UnknownFieldsImpl {
    fields: HashMap<i32, Vec<FieldData<Vec<u8>>>>,
}

impl UnknownFields for UnknownFieldsImpl {
    fn debug_struct_fields<'a, 'b>(&self, debug_struct: &mut DebugStruct<'a, 'b>) -> FmtResult {
        debug_struct.field("TODO_Maybe_unknown_fields", &());
        Ok(())
    }
}
