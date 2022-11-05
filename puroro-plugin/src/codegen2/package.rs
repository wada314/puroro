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
use std::borrow::Cow;

#[derive(Debug)]
pub struct Package<FileType> {
    name: Option<String>,
    subpackages: HashMap<String, Package<FileType>>,
    files: Vec<FileType>,
    root: Weak<Package<FileType>>,
}

impl<FileType: File> Package<FileType> {
    pub fn new_from_files<'a, I: Iterator<Item = &'a FileDescriptorProto>>(iter: I) -> Rc<Self> {
        let mut files = iter.collect::<Vec<_>>();
        files.sort_by_key(|f| f.package());
        Rc::new_cyclic(|weak_root| {
            Package::try_make_package("", "", &files, weak_root.clone()).unwrap()
        })
    }

    fn try_make_package(
        name: &str,
        full_name: &str,
        sorted_fds: &[&FileDescriptorProto],
        root: Weak<Package<FileType>>,
    ) -> Result<Self> {
        let (self_files, child_fds) = {
            let (self_fds, child_fds) = sorted_fds.split_until(|fd| fd.package() == full_name);
            let self_files = self_fds
                .into_iter()
                .map(|fd| FileType::try_new(fd))
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

        let subpackages = child_fds
            .group_by_key(|fd| get_package_name(fd, full_name))
            .map(|subpackage_fds| -> Result<_> {
                let subpackage_name = get_package_name(subpackage_fds.first().unwrap(), full_name);
                let subpackage_full_name: Cow<str> = if full_name.is_empty() {
                    subpackage_name.into()
                } else {
                    format!("{}.{}", full_name, subpackage_name).into()
                };
                Ok((
                    subpackage_name.to_string(),
                    Package::try_make_package(
                        subpackage_name,
                        &subpackage_full_name,
                        subpackage_fds,
                        root.clone(),
                    )?,
                ))
            })
            .collect::<Result<HashMap<_, _>>>()?;

        Ok(Package {
            name: (!name.is_empty()).then(|| name.to_string()),
            subpackages,
            files: self_files,
            root,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::super::file::FileFake;
    use super::FileDescriptorProto;
    use ::once_cell::sync::Lazy;
    type Package = super::Package<FileFake>;

    static FD_ROOT: Lazy<FileDescriptorProto> = Lazy::new(|| {
        let mut fd = FileDescriptorProto::default();
        *fd.name_mut() = "root_file".to_string();
        fd
    });

    #[test]
    fn test_make_package_empty() {
        let files = [Lazy::force(&FD_ROOT)];
        let root_package = Package::new_from_files(files.into_iter());
        assert_eq!(None, root_package.name);
        assert_eq!(1, root_package.files.len());
        assert_eq!(Lazy::force(&FD_ROOT), &root_package.files[0].proto);
    }
}
