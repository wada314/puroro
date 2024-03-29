
Yet another Rust implementation of [Google Protocol Buffer](https://developers.google.com/protocol-buffers).

__Warning! The interface is still experimental and it will change very frequently!!__

# How to compile your .pb files to .rs files

Please check [the `readme` of this repository](https://github.com/wada314/puroro#readme)

# Table of contents

1. Basic usage (this page)

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
mod library {
/// A mutable message type
#[derive(Default)]
pub struct Book { /* ... */ }

/// An immutable message type view, deref-ed from the `Book` struct.
pub struct BookView { /* ... */ }

# use std::ops::Deref;
impl Deref for Book {
    type Target = BookView;
    fn deref(&self) -> &BookView { 
        /* ... */ 
#       todo!()
    }
}

impl Book {
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
}
impl BookView {
    pub fn title_mut(&mut self) -> &mut ::puroro::String {
        // ...
#       todo!()
    }
    pub fn clear_title(&mut self) {
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
} // mod library
```

The `Book` struct and `BookView` struct's relationship is similar to
`Vec<T>` and `[T]`. The latter works as an immutable reference type for the former,
and the latter can be `Deref`-ed from the former.

The `Book` struct also implements [`Clone`], [`Default`], [`PartialEq`] and
[`Debug`](std::fmt::Debug) standard library traits.

Let's assume the generated code is in `doc_samples` module,
then you can use the generated protobuf like this:

```rust
# use puroro::doc_samples;
use doc_samples::library::Book;

let mut book = Book::default();
*book.title_mut() = "The C Programming Language".into();
// We are not setting the number of pages here.

assert_eq!("The C Programming Language", book.title());
assert!(!book.has_num_pages());
```

# Deserializing

You can use [`Message::from_bytes_iter()`] method or [`Message::merge_from_bytes_iter()`]
method to deserialize a message from [`std::io::Read`] bytes stream.

```rust
use puroro::Message; // For from_bytes_iter(), merge_from_bytes_iter() methods
use std::io::Read; // For bytes() method
# use puroro::doc_samples;
use doc_samples::library::Book;

let input1 = vec![0x10, 0x82, 0x01]; // encoded `num_pages: 130`
let input2 = vec![0x0a, 0x02, 0x59, 0x6f]; // encoded `title: "Yo"`

// You can use `from_bytes()` method to deserialize a message
// from an input buffer.
let mut book = Book::from_bytes_iter(input1.bytes()).unwrap();
assert_eq!(130, book.num_pages());

// Or, you can use `merge_from_bytes_iter(&mut self)` method to deserialize
// and merge from an input buffer to an existing message instance.
book.merge_from_bytes_iter(input2.bytes()).unwrap();
assert_eq!("Yo", book.title());
assert_eq!(130, book.num_pages());
```

# Serializing

You can serialize the message into [`std::io::Write`] using [`MessageView::to_bytes()`] method.

```rust
use puroro::MessageView; // For to_bytes() method
# use puroro::doc_samples;
use doc_samples::library::Book;

let mut output = vec![];
let mut book = Book::default();

*book.title_mut() = "Yo".into();
book.to_bytes(&mut output).unwrap();

assert_eq!(vec![0x0a, 0x02, 0x59, 0x6f], output);
```
