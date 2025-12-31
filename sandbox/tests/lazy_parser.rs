//! Tests for lazy parser implementation
//! Phase 1: age field (scalar integer)
//! Phase 2: scores field (repeated integer)

use sandbox::generated::lazy_parser::{decode_varint, parse_varint};
use sandbox::generated::person::PersonLazyImpl;
use ::allocator_extras::Global;

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

/// Encode a varint field (field number 2, value 30) for testing
fn encode_age_field(age: i32) -> Vec<u8> {
    let mut bytes = encode_field_tag(2, 0); // field 2, wire type 0 (varint)
    bytes.extend_from_slice(&encode_varint(age as u64));
    bytes
}

/// Encode a repeated varint field (field number 10) for testing
fn encode_score_field(score: i32) -> Vec<u8> {
    let mut bytes = encode_field_tag(10, 0); // field 10, wire type 0 (varint)
    bytes.extend_from_slice(&encode_varint(score as u64));
    bytes
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

#[test]
fn test_encode_age_field() {
    // Encode age = 30
    // Field tag: 0x10 (field 2, wire type 0)
    // Value: 0x1E (30)
    let bytes = encode_age_field(30);
    assert_eq!(bytes, vec![0x10, 0x1E]);
}

#[test]
fn test_person_lazy_age_field() {
    // Encode a simple message with age = 30
    let encoded = encode_age_field(30);
    
    // Create PersonLazyImpl from encoded bytes
    let person_rc = PersonLazyImpl::new(&encoded, Global);
    
    // Access age field - should parse and return 30
    let age = person_rc.age();
    assert_eq!(age, 30);
}

#[test]
fn test_person_lazy_age_field_multiple_occurrences() {
    // Test that later occurrences overwrite earlier ones
    // Encode: age = 25, then age = 30
    // Expected: age should be 30 (last value)
    let mut encoded = encode_age_field(25);
    encoded.extend_from_slice(&encode_age_field(30));
    
    let person_rc = PersonLazyImpl::new(&encoded, Global);
    let age = person_rc.age();
    assert_eq!(age, 30); // Should be the last value
}

#[test]
fn test_person_lazy_age_field_default() {
    // Test message with no age field - should return default value (0)
    let empty_message = vec![];
    let person_rc = PersonLazyImpl::new(&empty_message, Global);
    let age = person_rc.age();
    assert_eq!(age, 0); // Default value for i32
}

// ============================================================================
// Phase 2 Tests: Repeated Integer Field (scores)
// ============================================================================

#[test]
fn test_person_lazy_scores_field_single() {
    // Test message with a single score = 85
    let encoded = encode_score_field(85);
    
    let person_rc = PersonLazyImpl::new(&encoded, Global);
    let scores = person_rc.scores();
    
    // Check that we have one score
    assert_eq!(scores.iter().count(), 1);
    assert_eq!(scores.iter().next().copied().unwrap(), 85);
}

#[test]
fn test_person_lazy_scores_field_multiple() {
    // Test message with multiple scores: 10, 20, 30
    let mut encoded = Vec::new();
    encoded.extend_from_slice(&encode_score_field(10));
    encoded.extend_from_slice(&encode_score_field(20));
    encoded.extend_from_slice(&encode_score_field(30));
    
    let person_rc = PersonLazyImpl::new(&encoded, Global);
    let scores = person_rc.scores();
    
    // Check that we have three scores in order
    let scores_vec: Vec<i32> = scores.iter().copied().collect();
    assert_eq!(scores_vec, vec![10, 20, 30]);
}

#[test]
fn test_person_lazy_scores_field_empty() {
    // Test message with no scores field - should return empty list
    let empty_message = vec![];
    let person_rc = PersonLazyImpl::new(&empty_message, Global);
    let scores = person_rc.scores();
    
    assert!(scores.iter().next().is_none());
}

#[test]
fn test_person_lazy_scores_and_age_together() {
    // Test message with both age and scores fields
    let mut encoded = Vec::new();
    encoded.extend_from_slice(&encode_age_field(30));
    encoded.extend_from_slice(&encode_score_field(85));
    encoded.extend_from_slice(&encode_score_field(90));
    
    let person_rc = PersonLazyImpl::new(&encoded, Global);
    
    // Check age
    assert_eq!(person_rc.age(), 30);
    
    // Check scores
    let scores = person_rc.scores();
    let scores_vec: Vec<i32> = scores.iter().copied().collect();
    assert_eq!(scores_vec, vec![85, 90]);
}

