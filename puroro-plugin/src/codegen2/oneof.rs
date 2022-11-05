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

use super::*;
use crate::Result;
use ::puroro_protobuf_compiled::google::protobuf::OneofDescriptorProto;

pub trait OneofTrait: Sized {
    fn try_new(proto: &OneofDescriptorProto) -> Result<Self>;
}

#[derive(Debug)]
pub struct OneofImpl {}

pub type Oneof = OneofImpl;

impl OneofTrait for OneofImpl {
    fn try_new(proto: &OneofDescriptorProto) -> Result<Self> {
        todo!()
    }
}
