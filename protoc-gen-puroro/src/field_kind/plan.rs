//! Build a [`MessagePlan`] from a resolved message.

use super::{
    CatalogLayout, CatalogPresence, FieldKind, RepeatedEncodingKind, WireTypeKind,
    field_number_const, value_bit_const,
};
use crate::error::{Error, Result};
use crate::resolved::{Field, FieldOccurrence, Message, SingularPresence};
use ::std::collections::HashMap;

/// Per-message catalog plan: members in struct order + total presence bits.
#[derive(Debug)]
pub struct MessagePlan<'a> {
    message: &'a Message<'a>,
    members: Vec<MessageMember<'a>>,
    /// Bits consumed in `MessageCommon` presence storage (presence + bool values).
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
/// Custom defaults, map entries, and editions `enum_type` / packed overrides are
/// not read yet (defaults: no `HasDefault` param, enums `Open`, packable
/// repeated → `Packed`).
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
        // Real oneof members only — proto3 `optional` uses a synthetic oneof
        // index but resolves to Explicit presence, not Oneof.
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
    let wire = WireTypeKind::from_type_ref(field.type_ref());
    let field_const = field_number_const(field.name());

    if field.number() <= 0 {
        return Err(Error::Codegen(format!(
            "field `{}` has non-positive number {}",
            field.name(),
            field.number()
        )));
    }

    let kind = match field.occurrence() {
        FieldOccurrence::Repeated => {
            let encoding = if wire.is_packable() {
                // Editions `repeated_field_encoding` is not on the resolved field
                // yet; match the current feature trap (PACKED assumed).
                RepeatedEncodingKind::Packed
            } else {
                RepeatedEncodingKind::Expanded
            };
            FieldKind::Repeated { wire, encoding }
        }
        FieldOccurrence::Singular(presence) => {
            let catalog_presence = CatalogPresence::from_singular(presence, field.name(), next_bit);
            let layout = if wire.is_bool() {
                let value_bit = *next_bit;
                *next_bit += 1;
                CatalogLayout::BitPacked {
                    value_bit,
                    bit_const: value_bit_const(field.name()),
                }
            } else {
                CatalogLayout::Inline
            };
            FieldKind::Singular {
                wire,
                presence: catalog_presence,
                layout,
            }
        }
    };

    // Map fields are not distinguished in the descriptor IR yet; they appear as
    // singular messages. Emission must reject or special-case later.

    Ok(PlannedField {
        field,
        field_const,
        kind,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::descriptor::features::FeatureSet;
    use crate::descriptor::{
        FieldDesc, FieldLabel, FieldType, MessageDesc, OneofDesc, ProtoFile, ProtoFqn, Syntax,
    };
    use crate::field_kind::{CatalogLayout, CatalogPresence, EnumOpenness, presence_byte_len};
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
        }])];
        let set = resolve(&arena, &files).unwrap();
        let msg = set
            .lookup(".example.Address")
            .unwrap()
            .as_message()
            .unwrap();
        let plan = plan_message(msg).unwrap();
        assert_eq!(plan.bit_count(), 4);
        assert_eq!(presence_byte_len(4), 1);
        assert_eq!(plan.members().len(), 4);

        let MessageMember::Field(street) = &plan.members()[0] else {
            panic!("expected field");
        };
        assert_eq!(street.field_const(), "FIELD_STREET");
        match street.kind() {
            FieldKind::Singular {
                wire: WireTypeKind::String,
                presence: CatalogPresence::Explicit { bit: 0, bit_const },
                layout: CatalogLayout::Inline,
            } => assert_eq!(bit_const, "BIT_STREET"),
            other => panic!("unexpected kind: {other:?}"),
        }

        let MessageMember::Field(postal) = &plan.members()[2] else {
            panic!("expected field");
        };
        assert_eq!(postal.field_const(), "FIELD_POSTAL_CODE");
        match postal.kind() {
            FieldKind::Singular {
                wire: WireTypeKind::Fixed32,
                presence: CatalogPresence::Explicit { bit: 2, bit_const },
                layout: CatalogLayout::Inline,
            } => assert_eq!(bit_const, "BIT_POSTAL_CODE"),
            other => panic!("unexpected kind: {other:?}"),
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
        }])];
        let set = resolve(&arena, &files).unwrap();
        let msg = set.lookup(".example.Task").unwrap().as_message().unwrap();
        let plan = plan_message(msg).unwrap();

        // score: no bits
        // email_address (oneof string): no bits
        // done: value bit 0
        // flag: presence 1 + value 2
        // urgent (oneof bool): value bit 3
        assert_eq!(plan.bit_count(), 4);

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
        match notification.variants()[1].kind() {
            FieldKind::Singular {
                wire: WireTypeKind::Bool,
                presence: CatalogPresence::Oneof,
                layout:
                    CatalogLayout::BitPacked {
                        value_bit: 3,
                        bit_const,
                    },
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
                        value_bit: 0,
                        bit_const,
                    },
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
                        bit: 1,
                        bit_const: presence_const,
                    },
                layout:
                    CatalogLayout::BitPacked {
                        value_bit: 2,
                        bit_const: value_const,
                    },
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
                wire: WireTypeKind::String,
                encoding: RepeatedEncodingKind::Expanded,
            }
        );
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
            }],
            enums: vec![EnumDesc {
                name: "Status".into(),
                values: vec![EnumValueDesc {
                    name: "STATUS_UNSPECIFIED".into(),
                    number: 0,
                }],
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
            } => assert_eq!(*openness, EnumOpenness::Open),
            other => panic!("{other:?}"),
        }
    }
}
