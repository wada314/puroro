use crate::generators::shared::*;
use crate::generators::utils::*;
use crate::plugin::*;
use crate::{PuroroError, Result};
use itertools::Itertools;
use std::{borrow::Cow, collections::HashMap, fmt::Write};

#[derive(Debug, Clone)]
enum TypeOfIdent {
    Message,
    Enum,
}

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

    fn gen_struct<'w, W: Write>(
        &mut self,
        file: &mut FileGeneratorContext<'w, 'p, W>,
    ) -> Result<()> {
        todo!()
    }
}

struct EnumGenerator<'p /* 'p: Lifetime for the input protobuf */> {
    enume: &'p EnumDescriptorProto,
}
impl<'p> EnumGenerator<'p> {
    fn new(enume_proto: &'p EnumDescriptorProto) -> Self {
        Self { enume: enume_proto }
    }

    fn gen_enum<'w, W: Write>(&mut self, file: &mut FileGeneratorContext<'w, 'p, W>) -> Result<()> {
        let native_type_name = to_type_name(&self.enume.name);
        writeln!(file.writer(), "pub enum {name} {{", name = native_type_name,)?;
        file.indent(|file| {
            for value in &self.enume.value {
                let name = to_enum_value_name(&value.name);
                writeln!(
                    file.writer(),
                    "{name} = {number}",
                    name = name,
                    number = value.number
                )?;
            }
            Ok(())
        })?;
        writeln!(
            file.writer(),
            "}} // pub enum {name} {{",
            name = native_type_name,
        )?;
        Ok(())
    }
}

struct StructGeneratorContext<'w, 'p, W: Write> {
    write: Indentor<'w, W>,
    type_of_idents: HashMap<FullyQualifiedTypeName<'p>, TypeOfIdent>,

    package: Vec<&'p str>,
    path_to_package_root: String,
}
impl<'w, 'p, W: Write> StructGeneratorContext<'w, 'p, W> {
    fn new(write: &'w mut W) -> Self {
        Self {
            write: Indentor::new(write),
            type_of_idents: HashMap::new(),
            package: Vec::new(),
            path_to_package_root: "".into(),
        }
    }

    fn indent(&mut self) {
        self.write.indent()
    }
    fn unindent(&mut self) {
        self.write.unindent()
    }

    fn post_update_package(&mut self) {
        if self.package.is_empty() {
            self.path_to_package_root = "self".into();
        } else {
            let supers = std::iter::repeat("super").take(self.package.len());
            self.path_to_package_root = Itertools::intersperse(supers, "::").collect::<String>();
        }
    }
    fn update_package(&mut self, package: Vec<&'p str>) {
        self.package = package;
        self.post_update_package();
    }
    fn push_package(&mut self, p: &'p str) {
        self.package.push(p);
        self.post_update_package();
    }
    fn pop_package(&mut self) {
        self.package.pop();
        self.post_update_package();
    }

    fn path_to_package_root(&self) -> &str {
        &self.path_to_package_root
    }

    fn search_for_idents_type(&self, typename: &str) -> Option<TypeOfIdent> {
        let mfqtn = MaybeFullyQualifiedTypeName::from_maybe_fq_typename(typename);
        if let Some(fqtn) = mfqtn.try_to_absolute() {
            return self.type_of_idents.get(&fqtn).cloned();
        }
        let mut package = self.package.clone();
        loop {
            let fqtn = mfqtn.with_package(package.clone());
            if let Some(found) = self.type_of_idents.get(&fqtn) {
                return Some(found.clone());
            }
            if package.pop().is_none() {
                break;
            }
        }
        None
    }
    fn check_is_struct_or_enum(&self, field: &FieldDescriptorProto) -> Option<TypeOfIdent> {
        match field.type_ {
            Ok(FieldDescriptorProto_Type::TYPE_ENUM) => Some(TypeOfIdent::Enum),
            Ok(FieldDescriptorProto_Type::TYPE_MESSAGE) => Some(TypeOfIdent::Message),
            Err(0) => self.search_for_idents_type(&field.type_name),
            _ => None,
        }
    }

    fn gen_field(&mut self, field: &'p FieldDescriptorProto) -> Result<()> {
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

            Ok(FieldDescriptorProto_Type::TYPE_MESSAGE) => {
                MaybeFullyQualifiedTypeName::from_maybe_fq_typename(&field.type_name)
                    .to_native_maybe_qualified_typename(&self.path_to_package_root())
                    .into()
            }

            Ok(FieldDescriptorProto_Type::TYPE_ENUM) => {
                let name = MaybeFullyQualifiedTypeName::from_maybe_fq_typename(&field.type_name)
                    .to_native_maybe_qualified_typename(&self.path_to_package_root());
                format!("Result<{name}, i32>", name = name).into()
            }

            Ok(FieldDescriptorProto_Type::TYPE_BYTES) => "Vec<u8>".into(),

            _ => {
                if !field.type_name.is_empty() {
                    return Err(PuroroError::UnexpectedFieldType);
                } else {
                    MaybeFullyQualifiedTypeName::from_maybe_fq_typename(&field.type_name)
                        .to_native_maybe_qualified_typename(&self.path_to_package_root())
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

    fn gen_message(&mut self, message: &'p DescriptorProto) -> Result<()> {
        let native_type_name = to_type_name(&message.name);

        self.type_of_idents.insert(
            FullyQualifiedTypeName::new(self.package.clone(), &message.name),
            TypeOfIdent::Message,
        );
        // Check sub-messages and sub-enums first
        if !message.nested_type.is_empty() || !message.enum_type.is_empty() {
            writeln!(
                self.write,
                "pub mod {name} {{",
                name = to_module_name(&message.name)
            )?;
            self.indent();
            self.push_package(&message.name);
            for submessage in &message.nested_type {
                self.gen_message(submessage)?;
            }
            for enume in &message.enum_type {
                self.gen_enum(enume)?;
            }
            self.pop_package();

            self.unindent();
            writeln!(
                self.write,
                "}} // pub mod {name} {{",
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
                        let default_value: Cow<'static, str> = if let Some(TypeOfIdent::Enum) =
                            self.check_is_struct_or_enum(field)
                        {
                            let native_enum_name =
                                MaybeFullyQualifiedTypeName::from_maybe_fq_typename(
                                    &field.type_name,
                                )
                                .to_native_maybe_qualified_typename(self.path_to_package_root());
                            format!(
                                "<{name} as std::convert::TryFrom<i32>>::try_from(0i32)",
                                name = native_enum_name
                            )
                            .into()
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

    fn gen_enum(&mut self, enume: &'p EnumDescriptorProto) -> Result<()> {
        self.type_of_idents.insert(
            FullyQualifiedTypeName::new(self.package.clone(), &enume.name),
            TypeOfIdent::Enum,
        );
        let native_type_name = to_type_name(&enume.name);

        // enum body
        writeln!(self.write, "pub enum {name} {{", name = native_type_name,)?;
        self.indent();
        {
            for value in &enume.value {
                writeln!(
                    self.write,
                    "{name} = {number},",
                    name = to_enum_value_name(&value.name),
                    number = value.number,
                )?;
            }
        }
        self.unindent();
        writeln!(self.write, "}} // enum {name} {{", name = native_type_name,)?;

        // TryFrom<i32>
        writeln!(
            self.write,
            "impl std::convert::TryFrom<i32> for {name} {{",
            name = native_type_name,
        )?;
        self.indent();
        {
            writeln!(
                self.write,
                "type Error = i32;\n\
                 fn try_from(value: i32) -> Result<Self, Self::Error> {{"
            )?;
            self.indent();
            {
                writeln!(self.write, "match value {{")?;
                self.indent();
                for value in &enume.value {
                    writeln!(
                        self.write,
                        "{number} => Ok(Self::{name}),",
                        name = to_enum_value_name(&value.name),
                        number = value.number
                    )?;
                }
                writeln!(self.write, "x => Err(x),")?;
                self.unindent();
                writeln!(self.write, "}} // match value {{")?;
            }
            self.unindent();
            writeln!(
                self.write,
                "}} // fn try_from(value: T) -> Result<Self, Self::Error> {{"
            )?;
        }
        self.unindent();
        writeln!(
            self.write,
            "}} // impl std::convert::TryFrom<i32> for {name} {{",
            name = native_type_name,
        )?;

        Ok(())
    }

    fn gen_file(&mut self, fdp: &'p FileDescriptorProto) -> Result<()> {
        let package_iter = fdp.package.split('.').filter(|s| !s.is_empty());
        self.update_package(package_iter.clone().collect());

        for package in package_iter.clone() {
            writeln!(
                self.write,
                "pub mod {name} {{",
                name = to_module_name(package)
            )?;
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
                "}} // pub mod {name} {{",
                name = to_module_name(package)
            )?;
        }

        Ok(())
    }
}

pub(crate) fn generate_simple(cgreq: &CodeGeneratorRequest) -> Result<String> {
    let mut output = String::new();
    let mut context = StructGeneratorContext::new(&mut output);
    for fdp in &cgreq.proto_file {
        context.gen_file(fdp)?;
    }
    Ok(output)
}
