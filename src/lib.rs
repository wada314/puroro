#![feature(generic_associated_types)]

use std::iter::IntoIterator;
use std::ops::Deref;
use std::ops::Index;

trait RepeatedFieldView<'a, T: 'a, U>
where
    Self: IntoIterator<Item = &'a T> + Deref<Target = U>,
    U: Index<usize, Output = T> + ?Sized,
{
}
impl<'a, T: 'a> RepeatedFieldView<'a, T, [T]> for &'a [T] {}
// Read-only trait
trait Msg {
    #[allow(non_camel_case_types)]
    type RepeatedFieldView_rival<'a>: RepeatedFieldView<'a, i64, Self::RepeatedField_rival>;
    #[allow(non_camel_case_types)]
    type RepeatedField_rival: Index<usize, Output = i64> + ?Sized;
    fn ival(&self) -> i64;
    fn rival(&self) -> Self::RepeatedFieldView_rival<'_>;
}

struct MsgReadOnlyImpl {
    ival: i64,
    rival: Vec<i64>,
}
impl Msg for MsgReadOnlyImpl {
    type RepeatedFieldView_rival<'a> = &'a [i64];
    type RepeatedField_rival = [i64];

    fn ival(&self) -> i64 {
        self.ival
    }

    fn rival(&self) -> Self::RepeatedFieldView_rival<'_> {
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
