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
use crate::codegen::utils::StrExt;
use crate::Result;
use ::itertools::Itertools;
use ::once_cell::unsync::OnceCell;
use ::proc_macro2::TokenStream;
use ::puroro_protobuf_compiled::google::protobuf::FileDescriptorProto;
use ::quote::{format_ident, quote};
use ::std::borrow::Cow;
use ::std::collections::HashMap;
use ::std::fmt::Debug;
use ::std::iter;
use ::std::rc::{Rc, Weak};

pub(super) trait PackageTrait: Debug + PackageOrMessageTrait {
    fn full_name(&self) -> Result<Cow<'_, str>>;
    fn name(&self) -> Result<Cow<'_, str>>;
    fn base(&self) -> Result<&PackageBase>;
    fn as_dyn_rc(self: Rc<Self>) -> Rc<dyn PackageTrait>;
}

#[derive(Debug)]
pub(super) struct PackageBase {
    subpackages_map: HashMap<String, Rc<NonRootPackage>>,
    subpackages: OnceCell<Vec<Rc<dyn PackageTrait>>>,
    files: Vec<Rc<dyn InputFileTrait>>,
    root: Weak<RootPackage>,
    messages: OnceCell<Vec<Rc<dyn MessageTrait>>>,
    enums: OnceCell<Vec<Rc<dyn EnumTrait>>>,
}

#[derive(Debug)]
pub(super) struct NonRootPackage {
    name: String,
    parent: Weak<dyn PackageTrait>,
    base: PackageBase,
    module_file_dir: OnceCell<String>,
    rust_module_path: OnceCell<Rc<TokenStream>>,
}

#[derive(Debug)]
pub(super) struct RootPackage {
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
        F: 'static + InputFileTrait,
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
            .map(|(_, fd)| ff(fd) as Rc<dyn InputFileTrait>)
            .collect();

        // The remaining FDs belong to the child packages.
        let subpackages = name_fd_map
            .into_iter()
            .filter_map(|(name_opt, child_files)| name_opt.map(|name| (name, child_files)))
            .map(|(name, child_files)| {
                let subpackage = fp(&name, child_files);
                (name.to_string(), subpackage)
            })
            .collect();

        PackageBase {
            root,
            subpackages_map: subpackages,
            subpackages: OnceCell::new(),
            files: self_files,
            messages: OnceCell::new(),
            enums: OnceCell::new(),
        }
    }

    fn messages(&self) -> Result<&[Rc<dyn MessageTrait>]> {
        self.messages
            .get_or_try_init(|| {
                self.files
                    .iter()
                    .map(|f| Ok(f.messages()?.iter().cloned()))
                    .flatten_ok()
                    .try_collect()
            })
            .map(|v| v.as_slice())
    }

    fn enums(&self) -> Result<&[Rc<dyn EnumTrait>]> {
        self.enums
            .get_or_try_init(|| {
                self.files
                    .iter()
                    .map(|f| Ok(f.enums()?.iter().cloned()))
                    .flatten_ok()
                    .try_collect()
            })
            .map(|v| v.as_slice())
    }

    fn subpackages(&self) -> Result<&[Rc<dyn PackageTrait>]> {
        self.subpackages
            .get_or_try_init(|| {
                Ok(self
                    .subpackages_map
                    .values()
                    .cloned()
                    .map(|p| p as Rc<dyn PackageTrait>)
                    .collect())
            })
            .map(|sp| sp.as_slice())
    }

    fn root(&self) -> Result<Rc<RootPackage>> {
        self.root.try_upgrade()
    }
}

impl RootPackage {
    pub(super) fn new<'a>(fds: impl Iterator<Item = &'a FileDescriptorProto>) -> Rc<RootPackage> {
        Self::new_with(fds, InputFile::new)
    }

    pub(super) fn new_with<'a, FF, F>(
        fds: impl Iterator<Item = &'a FileDescriptorProto>,
        ff: FF,
    ) -> Rc<RootPackage>
    where
        FF: Fn(&FileDescriptorProto, Weak<dyn PackageTrait>) -> Rc<F>,
        F: 'static + InputFileTrait,
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
                        Weak::clone(weak_root) as Weak<dyn PackageTrait>,
                        Weak::clone(weak_root),
                        &ff,
                    )
                },
                |fd| (ff)(fd, Weak::clone(weak_root) as Weak<dyn PackageTrait>),
            );
            Self { base }
        })
    }

    pub(super) fn all_packages(self: &Rc<Self>) -> Vec<Rc<dyn PackageTrait>> {
        let mut ret = vec![Rc::clone(self) as Rc<dyn PackageTrait>];
        let mut stack = self.base.subpackages_map.values().cloned().collect_vec();
        while let Some(p) = stack.pop() {
            stack.extend(p.base.subpackages_map.values().cloned());
            ret.push(p as Rc<dyn PackageTrait>);
        }
        ret
    }
}

impl NonRootPackage {
    fn new_with<'a, PNI, FF, F>(
        name: &str,
        names_and_fds: impl Iterator<Item = (PNI, &'a FileDescriptorProto)>,
        parent: Weak<dyn PackageTrait>,
        root: Weak<RootPackage>,
        ff: &FF,
    ) -> Rc<NonRootPackage>
    where
        PNI: Iterator<Item = &'a str>,
        FF: Fn(&FileDescriptorProto, Weak<dyn PackageTrait>) -> Rc<F>,
        F: 'static + InputFileTrait,
    {
        Rc::new_cyclic(|weak_self| {
            let base = PackageBase::new(
                Weak::clone(&root),
                names_and_fds,
                |name, names_and_fds_vec| {
                    NonRootPackage::new_with(
                        name,
                        names_and_fds_vec.into_iter(),
                        Weak::clone(weak_self) as Weak<dyn PackageTrait>,
                        Weak::clone(&root),
                        ff,
                    )
                },
                |fd| (ff)(fd, Weak::clone(weak_self) as Weak<dyn PackageTrait>),
            );
            Self {
                name: name.to_string(),
                parent,
                base,
                module_file_dir: OnceCell::new(),
                rust_module_path: OnceCell::new(),
            }
        })
    }
}

impl PackageOrMessageTrait for RootPackage {
    fn messages(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<dyn MessageTrait>>>> {
        Ok(Box::new(self.base()?.messages()?.iter().cloned()))
    }
    fn enums(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<dyn EnumTrait>>>> {
        Ok(Box::new(self.base()?.enums()?.iter().cloned()))
    }
    fn subpackages(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<dyn PackageTrait>>>> {
        Ok(Box::new(self.base()?.subpackages()?.iter().cloned()))
    }
    fn root_package(&self) -> Result<Rc<RootPackage>> {
        self.base()?.root()
    }
    fn parent(&self) -> Result<Option<Rc<dyn PackageOrMessageTrait>>> {
        Ok(None)
    }
    fn module_file_path(&self) -> Result<Cow<'_, str>> {
        Ok("lib.rs".into())
    }
    fn module_file_dir(&self) -> Result<Cow<'_, str>> {
        Ok("".into())
    }
    fn gen_rust_module_path(&self) -> Result<Rc<TokenStream>> {
        Ok(Rc::new(quote! { self :: _puroro_root }))
    }
}

impl PackageTrait for RootPackage {
    fn name(&self) -> Result<Cow<'_, str>> {
        Ok("".into())
    }
    fn full_name(&self) -> Result<Cow<'_, str>> {
        Ok("".into())
    }
    fn as_dyn_rc(self: Rc<Self>) -> Rc<dyn PackageTrait> {
        self
    }
    fn base(&self) -> Result<&PackageBase> {
        Ok(&self.base)
    }
}

impl PackageOrMessageTrait for NonRootPackage {
    fn messages(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<dyn MessageTrait>>>> {
        Ok(Box::new(self.base()?.messages()?.iter().cloned()))
    }
    fn enums(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<dyn EnumTrait>>>> {
        Ok(Box::new(self.base()?.enums()?.iter().cloned()))
    }
    fn subpackages(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<dyn PackageTrait>>>> {
        Ok(Box::new(self.base()?.subpackages()?.iter().cloned()))
    }
    fn root_package(&self) -> Result<Rc<RootPackage>> {
        self.base()?.root()
    }
    fn parent(&self) -> Result<Option<Rc<dyn PackageOrMessageTrait>>> {
        Ok(Some(self.parent.try_upgrade()?))
    }
    fn module_file_path(&self) -> Result<Cow<'_, str>> {
        Ok(format!(
            "{}{}.rs",
            self.parent.try_upgrade()?.module_file_dir()?,
            self.name.to_lower_snake_case()
        )
        .into())
    }
    fn module_file_dir(&self) -> Result<Cow<'_, str>> {
        self.module_file_dir
            .get_or_try_init(|| {
                Ok(format!(
                    "{}{}/",
                    self.parent.try_upgrade()?.module_file_dir()?,
                    self.name.to_lower_snake_case()
                ))
            })
            .map(|s| s.into())
    }
    fn gen_rust_module_path(&self) -> Result<Rc<TokenStream>> {
        self.rust_module_path
            .get_or_try_init(|| {
                let parent = self.parent.try_upgrade()?.gen_rust_module_path()?;
                let ident = format_ident!(
                    "{}",
                    self.name()?.to_lower_snake_case().escape_rust_keywords()
                );
                Ok(Rc::new(quote! {
                    #parent :: #ident
                }))
            })
            .cloned()
    }
}

impl PackageTrait for NonRootPackage {
    fn name(&self) -> Result<Cow<'_, str>> {
        Ok(self.name.as_str().into())
    }
    fn full_name(&self) -> Result<Cow<'_, str>> {
        let parent = self.parent.try_upgrade()?;
        let parent_full_name = parent.full_name()?;
        if parent_full_name.is_empty() {
            Ok(self.name.as_str().into())
        } else {
            Ok(format!("{}.{}", parent_full_name, &self.name).into())
        }
    }
    fn as_dyn_rc(self: Rc<Self>) -> Rc<dyn PackageTrait> {
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
