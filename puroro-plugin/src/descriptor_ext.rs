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

//! Extend the raw protobuf descriptors to add a pointer to the parent descriptor.

use crate::{ErrorKind, Result};
use ::puroro_protobuf_compiled::google::protobuf::{
    DescriptorProto, EnumDescriptorProto, FieldDescriptorProto, FileDescriptorProto,
    OneofDescriptorProto,
};
use ::std::rc::{Rc, Weak};

pub struct DescriptorExt {
    proto: DescriptorProto,
    field: Vec<Rc<FieldDescriptorExt>>,
}

pub struct FieldDescriptorExt {
    proto: FieldDescriptorProto,
    parent_proto: Weak<DescriptorExt>,
}


