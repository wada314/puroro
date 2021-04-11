use crate::generators::shared::utils::{
    FullyQualifiedTypeName, MaybeFullyQualifiedTypeName, PackagePath,
};
use crate::generators::shared::visitor::{visit_in_file, DescriptorVisitor};
use crate::protos::*;
use crate::wrappers::FileDescriptor;
use crate::{ErrorKind, Result};
use ::once_cell::unsync::OnceCell;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub enum TypeOfIdent {
    Message,
    Enum,
}

pub struct Context<'c> {
    proto: CodeGeneratorRequest,
    lazy_file_descriptors: OnceCell<Vec<FileDescriptor<'c>>>,

    type_of_ident_map: OnceCell<HashMap<FullyQualifiedTypeName, TypeOfIdent>>,
    packages_subpackage_list: HashMap<PackagePath, HashSet<String>>,

    cur_package: PackagePath,
    path_to_package_root: String,
}

impl<'c> Context<'c> {
    pub fn new(cgreq: CodeGeneratorRequest) -> Result<Self> {
        Ok(Self {
            proto: cgreq.clone(), // for now...
            lazy_file_descriptors: Default::default(),
            type_of_ident_map: Default::default(),
            packages_subpackage_list: Self::generate_packages_subpackage_list(&cgreq),
            cur_package: PackagePath::new(""),
            path_to_package_root: "".into(),
        })
    }
    pub fn file_descriptors(&'c self) -> impl Iterator<Item = &FileDescriptor<'c>> + 'c {
        self.lazy_file_descriptors
            .get_or_init(|| {
                self.proto
                    .proto_file
                    .iter()
                    .map(|f| FileDescriptor::new(f, self))
                    .collect()
            })
            .iter()
    }

    pub fn cur_package(&self) -> &PackagePath {
        &self.cur_package
    }
    pub fn set_package(&mut self, package: &PackagePath) {
        self.cur_package = package.clone();
        self.path_to_package_root = Self::generate_path_to_package_root(&self.cur_package);
    }
    pub fn path_to_package_root(&self) -> &str {
        &self.path_to_package_root
    }

    pub fn enter_submessage_namespace(&mut self, message_name: &str) {
        self.cur_package.push(message_name);
        self.path_to_package_root = Self::generate_path_to_package_root(&self.cur_package);
    }

    pub fn leave_submessage_namespace(&mut self, message_name: &str) {
        if let Some(popped) = self.cur_package.pop() {
            debug_assert_eq!(message_name, popped);
        }
        self.path_to_package_root = Self::generate_path_to_package_root(&self.cur_package);
    }

    fn generate_path_to_package_root(package: &PackagePath) -> String {
        if package.is_empty() {
            "self".into()
        } else {
            let supers = std::iter::repeat("super").take(package.iter().count());
            Itertools::intersperse(supers, "::").collect::<String>()
        }
    }

    pub fn type_of_ident(&self, typename: &str) -> Result<Option<TypeOfIdent>> {
        let map = self
            .type_of_ident_map
            .get_or_try_init(|| self.generate_type_of_ident_map())?;
        let mut package = self.cur_package().clone();
        let mfqtn = MaybeFullyQualifiedTypeName::from_maybe_fq_typename(typename)?;
        if let Some(fqtn) = mfqtn.try_to_absolute() {
            return Ok(map.get(&fqtn).cloned());
        } else {
            loop {
                let fqtn = mfqtn.with_package(package.clone());
                if let Some(found) = map.get(&fqtn) {
                    return Ok(Some(found.clone()));
                }
                if package.pop().is_none() {
                    break;
                }
            }
            Ok(None)
        }
    }

    fn generate_type_of_ident_map(&self) -> Result<HashMap<FullyQualifiedTypeName, TypeOfIdent>> {
        struct Visitor<'a> {
            map: &'a mut HashMap<FullyQualifiedTypeName, TypeOfIdent>,
            package: PackagePath,
        }
        impl<'a> DescriptorVisitor for Visitor<'a> {
            fn handle_msg(&mut self, msg: &DescriptorProto) -> Result<()> {
                let fqtn = FullyQualifiedTypeName::new(self.package.clone(), &msg.name);
                if let Some(_) = self.map.insert(fqtn.clone(), TypeOfIdent::Message) {
                    Err(ErrorKind::ConflictedName {
                        name: fqtn.to_string(),
                    })?
                }
                Ok(())
            }

            fn handle_enum(&mut self, enume: &EnumDescriptorProto) -> Result<()> {
                let fqtn = FullyQualifiedTypeName::new(self.package.clone(), &enume.name);
                if let Some(_) = self.map.insert(fqtn.clone(), TypeOfIdent::Enum) {
                    Err(ErrorKind::ConflictedName {
                        name: fqtn.to_string(),
                    })?
                }
                Ok(())
            }

            fn enter_submodule(&mut self, name: &str) -> Result<()> {
                self.package.push(name);
                Ok(())
            }

            fn exit_submodule(&mut self, _name: &str) -> Result<()> {
                self.package.pop().unwrap();
                Ok(())
            }
        }

        let mut map = HashMap::new();
        for file in &self.proto.proto_file {
            let package = PackagePath::new(&file.package);
            let mut visitor = Visitor {
                map: &mut map,
                package,
            };
            visit_in_file(file, &mut visitor)?;
        }
        Ok(map)
    }

    pub fn packages_subpackage_list(&self) -> &HashMap<PackagePath, HashSet<String>> {
        &self.packages_subpackage_list
    }

    fn generate_packages_subpackage_list(
        cgreq: &CodeGeneratorRequest,
    ) -> HashMap<PackagePath, HashSet<String>> {
        let mut checked = HashSet::new();
        let mut map: HashMap<PackagePath, HashSet<String>> = HashMap::new();
        for file in &cgreq.proto_file {
            let mut package = PackagePath::new(&file.package);
            if !checked.insert(package.clone()) {
                // Already checked this package
                continue;
            }
            while let Some(popped) = package.pop() {
                map.entry(package.clone())
                    .or_insert_with(Default::default)
                    .insert(popped);
            }
        }
        map
    }
}
