
Yet another Rust implementation of [Google Protocol Buffer](https://developers.google.com/protocol-buffers).

__Warning! The interface is still experimental and it will change very frequently!!__

# How to compile your .pb files to .rs files

Please check [the `readme` of this repository](https://github.com/wada314/puroro#readme)

# Table of contents

1. Basic usage (this page)
1. [Generated struct](internal::impls::simple)
1. [Generated trait](internal::impls::traits)

# Simple example

For an input .proto file like this:
```protobuf
syntax = "proto3";
package library;
message Book {
    string title = 1;
    uint32 num_pages = 2;
}
```

A struct like this is output:
```rust
pub struct Book { /* ... */ }
impl Book {
    pub fn new() -> Self {
        // ...
#       todo!()
    }

    pub fn title(&self) -> &str {
        // ...
#       todo!()
    }
    pub fn title_opt(&self) -> Option<&str> {
        // ...
#       todo!()
    }
    pub fn has_title(&self) -> bool {
        // ...
#       todo!()
    }
    pub fn title_mut(&mut self) -> &mut String {
        // ...
#       todo!()
    }
    pub fn clear_title(&mut self) {
        // ...
#       todo!()
    }

    pub fn num_pages(&self) -> u32 {
        // ...
#       todo!()
    }
    pub fn num_pages_opt(&self) -> Option<u32> {
        // ...
#       todo!()
    }
    pub fn has_num_pages(&self) -> bool {
        // ...
#       todo!()
    }
    pub fn num_pages_mut(&mut self) -> &mut u32 {
        // ...
#       todo!()
    }
    pub fn clear_num_pages(&mut self) {
        // ...
#       todo!()
    }
}
```

The struct also implements [`Clone`], [`Default`], [`PartialEq`] and
[`Debug`](std::fmt::Debug) standard library traits.

Let's assume the generated code is in `puroro-doc-samples` crate,
then you can use the generated protobuf like this:

```rust
use ::puroro_doc_samples::library::Book;

let mut book = Book::new();
*book.title_mut() = "The C Programming Language".to_string();
// We are not setting the number of pages here.

assert_eq!("The C Programming Language", book.title());
assert!(!book.has_num_pages());
```

# Deserializing

You can use [`Message::from_bytes()`] method or [`Message::merge_from_bytes()`]
method to deserialize a message from [`std::io::Read`] bytes stream.

```rust
use puroro::Message; // For from_bytes(), merge_from_bytes() methods
use std::io::Read; // For bytes() method
use puroro_doc_samples::library::Book;

let input1 = vec![0x10, 0x82, 0x01]; // encoded `num_pages: 130`
let input2 = vec![0x0a, 0x02, 0x59, 0x6f]; // encoded `title: "Yo"`

// You can use `from_bytes()` method to deserialize a message
// from an input buffer.
let mut book = Book::from_bytes(input1.bytes()).unwrap();
assert_eq!(130, book.num_pages());

// Or, you can use `merge_from_bytes(&mut self)` method to deserialize
// and merge from an input buffer to an existing message instance.
book.merge_from_bytes(input2.bytes()).unwrap();
assert_eq!("Yo", book.title());
assert_eq!(130, book.num_pages());
```

# Serializing

You can serialize the message into [`std::io::Write`] using [`Message::ser()`] method.

```rust
use puroro::Message; // For ser() method
use puroro_doc_samples::library::Book;

let mut output = vec![];
let mut book = Book::new();

*book.title_mut() = "Yo".to_string();
book.ser(&mut output).unwrap();

assert_eq!(vec![0x0a, 0x02, 0x59, 0x6f], output);
```

# Trait and builder

The input proto message generates not only a struct, but also a trait.
The trait name is *MessageName*Trait (e.g. `BookTrait` for `message Book`).
The trait only has immutable methods.

```rust
pub trait BookTrait {
    fn title(&self) -> &str;
    fn has_title(&self) -> bool;
    fn title_opt(&self) -> Option<&str>;
    fn num_pages(&self) -> u32;
    fn has_num_pages(&self) -> bool;
    fn num_pages_opt(&self) -> Option<u32>;
}
```

Of course, the `struct Book` we described at above implements this trait.
Please note that for some kind of fields the `struct Book`'s interface and
`trait BookTrait`'s interface are slightly different (e.g. `repeated` fields).

The trait is not only implemented for the `struct Book`,
but also for some other types:

```rust
# trait BookTrait {}
// Returns default value for all fields
impl BookTrait for () {}

// If the value is present then behaves like the internal value.
// If the value is not present then returns default value for all fields.
impl<T: BookTrait> BookTrait for Option<T> {}

// Behaves as as the either of the types present.
impl<T: BookTrait, U: BookTrait> BookTrait for puroro::Either<T, U> {}

// Behaves like a merged message. Behaves like `U` is merged into `T`.
impl<T: BookTrait, U: BookTrait> BookTrait for (T, U) {}

// Trivial impls
impl<'a, T: BookTrait> BookTrait for &'a T {}
impl<'a, T: BookTrait> BookTrait for &'a mut T {}
impl<T: BookTrait> BookTrait for Box<T> {}
```

# Builders

puroro also generates a builder type for each message.
The name of builder is *MessageName*Builder (e.g. `BookBuilder` for `message Book`).

```rust
# use puroro_doc_samples::library::{
#     BookTrait, BookSingleField1, BookSingleField2
# };
pub struct BookBuilder<T>(T);

impl BookBuilder<()> {
    pub fn new() -> Self {
        // ...
#       todo!()
    }
}

impl<T> BookBuilder<T>
where
    T: BookTrait,
{
    pub fn build(self) -> T {
        // ...
#       todo!()
    }

    pub fn append_title<ScalarType>(
        self,
        value: ScalarType,
    ) -> BookBuilder<(T, BookSingleField1<ScalarType>)>
    where ScalarType: AsRef<str>,
    {
        // ...
#       todo!()
    }

    pub fn append_num_pages<ScalarType>(
        self,
        value: ScalarType,
    ) -> BookBuilder<(T, BookSingleField2<ScalarType>)>
    where ScalarType: Into<u32> + Clone,
    {
        // ...
#       todo!()
    }
}
```

This might look like little complicated, but the usage is very simple:

```rust
use puroro_doc_samples::library::{BookBuilder, BookTrait};

let book = BookBuilder::new()
    .append_title("The C Programming Language")
    .build();

assert_eq!("The C Programming Language", book.title());
assert!(!book.has_num_pages());
```

You need to call `new()` method of builder first, then call `append_***()` methods
as many as you like, then terminate with `build()` method.
Then you can get a generated type which is implementing `BookTrait` trait,
and of course it can be serialized.

There are some benefits of using this builder instead of the normal `struct Book`:
* The builder generated type has lesser memory footprint. It only consumes the memory
for explicitly appended fields.
* The field type is more flexible. Note that you don't need to call `to_string()` method
when setting the string field. Actually, in the example above the internal field type
is not `String` but `&str`, which does not allocate any heap memory.

Instead, the builder has some downsides compared to the normal struct.
* You can only `append` the field. Particurally, you cannot clear field nor
edit previously added repeated field values.
* You always need to manually write a code to use the builder. No deserialization support.

# Using [`bumpalo`](https://github.com/fitzgen/bumpalo) allocator

puroro has an experimental implementation using [`bumpalo`](https://github.com/fitzgen/bumpalo)
allocator instead of the default global allocator.

```rust
# use std::ops::DerefMut;
pub struct BookBumpalo<'bump> {
#   phantom: std::marker::PhantomData<&'bump ()>,
    // ...
}
impl<'bump> BookBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
#       todo!()
        // ...
    }

    pub fn title(&self) -> &str {
#       todo!()
        // ...
    }
    pub fn title_opt(&self) -> Option<&str> {
#       todo!()
        // ...
    }
    pub fn has_title(&self) -> bool {
#       todo!()
        // ...
    }
    pub fn title_mut(&mut self) -> impl '_ + DerefMut<Target = ::puroro::bumpalo::collections::String<'bump>> {
#       todo!(); unsafe { std::ptr::NonNull::dangling().as_mut() }
        // ...
    }
    pub fn clear_title(&mut self) {
#       todo!()
        // ...
    }

    pub fn num_pages(&self) -> u32 {
#       todo!()
        // ...
    }
    pub fn num_pages_opt(&self) -> Option<u32> {
#       todo!()
        // ...
    }
    pub fn has_num_pages(&self) -> bool {
#       todo!()
        // ...
    }
    pub fn num_pages_mut(&mut self) -> &mut u32 {
#       todo!()
        // ...
    }
    pub fn clear_num_pages(&mut self) {
#       todo!()
        // ...
    }
}
```

This behaves almost as same as the normal `struct Book` impl,
though the interface of `repeated` fields or `string` / `bytes` / `message`
fields are different with the normal one.
Especially, for `repated` field's mutable interface is not very useful yet
so please make sure about that before using this implementation.
