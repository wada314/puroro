use crate::types::SliceViewField;
use crate::{bumpalo, hashbrown};
use std::borrow::Cow;
use std::collections::HashMap;

/// We need this JUST ONLY for `bumpalo::boxed::Box`, because it doesn't have
/// the reference to the bumpalo instance so it cannot clone themself.
pub trait FieldClone<'bump>: Sized {
    fn clone(&self) -> Self;
    #[cfg(feature = "puroro-bumpalo")]
    fn clone_in_bumpalo(&self, _bump: &'bump bumpalo::Bump) -> Self {
        <Self as FieldClone>::clone(self)
    }
}

#[cfg(feature = "puroro-bumpalo")]
impl<'bump, T: Clone> FieldClone<'bump> for bumpalo::boxed::Box<'bump, T> {
    fn clone(&self) -> Self {
        panic!("bumpalo box needs a bumpalo instance to clone!");
    }
    #[cfg(feature = "puroro-bumpalo")]
    fn clone_in_bumpalo(&self, bump: &'bump bumpalo::Bump) -> Self {
        bumpalo::boxed::Box::new_in(self.as_ref().clone(), bump)
    }
}

impl<'bump, T: FieldClone<'bump>> FieldClone<'bump> for Option<T> {
    fn clone(&self) -> Self {
        match self {
            Some(x) => Some(<T as FieldClone>::clone(x)),
            None => None,
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    fn clone_in_bumpalo(&self, bump: &'bump bumpalo::Bump) -> Self {
        match self {
            Some(x) => Some(<T as FieldClone>::clone_in_bumpalo(x, bump)),
            None => None,
        }
    }
}

macro_rules! define_field_clone {
    ($ty:ty) => {
        impl<'bump> FieldClone<'bump> for $ty {
            fn clone(&self) -> Self {
                <Self as Clone>::clone(self)
            }
        }
    };
    ($ty:ty, < $( $($gp:ident)? $($lt:lifetime)? $( : $bound:ident)? ),* >) => {
        impl<'bump $(, $($gp)? $($lt)? $( : $bound)?)+ > FieldClone<'bump> for $ty {
            fn clone(&self) -> Self {
                <Self as Clone>::clone(self)
            }
        }
    };
}
define_field_clone!(i32);
define_field_clone!(i64);
define_field_clone!(u32);
define_field_clone!(u64);
define_field_clone!(f32);
define_field_clone!(f64);
define_field_clone!(bool);
define_field_clone!(String);
define_field_clone!(std::result::Result<T, i32>, <T: Clone>);
define_field_clone!(Box<T>, <T: Clone>);
define_field_clone!(Vec<T>, <T: Clone>);
define_field_clone!(HashMap<K, V>, <K: Clone, V: Clone>);
#[cfg(feature = "puroro-bumpalo")]
define_field_clone!(bumpalo::collections::Vec<'bump, T>, <T: Clone>);
#[cfg(feature = "puroro-bumpalo")]
define_field_clone!(bumpalo::collections::String<'bump>);
#[cfg(feature = "puroro-bumpalo")]
define_field_clone!(hashbrown::HashMap<K, V, hashbrown::hash_map::DefaultHashBuilder,
        hashbrown::BumpWrapper<'bump>>, <K: Clone, V: Clone>);
define_field_clone!(SliceViewField<'slice>, <'slice>);

impl<'slice, 'bump, T> FieldClone<'bump> for Cow<'slice, T>
where
    T: ?Sized + ToOwned,
{
    fn clone(&self) -> Self {
        <Self as Clone>::clone(self)
    }
    #[cfg(feature = "puroro-bumpalo")]
    fn clone_in_bumpalo(&self, _: &'bump bumpalo::Bump) -> Self {
        <Self as Clone>::clone(self)
    }
}
