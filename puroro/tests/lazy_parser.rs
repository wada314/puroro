//! Tests for lazy parser implementation
//!
//! Tests for the core lazy parser infrastructure:
//! - Varint decoding functions
//! - Field tag parsing
//! - Basic wire format parsing utilities

use puroro::lazy_parser::{decode_varint, parse_varint};

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
fn test_decode_varint() {
    // Test simple varint: 30 = 0x1E (single byte, no continuation)
    let bytes = vec![0x1E];
    let (value, consumed) = decode_varint(&bytes).unwrap();
    assert_eq!(value, 30);
    assert_eq!(consumed, 1);

    // Test multi-byte varint: 300 = 0xAC 0x02
    // 300 in binary: 100101100
    // Split into 7-bit chunks: 0101100 (0x2C) and 0000010 (0x02)
    // With continuation bit: 0xAC (0x2C | 0x80) and 0x02
    let bytes = vec![0xAC, 0x02];
    let (value, consumed) = decode_varint(&bytes).unwrap();
    assert_eq!(value, 300);
    assert_eq!(consumed, 2);
}

#[test]
fn test_parse_varint() {
    // Test simple varint: 30
    let bytes = vec![0x1E];
    let value = parse_varint(&bytes).unwrap();
    assert_eq!(value, 30);
}

#[test]
fn test_encode_field_tag() {
    // Field 2, wire type 0 (varint)
    // Tag = (2 << 3) | 0 = 16 = 0x10
    let bytes = encode_field_tag(2, 0);
    assert_eq!(bytes, vec![0x10]);
}
