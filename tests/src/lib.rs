#![feature(allocator_api)]
#![feature(generic_associated_types)]
#![feature(min_type_alias_impl_trait)]

mod proto;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
