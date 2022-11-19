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

    fn subpackages(&self) -> Box<dyn '_ + Iterator<Item = &dyn PackageTrait>>;
    fn module_file_name(&self) -> Result<String>;
    fn gen_module_file(&self) -> Result<TokenStream>;

    fn resolve_type_name(
        &self,
        type_name: &str,
    ) -> Result<MessageOrEnum<Weak<Box<dyn MessageTrait>>, Weak<Box<dyn EnumTrait>>>>;
}

#[derive(Debug, Default)]
pub(super) struct Package {
    name: String,
    full_name: String,
    subpackages: HashMap<String, Rc<Box<dyn PackageTrait>>>,
    files: Vec<Rc<Box<dyn InputFileTrait>>>,
    root: Option<Weak<Box<dyn PackageTrait>>>,
}

impl PackageTrait for Package {
    #[cfg(test)]
    fn as_package(&self) -> Option<&Package> {
        Some(self)
    }

    fn subpackages(&self) -> Box<dyn '_ + Iterator<Item = &dyn PackageTrait>> {
        Box::new(
            self.subpackages
                .iter()
                .map(|(_, p)| (Box::deref(Rc::deref(p))) as &dyn PackageTrait),
        )
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
    ) -> Result<MessageOrEnum<Weak<Box<dyn MessageTrait>>, Weak<Box<dyn EnumTrait>>>> {
        // If the type_name starts with '.', then redirect it to the root package.
        if let Some(abs_type_name) = type_name.strip_prefix('.') {
            let Some(weak_root) = &self.root else {
                Err(ErrorKind::UnknownTypeName { name: type_name.to_string() })?
            };
            return weak_root.try_upgrade()?.resolve_type_name(abs_type_name);
        }
        todo!()
    }
}

impl Package {
    pub(super) fn try_new_from_files<'a, I: Iterator<Item = &'a FileDescriptorProto>>(
        iter: I,
    ) -> Result<Rc<Box<dyn PackageTrait>>> {
        Self::try_new_from_files_with(iter, |fd, _weak| InputFile::try_new(fd))
    }

    pub(super) fn try_new_from_files_with<'a, I: Iterator<Item = &'a FileDescriptorProto>, FF>(
        iter: I,
        mut ff: FF,
    ) -> Result<Rc<Box<dyn PackageTrait>>>
    where
        FF: FnMut(
            &FileDescriptorProto,
            Weak<Box<dyn PackageTrait>>,
        ) -> Result<Rc<Box<dyn InputFileTrait>>>,
    {
        let mut files = iter.collect::<Vec<_>>();
        files.sort_by_key(|f| f.package());

        Package::try_make_package("", "", &files, None, &mut ff)
    }

    fn try_make_package<FF>(
        name: &str,
        full_name: &str,
        sorted_fds: &[&FileDescriptorProto],
        root: Option<Weak<Box<dyn PackageTrait>>>,
        ff: &mut FF,
    ) -> Result<Rc<Box<dyn PackageTrait>>>
    where
        FF: FnMut(
            &FileDescriptorProto,
            Weak<Box<dyn PackageTrait>>,
        ) -> Result<Rc<Box<dyn InputFileTrait>>>,
    {
        Rc::try_new_boxed_cyclic(|weak| {
            // The first few FDs might be FDs which are directly belonging to the current package.
            // The remaining FDs are belonging to the child packages.
            let (self_fds, child_fds) = sorted_fds.split_while(|fd| fd.package() == full_name);

            let self_files = self_fds
                .into_iter()
                .map(|fd| ff(fd, Weak::clone(&weak)))
                .collect::<Result<Vec<_>>>()?;

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
                .map(|(subpackage_name, subpackage_fds)| -> Result<_> {
                    let subpackage_full_name = if full_name.is_empty() {
                        subpackage_name.to_string()
                    } else {
                        format!("{}.{}", full_name, subpackage_name)
                    };
                    Ok((
                        subpackage_name.to_string(),
                        Package::try_make_package(
                            subpackage_name,
                            &subpackage_full_name,
                            subpackage_fds,
                            Some(Weak::clone(&root.as_ref().unwrap_or(&weak))),
                            ff,
                        )?,
                    ))
                })
                .collect::<Result<HashMap<_, _>>>()?;

            Ok(Box::new(Package {
                name: name.to_string(),
                full_name: full_name.to_string(),
                subpackages,
                files: self_files,
                root,
            }))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::super::input_file::InputFileTrait;
    use super::FileDescriptorProto;
    use crate::Result;
    use ::once_cell::sync::Lazy;
    use ::proc_macro2::TokenStream;
    use ::std::ops::Deref;
    use ::std::rc::Rc;
    type RootPackage = super::Package;

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
        fn syntax(&self) -> crate::codegen2::Syntax {
            unimplemented!()
        }
    }
    #[derive(Default, Debug)]
    struct InputFileFakeFactory {
        given_protos: Vec<Rc<FileDescriptorProto>>,
        generated_fakes: Vec<Rc<Box<dyn InputFileTrait>>>,
    }
    impl InputFileFakeFactory {
        fn add(&mut self, proto: &FileDescriptorProto) -> Rc<Box<dyn InputFileTrait>> {
            let rc_proto = Rc::new(proto.clone());
            let fake: Rc<Box<dyn InputFileTrait>> = Rc::new(Box::new(InputFileFake {
                proto: Rc::clone(&rc_proto),
            }));
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
            RootPackage::try_new_from_files_with(files.into_iter(), |f, _| Ok(factory.add(f)))
                .unwrap();
        let root_package = dyn_root_package.as_package().unwrap();
        assert_eq!(1, root_package.files.len());
        assert_eq!(Lazy::force(&FD_ROOT), factory.given_protos[0].as_ref());
    }

    #[test]
    fn test_make_package_single() {
        let files = [Lazy::force(&FD_G_P_DESC)];
        let mut factory = InputFileFakeFactory::default();
        let dyn_root_package =
            RootPackage::try_new_from_files_with(files.into_iter(), |f, _| Ok(factory.add(f)))
                .unwrap();
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
            RootPackage::try_new_from_files_with(files.into_iter(), |f, _| Ok(factory.add(f)))
                .unwrap();
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
