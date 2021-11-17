// Copyright 2021 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Bumpalo message structs
//!
//! **The implementation is highly experimental and the interface will change
//! in very soon. I'm planning to make the struct fields private and make a mutable
//! trait interface in future.**
//!

pub mod de;

use crate::bumpalo::collections::{String, Vec};
use crate::bumpalo::Bump;
use ::stable_deref_trait::StableDeref;
use ::std::fmt::Debug;
use ::std::ops::Deref;
use ::std::rc::Rc;
use ::std::sync::Arc;

pub trait BumpaloDefault<'bump> {
    fn default_in(bump: &'bump Bump) -> Self;
}
impl<'bump> BumpaloDefault<'bump> for String<'bump> {
    fn default_in(bump: &'bump Bump) -> Self {
        String::new_in(bump)
    }
}
impl<'bump, T> BumpaloDefault<'bump> for Vec<'bump, T> {
    fn default_in(bump: &'bump Bump) -> Self {
        Vec::new_in(bump)
    }
}
impl<'bump, T> BumpaloDefault<'bump> for Option<T> {
    fn default_in(_: &'bump Bump) -> Self {
        ::std::default::Default::default()
    }
}
macro_rules! impl_bumpalo_default {
    ($ty:ty) => {
        impl<'bump> BumpaloDefault<'bump> for $ty {
            fn default_in(_: &'bump Bump) -> Self {
                Default::default()
            }
        }
    };
}
impl_bumpalo_default!(i32);
impl_bumpalo_default!(u32);
impl_bumpalo_default!(f32);
impl_bumpalo_default!(i64);
impl_bumpalo_default!(u64);
impl_bumpalo_default!(f64);
impl_bumpalo_default!(bool);

/// A trait decides how the bumpalo message struct hold the
/// pointer to the `Bump` instance.
///
/// Basically, 2 types are defined in this trait:
///
/// - A type that THIS message struct should use to point the `Bump`
/// instance. e.g. `Rc<Bump>`, `&Bump`
/// - Same trait bound `BumpType` for the child message structs,
/// which decides the child struct's bump ptr type and grandchild's
/// `BumpType`, recursively.
pub trait BumpTypes {
    type BumpRef<'bump>: Sized + StableDeref + Deref<Target = Bump> + Debug;
    unsafe fn cast_ref_lt_unsafe<'short, 'long: 'short>(
        input: Self::BumpRef<'short>,
    ) -> Self::BumpRef<'long>;

    type ChildsBumpTypes: BumpTypes + Debug + PartialEq;
    fn make_bump_for_child<'bump>(
        bump_parent: &'bump Self::BumpRef<'bump>,
    ) -> <Self::ChildsBumpTypes as BumpTypes>::BumpRef<'bump>;
}

/// Use `Rc<Bump>` to point the `Bump`. Same for the children.
#[derive(PartialEq, Debug)]
pub struct BumpRc;
impl BumpTypes for BumpRc {
    type BumpRef<'bump> = Rc<Bump>;
    unsafe fn cast_ref_lt_unsafe<'short, 'long: 'short>(
        input: Self::BumpRef<'short>,
    ) -> Self::BumpRef<'long> {
        input // as-is
    }
    type ChildsBumpTypes = Self;
    fn make_bump_for_child<'bump>(
        bump_parent: &'bump Self::BumpRef<'bump>,
    ) -> <Self::ChildsBumpTypes as BumpTypes>::BumpRef<'bump> {
        bump_parent.clone()
    }
}

/// Use `Arc<Bump>` to point the `Bump`. Same for the children.
#[derive(PartialEq, Debug)]
pub struct BumpArc;
impl BumpTypes for BumpArc {
    type BumpRef<'bump> = Arc<Bump>;
    unsafe fn cast_ref_lt_unsafe<'short, 'long: 'short>(
        input: Self::BumpRef<'short>,
    ) -> Self::BumpRef<'long> {
        input // as-is
    }
    type ChildsBumpTypes = Self;
    fn make_bump_for_child<'bump>(
        bump_parent: &'bump Self::BumpRef<'bump>,
    ) -> <Self::ChildsBumpTypes as BumpTypes>::BumpRef<'bump> {
        bump_parent.clone()
    }
}

/// Use `&Bump` to point the `Bump`. Same for the children.
/// Note you'll need to keep the instance of `Bump` at somewhere
/// else if you use this ptr type.
#[derive(PartialEq, Debug)]
pub struct BumpRef;
impl BumpTypes for BumpRef {
    type BumpRef<'bump> = &'bump Bump;
    unsafe fn cast_ref_lt_unsafe<'short, 'long: 'short>(
        input: Self::BumpRef<'short>,
    ) -> Self::BumpRef<'long> {
        ::std::mem::transmute(input) // `&'short T` => `&'long T`, which is dangerous
    }
    type ChildsBumpTypes = BumpRef;
    fn make_bump_for_child<'bump>(
        bump_parent: &'bump Self::BumpRef<'bump>,
    ) -> <Self::ChildsBumpTypes as BumpTypes>::BumpRef<'bump> {
        *bump_parent
    }
}

/// Use `Box<Bump>` to point the `Bump`. The children will use
/// `&Bump` instead of the box because the box is not allowed to
/// be shared.
#[derive(PartialEq, Debug)]
pub struct BumpBox;
impl BumpTypes for BumpBox {
    type BumpRef<'bump> = Box<Bump>;
    unsafe fn cast_ref_lt_unsafe<'short, 'long: 'short>(
        input: Self::BumpRef<'short>,
    ) -> Self::BumpRef<'long> {
        input // as-is
    }
    type ChildsBumpTypes = BumpRef;
    fn make_bump_for_child<'bump>(
        bump_parent: &'bump Self::BumpRef<'bump>,
    ) -> <Self::ChildsBumpTypes as BumpTypes>::BumpRef<'bump> {
        bump_parent.as_ref()
    }
}

/// Bumpalo message, initialized from bump ptr instance.
pub trait BumpaloMessage<'bump> {
    type BumpTypes: BumpTypes;
    fn new_with_parents_bump<ParentsBT>(
        parents_bump: &'bump <ParentsBT as BumpTypes>::BumpRef<'bump>,
    ) -> Self
    where
        ParentsBT: BumpTypes<ChildsBumpTypes = Self::BumpTypes>;
}
impl<'bump, T> BumpaloMessage<'bump> for crate::bumpalo::boxed::Box<'bump, T>
where
    T: BumpaloMessage<'bump>,
{
    type BumpTypes = T::BumpTypes;
    fn new_with_parents_bump<ParentsBT>(
        parents_bump: &'bump <ParentsBT as BumpTypes>::BumpRef<'bump>,
    ) -> Self
    where
        ParentsBT: BumpTypes<ChildsBumpTypes = Self::BumpTypes>,
    {
        crate::bumpalo::boxed::Box::new_in(
            BumpaloMessage::new_with_parents_bump::<ParentsBT>(parents_bump),
            parents_bump, // Use the parent's bump to alloc the `Box`'s field.
        )
    }
}
