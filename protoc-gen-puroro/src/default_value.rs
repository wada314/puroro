//! Interpret `FieldDescriptorProto.default_value` text from protoc.
//!
//! Formats follow the comments on `default_value` in `descriptor.proto`:
//! numeric / bool / string (already decoded) / bytes (C-escaped) / enum name.

use crate::case::to_pascal_case;
use crate::error::{Error, Result};
use crate::field_kind::WireTypeKind;
use crate::resolved::Enum;

/// Message-local default marker when the proto default is not the type zero.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomDefault {
    /// e.g. `MaxRetriesDefault`
    pub marker_name: String,
    pub lit: DefaultLit,
}

/// Typed default payload ready for `HasDefault::DEFAULT` emission.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DefaultLit {
    Bool(bool),
    I32(i32),
    I64(i64),
    U32(u32),
    U64(u64),
    /// IEEE bits of an `f32` (avoids NaN `PartialEq` issues).
    F32Bits(u32),
    /// IEEE bits of an `f64`.
    F64Bits(u64),
    Str(String),
    Bytes(Vec<u8>),
    Enum {
        value_name: String,
        number: i32,
    },
}

/// Parse a descriptor default for a singular scalar / enum field.
///
/// Returns `Ok(None)` when the value equals the protobuf type zero (caller keeps
/// `ProtoDefault`). Message / group wires are rejected.
pub fn interpret_custom_default(
    field_name: &str,
    raw: &str,
    wire: &WireTypeKind<'_>,
) -> Result<Option<CustomDefault>> {
    let lit = parse_default_lit(field_name, raw, wire)?;
    if is_type_zero(&lit, wire) {
        return Ok(None);
    }
    Ok(Some(CustomDefault {
        marker_name: format!("{}Default", to_pascal_case(field_name)),
        lit,
    }))
}

fn parse_default_lit(field_name: &str, raw: &str, wire: &WireTypeKind<'_>) -> Result<DefaultLit> {
    match wire {
        WireTypeKind::Bool => parse_bool(field_name, raw).map(DefaultLit::Bool),
        WireTypeKind::Int32 | WireTypeKind::SInt32 | WireTypeKind::SFixed32 => {
            parse_i32(field_name, raw).map(DefaultLit::I32)
        }
        WireTypeKind::Int64 | WireTypeKind::SInt64 | WireTypeKind::SFixed64 => {
            parse_i64(field_name, raw).map(DefaultLit::I64)
        }
        WireTypeKind::UInt32 | WireTypeKind::Fixed32 => {
            parse_u32(field_name, raw).map(DefaultLit::U32)
        }
        WireTypeKind::UInt64 | WireTypeKind::Fixed64 => {
            parse_u64(field_name, raw).map(DefaultLit::U64)
        }
        WireTypeKind::Float => parse_f32(field_name, raw).map(|f| DefaultLit::F32Bits(f.to_bits())),
        WireTypeKind::Double => {
            parse_f64(field_name, raw).map(|f| DefaultLit::F64Bits(f.to_bits()))
        }
        WireTypeKind::String { .. } => Ok(DefaultLit::Str(raw.to_owned())),
        WireTypeKind::Bytes { .. } => unescape_c_bytes(field_name, raw).map(DefaultLit::Bytes),
        WireTypeKind::Enum { ty, .. } => parse_enum(field_name, raw, ty),
        WireTypeKind::Message(_) => Err(Error::Codegen(format!(
            "field `{field_name}`: message fields cannot have default values"
        ))),
    }
}

fn is_type_zero(lit: &DefaultLit, wire: &WireTypeKind<'_>) -> bool {
    match lit {
        DefaultLit::Bool(v) => !*v,
        DefaultLit::I32(v) => *v == 0,
        DefaultLit::I64(v) => *v == 0,
        DefaultLit::U32(v) => *v == 0,
        DefaultLit::U64(v) => *v == 0,
        DefaultLit::F32Bits(bits) => *bits == 0.0f32.to_bits(),
        DefaultLit::F64Bits(bits) => *bits == 0.0f64.to_bits(),
        DefaultLit::Str(s) => s.is_empty(),
        DefaultLit::Bytes(b) => b.is_empty(),
        DefaultLit::Enum { value_name, .. } => match wire {
            WireTypeKind::Enum { ty, .. } => ty
                .values()
                .next()
                .is_some_and(|first| first.name() == value_name),
            _ => false,
        },
    }
}

fn parse_bool(field_name: &str, raw: &str) -> Result<bool> {
    match raw {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(Error::Codegen(format!(
            "field `{field_name}`: invalid bool default `{raw}`"
        ))),
    }
}

fn parse_i32(field_name: &str, raw: &str) -> Result<i32> {
    parse_signed(field_name, raw)?.try_into().map_err(|_| {
        Error::Codegen(format!(
            "field `{field_name}`: i32 default out of range `{raw}`"
        ))
    })
}

fn parse_i64(field_name: &str, raw: &str) -> Result<i64> {
    parse_signed(field_name, raw)
}

fn parse_u32(field_name: &str, raw: &str) -> Result<u32> {
    parse_unsigned(field_name, raw)?.try_into().map_err(|_| {
        Error::Codegen(format!(
            "field `{field_name}`: u32 default out of range `{raw}`"
        ))
    })
}

fn parse_u64(field_name: &str, raw: &str) -> Result<u64> {
    parse_unsigned(field_name, raw)
}

/// Decimal / hex (`0x`) / octal (`0…`) like `strtol` base 0.
fn parse_signed(field_name: &str, raw: &str) -> Result<i64> {
    let (neg, digits) = match raw.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, raw.strip_prefix('+').unwrap_or(raw)),
    };
    let value = parse_unsigned(field_name, digits)?;
    let signed = i64::try_from(value).map_err(|_| {
        Error::Codegen(format!(
            "field `{field_name}`: integer default out of range `{raw}`"
        ))
    })?;
    if neg {
        signed.checked_neg().ok_or_else(|| {
            Error::Codegen(format!(
                "field `{field_name}`: integer default out of range `{raw}`"
            ))
        })
    } else {
        Ok(signed)
    }
}

fn parse_unsigned(field_name: &str, raw: &str) -> Result<u64> {
    if let Some(hex) = raw.strip_prefix("0x").or_else(|| raw.strip_prefix("0X")) {
        return u64::from_str_radix(hex, 16).map_err(|_| {
            Error::Codegen(format!("field `{field_name}`: invalid hex default `{raw}`"))
        });
    }
    if raw.len() > 1 && raw.starts_with('0') && raw.bytes().all(|b| b.is_ascii_digit()) {
        return u64::from_str_radix(raw, 8).map_err(|_| {
            Error::Codegen(format!(
                "field `{field_name}`: invalid octal default `{raw}`"
            ))
        });
    }
    raw.parse::<u64>().map_err(|_| {
        Error::Codegen(format!(
            "field `{field_name}`: invalid integer default `{raw}`"
        ))
    })
}

fn parse_f32(field_name: &str, raw: &str) -> Result<f32> {
    parse_float(field_name, raw).map(|v| v as f32)
}

fn parse_f64(field_name: &str, raw: &str) -> Result<f64> {
    parse_float(field_name, raw)
}

fn parse_float(field_name: &str, raw: &str) -> Result<f64> {
    match raw {
        "nan" | "NaN" => Ok(f64::NAN),
        "inf" | "Infinity" => Ok(f64::INFINITY),
        "-inf" | "-Infinity" => Ok(f64::NEG_INFINITY),
        _ => raw.parse::<f64>().map_err(|_| {
            Error::Codegen(format!(
                "field `{field_name}`: invalid float default `{raw}`"
            ))
        }),
    }
}

fn parse_enum(field_name: &str, raw: &str, ty: &Enum<'_>) -> Result<DefaultLit> {
    for v in ty.values() {
        if v.name() == raw {
            return Ok(DefaultLit::Enum {
                value_name: v.name().to_owned(),
                number: v.number(),
            });
        }
    }
    Err(Error::Codegen(format!(
        "field `{field_name}`: unknown enum default `{raw}` for `{}`",
        ty.name()
    )))
}

/// Reverse of protoc `CEscape` used for `bytes` defaults.
fn unescape_c_bytes(field_name: &str, raw: &str) -> Result<Vec<u8>> {
    let bytes = raw.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] != b'\\' {
            out.push(bytes[i]);
            i += 1;
            continue;
        }
        i += 1;
        if i >= bytes.len() {
            return Err(Error::Codegen(format!(
                "field `{field_name}`: trailing backslash in bytes default"
            )));
        }
        match bytes[i] {
            b'n' => out.push(b'\n'),
            b'r' => out.push(b'\r'),
            b't' => out.push(b'\t'),
            b'\\' => out.push(b'\\'),
            b'\'' => out.push(b'\''),
            b'"' => out.push(b'"'),
            b'a' => out.push(0x07),
            b'b' => out.push(0x08),
            b'f' => out.push(0x0c),
            b'v' => out.push(0x0b),
            b'?' => out.push(b'?'),
            d if (b'0'..=b'7').contains(&d) => {
                let mut value = u32::from(d - b'0');
                let mut consumed = 1;
                while consumed < 3 {
                    let next = i + consumed;
                    if next >= bytes.len() || !(b'0'..=b'7').contains(&bytes[next]) {
                        break;
                    }
                    value = (value << 3) | u32::from(bytes[next] - b'0');
                    consumed += 1;
                }
                if value > 0xff {
                    return Err(Error::Codegen(format!(
                        "field `{field_name}`: octal escape out of range in bytes default"
                    )));
                }
                out.push(value as u8);
                i += consumed - 1;
            }
            b'x' => {
                if i + 2 >= bytes.len() {
                    return Err(Error::Codegen(format!(
                        "field `{field_name}`: truncated hex escape in bytes default"
                    )));
                }
                let hi = hex_nibble(bytes[i + 1]).ok_or_else(|| {
                    Error::Codegen(format!(
                        "field `{field_name}`: invalid hex escape in bytes default"
                    ))
                })?;
                let lo = hex_nibble(bytes[i + 2]).ok_or_else(|| {
                    Error::Codegen(format!(
                        "field `{field_name}`: invalid hex escape in bytes default"
                    ))
                })?;
                out.push((hi << 4) | lo);
                i += 2;
            }
            other => {
                return Err(Error::Codegen(format!(
                    "field `{field_name}`: unknown escape `\\{}` in bytes default",
                    char::from(other)
                )));
            }
        }
        i += 1;
    }
    Ok(out)
}

fn hex_nibble(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::descriptor::features::Utf8Validation;
    use crate::field_kind::WireTypeKind;

    #[test]
    fn int_type_zero_is_none() {
        let d = interpret_custom_default("n", "0", &WireTypeKind::Int32).unwrap();
        assert!(d.is_none());
    }

    #[test]
    fn int_custom() {
        let d = interpret_custom_default("max_retries", "3", &WireTypeKind::Int32)
            .unwrap()
            .unwrap();
        assert_eq!(d.marker_name, "MaxRetriesDefault");
        assert_eq!(d.lit, DefaultLit::I32(3));
    }

    #[test]
    fn negative_and_hex() {
        assert_eq!(
            interpret_custom_default("webhook_id", "-1", &WireTypeKind::Int32)
                .unwrap()
                .unwrap()
                .lit,
            DefaultLit::I32(-1)
        );
        assert_eq!(
            interpret_custom_default("x", "0x10", &WireTypeKind::UInt32)
                .unwrap()
                .unwrap()
                .lit,
            DefaultLit::U32(16)
        );
    }

    #[test]
    fn string_empty_is_type_zero() {
        assert!(
            interpret_custom_default(
                "title",
                "",
                &WireTypeKind::String {
                    utf8: Utf8Validation::Verify
                }
            )
            .unwrap()
            .is_none()
        );
    }

    #[test]
    fn string_keeps_decoded_text() {
        let d = interpret_custom_default(
            "title",
            "hello\n",
            &WireTypeKind::String {
                utf8: Utf8Validation::Verify,
            },
        )
        .unwrap()
        .unwrap();
        assert_eq!(d.lit, DefaultLit::Str("hello\n".into()));
    }

    #[test]
    fn bytes_c_unescape() {
        let d = interpret_custom_default(
            "payload",
            r"a\000b\n\377",
            &WireTypeKind::Bytes {
                utf8: Utf8Validation::None,
            },
        )
        .unwrap()
        .unwrap();
        assert_eq!(d.lit, DefaultLit::Bytes(vec![b'a', 0, b'b', b'\n', 0xff]));
    }
}
