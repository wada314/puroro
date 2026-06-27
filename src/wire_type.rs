/// The six wire types defined by the Protocol Buffers specification.
///
/// Each wire type determines how many bytes follow a tag in the wire format.
/// See: <https://protobuf.dev/programming-guides/encoding/#structure>
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WireType {
    /// Used for: int32, int64, uint32, uint64, sint32, sint64, bool, enum
    Varint = 0,
    /// Used for: fixed64, sfixed64, double
    I64 = 1,
    /// Used for: string, bytes, embedded messages, packed repeated fields
    Len = 2,
    /// Deprecated group start — not generated, but must be parseable for forward compat.
    SGroup = 3,
    /// Deprecated group end.
    EGroup = 4,
    /// Used for: fixed32, sfixed32, float
    I32 = 5,
}

impl WireType {
    /// Decode a raw 3-bit value from a tag into a `WireType`.
    pub fn from_raw(raw: u8) -> Result<Self, crate::error::DecodeError> {
        match raw {
            0 => Ok(WireType::Varint),
            1 => Ok(WireType::I64),
            2 => Ok(WireType::Len),
            3 => Ok(WireType::SGroup),
            4 => Ok(WireType::EGroup),
            5 => Ok(WireType::I32),
            _ => Err(crate::error::DecodeError::InvalidTag),
        }
    }
}
