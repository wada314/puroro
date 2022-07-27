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

use super::descriptor_ext::{FileDescriptorExt, MessageOrEnum};
use crate::{ErrorKind, Result};
use ::itertools::Itertools;
use ::puroro_protobuf_compiled::google::protobuf::{
    DescriptorProto, EnumDescriptorProto, FileDescriptorProto,
};
use ::std::collections::HashMap;

#[derive(Debug)]
pub struct DescriptorResolver<'a> {
    fqtn_to_desc_map: HashMap<String, &'a dyn MessageOrEnum>,
    package_contents: HashMap<String, PackageContents<'a>>,
}
impl<'a> DescriptorResolver<'a> {
    pub fn new<I>(file_descriptors_iter: I) -> Result<Self>
    where
        I: Iterator<Item = &'a FileDescriptorProto>,
    {
        let mut fqtn_to_desc_map = HashMap::new();
        let mut package_contents: HashMap<_, PackageContents> = HashMap::new();
        for f in file_descriptors_iter {
            // package_contents for parent packages
            for (cur_package, subpackage) in f.package_ext().packages_and_subpackages() {
                let item = package_contents
                    .entry(cur_package.clone())
                    .or_insert_with(|| PackageContents {
                        name: cur_package
                            .rsplit_once('.')
                            .map_or(String::default(), |(_, name)| name.to_string()),
                        full_package: cur_package.clone(),
                        subpackages: Vec::new(),
                        input_files: Vec::new(),
                    });
                item.subpackages.push(subpackage.to_string());
            }

            // package_contents for the leaf package
            let term_item = package_contents.entry(f.package().to_string()).or_default();
            term_item.name = f
                .package()
                .rsplit_once('.')
                .map_or(String::default(), |(_, name)| name.to_string());
            term_item.full_package = f.package().to_string();
            term_item.input_files.push(f);
        }
        Ok(Self {
            fqtn_to_desc_map,
            package_contents,
        })
    }

    pub fn package_contents(&self, package: &str) -> Option<&PackageContents> {
        self.package_contents.get(package)
    }

    pub fn package_contents_or_err(&self, package: &str) -> Result<&PackageContents> {
        Ok(self
            .package_contents(package)
            .ok_or(ErrorKind::FqtnNotFound {
                fqtn: package.into(),
            })?)
    }

    #[allow(unused)]
    pub fn all_packages(&self) -> impl Iterator<Item = &PackageContents> {
        dbg!(&self.package_contents);
        self.package_contents.values()
    }
}

#[derive(Debug, Default)]
pub struct PackageContents<'a> {
    pub name: String,
    pub full_package: String,
    pub subpackages: Vec<String>,
    pub input_files: Vec<&'a FileDescriptorProto>,
}

fn visit_messages_and_enums<VM, VE>(file: &FileDescriptorProto, visit_message: VM, visit_enum: VE)
where
    VM: FnMut(&DescriptorProto, &[&DescriptorProto]),
    VE: FnMut(&EnumDescriptorProto, &[&DescriptorProto]),
{
    // let mut path = Vec::new();
    todo!()
}
