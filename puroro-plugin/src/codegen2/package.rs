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
use ::std::borrow::Cow;
use ::std::collections::HashMap;
use ::std::fmt::Debug;
use ::std::ops::Deref;
use ::std::rc::{Rc, Weak};

#[derive(Debug, Default)]
pub struct PackageCommon {
    subpackages: HashMap<String, NonRootPackage>,
    files: Vec<Rc<Box<dyn InputFileTrait>>>,
}

#[derive(Debug)]
pub struct NonRootPackage {
    common: PackageCommon,
    name: String,
    full_name: String,
    root: Weak<PackageCommon>,
}

pub trait PackageTrait: Debug {
    // fn subpackages(&self) -> &HashMap<String, NonRootPackage>;
    // fn module_file_name(&self) -> Cow<'_, str>;
}

impl PackageCommon {
    pub fn try_new_from_files<'a, I: Iterator<Item = &'a FileDescriptorProto>, FF>(
        iter: I,
        ff: FF,
    ) -> Result<Rc<PackageCommon>>
    where
        FF: FnMut(&FileDescriptorProto, Weak<PackageCommon>) -> Result<Rc<Box<dyn InputFileTrait>>>,
    {
        let mut files = iter.collect::<Vec<_>>();
        files.sort_by_key(|f| f.package());
        Rc::try_new_cyclic(|weak_root| -> Result<PackageCommon> {
            Ok(PackageCommon::try_make_package(
                "",
                &files,
                Weak::clone(weak_root),
                ff,
            )?)
        })
    }

    fn try_make_package<FF>(
        full_name: &str,
        sorted_fds: &[&FileDescriptorProto],
        root: Weak<PackageCommon>,
        ff: FF,
    ) -> Result<PackageCommon>
    where
        FF: FnMut(&FileDescriptorProto, Weak<PackageCommon>) -> Result<Rc<Box<dyn InputFileTrait>>>,
    {
        // The first few FDs might be FDs which are directly belonging to the current package.
        // The remaining FDs are belonging to the child packages.
        let (self_fds, child_fds) = sorted_fds.split_while(|fd| fd.package() == full_name);

        let self_files = self_fds
            .into_iter()
            .map(|fd| ff(fd, Weak::clone(&root)))
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
                    None => fd.package(),
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
                    NonRootPackage::try_make_package(
                        subpackage_name,
                        &subpackage_full_name,
                        subpackage_fds,
                        Weak::clone(&root),
                        ff,
                    )?,
                ))
            })
            .collect::<Result<HashMap<_, _>>>()?;

        Ok(PackageCommon {
            subpackages,
            files: self_files,
        })
    }

    pub fn get_all_subpackages(&self) -> impl IntoIterator<Item = &NonRootPackage> {
        let mut ret = Vec::new();
        for subpackage in self.subpackages.values() {
            subpackage.get_all_packages(&mut ret);
        }
        ret
    }
}

impl Deref for NonRootPackage {
    type Target = PackageCommon;
    fn deref(&self) -> &Self::Target {
        &self.common
    }
}

impl NonRootPackage {
    fn try_make_package<FF>(
        name: &str,
        full_name: &str,
        sorted_fds: &[&FileDescriptorProto],
        root: Weak<PackageCommon>,
        ff: FF,
    ) -> Result<Self>
    where
        FF: FnMut(&FileDescriptorProto, Weak<PackageCommon>) -> Result<Rc<Box<dyn InputFileTrait>>>,
    {
        Ok(NonRootPackage {
            common: PackageCommon::try_make_package(full_name, sorted_fds, root, ff)?,
            name: name.to_string(),
            full_name: full_name.to_string(),
            root,
        })
    }

    fn get_all_packages<'a>(&'a self, packages: &mut Vec<&'a NonRootPackage>) {
        packages.push(self);
        for subpackage in self.subpackages.values() {
            subpackage.get_all_packages(packages);
        }
    }
}

impl PackageTrait for PackageCommon {
    // fn subpackages(&self) -> &HashMap<String, NonRootPackage> {
    //     &self.subpackages
    // }
    // fn module_file_name(&self) -> Cow<'_, str> {
    //     "lib.rs".into()
    // }
}

impl PackageTrait for NonRootPackage {
    // fn subpackages(&self) -> &HashMap<String, NonRootPackage> {
    //     &self.subpackages
    // }
    // fn module_file_name(&self) -> Cow<'_, str> {
    //     format!(
    //         "{}.rs",
    //         self.full_name
    //             .split('.')
    //             .map(|package| package
    //                 .to_lower_snake_case()
    //                 .escape_rust_keywords()
    //                 .to_string())
    //             .join("/")
    //     )
    //     .into()
    // }
}

impl PackageCommon {
    pub fn gen_module_file(&self) -> Result<TokenStream> {
        self::gen_module_file_impl(self, self.gen_module_file_header()?)
    }
    fn gen_module_file_header(&self) -> Result<TokenStream> {
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
        })
    }
}

impl NonRootPackage {
    pub fn gen_module_file(&self) -> Result<TokenStream> {
        self::gen_module_file_impl(self, self.gen_module_file_header()?)
    }
    fn gen_module_file_header(&self) -> Result<TokenStream> {
        let comment = format!("Generated from package \"{}\"", &self.full_name);
        Ok(quote! {
            #![doc = #comment]
            pub mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }
            pub mod _puroro {
                pub use ::puroro::*;
            }
        })
    }
}

fn gen_module_file_impl(package: &impl PackageTrait, header: TokenStream) -> Result<TokenStream> {
    let submodules_from_packages = package
        .subpackages()
        .into_iter()
        .map(|(name, _)| format_ident!("{}", name.to_lower_snake_case().escape_rust_keywords()))
        .collect::<Vec<_>>();
    let message_structs = package
        .files()
        .into_iter()
        .map(|f| f.gen_structs_for_messages())
        .collect::<Result<Vec<_>>>()?;

    Ok(quote! {
        #header

        #(
            pub mod #submodules_from_packages;
        )*

        #(#message_structs)*
    })
}

#[cfg(test)]
mod tests {
    use super::super::input_file::InputFileFake;
    use super::FileDescriptorProto;
    use ::once_cell::sync::Lazy;
    use ::std::ops::Deref;
    type RootPackage = super::PackageCommon<InputFileFake>;

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
        assert_eq!("google", package_g.full_name);
        assert_eq!(0, package_g.files.len());
        assert_eq!(1, package_g.subpackages.len());
        assert!(package_g.subpackages.contains_key("protobuf"));

        let package_g_p = &package_g.subpackages["protobuf"];
        assert_eq!("protobuf", package_g_p.name);
        assert_eq!("google.protobuf", package_g_p.full_name);
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
        assert_eq!("google", package_g.full_name);
        assert_eq!(0, package_g.files.len());
        assert_eq!(1, package_g.subpackages.len());
        assert!(package_g.subpackages.contains_key("protobuf"));

        let package_g_p = &package_g.subpackages["protobuf"];
        assert_eq!("protobuf", package_g_p.name);
        assert_eq!("google.protobuf", package_g_p.full_name);
        assert_eq!(2, package_g_p.files.len());
        assert_eq!(1, package_g_p.subpackages.len());
        assert!(package_g_p.subpackages.contains_key("compiler"));

        let package_g_p_c = &package_g_p.subpackages["compiler"];
        assert_eq!("compiler", package_g_p_c.name);
        assert_eq!("google.protobuf.compiler", package_g_p_c.full_name);
        assert_eq!(1, package_g_p_c.files.len());
        assert_eq!(Lazy::force(&FD_G_P_C_PLUGIN), &package_g_p_c.files[0].proto);
        assert_eq!(0, package_g_p_c.subpackages.len());
    }
}
