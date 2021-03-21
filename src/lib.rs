#![feature(generic_associated_types)]
//#![feature(trait_alias)]

use std::iter::IntoIterator;
use std::ops::{Deref, Index};

trait RepeatedField<T>
where
    Self: ,
{
}

trait RepeatedFieldRef<'a, T: 'a, R>
where
    Self: Deref<Target = R> + IntoIterator<Item = &'a T>,
    R: ?Sized + Index<usize, Output = T>,
{
}

impl<'a, T: 'a> RepeatedFieldRef<'a, T, [T]> for &'a [T] {}

// Read-only trait
trait Msg {
    #[allow(non_camel_case_types)]
    type RepeatedField_rival: ?Sized + Index<usize, Output = i64>;
    #[allow(non_camel_case_types)]
    type RepeatedFieldRef_rival<'a>: RepeatedFieldRef<'a, i64, Self::RepeatedField_rival>;
    fn ival(&self) -> i64;
    fn rival(&self) -> Self::RepeatedFieldRef_rival<'_>;
}

struct MsgReadOnlyImpl {
    ival: i64,
    rival: Vec<i64>,
}
impl Msg for MsgReadOnlyImpl {
    type RepeatedField_rival = [i64];
    type RepeatedFieldRef_rival<'a> = &'a [i64];

    fn ival(&self) -> i64 {
        self.ival
    }

    fn rival(&self) -> Self::RepeatedFieldRef_rival<'_> {
        &self.rival
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
