use crate::generators::shared::*;
use crate::generators::utils::*;
use crate::plugin::*;
use crate::{ErrorKind, Result};
use itertools::Itertools;
use std::{borrow::Cow, fmt::Write};

struct Generator {}
impl FileGeneratorHandler for Generator {
    fn handle_msg<'p, W: Write>(
        &mut self,
        context: &InvocationContext,
        fc: &mut FileGeneratorContext<'p, W>,
        msg: &'p DescriptorProto,
    ) -> Result<()> {
        let native_type_name = to_type_name(&msg.name);

        // struct body
        write!(fc.writer(), "pub struct {name} ", name = native_type_name)?;
        fc.indent_with_braces(|fc| {
            for field in &msg.field {
                let field_native_type = self.gen_field_type(field, context, fc)?;
                writeln!(
                    fc.writer(),
                    "{name}: {type_},",
                    name = to_var_name(&field.name),
                    type_ = field_native_type
                )?;
            }
            Ok(())
        })?;

        // impl Default
        // Because enum is Result<enum, i32>, we need a special treatment for it.
        write!(
            fc.writer(),
            "impl ::std::default::Default for {name} ",
            name = native_type_name
        )?;
        fc.indent_with_braces(|fc| {
            write!(fc.writer(), "fn default() -> Self ")?;
            fc.indent_with_braces(|fc| {
                write!(fc.writer(), "Self ")?;
                fc.indent_with_braces(|fc| {
                    for field in &msg.field {
                        let native_field_name = to_var_name(&field.name);
                        if let Some(TypeOfIdent::Enum) =
                            context.type_of_ident(fc.package().clone(), &field.type_name)
                        {
                            writeln!(
                                fc.writer(),
                                "{name}: 0i32.try_into(),",
                                name = native_field_name
                            )?;
                        } else {
                            writeln!(
                                fc.writer(),
                                "{name}: std::default::Default::default(),",
                                name = native_field_name
                            )?;
                        }
                    }
                    Ok(())
                })
            })
        })?;

        Ok(())
    }

    fn handle_enum<'p, W: Write>(
        &mut self,
        _context: &InvocationContext,
        fc: &mut FileGeneratorContext<'p, W>,
        enume: &'p EnumDescriptorProto,
    ) -> Result<()> {
        let native_type_name = to_type_name(&enume.name);

        // enum body
        write!(fc.writer(), "pub enum {name} ", name = native_type_name)?;
        fc.indent_with_braces(|fc| {
            for value in &enume.value {
                let name = to_enum_value_name(&value.name);
                writeln!(
                    fc.writer(),
                    "{name} = {number},",
                    name = name,
                    number = value.number
                )?;
            }
            Ok(())
        })?;

        // TryFrom<i32>
        write!(
            fc.writer(),
            "impl std::convert::TryFrom<i32> for {name} ",
            name = native_type_name
        )?;
        fc.indent_with_braces(|fc| {
            write!(
                fc.writer(),
                "type Error = i32; \
                fn try_from(val: i32) -> std::result::Result<Self, i32> "
            )?;
            fc.indent_with_braces(|fc| {
                write!(fc.writer(), "match val ")?;
                fc.indent_with_braces(|fc| {
                    for value in &enume.value {
                        let value_name = to_enum_value_name(&value.name);
                        writeln!(
                            fc.writer(),
                            "{number} => Ok(Self::{name}),",
                            number = value.number,
                            name = value_name
                        )?;
                    }
                    writeln!(fc.writer(), "x => Err(x),")?;
                    Ok(())
                })
            })
        })?;

        Ok(())
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
impl Generator {
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

    fn gen_field_type<'p, W: Write>(
        &self,
        field: &'p FieldDescriptorProto,
        context: &InvocationContext,
        fc: &FileGeneratorContext<'p, W>,
    ) -> Result<Cow<'static, str>> {
        let bare_type =
            self.gen_field_bare_type(field.type_, &field.type_name, fc.path_to_package_root())?;
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
}

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
