use std::iter::FromIterator;

pub mod error;
pub use error::PuroroError;
pub type Result<T> = std::result::Result<T, PuroroError>;

pub trait Message: Sized {
    fn from_bytes<I: Iterator<Item = std::io::Result<u8>>>(iter: I) -> Result<Self>;
    type MergedType: Message;
    fn merge(&self, latter: &Self) -> Result<Self::MergedType>;

    fn get_field_as_i32(&self, field_number: usize) -> Result<i32>;
    fn get_field_as_i64(&self, field_number: usize) -> Result<i64>;
    fn get_field_as_si32(&self, field_number: usize) -> Result<i32>;
    fn get_field_as_si64(&self, field_number: usize) -> Result<i64>;
    fn get_field_as_u32(&self, field_number: usize) -> Result<u32>;
    fn get_field_as_u64(&self, field_number: usize) -> Result<u64>;
    fn get_field_as_bool(&self, field_number: usize) -> Result<bool>;
    fn collect_field_as_repeated_i32<T: FromIterator<i32>>(&self, field_number: usize)
        -> Result<T>;
    fn collect_field_as_repeated_i64<T: FromIterator<i64>>(&self, field_number: usize)
        -> Result<T>;
    fn collect_field_as_repeated_si32<T: FromIterator<i32>>(
        &self,
        field_number: usize,
    ) -> Result<T>;
    fn collect_field_as_repeated_si64<T: FromIterator<i64>>(
        &self,
        field_number: usize,
    ) -> Result<T>;
    fn collect_field_as_repeated_u32<T: FromIterator<u32>>(&self, field_number: usize)
        -> Result<T>;
    fn collect_field_as_repeated_u64<T: FromIterator<u64>>(&self, field_number: usize)
        -> Result<T>;
    fn collect_field_as_repeated_bool<T: FromIterator<bool>>(
        &self,
        field_number: usize,
    ) -> Result<T>;

    fn collect_field_as_str<S: FromIterator<char>>(&self, field_number: usize) -> Result<S>;
    fn collect_field_as_repeated_str<S: FromIterator<char>, T: FromIterator<S>>(
        &self,
        field_number: usize,
    ) -> Result<T>;
    fn get_field_as_message<T>(&self, field_number: usize) -> Result<Option<T>>
    where
        T: Message<MergedType = T>;
    fn collect_field_as_repeated_message<T: Message, U: FromIterator<T>>(
        &self,
        field_number: usize,
    ) -> Result<U>;
}

pub struct Merged<T: Message, U: Message> {
    prior: T,
    later: U,
}
pub fn merge<T: Message, U: Message>(prior: T, later: U) -> Merged<T, U> {
    Merged { prior, later }
}
//TODO: impl Message for Merged

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
