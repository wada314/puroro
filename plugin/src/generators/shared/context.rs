use super::utils::{FullyQualifiedTypeName, MaybeFullyQualifiedTypeName, PackagePath};
use super::visitor::{visit_in_file, DescriptorVisitor};
use crate::protos::*;
use crate::{ErrorKind, Result};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub enum TypeOfIdent {
    Message,
    Enum,
}

pub struct Context<'p> {
    cgreq: &'p CodeGeneratorRequest,
    type_of_ident_map: HashMap<FullyQualifiedTypeName<'p>, TypeOfIdent>,
    packages_subpackage_list: HashMap<PackagePath<'p>, HashSet<&'p str>>,

    cur_package: PackagePath<'p>,
    path_to_package_root: String,
}

impl<'p> Context<'p> {
    pub fn new(cgreq: &'p CodeGeneratorRequest) -> Result<Self> {
        Ok(Self {
            cgreq,
            type_of_ident_map: Self::generate_type_of_ident_map(cgreq)?,
            packages_subpackage_list: Self::generate_packages_subpackage_list(cgreq),
            cur_package: PackagePath::new(""),
            path_to_package_root: "".into(),
        })
    }
    pub fn cgreq(&self) -> &'p CodeGeneratorRequest {
        self.cgreq
    }
    pub fn cur_package(&self) -> &PackagePath<'p> {
        &self.cur_package
    }
    pub fn set_package(&mut self, package: &PackagePath<'p>) {
        self.cur_package = package.clone();
        self.path_to_package_root = Self::generate_path_to_package_root(&self.cur_package);
    }
    pub fn path_to_package_root(&self) -> &str {
        &self.path_to_package_root
    }

    pub fn enter_submessage_namespace(&mut self, message_name: &'p str) {
        self.cur_package.push(message_name);
        self.path_to_package_root = Self::generate_path_to_package_root(&self.cur_package);
    }

    pub fn leave_submessage_namespace(&mut self, message_name: &'p str) {
        if let Some(popped) = self.cur_package.pop() {
            debug_assert_eq!(message_name, popped);
        }
        self.path_to_package_root = Self::generate_path_to_package_root(&self.cur_package);
    }

    fn generate_path_to_package_root(package: &PackagePath<'p>) -> String {
        if package.is_empty() {
            "self".into()
        } else {
            let supers = std::iter::repeat("super").take(package.iter().count());
            Itertools::intersperse(supers, "::").collect::<String>()
        }
    }

    pub fn type_of_ident(&self, typename: &'p str) -> Option<TypeOfIdent> {
        let mut package = self.cur_package().clone();
        let mfqtn = MaybeFullyQualifiedTypeName::from_maybe_fq_typename(typename)?;
        if let Some(fqtn) = mfqtn.try_to_absolute() {
            return self.type_of_ident_map.get(&fqtn).cloned();
        } else {
            loop {
                let fqtn = mfqtn.with_package(&package);
                if let Some(found) = self.type_of_ident_map.get(&fqtn) {
                    return Some(found.clone());
                }
                if package.pop().is_none() {
                    break;
                }
            }
            None
        }
    }

    fn generate_type_of_ident_map(
        cgreq: &'p CodeGeneratorRequest,
    ) -> Result<HashMap<FullyQualifiedTypeName<'p>, TypeOfIdent>> {
        struct Visitor<'a, 'p> {
            map: &'a mut HashMap<FullyQualifiedTypeName<'p>, TypeOfIdent>,
            package: PackagePath<'p>,
        }
        impl<'a, 'p> DescriptorVisitor<'p> for Visitor<'a, 'p> {
            fn handle_msg(&mut self, msg: &'p DescriptorProto) -> Result<()> {
                let fqtn = FullyQualifiedTypeName::new(self.package.clone(), &msg.name);
                if let Some(_) = self.map.insert(fqtn.clone(), TypeOfIdent::Message) {
                    Err(ErrorKind::ConflictedName {
                        name: fqtn.to_string(),
                    })?
                }
                Ok(())
            }

            fn handle_enum(&mut self, enume: &'p EnumDescriptorProto) -> Result<()> {
                let fqtn = FullyQualifiedTypeName::new(self.package.clone(), &enume.name);
                if let Some(_) = self.map.insert(fqtn.clone(), TypeOfIdent::Enum) {
                    Err(ErrorKind::ConflictedName {
                        name: fqtn.to_string(),
                    })?
                }
                Ok(())
            }

            fn enter_submodule(&mut self, name: &'p str) -> Result<()> {
                self.package.push(name);
                Ok(())
            }

            fn exit_submodule(&mut self, _name: &'p str) -> Result<()> {
                self.package.pop().unwrap();
                Ok(())
            }
        }

        let mut map = HashMap::new();
        for file in &cgreq.proto_file {
            let package = PackagePath::new(&file.package);
            let mut visitor = Visitor {
                map: &mut map,
                package,
            };
            visit_in_file(file, &mut visitor)?;
        }
        Ok(map)
    }

    pub fn packages_subpackage_list(&self) -> &HashMap<PackagePath<'p>, HashSet<&'p str>> {
        &self.packages_subpackage_list
    }

    fn generate_packages_subpackage_list(
        cgreq: &'p CodeGeneratorRequest,
    ) -> HashMap<PackagePath<'p>, HashSet<&'p str>> {
        let mut checked = HashSet::new();
        let mut map: HashMap<PackagePath, HashSet<&'p str>> = HashMap::new();
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
