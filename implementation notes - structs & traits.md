# Structs and traits

How the proto's `message` should be mapped to the rust's `struct`s (or `trait`s)?

```protobuf
message Person {
    optional uint32 age = 1;
    optional Person partner = 2;
    repeated string full_names = 3;
}
```

# Issue #1: public fields vs. getter methods

Should the rust interface be like:

```rust
// 1-A: public fields
pub struct Person {
    pub age: Option<u32>,
    pub partner: Option<Box<Person>>,
    pub full_names: Vec<String>,
}
```

or, like:

```rust
// 1-B: getter methods
pub struct Person {
    /* ... */
}
impl Person {
    // There are still lots of the space for the return types of the methods
    pub fn age(&self) -> &Option<u32> { /* ... */ }
    pub fn partner(&self) -> &Option<Box<Person>> { /* ... */ }
    pub fn full_names(&self) -> &[String] { /* ... */ }
}
```

this?

### Pros for 1-A (public fields):

- The user can use the fields in shorter code: `person.age` vs `person.age()`.
- In 1-B, we need at least 2 methods for each field in difference of mutablitiy: e.g. `age(&self)` and `age_mut(&mut self)`.
- The user can split the struct into multiple mutable references. For example, the user can do this:

```rust
let p = Person::new();
let age_mut = &mut p.age;
let partner_mut = &mut p.partner;
```

This is not possible in 1-B interface.
[related rustonomicon document](https://doc.rust-lang.org/nomicon/borrow-splitting.html)

### Pros for 1-B (getter methods):

- The struct doesn't need to own the field type exactly the same as the interface. For example, (a slightly modified with above example) interface `fn age(&self) -> Option<&u32>` returns `Option<&u32>`, but the struct's actual inner type can be `Option<u32>`, `(bool, u32)`, or maybe use a shared bitvec like `Vec<bool>, u32` internally.
    - This can make the memory layout more efficient. In 1-A implementation, the `Optional<u32>` field takes 64 bits size, where it actually only needs 33 bits.
    - Ditto for `bool` fields.
    - Ditto for `enum` fields and `oneof` fields.
    - More optimization might be possible. For example, in C++ protobuf implementation, the scalar string fields are implemented as an union of heap-allocated string type and plain char array (If I remember correctly).
- The interface can do complex process instead of directly accessing the fields. We don't need much complex process for protobuf language actually, but at least there's one example which is a default value in proto2. If we use a simple `Option<u32>` public field interface for `option uint32 age = 1 [default = 20];` proto field, The only place we can set the default value is when the field is initialized. On the other hand, if we use the 1-B option, we can set the default value when the field is cleared by `clear_age(&mut self)` method, or can define a getter method that always returns `u32` instead of `Option<u32>` using the default value in case if the field is not set.
