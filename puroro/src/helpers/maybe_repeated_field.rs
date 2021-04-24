use super::InternalData;
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
    /// Returns a mutable reference for the:
    /// - Optional / required field: The field item.
    /// - Repeated field: A new item pushed into the tail of the list.
    fn push_and_get_mut<T>(&'a mut self, _internal: &'a T) -> &'a mut Self::Item
    where
        T: InternalData;
}

/// An utility for protobuf implementation code.
/// Used for simplifying the serialization / deserialization code.
/// The lifetime parameter `'a` is used for the generated iterator's lifetime.
/// This is a workaround for the lack of Generic Associated Types (GAT)
/// in the current (1.52.0) Rust.
/// Essentially `'a` should be equal to the lifetime of the `&self`.
pub trait MaybeRepeatedField2<'a> {
    type Item: 'a + ?Sized;
    type Iter: Iterator<Item = &'a Self::Item>;
    /// Returns an iterator over this field items.
    /// If this field is an optional / required field, then the iterator
    /// may or may not iterate over the single item, depend on if
    /// the field value is equal to default or not.
    /// If this field is a repeated field, then the iterator iterates
    /// over the repeated items.
    fn iter_for_ser(&'a self) -> Self::Iter;
    /// Returns a mutable reference for the:
    /// - Optional / required field: The field item.
    /// - Repeated field: A new item pushed into the tail of the list.
    fn push_and_get_mut<U>(&'a mut self, _internal: &'a U) -> &'a mut Self::Item
    where
        U: InternalData;
}

impl<'a, T> MaybeRepeatedField2<'a> for &&'a T
where
    T: Default + PartialEq + Copy,
{
    type Item = T;
    type Iter = std::option::IntoIter<&'a T>;
    fn iter_for_ser(&'a self) -> Self::Iter {
        if ***self == Default::default() {
            None
        } else {
            Some(self)
        }
        .into_iter()
    }
    fn push_and_get_mut<U>(&'a mut self, _internal: &'a U) -> &'a mut Self::Item
    where
        U: InternalData,
    {
        self
    }
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
            fn push_and_get_mut<T>(&'a mut self, _internal: &'a T) -> &'a mut Self::Item
            where
                T: InternalData,
            {
                self
            }
        }
        impl<'a> MaybeRepeatedField<'a> for Vec<$ty> {
            type Item = $ty;
            type Iter = std::slice::Iter<'a, $ty>;
            fn iter_for_ser(&'a self) -> Self::Iter {
                self.iter()
            }
            fn push_and_get_mut<T>(&'a mut self, _internal: &'a T) -> &'a mut Self::Item
            where
                T: InternalData,
            {
                self.push(Default::default());
                self.last_mut().unwrap()
            }
        }
        #[cfg(feature = "puroro-bumpalo")]
        impl<'a, 'bump> MaybeRepeatedField<'a> for ::bumpalo::collections::Vec<'bump, $ty> {
            type Item = $ty;
            type Iter = std::slice::Iter<'a, $ty>;
            fn iter_for_ser(&'a self) -> Self::Iter {
                self.iter()
            }
            fn push_and_get_mut<T>(&'a mut self, _internal: &'a T) -> &'a mut Self::Item
            where
                T: InternalData,
            {
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
            fn push_and_get_mut<T>(&'a mut self, _internal: &'a T) -> &'a mut Self::Item
            where
                T: InternalData,
            {
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
    fn push_and_get_mut<U>(&'a mut self, _internal: &'a U) -> &'a mut Self::Item
    where
        U: InternalData,
    {
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
    fn push_and_get_mut<U>(&'a mut self, _internal: &'a U) -> &'a mut Self::Item
    where
        U: InternalData,
    {
        self.push(T::try_from(0i32));
        self.last_mut().unwrap()
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'a, 'bump, T: 'a> MaybeRepeatedField<'a>
    for ::bumpalo::collections::Vec<'bump, std::result::Result<T, i32>>
where
    T: Clone + TryFrom<i32, Error = i32> + Into<i32>,
{
    type Item = std::result::Result<T, i32>;
    type Iter = std::slice::Iter<'a, std::result::Result<T, i32>>;
    fn iter_for_ser(&'a self) -> Self::Iter {
        self.iter()
    }
    fn push_and_get_mut<U>(&'a mut self, _internal: &'a U) -> &'a mut Self::Item
    where
        U: InternalData,
    {
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
    fn push_and_get_mut<U>(&'a mut self, _internal: &'a U) -> &'a mut Self::Item
    where
        U: InternalData,
    {
        self.get_or_insert(T::try_from(0i32))
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
    fn push_and_get_mut<U>(&'a mut self, _internal: &'a U) -> &'a mut Self::Item
    where
        U: InternalData,
    {
        self.as_mut()
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'a, 'bump, T: 'a> MaybeRepeatedField<'a> for ::bumpalo::boxed::Box<'bump, T>
where
    T: crate::serializer::Serializable,
{
    type Item = T;
    type Iter = std::iter::Once<&'a T>;
    fn iter_for_ser(&'a self) -> Self::Iter {
        std::iter::once(self)
    }
    fn push_and_get_mut<U>(&'a mut self, _internal: &'a U) -> &'a mut Self::Item
    where
        U: InternalData,
    {
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
    fn push_and_get_mut<U>(&'a mut self, _internal: &'a U) -> &'a mut Self::Item
    where
        U: InternalData,
    {
        self.push(Default::default());
        self.last_mut().unwrap()
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'a, 'bump, T: 'a> MaybeRepeatedField<'a> for ::bumpalo::collections::Vec<'bump, T>
where
    T: Default + crate::serializer::Serializable,
{
    type Item = T;
    type Iter = std::slice::Iter<'a, T>;
    fn iter_for_ser(&'a self) -> Self::Iter {
        self.iter()
    }
    fn push_and_get_mut<U>(&'a mut self, _internal: &'a U) -> &'a mut Self::Item
    where
        U: InternalData,
    {
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
    fn push_and_get_mut<U>(&'a mut self, _internal: &'a U) -> &'a mut Self::Item
    where
        U: InternalData,
    {
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
    fn push_and_get_mut<U>(&'a mut self, internal: &'a U) -> &'a mut Self::Item
    where
        U: InternalData,
    {
        self.get_or_insert_with(|| {
            ::bumpalo::boxed::Box::new_in(Default::default(), internal.bumpalo())
        })
        .as_mut()
    }
}
