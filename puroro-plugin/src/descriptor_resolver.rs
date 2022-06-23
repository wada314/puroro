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

use crate::{ErrorKind, Result};
use ::puroro_protobuf_compiled::google::protobuf::{
    DescriptorProto, EnumDescriptorProto, FieldDescriptorProto, FileDescriptorProto,
    OneofDescriptorProto,
};
use ::std::collections::HashMap;
use ::std::sync::RwLock;

pub struct DescriptorResolver<'a> {
    files: Vec<&'a FileDescriptorProto>,
    fqtn_to_desc_cache: RwLock<HashMap<String, Option<MessageOrEnumDescriptor<'a>>>>,
}
impl<'a> DescriptorResolver<'a> {
    pub fn init<I>(file_descriptors_iter: I) -> Result<Self>
    where
        I: Iterator<Item = &'a FileDescriptorProto>,
    {
        Ok(Self {
            files: file_descriptors_iter.collect(),
            fqtn_to_desc_cache: Default::default(),
        })
    }

    pub fn fqtn_to_desc(&self, fqtn: &str) -> Option<MessageOrEnumDescriptor<'_>> {
        debug_assert!(!fqtn.starts_with('.'));
        // search for cache
        if let Ok(cache) = self.fqtn_to_desc_cache.read() {
            if let Some(desc) = cache.get(fqtn) {
                return desc.clone();
            }
        }

        fn find_from_file<'a, 'b>(
            file: &'a FileDescriptorProto,
            nested_type_name: &'b str,
        ) -> Option<MessageOrEnumDescriptor<'a>> {
            let (nested_msg_names, type_name) = {
                match nested_type_name.rsplit_once('.') {
                    Some((remains, type_name)) => (remains.split('.').collect(), type_name),
                    None => (vec![], nested_type_name),
                }
            };
            todo!()
        }

        fn find_from_messages<I: Iterator<Item = &DescriptorProto>>(
            messages: I,
            name: &str,
        ) -> Option<&DescriptorProto> {
            messages.find(|m| m.name() == name)
        }

        todo!()
    }
}

#[derive(Clone, Copy)]
pub enum MessageOrEnumDescriptor<'a> {
    Message(&'a DescriptorProto),
    Enum(&'a EnumDescriptorProto),
}
