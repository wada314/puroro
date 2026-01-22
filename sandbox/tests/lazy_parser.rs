// Copyright 2021 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
use ::puroro::protobuf_core::{Field, FieldNumber, FieldValue, WriteExtProtobuf};
use sandbox::generated::person::PersonLazyImpl;

fn build_varint_field(field_number: u32, value: i32) -> Vec<u8> {
    let field = Field::<Vec<u8>>::new(
        FieldNumber::try_from(field_number).unwrap(),
        FieldValue::from_int32(value),
    );
    let mut bytes = Vec::new();
    bytes.write_protobuf_field(&field).unwrap();
    bytes
}

fn build_length_delimited_field(field_number: u32, value_bytes: &[u8]) -> Vec<u8> {
    let field = Field::<Vec<u8>>::new(
        FieldNumber::try_from(field_number).unwrap(),
        FieldValue::Len(value_bytes.to_vec()),
    );
    let mut bytes = Vec::new();
    bytes.write_protobuf_field(&field).unwrap();
    bytes
}

fn build_string_field(field_number: u32, value: &str) -> Vec<u8> {
    build_length_delimited_field(field_number, value.as_bytes())
}

/// Build an Address message
/// Returns the encoded Address message bytes (not wrapped in a Person.address field)
/// Only encodes non-empty/non-zero fields
fn build_address_message(street: &str, city: &str, zip_code: i32) -> Vec<u8> {
    let mut bytes = Vec::new();
    // Field 1: street (string) - only encode if non-empty
    if !street.is_empty() {
        bytes.extend_from_slice(&build_string_field(1, street));
    }
    // Field 2: city (string) - only encode if non-empty
    if !city.is_empty() {
        bytes.extend_from_slice(&build_string_field(2, city));
    }
    // Field 3: zip_code (varint) - only encode if non-zero
    if zip_code != 0 {
        bytes.extend_from_slice(&build_varint_field(3, zip_code));
    }
    bytes
}

/// Build a Person.address field (field number 6, Address message)
fn build_address_field(street: &str, city: &str, zip_code: i32) -> Vec<u8> {
    let address_bytes = build_address_message(street, city, zip_code);
    build_length_delimited_field(6, &address_bytes) // field 6 (address), wire type 2
}

#[test]
fn test_person_lazy_age_field() {
    // Encode a simple message with age = 30
    let encoded = build_varint_field(2, 30);

    // Create PersonLazyImpl from encoded bytes
    let person_rc = PersonLazyImpl::new(&encoded, Global, None);

    // Access age field - should parse and return 30
    let age = person_rc.age();
    assert_eq!(age, 30);
}

#[test]
fn test_person_lazy_age_field_multiple_occurrences() {
    // Test that later occurrences overwrite earlier ones
    // Encode: age = 25, then age = 30
    // Expected: age should be 30 (last value)
    let mut encoded = build_varint_field(2, 25);
    encoded.extend_from_slice(&build_varint_field(2, 30));

    let person_rc = PersonLazyImpl::new(&encoded, Global, None);
    let age = person_rc.age();
    assert_eq!(age, 30); // Should be the last value
}

#[test]
fn test_person_lazy_age_field_default() {
    // Test message with no age field - should return default value (0)
    let empty_message = vec![];
    let person_rc = PersonLazyImpl::new(&empty_message, Global, None);
    let age = person_rc.age();
    assert_eq!(age, 0); // Default value for i32
}

// ============================================================================
// Phase 2 Tests: Repeated Integer Field (scores)
// ============================================================================

#[test]
fn test_person_lazy_scores_field_single() {
    // Test message with a single score = 85
    let encoded = build_varint_field(10, 85);

    let person_rc = PersonLazyImpl::new(&encoded, Global, None);
    let scores = person_rc.scores();

    // Check that we have one score
    assert_eq!(scores.iter().count(), 1);
    assert_eq!(scores.iter().next().unwrap(), 85);
}

#[test]
fn test_person_lazy_scores_field_multiple() {
    // Test message with multiple scores: 10, 20, 30
    let mut encoded = Vec::new();
    encoded.extend_from_slice(&build_varint_field(10, 10));
    encoded.extend_from_slice(&build_varint_field(10, 20));
    encoded.extend_from_slice(&build_varint_field(10, 30));

    let person_rc = PersonLazyImpl::new(&encoded, Global, None);
    let scores = person_rc.scores();

    // Check that we have three scores in order
    let scores_vec: Vec<i32> = scores.iter().collect();
    assert_eq!(scores_vec, vec![10, 20, 30]);
}

#[test]
fn test_person_lazy_scores_field_empty() {
    // Test message with no scores field - should return empty list
    let empty_message = vec![];
    let person_rc = PersonLazyImpl::new(&empty_message, Global, None);
    let scores = person_rc.scores();

    assert!(scores.iter().next().is_none());
}

#[test]
fn test_person_lazy_scores_and_age_together() {
    // Test message with both age and scores fields
    let mut encoded = Vec::new();
    encoded.extend_from_slice(&build_varint_field(2, 30));
    encoded.extend_from_slice(&build_varint_field(10, 85));
    encoded.extend_from_slice(&build_varint_field(10, 90));

    let person_rc = PersonLazyImpl::new(&encoded, Global, None);

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
    let encoded = build_address_field("Main St", "New York", 10001);

    let person_rc = PersonLazyImpl::new(&encoded, Global, None);
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
    let person_rc = PersonLazyImpl::new(&empty_message, Global, None);
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
    let first_slice = build_address_message("Main St", "", 0);
    let first_address_field = build_length_delimited_field(6, &first_slice);

    // Second slice: city and zip_code fields
    let second_slice = build_address_message("", "New York", 10001);
    let second_address_field = build_length_delimited_field(6, &second_slice);

    // Combine both
    let mut encoded = first_address_field;
    encoded.extend_from_slice(&second_address_field);

    let person_rc = PersonLazyImpl::new(&encoded, Global, None);
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
    encoded.extend_from_slice(&build_varint_field(2, 30));
    encoded.extend_from_slice(&build_address_field("Oak Ave", "Boston", 02115));

    let person_rc = PersonLazyImpl::new(&encoded, Global, None);

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
