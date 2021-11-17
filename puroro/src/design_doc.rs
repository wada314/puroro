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

- `String<'bump>, Vec<'bump, T>` Similar to the `Box<'bump, T>`, analogus to the
standard library's `String` and `Vec`. Just one difference with the `Box` is that
these classes need to use the allocator instance even after the class is initialized,
(e.g. when the vector size is extended) so they keep own `&'bump Bump` reference inside it.
This is actually redundant in our case because our message also keeps own the pointer to `Bump`,
but let's ignore it for now...

## First attempt
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

## Second attempt, give `<B: Deref<Target=Bump>>`

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

// Straightforward.
pub type PersonRef<'bump> = Person<'bump, &'bump Bump>;

// No lifetime param we can use here except 'static,
// but it's cheaty and later it actually makes some problems...
pub type PersonRc = Person<'static, Rc<Bump>>;
```

This looks pretty straightforward except we are using `'static`.
But one of the problem in this code comes up when you write an
initializer function:

```ignore
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
```

We needed to introduce an `unsafe` block to make the compile pass here.
Unless using the `unsafe` block, we cannot initialize the `Person::_bump` field
because the local variable `bump` is already borrowed in previous fields initializations
so it cannot be moved.
Instead, we are manually cutting off the borrow checker using `std::mem::transmute`
function so that the borrow checker thinks the variable `bump` is "free" for moving.

```ignore
pub fn main() {
    // This is okay
    let person_rc = {
        let bump = Rc::new(Bump::new());
        PersonRc::new_in(bump)
    };

    // This results expected compile error. bump dies earlier than the person.
    // Good catch lifetime!.
    // let person_ref = {
    //     let bump = Bump::new();
    //     PersonRef::new_in(&bump)
    // };

    // Bad case, dangling pointer
    // This happens because we lied the lifetime as 'static
    let name = {
        let bump = Rc::new(Bump::new());
        let person_rc = PersonRc::new_in(bump);
        person_rc.name
    };
    println!("{}", name); // bad pointer!
}
```

The problem here is the last block of the code.
Because we lied the lifetime passed to `String` as `'static`,
compiler thinks it's actually movable out from the `PersonRc` struct.
Of course this is not correct, the code above is making a dangling pointer.
Because of this, the struct fields cannot be public anymore --
the user need to access the fields via getter methods which converts type into safe one:

```ignore
impl<'bump, B> Person<'bump, B> {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn name_mut(&mut self) -> &mut String<'_> {
        unsafe { std::mem::transmute(&mut self.name) }
    }
}

fn main() {
    // Because we hide the raw struct fields, this code compiles error correctly!
    // let name = {
    //     let bump = Rc::new(Bump::new());
    //     let mut person_rc = PersonRc::new_in(bump);
    //     person_rc.name_mut()
    // };

    // But this code also become compile error, which was okay in public fields...
    // let bump = Bump::new();
    // let name = {
    //     let mut person_ref = PersonRef::new_in(&bump);
    //     person_ref.name_mut().clone()
    // };

    // The okay version of the above, using the public field
    let bump = Bump::new();
    let name = {
        let mut person_ref = PersonRef::new_in(&bump);
        person_ref.name
    }; // because `bump` is still alive here, so the `name` is also alive.
}
```

Again, we are using `std::mem::transmute` in the mutable getter method.
We need this because the field type is `String<'bump>`, but the return type is `String<'_>`
(where the lifetime `'_` is the lifetime of the `Person` struct itself),
and the compiler doesn't know the relation between those 2 lifetimes.
You can make the code compile by adding `'_: 'bump` (actually you need to name the lifetime instead of `'_`) bound,
but remind that we are giving `'static` as a lifetime parameter for `PersonRc` struct.
If we add the bound above, the method become only callable for static variable!

As a side effect of introducing the `unsafe` conversion, the 2nd example in above code
become uncompilable. In this case the lifetime `'bump` actually outlives the lifetime
of the `PersonRef` struct, but because `PersonRc` struct lied about the `'bump` parameter
so our getter methods decided to not trust the `'bump` param and instead use the
struct's lifetime itself (`'_`). Sad...

## Third attempt

The last iteration is to implement the 3rd strategy I listed above,
the root meessage owns the allocator and the children refers it.
Let's say the root message owns the allocator by `::std::boxed::Box<Bump>`.
If we let it own the allocator by raw `Bump` type instead,
then the struct become unmovable and needed to be `Pin`ned.
That might be another possible choice but I don't discuss about it in here.

An unresolved question I have here is that if it's possible to own the allocator
using `bumpalo::boxed::Box<'static, Bump>`, which means allocate your allocator's
memory using the same allocator. Of course it needs to use `unsafe` code for
initialization but that should be actually safe. Though I'm not sure what will
happen when dropping it...

Returning to the struct implementation. In the 3rd strategy the `Bump` pointer type
might be different between a message struct and its child message structs,
so the struct needs 4 type info: the struct itself's `Bump` ptr type,
the child message struct's `Bump` ptr type, a converter from the former to the latter,
and the recursive set of 4 type params for the child message.

```ignore
use bumpalo::{Bump, boxed::Box, collections::{Vec, String}};
use std::ops::Deref;
use std::rc::Rc;
use std::marker::PhantomData;

pub trait BumpTypes {
    type BumpPtr: Deref<Target = Bump>;
    type ChildsBumpPtr<'parent>: Deref<Target = Bump>;
    fn conv(from: &Self::BumpPtr) -> Self::ChildsBumpPtr<'_>;
    type ChildBumpTypes<'parent>: BumpTypes;
}
struct CloningBumpType<B>(PhantomData<B>);
impl<B: Deref<Target = Bump> + Clone> BumpTypes for CloningBumpType {
    type BumpPtr = B;
    type ChildsBumpPtr<'parent> = B;
    fn conv(from: &Self::BumpPtr) -> Self::ChildsBumpPtr<'_> {
        from.clone()
    }
    type ChildBumpTypes<'parent> = Self;
}
struct BoxBumpTypes;
impl BumpTypes for BoxBumpTypes {
    type BumpPtr = std::boxed::Box<Bump>;
    type ChildsBumpPtr<'parent> = &'parent Bump;
    fn conv(from: &Self::BumpPtr) -> Self::ChildsBumpPtr<'_> {
        from.as_ref()
    }
    type ChildBumpTypes<'parent> = CloningBumpType<&'parent Bump>;
}

pub struct Person<'bump, BT: BumpTypes> {
    pub name: String<'bump>,
    pub partner: Option<Box<'bump, Person<'bump, BT::ChildBumpTypes<'bump>>>>,
    pub children: Vec<'bump, Person<'bump, BT::ChildBumpTypes<'bump>>>,
    _bump: BT::BumpPtr,
}

pub type PersonRef<'bump> = Person<'bump, CloningBumpType<&'bump Bump>>;
pub type PersonRc = Person<'static, CloningBumpType<Rc<Bump>>>;
pub type PersonBox = Person<'static, BoxBumpTypes>;
```

First of all, COMPILING THIS CODE MAKES COMPILER HANG FOREVER SO DO NOT COMPILE IT.

 */
