use super::{MaybeRepeatedField, MaybeRepeatedField2};
use std::convert::TryFrom;

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

pub trait MaybeRepeatedVariantField2<'a>: MaybeRepeatedField2<'a> {
    /// Extends the field by the given iterator.
    /// If the field is optional / required, then just pick up
    /// the last item of the iterator and set it to themself
    /// (Note that this trait is only for variant-wire types so
    /// we don't need to consider about merging Message types).
    /// If the field is repeated, then just extend the list by
    /// the iterator items.
    fn extend_field<I: Iterator<Item = Self::Item>>(&mut self, first: Self::Item, rest: I);
}
pub trait SingularVariantField {}
impl SingularVariantField for i32 {}
impl SingularVariantField for i64 {}
impl SingularVariantField for u32 {}
impl SingularVariantField for u64 {}
impl SingularVariantField for bool {}
impl<T> SingularVariantField for std::result::Result<T, i32> where T: TryFrom<i32, Error = i32> {}

impl<'a, T: 'a> MaybeRepeatedVariantField2<'a> for T
where
    T: SingularVariantField + Default + Copy + PartialEq,
{
    fn extend_field<I: Iterator<Item = Self::Item>>(&mut self, first: Self::Item, rest: I) {
        *self = rest.last().unwrap_or(first);
    }
}
impl<'a, T: 'a> MaybeRepeatedVariantField2<'a> for Vec<T>
where
    T: SingularVariantField + Default + Copy + PartialEq,
{
    fn extend_field<I: Iterator<Item = Self::Item>>(&mut self, first: Self::Item, rest: I) {
        self.push(first);
        self.extend(rest);
    }
}

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
impl<'a, 'bump, T: 'a> MaybeRepeatedVariantField<'a>
    for ::bumpalo::collections::Vec<'bump, std::result::Result<T, i32>>
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
