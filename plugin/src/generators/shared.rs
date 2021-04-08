use crate::generators::utils::*;
use crate::plugin::*;
use crate::Result;
use itertools::Itertools;
use std::{collections::HashMap, fmt::Write};

#[derive(Debug, Clone)]
pub(crate) enum TypeOfIdent {
    Message,
    Enum,
}

pub(crate) struct InvocationContext<'p> {
    type_of_ident_map: HashMap<FullyQualifiedTypeName<'p>, TypeOfIdent>,
}

impl<'p> InvocationContext<'p> {
    pub(crate) fn new(cgreq: &'p CodeGeneratorRequest) -> Self {
        Self {
            type_of_ident_map: HashMap::new(),
        }
    }
    fn generate_type_of_ident_map(
        cgreq: &'p CodeGeneratorRequest,
    ) -> HashMap<FullyQualifiedTypeName<'p>, TypeOfIdent> {
        let mut map = HashMap::new();
        let mut package = Vec::new();
        fn for_msg<'p>(
            msg: &'p DescriptorProto,
            map: &mut HashMap<FullyQualifiedTypeName<'p>, TypeOfIdent>,
            package: &mut Vec<&'p str>,
        ) {
            todo!()
        }

        for file in &cgreq.proto_file {
            package = file.package.split('.').collect();
            for msg in &file.message_type {}
        }
        map
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
    fn generate_path_to_package_root(package: &Vec<&str>) -> String {
        if package.is_empty() {
            "self".into()
        } else {
            let supers = std::iter::repeat("super").take(package.len());
            Itertools::intersperse(supers, "::").collect::<String>()
        }
    }

    pub(crate) fn indent<F: FnOnce(&mut FileGeneratorContext<'w, 'p, W>) -> Result<()>>(
        &mut self,
        f: F,
    ) -> Result<()> {
        self.writer.indent();
        let ret = (f)(self);
        self.writer.unindent();
        ret
    }

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
