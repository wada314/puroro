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

use ::once_cell::sync::Lazy;

pub struct MessageDescriptor {
    pub(crate) name: &'static str,
    pub(crate) lazy_fields: Lazy<&'static [FieldDescriptor]>,
}
impl MessageDescriptor {
    pub fn name(&self) -> &str {
        self.name
    }
    pub fn fields(&self) -> &'static [FieldDescriptor] {
        *Lazy::force(&self.lazy_fields)
    }
}

pub struct FieldDescriptor {
    pub(crate) name: &'static str,
    pub(crate) number: i32,
    pub(crate) lazy_containing_type: Lazy<&'static MessageDescriptor>,
}
impl FieldDescriptor {
    pub fn name(&self) -> &str {
        self.name
    }
    pub fn number(&self) -> i32 {
        self.number
    }
    pub fn containing_type(&self) -> &MessageDescriptor {
        *Lazy::force(&self.lazy_containing_type)
    }
}
