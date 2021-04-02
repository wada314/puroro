use std::iter::FromIterator;
use std::marker::PhantomData;

pub mod error;
pub mod tags;

pub use error::PuroroError;
use tags::FieldTypeTag;
pub type Result<T> = std::result::Result<T, PuroroError>;

pub trait Message {
    // Returns by value.
    fn get_singular_field_or_else<T: FieldTypeTag, F: FnOnce() -> T::SingularRustType>(
        &self,
        field_number: usize,
        f_default: F,
    ) -> Result<T::SingularRustType>;
    fn get_repeated_field<T: FieldTypeTag>(
        &self,
        field_number: usize,
    ) -> Result<T::RepeatedRustType>;

    fn get_field_as_i32(&self, field_number: usize) -> Result<i32>;
    fn get_field_as_i64(&self, field_number: usize) -> Result<i64>;
    fn get_field_as_si32(&self, field_number: usize) -> Result<i32>;
    fn get_field_as_si64(&self, field_number: usize) -> Result<i64>;
    fn get_field_as_u32(&self, field_number: usize) -> Result<u32>;
    fn get_field_as_u64(&self, field_number: usize) -> Result<u64>;
    fn get_field_as_bool(&self, field_number: usize) -> Result<bool>;
    fn handle_field_as_repeated_i32<H: RepeatedFieldHandler<Item = i32>>(
        &self,
        field_number: usize,
        handler: H,
    ) -> Result<H::Output>;
    fn handle_field_as_repeated_i64<H: RepeatedFieldHandler<Item = i64>>(
        &self,
        field_number: usize,
        handler: H,
    ) -> Result<H::Output>;
    fn handle_field_as_repeated_u32<H: RepeatedFieldHandler<Item = u32>>(
        &self,
        field_number: usize,
        handler: H,
    ) -> Result<H::Output>;
    fn handle_field_as_repeated_u64<H: RepeatedFieldHandler<Item = u64>>(
        &self,
        field_number: usize,
        handler: H,
    ) -> Result<H::Output>;
    fn handle_field_as_repeated_si32<H: RepeatedFieldHandler<Item = i32>>(
        &self,
        field_number: usize,
        handler: H,
    ) -> Result<H::Output>;
    fn handle_field_as_repeated_si64<H: RepeatedFieldHandler<Item = i64>>(
        &self,
        field_number: usize,
        handler: H,
    ) -> Result<H::Output>;
    fn handle_field_as_repeated_bool<H: RepeatedFieldHandler<Item = bool>>(
        &self,
        field_number: usize,
        handler: H,
    ) -> Result<H::Output>;

    fn handle_field_as_str<H: RepeatedFieldHandler<Item = char>>(
        &self,
        field_number: usize,
        handler: H,
    ) -> Result<H::Output>;
    fn handle_field_as_repeated_str<
        H: RepeatedFieldHandler<Item = char>,
        G: RepeatedFieldHandler<Item = H::Output>,
    >(
        &self,
        field_number: usize,
        string_handler: H,
        strings_handler: G,
    ) -> Result<G::Output>;

    fn collect_field_as_str<S: FromIterator<char>>(&self, field_number: usize) -> Result<S>;
    fn collect_field_as_repeated_str<S: FromIterator<char>, T: FromIterator<S>>(
        &self,
        field_number: usize,
    ) -> Result<T>;
    fn get_field_as_message<T>(&self, field_number: usize) -> Result<Option<T>>
    where
        T: Mergeable + Deserializable;
    fn collect_field_as_repeated_message<T: Deserializable, U: FromIterator<T>>(
        &self,
        field_number: usize,
    ) -> Result<U>;
}

pub trait Deserializable: Sized {
    fn from_bytes<I: Iterator<Item = std::io::Result<u8>>>(iter: I) -> Result<Self>;
}
pub trait Mergeable: Sized {
    fn merge(&self, latter: &Self) -> Result<Self>;
}

pub trait RepeatedFieldHandler {
    type Item;
    type Output;
    fn handle<I: Iterator<Item = Result<Self::Item>>>(self, iter: I) -> Result<Self::Output>;
}

pub struct RepeatedFieldCollector<T, U>(PhantomData<(T, U)>)
where
    U: FromIterator<T>;

impl<T, U> RepeatedFieldCollector<T, U>
where
    U: FromIterator<T>,
{
    pub fn new() -> Self {
        Self(PhantomData)
    }
}
impl<T, U> RepeatedFieldHandler for RepeatedFieldCollector<T, U>
where
    U: FromIterator<T>,
{
    type Item = T;
    type Output = U;

    fn handle<I: Iterator<Item = Result<Self::Item>>>(self, iter: I) -> Result<Self::Output> {
        iter.collect::<Result<U>>()
    }
}
