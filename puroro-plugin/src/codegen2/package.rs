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
use ::proc_macro2::TokenStream;
use ::puroro_protobuf_compiled::google::protobuf::FileDescriptorProto;
use ::quote::{format_ident, quote};
use ::std::collections::HashMap;
use ::std::fmt::Debug;
use ::std::ops::Deref;
use ::std::rc::{Rc, Weak};

pub(super) trait PackageTrait: Debug {
    #[cfg(test)]
    fn as_package(&self) -> Option<&Package>;

    fn messages(&self) -> Box<dyn '_ + Iterator<Item = Weak<dyn MessageTrait>>>;
    fn enums(&self) -> Box<dyn '_ + Iterator<Item = Weak<dyn EnumTrait>>>;

    fn subpackages(&self) -> Box<dyn '_ + Iterator<Item = &dyn PackageTrait>>;
    fn subpackage(&self, name: &str) -> Option<&dyn PackageTrait>;
    fn module_file_name(&self) -> Result<String>;
    fn gen_module_file(&self) -> Result<TokenStream>;

    fn resolve_type_name(
        &self,
        type_name: &str,
    ) -> Result<MessageOrEnum<Weak<Box<dyn MessageTrait>>, Weak<dyn EnumTrait>>>;
}

pub(super) struct PackageBase {
    subpackages: HashMap<String, Rc<dyn PackageTrait>>,
    files: Vec<Rc<dyn InputFileTrait>>,
}

pub(super) struct NonRootPackage {
    name: String,
    parent: Weak<dyn PackageTrait>,
    root: Weak<dyn PackageTrait>,
    base: PackageBase,
}
pub(super) struct RootPackage {
    base: PackageBase,
}

impl PackageBase {
    fn new<'a, FP, FF>(
        fds: impl Iterator<Item = &'a FileDescriptorProto>,
        package_depth: usize,
        mut fp: FP,
        mut ff: FF,
    ) -> Self
    where
        FP: FnMut(&[&FileDescriptorProto], usize) -> Rc<dyn PackageTrait>,
        FF: FnMut(&FileDescriptorProto) -> Rc<dyn InputFileTrait>,
    {
        // Some FDs might directly belong to the current package.
        // The remaining FDs belong to the child packages.
        let (self_fds, child_fds) = fds.partition::<Vec<_>, _>(|fd| {
            fd.package().split('.').filter(|s| !s.is_empty()).count() <= package_depth
        });

        let self_files = self_fds.into_iter().map(|fd| ff(fd)).collect::<Vec<_>>();

        let child_fds_map = child_fds
            .into_iter()
            .map(|fd| {
                let package_name = fd
                    .package()
                    .split('.')
                    .filter(|s| !s.is_empty())
                    .nth(package_depth)
                    .unwrap_or("")
                    .to_string();
                (package_name, *fd)
            })
            .into_group_map();

        PackageBase {
            subpackages,
            files: self_files,
        }
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

impl PackageTrait for Package {
    #[cfg(test)]
    fn as_package(&self) -> Option<&Package> {
        Some(self)
    }

    fn subpackages(&self) -> Box<dyn '_ + Iterator<Item = &dyn PackageTrait>> {
        Box::new(self.subpackages.iter().map(|(_, p)| Rc::deref(p)))
    }

    fn module_file_name(&self) -> Result<String> {
        Ok(if self.full_name.is_empty() {
            "lib.rs".into()
        } else {
            format!(
                "{}.rs",
                self.full_name
                    .split('.')
                    .map(|package| package
                        .to_lower_snake_case()
                        .escape_rust_keywords()
                        .to_string())
                    .join("/")
            )
            .into()
        })
    }

    fn gen_module_file(&self) -> Result<TokenStream> {
        let submodules_from_packages = self
            .subpackages
            .keys()
            .sorted()
            .map(|name| format_ident!("{}", name.to_lower_snake_case().escape_rust_keywords()))
            .collect::<Vec<_>>();
        let message_structs = self
            .files
            .iter()
            .map(|f| f.gen_structs_for_messages())
            .collect::<Result<Vec<_>>>()?;

        let header = if self.full_name.is_empty() {
            quote! {
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
            }
        } else {
            let comment = format!("Generated from package \"{}\"", &self.full_name);
            quote! {
                #![doc = #comment]
                pub mod _puroro_root {
                    pub use super::super::_puroro_root::*;
                }
                pub mod _puroro {
                    pub use ::puroro::*;
                }
            }
        };

        Ok(quote! {
            #header
            #(pub mod #submodules_from_packages;)*
            #(#message_structs)*
        })
    }

    fn resolve_type_name(
        &self,
        type_name: &str,
    ) -> Result<MessageOrEnum<Weak<Box<dyn MessageTrait>>, Weak<dyn EnumTrait>>> {
        // Case 1, the given type name is an absolute path.
        // If the type_name starts with '.', then redirect it to the root package.
        if let Some(abs_type_name) = type_name.strip_prefix('.') {
            return self.root.try_upgrade()?.resolve_type_name(abs_type_name);
        }

        let rest = type_name;
        let mut cur = MessageOrPackage::<&dyn MessageTrait, &dyn PackageTrait>::Package(self);
        while let Some((subcomponent_name, rest)) = rest.split_once('.') {
            // There are a sub-sub-component. Thus, the subcomponent is either a package or a message.
            if let MessageOrPackage::Package(p) = cur {
                if let Some(subsub) = p.subpackage(subcomponent_name) {}
            }
        }

        // Case 2, the next component of the path is a subpackage.
        // Extract the next component of the path. It must be non-empty, and must have another child.
        if let Some((subpackage_name, rest)) = type_name.split_once('.') {
            if let Some(subpackage) = self.subpackages.get(subpackage_name) {
                // Can dig into a subpackage. Go ahead.
                return subpackage.resolve_type_name(rest);
            } else {
                // When the target item is a nested message or enum in a message, then this can happen.
                // Do nothing, go to the next section.
            }
        }

        let (subitem_name, rest) = type_name.split_once('.').unwrap_or((type_name, ""));

        // Case 3, the next component of the path is a message.
        // The message can still have a submessage, so go deeper.
        if let Some(message) = self
            .messages()
            .try_find(|m| -> Result<_> { Ok(m.try_upgrade()?.name() == subitem_name) })?
        {
            return todo!();
        }

        // Case 4, the next component is an enum.
        // Enum cannot have a subitem so just return the found enum immediately.
        if let Some(enume) = self
            .enums()
            .try_find(|e| -> Result<_> { Ok(e.try_upgrade()?.name() == subitem_name) })?
        {
            // In this case, there should not be the remaining path component.
            if !rest.is_empty() {
                Err(ErrorKind::UnknownTypeName {
                    name: type_name.to_string(),
                })?;
            }

            return Ok(MessageOrEnum::Enum(enume));
        }

        // Case 5, Type not found case.
        Err(ErrorKind::UnknownTypeName {
            name: type_name.to_string(),
        })?
    }

    fn subpackage(&self, name: &str) -> Option<&dyn PackageTrait> {
        self.subpackages.get(name).map(|p| Rc::deref(p))
    }

    fn messages(&self) -> Box<dyn '_ + Iterator<Item = Weak<dyn MessageTrait>>> {
        Box::new(self.files.iter().flat_map(|f| f.messages()))
    }

    fn enums(&self) -> Box<dyn '_ + Iterator<Item = Weak<dyn EnumTrait>>> {
        Box::new(self.files.iter().flat_map(|f| f.enums()))
    }
}

impl Package {
    pub(super) fn new_from_files<'a, I: Iterator<Item = &'a FileDescriptorProto>>(
        iter: I,
    ) -> Rc<dyn PackageTrait> {
        Self::new_from_files_with(iter, |fd, _weak| InputFile::new(fd))
    }

    pub(super) fn new_from_files_with<'a, I: Iterator<Item = &'a FileDescriptorProto>, FF>(
        iter: I,
        mut ff: FF,
    ) -> Rc<dyn PackageTrait>
    where
        FF: FnMut(&FileDescriptorProto, Weak<Package>) -> Rc<dyn InputFileTrait>,
    {
        let mut files = iter.collect::<Vec<_>>();
        files.sort_by_key(|f| f.package());

        Package::make_package("", "", &files, None, &mut ff)
    }

    fn make_package<FF>(
        name: &str,
        full_name: &str,
        sorted_fds: &[&FileDescriptorProto],
        maybe_root: Option<Weak<dyn PackageTrait>>,
        ff: &mut FF,
    ) -> Rc<dyn PackageTrait>
    where
        FF: FnMut(&FileDescriptorProto, Weak<Package>) -> Rc<dyn InputFileTrait>,
    {
        Rc::new_cyclic(|weak| {
            // The first few FDs might be FDs which are directly belonging to the current package.
            // The remaining FDs are belonging to the child packages.
            let (self_fds, child_fds) = sorted_fds.split_while(|fd| fd.package() == full_name);
            let root = maybe_root.as_ref().map_or_else(
                || Weak::clone(weak) as Weak<dyn PackageTrait>,
                |r| Weak::clone(r),
            );

            let self_files = self_fds
                .into_iter()
                .map(|fd| ff(fd, Weak::clone(&weak)))
                .collect::<Vec<_>>();

            let subpackages = child_fds
                .group_by_key(|fd| {
                    let prefix_len = if full_name.is_empty() {
                        0
                    } else {
                        full_name.len() + 1
                    };
                    match fd.package()[prefix_len..].split_once('.') {
                        Some((name, _)) => name,
                        None => &fd.package()[prefix_len..],
                    }
                })
                .map(|(subpackage_name, subpackage_fds)| {
                    let subpackage_full_name = if full_name.is_empty() {
                        subpackage_name.to_string()
                    } else {
                        format!("{}.{}", full_name, subpackage_name)
                    };
                    (
                        subpackage_name.to_string(),
                        Package::make_package(
                            subpackage_name,
                            &subpackage_full_name,
                            subpackage_fds,
                            Some(Weak::clone(&root)),
                            ff,
                        ),
                    )
                })
                .collect::<HashMap<_, _>>();

            Package {
                name: name.to_string(),
                full_name: full_name.to_string(),
                subpackages,
                files: self_files,
                root,
            }
        })
    }
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
