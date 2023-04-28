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

pub trait PersonTrait: Message {
    fn try_age(&self) -> Result<u32> {
        <Self as Message>::try_get_u32(self)
    }
    fn try_name(&self) -> Result<&str> {
        <Self as Message>::try_get_string(self)
    }
    fn try_partner(&self) -> Result<&dyn PersonTrait> {
        <Self as Message>::try_get_message(self).map(|m| m as &dyn PersonTrait)
    }
}

impl PersonTrait for dyn Message {}
