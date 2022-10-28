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
use ::std::pin::Pin;

#[derive(Debug)]
pub struct Package {
    name: Option<String>,
    subpackages: Vec<Pin<Box<Package>>>,
    files: Vec<Pin<Box<File>>>,
}

impl Package {
    pub fn new_from_files<'a, I: Iterator<Item = &'a FileDescriptorProto>>(
        iter: I,
    ) -> Result<Pin<Box<Self>>> {
        let mut root = Box::pin(Package {
            name: None,
            subpackages: Vec::new(),
            files: Vec::new(),
        });
        for file in iter {
            root.as_mut().add_file(file)?;
        }
        Ok(root)
    }

    fn add_file(self: Pin<&mut Self>, file: &FileDescriptorProto) -> Result<()> {
        let package_name = file.package();
        todo!()
    }
}
