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

use itertools::Itertools;

use crate::untyped_message::UntypedMessage;
use crate::Result;

pub struct EnumDescriptorProto<'a>(UntypedMessage<'a>);
impl<'a> EnumDescriptorProto<'a> {
    pub fn name(&self) -> Result<Option<&str>> {
        self.0.field(1).as_scalar_string()
    }
    pub fn value(&self) -> impl IntoIterator<Item = Result<EnumValueDescriptorProto<'a>>> {
        self.0
            .field(2)
            .as_repeated_message()
            .into_iter()
            .map_ok(EnumValueDescriptorProto)
    }
}

pub struct EnumValueDescriptorProto<'a>(UntypedMessage<'a>);
impl<'a> EnumValueDescriptorProto<'a> {
    pub fn name(&self) -> Result<Option<&str>> {
        self.0.field(1).as_scalar_string()
    }
    pub fn number(&self) -> Result<Option<i32>> {
        self.0
            .field(2)
            .as_scalar_variant(true)?
            .map(|v| v.try_as_int32())
            .transpose()
    }
}
