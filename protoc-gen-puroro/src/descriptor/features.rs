//! Editions [`FeatureSet`](https://protobuf.dev/editions/overview/) subset.
//!
//! Mirrors `google.protobuf.FeatureSet` fields that protoc may set on file /
//! field options. Unset members are [`None`] (inherit from a parent scope or
//! edition defaults during resolve).

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

    /// Trap explicit overrides of features we do not apply yet (everything except
    /// [`FieldPresence`], which resolve already consumes).
    pub fn reject_unimplemented_overrides(&self, context: &str) {
        if self.enum_type.is_some() {
            unimplemented!(
                "editions features.enum_type override is not implemented yet ({context})"
            );
        }
        if self.repeated_field_encoding.is_some() {
            unimplemented!(
                "editions features.repeated_field_encoding override is not implemented yet ({context})"
            );
        }
        if self.utf8_validation.is_some() {
            unimplemented!(
                "editions features.utf8_validation override is not implemented yet ({context})"
            );
        }
        if self.message_encoding.is_some() {
            unimplemented!(
                "editions features.message_encoding override is not implemented yet ({context})"
            );
        }
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

    /// Trap at the sites where effective (merged) features must be applied.
    ///
    /// Today only [`FieldPresence`] is consumed for singular occurrence. Other
    /// features: edition-default values are asserted (codegen still hardcodes the
    /// same behavior — replace the assert body when wiring emit); any other value
    /// is [`unimplemented!`].
    pub fn apply_or_trap_for_field(
        &self,
        field_label_repeated: bool,
        field_type: super::FieldType,
    ) {
        // field_presence: applied by resolve_occurrence

        if field_label_repeated {
            assert!(
                matches!(
                    self.repeated_field_encoding,
                    Some(RepeatedFieldEncoding::Packed)
                ),
                "editions features.repeated_field_encoding={:?}: only PACKED is assumed until codegen reads this feature",
                self.repeated_field_encoding
            );
            // TODO: emit packed/expanded repeated wire helpers from this feature.
        }

        if field_type == super::FieldType::Enum {
            assert!(
                matches!(self.enum_type, Some(EnumType::Open)),
                "editions features.enum_type={:?}: only OPEN is assumed until codegen reads this feature",
                self.enum_type
            );
            // TODO: emit open/closed enum storage from this feature.
        }

        if matches!(
            field_type,
            super::FieldType::String | super::FieldType::Bytes
        ) {
            assert!(
                matches!(self.utf8_validation, Some(Utf8Validation::Verify)),
                "editions features.utf8_validation={:?}: only VERIFY is assumed until codegen reads this feature",
                self.utf8_validation
            );
            // TODO: emit UTF-8 verification policy from this feature.
        }

        if matches!(
            field_type,
            super::FieldType::Message | super::FieldType::Group
        ) {
            assert!(
                matches!(self.message_encoding, Some(MessageEncoding::LengthPrefixed)),
                "editions features.message_encoding={:?}: only LENGTH_PREFIXED is assumed until codegen reads this feature",
                self.message_encoding
            );
            // TODO: emit length-prefixed / delimited encoding from this feature.
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldPresence {
    Explicit = 1,
    Implicit = 2,
    LegacyRequired = 3,
}

impl FieldPresence {
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            1 => Some(Self::Explicit),
            2 => Some(Self::Implicit),
            3 => Some(Self::LegacyRequired),
            _ => None,
        }
    }
}

/// `FeatureSet.enum_type`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnumType {
    Open = 1,
    Closed = 2,
}

impl EnumType {
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            1 => Some(Self::Open),
            2 => Some(Self::Closed),
            _ => None,
        }
    }
}

/// `FeatureSet.repeated_field_encoding`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepeatedFieldEncoding {
    Packed = 1,
    Expanded = 2,
}

impl RepeatedFieldEncoding {
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            1 => Some(Self::Packed),
            2 => Some(Self::Expanded),
            _ => None,
        }
    }
}

/// `FeatureSet.utf8_validation` (wire value 1 is reserved).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Utf8Validation {
    Verify = 2,
    None = 3,
}

impl Utf8Validation {
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            2 => Some(Self::Verify),
            3 => Some(Self::None),
            _ => None,
        }
    }
}

/// `FeatureSet.message_encoding`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageEncoding {
    LengthPrefixed = 1,
    Delimited = 2,
}

impl MessageEncoding {
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            1 => Some(Self::LengthPrefixed),
            2 => Some(Self::Delimited),
            _ => None,
        }
    }
}

/// `FeatureSet.json_format`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JsonFormat {
    Allow = 1,
    LegacyBestEffort = 2,
}

impl JsonFormat {
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            1 => Some(Self::Allow),
            2 => Some(Self::LegacyBestEffort),
            _ => None,
        }
    }
}

/// `FeatureSet.enforce_naming_style` (edition 2024+).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnforceNamingStyle {
    Style2024 = 1,
    StyleLegacy = 2,
}

impl EnforceNamingStyle {
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            1 => Some(Self::Style2024),
            2 => Some(Self::StyleLegacy),
            _ => None,
        }
    }
}

/// `FeatureSet.default_symbol_visibility` (edition 2024+).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefaultSymbolVisibility {
    ExportAll = 1,
    ExportTopLevel = 2,
    LocalAll = 3,
    Strict = 4,
}

impl DefaultSymbolVisibility {
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            1 => Some(Self::ExportAll),
            2 => Some(Self::ExportTopLevel),
            3 => Some(Self::LocalAll),
            4 => Some(Self::Strict),
            _ => None,
        }
    }
}
