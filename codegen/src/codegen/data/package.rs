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
    DataTypeBase, Enum, InputFile, InputFileImpl, Message, Oneof, PackageOrMessage,
    PackageOrMessageCase,
};
use crate::Result;
use ::itertools::Itertools;
use ::once_cell::unsync::OnceCell;
use ::puroro_protobuf_compiled::google::protobuf::FileDescriptorProto;
use ::std::fmt::Debug;
use ::std::iter;
use ::std::rc::{Rc, Weak};

pub trait Package: PackageOrMessage + DataTypeBase + Debug {
    fn full_name(&self) -> Result<&str>;
    fn files(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<dyn InputFile>>>>;
}

#[derive(Debug)]
pub struct PackageBase {
    subpackages: Vec<Rc<NonRootPackage>>,
    files: Vec<Rc<dyn InputFile>>,
    root: Weak<RootPackage>,
    messages: OnceCell<Vec<Rc<Message>>>,
    enums: OnceCell<Vec<Rc<Enum>>>,
}

#[derive(Debug)]
pub struct NonRootPackage {
    cache: AnonymousCache,
    name: String,
    full_name: OnceCell<String>,
    parent: Weak<dyn Package>,
    base: PackageBase,
}

#[derive(Debug)]
pub struct RootPackage {
    cache: AnonymousCache,
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

    fn files(&self) -> Result<impl '_ + Iterator<Item = Rc<dyn InputFile>>> {
        Ok(self.files.iter().cloned())
    }

    fn messages(&self) -> Result<impl '_ + Iterator<Item = Rc<Message>>> {
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

    fn enums(&self) -> Result<impl '_ + Iterator<Item = Rc<Enum>>> {
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
                cache: Default::default(),
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
                cache: Default::default(),
                name: name.to_string(),
                full_name: OnceCell::new(),
                parent,
                base,
            }
        })
    }
}

impl DataTypeBase for RootPackage {
    fn cache(&self) -> &AnonymousCache {
        &self.cache
    }
    fn name(&self) -> Result<&str> {
        Ok("".into())
    }
}

impl PackageOrMessage for RootPackage {
    fn either(&self) -> PackageOrMessageCase<&dyn Package, &Message> {
        PackageOrMessageCase::Package(self)
    }

    fn messages(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<Message>>>> {
        Ok(Box::new(self.base.messages()?))
    }
    fn enums(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<Enum>>>> {
        Ok(Box::new(self.base.enums()?))
    }
    fn oneofs(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<Oneof>>>> {
        Ok(Box::new(iter::empty()))
    }
    fn subpackages(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<dyn Package>>>> {
        Ok(Box::new(
            self.base.subpackages()?.map(|p| p as Rc<dyn Package>),
        ))
    }
    fn root_package(&self) -> Result<Rc<RootPackage>> {
        self.base.root()
    }
    fn parent(&self) -> Result<Option<Rc<dyn PackageOrMessage>>> {
        Ok(None)
    }
}

impl Package for RootPackage {
    fn full_name(&self) -> Result<&str> {
        Ok("".into())
    }
    fn files(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<dyn InputFile>>>> {
        Ok(Box::new(self.base.files()?))
    }
}

impl DataTypeBase for NonRootPackage {
    fn cache(&self) -> &AnonymousCache {
        &self.cache
    }
    fn name(&self) -> Result<&str> {
        Ok(&self.name)
    }
}

impl PackageOrMessage for NonRootPackage {
    fn either(&self) -> PackageOrMessageCase<&dyn Package, &Message> {
        PackageOrMessageCase::Package(self)
    }

    fn messages(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<Message>>>> {
        Ok(Box::new(self.base.messages()?))
    }
    fn enums(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<Enum>>>> {
        Ok(Box::new(self.base.enums()?))
    }
    fn oneofs(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<Oneof>>>> {
        Ok(Box::new(iter::empty()))
    }
    fn subpackages(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<dyn Package>>>> {
        Ok(Box::new(
            self.base.subpackages()?.map(|p| p as Rc<dyn Package>),
        ))
    }
    fn root_package(&self) -> Result<Rc<RootPackage>> {
        self.base.root()
    }
    fn parent(&self) -> Result<Option<Rc<dyn PackageOrMessage>>> {
        Ok(Some(self.parent.try_upgrade()?))
    }
}

impl Package for NonRootPackage {
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
    fn files(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<dyn InputFile>>>> {
        Ok(Box::new(self.base.files()?))
    }
}

#[cfg(test)]
mod tests {
    use super::super::{InputFileFake, Package, PackageOrMessage, RootPackage};
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

    trait IteratorExt: Sized + Iterator<Item = Rc<dyn Package>> {
        fn find_subp(&mut self, name: &str) -> Option<Rc<dyn Package>> {
            self.find(|p| p.name().is_ok_and(|n| n == name))
        }
    }
    impl<T: Iterator<Item = Rc<dyn Package>>> IteratorExt for T {}

    #[test]
    fn test_make_package_empty() -> Result<()> {
        let files = [Lazy::force(&FD_ROOT)];
        let root_package = RootPackage::new_with(files.into_iter(), InputFileFake::new);
        let root_files = root_package.files()?.collect::<Vec<_>>();

        assert_eq!(1, root_files.len());
        assert_eq!(FD_ROOT.name(), root_files[0].name()?);
        assert_eq!(FD_ROOT.package(), root_files[0].package()?.full_name()?);
        Ok(())
    }

    #[test]
    fn test_make_package_single() -> Result<()> {
        let files = [Lazy::force(&FD_G_P_DESC)];
        let root_package = RootPackage::new_with(files.into_iter(), InputFileFake::new);
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
            Lazy::force(&FD_G_P_DESC),
            Lazy::force(&FD_ROOT),
            Lazy::force(&FD_G_P_EMPTY),
            Lazy::force(&FD_G_P_C_PLUGIN),
        ];
        let root_package = RootPackage::new_with(files.into_iter(), InputFileFake::new);

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
