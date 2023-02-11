# Design discussion: Message ref type

## Example

For the following proto3 message:

```protobuf
syntax = "proto3";
message Person {
    uint32 age = 1;
    Person partner = 2;
    repeated Person children = 3;
}
```

The generated rust's struct will be something like this
(ignoring some details like allocator):

```rust
pub struct Person { /* ... */ }
impl Person {
    pub fn age(&self) -> i32 { /* ... */ }
    pub fn age_mut(&mut self) -> &mut i32 { /* ... */ }
    pub fn partner(&self) -> Option<&Person> { /* ... */ }
    pub fn partner_mut(&mut self) -> &mut Person { /* ... */ }
    pub fn children(&self) -> &[Person] { /* ... */ }
    pub fn children_mut(&mut self) -> &mut Vec<Person> { /* ... */ }
}
```

Though we still have some space to discuss about this interface (like "Should the repeated field return a slice or an iterator?", "Should we provide an interface returning a mut ref or provide setters or use builder pattern?"), that's not the topic I discuss in this document.

Well, this interface will work as long as our generated `Person` struct is enough simple that have no customization options. But in future maybe we will want to add some generic parameters for `Person` struct like this:

```rust
pub struct Person<Alloc=Global> { /* ... */ }
impl<Alloc: Allocator> Person<Alloc> {
    /* ... */
}
```

Then, the every user code that is using the `Person` type need to update the code. Especially, for the functions which is taking an immutable reference, even though they actually do not need to know about the allocator but they are forced to write the complicated type bounds at every function definitions:

```rust
// If Person does not take generic parameter
fn process(person: &Person) { /* ... */ }

// If Person takes Allocator type param
fn process2<Alloc: Allocator>(person: &Person<Alloc>) { /* ... */}
```

This is not a good user experience. We want to add a way for these function to use `&Person` interface without knowing about the allocator details. We can imagine several solutions for this:

---

## Solution A: Not use the generic parameters, use an internal enum instead

```rust
pub enum Allocators {
    Global(Global),
    MyCustomAllocator(MyCustomAllocator),
}

pub struct Person {
    /* fields... */
    alloc: Allocators,
}

// Simple user method
fn process(person: &Person) { /* ... */ }
```

Of course, it is possible to move the generic parameters into the struct's internal enum value.

#### pros:
- Simple interface: No generic params.

#### cons:
- Runtime cost for `switch`ing the `enum`.
  - Though, in the allocator's case this cost only surfaces when mutating the fields.
- Memory footprint cost. `enum`'s size is the largest value's size. In the allocator's case, the default `Global` allocator is zero-size so it's always an extra cost.
- Cannot benefit from the static type checker: Cannot detect the wrong allocator use.

## Solution B: Create a view-only type, something like `[T]` for `Vec<T>`

This is the common pattern in the `std` library. We can do the similar thing with them:

```rust
pub struct PersonView { /* ... */ }
pub struct Person<Alloc> { /* ... */ }

impl PersonView {
    pub fn age(&self) -> i32 { /* ... */ }
    pub fn partner(&self) -> Option<&Person> { /* ... */ }
    pub fn children(&self) -> &[Person] { /* ... */ }
}
impl<Alloc: Allocator> Person<Alloc> {
    pub fn age_mut(&mut self) -> &mut i32 { /* ... */ }
    pub fn partner_mut(&mut self) -> &mut Person { /* ... */ }
    pub fn children_mut(&mut self) -> &mut Vec<Person> { /* ... */ }
}
impl<Alloc> Deref for Person<Alloc> {
    type Target = PersonView;
    fn deref(&self) -> &Self::Target {
        /* ... */
    }
}

// The user method doesn't know about the allocator
fn process(person: &PersonView) { /* ... */ }
```

#### pros:
- Less runtime time / memory footprint cost compared to the solution A.

#### cons:
- Limited in-`Person` struct structure. It must own `PersonView` separately from the generic params depending fields. This may cost an extra runtime cost, depending on the implementation and the generic params usages.
- Limited generic params usages. Apparently `PersonView` type cannot know about the generic params, so the view methods implementation should not depend on the generic params.
  - For example, we may want to customize the submessage type placement location between the heap and the stack (in-struct) using the generic param option. In such case, we need to fall back to solution A.

## Solution C: Define a trait using dynamic dispatch

Defining a new trait for `Person` is the another option.
We can have 2 options for this direction; dynamic or static dispatch.

```rust
// Same as the default example
pub struct Person<Alloc = Global> { /* ... */ }
impl<Alloc: Allocator> Person<Alloc> {
    pub fn age(&self) -> i32 { /* ... */ }
    pub fn age_mut(&mut self) -> &mut i32 { /* ... */ }
    pub fn partner(&self) -> Option<&Person> { /* ... */ }
    pub fn partner_mut(&mut self) -> &mut Person { /* ... */ }
    pub fn children(&self) -> &[Person] { /* ... */ }
    pub fn children_mut(&mut self) -> &mut Vec<Person> { /* ... */ }
}

pub trait PersonTrait {
    fn age(&self) -> i32;
    fn partner(&self) -> &dyn PersonTrait;
    // It is little tricky to return multiple `dyn` objects with the
    // minimum runtime cost. Not discussing in this article...
    fn children(&self) -> &dyn RepeatedField<Item=dyn PersonTrait>;
}

// The user method take an arbitral type implementing `PersonTrait`
fn process(person: &dyn PersonTrait) { /* ... */ }
```

A strong pros of this method compared to the previous 2 solutions is the extensibility. Because we are using the trait instead of the type, we can use the totally different implementation for `PersonTrait`.
For example, we can implement `PersonTrait` for `()` and make it behave as a default message value. Or we can implement for `Option<T> where T: PersonTrait` to treat the optional `PersonTrait` itself as another `PersonTrait`.

#### pros:
- Extensibility, as discussed above.
  - One of the most useful use case is the scalar message field getter interface. Normal `impl`s of the `partner` field getter are returning the `Option`-ed message reference, though we can use a raw `&dyn PersonTrait` return value in our trait method interface, because we can hide the `Option` behind the `PersonTrait`. This will make the caller code simpler in most cases.
- (Sometimes) faster runtime code. Imagine if you want to serialize a default instance of the `Person` message -- If you use the normal `struct Person`'s default instance to serialize, the serializer code still need to check the all field's existence even though we know those fields are all empty. If we can use the custom `trait` implementation for the empty message (e.g. use `()` as the example above), then the serialization code can immediately return because it knows there are no fields to serialize.

#### cons:
- Runtime cost. Even getting a single field value costs a dynamic dispatch cost, and the method inlining optimization won't work at all. This is the heaviest cost among the all of the previous solutions.
- Duplicated method implementations. We need to define the getter methods twice: in the `struct`'s `impl`, and in the `trait`'s `impl`. And the interfaces (return type of the methods) will be slightly different between these two. This will easily confuse the library users.
- Because we are presuming to use the `trait` for `dyn Trait`, we need to make sure the `trait` interface is object-safe and not including associated type aliases. This will give some limitations for the `trait`'s interface.
- Significantly lot more code to maintenance.

## Solution D: Define a trait using static dispatch

Similar to the previous solution, use the `trait` but not use the dynamic dispatch (i.e. not use `dyn`). This will make the code significantly complex but gives more flexibility for the implementation.

```rust
// Same as the default example
pub struct Person<Alloc = Global> { /* ... */ }
impl<Alloc: Allocator> Person<Alloc> {
    pub fn age(&self) -> i32 { /* ... */ }
    pub fn age_mut(&mut self) -> &mut i32 { /* ... */ }
    pub fn partner(&self) -> Option<&Person> { /* ... */ }
    pub fn partner_mut(&mut self) -> &mut Person { /* ... */ }
    pub fn children(&self) -> &[Person] { /* ... */ }
    pub fn children_mut(&mut self) -> &mut Vec<Person> { /* ... */ }
}

pub trait PersonTrait {
    fn age(&self) -> i32;

    type PartnerMessageType<'a>: PersonTrait where Self: 'a;
    fn partner(&self) -> Self::PartnerMessageType<'_>;

    type ChildrenMessageType<'a>: PersonTrait where Self: 'a;
    type ChildrenRepeatedType<'a>: Iterator<Item=Self::ChildrenMessageType<'a>> where Self: 'a;
    fn children(&self) -> self::ChildrenRepeatedType<'_>;
}

// The user method take an arbitral type implementing `PersonTrait`
fn process<T: PersonTrait>(person: &T) { /* ... */ }
```

The biggest benefit compared to the previous `dyn` solution is the static method dispatch. Instead, the generated code become more complicated and difficult to understand by the library users.
Another not obvious benefit is the scalar message getter type has been changed from the reference to an arbitral `Sized` type. In other words, in the previous `dyn` solution the scalar message getter method could not return the message by value but only could return the reference.
This makes difference in the more complex custom `PersonTrait` implementations. For example, a merged 2 `Person` messages type tuple `(T, U)` can implement `PersonTrait` only with this solution because the return type of the `partner` field getter is `(T::PartnerMessageType<'_>, U::PartnerMessageType<'_>)` tuple. In the previous solution C, the return type of the `partner` field getter must be a reference, so this method is not implementable.

#### pros:
- More extensible.
- Less runtime cost.

#### cons:
- More complicated.
- (Maybe) larger generated code.
