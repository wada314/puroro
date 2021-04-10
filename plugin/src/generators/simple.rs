use crate::generators::shared::writers::*;
use crate::generators::shared::*;
use crate::generators::utils::*;
use crate::plugin::*;
use crate::{ErrorKind, Result};
use itertools::Itertools;
use std::{borrow::Cow, fmt::Write};

mod enume;
mod msg;

pub(crate) fn generate_simple(context: &InvocationContext) -> Result<Vec<(String, String)>> {
    let mut filename_and_content = Vec::new();
    let mut generator = Generator {};
    for proto_file in &context.cgreq().proto_file {
        filename_and_content.push(generate_file_with_handler(
            context,
            proto_file,
            &mut generator,
        )?);
    }
    Ok(filename_and_content)
}

fn is_field_enum(
    field: &FieldDescriptorProto,
    context: &InvocationContext,
    fc: &FileGeneratorContext,
) -> bool {
    matches!(
        context.type_of_ident(fc.package().clone(), &field.type_name),
        Some(TypeOfIdent::Enum)
    )
}

fn is_field_repeated(field: &FieldDescriptorProto) -> bool {
    matches!(field.label, Ok(FieldDescriptorProto_Label::LABEL_REPEATED))
}

// Message -> the message type itself (no Option).
// Enum -> the enum type itself (no Result).
// repeated -> not considered.
fn gen_field_bare_type(
    field_type: std::result::Result<FieldDescriptorProto_Type, i32>,
    typename: &str,
    path_to_package_root: &str,
) -> Result<Cow<'static, str>> {
    match field_type {
        Ok(body) => {
            Ok(match body {
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
            })
        }
        Err(id) => Err(ErrorKind::UnknownFieldTypeId { id })?,
    }
}

fn gen_field_type<'p>(
    field: &'p FieldDescriptorProto,
    context: &InvocationContext,
    fc: &FileGeneratorContext<'p>,
) -> Result<Cow<'static, str>> {
    let bare_type = gen_field_bare_type(field.type_, &field.type_name, fc.path_to_package_root())?;
    let type_of_ident = context.type_of_ident(fc.package().clone(), &field.type_name);
    Ok(match field.label {
        Ok(label_body) => match label_body {
            FieldDescriptorProto_Label::LABEL_OPTIONAL
            | FieldDescriptorProto_Label::LABEL_REQUIRED => match type_of_ident {
                Some(TypeOfIdent::Enum) => {
                    // The proto enum may have an unknown value.
                    // In rust enum undefined value causes an undefined behavior...
                    // https://doc.rust-lang.org/reference/behavior-considered-undefined.html
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

struct Generator {}
impl FileGeneratorHandler for Generator {
    fn handle_msg<'p, W: Write>(
        &mut self,
        output: &mut Indentor<W>,
        context: &InvocationContext,
        fc: &mut FileGeneratorContext<'p>,
        msg: &'p DescriptorProto,
    ) -> Result<()> {
        msg::handle_msg(output, context, fc, msg)
    }

    fn handle_enum<'p, W: Write>(
        &mut self,
        output: &mut Indentor<W>,
        context: &InvocationContext,
        fc: &mut FileGeneratorContext<'p>,
        enume: &'p EnumDescriptorProto,
    ) -> Result<()> {
        enume::handle_enum(output, context, fc, enume)
    }

    fn generate_filename<'p>(
        &mut self,
        _context: &InvocationContext,
        file: &'p FileDescriptorProto,
    ) -> Result<String> {
        if file.package.is_empty() {
            Ok("mod.rs".into())
        } else {
            Ok(Itertools::intersperse(
                file.package.split('.').map(|p| to_module_name(p)),
                "/".to_string(),
            )
            .collect::<String>()
                + ".rs")
        }
    }
}
