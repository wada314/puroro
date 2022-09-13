// A generated source code by puroro library
// package oneofs2

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub mod _puroro {
    pub use ::puroro::*;
}

#[derive(Default, Clone)]
pub struct Msg {
    _bitfield: self::_puroro::bitvec::BitArray<0>,
}

impl Msg {}

impl self::_puroro::Message for Msg {
    fn from_bytes_iter<I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>>(
        iter: I,
    ) -> self::_puroro::Result<Self> {
        let mut msg: Self = ::std::default::Default::default();
        msg.merge_from_bytes_iter(iter)?;
        Ok(msg)
    }

    fn merge_from_bytes_iter<I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>>(
        &mut self,
        mut iter: I,
    ) -> self::_puroro::Result<()> {
        while let Some((number, field_data)) =
            self::_puroro::internal::ser::FieldData::from_bytes_iter(iter.by_ref())?
        {
            match number {
                _ => todo!(),
            }
        }
        Ok(())
    }
}
pub mod _msg {

    mod _puroro {
        pub use super::super::_puroro::*;
    }
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
    pub(crate) union GroupOne {
        _none: (),
        g1_int32: ::std::mem::ManuallyDrop<
            self::_puroro::internal::oneof_field_type::NumericalField<
                i32,
                self::_puroro::tags::Int32,
            >,
        >,
        g1_string: ::std::mem::ManuallyDrop<self::_puroro::internal::oneof_field_type::StringField>,
    }
    #[repr(u32)]
    pub enum GroupOneCase {
        G1Int32,
        G1String,
    }
    #[repr(u32)]
    pub enum GroupOneCaseRef<'a> {
        G1Int32(i32),
        G1String(&'a str),
    }

    impl GroupOne {
        pub(crate) fn try_g1_int32_opt(
            &self,
            index: u32,
        ) -> _puroro::Result<::std::option::Option<i32>> {
            #[allow(unused)]
            use self::_puroro::internal::oneof_field_type::OneofFieldType;
            #[allow(unused)]
            use ::std::option::Option::{None, Some};
            #[allow(unused)]
            use ::std::result::Result::{Err, Ok};

            todo!()
        }
        pub(crate) fn try_g1_string_opt(
            &self,
            index: u32,
        ) -> _puroro::Result<::std::option::Option<&str>> {
            #[allow(unused)]
            use self::_puroro::internal::oneof_field_type::OneofFieldType;
            #[allow(unused)]
            use ::std::option::Option::{None, Some};
            #[allow(unused)]
            use ::std::result::Result::{Err, Ok};

            todo!()
        }
    }
    pub(crate) union GroupTwo {
        _none: (),
        g2_f32: ::std::mem::ManuallyDrop<
            self::_puroro::internal::oneof_field_type::NumericalField<
                f32,
                self::_puroro::tags::Float,
            >,
        >,
        g2_string: ::std::mem::ManuallyDrop<self::_puroro::internal::oneof_field_type::StringField>,
        g2_submsg: ::std::mem::ManuallyDrop<
            self::_puroro::internal::oneof_field_type::HeapMessageField<
                _puroro_root::oneofs2::Submsg,
            >,
        >,
    }
    #[repr(u32)]
    pub enum GroupTwoCase {
        G2F32,
        G2String,
        G2Submsg,
    }
    #[repr(u32)]
    pub enum GroupTwoCaseRef<'a> {
        G2F32(f32),
        G2String(&'a str),
        G2Submsg(&'a _puroro_root::oneofs2::Submsg),
    }

    impl GroupTwo {
        pub(crate) fn try_g2_f32_opt(
            &self,
            index: u32,
        ) -> _puroro::Result<::std::option::Option<f32>> {
            #[allow(unused)]
            use self::_puroro::internal::oneof_field_type::OneofFieldType;
            #[allow(unused)]
            use ::std::option::Option::{None, Some};
            #[allow(unused)]
            use ::std::result::Result::{Err, Ok};

            todo!()
        }
        pub(crate) fn try_g2_string_opt(
            &self,
            index: u32,
        ) -> _puroro::Result<::std::option::Option<&str>> {
            #[allow(unused)]
            use self::_puroro::internal::oneof_field_type::OneofFieldType;
            #[allow(unused)]
            use ::std::option::Option::{None, Some};
            #[allow(unused)]
            use ::std::result::Result::{Err, Ok};

            todo!()
        }
        pub(crate) fn try_g2_submsg_opt(
            &self,
            index: u32,
        ) -> _puroro::Result<::std::option::Option<&_puroro_root::oneofs2::Submsg>> {
            #[allow(unused)]
            use self::_puroro::internal::oneof_field_type::OneofFieldType;
            #[allow(unused)]
            use ::std::option::Option::{None, Some};
            #[allow(unused)]
            use ::std::result::Result::{Err, Ok};

            todo!()
        }
    }
    pub(crate) union GroupThree {
        _none: (),
        g3_int32: ::std::mem::ManuallyDrop<
            self::_puroro::internal::oneof_field_type::NumericalField<
                i32,
                self::_puroro::tags::Int32,
            >,
        >,
    }
    #[repr(u32)]
    pub enum GroupThreeCase {
        G3Int32,
    }
    #[repr(u32)]
    pub enum GroupThreeCaseRef {
        G3Int32(i32),
    }

    impl GroupThree {
        pub(crate) fn try_g3_int32_opt(
            &self,
            index: u32,
        ) -> _puroro::Result<::std::option::Option<i32>> {
            #[allow(unused)]
            use self::_puroro::internal::oneof_field_type::OneofFieldType;
            #[allow(unused)]
            use ::std::option::Option::{None, Some};
            #[allow(unused)]
            use ::std::result::Result::{Err, Ok};

            todo!()
        }
    }
}

#[derive(Default, Clone)]
pub struct Submsg {
    // Optional, Variant(Int32)
    i32_optional: self::_puroro::internal::field_type::OptionalNumericalField<
        i32,
        self::_puroro::tags::Int32,
        0,
    >,

    _bitfield: self::_puroro::bitvec::BitArray<1>,
}

impl Submsg {
    // Optional, Variant(Int32)
    pub fn i32_optional(&self) -> i32 {
        #[allow(unused)]
        use self::_puroro::internal::field_type::{NonRepeatedFieldType, RepeatedFieldType};
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as NonRepeatedFieldType>::get_field(
            &self.i32_optional,
            &self._bitfield,
            ::std::default::Default::default(),
        )
    }
}

impl self::_puroro::Message for Submsg {
    fn from_bytes_iter<I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>>(
        iter: I,
    ) -> self::_puroro::Result<Self> {
        let mut msg: Self = ::std::default::Default::default();
        msg.merge_from_bytes_iter(iter)?;
        Ok(msg)
    }

    fn merge_from_bytes_iter<I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>>(
        &mut self,
        mut iter: I,
    ) -> self::_puroro::Result<()> {
        while let Some((number, field_data)) =
            self::_puroro::internal::ser::FieldData::from_bytes_iter(iter.by_ref())?
        {
            match number {
                1 => <self::_puroro::internal::field_type::OptionalNumericalField<
                    i32,
                    self::_puroro::tags::Int32,
                    0,
                > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                    &mut self.i32_optional,
                    &mut self._bitfield,
                    field_data,
                )?,
                _ => todo!(),
            }
        }
        Ok(())
    }
}
pub mod _submsg {

    mod _puroro {
        pub use super::super::_puroro::*;
    }
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
}
