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
    _bitfield: self::_puroro::bitvec::BitArray<1>,
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
        pub(crate) fn g1_int32_opt<B: self::_puroro::bitvec::BitSlice>(
            &self,
            bits: &B,
        ) -> ::std::option::Option<i32> {
            #[allow(unused)]
            use self::_puroro::internal::oneof_field_type::OneofFieldType;
            #[allow(unused)]
            use self::_puroro::internal::oneof_type::OneofCase;
            #[allow(unused)]
            use ::std::option::Option::{None, Some};

            let case_opt = self::GroupOneCase::from_bitslice(bits);
            if let Some(GroupOneCase::G1Int32) = case_opt {
                let item = unsafe { &self.g1_int32 };
                Some(item.get_field())
            } else {
                None
            }
        }
        pub(crate) fn g1_string_opt<B: self::_puroro::bitvec::BitSlice>(
            &self,
            bits: &B,
        ) -> ::std::option::Option<&str> {
            #[allow(unused)]
            use self::_puroro::internal::oneof_field_type::OneofFieldType;
            #[allow(unused)]
            use self::_puroro::internal::oneof_type::OneofCase;
            #[allow(unused)]
            use ::std::option::Option::{None, Some};

            let case_opt = self::GroupOneCase::from_bitslice(bits);
            if let Some(GroupOneCase::G1String) = case_opt {
                let item = unsafe { &self.g1_string };
                Some(item.get_field())
            } else {
                None
            }
        }

        pub(crate) fn _clear<B: self::_puroro::bitvec::BitSlice>(&mut self, bits: &mut B) {
            use self::_puroro::bitvec::BitSlice;
            // TODO: take manuallydrop value
            bits.set_range::<0, 2>(2);
            todo!()
        }
    }

    impl self::_puroro::internal::oneof_type::OneofCase<0, 2> for GroupOneCase {
        fn from_u32(x: u32) -> Option<Self> {
            #[allow(unused)]
            use ::std::option::Option::{None, Some};
            match x {
                0 => Some(self::GroupOneCase::G1Int32),
                1 => Some(self::GroupOneCase::G1String),
                _ => None,
            }
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
        pub(crate) fn g2_f32_opt<B: self::_puroro::bitvec::BitSlice>(
            &self,
            bits: &B,
        ) -> ::std::option::Option<f32> {
            #[allow(unused)]
            use self::_puroro::internal::oneof_field_type::OneofFieldType;
            #[allow(unused)]
            use self::_puroro::internal::oneof_type::OneofCase;
            #[allow(unused)]
            use ::std::option::Option::{None, Some};

            let case_opt = self::GroupTwoCase::from_bitslice(bits);
            if let Some(GroupTwoCase::G2F32) = case_opt {
                let item = unsafe { &self.g2_f32 };
                Some(item.get_field())
            } else {
                None
            }
        }
        pub(crate) fn g2_string_opt<B: self::_puroro::bitvec::BitSlice>(
            &self,
            bits: &B,
        ) -> ::std::option::Option<&str> {
            #[allow(unused)]
            use self::_puroro::internal::oneof_field_type::OneofFieldType;
            #[allow(unused)]
            use self::_puroro::internal::oneof_type::OneofCase;
            #[allow(unused)]
            use ::std::option::Option::{None, Some};

            let case_opt = self::GroupTwoCase::from_bitslice(bits);
            if let Some(GroupTwoCase::G2String) = case_opt {
                let item = unsafe { &self.g2_string };
                Some(item.get_field())
            } else {
                None
            }
        }
        pub(crate) fn g2_submsg_opt<B: self::_puroro::bitvec::BitSlice>(
            &self,
            bits: &B,
        ) -> ::std::option::Option<&_puroro_root::oneofs2::Submsg> {
            #[allow(unused)]
            use self::_puroro::internal::oneof_field_type::OneofFieldType;
            #[allow(unused)]
            use self::_puroro::internal::oneof_type::OneofCase;
            #[allow(unused)]
            use ::std::option::Option::{None, Some};

            let case_opt = self::GroupTwoCase::from_bitslice(bits);
            if let Some(GroupTwoCase::G2Submsg) = case_opt {
                let item = unsafe { &self.g2_submsg };
                Some(item.get_field())
            } else {
                None
            }
        }

        pub(crate) fn _clear<B: self::_puroro::bitvec::BitSlice>(&mut self, bits: &mut B) {
            use self::_puroro::bitvec::BitSlice;
            // TODO: take manuallydrop value
            bits.set_range::<2, 4>(3);
            todo!()
        }
    }

    impl self::_puroro::internal::oneof_type::OneofCase<2, 4> for GroupTwoCase {
        fn from_u32(x: u32) -> Option<Self> {
            #[allow(unused)]
            use ::std::option::Option::{None, Some};
            match x {
                0 => Some(self::GroupTwoCase::G2F32),
                1 => Some(self::GroupTwoCase::G2String),
                2 => Some(self::GroupTwoCase::G2Submsg),
                _ => None,
            }
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
        pub(crate) fn g3_int32_opt<B: self::_puroro::bitvec::BitSlice>(
            &self,
            bits: &B,
        ) -> ::std::option::Option<i32> {
            #[allow(unused)]
            use self::_puroro::internal::oneof_field_type::OneofFieldType;
            #[allow(unused)]
            use self::_puroro::internal::oneof_type::OneofCase;
            #[allow(unused)]
            use ::std::option::Option::{None, Some};

            let case_opt = self::GroupThreeCase::from_bitslice(bits);
            if let Some(GroupThreeCase::G3Int32) = case_opt {
                let item = unsafe { &self.g3_int32 };
                Some(item.get_field())
            } else {
                None
            }
        }

        pub(crate) fn _clear<B: self::_puroro::bitvec::BitSlice>(&mut self, bits: &mut B) {
            use self::_puroro::bitvec::BitSlice;
            // TODO: take manuallydrop value
            bits.set_range::<4, 5>(1);
            todo!()
        }
    }

    impl self::_puroro::internal::oneof_type::OneofCase<4, 5> for GroupThreeCase {
        fn from_u32(x: u32) -> Option<Self> {
            #[allow(unused)]
            use ::std::option::Option::{None, Some};
            match x {
                0 => Some(self::GroupThreeCase::G3Int32),
                _ => None,
            }
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
