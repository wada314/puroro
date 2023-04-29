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

/*
```protobuf
syntax = "proto3";
message Person {
    string name = 1;
    repeated string full_name = 2;
    uint32 age = 3;
    repeated uint32 id = 4;
    Person partner = 5;
    repeated children = 6;
}
```
 */

use crate::Result;

pub trait Message {
    fn try_get_u32(&self) -> Result<u32>;
    fn try_get_string(&self) -> Result<&str>;
    fn try_get_message(&self) -> Result<&dyn Message>;
}

pub trait PersonTrait {
    fn try_age(&self) -> Result<u32>;
    fn try_name(&self) -> Result<&str>;
    fn try_partner(&self) -> Result<&dyn PersonTrait>;
}

#[repr(transparent)]
struct PersonWrapper<M>(M);
impl<M> ::std::ops::Deref for PersonWrapper<M> {
    type Target = M;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<M: Message> PersonTrait for PersonWrapper<M> {
    fn try_age(&self) -> Result<u32> {
        <M as Message>::try_get_u32(&self.0)
    }
    fn try_name(&self) -> Result<&str> {
        <M as Message>::try_get_string(&self.0)
    }
    fn try_partner(&self) -> Result<&dyn PersonTrait> {
        // NEEDFIX
        <M as Message>::try_get_message(&self.0)
    }
}
