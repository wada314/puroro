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
use ::puroro_protobuf_compiled::google::protobuf::FileDescriptorProto;
use ::std::collections::HashMap;
use ::std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Package {
    name: Option<String>,
    subpackages: HashMap<String, Package>,
    files: Vec<File>,
    root: Weak<Package>,
}

impl Package {
    pub fn new_from_files<'a, I: Iterator<Item = &'a FileDescriptorProto>>(iter: I) -> Rc<Self> {
        let mut files = iter.collect::<Vec<_>>();
        files.sort_by_key(|f| f.package());
        Rc::new_cyclic(|weak_root| todo!())
    }

    fn make_package(name: &str, full_name: &str, files: &[&FileDescriptorProto]) -> Package {
        let prefix_len = if full_name.is_empty() {
            0
        } else {
            full_name.len() + 1
        };
        let get_subpackage_name = |f: &FileDescriptorProto| -> Option<&str> {
            f.package()
                .split_at(prefix_len)
                .1
                .split_once('.')
                .map(|(subp, _)| subp)
        };
        let grouped = files.group_by(|f1, f2| get_subpackage_name(f1) == get_subpackage_name(f2));
        todo!()
    }
}
