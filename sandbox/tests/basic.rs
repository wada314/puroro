//! Basic integration tests for our API design.

use ::allocator_extras::Global;
use sandbox::generated::person::{DynPerson, Person, PersonAppend, PersonImpl, PersonMut};

#[test]
fn test_person_creation() {
    let person = PersonImpl::new();
    assert_eq!(Person::name(&person), "");
    assert_eq!(Person::age(&person), 0);
    assert_eq!(Person::email(&person), None);

    // ImplicitOptional fields are present only if not equal to default value
    assert!(!DynPerson::has_name(&person)); // Empty string is default value
    // Sample: only has_name is available in DynPerson trait
}

#[test]
fn test_person_default() {
    let person: PersonImpl<Global> = Default::default();
    assert_eq!(Person::name(&person), "");
    assert_eq!(Person::age(&person), 0);
    assert_eq!(Person::email(&person), None);
}

#[test]
fn test_person_setters() {
    let mut person = PersonImpl::new();

    PersonAppend::set_name(&mut person, "Alice");
    // Sample: only set_name is available in DynPersonAppend trait

    assert_eq!(Person::name(&person), "Alice");

    // After setting, fields should be marked as "set"
    assert!(DynPerson::has_name(&person));
    // Sample: only has_name is available in DynPerson trait
}

#[test]
fn test_person_clear() {
    let mut person = PersonImpl::new();

    PersonAppend::set_name(&mut person, "Test");
    assert!(Person::has_name(&person));

    PersonMut::clear_name(&mut person);

    assert_eq!(Person::name(&person), "");
    assert_eq!(Person::age(&person), 0);
    // ImplicitOptional fields are present only if not equal to default value
    assert!(!Person::has_name(&person)); // Empty string is default value
}

#[test]
fn test_person_clone() {
    let mut person = PersonImpl::new();
    PersonAppend::set_name(&mut person, "Charlie");

    let cloned = person.clone();
    assert_eq!(person, cloned);
    assert_eq!(Person::name(&cloned), "Charlie");
    assert!(Person::has_name(&cloned));
}

#[test]
fn test_person_trait_usage() {
    // Test that we can use the Person trait for immutable access
    fn print_person_info(p: &impl Person) -> String {
        format!("{} (age: {})", Person::name(p), Person::age(p))
    }

    let mut person = PersonImpl::new();
    PersonAppend::set_name(&mut person, "Dave");

    assert_eq!(print_person_info(&person), "Dave (age: 0)");
}

#[test]
fn test_person_append_trait_usage() {
    // Test that we can use PersonAppend trait for append-only operations
    fn populate_person(p: &mut impl PersonAppend, name: &str) {
        PersonAppend::set_name(p, name);
        // Sample: only set_name is available in PersonAppend trait
        // p.clear_name(); // ❌ Would not compile - safe!
    }

    let mut person = PersonImpl::new();
    populate_person(&mut person, "Grace");

    assert_eq!(Person::name(&person), "Grace");
    assert!(Person::has_name(&person));
}

#[test]
fn test_person_mut_trait_usage() {
    // Test that we can use PersonMut trait for full mutable operations
    fn reset_person(p: &mut impl PersonMut) {
        PersonAppend::set_name(p, "Default");
        PersonMut::clear_name(p); // Only PersonMut can clear (sample: only clear_name is available)
    }

    let mut person = PersonImpl::new();
    PersonAppend::set_name(&mut person, "Alice");

    reset_person(&mut person);

    assert_eq!(Person::name(&person), "");
    assert_eq!(Person::age(&person), 0);
    // ImplicitOptional fields are present only if not equal to default value
    assert!(!Person::has_name(&person)); // Empty string is default value
}

#[test]
fn test_immutable_reference() {
    // Test that immutable references only allow Person trait operations
    let mut person = PersonImpl::new();
    PersonAppend::set_name(&mut person, "Henry");

    // Take an immutable reference - can only use Person trait methods
    // Note: Person is not dyn-compatible, so we use impl Person
    fn use_person_ref(p: &impl Person) {
        assert_eq!(Person::name(p), "Henry");
        assert_eq!(Person::age(p), 0); // Default value since set_age is not available in sample
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
#[ignore = "Layout is unstable while adding repeated fields; skip for now"]
fn test_memory_layout_optimized() {
    use std::mem::{align_of, size_of};

    // Verify memory layout is optimized
    let total_size = size_of::<PersonImpl>();

    // Expected sizes on 64-bit (allocator-aware layout):
    // - StringFieldWrapper<A>: 24 bytes each
    // - MessageFieldWrapper<_, A>: 16 bytes (Option<Box<_, A>> plus allocator)
    // - SharedFields<1, A>: 32 bytes (presence bits + unknown buffer + allocator)
    // - Scalars and enums: 16 bytes total
    // Total: 104 bytes

    println!("PersonImpl size: {} bytes", total_size);
    println!("PersonImpl alignment: {} bytes", align_of::<PersonImpl>());

    // On 64-bit systems, the allocator-aware form is 104 bytes.
    #[cfg(target_pointer_width = "64")]
    {
        assert_eq!(
            total_size, 104,
            "PersonImpl should be 104 bytes on 64-bit with allocator-aware wrappers and shared state"
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
    let _ = DynPerson::has_name(&person);

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
    PersonAppend::set_name(
        &mut person,
        "A very long string that requires heap allocation",
    );
    assert_eq!(
        Person::name(&person),
        "A very long string that requires heap allocation"
    );

    // Second set with shorter string - clone_into reuses allocation internally
    PersonAppend::set_name(&mut person, "Short");
    assert_eq!(Person::name(&person), "Short");

    // Third set with another long string
    PersonAppend::set_name(
        &mut person,
        "Another very long string that requires heap allocation",
    );
    assert_eq!(
        Person::name(&person),
        "Another very long string that requires heap allocation"
    );

    // The optimization is that clone_into avoids unnecessary deallocation/reallocation
    // This is verified by the implementation using v.clone_into(&mut self.name)
}
