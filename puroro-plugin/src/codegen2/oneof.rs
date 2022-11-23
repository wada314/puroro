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

use super::util::AnonymousCache;
use ::puroro_protobuf_compiled::google::protobuf::OneofDescriptorProto;
use ::std::fmt::Debug;
use ::std::rc::Rc;

pub trait Oneof: Debug {
    fn cache(&self) -> &AnonymousCache;
}

#[derive(Debug)]
pub struct OneofImpl {
    cache: AnonymousCache,
}

impl OneofImpl {
    #[allow(unused)]
    fn new(proto: &OneofDescriptorProto) -> Rc<Self> {
        Rc::new(OneofImpl {
            cache: Default::default(),
        })
    }
}

impl Oneof for OneofImpl {
    fn cache(&self) -> &AnonymousCache {
        &self.cache
    }
}
