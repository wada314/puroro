# Rust protobuf implementation strategy using a `GenericMessage` type and `Wrapper` types

## Background & motivations
It is not too difficult to mimic a C++'s protobuf implementation in Rust.
For example, for a simple .proto file like this:

```protobuf
syntax = "proto3";
message Person {
    string name = 1;
    Person partner = 2;
    repeated Person children = 3;
}
```

... then the generated rust `struct` will be something like this:

```rust
pub struct Person {
    name: String,
    partner: Option<Box<Person>>,
    children: Vec<Person>,
}
impl Person {
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    pub fn partner(&self) -> Option<&Person> {
        self.partner.as_deref()
    }
    pub fn children(&self) -> &[Person] {
        self.children.as_slice()
    }
    pub fn set_name(&mut self, value: &str) {
        self.name = value.to_string()
    }
    // And so on...
}
```

For the internal members of `struct Person`, we still have a lots of space to optimize, but that's not a main topic here. 

The point I want to point out is that the `Person` class is owning 2 responsibilites:
- Provide a user-facing methods interface for `Person` class (e.g. `pub fn name(&self) -> str`),
- And manage the data in the `Person` class.

This might looks like trivial and are difficult to separate, but I think these 2 function can be separated.
We introduce a trait `GenericMessage` (and `GenericField`), which is a trait providing a common interface for any message type storages.

```rust
pub trait GenericMessage {
    type FieldType<'a, const NUMBER: usize>: GenericField<'a> where Self: 'a;
    fn field<const NUMBER: usize>(&self) -> Self::FieldType<'_, NUMBER>;
}

pub trait GenericField<'a> {
    fn get_str(&'a self) -> Result<&'a str, ()>;

    type MessageType: GenericMessage;
    fn get_message(&'a self) -> Result<Self::MessageType, ()>;

    type RepeatedMessageType: Iterator<Item=Self::MessageType>;
    fn get_repeated_message(&'a self) -> Result<Self::RepeatedMessageType, ()>;

    // And so on...
}
```

Then, for the proto's each message type, generate a wrapper type:
```rust
pub struct PersonWrapper<M>(M);
impl<M: GenericMessage> PersonWrapper<M> {
    pub fn name(&self) -> &str {
        self.0.field::<1>().get_str().unwrap()
    }
    pub fn partner(&self) -> Option<PresonWrapper<impl GenericMessage>> {
        self.0.field::<2>().get_message().unwrap().map(|m| PersonWrapper(m))
    }
    pub fn children(&self) -> impl Iterator<Item = PersonWrapper<impl GenericMessage>> {
        self.0.field::<3>().get_repeated_message().unwrap().map(|m| PersonWrapper(m))
    }
}
```

So that this `PersonWrapper` struct can have almost the same interface as the previous straightforward `Person` struct.
And we can expect this indirection can be optimized out by compiler (unconfirmed) becasue every type / method parameters are statically given and there are no dynamic code (e.g. `dyn Trait` ).

For now, I skip for the actual implementation sample of `GenericMessage` and `GenericField`.

So what's a good point of this redundant implementation?
One good thing is that, we can customize the actual implementation of any messages.
Starting from simple ones:

```rust
/// Zero-space empty message impl
impl GenericMessage for () {
    type FieldType<'a, const NUMBER: usize> = () where Self: 'a;
    fn field<const NUMBER: usize>(&self) -> Self::FieldType<'_, NUMBER> {
        ()
    }
}
impl<'a> GenericField<'a> for () {
    fn get_str(&'a self) -> Result<&'a str, ()> {
        Ok("")
    }
    type MessageType = ();
    fn get_message(&'a self) -> Result<Self::MessageType, ()> {
        Ok(())
    }
    type RepeatedMessageType: ::std::iter::Empty<()>;
    fn get_repeated_message(&'a self) -> Result<Self::RepeatedMessageType, ()> {
        ::std::iter::empty()
    }
}

/// Reference (also mut ref version is needed)
impl<'a, M: GenericMessage> GenericMessage for &'a M {
    type FieldType<'b, const NUMBER: usize> = <M as GenericMessage>::FieldType<'b, NUMBER> where Self: 'b;
    fn field<const NUMBER: usize>(&self) -> Self::FieldType<'_, NUMBER> {
        <M as GenericMessage>::field(self)
    };
}

/// Option<M: GenericMessage> can also be a GenericMessage.
impl<M: GenericMessage> GenericMessage for Option<M> {
    type FieldType<'a, const NUMBER: usize> = Option<<M as GenericMessage>::FieldType<'a, NUMBER>> where Self: 'a;
    fn field<const NUMBER: usize>(&self) -> Self::FieldType<'_, NUMBER> {
        self.as_ref().map(|m| <M as GenericMessage>::field(m))
    }
}
impl<'a, F: GenericField> GenericField<'a> for Option<F> {
    fn get_str(&'a self) -> Result<'a str, ()> {
        Ok(self.as_ref().map(|f| <F as GenericField>::get_str(f)).unwrap_or(""))
    }
}
```


