//! Editions [`FeatureSet`](https://protobuf.dev/editions/overview/) subset.
//!
//! Mirrors `google.protobuf.FeatureSet` fields that protoc may set on file /
//! field / enum options. Unset members are [`None`] (inherit from a parent scope
//! or edition defaults during resolve).

use ::derive_more::TryFrom;

/// Resolved / declared Editions features (`google.protobuf.FeatureSet`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FeatureSet {
    pub field_presence: Option<FieldPresence>,
    pub enum_type: Option<EnumType>,
    pub repeated_field_encoding: Option<RepeatedFieldEncoding>,
    pub utf8_validation: Option<Utf8Validation>,
    pub message_encoding: Option<MessageEncoding>,
    pub json_format: Option<JsonFormat>,
    pub enforce_naming_style: Option<EnforceNamingStyle>,
    pub default_symbol_visibility: Option<DefaultSymbolVisibility>,
}

impl FeatureSet {
    pub fn is_empty(&self) -> bool {
        self.field_presence.is_none()
            && self.enum_type.is_none()
            && self.repeated_field_encoding.is_none()
            && self.utf8_validation.is_none()
            && self.message_encoding.is_none()
            && self.json_format.is_none()
            && self.enforce_naming_style.is_none()
            && self.default_symbol_visibility.is_none()
    }

    /// Overlay `over` onto `self`: each `Some` in `over` wins.
    pub fn overlay(self, over: &Self) -> Self {
        Self {
            field_presence: over.field_presence.or(self.field_presence),
            enum_type: over.enum_type.or(self.enum_type),
            repeated_field_encoding: over
                .repeated_field_encoding
                .or(self.repeated_field_encoding),
            utf8_validation: over.utf8_validation.or(self.utf8_validation),
            message_encoding: over.message_encoding.or(self.message_encoding),
            json_format: over.json_format.or(self.json_format),
            enforce_naming_style: over.enforce_naming_style.or(self.enforce_naming_style),
            default_symbol_visibility: over
                .default_symbol_visibility
                .or(self.default_symbol_visibility),
        }
    }

    /// Edition defaults after applying the cumulative `edition_defaults` chain
    /// up through the given edition (proto3-like open enums / packed / …, then
    /// edition-2023+ explicit presence, then 2024 naming / visibility).
    pub fn defaults_for_edition(edition: super::Edition) -> Self {
        let mut features = Self {
            field_presence: Some(FieldPresence::Explicit),
            enum_type: Some(EnumType::Open),
            repeated_field_encoding: Some(RepeatedFieldEncoding::Packed),
            utf8_validation: Some(Utf8Validation::Verify),
            message_encoding: Some(MessageEncoding::LengthPrefixed),
            json_format: Some(JsonFormat::Allow),
            enforce_naming_style: Some(EnforceNamingStyle::StyleLegacy),
            default_symbol_visibility: Some(DefaultSymbolVisibility::ExportAll),
        };
        match edition {
            super::Edition::Edition2023 => {}
            super::Edition::Edition2024 => {
                features.enforce_naming_style = Some(EnforceNamingStyle::Style2024);
                features.default_symbol_visibility = Some(DefaultSymbolVisibility::ExportTopLevel);
            }
        }
        features
    }

    /// Trap explicit overrides of features resolve / codegen do not apply yet.
    ///
    /// Consumed today: `field_presence`, `enum_type`, `repeated_field_encoding`,
    /// `utf8_validation`, and `message_encoding` (LENGTH_PREFIXED only).
    pub fn reject_unimplemented_overrides(&self, context: &str) {
        if self.json_format.is_some() {
            unimplemented!(
                "editions features.json_format override is not implemented yet ({context})"
            );
        }
        if self.enforce_naming_style.is_some() {
            unimplemented!(
                "editions features.enforce_naming_style override is not implemented yet ({context})"
            );
        }
        if self.default_symbol_visibility.is_some() {
            unimplemented!(
                "editions features.default_symbol_visibility override is not implemented yet ({context})"
            );
        }
    }

    /// File-scope features that are not applied anywhere in resolve/codegen yet.
    pub fn apply_or_trap_for_file(&self) {
        assert!(
            matches!(self.json_format, Some(JsonFormat::Allow)),
            "editions features.json_format={:?}: only ALLOW is assumed until codegen reads this feature",
            self.json_format
        );
        // TODO: emit JSON mapping policy from this feature.

        assert!(
            matches!(
                self.enforce_naming_style,
                Some(EnforceNamingStyle::StyleLegacy) | Some(EnforceNamingStyle::Style2024)
            ),
            "editions features.enforce_naming_style={:?} is not implemented yet",
            self.enforce_naming_style
        );
        // TODO: apply naming-style checks when validating / emitting idents.

        assert!(
            matches!(
                self.default_symbol_visibility,
                Some(DefaultSymbolVisibility::ExportAll)
                    | Some(DefaultSymbolVisibility::ExportTopLevel)
            ),
            "editions features.default_symbol_visibility={:?} is not implemented yet",
            self.default_symbol_visibility
        );
        // TODO: apply default symbol visibility when emitting modules.
    }
}

/// `FeatureSet.field_presence`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFrom)]
#[try_from(repr)]
#[repr(i32)]
pub enum FieldPresence {
    Explicit = 1,
    Implicit = 2,
    LegacyRequired = 3,
}

/// `FeatureSet.enum_type` (targets: file, enum).
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFrom)]
#[try_from(repr)]
#[repr(i32)]
pub enum EnumType {
    Open = 1,
    Closed = 2,
}

/// `FeatureSet.repeated_field_encoding` (targets: file, field).
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFrom)]
#[try_from(repr)]
#[repr(i32)]
pub enum RepeatedFieldEncoding {
    Packed = 1,
    Expanded = 2,
}

/// `FeatureSet.utf8_validation` (wire value 1 is reserved; targets: file, field).
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFrom)]
#[try_from(repr)]
#[repr(i32)]
pub enum Utf8Validation {
    Verify = 2,
    None = 3,
}

/// `FeatureSet.message_encoding` (targets: file, field).
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFrom)]
#[try_from(repr)]
#[repr(i32)]
pub enum MessageEncoding {
    LengthPrefixed = 1,
    Delimited = 2,
}

/// `FeatureSet.json_format`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFrom)]
#[try_from(repr)]
#[repr(i32)]
pub enum JsonFormat {
    Allow = 1,
    LegacyBestEffort = 2,
}

/// `FeatureSet.enforce_naming_style` (edition 2024+).
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFrom)]
#[try_from(repr)]
#[repr(i32)]
pub enum EnforceNamingStyle {
    Style2024 = 1,
    StyleLegacy = 2,
}

/// `FeatureSet.default_symbol_visibility` (edition 2024+).
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFrom)]
#[try_from(repr)]
#[repr(i32)]
pub enum DefaultSymbolVisibility {
    ExportAll = 1,
    ExportTopLevel = 2,
    LocalAll = 3,
    Strict = 4,
}
