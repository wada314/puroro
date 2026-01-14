//! Integration tests for lazy parser implementation using PersonLazyImpl
//! Phase 1: age field (scalar integer)
//! Phase 2: scores field (repeated integer)
//! Phase 3: address field (scalar message field)
//! Phase 4: addresses field (repeated message field)
//!
//! Note on repeated field lazy parsing:
//! Currently, the getters (scores(), addresses()) parse all fields before returning.
//! True lazy parsing (parsing elements on-demand when accessed via iterator) is deferred
//! and needs further design discussion. For now, we test correctness of the parsing results
//! but note that true laziness is not yet implemented.

use ::allocator_extras::Global;
use sandbox::generated::person::PersonLazyImpl;

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

/// Encode a length-delimited field (field number, value bytes)
/// Used for string fields and message fields
fn encode_length_delimited_field(field_number: u32, value_bytes: &[u8]) -> Vec<u8> {
    let mut bytes = encode_field_tag(field_number, 2); // wire type 2 (length-delimited)
    bytes.extend_from_slice(&encode_varint(value_bytes.len() as u64)); // length prefix
    bytes.extend_from_slice(value_bytes); // value bytes
    bytes
}

/// Encode a string field (field number, string value)
fn encode_string_field(field_number: u32, value: &str) -> Vec<u8> {
    encode_length_delimited_field(field_number, value.as_bytes())
}

/// Encode an Address message
/// Returns the encoded Address message bytes (not wrapped in a Person.address field)
/// Only encodes non-empty/non-zero fields
fn encode_address_message(street: &str, city: &str, zip_code: i32) -> Vec<u8> {
    let mut bytes = Vec::new();
    // Field 1: street (string) - only encode if non-empty
    if !street.is_empty() {
        bytes.extend_from_slice(&encode_string_field(1, street));
    }
    // Field 2: city (string) - only encode if non-empty
    if !city.is_empty() {
        bytes.extend_from_slice(&encode_string_field(2, city));
    }
    // Field 3: zip_code (varint) - only encode if non-zero
    if zip_code != 0 {
        let mut zip_bytes = encode_field_tag(3, 0); // field 3, wire type 0 (varint)
        zip_bytes.extend_from_slice(&encode_varint(zip_code as u64));
        bytes.extend_from_slice(&zip_bytes);
    }
    bytes
}

/// Encode a Person.address field (field number 6, Address message)
fn encode_address_field(street: &str, city: &str, zip_code: i32) -> Vec<u8> {
    let address_bytes = encode_address_message(street, city, zip_code);
    encode_length_delimited_field(6, &address_bytes) // field 6 (address), wire type 2
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
    assert_eq!(scores.iter().next().unwrap(), 85);
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
    let scores_vec: Vec<i32> = scores.iter().collect();
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
    let scores_vec: Vec<i32> = scores.iter().collect();
    assert_eq!(scores_vec, vec![85, 90]);
}

// ============================================================================
// Phase 3 Tests: Scalar Message Field (address)
// ============================================================================

#[test]
fn test_person_lazy_address_field_simple() {
    // Test message with address field containing street="Main St", city="New York", zip_code=10001
    let encoded = encode_address_field("Main St", "New York", 10001);

    let person_rc = PersonLazyImpl::new(&encoded, Global);
    let address_opt = person_rc.address();

    // Address should be present
    assert!(address_opt.is_some());
    let address = address_opt.unwrap();

    // Check Address fields
    assert_eq!(*address.street(), "Main St");
    assert_eq!(*address.city(), "New York");
    assert_eq!(address.zip_code(), 10001);
}

#[test]
fn test_person_lazy_address_field_empty() {
    // Test message with no address field - should return None
    let empty_message = vec![];
    let person_rc = PersonLazyImpl::new(&empty_message, Global);
    let address_opt = person_rc.address();

    assert!(address_opt.is_none());
}

#[test]
fn test_person_lazy_address_field_multiple_slices() {
    // Test that multiple address field occurrences are concatenated
    // This tests the key property of scalar message fields: all occurrences are concatenated
    //
    // First address slice: street="Main St"
    // Second address slice: city="New York", zip_code=10001
    // Expected: address should contain both slices concatenated

    // First slice: only street field
    let first_slice = encode_address_message("Main St", "", 0);
    let first_address_field = encode_length_delimited_field(6, &first_slice);

    // Second slice: city and zip_code fields
    let second_slice = encode_address_message("", "New York", 10001);
    let second_address_field = encode_length_delimited_field(6, &second_slice);

    // Combine both
    let mut encoded = first_address_field;
    encoded.extend_from_slice(&second_address_field);

    let person_rc = PersonLazyImpl::new(&encoded, Global);
    let address_opt = person_rc.address();

    // Address should be present
    assert!(address_opt.is_some());
    let address = address_opt.unwrap();

    // Check that both slices are concatenated
    assert_eq!(*address.street(), "Main St");
    assert_eq!(*address.city(), "New York");
    assert_eq!(address.zip_code(), 10001);
}

#[test]
fn test_person_lazy_address_and_age_together() {
    // Test message with both age and address fields
    let mut encoded = Vec::new();
    encoded.extend_from_slice(&encode_age_field(30));
    encoded.extend_from_slice(&encode_address_field("Oak Ave", "Boston", 02115));

    let person_rc = PersonLazyImpl::new(&encoded, Global);

    // Check age
    assert_eq!(person_rc.age(), 30);

    // Check address
    let address_opt = person_rc.address();
    assert!(address_opt.is_some());
    let address = address_opt.unwrap();
    assert_eq!(*address.street(), "Oak Ave");
    assert_eq!(*address.city(), "Boston");
    assert_eq!(address.zip_code(), 02115);
}
