#![feature(generic_associated_types)]
#![feature(min_type_alias_impl_trait)]

use std::iter::IntoIterator;
use std::ops::{Deref, Index};

// Read-only trait
trait Msg {
    type RepeatedField2: ?Sized + Index<usize, Output = i64>;
    type RepeatedFieldRef2<'a>: Deref<Target = Self::RepeatedField2> + IntoIterator<Item = &'a i64>;
    type RepeatedField4<'a>: ?Sized + Index<usize, Output = &'a str>;
    type RepeatedFieldRef4<'a>: Deref<Target = Self::RepeatedField4<'a>>
        + IntoIterator<Item = &'a str, IntoIter = Self::RepeatedFieldIterator4<'a>>;
    type RepeatedFieldIterator4<'a>: Iterator<Item = &'a str>;

    fn ival(&self) -> i64;
    fn rival(&self) -> Self::RepeatedFieldRef2<'_>;
    fn sval(&self) -> &str;
    fn rsval(&self) -> Self::RepeatedFieldRef4<'_>;
}

struct MsgReadOnlyImpl {
    ival: i64,
    rival: Vec<i64>,
    sval: String,
    rsval: Vec<String>,
}

impl Msg for MsgReadOnlyImpl {
    type RepeatedField2 = [i64];
    type RepeatedFieldRef2<'a> = &'a [i64];
    type RepeatedField4<'a> = RepeatedStringView<'a>;
    type RepeatedFieldRef4<'a> = RepeatedStringViewRef<'a>;
    type RepeatedFieldIterator4<'a> =
        std::iter::Map<std::slice::Iter<'a, String>, fn(&String) -> &str>;

    fn ival(&self) -> i64 {
        self.ival
    }
    fn rival(&self) -> Self::RepeatedFieldRef2<'_> {
        &self.rival
    }
    fn sval(&self) -> &str {
        &self.sval
    }
    fn rsval(&self) -> Self::RepeatedFieldRef4<'_> {
        (&self.rsval).into()
    }
}

struct RepeatedStringView<'a> {
    vec: &'a Vec<String>,
}
struct RepeatedStringViewRef<'a> {
    view: RepeatedStringView<'a>,
}
impl<'a> From<&'a Vec<String>> for RepeatedStringViewRef<'a> {
    fn from(from: &'a Vec<String>) -> Self {
        RepeatedStringViewRef {
            view: RepeatedStringView { vec: from },
        }
    }
}
impl<'a> Deref for RepeatedStringViewRef<'a> {
    type Target = RepeatedStringView<'a>;
    fn deref(&self) -> &Self::Target {
        &self.view
    }
}
impl<'a> Index<usize> for RepeatedStringView<'a> {
    type Output = &'a str;
    fn index(&self, index: usize) -> &Self::Output {
        self.vec[index].as_str()
    }
}
impl<'a> IntoIterator for RepeatedStringViewRef<'a> {
    type Item = &'a str;

    type IntoIter = std::iter::Map<std::slice::Iter<'a, String>, fn(&String) -> &str>;

    fn into_iter(self) -> Self::IntoIter {
        fn as_ref(x: &String) -> &str {
            x.as_ref()
        }
        (self.view.vec).into_iter().map(as_ref)
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
    }
}
