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

use super::Syntax;
use crate::Result;
use ::puroro_protobuf_compiled::google::protobuf::field_descriptor_proto::Label;
use ::puroro_protobuf_compiled::google::protobuf::FieldDescriptorProto;

#[derive(Debug, Clone, Copy)]
pub(super) enum FieldRule {
    Optional,
    Singular,
    Repeated,
}

impl FieldRule {
    pub(super) fn try_from_field_descriptor(
        fd: &FieldDescriptorProto,
        syntax: Syntax,
    ) -> Result<Self> {
        use Label::*;
        match (fd.label_opt(), fd.proto3_optional(), syntax) {
            (None, _, _) => todo!(),
            (Some(LabelOptional | LabelRequired), _, Syntax::Proto2) => todo!(),
            (Some(LabelRepeated), _, Syntax::Proto2) => todo!(),
            (Some(LabelOptional), false, Syntax::Proto3) => todo!(),
            (Some(LabelOptional), true, Syntax::Proto3) => todo!(),
            (Some(LabelRequired), _, Syntax::Proto3) => todo!(),
            (Some(LabelRepeated), _, Syntax::Proto3) => todo!(),
        }
    }
}
