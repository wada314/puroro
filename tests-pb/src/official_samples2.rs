// A generated source code by puroro library
// package official_samples2

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub mod _puroro {
    pub use ::puroro::*;
}

#[derive(Default, Clone)]
pub struct Test1 {
    // Optional, Variant(Int32)
    a: self::_puroro::internal::field_types::OptionalNumericalField<
        i32,
        self::_puroro::tags::Int32,
        0,
    >,

    _bitfield: self::_puroro::bitvec::BitArray<1>,
}

impl Test1 {
    // Optional, Variant(Int32)
    pub fn a(&self) -> i32 {
        #[allow(unused)]
        use self::_puroro::internal::field_types::{NonRepeatedFieldType, RepeatedFieldType};
        <self::_puroro::internal::field_types::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as NonRepeatedFieldType>::get_field(
            &self.a,
            &self._bitfield,
            ::std::default::Default::default(),
        )
    }
}

impl self::_puroro::Message for Test1 {
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
                1 => <self::_puroro::internal::field_types::OptionalNumericalField<
                    i32,
                    self::_puroro::tags::Int32,
                    0,
                > as self::_puroro::internal::field_types::FieldType>::deser_from_iter(
                    &mut self.a,
                    &mut self._bitfield,
                    field_data,
                )?,
                _ => todo!(),
            }
        }
        Ok(())
    }
}
pub mod _test1 {

    mod _puroro {
        pub use super::super::_puroro::*;
    }
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
}

#[derive(Default, Clone)]
pub struct Test2 {
    // Optional, LengthDelimited(String)
    b: self::_puroro::internal::field_types::OptionalStringField<0>,

    _bitfield: self::_puroro::bitvec::BitArray<1>,
}

impl Test2 {
    // Optional, LengthDelimited(String)
    pub fn b(&self) -> &str {
        #[allow(unused)]
        use self::_puroro::internal::field_types::{NonRepeatedFieldType, RepeatedFieldType};
        <self::_puroro::internal::field_types::OptionalStringField<0> as NonRepeatedFieldType>::get_field(
            &self.b, &self._bitfield, ::std::default::Default::default(),
        )
    }
}

impl self::_puroro::Message for Test2 {
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
                2 => <
                    self::_puroro::internal::field_types::OptionalStringField<0> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut self.b,
                    &mut self._bitfield,
                    field_data,
                )?,
                _ => todo!(),
            }
        }
        Ok(())
    }
}
pub mod _test2 {

    mod _puroro {
        pub use super::super::_puroro::*;
    }
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
}

#[derive(Default, Clone)]
pub struct Test3 {
    // Optional, LengthDelimited(Message(Fqtn(".official_samples2.Test1")))
    c: self::_puroro::internal::field_types::SingularHeapMessageField<
        _puroro_root::official_samples2::Test1,
    >,

    _bitfield: self::_puroro::bitvec::BitArray<1>,
}

impl Test3 {
    // Optional, LengthDelimited(Message(Fqtn(".official_samples2.Test1")))
    pub fn c(&self) -> Option<&_puroro_root::official_samples2::Test1> {
        #[allow(unused)]
        use self::_puroro::internal::field_types::{NonRepeatedFieldType, RepeatedFieldType};
        <self::_puroro::internal::field_types::SingularHeapMessageField<
            _puroro_root::official_samples2::Test1,
        > as NonRepeatedFieldType>::get_field(
            &self.c,
            &self._bitfield,
            ::std::default::Default::default(),
        )
    }
}

impl self::_puroro::Message for Test3 {
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
                3 => <self::_puroro::internal::field_types::SingularHeapMessageField<
                    _puroro_root::official_samples2::Test1,
                > as self::_puroro::internal::field_types::FieldType>::deser_from_iter(
                    &mut self.c,
                    &mut self._bitfield,
                    field_data,
                )?,
                _ => todo!(),
            }
        }
        Ok(())
    }
}
pub mod _test3 {

    mod _puroro {
        pub use super::super::_puroro::*;
    }
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
}

#[derive(Default, Clone)]
pub struct Test4 {
    // Repeated, Variant(Int32)
    d: self::_puroro::internal::field_types::RepeatedNumericalField<
        i32,
        self::_puroro::tags::Int32,
    >,

    _bitfield: self::_puroro::bitvec::BitArray<0>,
}

impl Test4 {
    // Repeated, Variant(Int32)
    pub fn d(&self) -> &[i32] {
        #[allow(unused)]
        use self::_puroro::internal::field_types::{NonRepeatedFieldType, RepeatedFieldType};
        <self::_puroro::internal::field_types::RepeatedNumericalField<
            i32,
            self::_puroro::tags::Int32,
        > as RepeatedFieldType>::get_field(&self.d, &self._bitfield)
    }
}

impl self::_puroro::Message for Test4 {
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
                4 => <self::_puroro::internal::field_types::RepeatedNumericalField<
                    i32,
                    self::_puroro::tags::Int32,
                > as self::_puroro::internal::field_types::FieldType>::deser_from_iter(
                    &mut self.d,
                    &mut self._bitfield,
                    field_data,
                )?,
                _ => todo!(),
            }
        }
        Ok(())
    }
}
pub mod _test4 {

    mod _puroro {
        pub use super::super::_puroro::*;
    }
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
}
