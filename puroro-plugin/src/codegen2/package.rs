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
pub struct RootPackage<FileType> {
    subpackages: HashMap<String, NonRootPackage<FileType>>,
    files: Vec<FileType>,
}

#[derive(Debug)]
pub struct NonRootPackage<FileType> {
    name: String,
    subpackages: HashMap<String, NonRootPackage<FileType>>,
    files: Vec<FileType>,
    root: Weak<RootPackage<FileType>>,
}

impl<FileType: FileTrait> RootPackage<FileType> {
    pub fn try_new_from_files<'a, I: Iterator<Item = &'a FileDescriptorProto>>(
        iter: I,
    ) -> Result<Rc<Self>> {
        let mut files = iter.collect::<Vec<_>>();
        files.sort_by_key(|f| f.package());
        let mut maybe_error = None;
        let maybe_root_package = Rc::new_cyclic(|weak_root| {
            match RootPackage::try_make_package(&files, weak_root.clone()) {
                Ok(root_package) => root_package,
                Err(error) => {
                    maybe_error = Some(error);
                    // Fortunately, a dummy `RootPackage` can be safely constructible.
                    RootPackage {
                        subpackages: HashMap::new(),
                        files: Vec::new(),
                    }
                }
            }
        });
        match maybe_error {
            Some(error) => Err(error)?,
            None => Ok(maybe_root_package),
        }
    }

    fn try_make_package(
        sorted_fds: &[&FileDescriptorProto],
        root: Weak<RootPackage<FileType>>,
    ) -> Result<Self> {
        fn get_package_name<'a>(fd: &'a FileDescriptorProto) -> &'a str {
            match fd.package().split_once('.') {
                Some((name, _)) => name,
                None => fd.package(),
            }
        }

        // The first few FDs might be FDs which are directly belonging to the current package.
        // The remaining FDs are belonging to the child packages.
        let (self_fds, child_fds) = sorted_fds.split_until(|fd| fd.package().is_empty());

        let self_files = self_fds
            .into_iter()
            .map(|fd| FileType::try_new(fd))
            .collect::<Result<Vec<_>>>()?;

        let subpackages = child_fds
            .group_by_key(|fd| get_package_name(fd))
            .map(|subpackage_fds| -> Result<_> {
                let subpackage_name = get_package_name(subpackage_fds.first().unwrap());
                Ok((
                    subpackage_name.to_string(),
                    NonRootPackage::try_make_package(
                        subpackage_name,
                        subpackage_name,
                        subpackage_fds,
                        root.clone(),
                    )?,
                ))
            })
            .collect::<Result<HashMap<_, _>>>()?;

        Ok(RootPackage {
            subpackages,
            files: self_files,
        })
    }
}

impl<FileType: FileTrait> NonRootPackage<FileType> {
    fn try_make_package(
        name: &str,
        full_name: &str,
        sorted_fds: &[&FileDescriptorProto],
        root: Weak<RootPackage<FileType>>,
    ) -> Result<Self> {
        fn get_package_name<'a>(fd: &'a FileDescriptorProto, full_name: &str) -> &'a str {
            let striped_path = &fd.package()[full_name.len() + 1..];
            match striped_path.split_once('.') {
                Some((name, _)) => name,
                None => striped_path,
            }
        }

        // The first few FDs might be FDs which are directly belonging to the current package.
        // The remaining FDs are belonging to the child packages.
        let (self_fds, child_fds) = sorted_fds.split_until(|fd| fd.package() == full_name);

        let self_files = self_fds
            .into_iter()
            .map(|fd| FileType::try_new(fd))
            .collect::<Result<Vec<_>>>()?;

        let subpackages = child_fds
            .group_by_key(|fd| get_package_name(fd, full_name))
            .map(|subpackage_fds| -> Result<_> {
                let subpackage_name = get_package_name(subpackage_fds.first().unwrap(), full_name);
                let subpackage_full_name = format!("{}.{}", full_name, subpackage_name);
                Ok((
                    subpackage_name.to_string(),
                    NonRootPackage::try_make_package(
                        subpackage_name,
                        &subpackage_full_name,
                        subpackage_fds,
                        root.clone(),
                    )?,
                ))
            })
            .collect::<Result<HashMap<_, _>>>()?;

        Ok(NonRootPackage {
            name: name.to_string(),
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
    use ::std::ops::Deref;
    type RootPackage = super::RootPackage<FileFake>;

    static FD_ROOT: Lazy<FileDescriptorProto> = Lazy::new(|| {
        let mut fd = FileDescriptorProto::default();
        *fd.name_mut() = "root_file".to_string();
        fd
    });

    static FD_G_P_DESC: Lazy<FileDescriptorProto> = Lazy::new(|| {
        let mut fd = FileDescriptorProto::default();
        *fd.name_mut() = "descriptor.proto".to_string();
        *fd.package_mut() = "google.protobuf".to_string();
        fd
    });

    static FD_G_P_EMPTY: Lazy<FileDescriptorProto> = Lazy::new(|| {
        let mut fd = FileDescriptorProto::default();
        *fd.name_mut() = "empty.proto".to_string();
        *fd.package_mut() = "google.protobuf".to_string();
        fd
    });

    static FD_G_P_C_PLUGIN: Lazy<FileDescriptorProto> = Lazy::new(|| {
        let mut fd = FileDescriptorProto::default();
        *fd.name_mut() = "plugin.proto".to_string();
        *fd.package_mut() = "google.protobuf.compiler".to_string();
        fd
    });

    #[test]
    fn test_make_package_empty() {
        let files = [Lazy::force(&FD_ROOT)];
        let root_package = RootPackage::try_new_from_files(files.into_iter()).unwrap();
        assert_eq!(1, root_package.files.len());
        assert_eq!(Lazy::force(&FD_ROOT), &root_package.files[0].proto);
    }

    #[test]
    fn test_make_package_single() {
        let files = [Lazy::force(&FD_G_P_DESC)];
        let root_package = RootPackage::try_new_from_files(files.into_iter()).unwrap();
        assert_eq!(0, root_package.files.len());
        assert_eq!(1, root_package.subpackages.len());
        assert!(root_package.subpackages.contains_key("google"));

        let package_g = &root_package.subpackages["google"];
        assert_eq!("google", package_g.name);
        assert_eq!(0, package_g.files.len());
        assert_eq!(1, package_g.subpackages.len());
        assert!(package_g.subpackages.contains_key("protobuf"));

        let package_g_p = &package_g.subpackages["protobuf"];
        assert_eq!("protobuf", package_g_p.name);
        assert_eq!(1, package_g_p.files.len());
        assert_eq!(Lazy::force(&FD_G_P_DESC), &package_g_p.files[0].proto);
        assert_eq!(0, package_g_p.subpackages.len());
    }

    #[test]
    fn test_make_package_many() {
        let files = [
            Lazy::force(&FD_G_P_DESC),
            Lazy::force(&FD_ROOT),
            Lazy::force(&FD_G_P_EMPTY),
            Lazy::force(&FD_G_P_C_PLUGIN),
        ];

        let root_package = RootPackage::try_new_from_files(files.into_iter()).unwrap();
        assert_eq!(1, root_package.files.len());
        assert_eq!(Lazy::force(&FD_ROOT), &root_package.files[0].proto);
        assert_eq!(1, root_package.subpackages.len());
        assert!(root_package.subpackages.contains_key("google"));

        let package_g = &root_package.subpackages["google"];
        assert_eq!("google", package_g.name);
        assert_eq!(0, package_g.files.len());
        assert_eq!(1, package_g.subpackages.len());
        assert!(package_g.subpackages.contains_key("protobuf"));

        let package_g_p = &package_g.subpackages["protobuf"];
        assert_eq!("protobuf", package_g_p.name);
        assert_eq!(2, package_g_p.files.len());
        assert_eq!(1, package_g_p.subpackages.len());
        assert!(package_g_p.subpackages.contains_key("compiler"));

        let package_g_p_c = &package_g_p.subpackages["compiler"];
        assert_eq!("compiler", package_g_p_c.name);
        assert_eq!(1, package_g_p_c.files.len());
        assert_eq!(Lazy::force(&FD_G_P_C_PLUGIN), &package_g_p_c.files[0].proto);
        assert_eq!(0, package_g_p_c.subpackages.len());
    }
}
