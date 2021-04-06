use crate::plugin::*;
use crate::utils::*;
use crate::{PuroroError, Result};
use itertools::Itertools;
use std::{collections::HashMap, fmt::Write};

#[derive(Debug, Clone)]
enum TypeOfIdent {
    Message,
    Enum,
}

struct Context<'a, 'b, W: Write> {
    write: Indentor<'a, W>,
    type_of_idents: HashMap<FullyQualifiedTypeName<'b>, TypeOfIdent>,

    path_to_package_root: Option<String>,
    package: Vec<&'b str>,
}
impl<'a, 'b, W: Write> Context<'a, 'b, W> {
    fn new(write: &'a mut W) -> Self {
        Self {
            write: Indentor::new(write),
            type_of_idents: HashMap::new(),
            path_to_package_root: None,
            package: Vec::new(),
        }
    }

    fn indent(&mut self) {
        self.write.indent()
    }
    fn unindent(&mut self) {
        self.write.unindent()
    }

    fn search_for_idents_type(&self, typename: &str) -> Option<TypeOfIdent> {
        if let Some(fqtn) = FullyQualifiedTypeName::from_typename(typename) {
            return self.type_of_idents.get(&fqtn).cloned();
        } else {
            let mut fqtn = FullyQualifiedTypeName::new(self.package.clone(), typename);
            loop {
                if let Some(found) = self.type_of_idents.get(&fqtn) {
                    return Some(found.clone());
                }
                if !fqtn.move_up() {
                    break;
                }
            }
        }
        None
    }

    fn gen_field(&mut self, field: &'b FieldDescriptorProto) -> Result<()> {
        let native_original_type = match field.type_ {
            Ok(FieldDescriptorProto_Type::TYPE_DOUBLE) => "f64".to_string(),
            Ok(FieldDescriptorProto_Type::TYPE_FLOAT) => "f32".to_string(),

            Ok(FieldDescriptorProto_Type::TYPE_INT32)
            | Ok(FieldDescriptorProto_Type::TYPE_SINT32)
            | Ok(FieldDescriptorProto_Type::TYPE_SFIXED32) => "i32".to_string(),

            Ok(FieldDescriptorProto_Type::TYPE_INT64)
            | Ok(FieldDescriptorProto_Type::TYPE_SINT64)
            | Ok(FieldDescriptorProto_Type::TYPE_SFIXED64) => "i64".to_string(),

            Ok(FieldDescriptorProto_Type::TYPE_UINT64)
            | Ok(FieldDescriptorProto_Type::TYPE_FIXED64) => "u64".to_string(),

            Ok(FieldDescriptorProto_Type::TYPE_UINT32)
            | Ok(FieldDescriptorProto_Type::TYPE_FIXED32) => "u32".to_string(),

            Ok(FieldDescriptorProto_Type::TYPE_BOOL) => "bool".to_string(),

            Ok(FieldDescriptorProto_Type::TYPE_STRING) => "String".to_string(),

            Ok(FieldDescriptorProto_Type::TYPE_MESSAGE)
            | Ok(FieldDescriptorProto_Type::TYPE_ENUM) => {
                FullyQualifiedTypeName::from_typename(&field.type_name)
                    .map(|fqtn| fqtn.to_qualified_typename(&self.path_to_package_root))
                    .unwrap_or_else(|| field.type_name.clone())
            }

            Ok(FieldDescriptorProto_Type::TYPE_BYTES) => "Vec<u8>".to_string(),
            _ => {
                if !field.type_name.is_empty() {
                    return Err(PuroroError::UnexpectedFieldType);
                } else {
                    FullyQualifiedTypeName::from_typename(&field.type_name)
                        .map(|fqtn| fqtn.to_qualified_typename(&self.path_to_package_root))
                        .unwrap_or_else(|| field.type_name.clone())
                }
            }
        };

        let typename = match field.label {
            Ok(FieldDescriptorProto_Label::LABEL_OPTIONAL) => {
                if matches!(field.type_, Ok(FieldDescriptorProto_Type::TYPE_MESSAGE))
                    || matches!(
                        self.search_for_idents_type(&field.name),
                        Some(TypeOfIdent::Message)
                    )
                {
                    format!("Box<Option<{}>>", native_original_type)
                } else {
                    native_original_type
                }
            }
            Ok(FieldDescriptorProto_Label::LABEL_REQUIRED) => {
                if matches!(field.type_, Ok(FieldDescriptorProto_Type::TYPE_MESSAGE))
                    || matches!(
                        self.search_for_idents_type(&field.name),
                        Some(TypeOfIdent::Message)
                    )
                {
                    format!("Box<{}>", native_original_type)
                } else {
                    native_original_type
                }
            }
            Ok(FieldDescriptorProto_Label::LABEL_REPEATED) => {
                format!("Vec<{}>", native_original_type)
            }
            _ => {
                return Err(PuroroError::InvalidFieldLabel);
            }
        };
        let name = to_var_name(&field.name);

        writeln!(
            self.write,
            "pub {name}: {typename},",
            name = name,
            typename = typename
        )?;

        Ok(())
    }

    fn gen_message(&mut self, message: &'b DescriptorProto) -> Result<()> {
        self.type_of_idents.insert(
            FullyQualifiedTypeName::new(self.package.clone(), &message.name),
            TypeOfIdent::Message,
        );

        writeln!(
            self.write,
            "pub struct {name} {{",
            name = to_type_name(&message.name)
        )?;
        self.indent();

        for field in &message.field {
            self.gen_field(field)?;
        }

        self.unindent();
        writeln!(
            self.write,
            "}} // struct {name} {{",
            name = to_type_name(&message.name)
        )?;
        Ok(())
    }

    fn gen_enum(&mut self, enume: &'b EnumDescriptorProto) -> Result<()> {
        self.type_of_idents.insert(
            FullyQualifiedTypeName::new(self.package.clone(), &enume.name),
            TypeOfIdent::Enum,
        );

        Ok(())
    }

    fn gen_file(&mut self, fdp: &'b FileDescriptorProto) -> Result<()> {
        let package_iter = fdp.package.split('.').filter(|s| !s.is_empty());
        self.package = package_iter.clone().collect();

        self.path_to_package_root = match package_iter.clone().count() {
            0 => None,
            x => {
                let supers = std::iter::repeat("super").take(x);
                Some(Itertools::intersperse(supers, "::").collect::<String>())
            }
        };
        for package in package_iter.clone() {
            writeln!(self.write, "mod {name} {{", name = to_module_name(package))?;
        }

        for desc in &fdp.message_type {
            self.gen_message(desc)?;
        }

        for package in package_iter.clone() {
            writeln!(
                self.write,
                "}} // mod {name} {{",
                name = to_module_name(package)
            )?;
        }

        Ok(())
    }
}

pub(crate) fn generate_simple(cgreq: &CodeGeneratorRequest) -> Result<String> {
    let mut output = String::new();
    let mut context = Context::new(&mut output);
    for fdp in &cgreq.proto_file {
        context.gen_file(fdp)?;
    }
    Ok(output)
}
