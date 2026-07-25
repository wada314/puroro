//! Build a [`FileSet`](super::FileSet) inside a caller-owned [`Arena`](super::Arena).

use super::{
    Arena, Enum, EnumValue, Field, FieldOccurrence, File, FileSet, Message, Oneof,
    SingularPresence, TypeItem, TypeRef,
};
use crate::descriptor::features::{
    EnumType, FeatureSet, FieldPresence, MessageEncoding, RepeatedFieldEncoding, Utf8Validation,
};
use crate::descriptor::{
    EnumDesc, FieldDesc, FieldLabel, FieldType, MessageDesc, ProtoFile, ProtoFqn, Syntax,
};
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
        messages.push(register_message(
            arena,
            desc,
            &package_fqn,
            proto.syntax,
            proto.features,
            types_by_fqn,
        )?);
    }

    let mut enums = Vec::with_capacity(proto.enums.len());
    for desc in &proto.enums {
        enums.push(register_enum(
            arena,
            desc,
            &package_fqn,
            proto.syntax,
            proto.features,
            types_by_fqn,
        )?);
    }

    Ok(arena.alloc(File {
        name: proto.name.clone(),
        package: proto.package.clone(),
        syntax: proto.syntax,
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
    syntax: Syntax,
    file_features: FeatureSet,
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
        nested_messages.push(register_message(
            arena,
            nested,
            &fqn,
            syntax,
            file_features,
            types_by_fqn,
        )?);
    }

    let mut nested_enums = Vec::with_capacity(desc.nested_enums.len());
    for nested in &desc.nested_enums {
        nested_enums.push(register_enum(
            arena,
            nested,
            &fqn,
            syntax,
            file_features,
            types_by_fqn,
        )?);
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
    syntax: Syntax,
    file_features: FeatureSet,
    types_by_fqn: &mut HashMap<ProtoFqn, TypeItem<'a>>,
) -> Result<&'a Enum<'a>> {
    let fqn = parent_fqn.append(&desc.name);

    if types_by_fqn.contains_key(&fqn) {
        return Err(Error::Codegen(format!(
            "duplicate type FQN `{fqn}` while resolving schema"
        )));
    }

    desc.features
        .reject_unimplemented_overrides(&format!("enum `{fqn}`"));

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
        openness: resolve_enum_openness(syntax, file_features, desc.features),
        values,
    });
    types_by_fqn.insert(fqn, TypeItem::Enum(enum_ty));
    Ok(enum_ty)
}

fn resolve_enum_openness(
    syntax: Syntax,
    file_features: FeatureSet,
    enum_features: FeatureSet,
) -> EnumType {
    match syntax {
        // Historical defaults (editions features are not used for proto2/proto3).
        Syntax::Proto2 => EnumType::Closed,
        Syntax::Proto3 => EnumType::Open,
        Syntax::Editions(edition) => FeatureSet::defaults_for_edition(edition)
            .overlay(&file_features)
            .overlay(&enum_features)
            .enum_type
            .expect("edition defaults always set enum_type"),
    }
}

fn fill_file_fields<'a>(
    proto: &ProtoFile,
    types_by_fqn: &HashMap<ProtoFqn, TypeItem<'a>>,
) -> Result<()> {
    if let Syntax::Editions(edition) = proto.syntax {
        proto
            .features
            .reject_unimplemented_overrides(&format!("file `{}`", proto.name));
        let file_features = FeatureSet::defaults_for_edition(edition).overlay(&proto.features);
        file_features.apply_or_trap_for_file();
    }

    let package_fqn = ProtoFqn::from_package(&proto.package);
    for desc in &proto.messages {
        fill_message_fields(
            desc,
            &package_fqn,
            proto.syntax,
            proto.features,
            types_by_fqn,
        )?;
    }
    Ok(())
}

fn fill_message_fields<'a>(
    desc: &MessageDesc,
    parent_fqn: &ProtoFqn,
    syntax: Syntax,
    file_features: FeatureSet,
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
        fields.push(resolve_field(
            field,
            &message.fqn,
            syntax,
            file_features,
            types_by_fqn,
        )?);
    }
    message
        .fields
        .set(fields)
        .map_err(|_| Error::Codegen(format!("fields set twice for `{fqn}`")))?;

    for nested in &desc.nested_messages {
        fill_message_fields(nested, &fqn, syntax, file_features, types_by_fqn)?;
    }
    Ok(())
}

fn resolve_field<'a>(
    field: &FieldDesc,
    owner_fqn: &ProtoFqn,
    syntax: Syntax,
    file_features: FeatureSet,
    types_by_fqn: &HashMap<ProtoFqn, TypeItem<'a>>,
) -> Result<Field<'a>> {
    if field.type_ == FieldType::Group {
        return Err(Error::Codegen(format!(
            "field `{owner_fqn}.{}`: deprecated group fields are not supported",
            field.name
        )));
    }

    let editions_features = match syntax {
        Syntax::Editions(edition) => {
            field
                .features
                .reject_unimplemented_overrides(&format!("field `{owner_fqn}.{}`", field.name));
            Some(
                FeatureSet::defaults_for_edition(edition)
                    .overlay(&file_features)
                    .overlay(&field.features),
            )
        }
        Syntax::Proto2 | Syntax::Proto3 => None,
    };

    if matches!(field.type_, FieldType::Message)
        && editions_features
            .map(|f| f.message_encoding == Some(MessageEncoding::Delimited))
            .unwrap_or(false)
    {
        return Err(Error::Codegen(format!(
            "field `{owner_fqn}.{}`: features.message_encoding=DELIMITED is not supported",
            field.name
        )));
    }

    let type_ref = match field.type_ {
        FieldType::Message => {
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
        FieldType::Group => unreachable!("rejected above"),
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
        occurrence: resolve_occurrence(field, syntax, editions_features.as_ref()),
        type_ref,
        oneof_index: field.oneof_index,
        utf8_validation: resolve_utf8_validation(field, syntax, editions_features.as_ref()),
    })
}

fn resolve_occurrence(
    field: &FieldDesc,
    syntax: Syntax,
    editions_features: Option<&FeatureSet>,
) -> FieldOccurrence {
    if field.label == FieldLabel::Repeated {
        let encoding = match syntax {
            Syntax::Editions(_) => editions_features
                .expect("Editions features computed above")
                .repeated_field_encoding
                .expect("edition defaults always set repeated_field_encoding"),
            Syntax::Proto3 => RepeatedFieldEncoding::Packed,
            Syntax::Proto2 => {
                if field.packed == Some(true) {
                    RepeatedFieldEncoding::Packed
                } else {
                    RepeatedFieldEncoding::Expanded
                }
            }
        };
        return FieldOccurrence::Repeated(encoding);
    }

    // Real oneof member (proto3 `optional` uses a synthetic oneof + proto3_optional).
    if field.oneof_index.is_some() && !field.proto3_optional {
        return FieldOccurrence::Singular(SingularPresence::Oneof);
    }

    if field.type_ == FieldType::Message {
        return FieldOccurrence::Singular(SingularPresence::Message);
    }

    if field.label == FieldLabel::Required {
        return FieldOccurrence::Singular(SingularPresence::LegacyRequired);
    }

    match syntax {
        Syntax::Proto2 => FieldOccurrence::Singular(SingularPresence::Explicit),
        Syntax::Proto3 => {
            if field.proto3_optional {
                FieldOccurrence::Singular(SingularPresence::Explicit)
            } else {
                FieldOccurrence::Singular(SingularPresence::Implicit)
            }
        }
        Syntax::Editions(_) => {
            let features = editions_features.expect("Editions features computed above");
            let feature = features
                .field_presence
                .expect("edition defaults always set field_presence");
            FieldOccurrence::Singular(match feature {
                FieldPresence::Explicit => SingularPresence::Explicit,
                FieldPresence::Implicit => SingularPresence::Implicit,
                FieldPresence::LegacyRequired => SingularPresence::LegacyRequired,
            })
        }
    }
}

fn resolve_utf8_validation(
    field: &FieldDesc,
    syntax: Syntax,
    editions_features: Option<&FeatureSet>,
) -> Option<Utf8Validation> {
    if !matches!(field.type_, FieldType::String | FieldType::Bytes) {
        return None;
    }
    Some(match syntax {
        Syntax::Editions(_) => editions_features
            .expect("Editions features computed above")
            .utf8_validation
            .expect("edition defaults always set utf8_validation"),
        Syntax::Proto2 | Syntax::Proto3 => Utf8Validation::Verify,
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
    use crate::descriptor::features::{EnumType, FeatureSet, FieldPresence, RepeatedFieldEncoding};
    use crate::descriptor::{
        Edition, EnumDesc, EnumValueDesc, FieldDesc, FieldLabel, FieldType, MessageDesc, OneofDesc,
        ProtoFile, ProtoFqn, Syntax,
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

    fn proto_file(
        package: &str,
        syntax: Syntax,
        messages: Vec<MessageDesc>,
        enums: Vec<EnumDesc>,
    ) -> ProtoFile {
        ProtoFile {
            name: "a.proto".into(),
            package: package.into(),
            syntax,
            features: FeatureSet::default(),
            dependency: vec![],
            messages,
            enums,
        }
    }

    fn scalar_field(
        name: &str,
        label: FieldLabel,
        oneof_index: Option<i32>,
        proto3_optional: bool,
    ) -> FieldDesc {
        FieldDesc {
            name: name.into(),
            number: 1,
            label,
            type_: FieldType::Int32,
            type_name: None,
            oneof_index,
            proto3_optional,
            packed: None,
            features: FeatureSet::default(),
        }
    }

    #[test]
    fn resolve_empty_message_under_package() {
        let arena = Arena::new();
        let files = [proto_file(
            "example.v1",
            Syntax::Proto3,
            vec![empty_msg("Empty")],
            vec![],
        )];
        let file_set = resolve(&arena, &files).unwrap();
        let msg = file_set
            .lookup(".example.v1.Empty")
            .unwrap()
            .as_message()
            .unwrap();
        assert_eq!(msg.name(), "Empty");
        assert!(msg.fields().next().is_none());
        assert!(msg.parent().is_none());
        assert_eq!(file_set.files().next().unwrap().syntax(), Syntax::Proto3);
    }

    #[test]
    fn resolve_message_field_to_peer() {
        let arena = Arena::new();
        let files = [proto_file(
            "example",
            Syntax::Proto3,
            vec![
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
                        packed: None,
                        features: FeatureSet::default(),
                    }],
                    nested_messages: vec![],
                    nested_enums: vec![],
                    oneofs: vec![],
                },
            ],
            vec![],
        )];
        let file_set = resolve(&arena, &files).unwrap();
        let task = file_set
            .lookup(".example.Task")
            .unwrap()
            .as_message()
            .unwrap();
        let address = file_set
            .lookup(".example.Address")
            .unwrap()
            .as_message()
            .unwrap();
        let field = task.fields().next().unwrap();
        assert!(task.fields().nth(1).is_none());
        assert!(ptr::eq(field.type_ref().as_message().unwrap(), address));
        assert_eq!(
            field.occurrence(),
            FieldOccurrence::Singular(SingularPresence::Message)
        );
    }

    #[test]
    fn resolve_nested_message_and_parent() {
        let arena = Arena::new();
        let files = [proto_file(
            "",
            Syntax::Proto3,
            vec![MessageDesc {
                name: "Outer".into(),
                fields: vec![],
                nested_messages: vec![empty_msg("Inner")],
                nested_enums: vec![],
                oneofs: vec![],
            }],
            vec![],
        )];
        let file_set = resolve(&arena, &files).unwrap();
        let outer = file_set.lookup(".Outer").unwrap().as_message().unwrap();
        let inner = file_set
            .lookup(".Outer.Inner")
            .unwrap()
            .as_message()
            .unwrap();
        assert!(ptr::eq(inner.parent().unwrap(), outer));
        assert!(ptr::eq(outer.nested_messages().next().unwrap(), inner));
    }

    #[test]
    fn resolve_enum_field() {
        let arena = Arena::new();
        let files = [proto_file(
            "example",
            Syntax::Proto3,
            vec![MessageDesc {
                name: "Task".into(),
                fields: vec![FieldDesc {
                    name: "status".into(),
                    number: 1,
                    label: FieldLabel::Optional,
                    type_: FieldType::Enum,
                    type_name: Some(ProtoFqn::parse(".example.Status")),
                    oneof_index: None,
                    proto3_optional: false,
                    packed: None,
                    features: FeatureSet::default(),
                }],
                nested_messages: vec![],
                nested_enums: vec![],
                oneofs: vec![],
            }],
            vec![EnumDesc {
                name: "Status".into(),
                values: vec![EnumValueDesc {
                    name: "STATUS_UNSPECIFIED".into(),
                    number: 0,
                }],
                features: FeatureSet::default(),
            }],
        )];
        let file_set = resolve(&arena, &files).unwrap();
        let task = file_set
            .lookup(".example.Task")
            .unwrap()
            .as_message()
            .unwrap();
        let status = file_set
            .lookup(".example.Status")
            .unwrap()
            .as_enum()
            .unwrap();
        assert_eq!(status.openness(), EnumType::Open);
        let field = task.fields().next().unwrap();
        assert!(ptr::eq(field.type_ref().as_enum().unwrap(), status));
        assert_eq!(
            field.occurrence(),
            FieldOccurrence::Singular(SingularPresence::Implicit)
        );
    }

    #[test]
    fn missing_type_name_errors() {
        let arena = Arena::new();
        let files = [proto_file(
            "",
            Syntax::Proto3,
            vec![MessageDesc {
                name: "Task".into(),
                fields: vec![FieldDesc {
                    name: "assignee".into(),
                    number: 1,
                    label: FieldLabel::Optional,
                    type_: FieldType::Message,
                    type_name: Some(ProtoFqn::parse(".Missing")),
                    oneof_index: None,
                    proto3_optional: false,
                    packed: None,
                    features: FeatureSet::default(),
                }],
                nested_messages: vec![],
                nested_enums: vec![],
                oneofs: vec![],
            }],
            vec![],
        )];
        let err = resolve(&arena, &files).unwrap_err();
        assert!(err.to_string().contains("unknown message type"));
    }

    #[test]
    fn mutual_message_refs() {
        let arena = Arena::new();
        let files = [proto_file(
            "",
            Syntax::Proto3,
            vec![
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
                        packed: None,
                        features: FeatureSet::default(),
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
                        packed: None,
                        features: FeatureSet::default(),
                    }],
                    nested_messages: vec![],
                    nested_enums: vec![],
                    oneofs: vec![],
                },
            ],
            vec![],
        )];
        let file_set = resolve(&arena, &files).unwrap();
        let a = file_set.lookup(".A").unwrap().as_message().unwrap();
        let b = file_set.lookup(".B").unwrap().as_message().unwrap();
        assert!(ptr::eq(
            a.fields().next().unwrap().type_ref().as_message().unwrap(),
            b
        ));
        assert!(ptr::eq(
            b.fields().next().unwrap().type_ref().as_message().unwrap(),
            a
        ));
    }

    #[test]
    fn singular_presence_matrix() {
        let arena = Arena::new();
        let files = [
            proto_file(
                "p3",
                Syntax::Proto3,
                vec![
                    empty_msg("Addr"),
                    MessageDesc {
                        name: "M".into(),
                        fields: vec![
                            scalar_field("implicit", FieldLabel::Optional, None, false),
                            FieldDesc {
                                name: "explicit".into(),
                                number: 2,
                                label: FieldLabel::Optional,
                                type_: FieldType::Int32,
                                type_name: None,
                                // Synthetic oneof for proto3 optional.
                                oneof_index: Some(0),
                                proto3_optional: true,
                                packed: None,
                                features: FeatureSet::default(),
                            },
                            FieldDesc {
                                name: "addr".into(),
                                number: 3,
                                label: FieldLabel::Optional,
                                type_: FieldType::Message,
                                type_name: Some(ProtoFqn::parse(".p3.Addr")),
                                oneof_index: None,
                                proto3_optional: false,
                                packed: None,
                                features: FeatureSet::default(),
                            },
                            FieldDesc {
                                name: "choice".into(),
                                number: 4,
                                label: FieldLabel::Optional,
                                type_: FieldType::Int32,
                                type_name: None,
                                oneof_index: Some(1),
                                proto3_optional: false,
                                packed: None,
                                features: FeatureSet::default(),
                            },
                            FieldDesc {
                                name: "tags".into(),
                                number: 5,
                                label: FieldLabel::Repeated,
                                type_: FieldType::Int32,
                                type_name: None,
                                oneof_index: None,
                                proto3_optional: false,
                                packed: None,
                                features: FeatureSet::default(),
                            },
                        ],
                        nested_messages: vec![],
                        nested_enums: vec![],
                        oneofs: vec![
                            OneofDesc {
                                name: "_explicit".into(),
                            },
                            OneofDesc {
                                name: "which".into(),
                            },
                        ],
                    },
                ],
                vec![],
            ),
            proto_file(
                "p2",
                Syntax::Proto2,
                vec![MessageDesc {
                    name: "M".into(),
                    fields: vec![
                        scalar_field("optional", FieldLabel::Optional, None, false),
                        scalar_field("required", FieldLabel::Required, None, false),
                    ],
                    nested_messages: vec![],
                    nested_enums: vec![],
                    oneofs: vec![],
                }],
                vec![],
            ),
        ];
        let file_set = resolve(&arena, &files).unwrap();

        let p3 = file_set.lookup(".p3.M").unwrap().as_message().unwrap();
        let mut p3_fields = p3.fields();
        assert_eq!(
            p3_fields.next().unwrap().occurrence(),
            FieldOccurrence::Singular(SingularPresence::Implicit)
        );
        assert_eq!(
            p3_fields.next().unwrap().occurrence(),
            FieldOccurrence::Singular(SingularPresence::Explicit)
        );
        assert_eq!(
            p3_fields.next().unwrap().occurrence(),
            FieldOccurrence::Singular(SingularPresence::Message)
        );
        assert_eq!(
            p3_fields.next().unwrap().occurrence(),
            FieldOccurrence::Singular(SingularPresence::Oneof)
        );
        assert_eq!(
            p3_fields.next().unwrap().occurrence(),
            FieldOccurrence::Repeated(RepeatedFieldEncoding::Packed)
        );

        let p2 = file_set.lookup(".p2.M").unwrap().as_message().unwrap();
        let mut p2_fields = p2.fields();
        assert_eq!(
            p2_fields.next().unwrap().occurrence(),
            FieldOccurrence::Singular(SingularPresence::Explicit)
        );
        assert_eq!(
            p2_fields.next().unwrap().occurrence(),
            FieldOccurrence::Singular(SingularPresence::LegacyRequired)
        );
    }

    #[test]
    fn editions_default_presence_is_explicit() {
        let arena = Arena::new();
        let files = [proto_file(
            "ed",
            Syntax::Editions(Edition::Edition2023),
            vec![MessageDesc {
                name: "M".into(),
                fields: vec![scalar_field("n", FieldLabel::Optional, None, false)],
                nested_messages: vec![],
                nested_enums: vec![],
                oneofs: vec![],
            }],
            vec![],
        )];
        let file_set = resolve(&arena, &files).unwrap();
        let m = file_set.lookup(".ed.M").unwrap().as_message().unwrap();
        assert_eq!(
            m.fields().next().unwrap().occurrence(),
            FieldOccurrence::Singular(SingularPresence::Explicit)
        );
    }

    #[test]
    fn editions_file_feature_implicit() {
        let arena = Arena::new();
        let mut file = proto_file(
            "ed",
            Syntax::Editions(Edition::Edition2024),
            vec![MessageDesc {
                name: "M".into(),
                fields: vec![
                    scalar_field("n", FieldLabel::Optional, None, false),
                    FieldDesc {
                        name: "override_explicit".into(),
                        number: 2,
                        label: FieldLabel::Optional,
                        type_: FieldType::Int32,
                        type_name: None,
                        oneof_index: None,
                        proto3_optional: false,
                        packed: None,
                        features: FeatureSet {
                            field_presence: Some(FieldPresence::Explicit),
                            ..FeatureSet::default()
                        },
                    },
                ],
                nested_messages: vec![],
                nested_enums: vec![],
                oneofs: vec![],
            }],
            vec![],
        );
        file.features = FeatureSet {
            field_presence: Some(FieldPresence::Implicit),
            ..FeatureSet::default()
        };
        let file_set = resolve(&arena, &[file]).unwrap();
        let m = file_set.lookup(".ed.M").unwrap().as_message().unwrap();
        let mut fields = m.fields();
        assert_eq!(
            fields.next().unwrap().occurrence(),
            FieldOccurrence::Singular(SingularPresence::Implicit)
        );
        assert_eq!(
            fields.next().unwrap().occurrence(),
            FieldOccurrence::Singular(SingularPresence::Explicit)
        );
    }

    #[test]
    fn editions_repeated_encoding_and_utf8() {
        use crate::descriptor::features::Utf8Validation;

        let arena = Arena::new();
        let file = ProtoFile {
            name: "a.proto".into(),
            package: "ed".into(),
            syntax: Syntax::Editions(Edition::Edition2023),
            features: FeatureSet::default(),
            dependency: vec![],
            messages: vec![MessageDesc {
                name: "M".into(),
                fields: vec![
                    FieldDesc {
                        name: "scores".into(),
                        number: 1,
                        label: FieldLabel::Repeated,
                        type_: FieldType::Int32,
                        type_name: None,
                        oneof_index: None,
                        proto3_optional: false,
                        packed: None,
                        features: FeatureSet {
                            repeated_field_encoding: Some(RepeatedFieldEncoding::Expanded),
                            ..FeatureSet::default()
                        },
                    },
                    FieldDesc {
                        name: "title".into(),
                        number: 2,
                        label: FieldLabel::Optional,
                        type_: FieldType::String,
                        type_name: None,
                        oneof_index: None,
                        proto3_optional: false,
                        packed: None,
                        features: FeatureSet {
                            utf8_validation: Some(Utf8Validation::None),
                            ..FeatureSet::default()
                        },
                    },
                ],
                nested_messages: vec![],
                nested_enums: vec![],
                oneofs: vec![],
            }],
            enums: vec![],
        };
        let file_set = resolve(&arena, &[file]).unwrap();
        let m = file_set.lookup(".ed.M").unwrap().as_message().unwrap();
        let mut fields = m.fields();
        let scores = fields.next().unwrap();
        assert_eq!(
            scores.occurrence(),
            FieldOccurrence::Repeated(RepeatedFieldEncoding::Expanded)
        );
        let title = fields.next().unwrap();
        assert_eq!(title.utf8_validation(), Some(Utf8Validation::None));
    }

    #[test]
    fn editions_enum_type_on_enum_and_file() {
        let arena = Arena::new();
        let file = ProtoFile {
            name: "a.proto".into(),
            package: "ed".into(),
            syntax: Syntax::Editions(Edition::Edition2023),
            features: FeatureSet {
                enum_type: Some(EnumType::Closed),
                ..FeatureSet::default()
            },
            dependency: vec![],
            messages: vec![],
            enums: vec![
                EnumDesc {
                    name: "ClosedByFile".into(),
                    values: vec![EnumValueDesc {
                        name: "A".into(),
                        number: 0,
                    }],
                    features: FeatureSet::default(),
                },
                EnumDesc {
                    name: "OpenOverride".into(),
                    values: vec![EnumValueDesc {
                        name: "B".into(),
                        number: 0,
                    }],
                    features: FeatureSet {
                        enum_type: Some(EnumType::Open),
                        ..FeatureSet::default()
                    },
                },
            ],
        };
        let file_set = resolve(&arena, &[file]).unwrap();
        assert_eq!(
            file_set
                .lookup(".ed.ClosedByFile")
                .unwrap()
                .as_enum()
                .unwrap()
                .openness(),
            EnumType::Closed
        );
        assert_eq!(
            file_set
                .lookup(".ed.OpenOverride")
                .unwrap()
                .as_enum()
                .unwrap()
                .openness(),
            EnumType::Open
        );
    }

    #[test]
    fn editions_delimited_message_encoding_errors() {
        use crate::descriptor::features::MessageEncoding;

        let arena = Arena::new();
        let files = [proto_file(
            "ed",
            Syntax::Editions(Edition::Edition2023),
            vec![
                empty_msg("Inner"),
                MessageDesc {
                    name: "Outer".into(),
                    fields: vec![FieldDesc {
                        name: "inner".into(),
                        number: 1,
                        label: FieldLabel::Optional,
                        type_: FieldType::Message,
                        type_name: Some(ProtoFqn::parse(".ed.Inner")),
                        oneof_index: None,
                        proto3_optional: false,
                        packed: None,
                        features: FeatureSet {
                            message_encoding: Some(MessageEncoding::Delimited),
                            ..FeatureSet::default()
                        },
                    }],
                    nested_messages: vec![],
                    nested_enums: vec![],
                    oneofs: vec![],
                },
            ],
            vec![],
        )];
        let err = resolve(&arena, &files).unwrap_err();
        assert!(err.to_string().contains("DELIMITED"));
    }

    #[test]
    fn group_field_is_rejected() {
        let arena = Arena::new();
        let files = [proto_file(
            "p2",
            Syntax::Proto2,
            vec![MessageDesc {
                name: "M".into(),
                fields: vec![FieldDesc {
                    name: "g".into(),
                    number: 1,
                    label: FieldLabel::Optional,
                    type_: FieldType::Group,
                    type_name: Some(ProtoFqn::parse(".p2.G")),
                    oneof_index: None,
                    proto3_optional: false,
                    packed: None,
                    features: FeatureSet::default(),
                }],
                nested_messages: vec![empty_msg("G")],
                nested_enums: vec![],
                oneofs: vec![],
            }],
            vec![],
        )];
        let err = resolve(&arena, &files).unwrap_err();
        assert!(err.to_string().contains("group"), "unexpected error: {err}");
    }

    #[test]
    fn proto2_packed_option() {
        let arena = Arena::new();
        let files = [proto_file(
            "p2",
            Syntax::Proto2,
            vec![MessageDesc {
                name: "M".into(),
                fields: vec![
                    FieldDesc {
                        name: "packed_nums".into(),
                        number: 1,
                        label: FieldLabel::Repeated,
                        type_: FieldType::Int32,
                        type_name: None,
                        oneof_index: None,
                        proto3_optional: false,
                        packed: Some(true),
                        features: FeatureSet::default(),
                    },
                    FieldDesc {
                        name: "expanded_nums".into(),
                        number: 2,
                        label: FieldLabel::Repeated,
                        type_: FieldType::Int32,
                        type_name: None,
                        oneof_index: None,
                        proto3_optional: false,
                        packed: None,
                        features: FeatureSet::default(),
                    },
                ],
                nested_messages: vec![],
                nested_enums: vec![],
                oneofs: vec![],
            }],
            vec![],
        )];
        let file_set = resolve(&arena, &files).unwrap();
        let m = file_set.lookup(".p2.M").unwrap().as_message().unwrap();
        let mut fields = m.fields();
        assert_eq!(
            fields.next().unwrap().occurrence(),
            FieldOccurrence::Repeated(RepeatedFieldEncoding::Packed)
        );
        assert_eq!(
            fields.next().unwrap().occurrence(),
            FieldOccurrence::Repeated(RepeatedFieldEncoding::Expanded)
        );
    }
}
