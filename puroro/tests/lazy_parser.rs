//! Tests for lazy parser implementation
//!
//! Tests for the core lazy parser infrastructure:
//! - Field tag encoding

/// Encode a varint value
fn encode_varint(mut value: u64) -> Vec<u8> {
    let mut bytes = Vec::new();
    loop {
        let mut byte = (value & 0x7F) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        bytes.push(byte);
        if value == 0 {
            break;
        }
    }
    bytes
}

/// Encode a field tag (field_number and wire_type)
fn encode_field_tag(field_number: u32, wire_type: u32) -> Vec<u8> {
    let tag = (field_number << 3) | wire_type;
    encode_varint(tag as u64)
}

#[test]
fn test_encode_field_tag() {
    // Field 2, wire type 0 (varint)
    // Tag = (2 << 3) | 0 = 16 = 0x10
    let bytes = encode_field_tag(2, 0);
    assert_eq!(bytes, vec![0x10]);
}
