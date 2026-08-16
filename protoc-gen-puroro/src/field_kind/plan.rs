//! Build a [`MessagePlan`] from a resolved message.

use super::{
    CatalogLayout, CatalogPresence, FieldKind, RepeatedEncodingKind, WireTypeKind,
    field_number_const, value_bit_const,
};
use crate::default_value::interpret_custom_default;
use crate::descriptor::{BytesLayout, StringLayout};
use crate::error::{Error, Result};
use crate::resolved::{Field, FieldOccurrence, Message, SingularPresence, TypeRef};
use ::std::collections::HashMap;

/// Per-message catalog plan: members in struct order + total presence bits.
#[derive(Debug)]
pub struct MessagePlan<'a> {
    message: &'a Message<'a>,
    members: Vec<MessageMember<'a>>,
    /// Bits consumed in `MessageCommon` (presence + bool values + string / bytes
    /// SSO heap bits; SSO bit set means heap arm).
    bit_count: usize,
}

/// One direct struct member of the generated message.
#[derive(Debug)]
pub enum MessageMember<'a> {
    Field(PlannedField<'a>),
    Oneof(PlannedOneof<'a>),
}

/// One catalog field (top-level or oneof variant).
#[derive(Debug)]
pub struct PlannedField<'a> {
    field: &'a Field<'a>,
    /// e.g. `FIELD_TITLE`
    field_const: String,
    kind: FieldKind<'a>,
}

/// `OneofSlot` group: variants keep individual [`FieldKind`]s with [`CatalogPresence::Oneof`].
#[derive(Debug)]
pub struct PlannedOneof<'a> {
    name: String,
    variants: Vec<PlannedField<'a>>,
}

impl<'a> MessagePlan<'a> {
    pub fn message(&self) -> &'a Message<'a> {
        self.message
    }

    pub fn members(&self) -> &[MessageMember<'a>] {
        &self.members
    }

    pub fn bit_count(&self) -> usize {
        self.bit_count
    }
}

impl<'a> PlannedField<'a> {
    pub fn field(&self) -> &'a Field<'a> {
        self.field
    }

    pub fn name(&self) -> &str {
        self.field.name()
    }

    pub fn number(&self) -> i32 {
        self.field.number()
    }

    pub fn field_const(&self) -> &str {
        &self.field_const
    }

    pub fn kind(&self) -> &FieldKind<'a> {
        &self.kind
    }
}

impl<'a> PlannedOneof<'a> {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn variants(&self) -> &[PlannedField<'a>] {
        &self.variants
    }
}

/// Plan catalog kinds and bit indices for `message`.
///
/// Bit assignment matches [`sample-generated`](../../../sample-generated/): one
/// ascending field-number pass; each field may take a presence bit then a bool
/// value bit. Struct member order places each oneof group at its lowest field
/// number; remaining fields keep numeric order.
///
/// Map fields become [`FieldKind::Map`]; synthetic map-entry nested messages are
/// still present on the resolved graph but skipped at emit. Editions
/// `enum_type`, `repeated_field_encoding`, and `utf8_validation` come from the
/// resolved field / enum.
pub fn plan_message<'a>(message: &'a Message<'a>) -> Result<MessagePlan<'a>> {
    let mut fields: Vec<&'a Field<'a>> = message.fields().collect();
    fields.sort_by_key(|f| f.number());

    // First pass: assign bits in field-number order (including oneof members).
    let mut next_bit = 0usize;
    let mut planned_by_number: HashMap<i32, PlannedField<'a>> = HashMap::new();
    for field in &fields {
        let planned = plan_field(field, &mut next_bit)?;
        if planned_by_number.insert(field.number(), planned).is_some() {
            return Err(Error::Codegen(format!(
                "duplicate field number {} in `{}`",
                field.number(),
                message.fqn()
            )));
        }
    }

    // Second pass: struct member order (oneof groups collapse to one slot).
    let oneof_names: Vec<String> = message.oneofs().map(|o| o.name().to_owned()).collect();
    let mut emitted_oneofs = vec![false; oneof_names.len()];
    let mut members = Vec::new();

    for field in &fields {
        // Real oneof members only. Proto3 `optional` is stripped in resolve
        // (Explicit presence, no oneof_index).
        if matches!(
            field.occurrence(),
            FieldOccurrence::Singular(SingularPresence::Oneof)
        ) {
            let idx = field.oneof_index().ok_or_else(|| {
                Error::Codegen(format!(
                    "field `{}.{}` has Oneof presence but no oneof_index",
                    message.fqn(),
                    field.name()
                ))
            })?;
            let idx = usize::try_from(idx).map_err(|_| {
                Error::Codegen(format!(
                    "field `{}.{}` has negative oneof_index",
                    message.fqn(),
                    field.name()
                ))
            })?;
            if idx >= oneof_names.len() {
                return Err(Error::Codegen(format!(
                    "field `{}.{}` oneof_index {idx} out of range ({} oneofs)",
                    message.fqn(),
                    field.name(),
                    oneof_names.len()
                )));
            }
            if emitted_oneofs[idx] {
                continue;
            }
            emitted_oneofs[idx] = true;

            let mut variants = Vec::new();
            for f in &fields {
                if matches!(
                    f.occurrence(),
                    FieldOccurrence::Singular(SingularPresence::Oneof)
                ) && f.oneof_index() == Some(idx as i32)
                {
                    variants.push(
                        planned_by_number
                            .remove(&f.number())
                            .expect("field planned in first pass"),
                    );
                }
            }
            members.push(MessageMember::Oneof(PlannedOneof {
                name: oneof_names[idx].clone(),
                variants,
            }));
        } else {
            members.push(MessageMember::Field(
                planned_by_number
                    .remove(&field.number())
                    .expect("field planned in first pass"),
            ));
        }
    }

    debug_assert!(planned_by_number.is_empty());

    Ok(MessagePlan {
        message,
        members,
        bit_count: next_bit,
    })
}

fn plan_field<'a>(field: &'a Field<'a>, next_bit: &mut usize) -> Result<PlannedField<'a>> {
    let field_const = field_number_const(field.name());

    if field.number() <= 0 {
        return Err(Error::Codegen(format!(
            "field `{}` has non-positive number {}",
            field.name(),
            field.number()
        )));
    }

    let kind = match field.occurrence() {
        FieldOccurrence::Map => {
            if field.default_value().is_some() {
                return Err(Error::Codegen(format!(
                    "field `{}`: map fields cannot have default values",
                    field.name()
                )));
            }
            plan_map_field(field)?
        }
        FieldOccurrence::Repeated(encoding) => {
            if field.default_value().is_some() {
                return Err(Error::Codegen(format!(
                    "field `{}`: repeated fields cannot have default values",
                    field.name()
                )));
            }
            let wire = WireTypeKind::from_field(field);
            // Non-packable types cannot use packed wire form regardless of feature.
            let encoding = if wire.is_packable() {
                RepeatedEncodingKind::from_resolved(encoding)
            } else {
                RepeatedEncodingKind::Expanded
            };
            FieldKind::Repeated { wire, encoding }
        }
        FieldOccurrence::Singular(presence) => {
            let wire = WireTypeKind::from_field(field);
            let catalog_presence = CatalogPresence::from_singular(presence, field.name(), next_bit);
            let layout = if wire.is_bool() {
                let value_bit = *next_bit;
                *next_bit += 1;
                CatalogLayout::BitPacked {
                    value_bit,
                    bit_const: value_bit_const(field.name()),
                }
            } else if (wire.is_string() && field.string_layout() != Some(StringLayout::Heap))
                || (wire.is_bytes() && field.bytes_layout() != Some(BytesLayout::Heap))
            {
                let heap_bit = *next_bit;
                *next_bit += 1;
                CatalogLayout::InlineOrHeap {
                    heap_bit,
                    bit_const: super::sso_bit_const(field.name()),
                }
            } else {
                CatalogLayout::Inline
            };
            let custom_default = match field.default_value() {
                Some(raw) => interpret_custom_default(field.name(), raw, &wire)?,
                None => None,
            };
            FieldKind::Singular {
                wire,
                presence: catalog_presence,
                layout,
                custom_default,
            }
        }
    };

    Ok(PlannedField {
        field,
        field_const,
        kind,
    })
}

fn plan_map_field<'a>(field: &'a Field<'a>) -> Result<FieldKind<'a>> {
    let TypeRef::Message(entry) = field.type_ref() else {
        return Err(Error::Codegen(format!(
            "map field `{}` does not resolve to a message type",
            field.name()
        )));
    };
    if !entry.is_map_entry() {
        return Err(Error::Codegen(format!(
            "map field `{}` type `{}` is not a map_entry message",
            field.name(),
            entry.fqn()
        )));
    }

    let mut key_field = None;
    let mut value_field = None;
    for f in entry.fields() {
        match f.number() {
            1 => key_field = Some(f),
            2 => value_field = Some(f),
            n => {
                return Err(Error::Codegen(format!(
                    "map entry `{}` has unexpected field number {n}",
                    entry.fqn()
                )));
            }
        }
    }
    let (Some(key_field), Some(value_field)) = (key_field, value_field) else {
        return Err(Error::Codegen(format!(
            "map entry `{}` must have fields key=1 and value=2",
            entry.fqn()
        )));
    };

    let key = WireTypeKind::from_field(key_field);
    let value = WireTypeKind::from_field(value_field);
    validate_map_key_wire(&key, field.name())?;
    validate_map_value_wire(&value, field.name())?;

    Ok(FieldKind::Map { key, value })
}

fn validate_map_key_wire(key: &WireTypeKind<'_>, field_name: &str) -> Result<()> {
    // Protobuf: integral, bool, or string — not float / bytes / enum / message.
    match key {
        WireTypeKind::Int32
        | WireTypeKind::Int64
        | WireTypeKind::UInt32
        | WireTypeKind::UInt64
        | WireTypeKind::SInt32
        | WireTypeKind::SInt64
        | WireTypeKind::Fixed32
        | WireTypeKind::Fixed64
        | WireTypeKind::SFixed32
        | WireTypeKind::SFixed64
        | WireTypeKind::Bool
        | WireTypeKind::String { .. } => Ok(()),
        other => Err(Error::Codegen(format!(
            "map field `{field_name}`: invalid map key type {other:?}"
        ))),
    }
}

fn validate_map_value_wire(value: &WireTypeKind<'_>, _field_name: &str) -> Result<()> {
    // Any non-map value is allowed (maps-of-maps do not appear in descriptors).
    match value {
        WireTypeKind::Double
        | WireTypeKind::Float
        | WireTypeKind::Int32
        | WireTypeKind::Int64
        | WireTypeKind::UInt32
        | WireTypeKind::UInt64
        | WireTypeKind::SInt32
        | WireTypeKind::SInt64
        | WireTypeKind::Fixed32
        | WireTypeKind::Fixed64
        | WireTypeKind::SFixed32
        | WireTypeKind::SFixed64
        | WireTypeKind::Bool
        | WireTypeKind::String { .. }
        | WireTypeKind::Bytes { .. }
        | WireTypeKind::Enum { .. }
        | WireTypeKind::Message(_) => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::descriptor::features::{
        EnumType, FeatureSet, RepeatedFieldEncoding, Utf8Validation,
    };
    use crate::descriptor::{
        BytesLayout, Edition, FieldDesc, FieldLabel, FieldType, MessageDesc, OneofDesc, ProtoFile,
        ProtoFqn, StringLayout, Syntax,
    };
    use crate::field_kind::{CatalogLayout, CatalogPresence, presence_byte_len};
    use crate::resolved::{Arena, resolve};

    fn proto3_file(messages: Vec<MessageDesc>) -> ProtoFile {
        ProtoFile {
            name: "t.proto".into(),
            package: "example".into(),
            syntax: Syntax::Proto3,
            features: FeatureSet::default(),
            dependency: vec![],
            messages,
            enums: vec![],
        }
    }

    fn field(
        name: &str,
        number: i32,
        type_: FieldType,
        label: FieldLabel,
        proto3_optional: bool,
        oneof_index: Option<i32>,
        type_name: Option<ProtoFqn>,
    ) -> FieldDesc {
        FieldDesc {
            name: name.into(),
            number,
            label,
            type_,
            type_name,
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
    fn empty_message_plan() {
        let arena = Arena::new();
        let files = [proto3_file(vec![MessageDesc {
            name: "Empty".into(),
            fields: vec![],
            nested_messages: vec![],
            nested_enums: vec![],
            oneofs: vec![],
            map_entry: false,
        }])];
        let set = resolve(&arena, &files).unwrap();
        let msg = set.lookup(".example.Empty").unwrap().as_message().unwrap();
        let plan = plan_message(msg).unwrap();
        assert!(plan.members().is_empty());
        assert_eq!(plan.bit_count(), 0);
        assert_eq!(presence_byte_len(plan.bit_count()), 0);
    }

    #[test]
    fn address_like_explicit_scalars() {
        // Matches sample-generated Address bit / field consts.
        let arena = Arena::new();
        let files = [proto3_file(vec![MessageDesc {
            name: "Address".into(),
            fields: vec![
                field(
                    "street",
                    1,
                    FieldType::String,
                    FieldLabel::Optional,
                    true,
                    None,
                    None,
                ),
                field(
                    "city",
                    2,
                    FieldType::String,
                    FieldLabel::Optional,
                    true,
                    None,
                    None,
                ),
                field(
                    "postal_code",
                    3,
                    FieldType::Fixed32,
                    FieldLabel::Optional,
                    true,
                    None,
                    None,
                ),
                field(
                    "latitude",
                    4,
                    FieldType::Double,
                    FieldLabel::Optional,
                    true,
                    None,
                    None,
                ),
            ],
            nested_messages: vec![],
            nested_enums: vec![],
            oneofs: vec![],
            map_entry: false,
        }])];
        let set = resolve(&arena, &files).unwrap();
        let msg = set
            .lookup(".example.Address")
            .unwrap()
            .as_message()
            .unwrap();
        let plan = plan_message(msg).unwrap();
        // street/city: presence + SSO heap bit each; postal_code/latitude: presence only.
        assert_eq!(plan.bit_count(), 6);
        assert_eq!(presence_byte_len(6), 1);
        assert_eq!(plan.members().len(), 4);

        let MessageMember::Field(street) = &plan.members()[0] else {
            panic!("expected field");
        };
        assert_eq!(street.field_const(), "FIELD_STREET");
        match street.kind() {
            FieldKind::Singular {
                wire:
                    WireTypeKind::String {
                        utf8: Utf8Validation::Verify,
                    },
                presence: CatalogPresence::Explicit { bit: 0, bit_const },
                layout:
                    CatalogLayout::InlineOrHeap {
                        heap_bit: 1,
                        bit_const: sso_const,
                    },
                custom_default: None,
            } => {
                assert_eq!(bit_const, "BIT_STREET");
                assert_eq!(sso_const, "BIT_STREET_SSO");
            }
            other => panic!("unexpected kind: {other:?}"),
        }

        let MessageMember::Field(postal) = &plan.members()[2] else {
            panic!("expected field");
        };
        assert_eq!(postal.field_const(), "FIELD_POSTAL_CODE");
        match postal.kind() {
            FieldKind::Singular {
                wire: WireTypeKind::Fixed32,
                presence: CatalogPresence::Explicit { bit: 4, bit_const },
                layout: CatalogLayout::Inline,
                custom_default: None,
            } => assert_eq!(bit_const, "BIT_POSTAL_CODE"),
            other => panic!("unexpected kind: {other:?}"),
        }
    }

    #[test]
    fn string_layout_unspecified_uses_sso_default() {
        let arena = Arena::new();
        let mut body = field(
            "body",
            1,
            FieldType::String,
            FieldLabel::Optional,
            true,
            None,
            None,
        );
        body.string_layout = Some(StringLayout::Unspecified);
        let files = [proto3_file(vec![MessageDesc {
            name: "M".into(),
            fields: vec![body],
            nested_messages: vec![],
            nested_enums: vec![],
            oneofs: vec![],
            map_entry: false,
        }])];
        let set = resolve(&arena, &files).unwrap();
        let msg = set.lookup(".example.M").unwrap().as_message().unwrap();
        let plan = plan_message(msg).unwrap();
        // Presence + SSO heap bit — same as an absent option.
        assert_eq!(plan.bit_count(), 2);
        let MessageMember::Field(body) = &plan.members()[0] else {
            panic!("expected field");
        };
        match body.kind() {
            FieldKind::Singular {
                layout: CatalogLayout::InlineOrHeap { heap_bit: 1, .. },
                ..
            } => {}
            other => panic!("expected SSO InlineOrHeap, got {other:?}"),
        }
    }

    #[test]
    fn string_layout_heap_uses_inline_heap_layout() {
        let arena = Arena::new();
        let mut body = field(
            "body",
            1,
            FieldType::String,
            FieldLabel::Optional,
            true,
            None,
            None,
        );
        body.string_layout = Some(StringLayout::Heap);
        let files = [proto3_file(vec![MessageDesc {
            name: "M".into(),
            fields: vec![body],
            nested_messages: vec![],
            nested_enums: vec![],
            oneofs: vec![],
            map_entry: false,
        }])];
        let set = resolve(&arena, &files).unwrap();
        let msg = set.lookup(".example.M").unwrap().as_message().unwrap();
        let plan = plan_message(msg).unwrap();
        // Presence bit only — no SSO heap bit.
        assert_eq!(plan.bit_count(), 1);
        let MessageMember::Field(body) = &plan.members()[0] else {
            panic!("expected field");
        };
        match body.kind() {
            FieldKind::Singular {
                layout: CatalogLayout::Inline,
                ..
            } => {}
            other => panic!("expected Inline heap string, got {other:?}"),
        }
    }

    #[test]
    fn bytes_layout_unspecified_uses_sso_default() {
        let arena = Arena::new();
        let mut body = field(
            "body",
            1,
            FieldType::Bytes,
            FieldLabel::Optional,
            true,
            None,
            None,
        );
        body.bytes_layout = Some(BytesLayout::Unspecified);
        let files = [proto3_file(vec![MessageDesc {
            name: "M".into(),
            fields: vec![body],
            nested_messages: vec![],
            nested_enums: vec![],
            oneofs: vec![],
            map_entry: false,
        }])];
        let set = resolve(&arena, &files).unwrap();
        let msg = set.lookup(".example.M").unwrap().as_message().unwrap();
        let plan = plan_message(msg).unwrap();
        assert_eq!(plan.bit_count(), 2);
        let MessageMember::Field(body) = &plan.members()[0] else {
            panic!("expected field");
        };
        match body.kind() {
            FieldKind::Singular {
                layout: CatalogLayout::InlineOrHeap { heap_bit: 1, .. },
                ..
            } => {}
            other => panic!("expected SSO InlineOrHeap, got {other:?}"),
        }
    }

    #[test]
    fn bytes_layout_heap_uses_inline_heap_layout() {
        let arena = Arena::new();
        let mut body = field(
            "body",
            1,
            FieldType::Bytes,
            FieldLabel::Optional,
            true,
            None,
            None,
        );
        body.bytes_layout = Some(BytesLayout::Heap);
        let files = [proto3_file(vec![MessageDesc {
            name: "M".into(),
            fields: vec![body],
            nested_messages: vec![],
            nested_enums: vec![],
            oneofs: vec![],
            map_entry: false,
        }])];
        let set = resolve(&arena, &files).unwrap();
        let msg = set.lookup(".example.M").unwrap().as_message().unwrap();
        let plan = plan_message(msg).unwrap();
        assert_eq!(plan.bit_count(), 1);
        let MessageMember::Field(body) = &plan.members()[0] else {
            panic!("expected field");
        };
        match body.kind() {
            FieldKind::Singular {
                layout: CatalogLayout::Inline,
                ..
            } => {}
            other => panic!("expected Inline heap bytes, got {other:?}"),
        }
    }

    #[test]
    fn task_like_bits_and_oneof_placement() {
        // Subset of sample-generated Task: score, done, flag, notification.urgent,
        // plus a string oneof variant before done — checks bit order and oneof slot.
        let arena = Arena::new();
        let files = [proto3_file(vec![MessageDesc {
            name: "Task".into(),
            fields: vec![
                field(
                    "score",
                    2,
                    FieldType::Int32,
                    FieldLabel::Optional,
                    false,
                    None,
                    None,
                ),
                field(
                    "email_address",
                    12,
                    FieldType::String,
                    FieldLabel::Optional,
                    false,
                    Some(0),
                    None,
                ),
                field(
                    "done",
                    16,
                    FieldType::Bool,
                    FieldLabel::Optional,
                    false,
                    None,
                    None,
                ),
                field(
                    "flag",
                    17,
                    FieldType::Bool,
                    FieldLabel::Optional,
                    true,
                    None,
                    None,
                ),
                field(
                    "urgent",
                    18,
                    FieldType::Bool,
                    FieldLabel::Optional,
                    false,
                    Some(0),
                    None,
                ),
            ],
            nested_messages: vec![],
            nested_enums: vec![],
            oneofs: vec![OneofDesc {
                name: "notification".into(),
            }],
            map_entry: false,
        }])];
        let set = resolve(&arena, &files).unwrap();
        let msg = set.lookup(".example.Task").unwrap().as_message().unwrap();
        let plan = plan_message(msg).unwrap();

        // score: no bits
        // email_address (oneof string): SSO heap bit 0
        // done: value bit 1
        // flag: presence 2 + value 3
        // urgent (oneof bool): value bit 4
        assert_eq!(plan.bit_count(), 5);

        assert_eq!(plan.members().len(), 4);
        assert!(matches!(plan.members()[0], MessageMember::Field(_)));
        assert!(
            matches!(&plan.members()[1], MessageMember::Oneof(o) if o.name() == "notification")
        );
        assert!(matches!(plan.members()[2], MessageMember::Field(_)));
        assert!(matches!(plan.members()[3], MessageMember::Field(_)));

        let MessageMember::Oneof(notification) = &plan.members()[1] else {
            panic!("expected oneof");
        };
        assert_eq!(notification.variants().len(), 2);
        assert_eq!(notification.variants()[0].name(), "email_address");
        assert_eq!(notification.variants()[1].name(), "urgent");
        match notification.variants()[0].kind() {
            FieldKind::Singular {
                wire: WireTypeKind::String { .. },
                presence: CatalogPresence::Oneof,
                layout:
                    CatalogLayout::InlineOrHeap {
                        heap_bit: 0,
                        bit_const,
                    },
                custom_default: None,
            } => assert_eq!(bit_const, "BIT_EMAIL_ADDRESS_SSO"),
            other => panic!("unexpected email_address kind: {other:?}"),
        }
        match notification.variants()[1].kind() {
            FieldKind::Singular {
                wire: WireTypeKind::Bool,
                presence: CatalogPresence::Oneof,
                layout:
                    CatalogLayout::BitPacked {
                        value_bit: 4,
                        bit_const,
                    },
                custom_default: None,
            } => assert_eq!(bit_const, "BIT_URGENT_VALUE"),
            other => panic!("unexpected urgent kind: {other:?}"),
        }

        let MessageMember::Field(done) = &plan.members()[2] else {
            panic!("expected done");
        };
        match done.kind() {
            FieldKind::Singular {
                wire: WireTypeKind::Bool,
                presence: CatalogPresence::Implicit,
                layout:
                    CatalogLayout::BitPacked {
                        value_bit: 1,
                        bit_const,
                    },
                custom_default: None,
            } => assert_eq!(bit_const, "BIT_DONE_VALUE"),
            other => panic!("unexpected done kind: {other:?}"),
        }

        let MessageMember::Field(flag) = &plan.members()[3] else {
            panic!("expected flag");
        };
        match flag.kind() {
            FieldKind::Singular {
                wire: WireTypeKind::Bool,
                presence:
                    CatalogPresence::Explicit {
                        bit: 2,
                        bit_const: presence_const,
                    },
                layout:
                    CatalogLayout::BitPacked {
                        value_bit: 3,
                        bit_const: value_const,
                    },
                custom_default: None,
            } => {
                assert_eq!(presence_const, "BIT_FLAG");
                assert_eq!(value_const, "BIT_FLAG_VALUE");
            }
            other => panic!("unexpected flag kind: {other:?}"),
        }
    }

    #[test]
    fn repeated_packable_defaults_to_packed() {
        let arena = Arena::new();
        let files = [proto3_file(vec![MessageDesc {
            name: "R".into(),
            fields: vec![
                field(
                    "tag_ids",
                    1,
                    FieldType::Int32,
                    FieldLabel::Repeated,
                    false,
                    None,
                    None,
                ),
                field(
                    "labels",
                    2,
                    FieldType::String,
                    FieldLabel::Repeated,
                    false,
                    None,
                    None,
                ),
            ],
            nested_messages: vec![],
            nested_enums: vec![],
            oneofs: vec![],
            map_entry: false,
        }])];
        let set = resolve(&arena, &files).unwrap();
        let msg = set.lookup(".example.R").unwrap().as_message().unwrap();
        let plan = plan_message(msg).unwrap();
        let MessageMember::Field(tags) = &plan.members()[0] else {
            panic!();
        };
        assert_eq!(
            tags.kind(),
            &FieldKind::Repeated {
                wire: WireTypeKind::Int32,
                encoding: RepeatedEncodingKind::Packed,
            }
        );
        let MessageMember::Field(labels) = &plan.members()[1] else {
            panic!();
        };
        assert_eq!(
            labels.kind(),
            &FieldKind::Repeated {
                wire: WireTypeKind::String {
                    utf8: Utf8Validation::Verify,
                },
                encoding: RepeatedEncodingKind::Expanded,
            }
        );
    }

    #[test]
    fn editions_features_flow_into_plan() {
        use crate::descriptor::{EnumDesc, EnumValueDesc};

        let arena = Arena::new();
        let file = ProtoFile {
            name: "t.proto".into(),
            package: "example".into(),
            syntax: Syntax::Editions(Edition::Edition2023),
            features: FeatureSet::default(),
            dependency: vec![],
            messages: vec![MessageDesc {
                name: "T".into(),
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
                    FieldDesc {
                        name: "priority".into(),
                        number: 3,
                        label: FieldLabel::Optional,
                        type_: FieldType::Enum,
                        type_name: Some(ProtoFqn::parse(".example.Priority")),
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
            enums: vec![EnumDesc {
                name: "Priority".into(),
                values: vec![EnumValueDesc {
                    name: "PRIORITY_UNSPECIFIED".into(),
                    number: 0,
                }],
                features: FeatureSet {
                    enum_type: Some(EnumType::Closed),
                    ..FeatureSet::default()
                },
            }],
        };
        let set = resolve(&arena, &[file]).unwrap();
        let msg = set.lookup(".example.T").unwrap().as_message().unwrap();
        let plan = plan_message(msg).unwrap();

        let MessageMember::Field(scores) = &plan.members()[0] else {
            panic!();
        };
        assert_eq!(
            scores.kind(),
            &FieldKind::Repeated {
                wire: WireTypeKind::Int32,
                encoding: RepeatedEncodingKind::Expanded,
            }
        );

        let MessageMember::Field(title) = &plan.members()[1] else {
            panic!();
        };
        match title.kind() {
            FieldKind::Singular {
                wire:
                    WireTypeKind::String {
                        utf8: Utf8Validation::None,
                    },
                ..
            } => {}
            other => panic!("{other:?}"),
        }

        let MessageMember::Field(priority) = &plan.members()[2] else {
            panic!();
        };
        match priority.kind() {
            FieldKind::Singular {
                wire:
                    WireTypeKind::Enum {
                        openness: EnumType::Closed,
                        ..
                    },
                ..
            } => {}
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn proto3_optional_synthetic_oneof_is_not_a_group() {
        let arena = Arena::new();
        let files = [proto3_file(vec![MessageDesc {
            name: "T".into(),
            fields: vec![field(
                "score",
                1,
                FieldType::Int32,
                FieldLabel::Optional,
                true,
                Some(0),
                None,
            )],
            nested_messages: vec![],
            nested_enums: vec![],
            oneofs: vec![OneofDesc {
                name: "_score".into(),
            }],
            map_entry: false,
        }])];
        let set = resolve(&arena, &files).unwrap();
        let msg = set.lookup(".example.T").unwrap().as_message().unwrap();
        let plan = plan_message(msg).unwrap();
        assert_eq!(plan.members().len(), 1);
        let MessageMember::Field(score) = &plan.members()[0] else {
            panic!("proto3 optional must stay a top-level field, not OneofSlot");
        };
        match score.kind() {
            FieldKind::Singular {
                presence: CatalogPresence::Explicit { bit: 0, .. },
                layout: CatalogLayout::Inline,
                ..
            } => {}
            other => panic!("{other:?}"),
        }
        assert_eq!(plan.bit_count(), 1);
    }

    #[test]
    fn enum_field_assumes_open() {
        use crate::descriptor::{EnumDesc, EnumValueDesc};

        let arena = Arena::new();
        let files = [ProtoFile {
            name: "t.proto".into(),
            package: "example".into(),
            syntax: Syntax::Proto3,
            features: FeatureSet::default(),
            dependency: vec![],
            messages: vec![MessageDesc {
                name: "T".into(),
                fields: vec![field(
                    "status",
                    1,
                    FieldType::Enum,
                    FieldLabel::Optional,
                    false,
                    None,
                    Some(ProtoFqn::parse(".example.Status")),
                )],
                nested_messages: vec![],
                nested_enums: vec![],
                oneofs: vec![],
                map_entry: false,
            }],
            enums: vec![EnumDesc {
                name: "Status".into(),
                values: vec![EnumValueDesc {
                    name: "STATUS_UNSPECIFIED".into(),
                    number: 0,
                }],
                features: FeatureSet::default(),
            }],
        }];
        let set = resolve(&arena, &files).unwrap();
        let msg = set.lookup(".example.T").unwrap().as_message().unwrap();
        let plan = plan_message(msg).unwrap();
        let MessageMember::Field(status) = &plan.members()[0] else {
            panic!();
        };
        match status.kind() {
            FieldKind::Singular {
                wire: WireTypeKind::Enum { openness, .. },
                presence: CatalogPresence::Implicit,
                layout: CatalogLayout::Inline,
                ..
            } => assert_eq!(*openness, EnumType::Open),
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn map_string_int32_plans_map_kind() {
        let arena = Arena::new();
        let files = [proto3_file(vec![MessageDesc {
            name: "Holder".into(),
            fields: vec![field(
                "attributes",
                1,
                FieldType::Message,
                FieldLabel::Repeated,
                false,
                None,
                Some(ProtoFqn::parse(".example.Holder.AttributesEntry")),
            )],
            nested_messages: vec![MessageDesc {
                name: "AttributesEntry".into(),
                fields: vec![
                    field(
                        "key",
                        1,
                        FieldType::String,
                        FieldLabel::Optional,
                        false,
                        None,
                        None,
                    ),
                    field(
                        "value",
                        2,
                        FieldType::Int32,
                        FieldLabel::Optional,
                        false,
                        None,
                        None,
                    ),
                ],
                nested_messages: vec![],
                nested_enums: vec![],
                oneofs: vec![],
                map_entry: true,
            }],
            nested_enums: vec![],
            oneofs: vec![],
            map_entry: false,
        }])];
        let set = resolve(&arena, &files).unwrap();
        let msg = set.lookup(".example.Holder").unwrap().as_message().unwrap();
        let plan = plan_message(msg).unwrap();
        assert_eq!(plan.bit_count(), 0);
        let MessageMember::Field(attrs) = &plan.members()[0] else {
            panic!("map must be a top-level field");
        };
        match attrs.kind() {
            FieldKind::Map {
                key: WireTypeKind::String { .. },
                value: WireTypeKind::Int32,
            } => {}
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn map_string_string_plans_map_kind() {
        let arena = Arena::new();
        let files = [proto3_file(vec![MessageDesc {
            name: "Holder".into(),
            fields: vec![field(
                "labels",
                1,
                FieldType::Message,
                FieldLabel::Repeated,
                false,
                None,
                Some(ProtoFqn::parse(".example.Holder.LabelsEntry")),
            )],
            nested_messages: vec![MessageDesc {
                name: "LabelsEntry".into(),
                fields: vec![
                    field(
                        "key",
                        1,
                        FieldType::String,
                        FieldLabel::Optional,
                        false,
                        None,
                        None,
                    ),
                    field(
                        "value",
                        2,
                        FieldType::String,
                        FieldLabel::Optional,
                        false,
                        None,
                        None,
                    ),
                ],
                nested_messages: vec![],
                nested_enums: vec![],
                oneofs: vec![],
                map_entry: true,
            }],
            nested_enums: vec![],
            oneofs: vec![],
            map_entry: false,
        }])];
        let set = resolve(&arena, &files).unwrap();
        let msg = set.lookup(".example.Holder").unwrap().as_message().unwrap();
        let plan = plan_message(msg).unwrap();
        let MessageMember::Field(labels) = &plan.members()[0] else {
            panic!("map must be a top-level field");
        };
        match labels.kind() {
            FieldKind::Map {
                key: WireTypeKind::String { .. },
                value: WireTypeKind::String { .. },
            } => {}
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn map_int32_bool_plans_map_kind() {
        let arena = Arena::new();
        let files = [proto3_file(vec![MessageDesc {
            name: "Holder".into(),
            fields: vec![field(
                "flags",
                1,
                FieldType::Message,
                FieldLabel::Repeated,
                false,
                None,
                Some(ProtoFqn::parse(".example.Holder.FlagsEntry")),
            )],
            nested_messages: vec![MessageDesc {
                name: "FlagsEntry".into(),
                fields: vec![
                    field(
                        "key",
                        1,
                        FieldType::Int32,
                        FieldLabel::Optional,
                        false,
                        None,
                        None,
                    ),
                    field(
                        "value",
                        2,
                        FieldType::Bool,
                        FieldLabel::Optional,
                        false,
                        None,
                        None,
                    ),
                ],
                nested_messages: vec![],
                nested_enums: vec![],
                oneofs: vec![],
                map_entry: true,
            }],
            nested_enums: vec![],
            oneofs: vec![],
            map_entry: false,
        }])];
        let set = resolve(&arena, &files).unwrap();
        let msg = set.lookup(".example.Holder").unwrap().as_message().unwrap();
        let plan = plan_message(msg).unwrap();
        let MessageMember::Field(flags) = &plan.members()[0] else {
            panic!("map must be a top-level field");
        };
        match flags.kind() {
            FieldKind::Map {
                key: WireTypeKind::Int32,
                value: WireTypeKind::Bool,
            } => {}
            other => panic!("{other:?}"),
        }
    }
}
