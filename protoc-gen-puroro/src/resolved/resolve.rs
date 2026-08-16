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
use ::std::collections::HashMap;
use ::std::mem;
use ::std::ptr;

/// Message + defining descriptor, queued while registering so the link pass
/// does not re-walk the tree or re-derive FQNs.
struct PendingFields<'a, 'd> {
    message: &'a Message<'a>,
    desc: &'d MessageDesc,
    file: &'d ProtoFile,
    /// Descriptor `oneof_index` → index in the resolved [`Message::oneofs`]
    /// list (synthetic proto3-optional oneofs are dropped).
    oneof_remap: Vec<Option<i32>>,
}

/// Name map + pending field edges for one [`resolve`] call.
struct ResolveCtx<'a, 'd> {
    arena: &'a Arena,
    types_by_fqn: HashMap<ProtoFqn, TypeItem<'a>>,
    pending: Vec<PendingFields<'a, 'd>>,
}

/// Resolve descriptor type names into an arena-local [`FileSet`].
///
/// Plugin metadata ([`crate::descriptor::CodegenMeta`]) is intentionally unused
/// here — keep it beside the returned `FileSet` and pass both by reference.
pub fn resolve<'a>(arena: &'a Arena, proto_files: &[ProtoFile]) -> Result<FileSet<'a>> {
    let mut ctx = ResolveCtx {
        arena,
        types_by_fqn: HashMap::new(),
        pending: Vec::new(),
    };
    let mut files = Vec::with_capacity(proto_files.len());

    // Pass 1: allocate every message/enum node and register by FQN.
    for proto in proto_files {
        files.push(ctx.register_file(proto)?);
    }

    // Pass 2: file-level feature traps, then link queued field edges.
    for proto in proto_files {
        check_file_features(proto);
    }
    ctx.link_pending()?;

    Ok(FileSet {
        files,
        types_by_fqn: ctx.types_by_fqn,
    })
}

impl<'a, 'd> ResolveCtx<'a, 'd> {
    fn register_file(&mut self, proto: &'d ProtoFile) -> Result<&'a File<'a>> {
        let package_fqn = ProtoFqn::from_package(&proto.package);

        let mut messages = Vec::with_capacity(proto.messages.len());
        for desc in &proto.messages {
            messages.push(self.register_message(desc, &package_fqn, proto, None)?);
        }

        let mut enums = Vec::with_capacity(proto.enums.len());
        for desc in &proto.enums {
            enums.push(self.register_enum(desc, &package_fqn, proto, None)?);
        }

        Ok(self.arena.alloc(File {
            name: proto.name.clone(),
            package: proto.package.clone(),
            syntax: proto.syntax,
            dependency: proto.dependency.clone(),
            messages,
            enums,
        }))
    }

    fn register_message(
        &mut self,
        desc: &'d MessageDesc,
        // FQN of the enclosing package (possibly ".") or parent message.
        parent_fqn: &ProtoFqn,
        file: &'d ProtoFile,
        parent: Option<&'a Message<'a>>,
    ) -> Result<&'a Message<'a>> {
        let fqn = parent_fqn.append(&desc.name);

        if self.types_by_fqn.contains_key(&fqn) {
            return Err(Error::Codegen(format!(
                "duplicate type FQN `{fqn}` while resolving schema"
            )));
        }

        let (oneofs, oneof_remap) = real_oneofs(desc);

        let message = self.arena.alloc(Message {
            name: desc.name.clone(),
            fqn: fqn.clone(),
            parent,
            fields: Vec::new(),
            nested_messages: Vec::new(),
            nested_enums: Vec::new(),
            oneofs,
            map_entry: desc.map_entry,
        });
        self.types_by_fqn
            .insert(fqn.clone(), TypeItem::Message(message));
        self.pending.push(PendingFields {
            message,
            desc,
            file,
            oneof_remap,
        });

        let mut nested_messages = Vec::with_capacity(desc.nested_messages.len());
        for nested in &desc.nested_messages {
            nested_messages.push(self.register_message(nested, &fqn, file, Some(message))?);
        }
        write_shared(&message.nested_messages, nested_messages);

        let mut nested_enums = Vec::with_capacity(desc.nested_enums.len());
        for nested in &desc.nested_enums {
            nested_enums.push(self.register_enum(nested, &fqn, file, Some(message))?);
        }
        write_shared(&message.nested_enums, nested_enums);

        Ok(message)
    }

    fn register_enum(
        &mut self,
        desc: &EnumDesc,
        // FQN of the enclosing package (possibly ".") or parent message.
        parent_fqn: &ProtoFqn,
        file: &ProtoFile,
        parent: Option<&'a Message<'a>>,
    ) -> Result<&'a Enum<'a>> {
        let fqn = parent_fqn.append(&desc.name);

        if self.types_by_fqn.contains_key(&fqn) {
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

        let enum_ty = self.arena.alloc(Enum {
            name: desc.name.clone(),
            fqn: fqn.clone(),
            parent,
            openness: resolve_enum_openness(file.syntax, &file.features, &desc.features),
            values,
        });
        self.types_by_fqn.insert(fqn, TypeItem::Enum(enum_ty));
        Ok(enum_ty)
    }

    fn link_pending(&mut self) -> Result<()> {
        let pending = mem::take(&mut self.pending);
        for item in pending {
            self.link_message_fields(item)?;
        }
        Ok(())
    }

    fn link_message_fields(&self, pending: PendingFields<'a, 'd>) -> Result<()> {
        let mut fields = Vec::with_capacity(pending.desc.fields.len());
        for field in &pending.desc.fields {
            fields.push(self.resolve_field(
                field,
                &pending.message.fqn,
                pending.file,
                &pending.oneof_remap,
            )?);
        }
        write_shared(&pending.message.fields, fields);
        Ok(())
    }

    fn resolve_field(
        &self,
        field: &FieldDesc,
        owner_fqn: &ProtoFqn,
        file: &ProtoFile,
        oneof_remap: &[Option<i32>],
    ) -> Result<Field<'a>> {
        if field.type_ == FieldType::Group {
            return Err(Error::Codegen(format!(
                "field `{owner_fqn}.{}`: deprecated group fields are not supported",
                field.name
            )));
        }

        let editions_features = match file.syntax {
            Syntax::Editions(edition) => {
                field
                    .features
                    .reject_unimplemented_overrides(&format!("field `{owner_fqn}.{}`", field.name));
                Some(
                    FeatureSet::defaults_for_edition(edition)
                        .overlay(&file.features)
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
                let target = self.lookup_message(type_name).ok_or_else(|| {
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
                let target = self.lookup_enum(type_name).ok_or_else(|| {
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

        let occurrence =
            resolve_occurrence(field, file.syntax, editions_features.as_ref(), &type_ref);
        Ok(Field {
            name: field.name.clone(),
            number: field.number,
            occurrence,
            type_ref,
            oneof_index: resolved_oneof_index(field, owner_fqn, oneof_remap)?,
            default_value: field.default_value.clone(),
            utf8_validation: resolve_utf8_validation(
                field,
                file.syntax,
                editions_features.as_ref(),
            ),
            string_layout: field.string_layout,
            bytes_layout: field.bytes_layout,
        })
    }

    fn lookup_message(&self, type_name: &ProtoFqn) -> Option<&'a Message<'a>> {
        match self.types_by_fqn.get(type_name)? {
            TypeItem::Message(m) => Some(*m),
            TypeItem::Enum(_) => None,
        }
    }

    fn lookup_enum(&self, type_name: &ProtoFqn) -> Option<&'a Enum<'a>> {
        match self.types_by_fqn.get(type_name)? {
            TypeItem::Enum(e) => Some(*e),
            TypeItem::Message(_) => None,
        }
    }
}

/// Overwrite a field of an arena node that is already shared by `&`.
///
/// Only used during [`resolve`], before [`FileSet`] is returned. There must be
/// no outstanding borrow of `slot`'s contents.
fn write_shared<T>(slot: &T, value: T) {
    // Safety: `resolve` is single-threaded and does not read these slots until
    // after the matching write. The arena allocation outlives both.
    unsafe {
        let _dropped = ptr::replace(ptr::from_ref(slot).cast_mut(), value);
    }
}

fn resolve_enum_openness(
    syntax: Syntax,
    file_features: &FeatureSet,
    enum_features: &FeatureSet,
) -> EnumType {
    match syntax {
        // Historical defaults (editions features are not used for proto2/proto3).
        Syntax::Proto2 => EnumType::Closed,
        Syntax::Proto3 => EnumType::Open,
        Syntax::Editions(edition) => FeatureSet::defaults_for_edition(edition)
            .overlay(file_features)
            .overlay(enum_features)
            .enum_type
            .expect("edition defaults always set enum_type"),
    }
}

/// Keep oneofs that have at least one real member. Proto3 `optional` fields
/// are stored as a synthetic single-field oneof in the descriptor; those
/// groups are dropped and remaining indices are remapped.
fn real_oneofs(desc: &MessageDesc) -> (Vec<Oneof>, Vec<Option<i32>>) {
    let n = desc.oneofs.len();
    let mut is_real = vec![false; n];
    for field in &desc.fields {
        if field.proto3_optional {
            continue;
        }
        let Some(idx) = field.oneof_index else {
            continue;
        };
        let Ok(idx) = usize::try_from(idx) else {
            continue;
        };
        if let Some(slot) = is_real.get_mut(idx) {
            *slot = true;
        }
    }

    let mut remap = vec![None; n];
    let mut oneofs = Vec::new();
    for (i, oneof) in desc.oneofs.iter().enumerate() {
        if is_real[i] {
            remap[i] = Some(i32::try_from(oneofs.len()).expect("oneof count fits i32"));
            oneofs.push(Oneof {
                name: oneof.name.clone(),
            });
        }
    }
    (oneofs, remap)
}

fn resolved_oneof_index(
    field: &FieldDesc,
    owner_fqn: &ProtoFqn,
    remap: &[Option<i32>],
) -> Result<Option<i32>> {
    if field.proto3_optional {
        return Ok(None);
    }
    let Some(idx) = field.oneof_index else {
        return Ok(None);
    };
    let Ok(idx) = usize::try_from(idx) else {
        return Err(Error::Codegen(format!(
            "field `{owner_fqn}.{}` has negative oneof_index",
            field.name
        )));
    };
    match remap.get(idx).copied() {
        Some(Some(mapped)) => Ok(Some(mapped)),
        Some(None) => Err(Error::Codegen(format!(
            "field `{owner_fqn}.{}` oneof_index {idx} is not a real oneof",
            field.name
        ))),
        None => Err(Error::Codegen(format!(
            "field `{owner_fqn}.{}` oneof_index {idx} out of range ({} oneofs)",
            field.name,
            remap.len()
        ))),
    }
}

fn check_file_features(proto: &ProtoFile) {
    if let Syntax::Editions(edition) = proto.syntax {
        proto
            .features
            .reject_unimplemented_overrides(&format!("file `{}`", proto.name));
        let file_features = FeatureSet::defaults_for_edition(edition).overlay(&proto.features);
        file_features.apply_or_trap_for_file();
    }
}

fn resolve_occurrence(
    field: &FieldDesc,
    syntax: Syntax,
    editions_features: Option<&FeatureSet>,
    type_ref: &TypeRef<'_>,
) -> FieldOccurrence {
    if field.label == FieldLabel::Repeated {
        if let TypeRef::Message(entry) = type_ref
            && entry.is_map_entry()
        {
            return FieldOccurrence::Map;
        }
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
            map_entry: false,
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
            default_value: None,
            packed: None,
            string_layout: None,
            bytes_layout: None,
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
                        default_value: None,
                        packed: None,
                        string_layout: None,
                        bytes_layout: None,
                        features: FeatureSet::default(),
                    }],
                    nested_messages: vec![],
                    nested_enums: vec![],
                    oneofs: vec![],
                    map_entry: false,
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
                map_entry: false,
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
                    default_value: None,
                    packed: None,
                    string_layout: None,
                    bytes_layout: None,
                    features: FeatureSet::default(),
                }],
                nested_messages: vec![],
                nested_enums: vec![],
                oneofs: vec![],
                map_entry: false,
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
                    default_value: None,
                    packed: None,
                    string_layout: None,
                    bytes_layout: None,
                    features: FeatureSet::default(),
                }],
                nested_messages: vec![],
                nested_enums: vec![],
                oneofs: vec![],
                map_entry: false,
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
                        default_value: None,
                        packed: None,
                        string_layout: None,
                        bytes_layout: None,
                        features: FeatureSet::default(),
                    }],
                    nested_messages: vec![],
                    nested_enums: vec![],
                    oneofs: vec![],
                    map_entry: false,
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
                        default_value: None,
                        packed: None,
                        string_layout: None,
                        bytes_layout: None,
                        features: FeatureSet::default(),
                    }],
                    nested_messages: vec![],
                    nested_enums: vec![],
                    oneofs: vec![],
                    map_entry: false,
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
                                default_value: None,
                                packed: None,
                                string_layout: None,
                                bytes_layout: None,
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
                                default_value: None,
                                packed: None,
                                string_layout: None,
                                bytes_layout: None,
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
                                default_value: None,
                                packed: None,
                                string_layout: None,
                                bytes_layout: None,
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
                                default_value: None,
                                packed: None,
                                string_layout: None,
                                bytes_layout: None,
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
                        map_entry: false,
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
                    map_entry: false,
                }],
                vec![],
            ),
        ];
        let file_set = resolve(&arena, &files).unwrap();

        let p3 = file_set.lookup(".p3.M").unwrap().as_message().unwrap();
        let p3_oneofs: Vec<_> = p3.oneofs().map(|o| o.name()).collect();
        assert_eq!(p3_oneofs, ["which"]);
        let mut p3_fields = p3.fields();
        let implicit = p3_fields.next().unwrap();
        assert_eq!(
            implicit.occurrence(),
            FieldOccurrence::Singular(SingularPresence::Implicit)
        );
        assert!(implicit.oneof_index().is_none());
        let explicit = p3_fields.next().unwrap();
        assert_eq!(
            explicit.occurrence(),
            FieldOccurrence::Singular(SingularPresence::Explicit)
        );
        assert!(
            explicit.oneof_index().is_none(),
            "proto3 optional synthetic oneof is stripped"
        );
        assert_eq!(
            p3_fields.next().unwrap().occurrence(),
            FieldOccurrence::Singular(SingularPresence::Message)
        );
        let choice = p3_fields.next().unwrap();
        assert_eq!(
            choice.occurrence(),
            FieldOccurrence::Singular(SingularPresence::Oneof)
        );
        assert_eq!(choice.oneof_index(), Some(0));
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
                map_entry: false,
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
                        default_value: None,
                        packed: None,
                        string_layout: None,
                        bytes_layout: None,
                        features: FeatureSet {
                            field_presence: Some(FieldPresence::Explicit),
                            ..FeatureSet::default()
                        },
                    },
                ],
                nested_messages: vec![],
                nested_enums: vec![],
                oneofs: vec![],
                map_entry: false,
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
                        default_value: None,
                        packed: None,
                        string_layout: None,
                        bytes_layout: None,
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
                        default_value: None,
                        packed: None,
                        string_layout: None,
                        bytes_layout: None,
                        features: FeatureSet {
                            utf8_validation: Some(Utf8Validation::None),
                            ..FeatureSet::default()
                        },
                    },
                ],
                nested_messages: vec![],
                nested_enums: vec![],
                oneofs: vec![],
                map_entry: false,
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
                        default_value: None,
                        packed: None,
                        string_layout: None,
                        bytes_layout: None,
                        features: FeatureSet {
                            message_encoding: Some(MessageEncoding::Delimited),
                            ..FeatureSet::default()
                        },
                    }],
                    nested_messages: vec![],
                    nested_enums: vec![],
                    oneofs: vec![],
                    map_entry: false,
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
                    default_value: None,
                    packed: None,
                    string_layout: None,
                    bytes_layout: None,
                    features: FeatureSet::default(),
                }],
                nested_messages: vec![empty_msg("G")],
                nested_enums: vec![],
                oneofs: vec![],
                map_entry: false,
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
                        default_value: None,
                        packed: Some(true),
                        string_layout: None,
                        bytes_layout: None,
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
                        default_value: None,
                        packed: None,
                        string_layout: None,
                        bytes_layout: None,
                        features: FeatureSet::default(),
                    },
                ],
                nested_messages: vec![],
                nested_enums: vec![],
                oneofs: vec![],
                map_entry: false,
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

    #[test]
    fn resolve_map_field_occurrence() {
        let arena = Arena::new();
        let files = [proto_file(
            "example",
            Syntax::Proto3,
            vec![MessageDesc {
                name: "Holder".into(),
                fields: vec![FieldDesc {
                    name: "attributes".into(),
                    number: 1,
                    label: FieldLabel::Repeated,
                    type_: FieldType::Message,
                    type_name: Some(ProtoFqn::parse(".example.Holder.AttributesEntry")),
                    oneof_index: None,
                    proto3_optional: false,
                    default_value: None,
                    packed: None,
                    string_layout: None,
                    bytes_layout: None,
                    features: FeatureSet::default(),
                }],
                nested_messages: vec![MessageDesc {
                    name: "AttributesEntry".into(),
                    fields: vec![
                        FieldDesc {
                            name: "key".into(),
                            number: 1,
                            label: FieldLabel::Optional,
                            type_: FieldType::String,
                            type_name: None,
                            oneof_index: None,
                            proto3_optional: false,
                            default_value: None,
                            packed: None,
                            string_layout: None,
                            bytes_layout: None,
                            features: FeatureSet::default(),
                        },
                        FieldDesc {
                            name: "value".into(),
                            number: 2,
                            label: FieldLabel::Optional,
                            type_: FieldType::Int32,
                            type_name: None,
                            oneof_index: None,
                            proto3_optional: false,
                            default_value: None,
                            packed: None,
                            string_layout: None,
                            bytes_layout: None,
                            features: FeatureSet::default(),
                        },
                    ],
                    nested_messages: vec![],
                    nested_enums: vec![],
                    oneofs: vec![],
                    map_entry: true,
                }],
                nested_enums: vec![],
                oneofs: vec![],
                map_entry: false,
            }],
            vec![],
        )];
        let file_set = resolve(&arena, &files).unwrap();
        let holder = file_set
            .lookup(".example.Holder")
            .unwrap()
            .as_message()
            .unwrap();
        let entry = file_set
            .lookup(".example.Holder.AttributesEntry")
            .unwrap()
            .as_message()
            .unwrap();
        assert!(entry.is_map_entry());
        let field = holder.fields().next().unwrap();
        assert_eq!(field.occurrence(), FieldOccurrence::Map);
        assert!(ptr::eq(field.type_ref().as_message().unwrap(), entry));
    }
}
