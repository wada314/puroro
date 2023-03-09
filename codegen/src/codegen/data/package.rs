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

use super::super::util::*;
use super::{
    DataTypeBase, Enum, InputFile, Message, Oneof, PackageOrMessage, PackageOrMessageCase,
};
use crate::{FatalErrorKind, Result};
use ::itertools::Itertools;
use ::once_cell::unsync::OnceCell;
use ::puroro::protobuf::google::protobuf::FileDescriptorProtoView;
use ::std::fmt::Debug;
use ::std::iter;
use ::std::rc::{Rc, Weak};

#[derive(Debug)]
pub(crate) struct Package {
    cache: AnonymousCache,
    subpackages: Vec<Rc<Package>>,
    files: Vec<Rc<InputFile>>,
    root: Weak<Package>,
    messages: OnceCell<Vec<Rc<Message>>>,
    enums: OnceCell<Vec<Rc<Enum>>>,
    is_root: IsRoot,
}

#[derive(Debug)]
enum IsRoot {
    Root,
    NonRoot {
        name: String,
        full_name: OnceCell<String>,
        parent: Weak<Package>,
    },
}

impl Package {
    pub(crate) fn new_root<'a>(
        fds: impl Iterator<Item = &'a FileDescriptorProtoView>,
    ) -> Rc<Package> {
        Rc::new_cyclic(|weak_root| {
            let names_and_fds = fds.map(|fd| {
                let package_name_iter = fd.package().split('.').filter(|s| !s.is_empty());
                (package_name_iter, fd)
            });

            Self::new_shared(
                Weak::clone(weak_root),
                Weak::clone(weak_root),
                names_and_fds,
                IsRoot::Root,
            )
        })
    }

    fn new_non_root<'a, PNI>(
        name: &str,
        weak_root: Weak<Package>,
        parent: Weak<Package>,
        names_and_fds: impl Iterator<Item = (PNI, &'a FileDescriptorProtoView)>,
    ) -> Rc<Self>
    where
        PNI: Iterator<Item = &'a str>,
    {
        Rc::new_cyclic(|weak_self| {
            Self::new_shared(
                Weak::clone(weak_self),
                weak_root,
                names_and_fds,
                IsRoot::NonRoot {
                    name: name.to_string(),
                    full_name: OnceCell::new(),
                    parent,
                },
            )
        })
    }

    fn new_shared<'a, PNI>(
        weak_self: Weak<Package>,
        weak_root: Weak<Package>,
        names_and_fds: impl Iterator<Item = (PNI, &'a FileDescriptorProtoView)>,
        is_root: IsRoot,
    ) -> Package
    where
        PNI: Iterator<Item = &'a str>,
    {
        let name_fd_map = names_and_fds
            .map(|(mut name_iter, fd)| {
                let name = name_iter.next();
                (name, (name_iter, fd))
            })
            .into_group_map();

        // Some FDs might directly belong to the current package.
        let self_files = name_fd_map
            .get(&None)
            .into_iter()
            .flatten()
            .map(|(_, fd)| InputFile::new(fd, Weak::clone(&weak_self)))
            .collect();

        // The remaining FDs belong to the child packages.
        let subpackages = name_fd_map
            .into_iter()
            .filter_map(|(name_opt, child_files)| name_opt.map(|name| (name, child_files)))
            .map(|(name, child_files)| {
                Package::new_non_root(
                    name,
                    Weak::clone(&weak_root),
                    Weak::clone(&weak_self),
                    child_files.into_iter(),
                )
            })
            .collect();

        Package {
            cache: Default::default(),
            root: Weak::clone(&weak_root),
            subpackages,
            files: self_files,
            messages: OnceCell::new(),
            enums: OnceCell::new(),
            is_root,
        }
    }

    // Will no one use this???
    #[allow(unused)]
    fn files(&self) -> Result<impl Iterator<Item = &Rc<InputFile>>> {
        Ok(self.files.iter())
    }

    fn messages(&self) -> Result<impl Iterator<Item = &Rc<Message>>> {
        Ok(self
            .messages
            .get_or_try_init(|| {
                self.files
                    .iter()
                    .map(|f| f.messages())
                    .flatten_ok()
                    .map(|f| f.cloned())
                    .try_collect()
            })?
            .iter())
    }

    fn enums(&self) -> Result<impl Iterator<Item = &Rc<Enum>>> {
        Ok(self
            .enums
            .get_or_try_init(|| {
                self.files
                    .iter()
                    .map(|f| f.enums())
                    .flatten_ok()
                    .map(|o| o.cloned())
                    .try_collect()
            })?
            .iter())
    }

    fn subpackages(&self) -> Result<impl Iterator<Item = &Rc<Package>>> {
        Ok(self.subpackages.iter())
    }

    fn root(&self) -> Result<Rc<Package>> {
        self.root.try_upgrade()
    }

    // I think I will use this in somewhere document generating
    #[allow(unused)]
    pub(crate) fn full_name(&self) -> Result<&str> {
        if let IsRoot::NonRoot {
            name,
            full_name,
            parent,
        } = &self.is_root
        {
            full_name
                .get_or_try_init(|| {
                    let parent = parent.try_upgrade()?;
                    let parent_full_name = parent.full_name()?;
                    if parent_full_name.is_empty() {
                        Ok(name.clone())
                    } else {
                        Ok(format!("{}.{}", parent_full_name, &name))
                    }
                })
                .map(|s| s.as_str())
        } else {
            Ok("")
        }
    }
}

impl DataTypeBase for Package {
    fn cache(&self) -> &AnonymousCache {
        &self.cache
    }
    fn name(&self) -> Result<&str> {
        Ok(match &self.is_root {
            IsRoot::Root => Err(FatalErrorKind::InternalError {
                detail: "name requested for root package".to_string(),
            })?,
            IsRoot::NonRoot { name, .. } => name,
        })
    }
}

impl PackageOrMessage for Package {
    fn either(&self) -> PackageOrMessageCase<&Package, &Message> {
        PackageOrMessageCase::Package(self)
    }

    fn messages(&self) -> Result<Box<dyn '_ + Iterator<Item = &Rc<Message>>>> {
        Ok(Box::new(self.messages()?))
    }
    fn enums(&self) -> Result<Box<dyn '_ + Iterator<Item = &Rc<Enum>>>> {
        Ok(Box::new(self.enums()?))
    }
    fn oneofs(&self) -> Result<Box<dyn '_ + Iterator<Item = &Rc<Oneof>>>> {
        Ok(Box::new(iter::empty()))
    }
    fn subpackages(&self) -> Result<Box<dyn '_ + Iterator<Item = &Rc<Package>>>> {
        Ok(Box::new(self.subpackages()?))
    }
    fn root_package(&self) -> Result<Rc<Package>> {
        self.root()
    }
    fn parent(&self) -> Result<Option<Rc<dyn PackageOrMessage>>> {
        if let IsRoot::NonRoot { parent, .. } = &self.is_root {
            Ok(Some(parent.try_upgrade()?))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::{DataTypeBase, Package};
    use crate::Result;
    use ::once_cell::sync::Lazy;
    use ::puroro::protobuf::google::protobuf::FileDescriptorProto;
    use ::std::ops::Deref;
    use ::std::rc::Rc;

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

    trait IteratorExt<'a>: Sized + Iterator<Item = &'a Rc<Package>> {
        fn find_subp(&mut self, name: &str) -> Option<Rc<Package>> {
            self.find(|p| p.name().is_ok_and(|n| n == name)).cloned()
        }
    }
    impl<'a, T: Iterator<Item = &'a Rc<Package>>> IteratorExt<'a> for T {}

    #[test]
    fn test_make_package_empty() -> Result<()> {
        let files = [Lazy::force(&FD_ROOT).deref()];
        let root_package = Package::new_root(files.into_iter());
        let root_files = root_package.files()?.collect::<Vec<_>>();

        assert_eq!(1, root_files.len());
        assert_eq!(FD_ROOT.name(), root_files[0].name()?);
        assert_eq!(FD_ROOT.package(), root_files[0].package()?.full_name()?);
        Ok(())
    }

    #[test]
    fn test_make_package_single() -> Result<()> {
        let files = [Lazy::force(&FD_G_P_DESC).deref()];
        let root_package = Package::new_root(files.into_iter());
        let root_files = root_package.files()?.collect::<Vec<_>>();

        assert_eq!(0, root_files.len());
        assert_eq!(1, root_package.subpackages()?.count());

        let Some(g_package) = root_package.subpackages()?.find_subp("google") else {
            panic!()
        };
        assert_eq!(0, g_package.files()?.count());
        assert_eq!(1, g_package.subpackages()?.count());

        let Some(g_p_package) = g_package.subpackages()?.find_subp("protobuf") else {
            panic!()
        };
        let g_p_files = g_p_package.files()?.collect::<Vec<_>>();
        assert_eq!(1, g_p_files.len());
        assert_eq!("google.protobuf", g_p_files[0].package()?.full_name()?);
        assert_eq!(0, g_p_package.subpackages()?.count());

        Ok(())
    }

    #[test]
    fn test_make_package_many() -> Result<()> {
        let files = [
            Lazy::force(&FD_G_P_DESC).deref(),
            Lazy::force(&FD_ROOT).deref(),
            Lazy::force(&FD_G_P_EMPTY).deref(),
            Lazy::force(&FD_G_P_C_PLUGIN).deref(),
        ];
        let root_package = Package::new_root(files.into_iter());

        let root_files = root_package.files()?.collect::<Vec<_>>();
        assert_eq!(1, root_files.len());
        assert_eq!("", root_files[0].package()?.full_name()?);
        assert_eq!(1, root_package.subpackages()?.count());

        let Some(g_package) = root_package.subpackages()?.find_subp("google") else {
            panic!()
        };
        assert_eq!(0, g_package.files()?.count());
        assert_eq!(1, g_package.subpackages()?.count());

        let Some(g_p_package) = g_package.subpackages()?.find_subp("protobuf") else {
            panic!()
        };
        let g_p_files = g_p_package.files()?.collect::<Vec<_>>();
        assert_eq!(2, g_p_files.len());
        assert_eq!("google.protobuf", g_p_files[0].package()?.full_name()?);
        assert_eq!("google.protobuf", g_p_files[1].package()?.full_name()?);
        assert_eq!(1, g_p_package.subpackages()?.count());

        let Some(g_p_c_package) = g_p_package.subpackages()?.find_subp("compiler") else {
            panic!()
        };
        assert_eq!(1, g_p_c_package.files()?.count());
        assert_eq!(0, g_p_c_package.subpackages()?.count());

        Ok(())
    }
}
