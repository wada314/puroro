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
use crate::{ErrorKind, Result};
use ::puroro_protobuf_compiled::google::protobuf::field_descriptor_proto::Label;
use ::puroro_protobuf_compiled::google::protobuf::FieldDescriptorProto;

#[derive(Debug, Clone, Copy)]
pub(super) enum FieldRule {
    Optional,
    Singular,
    Repeated,
}

impl FieldRule {
    pub(super) fn try_new(label: Label, syntax: Syntax, proto3_optional: bool) -> Result<Self> {
        use FieldRule::*;
        use Label::*;
        Ok(match (label, proto3_optional, syntax) {
            (LabelOptional | LabelRequired, false, Syntax::Proto2) => Optional,
            (LabelRepeated, false, Syntax::Proto2) => Repeated,
            (LabelOptional, false, Syntax::Proto3) => Singular,
            (LabelOptional, true, Syntax::Proto3) => Optional,
            (LabelRepeated, false, Syntax::Proto3) => Repeated,
            (label, proto3_optional, syntax) => Err(ErrorKind::InvalidLabel {
                label: format!("{:?}", label),
                syntax: format!("{:?}", syntax),
                proto3_optional,
            })?,
        })
    }
}
