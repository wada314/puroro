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

use super::descriptor_ext::{DescriptorExt, EnumDescriptorExt, FileDescriptorExt};
use crate::{ErrorKind, Result};
use ::std::collections::HashMap;
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
            let packages = f.package().split('.');
            let mut current_package = "".to_string();
            for subpackage in packages {
                let item = package_contents.entry(current_package.clone()).or_default();
                item.subpackages.push(subpackage.to_string());
                current_package.push_str(&format!(".{}", subpackage));
            }
            let item = package_contents.entry(current_package.clone()).or_default();
            item.messages.extend(f.message_type().iter().cloned());
            item.enums.extend(f.enum_type().iter().cloned());
        }
        Ok(Self {
            fqtn_to_desc_map,
            package_contents,
        })
    }

    pub fn fqtn_to_desc(&self, fqtn: &str) -> Option<RcMessageOrEnum> {
        self.fqtn_to_desc_map.get(fqtn).cloned()
    }

    pub fn fqtn_to_desc_or_err(&self, fqtn: &str) -> Result<RcMessageOrEnum> {
        Ok(self.fqtn_to_desc(fqtn).ok_or(ErrorKind::FqtnNotFound)?)
    }

    pub fn package_contents(&self, package: &str) -> Option<&PackageContents> {
        self.package_contents.get(package).as_ref()
    }

    pub fn package_contents_or_err(&self, package: &str) -> Result<&PackageContents> {
        Ok(self
            .package_contents(package)
            .ok_or(ErrorKind::FqtnNotFound)?)
    }
}

#[derive(Debug, Clone)]
pub enum RcMessageOrEnum {
    Message(Rc<DescriptorExt>),
    Enum(Rc<EnumDescriptorExt>),
}

#[derive(Debug, Default)]
pub struct PackageContents {
    subpackages: Vec<String>,
    messages: Vec<Rc<DescriptorExt>>,
    enums: Vec<Rc<EnumDescriptorExt>>,
}
