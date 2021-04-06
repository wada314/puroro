use crate::plugin::*;
use crate::{PuroroError, Result};
use ::itertools::Itertools;
use std::fmt::Write;

struct Indentor<'a, W: Write> {
    write: &'a mut W,
    indent_next: bool,
    level: usize,
}
impl<'a, W: Write> Indentor<'a, W> {
    fn new(write: &'a mut W) -> Self {
        Self {
            write,
            indent_next: false,
            level: 0,
        }
    }
    fn indent(&mut self) {
        self.level += 1;
    }
    fn unindent(&mut self) {
        assert_ne!(0, self.level, "unindenting too much");
        self.level -= 1;
    }
}
impl<'a, W: Write> Write for Indentor<'a, W> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        for c in s.chars() {
            self.write_char(c)?;
        }
        Ok(())
    }
    fn write_char(&mut self, c: char) -> std::fmt::Result {
        if self.indent_next {
            self.indent_next = false;
            self.write.write_str(
                &std::iter::repeat(' ')
                    .take(4 * self.level)
                    .collect::<String>(),
            )?;
        }
        if c == '\n' {
            self.indent_next = true;
        }
        self.write.write_char(c)
    }
}

fn snake_case_to_camel_case(input: &str) -> String {
    input
        .chars()
        .scan(true, |capitalize_next, c| {
            if c == '_' {
                *capitalize_next = true;
                Some(c)
            } else {
                if *capitalize_next {
                    *capitalize_next = false;
                    Some(c.to_ascii_uppercase())
                } else {
                    Some(c)
                }
            }
        })
        .collect()
}

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
    // For all kinds of messages
    desc.field
        .iter()
        .filter_map(|field| {
            if matches!(field.type_, Ok(FieldDescriptorProto_Type::TYPE_MESSAGE)) {
                Some(&field.type_name)
            } else {
                None
            }
        })
        .unique()
        .try_for_each(|msgname| {
            todo!()
            /*write!(
                o,
                "type {name}Iterator: Iterator<Item=Result<Option<&'a {camel_name}>>>;",
            )*/
        })?;

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
