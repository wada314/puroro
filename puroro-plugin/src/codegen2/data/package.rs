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
use super::super::{Enum, InputFile, InputFileImpl, Message, PackageOrMessage};
use crate::Result;
use ::itertools::Itertools;
use ::once_cell::unsync::OnceCell;
use ::puroro_protobuf_compiled::google::protobuf::FileDescriptorProto;
use ::std::fmt::Debug;
use ::std::rc::{Rc, Weak};

pub trait Package: Debug + PackageOrMessage {
    fn cache(&self) -> &AnonymousCache;
    fn full_name(&self) -> Result<&str>;
    fn base(&self) -> Result<&PackageBase>;
    fn as_dyn_rc(self: Rc<Self>) -> Rc<dyn Package>;
}

#[derive(Debug)]
pub struct PackageBase {
    subpackages: Vec<Rc<NonRootPackage>>,
    files: Vec<Rc<dyn InputFile>>,
    root: Weak<RootPackage>,
    messages: OnceCell<Vec<Rc<dyn Message>>>,
    enums: OnceCell<Vec<Rc<dyn Enum>>>,
}

#[derive(Debug)]
pub struct NonRootPackage {
    cache1: AnonymousCache,
    cache2: AnonymousCache,
    name: String,
    full_name: OnceCell<String>,
    parent: Weak<dyn Package>,
    base: PackageBase,
}

#[derive(Debug)]
pub struct RootPackage {
    cache1: AnonymousCache,
    cache2: AnonymousCache,
    base: PackageBase,
}

impl PackageBase {
    fn new<'a, FP, FF, F, PNI>(
        root: Weak<RootPackage>,
        names_and_fds: impl Iterator<Item = (PNI, &'a FileDescriptorProto)>,
        fp: FP,
        ff: FF,
    ) -> Self
    where
        PNI: Iterator<Item = &'a str>,
        FP: Fn(&str, Vec<(PNI, &'a FileDescriptorProto)>) -> Rc<NonRootPackage>,
        FF: Fn(&FileDescriptorProto) -> Rc<F>,
        F: 'static + InputFile,
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
            .map(|(_, fd)| ff(fd) as Rc<dyn InputFile>)
            .collect();

        // The remaining FDs belong to the child packages.
        let subpackages = name_fd_map
            .into_iter()
            .filter_map(|(name_opt, child_files)| name_opt.map(|name| (name, child_files)))
            .map(|(name, child_files)| fp(&name, child_files))
            .collect();

        PackageBase {
            root,
            subpackages,
            files: self_files,
            messages: OnceCell::new(),
            enums: OnceCell::new(),
        }
    }

    fn messages(&self) -> Result<impl '_ + Iterator<Item = Rc<dyn Message>>> {
        self.messages
            .get_or_try_init(|| {
                self.files
                    .iter()
                    .map(|f| f.messages())
                    .flatten_ok()
                    .try_collect()
            })
            .map(|v| v.iter().cloned())
    }

    fn enums(&self) -> Result<impl '_ + Iterator<Item = Rc<dyn Enum>>> {
        self.enums
            .get_or_try_init(|| {
                self.files
                    .iter()
                    .map(|f| f.enums())
                    .flatten_ok()
                    .try_collect()
            })
            .map(|v| v.iter().cloned())
    }

    fn subpackages(&self) -> Result<impl '_ + Iterator<Item = Rc<NonRootPackage>>> {
        Ok(self.subpackages.iter().cloned())
    }

    fn root(&self) -> Result<Rc<RootPackage>> {
        self.root.try_upgrade()
    }
}

impl RootPackage {
    pub fn new<'a>(fds: impl Iterator<Item = &'a FileDescriptorProto>) -> Rc<RootPackage> {
        Self::new_with(fds, InputFileImpl::new)
    }

    pub fn new_with<'a, FF, F>(
        fds: impl Iterator<Item = &'a FileDescriptorProto>,
        ff: FF,
    ) -> Rc<RootPackage>
    where
        FF: Fn(&FileDescriptorProto, Weak<dyn Package>) -> Rc<F>,
        F: 'static + InputFile,
    {
        Rc::new_cyclic(|weak_root| {
            let names_and_fds = fds.map(|fd| {
                let package_name_iter = fd.package().split('.').filter(|s| !s.is_empty());
                (package_name_iter, fd)
            });
            let base = PackageBase::new(
                Weak::clone(weak_root),
                names_and_fds,
                |name, names_and_fds_vec| {
                    NonRootPackage::new_with(
                        name,
                        names_and_fds_vec.into_iter(),
                        Weak::clone(weak_root) as Weak<dyn Package>,
                        Weak::clone(weak_root),
                        &ff,
                    )
                },
                |fd| (ff)(fd, Weak::clone(weak_root) as Weak<dyn Package>),
            );
            Self {
                cache1: Default::default(),
                cache2: Default::default(),
                base,
            }
        })
    }
}

impl NonRootPackage {
    fn new_with<'a, PNI, FF, F>(
        name: &str,
        names_and_fds: impl Iterator<Item = (PNI, &'a FileDescriptorProto)>,
        parent: Weak<dyn Package>,
        root: Weak<RootPackage>,
        ff: &FF,
    ) -> Rc<NonRootPackage>
    where
        PNI: Iterator<Item = &'a str>,
        FF: Fn(&FileDescriptorProto, Weak<dyn Package>) -> Rc<F>,
        F: 'static + InputFile,
    {
        Rc::new_cyclic(|weak_self| {
            let base = PackageBase::new(
                Weak::clone(&root),
                names_and_fds,
                |name, names_and_fds_vec| {
                    NonRootPackage::new_with(
                        name,
                        names_and_fds_vec.into_iter(),
                        Weak::clone(weak_self) as Weak<dyn Package>,
                        Weak::clone(&root),
                        ff,
                    )
                },
                |fd| (ff)(fd, Weak::clone(weak_self) as Weak<dyn Package>),
            );
            Self {
                cache1: Default::default(),
                cache2: Default::default(),
                name: name.to_string(),
                full_name: OnceCell::new(),
                parent,
                base,
            }
        })
    }
}

impl PackageOrMessage for RootPackage {
    fn cache(&self) -> &AnonymousCache {
        &self.cache1
    }
    fn name(&self) -> Result<&str> {
        Ok("".into())
    }
    fn messages(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<dyn Message>>>> {
        Ok(Box::new(self.base()?.messages()?))
    }
    fn enums(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<dyn Enum>>>> {
        Ok(Box::new(self.base()?.enums()?))
    }
    fn subpackages(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<dyn Package>>>> {
        Ok(Box::new(
            self.base()?.subpackages()?.map(|p| p as Rc<dyn Package>),
        ))
    }
    fn root_package(&self) -> Result<Rc<RootPackage>> {
        self.base()?.root()
    }
    fn parent(&self) -> Result<Option<Rc<dyn PackageOrMessage>>> {
        Ok(None)
    }
}

impl Package for RootPackage {
    fn cache(&self) -> &AnonymousCache {
        &self.cache2
    }
    fn full_name(&self) -> Result<&str> {
        Ok("".into())
    }
    fn as_dyn_rc(self: Rc<Self>) -> Rc<dyn Package> {
        self
    }
    fn base(&self) -> Result<&PackageBase> {
        Ok(&self.base)
    }
}

impl PackageOrMessage for NonRootPackage {
    fn cache(&self) -> &AnonymousCache {
        &self.cache1
    }
    fn name(&self) -> Result<&str> {
        Ok(&self.name)
    }
    fn messages(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<dyn Message>>>> {
        Ok(Box::new(self.base()?.messages()?))
    }
    fn enums(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<dyn Enum>>>> {
        Ok(Box::new(self.base()?.enums()?))
    }
    fn subpackages(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<dyn Package>>>> {
        Ok(Box::new(
            self.base()?.subpackages()?.map(|p| p as Rc<dyn Package>),
        ))
    }
    fn root_package(&self) -> Result<Rc<RootPackage>> {
        self.base()?.root()
    }
    fn parent(&self) -> Result<Option<Rc<dyn PackageOrMessage>>> {
        Ok(Some(self.parent.try_upgrade()?))
    }
}

impl Package for NonRootPackage {
    fn cache(&self) -> &AnonymousCache {
        &self.cache2
    }
    fn full_name(&self) -> Result<&str> {
        self.full_name
            .get_or_try_init(|| {
                let parent = self.parent.try_upgrade()?;
                let parent_full_name = parent.full_name()?;
                if parent_full_name.is_empty() {
                    Ok(self.name.clone())
                } else {
                    Ok(format!("{}.{}", parent_full_name, &self.name))
                }
            })
            .map(|s| s.as_str())
    }
    fn as_dyn_rc(self: Rc<Self>) -> Rc<dyn Package> {
        self
    }
    fn base(&self) -> Result<&PackageBase> {
        Ok(&self.base)
    }
}

#[cfg(test)]
mod tests {
    use super::super::input_file::InputFileFake;
    use super::RootPackage;
    use crate::Result;
    use ::once_cell::sync::Lazy;
    use ::puroro_protobuf_compiled::google::protobuf::FileDescriptorProto;
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

    #[test]
    fn test_make_package_empty() -> Result<()> {
        let files = [Lazy::force(&FD_ROOT)];
        let root_package = RootPackage::new_with(files.into_iter(), InputFileFake::new);

        assert_eq!(1, root_package.base.files.len());
        assert_eq!(FD_ROOT.name(), root_package.base.files[0].name()?);
        assert_eq!(
            FD_ROOT.package(),
            root_package.base.files[0].package()?.full_name()?
        );
        Ok(())
    }

    #[test]
    fn test_make_package_single() -> Result<()> {
        let files = [Lazy::force(&FD_G_P_DESC)];
        let root_package = RootPackage::new_with(files.into_iter(), InputFileFake::new);

        assert_eq!(0, root_package.base.files.len());
        assert_eq!(1, root_package.base.subpackages_map.len());
        assert!(root_package.base.subpackages_map.contains_key("google"));

        let package_g = Rc::clone(&root_package.base.subpackages_map["google"]);
        assert_eq!("google", package_g.name);
        assert_eq!(0, package_g.base.files.len());
        assert_eq!(1, package_g.base.subpackages_map.len());
        assert!(package_g.base.subpackages_map.contains_key("protobuf"));

        let package_g_p = Rc::clone(&package_g.base.subpackages_map["protobuf"]);
        assert_eq!("protobuf", package_g_p.name);
        assert_eq!(1, package_g_p.base.files.len());
        assert_eq!(
            "google.protobuf",
            package_g_p.base.files[0].package()?.full_name()?
        );
        assert_eq!(0, package_g_p.base.subpackages_map.len());

        Ok(())
    }

    #[test]
    fn test_make_package_many() -> Result<()> {
        let files = [
            Lazy::force(&FD_G_P_DESC),
            Lazy::force(&FD_ROOT),
            Lazy::force(&FD_G_P_EMPTY),
            Lazy::force(&FD_G_P_C_PLUGIN),
        ];
        let root_package = RootPackage::new_with(files.into_iter(), InputFileFake::new);

        assert_eq!(1, root_package.base.files.len());
        assert_eq!("", root_package.base.files[0].package()?.full_name()?);
        assert_eq!(1, root_package.base.subpackages_map.len());
        assert!(root_package.base.subpackages_map.contains_key("google"));

        let package_g = Rc::clone(&root_package.base.subpackages_map["google"]);
        assert_eq!("google", package_g.name);
        assert_eq!(0, package_g.base.files.len());
        assert_eq!(1, package_g.base.subpackages_map.len());
        assert!(package_g.base.subpackages_map.contains_key("protobuf"));

        let package_g_p = Rc::clone(&package_g.base.subpackages_map["protobuf"]);
        assert_eq!("protobuf", package_g_p.name);
        assert_eq!(2, package_g_p.base.files.len());
        assert_eq!(
            "google.protobuf",
            package_g_p.base.files[0].package()?.full_name()?
        );
        assert_eq!(
            "google.protobuf",
            package_g_p.base.files[1].package()?.full_name()?
        );
        assert_eq!(1, package_g_p.base.subpackages_map.len());
        assert!(package_g_p.base.subpackages_map.contains_key("compiler"));

        let package_g_p_c = Rc::clone(&package_g_p.base.subpackages_map["compiler"]);
        assert_eq!("compiler", package_g_p_c.name);
        assert_eq!(1, package_g_p_c.base.files.len());
        assert_eq!(0, package_g_p_c.base.subpackages_map.len());

        Ok(())
    }
}
