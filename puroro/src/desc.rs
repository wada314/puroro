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

pub trait DescriptorProtoTrait {
    type NameValueType<'a>: AsRef<str>
    where
        Self: 'a;
    fn name(&self) -> Self::NameValueType<'_>;
    type FieldValueType<'a>: FieldDescriptorProtoTrait
    where
        Self: 'a;
    type FieldIteratorType<'a>: Iterator<Item = Self::FieldValueType<'a>>
    where
        Self: 'a;
    fn field(&self) -> Self::FieldIteratorType<'_>;
}

pub trait FieldDescriptorProtoTrait {
    type NameValueType<'a>: AsRef<str>
    where
        Self: 'a;
    fn name(&self) -> Self::NameValueType<'_>;
    fn number(&self) -> u32;
    fn label(&self) -> field_descriptor_proto::Label;
    fn r#type(&self) -> field_descriptor_proto::Type;
}

pub mod field_descriptor_proto {
    pub enum Label {
        LabelRequired,
        LabelOptional,
        LabelRepeated,
    }
    pub enum Type {
        TypeU32,
        TypeString,
        TypeMessage,
    }
}
