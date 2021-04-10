use crate::generators::utils::*;
use crate::plugin::*;
use crate::{ErrorKind, Result};
use itertools::Itertools;
use std::{collections::HashMap, fmt::Write};

pub(crate) mod writers;

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
        struct Visitor<'a, 'p> {
            map: &'a mut HashMap<FullyQualifiedTypeName<'p>, TypeOfIdent>,
            package: Vec<&'p str>,
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
            let package = file.package.split('.').collect();
            let mut visitor = Visitor {
                map: &mut map,
                package,
            };
            visit_in_file(file, &mut visitor)?;
        }
        Ok(map)
    }
}

pub(crate) struct FileGeneratorContext<'p> {
    package: Vec<&'p str>,
    path_to_package_root: String,
}
impl<'p> FileGeneratorContext<'p> {
    pub(crate) fn new(package: Vec<&'p str>) -> Self {
        let path_to_package_root = Self::generate_path_to_package_root(&package);
        Self {
            package,
            path_to_package_root,
        }
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

    pub(crate) fn enter_submessage_namespace(&mut self, message_name: &'p str) {
        self.package.push(message_name);
        self.path_to_package_root = Self::generate_path_to_package_root(&self.package);
    }
    pub(crate) fn leave_submessage_namespace(&mut self, message_name: &'p str) {
        if let Some(popped) = self.package.pop() {
            debug_assert_eq!(message_name, popped);
        }
        self.path_to_package_root = Self::generate_path_to_package_root(&self.package);
    }
}

pub(crate) trait DescriptorVisitor<'p> {
    fn handle_msg(&mut self, msg: &'p DescriptorProto) -> Result<()>;
    fn handle_enum(&mut self, enume: &'p EnumDescriptorProto) -> Result<()>;
    fn enter_submodule(&mut self, name: &'p str) -> Result<()>;
    fn exit_submodule(&mut self, name: &'p str) -> Result<()>;
}

pub(crate) fn visit_in_file<'p, T: DescriptorVisitor<'p>>(
    file: &'p FileDescriptorProto,
    visitor: &mut T,
) -> Result<()> {
    enum Task<'q> {
        HandleMsg(&'q DescriptorProto),
        HandleEnum(&'q EnumDescriptorProto),
        EnterSubmodule(&'q str),
        ExitSubmodule(&'q str),
    }
    let mut tasks = file
        .message_type
        .iter()
        .map(|msg| Task::HandleMsg(msg))
        .chain(file.enum_type.iter().map(|enume| Task::HandleEnum(enume)))
        .collect::<Vec<_>>();

    while let Some(task) = tasks.pop() {
        match task {
            Task::HandleMsg(msg) => {
                visitor.handle_msg(msg)?;
                if !msg.nested_type.is_empty() || !msg.enum_type.is_empty() {
                    tasks.push(Task::ExitSubmodule(&msg.name));
                    tasks.extend(msg.nested_type.iter().map(|submsg| Task::HandleMsg(submsg)));
                    tasks.extend(msg.enum_type.iter().map(|enume| Task::HandleEnum(enume)));
                    tasks.push(Task::EnterSubmodule(&msg.name));
                }
            }
            Task::HandleEnum(enume) => {
                visitor.handle_enum(enume)?;
            }
            Task::EnterSubmodule(name) => {
                visitor.enter_submodule(name)?;
            }
            Task::ExitSubmodule(name) => {
                visitor.exit_submodule(name)?;
            }
        }
    }

    Ok(())
}

pub(crate) trait FileGeneratorHandler {
    fn handle_msg<'p, W: std::fmt::Write>(
        &mut self,
        out: &mut Indentor<W>,
        context: &InvocationContext,
        fc: &mut FileGeneratorContext<'p>,
        msg: &'p DescriptorProto,
    ) -> Result<()>;
    fn handle_enum<'p, W: std::fmt::Write>(
        &mut self,
        out: &mut Indentor<W>,
        context: &InvocationContext,
        fc: &mut FileGeneratorContext<'p>,
        enume: &'p EnumDescriptorProto,
    ) -> Result<()>;
    fn generate_filename<'p>(
        &mut self,
        context: &InvocationContext,
        file: &'p FileDescriptorProto,
    ) -> Result<String>;
}

pub(crate) fn generate_file_with_handler<'p, H>(
    context: &InvocationContext,
    input_file: &'p FileDescriptorProto,
    handler: &mut H,
) -> Result<(String, String)>
where
    H: FileGeneratorHandler,
{
    let package = input_file.package.split('.').collect::<Vec<_>>();
    let mut fc = FileGeneratorContext::new(package);
    let filename = handler.generate_filename(context, input_file)?;

    struct InnerVisitor<'a, 'q, H: FileGeneratorHandler> {
        output: Indentor<String>,
        context: &'a InvocationContext<'q>,
        fc: &'a mut FileGeneratorContext<'q>,
        handler: &'a mut H,
    }
    impl<'a, 'q, H> DescriptorVisitor<'q> for InnerVisitor<'a, 'q, H>
    where
        H: FileGeneratorHandler,
    {
        fn handle_msg(&mut self, msg: &'q DescriptorProto) -> Result<()> {
            self.handler
                .handle_msg(&mut self.output, self.context, self.fc, msg)
        }
        fn handle_enum(&mut self, enume: &'q EnumDescriptorProto) -> Result<()> {
            self.handler
                .handle_enum(&mut self.output, self.context, self.fc, enume)
        }
        fn enter_submodule(&mut self, name: &'q str) -> Result<()> {
            let module_name = to_module_name(name);
            writeln!(&mut self.output, "mod {name} {{", name = module_name)?;
            self.fc.enter_submessage_namespace(name);
            self.output.indent();
            Ok(())
        }
        fn exit_submodule(&mut self, name: &'q str) -> Result<()> {
            self.fc.leave_submessage_namespace(name);
            self.output.unindent();
            writeln!(self.output, "}}")?;
            Ok(())
        }
    }

    let mut inner_visitor = InnerVisitor {
        output: Indentor::new(String::new()),
        context,
        fc: &mut fc,
        handler,
    };
    visit_in_file(&input_file, &mut inner_visitor)?;

    Ok((filename, inner_visitor.output.into_inner()))
}
