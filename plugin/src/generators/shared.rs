use crate::generators::utils::*;
use crate::plugin::*;
use crate::{ErrorKind, Result};
use itertools::Itertools;
use std::{collections::HashMap, fmt::Write};

#[derive(Debug, Clone)]
pub(crate) enum TypeOfIdent {
    Message,
    Enum,
}

pub(crate) struct InvocationContext<'p> {
    cgreq: &'p CodeGeneratorRequest,
    type_of_ident_map: HashMap<FullyQualifiedTypeName<'p>, TypeOfIdent>,
}

impl<'p> InvocationContext<'p> {
    pub(crate) fn new(cgreq: &'p CodeGeneratorRequest) -> Result<Self> {
        Ok(Self {
            cgreq,
            type_of_ident_map: Self::generate_type_of_ident_map(cgreq)?,
        })
    }
    pub(crate) fn cgreq(&self) -> &CodeGeneratorRequest {
        self.cgreq
    }
    pub(crate) fn type_of_ident(
        &self,
        mut package: Vec<&'p str>,
        typename: &'p str,
    ) -> Option<TypeOfIdent> {
        let mfqtn = MaybeFullyQualifiedTypeName::from_maybe_fq_typename(typename);
        if let Some(fqtn) = mfqtn.try_to_absolute() {
            return self.type_of_ident_map.get(&fqtn).cloned();
        } else {
            loop {
                let fqtn = mfqtn.with_package(package.clone());
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
        fn for_msg<'p>(
            msg: &'p DescriptorProto,
            map: &mut HashMap<FullyQualifiedTypeName<'p>, TypeOfIdent>,
            package: &mut Vec<&'p str>,
        ) -> Result<()> {
            // Work for sub-msg and sub-enum
            package.push(&msg.name);
            for submsg in &msg.nested_type {
                for_msg(submsg, map, package)?;
            }
            for subenum in &msg.enum_type {
                for_enum(subenum, map, package)?;
            }
            package.pop();
            // Work for msg itself
            let fqtn = FullyQualifiedTypeName::new(package.clone(), &msg.name);
            if let Some(_) = map.insert(fqtn.clone(), TypeOfIdent::Message) {
                Err(ErrorKind::ConflictedName {
                    name: format!("{}", fqtn),
                })?;
            }
            Ok(())
        }
        fn for_enum<'p>(
            enume: &'p EnumDescriptorProto,
            map: &mut HashMap<FullyQualifiedTypeName<'p>, TypeOfIdent>,
            package: &mut Vec<&'p str>,
        ) -> Result<()> {
            let fqtn = FullyQualifiedTypeName::new(package.clone(), &enume.name);
            if let Some(_) = map.insert(fqtn.clone(), TypeOfIdent::Enum) {
                Err(ErrorKind::ConflictedName {
                    name: format!("{}", fqtn),
                })?;
            }
            Ok(())
        }

        let mut map = HashMap::new();
        let mut package;
        for file in &cgreq.proto_file {
            package = file.package.split('.').collect();
            for enume in &file.enum_type {
                for_enum(enume, &mut map, &mut package)?;
            }
            for msg in &file.message_type {
                for_msg(msg, &mut map, &mut package)?;
            }
        }
        Ok(map)
    }
}

pub(crate) struct FileGeneratorContext<'w, 'p, W: Write> {
    writer: Indentor<'w, W>,
    package: Vec<&'p str>,
    path_to_package_root: String,
}
impl<'w, 'p, W: Write> FileGeneratorContext<'w, 'p, W> {
    pub(crate) fn new(writer: &'w mut W, package: Vec<&'p str>) -> Self {
        let path_to_package_root = Self::generate_path_to_package_root(&package);
        Self {
            writer: Indentor::new(writer),
            package,
            path_to_package_root,
        }
    }
    pub(crate) fn writer(&mut self) -> &mut Indentor<'w, W> {
        &mut self.writer
    }
    pub(crate) fn package(&self) -> &Vec<&'p str> {
        &self.package
    }
    pub(crate) fn path_to_package_root(&self) -> &str {
        &self.path_to_package_root
    }

    fn generate_path_to_package_root(package: &Vec<&str>) -> String {
        if package.is_empty() {
            "self".into()
        } else {
            let supers = std::iter::repeat("super").take(package.len());
            Itertools::intersperse(supers, "::").collect::<String>()
        }
    }

    #[must_use]
    pub(crate) fn indent<F: FnOnce(&mut FileGeneratorContext<'w, 'p, W>) -> Result<()>>(
        &mut self,
        f: F,
    ) -> Result<()> {
        self.writer.indent();
        let ret = (f)(self);
        self.writer.unindent();
        ret
    }

    #[must_use]
    pub(crate) fn enter_submessage_namespace<
        F: FnOnce(&mut FileGeneratorContext<'w, 'p, W>) -> Result<()>,
    >(
        &mut self,
        message_name: &'p str,
        f: F,
    ) -> Result<()> {
        self.package.push(message_name);
        self.path_to_package_root = Self::generate_path_to_package_root(&self.package);
        let ret = (f)(self);
        self.package.pop();
        self.path_to_package_root = Self::generate_path_to_package_root(&self.package);
        ret
    }
}
