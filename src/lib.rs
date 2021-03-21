//#![feature(generic_associated_types)]
//#![feature(trait_alias)]

use std::ops::{Deref, Index};
use std::{iter::IntoIterator, marker::PhantomData};

trait RepeatedField<T>
where
    Self: Index<usize, Output = T>,
{
}

trait RepeatedFieldRef<'a, T: 'a, R: 'a>
where
    Self: Deref<Target = R>,
    Self: IntoIterator<Item = &'a T>,
    R: RepeatedField<T> + ?Sized,
{
}

impl<T> RepeatedField<T> for [T] {}
impl<'a, T: 'a> RepeatedFieldRef<'a, T, [T]> for &'a [T] {}

// Read-only trait
trait Msg<'a> {
    #[allow(non_camel_case_types)]
    type RepeatedField_rival: 'a + RepeatedField<i64> + ?Sized;
    #[allow(non_camel_case_types)]
    type RepeatedFieldRef_rival: RepeatedFieldRef<'a, i64, Self::RepeatedField_rival>;
    fn ival(&'a self) -> i64;
    fn rival(&'a self) -> Self::RepeatedFieldRef_rival;
}

struct MsgReadOnlyImpl<'a> {
    ival: i64,
    rival: Vec<i64>,
    _phantom_data: PhantomData<&'a Self>,
}
impl<'a> Msg<'a> for MsgReadOnlyImpl<'a> {
    type RepeatedField_rival = [i64];
    type RepeatedFieldRef_rival = &'a [i64];

    fn ival(&self) -> i64 {
        self.ival
    }

    fn rival(&'a self) -> Self::RepeatedFieldRef_rival {
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
