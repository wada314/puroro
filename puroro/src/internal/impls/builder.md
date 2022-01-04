# Builders

A `Builder` struct is generated by puroro to compose a struct implementing
the message trait.
The struct name is postfixed by "Builder", so for example from `message Foo`
a `struct FooBuilder` is generated.

From a given input:

```protobuf
syntax = "proto3";
message Book {    
    string title = 1;
    uint32 num_pages = 2;
}
```

generates rust code like this:

```rust
# use ::std::ops::Deref;
# use ::puroro_doc_samples::library::{
#     BookTrait,
#     BookSingleField1,
#     BookSingleField2,
# };
pub struct BookBuilder<T>(T);

impl BookBuilder<()> {
    pub fn new() -> Self {
        Self(())
    }
}
impl<T> BookBuilder<T>
where
    T: BookTrait,
{
    pub fn append_title<ScalarType>(
        self,
        value: ScalarType,
    ) -> BookBuilder<(T, BookSingleField1<ScalarType>)>
    where
        ScalarType: AsRef<str>,
    {
        // ...
#        todo!()
    }

    pub fn append_num_pages<ScalarType>(
        self,
        value: ScalarType,
    ) -> BookBuilder<(T, BookSingleField2<ScalarType>)>
    where
        ScalarType: Into<u32> + Clone,
    {
        // ...
#        todo!()
    }

    pub fn build(self) -> T {
        self.0
    }
}
```

A sample usage is like this:

```rust
# use ::puroro_doc_samples::library::{BookBuilder, BookTrait};
let book = BookBuilder::new()
    .append_title("The C Programming Language")
    .append_num_pages(123u32)
    .build();

assert_eq!("The C Programming Language", book.title());
assert_eq!(123u32, book.num_pages());
```

There are some benefits of using this builder instead of the normal `struct Book`:
* The builder generated type has lesser memory footprint. It only consumes the memory
for explicitly appended fields.
* The field type is more flexible. Note that you don't need to call `to_string()` method
when setting the string field. Actually, in the example above the internal field type
is not `String` but `&str`, which does not allocate any heap memory.

Instead, the builder has some downsides compared to the normal struct.
* You can only `append` the field. Particurally, you cannot clear field nor
edit previously added repeated field values (more details at below).
* You always need to manually write a code to use the builder. No deserialization support.

# Limitations
You may feel strange that the methods are named as `append_something`, not `set_something`.
This is because that our builder methods have some limitations:

* For singular fields, you cannot "clear" the field once you have set some
value. You can only overwrite it.
    * Particularly for singular message fields, you cannot "clear"
    the existing field but you can only merge into the existing field.
* For singular proto3 fields, you cannot set the field value to
a default value (e.g. `0` for int32, `""` for string) once you have set a value.
* For repeated fields, you can only append items. You cannot remove or edit any items you have already added.

These limitations come from the protobuf language spec and our implementation
of builder structs. Our builder's append methods works as appending a serilized
field into a serialized message.

In the protocol buffer's specification, it is legal that the same field appears
several times in the serialized bytes array and the behavior for that is well
defined (as the above limitations).

# Field types

|proto field type|append_***() method param's required bound|
|----------------|---------------------------------|
|singular numeric (e.g. `int32`)| `Into<i32> + Clone` |
|singular bytes   | `AsRef<[u8]>` |
|singular string  | `AsRef<str>`  |
|singular message (e.g. `message Bar`) | `BarTrait` |
|repeated numeric | `IntoIterator<Item = impl Into<i32> + Clone>` |
|repeated bytes   | `IntoIterator<Item = impl AsRef<[u8]>>` |
|repeated string  | `IntoIterator<Item = impl AsRef<str>>` |
|repeated message | `IntoIterator<Item = impl BarTrait>` |

# Samples
File [`tests/src/cases/builder.rs`](https://github.com/wada314/puroro/blob/master/tests/src/cases/builder.rs)
contains some test cases against `tests-pb/protos/full-coverage3.proto` .
