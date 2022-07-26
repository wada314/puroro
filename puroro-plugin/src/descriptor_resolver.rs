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

use super::descriptor_ext::{DescriptorExt, EnumDescriptorExt, FileDescriptorExt, RcMessageOrEnum};
use crate::utils::StrExt;
use crate::{ErrorKind, Result};
use ::itertools::Itertools;
use ::std::collections::HashMap;
use ::std::iter;
use ::std::rc::Rc;

#[derive(Debug)]
pub struct DescriptorResolver {
    fqtn_to_desc_map: HashMap<String, RcMessageOrEnum>,
    package_contents: HashMap<String, PackageContents>,
}
impl<'a> DescriptorResolver {
    pub fn new<I>(file_descriptors_iter: I) -> Result<Self>
    where
        I: Iterator<Item = Rc<FileDescriptorExt>>,
    {
        let mut fqtn_to_desc_map = HashMap::new();
        let mut package_contents: HashMap<_, PackageContents> = HashMap::new();
        for f in file_descriptors_iter {
            // fqtn_to_desc_map
            f.for_each_message(|m| {
                fqtn_to_desc_map.insert(
                    m.try_fqtn().unwrap().to_string(),
                    RcMessageOrEnum::Message(m),
                );
            });
            f.for_each_enum(|e| {
                fqtn_to_desc_map
                    .insert(e.try_fqtn().unwrap().to_string(), RcMessageOrEnum::Enum(e));
            });

            // package_contents
            let package_iter = f.package().split('.');
            let mut cur_package_vec: Vec<String> = Vec::new();
            for subpackage in package_iter {
                let cur_package_string = cur_package_vec.iter().join(".");
                let item = package_contents
                    .entry(cur_package_string.clone())
                    .or_insert_with(|| PackageContents {
                        name: cur_package_vec.last().cloned().unwrap_or_default(),
                        full_package: cur_package_string.clone(),
                        subpackages: Vec::new(),
                        input_files: Vec::new(),
                    });
                item.subpackages.push(subpackage.to_string());
                cur_package_vec.push(subpackage.to_string());
            }

            let last_package_string = cur_package_vec.iter().join(".");
            let term_item = package_contents
                .entry(last_package_string.clone())
                .or_default();
            term_item.name = cur_package_vec.last().cloned().unwrap_or_default();
            term_item.full_package = last_package_string;
            term_item.input_files.push(f.clone());
        }
        Ok(Self {
            fqtn_to_desc_map,
            package_contents,
        })
    }

    #[allow(unused)]
    pub fn fqtn_to_desc(&self, fqtn: &str) -> Option<RcMessageOrEnum> {
        self.fqtn_to_desc_map.get(fqtn).cloned()
    }

    #[allow(unused)]
    pub fn fqtn_to_desc_or_err(&self, fqtn: &str) -> Result<RcMessageOrEnum> {
        Ok(self
            .fqtn_to_desc(fqtn)
            .ok_or(ErrorKind::FqtnNotFound { fqtn: fqtn.into() })?)
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
pub struct PackageContents {
    pub name: String,
    pub full_package: String,
    pub subpackages: Vec<String>,
    pub input_files: Vec<Rc<FileDescriptorExt>>,
}

fn visit_messages_and_enums<VM, VE>(file: &FileDescriptorExt, visit_message: VM, visit_enum: VE)
where
    VM: FnMut(&DescriptorExt, &[&DescriptorExt]),
    VE: FnMut(&EnumDescriptorExt, &[&DescriptorExt]),
{
    todo!()
}
