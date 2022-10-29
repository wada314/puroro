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

#[derive(Debug)]
pub struct Package {
    name: Option<String>,
    subpackages: HashMap<String, Package>,
    files: Vec<File>,
}

impl Package {
    pub fn try_new_from_files<'a, I: Iterator<Item = &'a FileDescriptorProto>>(
        iter: I,
    ) -> Result<Self> {
        let mut root = Package {
            name: None,
            subpackages: HashMap::new(),
            files: Vec::new(),
        };
        for file in iter {
            root.try_add_file(file)?;
        }
        Ok(root)
    }

    fn try_add_file(&mut self, file: &FileDescriptorProto) -> Result<()> {
        let leaf = file
            .package()
            .split('.')
            .try_fold(self, |package, name| -> Result<_> {
                let subpackage = package
                    .subpackages
                    .entry(name.to_string())
                    .or_insert_with(|| Package {
                        name: Some(name.to_string()),
                        subpackages: HashMap::new(),
                        files: Vec::new(),
                    });
                Ok(subpackage)
            })?;
        leaf.files.push(File::try_new(file)?);
        Ok(())
    }
}
