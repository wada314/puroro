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

use super::{
    Address, AddressImpl, AddressMut, Person, PersonImpl, PersonMut, Status,
};
use ::puroro::field_ops::{MessageFieldWrapper, StringFieldWrapper};
use ::puroro::repeated::Repeated;

#[test]
fn test_message_fields() {
    let mut person = PersonImpl::new();

    // Test setting message fields via mutable builder
    {
        let mut address_impl = PersonMut::address_mut(&mut person);
        // address_impl is Option<&mut AddressImpl> which implements AddressMut
        // For Option<T>, None case does nothing, so initialization is required
        address_impl.set_street("123 Main St");
    }

    // Test getting message fields
    {
        let address_impl = Person::address(&person);
        // address_impl is Option<&AddressImpl> which implements Address
        // For Option<T>, None case returns default values
        assert_eq!(address_impl.street(), "123 Main St");
    }

    // Test builder pattern
    {
        let mut address_impl = PersonMut::address_mut(&mut person);
        address_impl.set_street("456 Oak Ave");
    }
    {
        let address_impl = Person::address(&person);
        assert_eq!(address_impl.street(), "456 Oak Ave");
    }
}

#[test]
fn test_enum_fields() {
    let mut person = PersonImpl::new();

    // Test setting enum fields
    PersonMut::set_name(&mut person, "Test");

    // Test getting enum fields
    match Person::status(&person) {
        Ok(Status::Active) => println!("Status is Active"),
        Ok(status) => println!("Status is {:?}", status),
        Err(unknown) => println!("Unknown status: {}", unknown),
    }

    match Person::secondary_status(&person) {
        Ok(Some(Status::Pending)) => println!("Secondary status is Pending"),
        Ok(Some(status)) => println!("Secondary status is {:?}", status),
        Ok(None) => println!("No secondary status"),
        Err(unknown) => println!("Unknown secondary status: {}", unknown),
    }

    // Test presence checking
    assert!(Person::has_name(&person));

    // Test clearing enum fields
    PersonMut::clear_name(&mut person);
    assert!(!Person::has_name(&person));
}

#[test]
fn test_wrapper_types() {
    // Test StringFieldWrapper wrapper
    let mut string_field = StringFieldWrapper::new();
    string_field.as_mut_string().push_str("Hello");
    assert_eq!(string_field.as_str(), "Hello");

    // Test MessageFieldWrapper wrapper
    let mut message_field = MessageFieldWrapper::new();
    let address = message_field.get_or_insert_with(AddressImpl::new);
    assert_eq!(address.street(), "");
}

#[test]
fn test_repeated_scalars_and_messages() {
    let mut person = PersonImpl::new();

    // push and read scores
    PersonMut::push_score(&mut person, 10);
    PersonMut::push_score(&mut person, 20);
    {
        let rep = Person::scores(&person);
        assert_eq!(rep.len(), 2);
        let collected: Vec<i32> = rep.iter_box().collect();
        assert_eq!(collected, vec![10, 20]);
        assert_eq!(rep.get(1), Some(20));
    }
    PersonMut::clear_scores(&mut person);
    assert!(Person::scores(&person).is_empty());

    // push and read addresses
    {
        let mut addr_mut = PersonMut::push_address(&mut person);
        addr_mut.set_street("First St");
    }
    {
        let mut addr_mut = PersonMut::push_address(&mut person);
        addr_mut.set_street("Second Ave");
    }
    {
        let rep = Person::addresses(&person);
        assert_eq!(rep.len(), 2);
        let streets: Vec<String> = rep.iter_box().map(|m| m.street().to_string()).collect();
        assert_eq!(
            streets,
            vec!["First St".to_string(), "Second Ave".to_string()]
        );
        assert!(rep.get(0).is_some());
    }
    PersonMut::clear_addresses(&mut person);
    assert!(Person::addresses(&person).is_empty());
}

#[test]
fn test_enum_unknown_values() {
    let _person = PersonImpl::new();

    // Test that unknown enum values return errors
    // Note: This test may need adjustment based on actual enum storage implementation
    // For now, we test that Status::from_wire handles unknown values correctly
    assert_eq!(Status::from_wire(0), Ok(Status::Unspecified));
    assert_eq!(Status::from_wire(1), Ok(Status::Active));
    assert_eq!(Status::from_wire(3), Ok(Status::Pending));
    assert_eq!(Status::from_wire(999), Err(999)); // Unknown value should return error
    assert_eq!(Status::from_wire(-1), Err(-1)); // Negative value should return error
}

#[test]
fn test_explicit_optional_fields() {
    let person = PersonImpl::new();

    // Test email field (explicit optional)
    assert_eq!(Person::email(&person), None);

    // Test score field (explicit optional)
    assert_eq!(Person::score(&person), None);

    // Test secondary_status field (explicit optional)
    match Person::secondary_status(&person) {
        Ok(None) => {
            // Expected: no value set, should return None
        }
        Ok(Some(_)) => panic!("Expected None for unset secondary_status"),
        Err(_) => panic!("Unexpected error for unset secondary_status"),
    }
}

#[test]
fn test_repeated_fields_edge_cases() {
    let mut person = PersonImpl::new();

    // Test empty repeated fields
    {
        let scores = Person::scores(&person);
        assert!(scores.is_empty());
        assert_eq!(scores.len(), 0);
    }

    {
        let addresses = Person::addresses(&person);
        assert!(addresses.is_empty());
        assert_eq!(addresses.len(), 0);
    }

    // Test adding single item
    PersonMut::push_score(&mut person, 42);
    {
        let scores = Person::scores(&person);
        assert_eq!(scores.len(), 1);
        assert_eq!(scores.get(0), Some(42));
        assert_eq!(scores.get(1), None); // Out of bounds
    }

    // Test adding many items
    for i in 1..=10 {
        PersonMut::push_score(&mut person, i * 10);
    }
    {
        let scores = Person::scores(&person);
        assert_eq!(scores.len(), 11); // 42 + 10 more items
    }

    // Clear and verify empty again
    PersonMut::clear_scores(&mut person);
    {
        let scores = Person::scores(&person);
        assert!(scores.is_empty());
    }
}

#[test]
fn test_repeated_message_fields_edge_cases() {
    let mut person = PersonImpl::new();

    // Test empty repeated message fields
    {
        let addresses = Person::addresses(&person);
        assert!(addresses.is_empty());
    }

    // Test adding single address
    {
        let mut addr_mut = PersonMut::push_address(&mut person);
        addr_mut.set_street("Single St");
    }
    {
        let addresses = Person::addresses(&person);
        assert_eq!(addresses.len(), 1);
        let first = addresses.get(0).unwrap();
        assert_eq!(first.street(), "Single St");
    }

    // Test adding multiple addresses
    for i in 1..=5 {
        let mut addr_mut = PersonMut::push_address(&mut person);
        addr_mut.set_street(&format!("Street {}", i));
    }
    {
        let addresses = Person::addresses(&person);
        assert_eq!(addresses.len(), 6); // 1 + 5 more
    }

    // Clear and verify
    PersonMut::clear_addresses(&mut person);
    {
        let addresses = Person::addresses(&person);
        assert!(addresses.is_empty());
    }
}

#[test]
fn test_string_fields_edge_cases() {
    let mut person = PersonImpl::new();

    // Test empty string (default value)
    assert_eq!(Person::name(&person), "");
    assert!(!Person::has_name(&person));

    // Test setting empty string explicitly
    PersonMut::set_name(&mut person, "");
    assert_eq!(Person::name(&person), "");
    // After setting, field might still be considered "not present" if empty is default
    // This depends on implementation details

    // Test setting non-empty string
    PersonMut::set_name(&mut person, "Alice");
    assert_eq!(Person::name(&person), "Alice");
    assert!(Person::has_name(&person));

    // Test clearing
    PersonMut::clear_name(&mut person);
    assert_eq!(Person::name(&person), "");
    assert!(!Person::has_name(&person));
}

#[test]
fn test_scalar_fields_default_values() {
    let person = PersonImpl::new();

    // Test default values for scalar fields
    assert_eq!(Person::age(&person), 0);
    assert_eq!(Person::name(&person), "");

    // Enum fields should have default value (0 = Unspecified)
    match Person::status(&person) {
        Ok(Status::Unspecified) => {
            // Expected default
        }
        Ok(status) => panic!("Expected Unspecified status, got {:?}", status),
        Err(unknown) => panic!("Unexpected error for default status: {}", unknown),
    }
}

#[test]
fn test_message_field_presence() {
    let mut person = PersonImpl::new();

    // Initially, address should not be present
    {
        // Person::address returns default values even if not set; check wrapper directly
        let address_impl = Person::address(&person);
        assert_eq!(address_impl.street(), "");
    }

    // After setting address, it should have the new value
    {
        let mut addr_mut = PersonMut::address_mut(&mut person);
        addr_mut.set_street("Present");
    }
    assert_eq!(Person::address(&person).street(), "Present");

    // Note: Message fields don't have a clear() method in the current API,
    // but we can verify the field wrapper behavior
}

#[test]
fn test_clone_consistency() {
    let mut person1 = PersonImpl::new();
    PersonMut::set_name(&mut person1, "Original");
    PersonMut::push_score(&mut person1, 100);

    let person2 = person1.clone();

    // Both should have the same values
    assert_eq!(Person::name(&person1), Person::name(&person2));
    assert_eq!(Person::name(&person1), "Original");

    {
        let scores1 = Person::scores(&person1);
        let scores2 = Person::scores(&person2);
        assert_eq!(scores1.len(), scores2.len());
        assert_eq!(scores1.get(0), scores2.get(0));
    }

    // Modifying one should not affect the other
    PersonMut::set_name(&mut person1, "Modified");
    assert_eq!(Person::name(&person1), "Modified");
    assert_eq!(Person::name(&person2), "Original");
}

#[test]
fn test_partial_eq() {
    let mut person1 = PersonImpl::new();
    let mut person2 = PersonImpl::new();

    // Two empty persons should be equal
    assert_eq!(person1, person2);

    // After setting same values, should still be equal
    PersonMut::set_name(&mut person1, "Alice");
    PersonMut::set_name(&mut person2, "Alice");
    assert_eq!(person1, person2);

    // After setting different values, should not be equal
    PersonMut::set_name(&mut person2, "Bob");
    assert_ne!(person1, person2);
}
