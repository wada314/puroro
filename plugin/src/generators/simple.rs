use crate::generators::shared::*;
use crate::generators::utils::*;
use crate::plugin::*;
use crate::{ErrorKind, Result};
use itertools::Itertools;
use std::{borrow::Cow, fmt::Write};

struct MessageGenerator<'p /* 'p: Lifetime for the input protobuf */> {
    desc_proto: &'p DescriptorProto,
    sub_msg_generators: Vec<MessageGenerator<'p>>,
    sub_enum_generators: Vec<EnumGenerator<'p>>,
}
impl<'p> MessageGenerator<'p> {
    fn new(desc_proto: &'p DescriptorProto) -> Self {
        let sub_msg_generators = desc_proto
            .nested_type
            .iter()
            .map(|msg| MessageGenerator::new(msg))
            .collect::<Vec<_>>();
        let sub_enum_generators = desc_proto
            .enum_type
            .iter()
            .map(|enume| EnumGenerator::new(enume))
            .collect::<Vec<_>>();
        Self {
            desc_proto,
            sub_msg_generators,
            sub_enum_generators,
        }
    }

    fn gen_struct<W: Write>(
        &self,
        context: &InvocationContext,
        file: &mut FileGeneratorContext<'p, W>,
    ) -> Result<()> {
        if !self.desc_proto.nested_type.is_empty() || !self.desc_proto.enum_type.is_empty() {
            file.enter_submessage_namespace(&self.desc_proto.name, |file| {
                let module_name = to_module_name(&self.desc_proto.name);
                writeln!(file.writer(), "mod {name} {{", name = module_name)?;
                file.indent(|file| {
                    for subenum in &self.sub_enum_generators {
                        subenum.gen_enum(context, file)?;
                    }
                    for submsg in &self.sub_msg_generators {
                        submsg.gen_struct(context, file)?;
                    }
                    Ok(())
                })?;
                writeln!(file.writer(), "}}")?;
                Ok(())
            })?;
        }

        let native_type_name = to_type_name(&self.desc_proto.name);
        writeln!(
            file.writer(),
            "pub struct {name} {{",
            name = native_type_name
        )?;
        file.indent(|file| {
            for field in &self.desc_proto.field {
                let field_native_type = self.gen_field_type(field, context, file)?;
                writeln!(
                    file.writer(),
                    "{name}: {type_},",
                    name = to_var_name(&field.name),
                    type_ = field_native_type
                )?;
            }
            Ok(())
        })?;
        writeln!(file.writer(), "}}")?;
        Ok(())
    }

    // Message -> the message type itself (no Option).
    // Enum -> the enum type itself (no Result).
    // repeated -> not considered.
    fn gen_field_bare_type(
        &self,
        field_type: std::result::Result<FieldDescriptorProto_Type, i32>,
        typename: &str,
        path_to_package_root: &str,
    ) -> Result<Cow<'static, str>> {
        match field_type {
            Ok(body) => Ok(match body {
                FieldDescriptorProto_Type::TYPE_DOUBLE => "f64".into(),
                FieldDescriptorProto_Type::TYPE_FLOAT => "f32".into(),
                FieldDescriptorProto_Type::TYPE_UINT64
                | FieldDescriptorProto_Type::TYPE_FIXED64 => "u64".into(),
                FieldDescriptorProto_Type::TYPE_UINT32
                | FieldDescriptorProto_Type::TYPE_FIXED32 => "u32".into(),
                FieldDescriptorProto_Type::TYPE_INT64
                | FieldDescriptorProto_Type::TYPE_SINT64
                | FieldDescriptorProto_Type::TYPE_SFIXED64 => "i64".into(),
                FieldDescriptorProto_Type::TYPE_INT32
                | FieldDescriptorProto_Type::TYPE_SINT32
                | FieldDescriptorProto_Type::TYPE_SFIXED32 => "i32".into(),
                FieldDescriptorProto_Type::TYPE_BOOL => "bool".into(),
                FieldDescriptorProto_Type::TYPE_STRING => "String".into(),
                FieldDescriptorProto_Type::TYPE_BYTES => "::std::vec::Vec<u8>".into(),
                FieldDescriptorProto_Type::TYPE_MESSAGE | FieldDescriptorProto_Type::TYPE_ENUM => {
                    MaybeFullyQualifiedTypeName::from_maybe_fq_typename(typename)
                        .to_native_maybe_qualified_typename(path_to_package_root)
                        .into()
                }
                FieldDescriptorProto_Type::TYPE_GROUP => Err(ErrorKind::GroupNotSupported {})?,
            }),
            Err(id) => Err(ErrorKind::UnknownFieldTypeId { id })?,
        }
    }

    fn gen_field_type<W: Write>(
        &self,
        field: &FieldDescriptorProto,
        context: &InvocationContext,
        file: &FileGeneratorContext<'p, W>,
    ) -> Result<Cow<'static, str>> {
        let bare_type =
            self.gen_field_bare_type(field.type_, &field.type_name, file.path_to_package_root())?;
        let type_of_ident = context.type_of_ident(file.package().clone(), &field.type_name);
        Ok(match field.label {
            Ok(label_body) => match label_body {
                FieldDescriptorProto_Label::LABEL_OPTIONAL
                | FieldDescriptorProto_Label::LABEL_REQUIRED => match type_of_ident {
                    Some(TypeOfIdent::Enum) => {
                        format!("::std::result::Result<{}, i32>", bare_type).into()
                    }
                    Some(TypeOfIdent::Message) => {
                        format!("::std::option::Option<::std::boxed::Box<{}>>", bare_type).into()
                    }
                    _ => bare_type.into(),
                },
                FieldDescriptorProto_Label::LABEL_REPEATED => match type_of_ident {
                    Some(TypeOfIdent::Enum) => {
                        format!("::std::vec::Vec<std::result::Result<{}, i32>>", bare_type).into()
                    }
                    _ => format!("::std::vec::Vec<{}>", bare_type).into(),
                },
            },
            Err(id) => Err(ErrorKind::UnknownLabelId { id })?,
        })
    }
}

struct EnumGenerator<'p /* 'p: Lifetime for the input protobuf */> {
    enume: &'p EnumDescriptorProto,
}
impl<'p> EnumGenerator<'p> {
    fn new(enume_proto: &'p EnumDescriptorProto) -> Self {
        Self { enume: enume_proto }
    }

    fn gen_enum<W: Write>(
        &self,
        _context: &InvocationContext,
        file: &mut FileGeneratorContext<'p, W>,
    ) -> Result<()> {
        let native_type_name = to_type_name(&self.enume.name);
        writeln!(file.writer(), "pub enum {name} {{", name = native_type_name,)?;
        file.indent(|file| {
            for value in &self.enume.value {
                let name = to_enum_value_name(&value.name);
                writeln!(
                    file.writer(),
                    "{name} = {number},",
                    name = name,
                    number = value.number
                )?;
            }
            Ok(())
        })?;
        writeln!(file.writer(), "}}")?;
        Ok(())
    }
}

pub(crate) fn generate_simple(context: &InvocationContext) -> Result<Vec<(String, String)>> {
    let mut filename_and_content = Vec::new();
    for proto_file in &context.cgreq().proto_file {
        let package = proto_file.package.split('.').collect::<Vec<_>>();
        let mut file_context = FileGeneratorContext::new(String::new(), package.clone());
        for enume in &proto_file.enum_type {
            let generator = EnumGenerator::new(enume);
            generator.gen_enum(context, &mut file_context)?;
        }
        for msg in &proto_file.message_type {
            let generator = MessageGenerator::new(msg);
            generator.gen_struct(context, &mut file_context)?;
        }

        let output_filename = Itertools::intersperse(
            package.iter().map(|package| to_module_name(*package)),
            "/".to_string(), // This always need to be "/" even for Windows OS
        )
        .collect::<String>()
            + ".rs";
        filename_and_content.push((output_filename, file_context.into_inner()));
    }
    Ok(filename_and_content)
}
