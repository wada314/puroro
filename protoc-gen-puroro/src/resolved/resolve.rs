//! Build a [`FileSet`](super::FileSet) inside a caller-owned [`Arena`](super::Arena).

use super::{Arena, Enum, EnumValue, Field, File, FileSet, Message, Oneof, TypeItem, TypeRef};
use crate::descriptor::{EnumDesc, FieldDesc, FieldType, MessageDesc, ProtoFile, ProtoFqn};
use crate::error::{Error, Result};
use ::std::cell::OnceCell;
use ::std::collections::HashMap;

/// Resolve descriptor type names into an arena-local [`FileSet`].
///
/// Plugin metadata ([`crate::descriptor::CodegenMeta`]) is intentionally unused
/// here — keep it beside the returned `FileSet` and pass both by reference.
pub fn resolve<'a>(arena: &'a Arena, proto_files: &[ProtoFile]) -> Result<FileSet<'a>> {
    let mut types_by_fqn: HashMap<ProtoFqn, TypeItem<'a>> = HashMap::new();
    let mut files = Vec::with_capacity(proto_files.len());

    // Pass 1: allocate every message/enum node and register by FQN.
    for proto in proto_files {
        files.push(register_file(arena, proto, &mut types_by_fqn)?);
    }

    // Pass 2: fill fields with resolved TypeRefs.
    for proto in proto_files {
        fill_file_fields(proto, &types_by_fqn)?;
    }

    Ok(FileSet {
        files,
        types_by_fqn,
    })
}

fn register_file<'a>(
    arena: &'a Arena,
    proto: &ProtoFile,
    types_by_fqn: &mut HashMap<ProtoFqn, TypeItem<'a>>,
) -> Result<&'a File<'a>> {
    let package_fqn = ProtoFqn::from_package(&proto.package);

    let mut messages = Vec::with_capacity(proto.messages.len());
    for desc in &proto.messages {
        messages.push(register_message(arena, desc, &package_fqn, types_by_fqn)?);
    }

    let mut enums = Vec::with_capacity(proto.enums.len());
    for desc in &proto.enums {
        enums.push(register_enum(arena, desc, &package_fqn, types_by_fqn)?);
    }

    Ok(arena.alloc(File {
        name: proto.name.clone(),
        package: proto.package.clone(),
        dependency: proto.dependency.clone(),
        messages,
        enums,
    }))
}

fn register_message<'a>(
    arena: &'a Arena,
    desc: &MessageDesc,
    // FQN of the enclosing package (possibly ".") or parent message.
    parent_fqn: &ProtoFqn,
    types_by_fqn: &mut HashMap<ProtoFqn, TypeItem<'a>>,
) -> Result<&'a Message<'a>> {
    let fqn = parent_fqn.append(&desc.name);

    if types_by_fqn.contains_key(&fqn) {
        return Err(Error::Codegen(format!(
            "duplicate type FQN `{fqn}` while resolving schema"
        )));
    }

    // Build children first so `nested_*` can be plain Vecs on the parent.
    let mut nested_messages = Vec::with_capacity(desc.nested_messages.len());
    for nested in &desc.nested_messages {
        nested_messages.push(register_message(arena, nested, &fqn, types_by_fqn)?);
    }

    let mut nested_enums = Vec::with_capacity(desc.nested_enums.len());
    for nested in &desc.nested_enums {
        nested_enums.push(register_enum(arena, nested, &fqn, types_by_fqn)?);
    }

    let oneofs = desc
        .oneofs
        .iter()
        .map(|o| Oneof {
            name: o.name.clone(),
        })
        .collect();

    let message = arena.alloc(Message {
        name: desc.name.clone(),
        fqn: fqn.clone(),
        parent: OnceCell::new(),
        fields: OnceCell::new(),
        nested_messages,
        nested_enums,
        oneofs,
    });
    types_by_fqn.insert(fqn.clone(), TypeItem::Message(message));

    for child in &message.nested_messages {
        child
            .parent
            .set(message)
            .map_err(|_| Error::Codegen(format!("parent set twice for `{}`", child.fqn)))?;
    }
    for child in &message.nested_enums {
        child
            .parent
            .set(message)
            .map_err(|_| Error::Codegen(format!("parent set twice for `{}`", child.fqn)))?;
    }

    Ok(message)
}

fn register_enum<'a>(
    arena: &'a Arena,
    desc: &EnumDesc,
    // FQN of the enclosing package (possibly ".") or parent message.
    parent_fqn: &ProtoFqn,
    types_by_fqn: &mut HashMap<ProtoFqn, TypeItem<'a>>,
) -> Result<&'a Enum<'a>> {
    let fqn = parent_fqn.append(&desc.name);

    if types_by_fqn.contains_key(&fqn) {
        return Err(Error::Codegen(format!(
            "duplicate type FQN `{fqn}` while resolving schema"
        )));
    }

    let values = desc
        .values
        .iter()
        .map(|v| EnumValue {
            name: v.name.clone(),
            number: v.number,
        })
        .collect();

    let enum_ty = arena.alloc(Enum {
        name: desc.name.clone(),
        fqn: fqn.clone(),
        parent: OnceCell::new(),
        values,
    });
    types_by_fqn.insert(fqn, TypeItem::Enum(enum_ty));
    Ok(enum_ty)
}

fn fill_file_fields<'a>(
    proto: &ProtoFile,
    types_by_fqn: &HashMap<ProtoFqn, TypeItem<'a>>,
) -> Result<()> {
    let package_fqn = ProtoFqn::from_package(&proto.package);
    for desc in &proto.messages {
        fill_message_fields(desc, &package_fqn, types_by_fqn)?;
    }
    Ok(())
}

fn fill_message_fields<'a>(
    desc: &MessageDesc,
    parent_fqn: &ProtoFqn,
    types_by_fqn: &HashMap<ProtoFqn, TypeItem<'a>>,
) -> Result<()> {
    let fqn = parent_fqn.append(&desc.name);

    let TypeItem::Message(message) = types_by_fqn.get(&fqn).copied().ok_or_else(|| {
        Error::Codegen(format!(
            "internal error: message `{fqn}` missing after registration"
        ))
    })?
    else {
        return Err(Error::Codegen(format!(
            "internal error: FQN `{fqn}` registered as non-message"
        )));
    };

    let mut fields = Vec::with_capacity(desc.fields.len());
    for field in &desc.fields {
        fields.push(resolve_field(field, &message.fqn, types_by_fqn)?);
    }
    message
        .fields
        .set(fields)
        .map_err(|_| Error::Codegen(format!("fields set twice for `{fqn}`")))?;

    for nested in &desc.nested_messages {
        fill_message_fields(nested, &fqn, types_by_fqn)?;
    }
    Ok(())
}

fn resolve_field<'a>(
    field: &FieldDesc,
    owner_fqn: &ProtoFqn,
    types_by_fqn: &HashMap<ProtoFqn, TypeItem<'a>>,
) -> Result<Field<'a>> {
    let type_ref = match field.type_ {
        FieldType::Message | FieldType::Group => {
            let type_name = field.type_name.as_ref().ok_or_else(|| {
                Error::Codegen(format!(
                    "field `{owner_fqn}.{}` has message type but no type_name",
                    field.name
                ))
            })?;
            let target = lookup_message(type_name, types_by_fqn).ok_or_else(|| {
                Error::Codegen(format!(
                    "field `{owner_fqn}.{}` references unknown message type `{type_name}`",
                    field.name
                ))
            })?;
            TypeRef::Message(target)
        }
        FieldType::Enum => {
            let type_name = field.type_name.as_ref().ok_or_else(|| {
                Error::Codegen(format!(
                    "field `{owner_fqn}.{}` has enum type but no type_name",
                    field.name
                ))
            })?;
            let target = lookup_enum(type_name, types_by_fqn).ok_or_else(|| {
                Error::Codegen(format!(
                    "field `{owner_fqn}.{}` references unknown enum type `{type_name}`",
                    field.name
                ))
            })?;
            TypeRef::Enum(target)
        }
        scalar => TypeRef::from_scalar(scalar).ok_or_else(|| {
            Error::Codegen(format!(
                "field `{owner_fqn}.{}` has unexpected type {:?}",
                field.name, scalar
            ))
        })?,
    };

    Ok(Field {
        name: field.name.clone(),
        number: field.number,
        label: field.label,
        type_ref,
        oneof_index: field.oneof_index,
        proto3_optional: field.proto3_optional,
    })
}

fn lookup_message<'a>(
    type_name: &ProtoFqn,
    types_by_fqn: &HashMap<ProtoFqn, TypeItem<'a>>,
) -> Option<&'a Message<'a>> {
    match types_by_fqn.get(type_name)? {
        TypeItem::Message(m) => Some(*m),
        TypeItem::Enum(_) => None,
    }
}

fn lookup_enum<'a>(
    type_name: &ProtoFqn,
    types_by_fqn: &HashMap<ProtoFqn, TypeItem<'a>>,
) -> Option<&'a Enum<'a>> {
    match types_by_fqn.get(type_name)? {
        TypeItem::Enum(e) => Some(*e),
        TypeItem::Message(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::descriptor::{
        EnumDesc, EnumValueDesc, FieldDesc, FieldLabel, FieldType, MessageDesc, ProtoFile, ProtoFqn,
    };
    use ::std::ptr;

    fn empty_msg(name: &str) -> MessageDesc {
        MessageDesc {
            name: name.into(),
            fields: vec![],
            nested_messages: vec![],
            nested_enums: vec![],
            oneofs: vec![],
        }
    }

    #[test]
    fn resolve_empty_message_under_package() {
        let arena = Arena::new();
        let files = [ProtoFile {
            name: "a.proto".into(),
            package: "example.v1".into(),
            dependency: vec![],
            messages: vec![empty_msg("Empty")],
            enums: vec![],
        }];
        let file_set = resolve(&arena, &files).unwrap();
        let msg = file_set.message(".example.v1.Empty").unwrap();
        assert_eq!(msg.name, "Empty");
        assert!(msg.fields.get().unwrap().is_empty());
        assert!(msg.parent.get().is_none());
    }

    #[test]
    fn resolve_message_field_to_peer() {
        let arena = Arena::new();
        let files = [ProtoFile {
            name: "a.proto".into(),
            package: "example".into(),
            dependency: vec![],
            messages: vec![
                empty_msg("Address"),
                MessageDesc {
                    name: "Task".into(),
                    fields: vec![FieldDesc {
                        name: "assignee".into(),
                        number: 1,
                        label: FieldLabel::Optional,
                        type_: FieldType::Message,
                        type_name: Some(ProtoFqn::parse(".example.Address")),
                        oneof_index: None,
                        proto3_optional: true,
                    }],
                    nested_messages: vec![],
                    nested_enums: vec![],
                    oneofs: vec![],
                },
            ],
            enums: vec![],
        }];
        let file_set = resolve(&arena, &files).unwrap();
        let task = file_set.message(".example.Task").unwrap();
        let address = file_set.message(".example.Address").unwrap();
        let fields = task.fields.get().unwrap();
        assert_eq!(fields.len(), 1);
        assert!(ptr::eq(fields[0].type_ref.as_message().unwrap(), address));
    }

    #[test]
    fn resolve_nested_message_and_parent() {
        let arena = Arena::new();
        let files = [ProtoFile {
            name: "a.proto".into(),
            package: String::new(),
            dependency: vec![],
            messages: vec![MessageDesc {
                name: "Outer".into(),
                fields: vec![],
                nested_messages: vec![empty_msg("Inner")],
                nested_enums: vec![],
                oneofs: vec![],
            }],
            enums: vec![],
        }];
        let file_set = resolve(&arena, &files).unwrap();
        let outer = file_set.message(".Outer").unwrap();
        let inner = file_set.message(".Outer.Inner").unwrap();
        assert!(ptr::eq(*inner.parent.get().unwrap(), outer));
        assert!(ptr::eq(outer.nested_messages[0], inner));
    }

    #[test]
    fn resolve_enum_field() {
        let arena = Arena::new();
        let files = [ProtoFile {
            name: "a.proto".into(),
            package: "example".into(),
            dependency: vec![],
            messages: vec![MessageDesc {
                name: "Task".into(),
                fields: vec![FieldDesc {
                    name: "status".into(),
                    number: 1,
                    label: FieldLabel::Optional,
                    type_: FieldType::Enum,
                    type_name: Some(ProtoFqn::parse(".example.Status")),
                    oneof_index: None,
                    proto3_optional: false,
                }],
                nested_messages: vec![],
                nested_enums: vec![],
                oneofs: vec![],
            }],
            enums: vec![EnumDesc {
                name: "Status".into(),
                values: vec![EnumValueDesc {
                    name: "STATUS_UNSPECIFIED".into(),
                    number: 0,
                }],
            }],
        }];
        let file_set = resolve(&arena, &files).unwrap();
        let task = file_set.message(".example.Task").unwrap();
        let status = file_set.enum_ty(".example.Status").unwrap();
        assert!(ptr::eq(
            task.fields.get().unwrap()[0].type_ref.as_enum().unwrap(),
            status
        ));
    }

    #[test]
    fn missing_type_name_errors() {
        let arena = Arena::new();
        let files = [ProtoFile {
            name: "a.proto".into(),
            package: String::new(),
            dependency: vec![],
            messages: vec![MessageDesc {
                name: "Task".into(),
                fields: vec![FieldDesc {
                    name: "assignee".into(),
                    number: 1,
                    label: FieldLabel::Optional,
                    type_: FieldType::Message,
                    type_name: Some(ProtoFqn::parse(".Missing")),
                    oneof_index: None,
                    proto3_optional: false,
                }],
                nested_messages: vec![],
                nested_enums: vec![],
                oneofs: vec![],
            }],
            enums: vec![],
        }];
        let err = resolve(&arena, &files).unwrap_err();
        assert!(err.to_string().contains("unknown message type"));
    }

    #[test]
    fn mutual_message_refs() {
        let arena = Arena::new();
        let files = [ProtoFile {
            name: "a.proto".into(),
            package: String::new(),
            dependency: vec![],
            messages: vec![
                MessageDesc {
                    name: "A".into(),
                    fields: vec![FieldDesc {
                        name: "b".into(),
                        number: 1,
                        label: FieldLabel::Optional,
                        type_: FieldType::Message,
                        type_name: Some(ProtoFqn::parse(".B")),
                        oneof_index: None,
                        proto3_optional: false,
                    }],
                    nested_messages: vec![],
                    nested_enums: vec![],
                    oneofs: vec![],
                },
                MessageDesc {
                    name: "B".into(),
                    fields: vec![FieldDesc {
                        name: "a".into(),
                        number: 1,
                        label: FieldLabel::Optional,
                        type_: FieldType::Message,
                        type_name: Some(ProtoFqn::parse(".A")),
                        oneof_index: None,
                        proto3_optional: false,
                    }],
                    nested_messages: vec![],
                    nested_enums: vec![],
                    oneofs: vec![],
                },
            ],
            enums: vec![],
        }];
        let file_set = resolve(&arena, &files).unwrap();
        let a = file_set.message(".A").unwrap();
        let b = file_set.message(".B").unwrap();
        assert!(ptr::eq(
            a.fields.get().unwrap()[0].type_ref.as_message().unwrap(),
            b
        ));
        assert!(ptr::eq(
            b.fields.get().unwrap()[0].type_ref.as_message().unwrap(),
            a
        ));
    }
}
