pub mod unknown_message;
pub use unknown_message::UnknownMessage;
pub enum PuroroError {}
pub type Result<T> = std::result::Result<T, PuroroError>;

pub trait Message: Sized {
    fn from_read<R: std::io::Read>(read: R) -> Result<Self>;

    fn get_field_as_i32(&self, field_number: usize) -> Result<i32>;
    fn get_field_as_repeated_i32(
        &self,
        field_number: usize,
    ) -> Result<Box<dyn Iterator<Item = i32>>>;
    fn get_field_as_str(&self, field_number: usize) -> Result<&dyn AsRef<str>>;
    fn get_field_as_message<T: Message>(&self, field_number: usize) -> Result<T>;
}
pub struct Merged<T: Message, U: Message> {
    prior: T,
    later: U,
}
pub fn merge<T: Message, U: Message>(prior: T, later: U) -> Merged<T, U> {
    Merged { prior, later }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
