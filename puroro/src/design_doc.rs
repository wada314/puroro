/*!
Design related notes and documents. No code here.

# Bumpalo message

[`Bumpalo`](https://docs.rs/bumpalo) is an arena (or a.k.a. allocator) library
which has relatively simple implementation.
I'm using this library in puroro to try implementing custom allocator messages.

## Basic 'bumpalo' providing classes

- `Bump`, which is a memory allocator instance.
The memory allocated from this instance remains until the allocator instance is get dropped.

- `Box<'bump, T>`, an analogus to `std::boxed::Box<T>`.
It has an initializer method `::new_in(t: T, bump: &'bump Bump)`,
which takes the reference to the `Bump` allocator with lifetime `'bump`.
Though `Box` does not keep `&'bump Bump` after the initialization is done because
`Box<'bump, T>` has no chance to reallocate the buffer.
The lifetime `'bump` of `Box<'bump, t>` indicates:
"This Box cannot outlive the source Bump instance which was given when I was allocated."

- `String<'bump>, Vec<'bump, T>` Almost the same as `Box<'bump, T>`, analogus to the
standard library's `String` and `Vec`. Just one difference with the `Box` is that
these classes need to use the allocator instance even after the class is initialized,
(e.g. when the vector size is extended) so they keep own `&'bump Bump` reference inside it.


## First attempt
So what will happen if I say "Use bumpalo in my protobuf implementation"?
The first step is quite simple, just replace the all `Box`s, `Vec`s and `String`s into
the bumpalo's one.

```protobuf
sytnax = "proto3"
message Person {
    string name = 1;
    Person partner = 2;
    repeated Person children = 3;
}
```

And translate this to a rust struct.
For now let's think for a simple public-field struct, because the interfacing is not the topic at here.

```rust
use bumpalo::{Bump, boxed::Box, collections::{Vec, String}};

pub struct Person<'bump> {
    pub name: String<'bump>,
    pub partner: Option<Box<'bump, Person<'bump>>>,
    pub children: Vec<'bump, Person<'bump>>,
    _bump: &'bump Bump,
}
```

why do we need `_bump: &'bump Bump` field stored? There are two reasons:
1. The default value of `Person::partner` is `None`, so when editing it
we need to allocate `Box<'bump, T>` using the `_bump` field.
2. As a similar reason, when we edit this struct we need the `_bump` field
so that `Person` structs in `partner` and `children` fields can be initialized.

This implementation is very straightforward but has a little problem.
When we use this implementation we always need to keep a `Bump` instance
somewhere and manage it so that it does not get dropped before any message gets dropped.

Generally speaking, if a tree of message is allocated by a single allocator,
we can imagine several strategies about who owns the allocator instance.

1. No one owns. As like the example above, just refer an external instance.
Or the allocator instance can be a static value like a global allocator.
2. Everyone owns. By wrapping the allocator by `Rc` or `Arc`, we can make sure
the allocator is never dropped until and surely dropped when the all message struct
instances are dropped.
3. Only root owns. The root message owns the allocator, and the children messages
all refer to the root allocator instance.

We already see no.1 (no one owns) example above, so let's try implement the others.
Let's write an implementation which accepts 1 and 2:

```rust
use bumpalo::{Bump, boxed::Box, collections::{Vec, String}};
use std::ops::Deref;
use std::rc::Rc;

pub struct Person<'bump, B> {
    pub name: String<'bump>,
    pub partner: Option<Box<'bump, Person<'bump, B>>>,
    pub children: Vec<'bump, Person<'bump, B>>,
    _bump: B,
}
// Straightforward
type PersonRef<'bump> = Person<'bump, &'bump Bump>;
// No lifetime param we can use here except 'static, but it's cheaty
type PersonRc = Person<'static, Rc<Bump>>;

impl<'bump, B: Deref<Target=Bump>> Person<'bump, B> {
    pub fn new_in(bump: B) -> Self {
        let bump_ref: &Bump = unsafe {
            std::mem::transmute(bump.deref())
        };
        Self {
            name: String::new_in(bump_ref),
            partner: None,
            children: Vec::new_in(bump_ref),
            _bump: bump,
        }
    }
}

pub fn main() {
    // This is okay
    let person_rc = {
        let bump = Rc::new(Bump::new());
        PersonRc::new_in(bump)
    };

    // This is not, bump dies earlier than the person.
    // let person_ref = {
    //     let bump = Bump::new();
    //     PersonRef::new_in(&bump)
    // };

    // Another bad case, dangling pointer
    // This happens because we lie the lifetime as 'static
    let name = {
        let bump = Rc::new(Bump::new());
        let person_rc = PersonRc::new_in(bump);
        person_rc.name
    };
}
```

 */
