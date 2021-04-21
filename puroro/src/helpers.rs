use crate::types::FieldData;
use std::collections::HashMap;
use std::convert::TryFrom;

/// An utility for protobuf implementation code.
/// Used for simplifying the serialization / deserialization code.
/// The lifetime parameter `'a` is used for the generated iterator's lifetime.
/// This is a workaround for the lack of Generic Associated Types (GAT)
/// in the current (1.52.0) Rust.
/// Essentially `'a` should be equal to the lifetime of the `&self`.
pub trait MaybeRepeatedField<'a> {
    type Item: 'a;
    type Iter: Iterator<Item = &'a Self::Item>;
    /// Returns an iterator over this field items.
    /// If this field is an optional / required field, then the iterator
    /// may or may not iterate over the single item, depend on if
    /// the field value is equal to default or not.
    /// If this field is a repeated field, then the iterator iterates
    /// over the repeated items.
    fn iter_for_ser(&'a self) -> Self::Iter;
    /// Returns an mutable reference for the:
    /// - Optional / required field: The field item.
    /// - Repeated field: A new item pushed into the tail of the list.
    fn push_and_get_mut(&'a mut self) -> &'a mut Self::Item;
    fn push_and_get_mut2<T>(&'a mut self, internal: &'a T) -> &'a mut Self::Item
    where
        T: InternalData,
    {
        self.push_and_get_mut()
    }
}
pub trait MaybeRepeatedVariantField<'a>: MaybeRepeatedField<'a> {
    /// Extends the field by the given iterator.
    /// If the field is optional / required, then just pick up
    /// the last item of the iterator and set it to themself
    /// (Note that this trait is only for variant-wire types so
    /// we don't need to consider about merging Message types).
    /// If the field is repeated, then just extend the list by
    /// the iterator items.
    fn extend<I: Iterator<Item = Self::Item>>(&mut self, first: Self::Item, rest: I);
}

macro_rules! define_maybe_repeated_field {
    ($ty:ty) => {
        impl<'a> MaybeRepeatedField<'a> for $ty {
            type Item = $ty;
            type Iter = std::option::IntoIter<&'a $ty>;
            fn iter_for_ser(&'a self) -> Self::Iter {
                // Do not encode if the value is equal to default
                // Not applicable for proto2 because they have [default=xx]!!
                if *self != <$ty as Default>::default() {
                    Some(self)
                } else {
                    None
                }
                .into_iter()
            }
            fn push_and_get_mut(&'a mut self) -> &'a mut Self::Item {
                self
            }
        }
        impl<'a> MaybeRepeatedField<'a> for Vec<$ty> {
            type Item = $ty;
            type Iter = std::slice::Iter<'a, $ty>;
            fn iter_for_ser(&'a self) -> Self::Iter {
                self.iter()
            }
            fn push_and_get_mut(&'a mut self) -> &'a mut Self::Item {
                self.push(Default::default());
                self.last_mut().unwrap()
            }
        }
        #[cfg(feature = "puroro-bumpalo")]
        impl<'a, 'b> MaybeRepeatedField<'a> for ::bumpalo::collections::Vec<'b, $ty> {
            type Item = $ty;
            type Iter = std::slice::Iter<'a, $ty>;
            fn iter_for_ser(&'a self) -> Self::Iter {
                self.iter()
            }
            fn push_and_get_mut(&'a mut self) -> &'a mut Self::Item {
                self.push(Default::default());
                self.last_mut().unwrap()
            }
        }
        impl<'a> MaybeRepeatedField<'a> for Option<$ty> {
            type Item = $ty;
            type Iter = std::option::Iter<'a, $ty>;
            fn iter_for_ser(&'a self) -> Self::Iter {
                self.iter()
            }
            fn push_and_get_mut(&'a mut self) -> &'a mut Self::Item {
                self.get_or_insert_with(Default::default)
            }
        }
    };
}
define_maybe_repeated_field!(i32);
define_maybe_repeated_field!(i64);
define_maybe_repeated_field!(u32);
define_maybe_repeated_field!(u64);
define_maybe_repeated_field!(f32);
define_maybe_repeated_field!(f64);
define_maybe_repeated_field!(bool);
define_maybe_repeated_field!(String);
define_maybe_repeated_field!(Vec<u8>);

macro_rules! define_maybe_repeated_variant_field {
    ($ty:ty) => {
        impl<'a> MaybeRepeatedVariantField<'a> for $ty {
            fn extend<I: Iterator<Item = Self::Item>>(&mut self, first: Self::Item, rest: I) {
                *self = rest.last().unwrap_or(first);
            }
        }
        impl<'a> MaybeRepeatedVariantField<'a> for Vec<$ty> {
            fn extend<I: Iterator<Item = Self::Item>>(&mut self, first: Self::Item, rest: I) {
                self.push(first);
                <Self as std::iter::Extend<Self::Item>>::extend(self, rest);
            }
        }
        #[cfg(feature = "puroro-bumpalo")]
        impl<'a, 'b> MaybeRepeatedVariantField<'a> for ::bumpalo::collections::Vec<'b, $ty> {
            fn extend<I: Iterator<Item = Self::Item>>(&mut self, first: Self::Item, rest: I) {
                self.push(first);
                <Self as std::iter::Extend<Self::Item>>::extend(self, rest);
            }
        }
        impl<'a> MaybeRepeatedVariantField<'a> for Option<$ty> {
            fn extend<I: Iterator<Item = Self::Item>>(&mut self, first: Self::Item, rest: I) {
                *self = Some(rest.last().unwrap_or(first));
            }
        }
    };
}
define_maybe_repeated_variant_field!(i32);
define_maybe_repeated_variant_field!(i64);
define_maybe_repeated_variant_field!(u32);
define_maybe_repeated_variant_field!(u64);
define_maybe_repeated_variant_field!(bool);

// Manual implementation for enum
impl<'a, T: 'a> MaybeRepeatedField<'a> for std::result::Result<T, i32>
where
    T: Clone + TryFrom<i32, Error = i32> + Into<i32>,
{
    type Item = std::result::Result<T, i32>;
    type Iter = std::option::IntoIter<&'a std::result::Result<T, i32>>;
    fn iter_for_ser(&'a self) -> Self::Iter {
        let int_value = match self {
            Ok(x) => T::into(x.clone()),
            Err(i) => *i,
        };
        if int_value != 0 { Some(self) } else { None }.into_iter()
    }
    fn push_and_get_mut(&'a mut self) -> &'a mut Self::Item {
        self
    }
}
impl<'a, T: 'a> MaybeRepeatedField<'a> for Vec<std::result::Result<T, i32>>
where
    T: Clone + TryFrom<i32, Error = i32> + Into<i32>,
{
    type Item = std::result::Result<T, i32>;
    type Iter = std::slice::Iter<'a, std::result::Result<T, i32>>;
    fn iter_for_ser(&'a self) -> Self::Iter {
        self.iter()
    }
    fn push_and_get_mut(&'a mut self) -> &'a mut Self::Item {
        self.push(T::try_from(0i32));
        self.last_mut().unwrap()
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'a, 'b, T: 'a> MaybeRepeatedField<'a>
    for ::bumpalo::collections::Vec<'b, std::result::Result<T, i32>>
where
    T: Clone + TryFrom<i32, Error = i32> + Into<i32>,
{
    type Item = std::result::Result<T, i32>;
    type Iter = std::slice::Iter<'a, std::result::Result<T, i32>>;
    fn iter_for_ser(&'a self) -> Self::Iter {
        self.iter()
    }
    fn push_and_get_mut(&'a mut self) -> &'a mut Self::Item {
        self.push(T::try_from(0i32));
        self.last_mut().unwrap()
    }
}
impl<'a, T: 'a> MaybeRepeatedField<'a> for Option<std::result::Result<T, i32>>
where
    T: Clone + TryFrom<i32, Error = i32> + Into<i32>,
{
    type Item = std::result::Result<T, i32>;
    type Iter = std::option::Iter<'a, std::result::Result<T, i32>>;
    fn iter_for_ser(&'a self) -> Self::Iter {
        self.iter()
    }
    fn push_and_get_mut(&'a mut self) -> &'a mut Self::Item {
        self.get_or_insert(T::try_from(0i32))
    }
}
impl<'a, T: 'a> MaybeRepeatedVariantField<'a> for std::result::Result<T, i32>
where
    T: Clone + TryFrom<i32, Error = i32> + Into<i32>,
{
    fn extend<I: Iterator<Item = Self::Item>>(&mut self, first: Self::Item, rest: I) {
        *self = rest.last().unwrap_or(first);
    }
}
impl<'a, T: 'a> MaybeRepeatedVariantField<'a> for Vec<std::result::Result<T, i32>>
where
    T: Clone + TryFrom<i32, Error = i32> + Into<i32>,
{
    fn extend<I: Iterator<Item = Self::Item>>(&mut self, first: Self::Item, rest: I) {
        self.push(first);
        <Self as std::iter::Extend<Self::Item>>::extend(self, rest);
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'a, 'b, T: 'a> MaybeRepeatedVariantField<'a>
    for ::bumpalo::collections::Vec<'b, std::result::Result<T, i32>>
where
    T: Clone + TryFrom<i32, Error = i32> + Into<i32>,
{
    fn extend<I: Iterator<Item = Self::Item>>(&mut self, first: Self::Item, rest: I) {
        self.push(first);
        <Self as std::iter::Extend<Self::Item>>::extend(self, rest);
    }
}
impl<'a, T: 'a> MaybeRepeatedVariantField<'a> for Option<std::result::Result<T, i32>>
where
    T: Clone + TryFrom<i32, Error = i32> + Into<i32>,
{
    fn extend<I: Iterator<Item = Self::Item>>(&mut self, first: Self::Item, rest: I) {
        *self = Some(rest.last().unwrap_or(first));
    }
}

// Manual implementation for message
impl<'a, T: 'a> MaybeRepeatedField<'a> for Box<T>
where
    T: crate::serializer::Serializable,
{
    type Item = T;
    type Iter = std::iter::Once<&'a T>;
    fn iter_for_ser(&'a self) -> Self::Iter {
        std::iter::once(self)
    }
    fn push_and_get_mut(&'a mut self) -> &'a mut Self::Item {
        self.as_mut()
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'a, 'b, T: 'a> MaybeRepeatedField<'a> for ::bumpalo::boxed::Box<'b, T>
where
    T: crate::serializer::Serializable,
{
    type Item = T;
    type Iter = std::iter::Once<&'a T>;
    fn iter_for_ser(&'a self) -> Self::Iter {
        std::iter::once(self)
    }
    fn push_and_get_mut(&'a mut self) -> &'a mut Self::Item {
        self.as_mut()
    }
}
impl<'a, T: 'a> MaybeRepeatedField<'a> for Vec<T>
where
    T: Default + crate::serializer::Serializable,
{
    type Item = T;
    type Iter = std::slice::Iter<'a, T>;
    fn iter_for_ser(&'a self) -> Self::Iter {
        self.iter()
    }
    fn push_and_get_mut(&'a mut self) -> &'a mut Self::Item {
        self.push(Default::default());
        self.last_mut().unwrap()
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'a, 'b, T: 'a> MaybeRepeatedField<'a> for ::bumpalo::collections::Vec<'b, T>
where
    T: Default + crate::serializer::Serializable,
{
    type Item = T;
    type Iter = std::slice::Iter<'a, T>;
    fn iter_for_ser(&'a self) -> Self::Iter {
        self.iter()
    }
    fn push_and_get_mut(&'a mut self) -> &'a mut Self::Item {
        self.push(Default::default());
        self.last_mut().unwrap()
    }
}
impl<'a, T: 'a> MaybeRepeatedField<'a> for Option<Box<T>>
where
    T: Default + crate::serializer::Serializable,
{
    type Item = T;
    type Iter = std::option::IntoIter<&'a T>;
    fn iter_for_ser(&'a self) -> Self::Iter {
        Option::<&'a Self::Item>::into_iter(self.as_deref())
    }
    fn push_and_get_mut(&'a mut self) -> &'a mut Self::Item {
        self.get_or_insert_with(|| Box::new(Default::default()))
            .as_mut()
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'a, T: 'a> MaybeRepeatedField<'a> for Option<::bumpalo::boxed::Box<'a, T>>
where
    T: Default + crate::serializer::Serializable,
{
    type Item = T;
    type Iter = std::option::IntoIter<&'a T>;
    fn iter_for_ser(&'a self) -> Self::Iter {
        Option::<&'a Self::Item>::into_iter(self.as_deref())
    }
    fn push_and_get_mut(&'a mut self) -> &'a mut Self::Item {
        unimplemented!(
            "Please call the push_and_get_mut2 method for this type \
                because we need an allocator info."
        )
    }
    fn push_and_get_mut2<U>(&'a mut self, internal: &'a U) -> &'a mut Self::Item
    where
        U: InternalData,
    {
        self.get_or_insert_with(|| {
            ::bumpalo::boxed::Box::new_in(Default::default(), internal.bumpalo())
        })
        .as_mut()
    }
}

pub trait InternalData {
    #[cfg(feature = "puroro-bumpalo")]
    fn bumpalo(&self) -> &bumpalo::Bump {
        panic!("The Bumpalo data field should only be owned by a Bumpalo struct!")
    }
}

#[derive(Debug, Clone)]
pub struct InternalDataForNormalStruct {
    unknown_fields: Option<HashMap<usize, FieldData<Vec<u8>>>>,
}
impl InternalDataForNormalStruct {
    pub fn new() -> Self {
        Self {
            unknown_fields: None,
        }
    }
}
impl InternalData for InternalDataForNormalStruct {}

#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct InternalDataForBumpaloStruct<'b> {
    // No hashmap implementation in bumpalo...
    unknown_fields: Option<
        ::bumpalo::collections::Vec<'b, (usize, FieldData<::bumpalo::collections::Vec<'b, u8>>)>,
    >,
    bump: &'b ::bumpalo::Bump,
}

#[cfg(feature = "puroro-bumpalo")]
impl<'b> InternalDataForBumpaloStruct<'b> {
    pub fn new(bump: &'b ::bumpalo::Bump) -> Self {
        Self {
            unknown_fields: None,
            bump,
        }
    }
}
impl<'b> InternalData for InternalDataForBumpaloStruct<'b> {
    /// Note that the returned bumpalo lifetime is not `'b' but `'_`.
    /// This is because I don't want to introduce the lifetime parameter
    /// `'b` into the trait's definition. The lifetime `'_` might be shorter
    /// than `'b`, but I believe it's not a problem.
    fn bumpalo(&self) -> &bumpalo::Bump {
        self.bump
    }
}
