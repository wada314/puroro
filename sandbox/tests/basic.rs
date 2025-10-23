//! Basic integration tests for our API design.

use sandbox::generated::person::{
    Person, PersonAppend, PersonAppendTry, PersonImpl, PersonMut, PersonTry,
};

#[test]
fn test_person_creation() {
    let person = PersonImpl::new();
    assert_eq!(person.name(), "");
    assert_eq!(person.age(), 0);
    assert_eq!(person.email(), "");

    // ImplicitOptional fields are always considered "present"
    assert!(person.has_name());
    assert!(person.has_age());
    assert!(person.has_email());
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
    // ImplicitOptional fields are always considered "present" even after clear
    assert!(person.has_name());
    assert!(person.has_age());
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
    // Test that we can use the Person trait for immutable access
    fn print_person_info(p: &impl Person) -> String {
        format!("{} (age: {})", p.name(), p.age())
    }

    let mut person = PersonImpl::new();
    person.set_name("Dave");
    person.set_age(40);

    assert_eq!(print_person_info(&person), "Dave (age: 40)");
}

#[test]
fn test_person_append_trait_usage() {
    // Test that we can use PersonAppend trait for append-only operations
    fn populate_person(p: &mut impl PersonAppend, name: &str, age: i32) {
        p.set_name(name);
        p.set_age(age);
        // p.clear_name(); // ❌ Would not compile - safe!
    }

    let mut person = PersonImpl::new();
    populate_person(&mut person, "Grace", 28);

    assert_eq!(person.name(), "Grace");
    assert_eq!(person.age(), 28);
    assert!(person.has_name());
    assert!(person.has_age());
}

#[test]
fn test_person_mut_trait_usage() {
    // Test that we can use PersonMut trait for full mutable operations
    fn reset_person(p: &mut impl PersonMut) {
        p.set_name("Default");
        p.clear_age(); // Only PersonMut can clear
    }

    let mut person = PersonImpl::new();
    person.set_name("Alice");
    person.set_age(30);

    reset_person(&mut person);

    assert_eq!(person.name(), "Default");
    assert_eq!(person.age(), 0);
    // ImplicitOptional fields are always considered "present" even after clear
    assert!(person.has_age());
}

#[test]
fn test_immutable_reference() {
    // Test that immutable references only allow Person trait operations
    let mut person = PersonImpl::new();
    person.set_name("Henry");
    person.set_age(50);

    // Take an immutable reference - can only use Person trait methods
    let person_ref: &dyn Person = &person;
    assert_eq!(person_ref.name(), "Henry");
    assert_eq!(person_ref.age(), 50);
    assert!(person_ref.has_name());
    // person_ref.set_name("test"); // This would not compile - good!
}

#[test]
fn test_person_string_coercion() {
    let mut person = PersonImpl::new();

    // Test that String can be coerced to &str
    let name_string = "Eve".to_string();
    person.set_name(&name_string);
    assert_eq!(person.name(), "Eve");

    // &str literal should also work
    person.set_name("Frank");
    assert_eq!(person.name(), "Frank");
}

#[test]
fn test_person_try_trait() {
    // Test that we can use PersonTry for fallible operations
    let mut person = PersonImpl::new();
    person.set_name("Iris");
    person.set_age(45);

    // PersonImpl's fallible operations always succeed
    assert_eq!(person.try_name().unwrap(), "Iris");
    assert_eq!(person.try_age().unwrap(), 45);
    assert_eq!(person.try_email().unwrap(), "");
}

#[test]
fn test_person_try_mut_trait() {
    // Test that we can use PersonTryMut for fallible mutations
    let mut person = PersonImpl::new();

    // PersonImpl's fallible operations always succeed
    person.try_set_name("Jack").unwrap();
    person.try_set_age(55).unwrap();

    assert_eq!(person.try_name().unwrap(), "Jack");
    assert_eq!(person.try_age().unwrap(), 55);
}

#[test]
fn test_person_append_try_trait() {
    // Test that we can use PersonAppendTry for fallible append operations
    fn try_populate_person(
        p: &mut impl PersonAppendTry,
        name: &str,
        age: i32,
    ) -> Result<(), puroro::error::Error> {
        p.try_set_name(name)?;
        p.try_set_age(age)?;
        Ok(())
    }

    let mut person = PersonImpl::new();
    try_populate_person(&mut person, "Laura", 40).unwrap();

    assert_eq!(person.try_name().unwrap(), "Laura");
    assert_eq!(person.try_age().unwrap(), 40);
}

#[test]
fn test_fallible_generic_code() {
    // Test that we can write generic code using fallible traits
    fn get_person_summary(p: &impl PersonTry) -> Result<String, puroro::error::Error> {
        Ok(format!("{} (age: {})", p.try_name()?, p.try_age()?))
    }

    let mut person = PersonImpl::new();
    person.set_name("Kate");
    person.set_age(33);

    assert_eq!(get_person_summary(&person).unwrap(), "Kate (age: 33)");
}

#[test]
fn test_memory_layout_optimized() {
    use std::mem::{align_of, size_of};

    // Verify memory layout is optimized
    let total_size = size_of::<PersonImpl>();

    // Expected sizes on 64-bit:
    // - String: 24 bytes (3 words: ptr, len, cap)
    // - String: 24 bytes
    // - BitArr!(for 3, in u8): 1 byte (fixed-size, stack-allocated)
    // - padding: 3 bytes (to align i32)
    // - i32: 4 bytes (age)
    // Total: 56 bytes (same as u32 approach!)

    println!("PersonImpl size: {} bytes", total_size);
    println!("PersonImpl alignment: {} bytes", align_of::<PersonImpl>());

    // On 64-bit systems, should be 56 bytes with BitArr
    // (BitArr is stack-allocated, same efficiency as u32, but supports unlimited fields)
    #[cfg(target_pointer_width = "64")]
    {
        assert_eq!(
            total_size, 56,
            "PersonImpl should be 56 bytes on 64-bit with BitArr for presence tracking"
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
    person.set_name("Inline Test");
    let _ = person.name();
    let _ = person.has_name();

    // Just verify functionality
    assert_eq!(person.name(), "Inline Test");
}

#[test]
fn test_clone_into_optimization() {
    // Test that clone_into correctly updates the string value
    // Note: We can't directly test allocation reuse through the public API
    // (since name() returns &str, not &String), but clone_into guarantees
    // allocation reuse when possible, which is better than v.into()

    let mut person = PersonImpl::new();

    // First set - allocates
    person.set_name("A very long string that requires heap allocation");
    assert_eq!(
        person.name(),
        "A very long string that requires heap allocation"
    );

    // Second set with shorter string - clone_into reuses allocation internally
    person.set_name("Short");
    assert_eq!(person.name(), "Short");

    // Third set with another long string
    person.set_name("Another very long string that requires heap allocation");
    assert_eq!(
        person.name(),
        "Another very long string that requires heap allocation"
    );

    // The optimization is that clone_into avoids unnecessary deallocation/reallocation
    // This is verified by the implementation using v.clone_into(&mut self.name)
}
