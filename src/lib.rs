#![feature(generic_associated_types)]

use std::iter::IntoIterator;
use std::ops::{Deref, Index};

// Read-only trait
trait Msg {
    type RepeatedField<T>: ?Sized + Index<usize, Output = T>;
    type RepeatedFieldRef<'a, T: 'a>: Deref<Target = Self::RepeatedField<T>>
        + IntoIterator<Item = &'a T>;

    fn ival(&self) -> i64;
    fn rival(&self) -> Self::RepeatedFieldRef<'_, i64>;
}

struct MsgReadOnlyImpl {
    ival: i64,
    rival: Vec<i64>,
}

impl Msg for MsgReadOnlyImpl {
    type RepeatedField<T> = [T];
    type RepeatedFieldRef<'a, T: 'a> = &'a [T];

    fn ival(&self) -> i64 {
        self.ival
    }

    fn rival(&self) -> Self::RepeatedFieldRef<'_, i64> {
        &self.rival
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hoge<M: Msg>(m: M) {}
}
