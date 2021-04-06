use crate::plugin::*;
use crate::utils::*;
use crate::{PuroroError, Result};
use std::{collections::HashMap, fmt::Write};

enum TypeOfIdent {
    Message,
    Enum,
}

struct Context<'a, W: Write> {
    write: Indentor<'a, W>,
    type_of_idents: HashMap<String, TypeOfIdent>,
}
impl<'a, W: Write> Context<'a, W> {
    fn new(write: &'a mut W) -> Self {
        Self {
            write: Indentor::new(write),
            type_of_idents: HashMap::new(),
        }
    }

    fn indent(&mut self) {
        self.write.indent()
    }
    fn unindent(&mut self) {
        self.write.unindent()
    }

    fn gen_field(&mut self, field: &FieldDescriptorProto) -> Result<()> {
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
                    .map(|fqtn| fqtn.to_typename_from_root())
                    .unwrap_or_else(|| field.type_name.clone())
            }

            Ok(FieldDescriptorProto_Type::TYPE_BYTES) => "Vec<u8>".to_string(),
            _ => {
                if !field.type_name.is_empty() {
                    return Err(PuroroError::UnexpectedFieldType);
                } else {
                    FullyQualifiedTypeName::from_typename(&field.type_name)
                        .map(|fqtn| fqtn.to_typename_from_root())
                        .unwrap_or_else(|| field.type_name.clone())
                }
            }
        };

        let typename = match field.label {
            Ok(FieldDescriptorProto_Label::LABEL_OPTIONAL) => {
                if matches!(field.type_, Ok(FieldDescriptorProto_Type::TYPE_MESSAGE))
                    || matches!(
                        self.type_of_idents.get(&field.name),
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
                        self.type_of_idents.get(&field.name),
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
        let name = get_keywoard_safe_ident(&field.name);

        writeln!(
            self.write,
            "{name}: {typename},",
            name = name,
            typename = typename
        )?;

        Ok(())
    }

    fn gen_message(&mut self, message: &DescriptorProto) -> Result<()> {
        self.type_of_idents
            .insert(message.name.clone(), TypeOfIdent::Message);

        writeln!(
            self.write,
            "pub struct {name} {{",
            name = snake_case_to_camel_case(&message.name)
        )?;
        self.indent();

        for field in &message.field {
            self.gen_field(field)?;
        }

        writeln!(self.write, "}}")?; // struct {
        self.unindent();
        Ok(())
    }
}

pub(crate) fn generate_simple(cgreq: &CodeGeneratorRequest) -> Result<String> {
    let descriptors = cgreq
        .proto_file
        .iter()
        .map(|fdp| &fdp.message_type)
        .flatten();
    let mut output = String::new();
    let mut context = Context::new(&mut output);
    for desc in descriptors {
        context.gen_message(desc)?;
    }
    Ok(output)
}
