//! Basic integration tests for our API design.

use sandbox::generated::person::Person;

#[test]
fn test_person_creation() {
    let person = Person::new();
    assert_eq!(person.name, "");
    assert_eq!(person.age, 0);
    assert_eq!(person.email, "");
}

#[test]
fn test_person_default() {
    let person = Person::default();
    assert_eq!(person.name, "");
    assert_eq!(person.age, 0);
    assert_eq!(person.email, "");
}

#[test]
fn test_person_mutation() {
    let mut person = Person::new();
    person.name = "Alice".to_string();
    person.age = 30;
    person.email = "alice@example.com".to_string();
    
    assert_eq!(person.name, "Alice");
    assert_eq!(person.age, 30);
    assert_eq!(person.email, "alice@example.com");
}

#[test]
fn test_person_clone() {
    let mut person = Person::new();
    person.name = "Bob".to_string();
    
    let cloned = person.clone();
    assert_eq!(person, cloned);
}

