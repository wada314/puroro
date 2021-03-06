# proto2 default values
Proto2 has a feature to set a default value like this:

`optional int32 foo = 1; [default = 42]`

However, in puroro implementation there is a difficulty to implement this. This is because of our initial design decision -- map `optional int32` type into `Option<i32>`. This looks like very straightforward in Rust, but the official C++ and Java (I haven't checked others but I assume the others are the same) implementation are using 2 fields, the bare value `int32_t` field and the field existence `bool` value field.
What's the difference between these two implementations? That is, where the official implementations allow a status that "the field is cleared but the value is available" but our implementation does not. And the proto2 default value behavior is tightly connected to this implementation. The proto2 default value is explicitly set to the field when the message structure is constructed, or when the `clear_foo()` method is called, and in the both cases the field existence bit is cleared. This makes lots of interesting (and good) behaviors:

 1. When the message class is constructed by a default constructor and then immediately be serialized, then no fields are serialized because the field existence bit is not set.
 1. Though, if the user explicitly set the field value to `42`, then the field is serialized because the field existence bit is set.
 1. When deserializing the message, if the field does not exist in the input bytes then the field value is not set and the `get_foo()` methods returns the value which was available before deserialized. Very natural behavior.

It is difficult to reproduce all these behaviors in puroro as long as we are using `Option<i32>` as a type of the field and as a return type of trait's getter methods. There are few options I can take:

## Option 1. Give up using `Option<T>` and use a pair of `T` and `bool` fields
This is almost identical to give up using a public field struct for the generated message struct. I was not like this first, but after a while I'm starting to think we should stop using a public fields anyway.

## Option 2. Set field default values in `Default::default()` method
This looks like a most straightforward way, but the behavior will be different with the official protobuf implementation in 1. above. Also, even when the user explicitly set `None` as a field value, the default value is not set. This is a different behavior with the official implementation but it's more natural to Rust language users so I think it's acceptable. User can retrieve the default value anytime using a code like this:

```rust
// imagine we have a field:
// optional int32 foo = 1; [default = 42]
let value_or_default = my_message.foo.or(MyMessage::default().foo).unwrap();
```

Though this requires `.unwrap()` so it might be a option to define an associated constant with bare (non-optional) default value.

### Subproblem: Trait getter method behavior

Another problem rose here is the behavior of trait getter methods. If the field value is `None` internally, should the getter method return the default value or just return `None`? Similar question is, if the field is undefined for the impl type (e.g. `()`), then should the getter method return the default value or `None`?

In `impl MyMessageTrait for (T, U)` code's scalar getter methods, we are doing something like checking the `U`'s getter method return value and if it's default then return `T`'s getter method return value. For this default check behavior, we must strictly define what the default value is.

If we consider `Some(42)` also be a default value in addition to `None`, then we need a code change in few places but it's acceptable. The trait methods *can* return `Some(42)` when the internal field value is `None` or not exist. 
We might also want to ignore the `Some(42)` field when serializing the message, but this is a bad idea I think because the user loses a way to explicitly serialize `Some(42)` value (remember, this is possible in the official protobuf implementation). These different behaviors between `(T, U)` and serialization might make the users confused.

If we do not consider `Some(42)` as a default value, then the trait methods can only return `None` as-is for the `None` fields or not existing fields. This can cause another confusion to the user:
```rust
use MyMessageTrait;
assert_eq!(Some(42), MyMessage::default().foo());
assert_eq!(None, ()::default().foo()); // (did you know `()` implements `Default`?)
```
Why these two almost identical code behaves differently? We can only say that because `MyMessage` has a field value instance but `()` does not. This is not clear to the user...

## Option 3. Define a custom `Option<T>` type

~~Seriously?~~

Maybe we can copy-and-paste the `Option<T>` code and add something like this:

```rust
// name TBD
pub enum NewOption<T> {
    Some(T),
    None(T), // <= What!?
}
impl<T> NewOption<T> {
    pub fn unwrap_or_default(self) -> T {
        match self {
            NewOption::Some(t) => t,
            NewOption::None(t) => t,
        }
    }
    pub fn take_some(self) -> Option<T> {
        match self {
            NewOption::Some(t) => Some(t),
            NewOption::None(_) => None,
        }
    }
}
```

If the field does not have the default value then we can set the protobuf-defined default value to the `None(T)` variant. It doesn't add memory footprint because we already have `Some(T)` variant. Hooray, it works! ...Really??
Of course normally it's not a good idea to write a custom version of language core library feature.

```rust
// begin generated code
pub struct MyMessage {
    // Probably this can also be `NewOption<i32>`
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
// end generated code

let msg = MyMessage::default();
assert_eq!(42, msg.foo().unwrap_or_default());
assert_eq!(None, msg.foo().take_some());
```

It would be better to limit the use of this `NewOption<T>` fields / getters only for proto2 default_value fields.
The user who got this value explicitly need to select if they want a maybe-default value (via `unwrap_or_default()` method above), or make sure to get only an explicitly set value (via `take_some()` method above).

I'm starting to feel like this is a possible solution, though the struct's raw field users (who don't use trait's getter methods) still cannot get benefit of the default value.

Another similar idea is to encode the default value into the custom option type's const generic param, though as of the current rust version (2021 Oct.) the const generic param supports only integral types, not float or string types. ([rfc](https://github.com/rust-lang/rfcs/blob/master/text/2000-const-generics.md))

## My thoughts

Initially I designed the bare fields struct (`struct MyMessage`) for easy to use and it does not suit for performance critical purpose anyway. So I think the struct's code should be simple and straightforward. Let's just keep using `Option<T>` for the default value fields, and set the default value when it's constructed via `Default::default()` method.

OTOH, the getter trait (`trait MyMessageTrait`) should be able to accept the heavily performance tuned implementation. The current trait interface for proto2 `optional` field returns `Option<T>`, though this interface cannot express the state "The field is not set but the default value is available" as I mentioned above.
Probably I should redesign the getter method and split it to two methods; `foo(&self) -> T` and `has_foo(&self) -> bool`. One of the concern for this method is name conflict. How C++ and Java are treating the case that 2 fields named `foo` and `has_foo` are existing?
    ==> Actually c++ compiler is not caring about the name conflict at all... Then it might be okay that our library to generate `foo` and `has_foo`, and maybe more like `clear_foo` and `foo_mut` etc...

## My thoughts V2

If I give up a public field message struct then this issue gonna be solved. Actually how much important to keep the struct public fields? The readability won't change so much I guess... Accessing the fields immutably just needs two more characters "()". Accessing the fields mutablly needs few more characters ("_mut()"), but anyway it's not a big deal.
The biggest actual code writing benefit for the public field struct is the [split borrowing](https://doc.rust-lang.org/nomicon/borrow-splitting.html). This is not possible to do with the normal `msg.foo()` and `msg.foo_mut()` accessor method structs. Though, I'm starting to think it's possible if we make a certain interface for it. I don't have a clear picture yet, but for example:

```rust
pub trait FieldTypesGen {
    type Field1Type = (); // Or `!` probably?
    type Field2Type = ();
}
pub struct MsgMultipleFieldGetter<T: FieldTypeGen> {
    pub first_field: T::Field1Type,
    pub second_field: T::Field2Type,
}
```
And then create a recursive implementation of `FieldTypesGen` which takes another `FieldTypesGen` and just override a single field's type to an actual type (e.g. `&mut i32`, where field1 type is i32). Then the struct `MsgMultipleFieldGetter` can have a minimum memory footprint only used by a user requested field.

(TBD)

# Why `MsgTrait`'s message field getter returns by value, not by reference

```rust
trait MsgTrait {
    // The current impl
    type Field1MessageType<'this>: AnotherMsgTrait;
    fn field_1(&self) -> Option<Self::Field1MessageTrait<'_>>;
    
    // Why we not do this?
    type Field2MessageType: AnotherMsgTrait;
    fn field_2(&self) -> Option<&'_ Self::Field2MessageTrait<'_>>;
}
```

Because in some implementation of `MsgTrait` it cannot return by value. For example, `Either<U, T>` type implements `MsgTrait`, and that implementation will be like this:

```rust
impl<T, U> MsgTrait for Either<T, U>
where
    T: MsgTrait,
    U: MsgTrait,
{
    type Field1MessageType<'this>: = Either<
        T::Field1MessageType<'this>,
        U::Field1MessageType<'this>,
    >;
    fn field_1(&self) -> Option<Self::Field1MessageTrait<'_>> {
        self.as_ref().either(
            |t| t.field_1().map(|f1| Either::Left(f1)),
            |u| u.field_1().map(|f1| Either::Right(f1)),
        ) // : Option<Either<T::Field1MessageType<'this>, U::Field1MessageType<'this>>>
    }
}
```

As you can see, there are no way to implement `field_2()` in `Either<T, U>`. It always need to create a new `Either` instance to return, so the reference is not available.

