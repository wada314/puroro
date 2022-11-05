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

    fn try_make_package(
        name: &str,
        full_name: &str,
        sorted_fds: &[&FileDescriptorProto],
        root: Weak<Package>,
    ) -> Result<Package> {
        let (self_files, child_fds) = {
            let (self_fds, child_fds) = sorted_fds.split_until(|fd| fd.package() == full_name);
            let self_files = self_fds
                .into_iter()
                .map(|fd| File::try_new(fd))
                .collect::<Result<Vec<_>>>()?;
            (self_files, child_fds)
        };

        fn get_package_name<'a>(fd: &'a FileDescriptorProto, full_name: &str) -> &'a str {
            let striped_path = if full_name.is_empty() {
                fd.package()
            } else {
                &fd.package()[full_name.len() + 1..]
            };
            if let Some((name, _)) = striped_path.split_once('.') {
                name
            } else {
                striped_path
            }
        };

        let subpackages = child_fds.group_by_key(|fd| get_package_name(fd, full_name));

        let mut package = Package {
            name: Some(name.to_string()),
            subpackages: todo!(),
            files: self_files,
            root,
        };
    }
}
