use crate::plugin::*;
use crate::utils::*;
use crate::{PuroroError, Result};
use std::fmt::Write;

fn generate_field<W: Write>(o: &mut Indentor<W>, field: &FieldDescriptorProto) -> Result<()> {
    enum FieldType {
        SimpleGetSet { native_type: String },
        IteratorGetSet { native_item_type: String },
        ByReferenceGetSet { native_item_type: String },
    }
    let field_type = match field.type_ {
        Ok(FieldDescriptorProto_Type::TYPE_DOUBLE) => FieldType::SimpleGetSet {
            native_type: "f64".to_string(),
        },
        Ok(FieldDescriptorProto_Type::TYPE_FLOAT) => FieldType::SimpleGetSet {
            native_type: "f32".to_string(),
        },
        Ok(FieldDescriptorProto_Type::TYPE_INT64) => FieldType::SimpleGetSet {
            native_type: "i64".to_string(),
        },
        Ok(FieldDescriptorProto_Type::TYPE_UINT64) => FieldType::SimpleGetSet {
            native_type: "u64".to_string(),
        },
        Ok(FieldDescriptorProto_Type::TYPE_INT32) => FieldType::SimpleGetSet {
            native_type: "i32".to_string(),
        },
        Ok(FieldDescriptorProto_Type::TYPE_FIXED64) => FieldType::SimpleGetSet {
            native_type: "u64".to_string(),
        },
        Ok(FieldDescriptorProto_Type::TYPE_FIXED32) => FieldType::SimpleGetSet {
            native_type: "u32".to_string(),
        },
        Ok(FieldDescriptorProto_Type::TYPE_BOOL) => FieldType::SimpleGetSet {
            native_type: "bool".to_string(),
        },
        Ok(FieldDescriptorProto_Type::TYPE_STRING) => FieldType::IteratorGetSet {
            native_item_type: "Self::CharsIterator".to_string(),
        },
        Ok(FieldDescriptorProto_Type::TYPE_MESSAGE) => FieldType::ByReferenceGetSet {
            native_item_type: snake_case_to_camel_case(&field.type_name),
        },
        Ok(FieldDescriptorProto_Type::TYPE_BYTES) => FieldType::IteratorGetSet {
            native_item_type: "Self::BytesIterator".to_string(),
        },
        Ok(FieldDescriptorProto_Type::TYPE_UINT32) => FieldType::SimpleGetSet {
            native_type: "u32".to_string(),
        },
        Ok(FieldDescriptorProto_Type::TYPE_ENUM) => FieldType::SimpleGetSet {
            native_type: format!(
                "std::result::Result<{}, i32>",
                snake_case_to_camel_case(&field.type_name)
            ),
        },
        Ok(FieldDescriptorProto_Type::TYPE_SFIXED32) => FieldType::SimpleGetSet {
            native_type: "i32".to_string(),
        },
        Ok(FieldDescriptorProto_Type::TYPE_SFIXED64) => FieldType::SimpleGetSet {
            native_type: "i64".to_string(),
        },
        Ok(FieldDescriptorProto_Type::TYPE_SINT32) => FieldType::SimpleGetSet {
            native_type: "i32".to_string(),
        },
        Ok(FieldDescriptorProto_Type::TYPE_SINT64) => FieldType::SimpleGetSet {
            native_type: "i64".to_string(),
        },
        _ => return Err(PuroroError::UnexpectedFieldType),
    };
    Ok(())
}

fn generate_traits_for_descriptor<W: Write>(
    o: &mut Indentor<W>,
    desc: &DescriptorProto,
) -> Result<()> {
    write!(o, "trait {name} {{\n", name = desc.name)?;
    o.indent();

    // If any String field exists, define a CharsIterator type.
    if desc
        .field
        .iter()
        .any(|field| matches!(field.type_, Ok(FieldDescriptorProto_Type::TYPE_STRING)))
    {
        write!(o, "type CharsIterator: Iterator<Item=Result<char>>;\n")?;
    }
    // Same for Bytes
    if desc
        .field
        .iter()
        .any(|field| matches!(field.type_, Ok(FieldDescriptorProto_Type::TYPE_BYTES)))
    {
        write!(o, "type BytesIterator: Iterator<Item=Result<u8>>;\n")?;
    }

    for field in &desc.field {
        generate_field(o, field)?;
    }
    write!(o, "}}\n")?; // close trait
    o.unindent();

    Ok(())
}

pub(crate) fn generate_traits(cgreq: &CodeGeneratorRequest) -> Result<String> {
    let descriptors = cgreq
        .proto_file
        .iter()
        .map(|fdp| &fdp.message_type)
        .flatten();
    let mut output = String::new();
    let mut indentor = Indentor::new(&mut output);
    for desc in descriptors {
        generate_traits_for_descriptor(&mut indentor, desc)?;
    }
    Ok(output)
}
