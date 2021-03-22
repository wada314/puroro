#![feature(generic_associated_types)]
#![feature(min_type_alias_impl_trait)]

use std::iter::IntoIterator;
use std::ops::{Deref, Index};

trait HasLength {
    fn len(&self) -> usize;
}

impl<T> HasLength for [T] {
    fn len(&self) -> usize {
        self.len()
    }
}
impl<U, T> HasLength for U
where
    U: Deref<Target = T>,
    T: ?Sized + HasLength,
{
    fn len(&self) -> usize {
        T::len(self.deref())
    }
}

// Read-only trait
trait Msg {
    // Types for  int64
    type Field2: ?Sized + Index<usize, Output = i64> + HasLength;
    type Field2Ref<'a>: Deref<Target = Self::Field2>
        + IntoIterator<Item = &'a i64, IntoIter = Self::Field2Iterator<'a>>;
    type Field2Iterator<'a>: Iterator<Item = &'a i64>;
    // Types for  string
    type Field3<'a>: 'a + AsRef<str>;
    type Field4<'a>: ?Sized + Index<usize, Output = Self::Field4Item<'a>> + HasLength;
    type Field4Ref<'a>: Deref<Target = Self::Field4<'a>>
        + IntoIterator<Item = &'a Self::Field4Item<'a>, IntoIter = Self::Field4Iterator<'a>>;
    type Field4Iterator<'a>: Iterator<Item = &'a Self::Field4Item<'a>>;
    type Field4Item<'a>: 'a + AsRef<str>;
    // Types for SubMsg item in this message
    type Field5: Msg_SubMsg;
    type Field5Ref<'a>: Deref<Target = Self::Field5>;

    fn ival(&self) -> i64;
    fn rival(&self) -> Self::Field2Ref<'_>;
    fn sval(&self) -> Self::Field3<'_>;
    fn rsval(&self) -> Self::Field4Ref<'_>;
    fn msg(&self) -> Option<Self::Field5Ref<'_>>;
}

trait Msg_SubMsg {}

struct MsgReadOnlyImpl {
    ival: i64,
    rival: Vec<i64>,
    sval: String,
    rsval: Vec<String>,
    msg: Option<Msg_SubMsgReadOnlyImpl>,
}

struct Msg_SubMsgReadOnlyImpl {}

impl Msg for MsgReadOnlyImpl {
    type Field2 = [i64];
    type Field2Ref<'a> = &'a [i64];
    type Field2Iterator<'a> = std::slice::Iter<'a, i64>;
    type Field3<'a> = &'a str;
    type Field4<'a> = Vec<String>;
    type Field4Ref<'a> = &'a Vec<String>;
    type Field4Iterator<'a> = std::slice::Iter<'a, String>;
    type Field4Item<'a> = String;
    type Field5 = Msg_SubMsgReadOnlyImpl;
    type Field5Ref<'a> = &'a Msg_SubMsgReadOnlyImpl;

    fn ival(&self) -> i64 {
        self.ival
    }
    fn rival(&self) -> Self::Field2Ref<'_> {
        &self.rival
    }
    fn sval(&self) -> Self::Field3<'_> {
        &self.sval
    }
    fn rsval(&self) -> Self::Field4Ref<'_> {
        &self.rsval
    }
    fn msg(&self) -> Option<Self::Field5Ref<'_>> {
        self.msg.as_ref()
    }
}

impl Msg_SubMsg for Msg_SubMsgReadOnlyImpl {}

#[cfg(test)]
mod tests {
    use super::*;

    fn hoge<M: Msg>(m: M) {
        let _i = m.rival()[0];
        for _i in m.rival() {
            let t = _i;
        }
        let _l = m.rival().into_iter();
        let _j = m.rsval().len();
        let _f3: &str = m.sval().as_ref();
    }
}
