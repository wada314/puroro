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

pub mod getter;
pub mod getter_opt;

use crate::internal::{FieldsContainer, HasField};
use crate::tags;
use crate::Message;

#[derive(Default, Clone)]
pub struct OptionFields;
impl FieldsContainer for OptionFields {}
impl<const NUMBER: i32> HasField<NUMBER> for OptionFields {
    type Type = ();
    fn get(&self) -> &Self::Type {
        &()
    }
    fn get_mut(&mut self) -> &mut Self::Type {
        unreachable!()
    }
}

#[derive(Default, Clone, Debug)]
pub struct OptionShared<T> {
    option: Option<T>,
}
impl<T> From<Option<T>> for OptionShared<T> {
    fn from(v: Option<T>) -> Self {
        Self { option: v }
    }
}
impl<MP, T> From<Option<T>> for Message<MP, tags::OptionImpl, OptionFields, OptionShared<T>> {
    fn from(val: Option<T>) -> Self {
        Message {
            fields: Default::default(),
            shared: Into::into(val),
            _phantom: Default::default(),
        }
    }
}
