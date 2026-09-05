//! Build a [`FileSet`](super::FileSet) inside a caller-owned [`Arena`](super::Arena).

use super::{
    Arena, Enum, EnumValue, Field, FieldOccurrence, File, FileSet, Message, Oneof,
    SingularPresence, TypeRef,
};
use crate::descriptor::features::{
    EnumType, FeatureSet, FieldPresence, FinalizedFeatureSet, MessageEncoding,
    RepeatedFieldEncoding, Utf8Validation,
};
use crate::descriptor::{
    EnumDesc, FieldDesc, FieldLabel, FieldType, MessageDesc, ProtoFile, ProtoFqn, Syntax,
};
use crate::error::{Error, Result};
use ::std::collections::HashMap;
use ::std::iter::IntoIterator;
use ::std::mem;
use ::std::ptr;

/// Descriptor `oneof_index` → index in the resolved [`Message::oneofs`] list.
///
/// Synthetic proto3-optional oneofs are dropped; remaining indices are remapped.
struct OneofRemap {
    remap: Vec<Option<i32>>,
}

impl OneofRemap {
    fn from_message(desc: &MessageDesc) -> (Vec<Oneof>, Self) {
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
        (oneofs, Self { remap })
    }

    fn index(&self, field: &FieldDesc, owner_fqn: &ProtoFqn) -> Result<Option<i32>> {
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
        match self.remap.get(idx).copied() {
            Some(Some(mapped)) => Ok(Some(mapped)),
            Some(None) => Err(Error::Codegen(format!(
                "field `{owner_fqn}.{}` oneof_index {idx} is not a real oneof",
                field.name
            ))),
            None => Err(Error::Codegen(format!(
                "field `{owner_fqn}.{}` oneof_index {idx} out of range ({} oneofs)",
                field.name,
                self.remap.len()
            ))),
        }
    }
}

/// Message + defining descriptor, queued while registering so the link pass
/// does not re-walk the tree or re-derive FQNs.
struct PendingFields<'a, 'd> {
    message: &'a Message<'a>,
    desc: &'d MessageDesc,
    file: &'d ProtoFile,
    oneof_remap: OneofRemap,
}

/// Name map + pending field edges for one [`resolve`] call.
struct ResolveCtx<'a, 'd> {
    arena: &'a Arena,
    /// proto2 syntax default for `utf8_validation` (spec is `NONE`).
    proto2_utf8: Utf8Validation,
    messages_by_fqn: HashMap<ProtoFqn, &'a Message<'a>>,
    enums_by_fqn: HashMap<ProtoFqn, &'a Enum<'a>>,
    pending: Vec<PendingFields<'a, 'd>>,
}

/// Resolve descriptor type names into an arena-local [`FileSet`].
///
/// Plugin metadata ([`crate::descriptor::CodegenMeta`]) is intentionally unused
/// here — keep it beside the returned `FileSet` and pass both by reference.
/// Proto2 `utf8_validation` defaults to [`Utf8Validation::None`]; use
/// [`resolve_with`] to overlay `VERIFY`.
pub fn resolve<'a>(arena: &'a Arena, proto_files: &[ProtoFile]) -> Result<FileSet<'a>> {
    resolve_with(arena, proto_files, Utf8Validation::None)
}

/// Like [`resolve`], with a proto2-only overlay for the `utf8_validation`
/// syntax default.
///
/// Editions files (and their explicit field features) are unchanged.
pub fn resolve_with<'a>(
    arena: &'a Arena,
    proto_files: &[ProtoFile],
    proto2_utf8: Utf8Validation,
) -> Result<FileSet<'a>> {
    let mut ctx = ResolveCtx {
        arena,
        proto2_utf8,
        messages_by_fqn: HashMap::new(),
        enums_by_fqn: HashMap::new(),
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

    Ok(FileSet { files })
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

        self.check_unique_fqn(&fqn)?;

        let (oneofs, oneof_remap) = OneofRemap::from_message(desc);

        let message = self.arena.alloc(Message {
            name: desc.name.clone(),
            fqn: fqn.clone(),
            parent,
            fields: Vec::new(),
            nested_messages: Vec::new(),
            nested_enums: Vec::new(),
            oneofs,
            map_entry: desc.map_entry,
            unknown_fields: desc.unknown_fields,
        });
        self.messages_by_fqn.insert(fqn.clone(), message);
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

        self.check_unique_fqn(&fqn)?;

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
            openness: EnumType::resolve(file, &desc.features, self.proto2_utf8),
            values,
        });
        self.enums_by_fqn.insert(fqn, enum_ty);
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
        oneof_remap: &OneofRemap,
    ) -> Result<Field<'a>> {
        if field.type_ == FieldType::Group {
            return Err(Error::Codegen(format!(
                "field `{owner_fqn}.{}`: deprecated group fields are not supported",
                field.name
            )));
        }

        if matches!(file.syntax, Syntax::Editions(_)) {
            field
                .features
                .reject_unimplemented_overrides(&format!("field `{owner_fqn}.{}`", field.name));
        }

        let features = FinalizedFeatureSet::resolve_field(file, field, self.proto2_utf8);

        if matches!(field.type_, FieldType::Message)
            && features.message_encoding == MessageEncoding::Delimited
        {
            return Err(Error::Codegen(format!(
                "field `{owner_fqn}.{}`: features.message_encoding=DELIMITED is not supported",
                field.name
            )));
        }

        let type_ref = self.resolve_type_ref(field, owner_fqn)?;

        Ok(Field {
            name: field.name.clone(),
            number: field.number,
            occurrence: FieldOccurrence::resolve(field, &features, &type_ref),
            type_ref,
            oneof_index: oneof_remap.index(field, owner_fqn)?,
            default_value: field.default_value.clone(),
            utf8_validation: Utf8Validation::resolve(field, &features),
            string_layout: field.string_layout,
            bytes_layout: field.bytes_layout,
            message_layout: field.message_layout,
            legacy_required: features.field_presence == FieldPresence::LegacyRequired,
        })
    }

    fn resolve_type_ref(&self, field: &FieldDesc, owner_fqn: &ProtoFqn) -> Result<TypeRef<'a>> {
        Ok(match field.type_ {
            FieldType::Double => TypeRef::Double,
            FieldType::Float => TypeRef::Float,
            FieldType::Int64 => TypeRef::Int64,
            FieldType::UInt64 => TypeRef::UInt64,
            FieldType::Int32 => TypeRef::Int32,
            FieldType::Fixed64 => TypeRef::Fixed64,
            FieldType::Fixed32 => TypeRef::Fixed32,
            FieldType::Bool => TypeRef::Bool,
            FieldType::String => TypeRef::String,
            FieldType::Bytes => TypeRef::Bytes,
            FieldType::UInt32 => TypeRef::UInt32,
            FieldType::SFixed32 => TypeRef::SFixed32,
            FieldType::SFixed64 => TypeRef::SFixed64,
            FieldType::SInt32 => TypeRef::SInt32,
            FieldType::SInt64 => TypeRef::SInt64,
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
        })
    }

    fn lookup_message(&self, type_name: &ProtoFqn) -> Option<&'a Message<'a>> {
        self.messages_by_fqn.get(type_name).copied()
    }

    fn lookup_enum(&self, type_name: &ProtoFqn) -> Option<&'a Enum<'a>> {
        self.enums_by_fqn.get(type_name).copied()
    }

    fn check_unique_fqn(&self, fqn: &ProtoFqn) -> Result<()> {
        if self.messages_by_fqn.contains_key(fqn) || self.enums_by_fqn.contains_key(fqn) {
            return Err(Error::Codegen(format!(
                "duplicate type FQN `{fqn}` while resolving schema"
            )));
        }
        Ok(())
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

impl FinalizedFeatureSet {
    /// Syntax defaults, then `file.features`, then each of `inner` in order
    /// (later `Some` wins). Classic proto ignores every overlay except the
    /// generate-time proto2 `utf8_validation` default (`proto2_utf8`).
    fn resolve<'a>(
        file: &ProtoFile,
        inner: impl IntoIterator<Item = &'a FeatureSet>,
        proto2_utf8: Utf8Validation,
    ) -> Self {
        let mut features = Self::defaults_for_syntax(file.syntax);
        if matches!(file.syntax, Syntax::Proto2) {
            features.utf8_validation = proto2_utf8;
        }
        if matches!(file.syntax, Syntax::Editions(_)) {
            features = features.overlay(&file.features);
            for over in inner {
                features = features.overlay(over);
            }
        }
        features
    }

    /// [`Self::resolve`] plus proto2/proto3 descriptor knobs that editions
    /// express as feature overrides (`packed`, `required`, `proto3_optional`).
    fn resolve_field(file: &ProtoFile, field: &FieldDesc, proto2_utf8: Utf8Validation) -> Self {
        let mut features = Self::resolve(file, [&field.features], proto2_utf8);
        if field.proto3_optional {
            features.field_presence = FieldPresence::Explicit;
        }
        if field.label == FieldLabel::Required {
            features.field_presence = FieldPresence::LegacyRequired;
        }
        match field.packed {
            Some(true) => features.repeated_field_encoding = RepeatedFieldEncoding::Packed,
            Some(false) => features.repeated_field_encoding = RepeatedFieldEncoding::Expanded,
            None => {}
        }
        features
    }
}

impl EnumType {
    fn resolve(file: &ProtoFile, enum_features: &FeatureSet, proto2_utf8: Utf8Validation) -> Self {
        FinalizedFeatureSet::resolve(file, [enum_features], proto2_utf8).enum_type
    }
}

fn check_file_features(proto: &ProtoFile) {
    if matches!(proto.syntax, Syntax::Editions(_)) {
        proto
            .features
            .reject_unimplemented_overrides(&format!("file `{}`", proto.name));
        FinalizedFeatureSet::resolve(proto, None, Utf8Validation::None).apply_or_trap_for_file();
    }
}

impl FieldOccurrence {
    fn resolve(field: &FieldDesc, features: &FinalizedFeatureSet, type_ref: &TypeRef<'_>) -> Self {
        if field.label == FieldLabel::Repeated {
            if let TypeRef::Message(entry) = type_ref
                && entry.is_map_entry()
            {
                return Self::Map;
            }
            return Self::Repeated(features.repeated_field_encoding);
        }

        // Real oneof member (proto3 `optional` uses a synthetic oneof + proto3_optional).
        if field.oneof_index.is_some() && !field.proto3_optional {
            return Self::Singular(SingularPresence::Oneof);
        }

        if field.type_ == FieldType::Message {
            return Self::Singular(SingularPresence::Message);
        }

        Self::Singular(match features.field_presence {
            FieldPresence::Explicit => SingularPresence::Explicit,
            FieldPresence::Implicit => SingularPresence::Implicit,
            FieldPresence::LegacyRequired => SingularPresence::LegacyRequired,
        })
    }
}

impl Utf8Validation {
    fn resolve(field: &FieldDesc, features: &FinalizedFeatureSet) -> Option<Self> {
        if !matches!(field.type_, FieldType::String | FieldType::Bytes) {
            return None;
        }
        Some(features.utf8_validation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::descriptor::features::{EnumType, FeatureSet, FieldPresence, RepeatedFieldEncoding};
    use crate::descriptor::test_helpers as desc;
    use crate::descriptor::{
        Edition, EnumDesc, FieldDesc, FieldLabel, FieldType, MessageDesc, ProtoFile, ProtoFqn,
        Syntax,
    };
    use ::std::ptr;

    #[derive(Clone, Copy)]
    enum TypeItem<'a> {
        Message(&'a Message<'a>),
        Enum(&'a Enum<'a>),
    }

    fn lookup_message<'a>(file_set: &FileSet<'a>, fqn: impl AsRef<str>) -> Option<&'a Message<'a>> {
        lookup_item(file_set, fqn.as_ref()).and_then(TypeItem::as_message)
    }

    fn lookup_enum<'a>(file_set: &FileSet<'a>, fqn: impl AsRef<str>) -> Option<&'a Enum<'a>> {
        lookup_item(file_set, fqn.as_ref()).and_then(TypeItem::as_enum)
    }

    fn lookup_item<'a>(file_set: &FileSet<'a>, fqn: &str) -> Option<TypeItem<'a>> {
        debug_assert!(
            fqn.starts_with('.'),
            "lookup expects a canonical ProtoFqn (leading `.`), got {fqn:?}"
        );
        for file in file_set.files() {
            for message in file.messages() {
                if let Some(item) = lookup_in_message(message, fqn) {
                    return Some(item);
                }
            }
            for e in file.enums() {
                if e.fqn().as_str() == fqn {
                    return Some(TypeItem::Enum(e));
                }
            }
        }
        None
    }

    fn lookup_in_message<'a>(message: &'a Message<'a>, fqn: &str) -> Option<TypeItem<'a>> {
        if message.fqn().as_str() == fqn {
            return Some(TypeItem::Message(message));
        }
        for nested in message.nested_messages() {
            if let Some(item) = lookup_in_message(nested, fqn) {
                return Some(item);
            }
        }
        for e in message.nested_enums() {
            if e.fqn().as_str() == fqn {
                return Some(TypeItem::Enum(e));
            }
        }
        None
    }

    impl<'a> TypeItem<'a> {
        fn as_message(self) -> Option<&'a Message<'a>> {
            match self {
                Self::Message(m) => Some(m),
                Self::Enum(_) => None,
            }
        }

        fn as_enum(self) -> Option<&'a Enum<'a>> {
            match self {
                Self::Enum(e) => Some(e),
                Self::Message(_) => None,
            }
        }
    }

    fn proto_file(
        package: &str,
        syntax: Syntax,
        messages: Vec<MessageDesc>,
        enums: Vec<EnumDesc>,
    ) -> ProtoFile {
        ProtoFile {
            syntax,
            messages,
            enums,
            ..desc::proto_file("a.proto", package)
        }
    }

    fn scalar_field(
        name: &str,
        label: FieldLabel,
        oneof_index: Option<i32>,
        proto3_optional: bool,
    ) -> FieldDesc {
        FieldDesc {
            label,
            oneof_index,
            proto3_optional,
            ..desc::field(name, 1, FieldType::Int32)
        }
    }

    #[test]
    fn resolve_empty_message_under_package() {
        let arena = Arena::new();
        let files = [proto_file(
            "example.v1",
            Syntax::Proto3,
            vec![desc::message("Empty")],
            vec![],
        )];
        let file_set = resolve(&arena, &files).unwrap();
        let msg = lookup_message(&file_set, ".example.v1.Empty").unwrap();
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
                desc::message("Address"),
                MessageDesc {
                    fields: vec![FieldDesc {
                        proto3_optional: true,
                        type_name: Some(ProtoFqn::parse(".example.Address")),
                        ..desc::field("assignee", 1, FieldType::Message)
                    }],
                    ..desc::message("Task")
                },
            ],
            vec![],
        )];
        let file_set = resolve(&arena, &files).unwrap();
        let task = lookup_message(&file_set, ".example.Task").unwrap();
        let address = lookup_message(&file_set, ".example.Address").unwrap();
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
                nested_messages: vec![desc::message("Inner")],
                ..desc::message("Outer")
            }],
            vec![],
        )];
        let file_set = resolve(&arena, &files).unwrap();
        let outer = lookup_message(&file_set, ".Outer").unwrap();
        let inner = lookup_message(&file_set, ".Outer.Inner").unwrap();
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
                fields: vec![FieldDesc {
                    type_name: Some(ProtoFqn::parse(".example.Status")),
                    ..desc::field("status", 1, FieldType::Enum)
                }],
                ..desc::message("Task")
            }],
            vec![desc::enumeration(
                "Status",
                vec![desc::enum_value("STATUS_UNSPECIFIED", 0)],
            )],
        )];
        let file_set = resolve(&arena, &files).unwrap();
        let task = lookup_message(&file_set, ".example.Task").unwrap();
        let status = lookup_enum(&file_set, ".example.Status").unwrap();
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
                fields: vec![FieldDesc {
                    type_name: Some(ProtoFqn::parse(".Missing")),
                    ..desc::field("assignee", 1, FieldType::Message)
                }],
                ..desc::message("Task")
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
                    fields: vec![FieldDesc {
                        type_name: Some(ProtoFqn::parse(".B")),
                        ..desc::field("b", 1, FieldType::Message)
                    }],
                    ..desc::message("A")
                },
                MessageDesc {
                    fields: vec![FieldDesc {
                        type_name: Some(ProtoFqn::parse(".A")),
                        ..desc::field("a", 1, FieldType::Message)
                    }],
                    ..desc::message("B")
                },
            ],
            vec![],
        )];
        let file_set = resolve(&arena, &files).unwrap();
        let a = lookup_message(&file_set, ".A").unwrap();
        let b = lookup_message(&file_set, ".B").unwrap();
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
                    desc::message("Addr"),
                    MessageDesc {
                        fields: vec![
                            scalar_field("implicit", FieldLabel::Optional, None, false),
                            FieldDesc {
                                // Synthetic oneof for proto3 optional.
                                oneof_index: Some(0),
                                proto3_optional: true,
                                ..desc::field("explicit", 2, FieldType::Int32)
                            },
                            FieldDesc {
                                type_name: Some(ProtoFqn::parse(".p3.Addr")),
                                ..desc::field("addr", 3, FieldType::Message)
                            },
                            FieldDesc {
                                oneof_index: Some(1),
                                ..desc::field("choice", 4, FieldType::Int32)
                            },
                            FieldDesc {
                                label: FieldLabel::Repeated,
                                ..desc::field("tags", 5, FieldType::Int32)
                            },
                        ],
                        oneofs: vec![desc::oneof("_explicit"), desc::oneof("which")],
                        ..desc::message("M")
                    },
                ],
                vec![],
            ),
            proto_file(
                "p2",
                Syntax::Proto2,
                vec![MessageDesc {
                    fields: vec![
                        scalar_field("optional", FieldLabel::Optional, None, false),
                        scalar_field("required", FieldLabel::Required, None, false),
                    ],
                    ..desc::message("M")
                }],
                vec![],
            ),
        ];
        let file_set = resolve(&arena, &files).unwrap();

        let p3 = lookup_message(&file_set, ".p3.M").unwrap();
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

        let p2 = lookup_message(&file_set, ".p2.M").unwrap();
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
                fields: vec![scalar_field("n", FieldLabel::Optional, None, false)],
                ..desc::message("M")
            }],
            vec![],
        )];
        let file_set = resolve(&arena, &files).unwrap();
        let m = lookup_message(&file_set, ".ed.M").unwrap();
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
                fields: vec![
                    scalar_field("n", FieldLabel::Optional, None, false),
                    FieldDesc {
                        features: FeatureSet {
                            field_presence: Some(FieldPresence::Explicit),
                            ..FeatureSet::default()
                        },
                        ..desc::field("override_explicit", 2, FieldType::Int32)
                    },
                ],
                ..desc::message("M")
            }],
            vec![],
        );
        file.features = FeatureSet {
            field_presence: Some(FieldPresence::Implicit),
            ..FeatureSet::default()
        };
        let file_set = resolve(&arena, &[file]).unwrap();
        let m = lookup_message(&file_set, ".ed.M").unwrap();
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
            syntax: Syntax::Editions(Edition::Edition2023),
            messages: vec![MessageDesc {
                fields: vec![
                    FieldDesc {
                        label: FieldLabel::Repeated,
                        features: FeatureSet {
                            repeated_field_encoding: Some(RepeatedFieldEncoding::Expanded),
                            ..FeatureSet::default()
                        },
                        ..desc::field("scores", 1, FieldType::Int32)
                    },
                    FieldDesc {
                        features: FeatureSet {
                            utf8_validation: Some(Utf8Validation::None),
                            ..FeatureSet::default()
                        },
                        ..desc::field("title", 2, FieldType::String)
                    },
                ],
                ..desc::message("M")
            }],
            ..desc::proto_file("a.proto", "ed")
        };
        let file_set = resolve(&arena, &[file]).unwrap();
        let m = lookup_message(&file_set, ".ed.M").unwrap();
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
    fn proto2_string_utf8_defaults_to_none_unless_overlaid() {
        use crate::descriptor::features::Utf8Validation;

        let arena = Arena::new();
        let file = proto_file(
            "p2",
            Syntax::Proto2,
            vec![MessageDesc {
                fields: vec![desc::field("title", 1, FieldType::String)],
                ..desc::message("M")
            }],
            vec![],
        );
        let file_set = resolve(&arena, &[file]).unwrap();
        let title = lookup_message(&file_set, ".p2.M")
            .unwrap()
            .fields()
            .next()
            .unwrap();
        assert_eq!(title.utf8_validation(), Some(Utf8Validation::None));

        let arena = Arena::new();
        let file = proto_file(
            "p2",
            Syntax::Proto2,
            vec![MessageDesc {
                fields: vec![desc::field("title", 1, FieldType::String)],
                ..desc::message("M")
            }],
            vec![],
        );
        let file_set = resolve_with(&arena, &[file], Utf8Validation::Verify).unwrap();
        let title = lookup_message(&file_set, ".p2.M")
            .unwrap()
            .fields()
            .next()
            .unwrap();
        assert_eq!(title.utf8_validation(), Some(Utf8Validation::Verify));
    }

    #[test]
    fn proto2_utf8_overlay_does_not_change_editions_none() {
        use crate::descriptor::features::Utf8Validation;

        let arena = Arena::new();
        let file = ProtoFile {
            syntax: Syntax::Editions(Edition::Edition2023),
            messages: vec![MessageDesc {
                fields: vec![FieldDesc {
                    features: FeatureSet {
                        utf8_validation: Some(Utf8Validation::None),
                        ..FeatureSet::default()
                    },
                    ..desc::field("title", 1, FieldType::String)
                }],
                ..desc::message("M")
            }],
            ..desc::proto_file("a.proto", "ed")
        };
        let file_set = resolve_with(&arena, &[file], Utf8Validation::Verify).unwrap();
        let title = lookup_message(&file_set, ".ed.M")
            .unwrap()
            .fields()
            .next()
            .unwrap();
        assert_eq!(title.utf8_validation(), Some(Utf8Validation::None));
    }

    #[test]
    fn editions_enum_type_on_enum_and_file() {
        let arena = Arena::new();
        let file = ProtoFile {
            syntax: Syntax::Editions(Edition::Edition2023),
            features: FeatureSet {
                enum_type: Some(EnumType::Closed),
                ..FeatureSet::default()
            },
            enums: vec![
                desc::enumeration("ClosedByFile", vec![desc::enum_value("A", 0)]),
                EnumDesc {
                    features: FeatureSet {
                        enum_type: Some(EnumType::Open),
                        ..FeatureSet::default()
                    },
                    ..desc::enumeration("OpenOverride", vec![desc::enum_value("B", 0)])
                },
            ],
            ..desc::proto_file("a.proto", "ed")
        };
        let file_set = resolve(&arena, &[file]).unwrap();
        assert_eq!(
            lookup_enum(&file_set, ".ed.ClosedByFile")
                .unwrap()
                .openness(),
            EnumType::Closed
        );
        assert_eq!(
            lookup_enum(&file_set, ".ed.OpenOverride")
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
                desc::message("Inner"),
                MessageDesc {
                    fields: vec![FieldDesc {
                        type_name: Some(ProtoFqn::parse(".ed.Inner")),
                        features: FeatureSet {
                            message_encoding: Some(MessageEncoding::Delimited),
                            ..FeatureSet::default()
                        },
                        ..desc::field("inner", 1, FieldType::Message)
                    }],
                    ..desc::message("Outer")
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
                fields: vec![FieldDesc {
                    type_name: Some(ProtoFqn::parse(".p2.G")),
                    ..desc::field("g", 1, FieldType::Group)
                }],
                nested_messages: vec![desc::message("G")],
                ..desc::message("M")
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
                fields: vec![
                    FieldDesc {
                        label: FieldLabel::Repeated,
                        packed: Some(true),
                        ..desc::field("packed_nums", 1, FieldType::Int32)
                    },
                    FieldDesc {
                        label: FieldLabel::Repeated,
                        ..desc::field("expanded_nums", 2, FieldType::Int32)
                    },
                ],
                ..desc::message("M")
            }],
            vec![],
        )];
        let file_set = resolve(&arena, &files).unwrap();
        let m = lookup_message(&file_set, ".p2.M").unwrap();
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
                fields: vec![FieldDesc {
                    label: FieldLabel::Repeated,
                    type_name: Some(ProtoFqn::parse(".example.Holder.AttributesEntry")),
                    ..desc::field("attributes", 1, FieldType::Message)
                }],
                nested_messages: vec![desc::map_entry(
                    "AttributesEntry",
                    desc::field("key", 1, FieldType::String),
                    desc::field("value", 2, FieldType::Int32),
                )],
                ..desc::message("Holder")
            }],
            vec![],
        )];
        let file_set = resolve(&arena, &files).unwrap();
        let holder = lookup_message(&file_set, ".example.Holder").unwrap();
        let entry = lookup_message(&file_set, ".example.Holder.AttributesEntry").unwrap();
        assert!(entry.is_map_entry());
        let field = holder.fields().next().unwrap();
        assert_eq!(field.occurrence(), FieldOccurrence::Map);
        assert!(ptr::eq(field.type_ref().as_message().unwrap(), entry));
    }
}
