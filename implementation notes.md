# proto2 default values
Proto2 has a feature to set a default value like this:

`optional int32 foo = 1; [default = 42]`

However, in puroro implementation there is a difficulty to implement this. This is because of our initial design decision -- map `optional int32` type into `Option<i32>`. This looks very straightforward, but the official C++ and Java (I haven't checked others but I assume the others are the same) implementation are using 2 fields, the bare value `int32_t` field and the field existence `bool` value field.
What's the difference between these two implementations? That is, where the official implementations allow a status that "the field is cleared but the value is available" but our implementation does not. And the proto2 default value behavior is tightly connected to this implementation. The proto2 default value is explicitly set to the field when the message structure is constructed, or when the `clear_foo()` method is called, and in the both cases the field existence bit is cleared. This makes lots of interesting (and good) behaviors:

 1. When the message class is constructed by a default constructor and then immediately be serialized, then no fields are serialized because the field existence bit is not set.
 1. Though, if the user explicitly set the field value to `42`, then the field is serialized because the field existence bit is set.
 1. When deserializing the message, if the field does not exist in the input bytes then the field value is not set and the `get_foo()` methods returns the value which was available before deserialized. Very natural behavior.

It is difficult to reproduce all these behaviors in puroro as long as we are using `Option<i32>` as a type of the field and as a return type of trait's getter methods. There are few options I can take:

## Option 1. Give up using `Option<T>` and use a pair of `T` and `bool` fields
This is almost identical to give up using a public field struct for the generated message struct. At least for now, I don't want to do that.

## Option 2. Set field default values in `Default::default()` method
This looks like a most straightforward way, but the behavior will be different with the official protobuf implementation in 1. above. Also, even when the user explicitly set `None` as a field value, the default value is not set. This is a different behavior with the official implementation but it's more natural to Rust language users so I think it's acceptable. User can retrieve the default value anytime using a code like this:

```rust
// imagine we have a field:
// optional int32 foo = 1; [default = 42]
let value_or_default = my_message.foo.or(MyMessage::default().foo).unwrap();
```

Though this requires `.unwrap()` so it might be a option to define an associated constant with bare (non-optional) default value.

### Subproblem: Trait getter method behavior

Another problem rose here is the behavior of trait getter methods. If the field is `None` internally, should the getter method return the default value or just return `None`? Similar question is, if the field is undefined for the impl type (e.g. `()`), then should the getter method return the default value or `None`?

In `impl MyMessageTrait for (T, U)` code's scalar getter methods, we are doing something like checking the `U`'s getter method return value and if it's default then return `T`'s getter method return value. For this default check behavior, we must strictly define what the default value is.

If we consider `Some(42)` also be a default value in addition to `None`, then we need a code change in few places but it's acceptable. The trait methods *can* return `Some(42)` when the internal field value is `None` or not exist. 
We might also want to ignore the `Some(42)` field when serializing the message, but this is a bad idea I think because the user loses a way to explicitly serialize `Some(42)` value (remember, this is possible in the official protobuf implementation). These different behaviors between `(T, U)` and serialization might make the users confused.

If we do not consider `Some(42)` as a default value, then the trait methods can only return `None` as-is for the `None` fields or not existing fields. This can cause another confusion to the user:
```rust
use MyMessageTrait;
assert_eq!(Some(42), MyMessage::default().foo());
assert_eq!(None, ()::default().foo()); // (did you know `()` implements `Default`?)
```
Why these two almost same code behaves differently? We can only say that because `MyMessage` has a field value instance but `()` does not. This is not clear to the user...

## Option 3. Define a custom `Option<T>` type

Seriously?

Yes, it's true that maybe we can copy-and-paste the `Option<T>` code and add something like this:

```rust
pub enum NewOption<T> {
    Some(T),
    None(T), // <= What!?
}
impl<T> NewOption<T> {
    pub fn unwrap_or_default(self) -> T {
        match self {
            Some(t) => t,
            None(t) => t,
        }
    }
}
```

If the field does not have the default value then we can set the protobuf-defined default value to the `None(T)` variant. It doesn't add memory footprint because we already have `Some(T)` variant. Hooray, it works! ...Really??

Of course normally it's not a good idea to write a custom version of language core library feature.

```rust
// generated code
pub struct MyMessage {
    pub foo: Option<i32>,
}
impl MyMessageTrait for MyMessage {
    fn foo(&self) -> NewOption<i32> {
        match self.foo {
            Some(x) => NewOption::Some(x),
            None => NewOption::None(42),
        }
    }
}
```

