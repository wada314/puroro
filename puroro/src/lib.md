
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
} // mod library
```

The struct also implements [`Clone`], [`Default`], [`PartialEq`] and
[`Debug`](std::fmt::Debug) standard library traits.

Let's assume the generated code is in `puroro-doc-samples` crate,
then you can use the generated protobuf like this:

```rust
use ::puroro_doc_samples::library::Book;

let mut book = Book::default();
*book.title_mut() = "The C Programming Language".to_string();
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
use puroro_doc_samples::library::Book;

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

You can serialize the message into [`std::io::Write`] using [`Message::to_bytes()`] method.

```rust
use puroro::Message; // For to_bytes() method
use puroro_doc_samples::library::Book;

let mut output = vec![];
let mut book = Book::default();

*book.title_mut() = "Yo".to_string();
book.to_bytes(&mut output).unwrap();

assert_eq!(vec![0x0a, 0x02, 0x59, 0x6f], output);
```
