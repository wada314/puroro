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

use super::descriptor_ext::FileDescriptorExt;
use crate::restructure::{File, MessageOrEnumRef};
use crate::utils::{Fqtn, Package};
use crate::{ErrorKind, Result};
use ::puroro_protobuf_compiled::google::protobuf::FileDescriptorProto;
use ::std::borrow::Borrow;
use ::std::collections::HashMap;
use ::std::fmt::Debug;

#[derive(Debug)]
pub struct DescriptorResolver<'a> {
    fqtn_to_desc_map: HashMap<Fqtn<String>, MessageOrEnumRef<'a>>,
    package_contents: HashMap<String, PackageContents<'a>>,
}
impl<'a> DescriptorResolver<'a> {
    pub fn new<I>(input_files: I) -> Result<Self>
    where
        I: Iterator<Item = &'a File<'a>>,
    {
        let mut fqtn_to_desc_map = HashMap::new();
        let mut package_contents = HashMap::new();
        for f in input_files {
            Self::generate_fqtn_to_desc_map(&mut fqtn_to_desc_map, f);
            Self::generate_package_contents(&mut package_contents, f.proto());
        }
        Ok(Self {
            fqtn_to_desc_map,
            package_contents,
        })
    }

    fn generate_fqtn_to_desc_map(
        fqtn_to_desc_map: &mut HashMap<Fqtn<String>, MessageOrEnumRef<'a>>,
        file: &'a File<'a>,
    ) {
        for &m in file.all_messages() {
            fqtn_to_desc_map.insert(m.fqtn().to_owned(), MessageOrEnumRef::Message(m));
        }
        for &e in file.all_enums() {
            fqtn_to_desc_map.insert(e.fqtn().to_owned(), MessageOrEnumRef::Enum(e));
        }
    }

    fn generate_package_contents(
        package_contents: &mut HashMap<String, PackageContents<'a>>,
        file: &'a FileDescriptorProto,
    ) {
        // package_contents for parent packages
        for (cur_package, subpackage) in file.package_ext().packages_and_subpackages() {
            let item = package_contents
                .entry(cur_package.full_package_path().to_string())
                .or_insert_with(|| PackageContents {
                    package_name: cur_package.leaf_package_name().map(|s| s.to_string()),
                    full_package: cur_package.to_owned(),
                    subpackages: Vec::new(),
                    input_files: Vec::new(),
                });
            item.subpackages.push(subpackage.to_string());
        }

        // package_contents for the leaf package
        let term_item = package_contents
            .entry(file.package().to_string())
            .or_default();
        term_item.package_name = file
            .package_ext()
            .leaf_package_name()
            .map(|s| s.to_string());
        term_item.full_package = file.package_ext().to_owned();
        term_item.input_files.push(File::new(file))
    }

    pub fn fqtn_to_desc<S: Borrow<str>>(&self, fqtn: &Fqtn<S>) -> Option<MessageOrEnumRef<'a>> {
        self.fqtn_to_desc_map.get::<str>(fqtn.borrow()).cloned()
    }

    #[allow(unused)]
    pub fn fqtn_to_desc_or_err<S: Borrow<str> + Debug>(
        &self,
        fqtn: &Fqtn<S>,
    ) -> Result<MessageOrEnumRef<'a>> {
        Ok(self.fqtn_to_desc(fqtn).ok_or(ErrorKind::UnknownTypeName {
            name: format!("{:?}", fqtn),
        })?)
    }

    pub fn package_contents(&self, package: &str) -> Option<&PackageContents<'a>> {
        self.package_contents.get(package)
    }

    pub fn package_contents_or_err(&self, package: &str) -> Result<&PackageContents<'a>> {
        Ok(self
            .package_contents(package)
            .ok_or(ErrorKind::UnknownTypeName {
                name: package.into(),
            })?)
    }

    #[allow(unused)]
    pub fn all_packages(&'a self) -> impl Iterator<Item = &PackageContents<'a>> {
        self.package_contents.values()
    }
}

#[derive(Debug, Default)]
pub struct PackageContents<'a> {
    pub package_name: Option<String>,
    pub full_package: Package<String>,
    pub subpackages: Vec<String>,
    pub input_files: Vec<File<'a>>,
}
