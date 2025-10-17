//! Basic integration tests for our API design.

use sandbox::generated::person::{Person, PersonImpl};

#[test]
fn test_person_creation() {
    let person = PersonImpl::new();
    assert_eq!(person.name(), "");
    assert_eq!(person.age(), 0);
    assert_eq!(person.email(), "");

    // Initially, no fields should be marked as "set"
    assert!(!person.has_name());
    assert!(!person.has_age());
    assert!(!person.has_email());
}

#[test]
fn test_person_default() {
    let person = PersonImpl::default();
    assert_eq!(person.name(), "");
    assert_eq!(person.age(), 0);
    assert_eq!(person.email(), "");
}

#[test]
fn test_person_setters() {
    let mut person = PersonImpl::new();

    person.set_name("Alice");
    person.set_age(30);
    person.set_email("alice@example.com");

    assert_eq!(person.name(), "Alice");
    assert_eq!(person.age(), 30);
    assert_eq!(person.email(), "alice@example.com");

    // After setting, fields should be marked as "set"
    assert!(person.has_name());
    assert!(person.has_age());
    assert!(person.has_email());
}

#[test]
fn test_person_clear() {
    let mut person = PersonImpl::new();

    person.set_name("Bob");
    person.set_age(25);
    assert!(person.has_name());
    assert!(person.has_age());

    person.clear_name();
    person.clear_age();

    assert_eq!(person.name(), "");
    assert_eq!(person.age(), 0);
    assert!(!person.has_name());
    assert!(!person.has_age());
}

#[test]
fn test_person_clone() {
    let mut person = PersonImpl::new();
    person.set_name("Charlie");
    person.set_age(35);

    let cloned = person.clone();
    assert_eq!(person, cloned);
    assert_eq!(cloned.name(), "Charlie");
    assert_eq!(cloned.age(), 35);
    assert!(cloned.has_name());
    assert!(cloned.has_age());
}

#[test]
fn test_person_trait_usage() {
    // Test that we can use the Person trait for generic code
    fn print_person_name(p: &impl Person) -> String {
        p.name().to_string()
    }

    let mut person = PersonImpl::new();
    person.set_name("Dave");

    assert_eq!(print_person_name(&person), "Dave");
}

#[test]
fn test_person_into_string() {
    let mut person = PersonImpl::new();

    // Test that Into<String> works for set_name
    person.set_name("Eve".to_string());
    assert_eq!(person.name(), "Eve");

    person.set_name("Frank"); // &str should also work
    assert_eq!(person.name(), "Frank");
}
