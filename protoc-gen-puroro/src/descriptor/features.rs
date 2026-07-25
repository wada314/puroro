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
