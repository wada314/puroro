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
use ::std::rc::{Rc, Weak};

pub(super) trait PackageTrait: Debug {
    // fn messages(&self) -> Box<dyn '_ + Iterator<Item = Weak<dyn MessageTrait>>>;
    // fn enums(&self) -> Box<dyn '_ + Iterator<Item = Weak<dyn EnumTrait>>>;

    // fn subpackages(&self) -> Box<dyn '_ + Iterator<Item = &dyn PackageTrait>>;
    // fn subpackage(&self, name: &str) -> Option<&dyn PackageTrait>;
    fn module_file_path(&self) -> Result<Cow<'_, str>>;
    fn module_file_dir(&self) -> Result<Cow<'_, str>>;
    fn gen_module_file(&self) -> Result<TokenStream>;

    // fn resolve_type_name(
    //     &self,
    //     type_name: &str,
    // ) -> Result<MessageOrEnum<Weak<Box<dyn MessageTrait>>, Weak<dyn EnumTrait>>>;
}

#[derive(Debug)]
pub(super) struct PackageBase {
    subpackages: HashMap<String, Rc<dyn PackageTrait>>,
    files: Vec<Rc<dyn InputFileTrait>>,
}

#[derive(Debug)]
pub(super) struct NonRootPackage {
    name: String,
    parent: Weak<dyn PackageTrait>,
    root: Weak<dyn PackageTrait>,
    base: PackageBase,
    module_file_dir: OnceCell<String>,
}

#[derive(Debug)]
pub(super) struct RootPackage {
    base: PackageBase,
}

impl PackageBase {
    fn new<'a, FP, FF, PNI>(
        names_and_fds: impl Iterator<Item = (PNI, &'a FileDescriptorProto)>,
        mut fp: FP,
        mut ff: FF,
    ) -> Self
    where
        PNI: Iterator<Item = &'a str>,
        FP: FnMut(&str, Vec<(PNI, &FileDescriptorProto)>) -> Rc<dyn PackageTrait>,
        FF: FnMut(&FileDescriptorProto) -> Rc<dyn InputFileTrait>,
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
            .map(|(_, fd)| ff(fd))
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
            subpackages,
            files: self_files,
        }
    }

    fn gen_submodule_decls(&self) -> Result<Vec<TokenStream>> {
        Ok(self
            .subpackages
            .keys()
            .sorted()
            .map(|name| {
                let ident = format_ident!("{}", name.to_lower_snake_case().escape_rust_keywords());
                quote! {
                    pub mod ident;
                }
            })
            .collect())
    }

    fn gen_struct_decls(&self) -> Result<Vec<TokenStream>> {
        Ok(self
            .files
            .iter()
            .map(|f| f.gen_structs_for_messages())
            .try_collect()?)
    }
}

impl RootPackage {
    pub(super) fn new<'a>(fds: impl Iterator<Item = &'a FileDescriptorProto>) -> Rc<RootPackage> {
        Self::new_with(fds, |fd, weak_package| InputFile::new(fd))
    }

    pub(super) fn new_with<'a, FF>(
        fds: impl Iterator<Item = &'a FileDescriptorProto>,
        ff: FF,
    ) -> Rc<RootPackage>
    where
        FF: FnMut(&FileDescriptorProto, Weak<dyn PackageTrait>) -> Rc<dyn InputFileTrait>,
    {
        Rc::new_cyclic(|weak_root| {
            let names_and_fds = fds.map(|fd| {
                let package_name_iter = fd.package().split('.').filter(|s| !s.is_empty());
                (package_name_iter, fd)
            });
            let base = PackageBase::new(
                names_and_fds,
                |name, names_and_fds_vec| {
                    NonRootPackage::new_with(
                        name,
                        names_and_fds_vec.into_iter(),
                        Weak::clone(weak_root) as Weak<dyn PackageTrait>,
                        Weak::clone(weak_root) as Weak<dyn PackageTrait>,
                        &mut ff,
                    )
                },
                |fd| (ff)(fd, Weak::clone(weak_root) as Weak<dyn PackageTrait>),
            );
            Self { base }
        })
    }
}

impl NonRootPackage {
    fn new_with<'a, PNI, FF>(
        name: &str,
        names_and_fds: impl Iterator<Item = (PNI, &'a FileDescriptorProto)>,
        parent: Weak<dyn PackageTrait>,
        root: Weak<dyn PackageTrait>,
        ff: FF,
    ) -> Rc<NonRootPackage>
    where
        PNI: Iterator<Item = &'a str>,
        FF: FnMut(&FileDescriptorProto, Weak<dyn PackageTrait>) -> Rc<dyn InputFileTrait>,
    {
        Rc::new_cyclic(|weak_self| {
            let base = PackageBase::new(
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
                root,
                base,
                module_file_dir: OnceCell::new(),
            }
        })
    }
}

impl PackageTrait for RootPackage {
    fn module_file_path(&self) -> Result<Cow<'_, str>> {
        Ok("lib.rs".into())
    }
    fn module_file_dir(&self) -> Result<Cow<'_, str>> {
        Ok("".into())
    }
    fn gen_module_file(&self) -> Result<TokenStream> {
        let submodule_decls = self.base.gen_submodule_decls()?;
        let struct_decls = self.base.gen_struct_decls()?;
        Ok(quote! {
            //! "Generated from root package"

            /// re-export puroro.
            pub use ::puroro;
            /// re-export the primitive types in puroro namespace.
            /// by using the "*", it can be hidden by the same typename explicitly defined in this file.
            pub use ::puroro::*;
            pub mod _puroro_root {
                pub use super::*;
            }
            pub mod _puroro {
                pub use ::puroro::*;
            }

            #(#submodule_decls)*
            #(#struct_decls)*
        })
    }
}

impl PackageTrait for NonRootPackage {
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
    fn gen_module_file(&self) -> Result<TokenStream> {
        let submodule_decls = self.base.gen_submodule_decls()?;
        let struct_decls = self.base.gen_struct_decls()?;
        Ok(quote! {
            pub mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }
            pub mod _puroro {
                pub use ::puroro::*;
            }
            #(#submodule_decls)*
            #(#struct_decls)*
        })
    }
}

#[derive(Debug)]
pub(super) struct Package {
    name: String,
    full_name: String,
    subpackages: HashMap<String, Rc<dyn PackageTrait>>,
    files: Vec<Rc<dyn InputFileTrait>>,
    root: Weak<dyn PackageTrait>,
}

impl Package {
    // fn resolve_type_name(
    //     &self,
    //     type_name: &str,
    // ) -> Result<MessageOrEnum<Weak<Box<dyn MessageTrait>>, Weak<dyn EnumTrait>>> {
    //     // Case 1, the given type name is an absolute path.
    //     // If the type_name starts with '.', then redirect it to the root package.
    //     if let Some(abs_type_name) = type_name.strip_prefix('.') {
    //         return self.root.try_upgrade()?.resolve_type_name(abs_type_name);
    //     }

    //     let rest = type_name;
    //     let mut cur = MessageOrPackage::<&dyn MessageTrait, &dyn PackageTrait>::Package(self);
    //     while let Some((subcomponent_name, rest)) = rest.split_once('.') {
    //         // There are a sub-sub-component. Thus, the subcomponent is either a package or a message.
    //         if let MessageOrPackage::Package(p) = cur {
    //             if let Some(subsub) = p.subpackage(subcomponent_name) {}
    //         }
    //     }

    //     // Case 2, the next component of the path is a subpackage.
    //     // Extract the next component of the path. It must be non-empty, and must have another child.
    //     if let Some((subpackage_name, rest)) = type_name.split_once('.') {
    //         if let Some(subpackage) = self.subpackages.get(subpackage_name) {
    //             // Can dig into a subpackage. Go ahead.
    //             return subpackage.resolve_type_name(rest);
    //         } else {
    //             // When the target item is a nested message or enum in a message, then this can happen.
    //             // Do nothing, go to the next section.
    //         }
    //     }

    //     let (subitem_name, rest) = type_name.split_once('.').unwrap_or((type_name, ""));

    //     // Case 3, the next component of the path is a message.
    //     // The message can still have a submessage, so go deeper.
    //     if let Some(message) = self
    //         .messages()
    //         .try_find(|m| -> Result<_> { Ok(m.try_upgrade()?.name() == subitem_name) })?
    //     {
    //         return todo!();
    //     }

    //     // Case 4, the next component is an enum.
    //     // Enum cannot have a subitem so just return the found enum immediately.
    //     if let Some(enume) = self
    //         .enums()
    //         .try_find(|e| -> Result<_> { Ok(e.try_upgrade()?.name() == subitem_name) })?
    //     {
    //         // In this case, there should not be the remaining path component.
    //         if !rest.is_empty() {
    //             Err(ErrorKind::UnknownTypeName {
    //                 name: type_name.to_string(),
    //             })?;
    //         }

    //         return Ok(MessageOrEnum::Enum(enume));
    //     }

    //     // Case 5, Type not found case.
    //     Err(ErrorKind::UnknownTypeName {
    //         name: type_name.to_string(),
    //     })?
    // }
}

#[cfg(test)]
mod tests {
    use super::super::input_file::InputFileTrait;
    use super::super::Syntax;
    use super::FileDescriptorProto;
    use super::Package;
    use crate::Result;
    use ::once_cell::sync::Lazy;
    use ::proc_macro2::TokenStream;
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

    #[derive(Debug)]
    struct InputFileFake {
        pub proto: Rc<FileDescriptorProto>,
    }
    impl InputFileTrait for InputFileFake {
        fn gen_structs_for_messages(&self) -> Result<TokenStream> {
            Ok(TokenStream::new())
        }
        fn syntax(&self) -> Result<Syntax> {
            unimplemented!()
        }
        fn messages(
            &self,
        ) -> Box<dyn '_ + Iterator<Item = std::rc::Weak<dyn crate::codegen2::message::MessageTrait>>>
        {
            todo!()
        }

        fn enums(
            &self,
        ) -> Box<dyn '_ + Iterator<Item = std::rc::Weak<dyn crate::codegen2::r#enum::EnumTrait>>>
        {
            todo!()
        }
    }
    #[derive(Default, Debug)]
    struct InputFileFakeFactory {
        given_protos: Vec<Rc<FileDescriptorProto>>,
        generated_fakes: Vec<Rc<dyn InputFileTrait>>,
    }
    impl InputFileFakeFactory {
        fn add(&mut self, proto: &FileDescriptorProto) -> Rc<dyn InputFileTrait> {
            let rc_proto = Rc::new(proto.clone());
            let fake: Rc<dyn InputFileTrait> = Rc::new(InputFileFake {
                proto: Rc::clone(&rc_proto),
            });
            self.given_protos.push(rc_proto);
            self.generated_fakes.push(Rc::clone(&fake));
            fake
        }
    }

    #[test]
    fn test_make_package_empty() {
        let files = [Lazy::force(&FD_ROOT)];
        let mut factory = InputFileFakeFactory::default();
        let dyn_root_package =
            Package::new_from_files_with(files.into_iter(), |f, _| factory.add(f));
        let root_package = dyn_root_package.as_package().unwrap();
        assert_eq!(1, root_package.files.len());
        assert_eq!(Lazy::force(&FD_ROOT), factory.given_protos[0].as_ref());
    }

    #[test]
    fn test_make_package_single() {
        let files = [Lazy::force(&FD_G_P_DESC)];
        let mut factory = InputFileFakeFactory::default();
        let dyn_root_package =
            Package::new_from_files_with(files.into_iter(), |f, _| factory.add(f));
        let root_package = dyn_root_package.as_package().unwrap();
        assert_eq!(0, root_package.files.len());
        assert_eq!(1, root_package.subpackages.len());
        assert!(root_package.subpackages.contains_key("google"));

        let package_g = root_package.subpackages["google"].as_package().unwrap();
        assert_eq!("google", package_g.name);
        assert_eq!("google", package_g.full_name);
        assert_eq!(0, package_g.files.len());
        assert_eq!(1, package_g.subpackages.len());
        assert!(package_g.subpackages.contains_key("protobuf"));

        let package_g_p = package_g.subpackages["protobuf"].as_package().unwrap();
        assert_eq!("protobuf", package_g_p.name);
        assert_eq!("google.protobuf", package_g_p.full_name);
        assert_eq!(1, package_g_p.files.len());
        assert_eq!(Lazy::force(&FD_G_P_DESC), factory.given_protos[0].as_ref());
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
        let mut factory = InputFileFakeFactory::default();

        let dyn_root_package =
            Package::new_from_files_with(files.into_iter(), |f, _| factory.add(f));
        let root_package = dyn_root_package.as_package().unwrap();
        assert_eq!(1, root_package.files.len());
        assert_eq!(Lazy::force(&FD_ROOT), factory.given_protos[0].as_ref());
        assert_eq!(1, root_package.subpackages.len());
        assert!(root_package.subpackages.contains_key("google"));

        let package_g = root_package.subpackages["google"].as_package().unwrap();
        assert_eq!("google", package_g.name);
        assert_eq!("google", package_g.full_name);
        assert_eq!(0, package_g.files.len());
        assert_eq!(1, package_g.subpackages.len());
        assert!(package_g.subpackages.contains_key("protobuf"));

        let package_g_p = package_g.subpackages["protobuf"].as_package().unwrap();
        assert_eq!("protobuf", package_g_p.name);
        assert_eq!("google.protobuf", package_g_p.full_name);
        assert_eq!(2, package_g_p.files.len());
        assert_eq!(1, package_g_p.subpackages.len());
        assert!(package_g_p.subpackages.contains_key("compiler"));

        let package_g_p_c = package_g_p.subpackages["compiler"].as_package().unwrap();
        assert_eq!("compiler", package_g_p_c.name);
        assert_eq!("google.protobuf.compiler", package_g_p_c.full_name);
        assert_eq!(1, package_g_p_c.files.len());
        assert_eq!(
            Lazy::force(&FD_G_P_C_PLUGIN),
            factory.given_protos[3].as_ref()
        );
        assert_eq!(0, package_g_p_c.subpackages.len());
    }
}
