// A utility for protobuf implementation.
trait MaybeRepeatedField<'a> {
    type Item: 'a;
    type Iter: Iterator<Item = &'a Self::Item>;
    fn iter(&'a self) -> Self::Iter;
}
trait MaybeRepeatedVariantField<'a>: MaybeRepeatedField<'a> {
    fn extend<I: Iterator<Item = Self::Item>>(&mut self, first: Self::Item, rest: I);
}
macro_rules! define_maybe_repeated_field {
    ($ty:ty) => {
        impl<'a> MaybeRepeatedField<'a> for $ty {
            type Item = $ty;
            type Iter = std::option::IntoIter<&'a $ty>;
            fn iter(&'a self) -> Self::Iter {
                // Do not encode if the value is equal to default
                // Not applicable for proto2 because they have [default=xx]!!
                if *self == <$ty as Default>::default() {
                    Some(self)
                } else {
                    None
                }
                .into_iter()
            }
        }
        impl<'a> MaybeRepeatedField<'a> for Vec<$ty> {
            type Item = $ty;
            type Iter = std::slice::Iter<'a, $ty>;
            fn iter(&'a self) -> Self::Iter {
                <[$ty]>::iter(self)
            }
        }
        impl<'a> MaybeRepeatedField<'a> for Option<$ty> {
            type Item = $ty;
            type Iter = std::option::Iter<'a, $ty>;
            fn iter(&'a self) -> Self::Iter {
                Option::<Self::Item>::iter(self)
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
    i32: From<T>,
    T: Clone,
{
    type Item = std::result::Result<T, i32>;
    type Iter = std::option::IntoIter<&'a std::result::Result<T, i32>>;
    fn iter(&'a self) -> Self::Iter {
        let int_value = match self {
            Ok(x) => i32::from(x.clone()),
            Err(i) => *i,
        };
        if int_value != 0 { Some(self) } else { None }.into_iter()
    }
}
impl<'a, T: 'a> MaybeRepeatedField<'a> for Vec<std::result::Result<T, i32>> {
    type Item = std::result::Result<T, i32>;
    type Iter = std::slice::Iter<'a, std::result::Result<T, i32>>;
    fn iter(&'a self) -> Self::Iter {
        <[Self::Item]>::iter(self)
    }
}
impl<'a, T: 'a> MaybeRepeatedField<'a> for Option<std::result::Result<T, i32>> {
    type Item = std::result::Result<T, i32>;
    type Iter = std::option::Iter<'a, std::result::Result<T, i32>>;
    fn iter(&'a self) -> Self::Iter {
        Option::<Self::Item>::iter(self)
    }
}
impl<'a, T: 'a> MaybeRepeatedVariantField<'a> for std::result::Result<T, i32>
where
    i32: From<T>,
    T: Clone,
{
    fn extend<I: Iterator<Item = Self::Item>>(&mut self, first: Self::Item, rest: I) {
        *self = rest.last().unwrap_or(first);
    }
}
impl<'a, T: 'a> MaybeRepeatedVariantField<'a> for Vec<std::result::Result<T, i32>> {
    fn extend<I: Iterator<Item = Self::Item>>(&mut self, first: Self::Item, rest: I) {
        self.push(first);
        <Self as std::iter::Extend<Self::Item>>::extend(self, rest);
    }
}
impl<'a, T: 'a> MaybeRepeatedVariantField<'a> for Option<std::result::Result<T, i32>> {
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
    fn iter(&'a self) -> Self::Iter {
        std::iter::once(self)
    }
}
impl<'a, T: 'a> MaybeRepeatedField<'a> for Vec<T>
where
    T: crate::serializer::Serializable,
{
    type Item = T;
    type Iter = std::slice::Iter<'a, T>;
    fn iter(&'a self) -> Self::Iter {
        <[T]>::iter(self)
    }
}
impl<'a, T: 'a> MaybeRepeatedField<'a> for Option<Box<T>>
where
    T: crate::serializer::Serializable,
{
    type Item = T;
    type Iter = std::option::IntoIter<&'a T>;
    fn iter(&'a self) -> Self::Iter {
        Option::<&'a Self::Item>::into_iter(self.as_deref())
    }
}
