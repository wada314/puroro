use super::shared::context::{Context, TypeOfIdent};
use super::shared::utils::{to_module_name, Indentor, MaybeFullyQualifiedTypeName};
use super::shared::writers::*;
use super::shared::{generate_file_with_handler, FileGeneratorHandler};
use crate::protos::*;
use crate::{ErrorKind, Result};
use itertools::Itertools;
use std::{borrow::Cow, collections::HashMap, fmt::Write};

mod enume;
mod msg;

pub fn generate_simple(context: &mut Context) -> Result<Vec<(String, String)>> {
    let mut filenames_and_contents = HashMap::new();
    let mut generator = Generator {};
    for proto_file in &context.cgreq().proto_file {
        let (filename, content) = generate_file_with_handler(context, proto_file, &mut generator)?;
        filenames_and_contents.insert(filename, content);
    }
    // We need to generate files for rust module / proto package structuring.
    // i.e. Files for module declaration `mod <package-name>;`.
    // Sometimes this is written into an existing file, and sometimes we
    // need to create a new file.
    for (package, subpackages) in context.packages_subpackage_list() {
        let filename = if package.is_empty() {
            "mod.rs".to_string()
        } else {
            Itertools::intersperse(package.iter().map(|p| to_module_name(p)), "/".to_string())
                .collect::<String>()
                + ".rs"
        };
        let decls = subpackages
            .iter()
            .map(|subp| format!("pub mod {name};\n", name = to_module_name(&subp)))
            .collect::<String>();
        // Generate a new file, or insert it to the beggining of the existing file.
        let entry = filenames_and_contents.entry(filename.clone());
        let content = entry.or_default();
        *content = decls + content;
    }

    Ok(filenames_and_contents.into_iter().collect::<Vec<_>>())
}

fn is_field_msg(field: &FieldDescriptorProto, context: &Context) -> bool {
    matches!(
        context.type_of_ident(&field.type_name),
        Some(TypeOfIdent::Message)
    )
}

fn is_field_enum(field: &FieldDescriptorProto, context: &Context) -> bool {
    matches!(
        context.type_of_ident(&field.type_name),
        Some(TypeOfIdent::Enum)
    )
}

fn is_field_repeated(field: &FieldDescriptorProto) -> bool {
    matches!(
        field.label,
        Ok(field_descriptor_proto::Label::LabelRepeated)
    )
}

// Message -> the message type itself (no Option).
// Enum -> the enum type itself (no Result).
// repeated -> not considered.
fn gen_field_bare_type(
    field_type: std::result::Result<field_descriptor_proto::Type, i32>,
    typename: &str,
    path_to_package_root: &str,
) -> Result<Cow<'static, str>> {
    match field_type {
        Ok(body) => Ok(match body {
            field_descriptor_proto::Type::TypeDouble => "f64".into(),
            field_descriptor_proto::Type::TypeFloat => "f32".into(),
            field_descriptor_proto::Type::TypeUint64
            | field_descriptor_proto::Type::TypeFixed64 => "u64".into(),
            field_descriptor_proto::Type::TypeUint32
            | field_descriptor_proto::Type::TypeFixed32 => "u32".into(),
            field_descriptor_proto::Type::TypeInt64
            | field_descriptor_proto::Type::TypeSint64
            | field_descriptor_proto::Type::TypeSfixed64 => "i64".into(),
            field_descriptor_proto::Type::TypeInt32
            | field_descriptor_proto::Type::TypeSint32
            | field_descriptor_proto::Type::TypeSfixed32 => "i32".into(),
            field_descriptor_proto::Type::TypeBool => "bool".into(),
            field_descriptor_proto::Type::TypeString => "String".into(),
            field_descriptor_proto::Type::TypeBytes => "::std::vec::Vec<u8>".into(),
            field_descriptor_proto::Type::TypeMessage | field_descriptor_proto::Type::TypeEnum => {
                MaybeFullyQualifiedTypeName::from_maybe_fq_typename(typename)
                    .unwrap()
                    .to_native_maybe_qualified_typename(path_to_package_root)
                    .into()
            }
            field_descriptor_proto::Type::TypeGroup => Err(ErrorKind::GroupNotSupported {})?,
        }),
        Err(id) => Err(ErrorKind::UnknownFieldTypeId { id })?,
    }
}

fn gen_field_type<'p>(
    field: &'p FieldDescriptorProto,
    context: &Context<'p>,
) -> Result<Cow<'static, str>> {
    let bare_type = gen_field_bare_type(
        field.type_.clone(),
        &field.type_name,
        context.path_to_package_root(),
    )?;
    let type_of_ident = context.type_of_ident(&field.type_name);
    Ok(match &field.label {
        Ok(label_body) => match label_body {
            field_descriptor_proto::Label::LabelOptional
            | field_descriptor_proto::Label::LabelRequired => match type_of_ident {
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
            field_descriptor_proto::Label::LabelRepeated => match type_of_ident {
                Some(TypeOfIdent::Enum) => {
                    format!("::std::vec::Vec<std::result::Result<{}, i32>>", bare_type).into()
                }
                _ => format!("::std::vec::Vec<{}>", bare_type).into(),
            },
        },
        Err(id) => Err(ErrorKind::UnknownLabelId { id: *id })?,
    })
}

struct Generator {}
impl FileGeneratorHandler for Generator {
    fn handle_msg<'p, W: Write>(
        &mut self,
        output: &mut Indentor<W>,
        context: &Context<'p>,
        msg: &'p DescriptorProto,
    ) -> Result<()> {
        msg::handle_msg(output, context, msg)
    }

    fn handle_enum<'p, W: Write>(
        &mut self,
        output: &mut Indentor<W>,
        context: &Context<'p>,
        enume: &'p EnumDescriptorProto,
    ) -> Result<()> {
        enume::handle_enum(output, context, enume)
    }

    fn generate_filename<'p>(
        &mut self,
        _context: &Context<'p>,
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
