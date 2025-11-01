//! Basic integration tests for our API design.

use sandbox::generated::person::{Person, PersonAppend, PersonImpl, PersonMut};

#[test]
fn test_person_creation() {
    let person = PersonImpl::new();
    assert_eq!(Person::name(&person), "");
    assert_eq!(Person::age(&person), 0);
    assert_eq!(Person::email(&person), None);

    // ImplicitOptional fields are present only if not equal to default value
    assert!(!Person::has_name(&person)); // Empty string is default value
    assert!(!Person::has_age(&person)); // 0 is default value
    assert!(!Person::has_email(&person)); // ExplicitOptional field is not set initially
}

#[test]
fn test_person_default() {
    let person = PersonImpl::default();
    assert_eq!(Person::name(&person), "");
    assert_eq!(Person::age(&person), 0);
    assert_eq!(Person::email(&person), None);
}

#[test]
fn test_person_setters() {
    let mut person = PersonImpl::new();

    PersonAppend::set_name(&mut person, "Alice");
    PersonAppend::set_age(&mut person, 30);
    PersonAppend::set_email(&mut person, "alice@example.com");

    assert_eq!(Person::name(&person), "Alice");
    assert_eq!(Person::age(&person), 30);
    assert_eq!(Person::email(&person), Some("alice@example.com"));

    // After setting, fields should be marked as "set"
    assert!(Person::has_name(&person));
    assert!(Person::has_age(&person));
    assert!(Person::has_email(&person));
}

#[test]
fn test_person_clear() {
    let mut person = PersonImpl::new();

    PersonAppend::set_name(&mut person, "Bob");
    PersonAppend::set_age(&mut person, 25);
    assert!(Person::has_name(&person));
    assert!(Person::has_age(&person));

    PersonMut::clear_name(&mut person);
    PersonMut::clear_age(&mut person);

    assert_eq!(Person::name(&person), "");
    assert_eq!(Person::age(&person), 0);
    // ImplicitOptional fields are present only if not equal to default value
    assert!(!Person::has_name(&person)); // Empty string is default value
    assert!(!Person::has_age(&person)); // 0 is default value
}

#[test]
fn test_person_clone() {
    let mut person = PersonImpl::new();
    PersonAppend::set_name(&mut person, "Charlie");
    PersonAppend::set_age(&mut person, 35);

    let cloned = person.clone();
    assert_eq!(person, cloned);
    assert_eq!(Person::name(&cloned), "Charlie");
    assert_eq!(Person::age(&cloned), 35);
    assert!(Person::has_name(&cloned));
    assert!(Person::has_age(&cloned));
}

#[test]
fn test_person_trait_usage() {
    // Test that we can use the Person trait for immutable access
    fn print_person_info(p: &impl Person) -> String {
        format!("{} (age: {})", Person::name(p), Person::age(p))
    }

    let mut person = PersonImpl::new();
    PersonAppend::set_name(&mut person, "Dave");
    PersonAppend::set_age(&mut person, 40);

    assert_eq!(print_person_info(&person), "Dave (age: 40)");
}

#[test]
fn test_person_append_trait_usage() {
    // Test that we can use PersonAppend trait for append-only operations
    fn populate_person(p: &mut impl PersonAppend, name: &str, age: i32) {
        PersonAppend::set_name(p, name);
        PersonAppend::set_age(p, age);
        // p.clear_name(); // ❌ Would not compile - safe!
    }

    let mut person = PersonImpl::new();
    populate_person(&mut person, "Grace", 28);

    assert_eq!(Person::name(&person), "Grace");
    assert_eq!(Person::age(&person), 28);
    assert!(Person::has_name(&person));
    assert!(Person::has_age(&person));
}

#[test]
fn test_person_mut_trait_usage() {
    // Test that we can use PersonMut trait for full mutable operations
    fn reset_person(p: &mut impl PersonMut) {
        PersonAppend::set_name(p, "Default");
        PersonMut::clear_age(p); // Only PersonMut can clear
    }

    let mut person = PersonImpl::new();
    PersonAppend::set_name(&mut person, "Alice");
    PersonAppend::set_age(&mut person, 30);

    reset_person(&mut person);

    assert_eq!(Person::name(&person), "Default");
    assert_eq!(Person::age(&person), 0);
    // ImplicitOptional fields are present only if not equal to default value
    assert!(!Person::has_age(&person)); // 0 is default value
}

#[test]
fn test_immutable_reference() {
    // Test that immutable references only allow Person trait operations
    let mut person = PersonImpl::new();
    PersonAppend::set_name(&mut person, "Henry");
    PersonAppend::set_age(&mut person, 50);

    // Take an immutable reference - can only use Person trait methods
    // Note: Person is not dyn-compatible, so we use impl Person
    fn use_person_ref(p: &impl Person) {
        assert_eq!(Person::name(p), "Henry");
        assert_eq!(Person::age(p), 50);
        assert!(Person::has_name(p));
        // p.set_name("test"); // This would not compile - good!
    }
    use_person_ref(&person);
}

#[test]
fn test_person_string_coercion() {
    let mut person = PersonImpl::new();

    // Test that String can be coerced to &str
    let name_string = "Eve".to_string();
    PersonAppend::set_name(&mut person, &name_string);
    assert_eq!(Person::name(&person), "Eve");

    // &str literal should also work
    PersonAppend::set_name(&mut person, "Frank");
    assert_eq!(Person::name(&person), "Frank");
}

#[test]
fn test_memory_layout_optimized() {
    use std::mem::{align_of, size_of};

    // Verify memory layout is optimized
    let total_size = size_of::<PersonImpl>();

    // Expected sizes on 64-bit:
    // - StringFieldWrapper: 24 bytes (String)
    // - StringFieldWrapper: 24 bytes (String)
    // - MessageFieldWrapper: 8 bytes (Option<Box<_>>)
    // - SharedFields<1>: 1 byte (BitArr storage)
    // - padding: 7 bytes (alignment for i32 fields)
    // - i32: 4 bytes (age)
    // - i32: 4 bytes (score)
    // - i32: 4 bytes (status)
    // - i32: 4 bytes (secondary_status)
    // Total: 80 bytes

    println!("PersonImpl size: {} bytes", total_size);
    println!("PersonImpl alignment: {} bytes", align_of::<PersonImpl>());

    // On 64-bit systems, should be 80 bytes with BitArr
    // (BitArr is stack-allocated, same efficiency as u32, but supports unlimited fields)
    #[cfg(target_pointer_width = "64")]
    {
        assert_eq!(
            total_size, 80,
            "PersonImpl should be 80 bytes on 64-bit with BitArr for presence tracking (2 strings + 1 message + 2 enums + 2 scalars + shared fields)"
        );
    }

    // Alignment should be 8 (word size)
    #[cfg(target_pointer_width = "64")]
    {
        assert_eq!(align_of::<PersonImpl>(), 8);
    }
}

#[test]
fn test_inline_optimization_hint() {
    // This test doesn't actually verify inlining (that requires benchmarks),
    // but serves as documentation that all methods should be inlined.
    //
    // The compiler will inline methods marked with #[inline] when beneficial.
    // We've marked all getters, setters, and trait methods with #[inline].

    let mut person = PersonImpl::new();

    // These calls should be inlined in release builds
    PersonAppend::set_name(&mut person, "Inline Test");
    let _ = Person::name(&person);
    let _ = Person::has_name(&person);

    // Just verify functionality
    assert_eq!(Person::name(&person), "Inline Test");
}

#[test]
fn test_clone_into_optimization() {
    // Test that clone_into correctly updates the string value
    // Note: We can't directly test allocation reuse through the public API
    // (since name() returns &str, not &String), but clone_into guarantees
    // allocation reuse when possible, which is better than v.into()

    let mut person = PersonImpl::new();

    // First set - allocates
    PersonAppend::set_name(&mut person, "A very long string that requires heap allocation");
    assert_eq!(
        Person::name(&person),
        "A very long string that requires heap allocation"
    );

    // Second set with shorter string - clone_into reuses allocation internally
    PersonAppend::set_name(&mut person, "Short");
    assert_eq!(Person::name(&person), "Short");

    // Third set with another long string
    PersonAppend::set_name(&mut person, "Another very long string that requires heap allocation");
    assert_eq!(
        Person::name(&person),
        "Another very long string that requires heap allocation"
    );

    // The optimization is that clone_into avoids unnecessary deallocation/reallocation
    // This is verified by the implementation using v.clone_into(&mut self.name)
}
