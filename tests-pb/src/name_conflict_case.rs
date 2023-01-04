mod _root {
    #[allow(unused)]
    pub(crate) use super::super::_root::*;
}
mod _puroro {
    #[allow(unused)]
    pub(crate) use ::puroro::*;
}
mod _pinternal {
    #[allow(unused)]
    pub(crate) use ::puroro::internal::*;
}
pub mod message;
#[derive(::std::default::Default)]
pub struct Message {
    test: self::_root::name_conflict_case::message::Test::<
        self::_pinternal::NumericalField::<i32, self::_pinternal::tags::Int32>,
    >,
    _bitfield: self::_pinternal::BitArray<1usize>,
}
impl Message {
    pub fn test(
        &self,
    ) -> ::std::option::Option<
        self::_root::name_conflict_case::message::_case::TestCase::<i32>,
    > {
        use self::_pinternal::OneofUnion as _;
        self.test.case_ref(&self._bitfield)
    }
    pub fn clear_test(&mut self) {
        use self::_pinternal::OneofUnion as _;
        self.test.clear(&mut self._bitfield)
    }
    pub fn this_is_oneof(&self) -> i32 {
        self.test.this_is_oneof(&self._bitfield)
    }
    pub fn this_is_oneof_opt(&self) -> ::std::option::Option::<i32> {
        self.test.this_is_oneof_opt(&self._bitfield)
    }
    pub fn this_is_oneof_mut(&mut self) -> &mut i32 {
        self.test.this_is_oneof_mut(&mut self._bitfield)
    }
    pub fn has_this_is_oneof(&self) -> bool {
        self.this_is_oneof_opt().is_some()
    }
    pub fn clear_this_is_oneof(&mut self) {
        #[allow(unused)]
        use ::std::option::Option::Some;
        use self::_pinternal::OneofCase;
        use self::_pinternal::OneofUnion;
        if let Some(
            self::_root::name_conflict_case::message::_case::TestCase::ThisIsOneof(_),
        ) = OneofCase::from_bitslice(&self._bitfield) {
            self.test.clear(&mut self._bitfield)
        }
    }
}
impl self::_puroro::Message for Message {
    fn from_bytes_iter<I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>>(
        iter: I,
    ) -> self::_puroro::Result<Self> {
        let mut msg = <Self as ::std::default::Default>::default();
        msg.merge_from_bytes_iter(iter)?;
        ::std::result::Result::Ok(msg)
    }
    fn merge_from_bytes_iter<I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>>(
        &mut self,
        mut iter: I,
    ) -> self::_puroro::Result<()> {
        use self::_pinternal::ser::FieldData;
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            match number {
                1i32 => {
                    self
                        .test
                        .deser_from_iter(
                            &mut self._bitfield,
                            field_data,
                            self::_root::name_conflict_case::message::_case::TestCase::ThisIsOneof(()),
                        )?
                }
                _ => todo!(),
            }
        }
        ::std::result::Result::Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        self.test.ser_to_write(&self._bitfield, out)?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for Message {
    fn clone(&self) -> Self {
        Self {
            test: self::_pinternal::OneofUnion::clone(&self.test, &self._bitfield),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
impl ::std::ops::Drop for Message {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        self.test.clear(&mut self._bitfield);
    }
}
impl ::std::fmt::Debug for Message {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct(stringify!(Message))
            .field(stringify!(this_is_oneof), &self.this_is_oneof_opt())
            .finish()
    }
}
impl ::std::cmp::PartialEq for Message {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        true && self.test() == rhs.test()
    }
}
pub mod _fields {
    mod _root {
        #[allow(unused)]
        pub use super::super::_root::*;
    }
    mod _puroro {
        #[allow(unused)]
        pub use ::puroro::*;
    }
    mod _pinternal {
        #[allow(unused)]
        pub use ::puroro::internal::*;
    }
    pub struct MessageFields {
        pub foo: i32,
    }
}
pub use self::_fields::*;
