use crate::plugin::*;
use crate::utils::*;
use crate::{PuroroError, Result};
use itertools::Itertools;
use std::{borrow::Cow, collections::HashMap, fmt::Write};

#[derive(Debug, Clone)]
enum TypeOfIdent {
    Message,
    Enum,
}

struct StructGenerator<'a, 'b, W: Write> {
    write: Indentor<'a, W>,
    type_of_idents: HashMap<MaybeFullyQualifiedTypeName<'b>, TypeOfIdent>,
    package: Vec<&'b str>,
}
impl<'a, 'b, W: Write> StructGenerator<'a, 'b, W> {
    fn new(write: &'a mut W) -> Self {
        Self {
            write: Indentor::new(write),
            type_of_idents: HashMap::new(),
            package: Vec::new(),
        }
    }

    fn indent(&mut self) {
        self.write.indent()
    }
    fn unindent(&mut self) {
        self.write.unindent()
    }

    fn path_to_package_root(&self) -> Option<String> {
        if self.package.is_empty() {
            None
        } else {
            let supers = std::iter::repeat("super").take(self.package.len());
            Some(Itertools::intersperse(supers, "::").collect::<String>())
        }
    }

    fn search_for_idents_type(&self, typename: &str) -> Option<TypeOfIdent> {
        let fqtn = MaybeFullyQualifiedTypeName::from_maybe_fq_typename(typename);

        let mut fqtn = MaybeFullyQualifiedTypeName::new(self.package.clone(), typename);
        loop {
            if let Some(found) = self.type_of_idents.get(&fqtn) {
                return Some(found.clone());
            }
            if !fqtn.pop() {
                break;
            }
        }
        None;
        todo!()
    }
    fn check_is_struct_or_enum(&self, field: &FieldDescriptorProto) -> Option<TypeOfIdent> {
        match field.type_ {
            Ok(FieldDescriptorProto_Type::TYPE_ENUM) => Some(TypeOfIdent::Enum),
            Ok(FieldDescriptorProto_Type::TYPE_MESSAGE) => Some(TypeOfIdent::Message),
            Err(0) => self.search_for_idents_type(&field.type_name),
            _ => None,
        }
    }

    fn gen_field(&mut self, field: &'b FieldDescriptorProto) -> Result<()> {
        let native_original_type: Cow<'static, str> = match field.type_ {
            Ok(FieldDescriptorProto_Type::TYPE_DOUBLE) => "f64".into(),
            Ok(FieldDescriptorProto_Type::TYPE_FLOAT) => "f32".into(),

            Ok(FieldDescriptorProto_Type::TYPE_INT32)
            | Ok(FieldDescriptorProto_Type::TYPE_SINT32)
            | Ok(FieldDescriptorProto_Type::TYPE_SFIXED32) => "i32".into(),

            Ok(FieldDescriptorProto_Type::TYPE_INT64)
            | Ok(FieldDescriptorProto_Type::TYPE_SINT64)
            | Ok(FieldDescriptorProto_Type::TYPE_SFIXED64) => "i64".into(),

            Ok(FieldDescriptorProto_Type::TYPE_UINT64)
            | Ok(FieldDescriptorProto_Type::TYPE_FIXED64) => "u64".into(),

            Ok(FieldDescriptorProto_Type::TYPE_UINT32)
            | Ok(FieldDescriptorProto_Type::TYPE_FIXED32) => "u32".into(),

            Ok(FieldDescriptorProto_Type::TYPE_BOOL) => "bool".into(),

            Ok(FieldDescriptorProto_Type::TYPE_STRING) => "String".into(),

            Ok(FieldDescriptorProto_Type::TYPE_MESSAGE)
            | Ok(FieldDescriptorProto_Type::TYPE_ENUM) => {
                MaybeFullyQualifiedTypeName::from_maybe_fq_typename(&field.type_name)
                    .map(|fqtn| fqtn.to_qualified_typename(&self.path_to_package_root()))
                    .unwrap_or_else(|| field.type_name.clone())
                    .into()
            }

            Ok(FieldDescriptorProto_Type::TYPE_BYTES) => "Vec<u8>".into(),
            _ => {
                if !field.type_name.is_empty() {
                    return Err(PuroroError::UnexpectedFieldType);
                } else {
                    MaybeFullyQualifiedTypeName::from_maybe_fq_typename(&field.type_name)
                        .map(|fqtn| fqtn.to_qualified_typename(&self.path_to_package_root()))
                        .unwrap_or_else(|| field.type_name.clone())
                        .into()
                }
            }
        };

        let typename: Cow<'static, str> = match field.label {
            Ok(FieldDescriptorProto_Label::LABEL_OPTIONAL) => {
                if matches!(field.type_, Ok(FieldDescriptorProto_Type::TYPE_MESSAGE))
                    || matches!(
                        self.search_for_idents_type(&field.name),
                        Some(TypeOfIdent::Message)
                    )
                {
                    format!("Box<Option<{}>>", native_original_type).into()
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
                    format!("Box<{}>", native_original_type).into()
                } else {
                    native_original_type
                }
            }
            Ok(FieldDescriptorProto_Label::LABEL_REPEATED) => {
                format!("Vec<{}>", native_original_type).into()
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
        let native_type_name = to_type_name(&message.name);

        self.type_of_idents.insert(
            MaybeFullyQualifiedTypeName::new(self.package.clone(), &message.name),
            TypeOfIdent::Message,
        );
        // Check sub-messages and sub-enums first
        if !message.nested_type.is_empty() || !message.enum_type.is_empty() {
            writeln!(
                self.write,
                "mod {name} {{",
                name = to_module_name(&message.name)
            )?;
            self.indent();
            self.package.push(&message.name);
            for submessage in &message.nested_type {
                self.gen_message(submessage)?;
            }
            for enume in &message.enum_type {
                self.gen_enum(enume)?;
            }
            self.package.pop();

            self.unindent();
            writeln!(
                self.write,
                "}} // mod {name} {{",
                name = to_module_name(&message.name)
            )?;
        }

        // Struct body
        writeln!(self.write, "pub struct {name} {{", name = native_type_name)?;
        self.indent();

        for field in &message.field {
            self.gen_field(field)?;
        }

        self.unindent();
        writeln!(
            self.write,
            "}} // struct {name} {{",
            name = native_type_name
        )?;

        // Default
        writeln!(
            self.write,
            "impl Default for {name} {{",
            name = native_type_name
        )?;
        self.indent();
        {
            writeln!(self.write, "fn default() -> Self {{")?;
            self.indent();
            {
                writeln!(self.write, "Self {{")?;
                self.indent();
                {
                    for field in &message.field {
                        let default_value: Cow<'static, str> =
                            if let Some(TypeOfIdent::Enum) = self.check_is_struct_or_enum(field) {
                                "std::convert::TryFrom::try_from(0i32)".into()
                            } else {
                                "Default::default()".into()
                            };
                        writeln!(
                            self.write,
                            "{name}: {value},",
                            name = to_var_name(&field.name),
                            value = default_value
                        )?;
                    }
                }
                self.unindent();
                writeln!(self.write, "}} // Self {{")?;
            }
            self.unindent();
            writeln!(self.write, "}} // fn default() -> Self {{")?;
        }
        self.unindent();
        writeln!(
            self.write,
            "}} // impl Default for {name} {{",
            name = native_type_name
        )?;

        // Deserialize
        writeln!(
            self.write,
            "impl ::puroro::Deserializable for {name} {{",
            name = native_type_name
        )?;
        self.indent();
        {
            writeln!(
                self.write,
                "fn from_bytes<I: Iterator<Item = std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {{"
            )?;
            self.indent();
            {
                writeln!(self.write, "todo!()")?;
            }
            self.unindent();
            writeln!(self.write, "}} // fn from_bytes() {{")?;
        }
        self.unindent();
        writeln!(
            self.write,
            "}} // impl ::puroro::Deserializable for {name} {{",
            name = native_type_name
        )?;

        Ok(())
    }

    fn gen_enum(&mut self, enume: &'b EnumDescriptorProto) -> Result<()> {
        self.type_of_idents.insert(
            MaybeFullyQualifiedTypeName::new(self.package.clone(), &enume.name),
            TypeOfIdent::Enum,
        );

        writeln!(
            self.write,
            "pub enum {name} {{",
            name = to_type_name(&enume.name)
        )?;
        self.indent();

        for value in &enume.value {
            self.gen_enum_value(value)?;
        }

        self.unindent();
        writeln!(
            self.write,
            "}} // enum {name} {{",
            name = to_type_name(&enume.name)
        )?;

        Ok(())
    }

    fn gen_enum_value(&mut self, value: &'b EnumValueDescriptorProto) -> Result<()> {
        writeln!(
            self.write,
            "{name},",
            name = to_enum_value_name(&value.name)
        )?;
        Ok(())
    }

    fn gen_file(&mut self, fdp: &'b FileDescriptorProto) -> Result<()> {
        let package_iter = fdp.package.split('.').filter(|s| !s.is_empty());
        self.package = package_iter.clone().collect();

        for package in package_iter.clone() {
            writeln!(self.write, "mod {name} {{", name = to_module_name(package))?;
        }

        for enume in &fdp.enum_type {
            self.gen_enum(enume)?;
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
    let mut context = StructGenerator::new(&mut output);
    for fdp in &cgreq.proto_file {
        context.gen_file(fdp)?;
    }
    Ok(output)
}
