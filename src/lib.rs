#![feature(generic_associated_types)]
#![feature(min_type_alias_impl_trait)]

use std::iter::IntoIterator;
use std::ops::{Deref, Index};

// Read-only trait
trait Msg {
    type RepeatedField2: ?Sized + Index<usize, Output = i64>;
    type RepeatedField2Ref<'a>: Deref<Target = Self::RepeatedField2>
        + IntoIterator<Item = &'a i64, IntoIter = Self::RepeatedField2Iterator<'a>>;
    type RepeatedField2Iterator<'a>: Iterator<Item = &'a i64>;
    type RepeatedField4<'a>: ?Sized + Index<usize, Output = Self::RepeatedField4Item<'a>>;
    type RepeatedField4Ref<'a>: Deref<Target = Self::RepeatedField4<'a>>
        + IntoIterator<
            Item = &'a Self::RepeatedField4Item<'a>,
            IntoIter = Self::RepeatedField4Iterator<'a>,
        >;
    type RepeatedField4Iterator<'a>: Iterator<Item = &'a Self::RepeatedField4Item<'a>>;
    type RepeatedField4Item<'a>: 'a + AsRef<str>;

    fn ival(&self) -> i64;
    fn rival(&self) -> Self::RepeatedField2Ref<'_>;
    fn sval(&self) -> &str;
    fn rsval(&self) -> Self::RepeatedField4Ref<'_>;
}

struct MsgReadOnlyImpl {
    ival: i64,
    rival: Vec<i64>,
    sval: String,
    rsval: Vec<String>,
}

impl Msg for MsgReadOnlyImpl {
    type RepeatedField2 = [i64];
    type RepeatedField2Ref<'a> = &'a [i64];
    type RepeatedField2Iterator<'a> = std::slice::Iter<'a, i64>;
    type RepeatedField4<'a> = Vec<String>;
    type RepeatedField4Ref<'a> = &'a Vec<String>;
    type RepeatedField4Iterator<'a> = std::slice::Iter<'a, String>;
    type RepeatedField4Item<'a> = String;

    fn ival(&self) -> i64 {
        self.ival
    }
    fn rival(&self) -> Self::RepeatedField2Ref<'_> {
        &self.rival
    }
    fn sval(&self) -> &str {
        &self.sval
    }
    fn rsval(&self) -> Self::RepeatedField4Ref<'_> {
        &self.rsval
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hoge<M: Msg>(m: M) {
        let _i = m.rival()[0];
        for _i in m.rival() {
            let t = _i;
        }
        let l = m.rival().into_iter();
    }
}
