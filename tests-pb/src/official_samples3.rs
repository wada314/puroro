// A generated source code by puroro library
// package official_samples3

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub mod _puroro {
    pub use ::puroro::*;
}

#[derive(Default, Clone)]
pub struct Test1 {
    // Singular, Variant(Int32)
    a: self::_puroro::internal::field_type::SingularNumericalField<i32, self::_puroro::tags::Int32>,

    _bitfield: self::_puroro::bitvec::BitArray<0>,
}

impl Test1 {
    // Singular, Variant(Int32)
    pub fn a(&self) -> i32 {
        #[allow(unused)]
        use self::_puroro::internal::field_type::{NonRepeatedFieldType, RepeatedFieldType};
        <self::_puroro::internal::field_type::SingularNumericalField<i32, self::_puroro::tags::Int32> as NonRepeatedFieldType>::get_field(
            &self.a, &self._bitfield, ::std::default::Default::default(),
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
                1 => <self::_puroro::internal::field_type::SingularNumericalField<
                    i32,
                    self::_puroro::tags::Int32,
                > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
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
    // Singular, LengthDelimited(String)
    b: self::_puroro::internal::field_type::SingularStringField,

    _bitfield: self::_puroro::bitvec::BitArray<0>,
}

impl Test2 {
    // Singular, LengthDelimited(String)
    pub fn b(&self) -> &str {
        #[allow(unused)]
        use self::_puroro::internal::field_type::{NonRepeatedFieldType, RepeatedFieldType};
        <self::_puroro::internal::field_type::SingularStringField as NonRepeatedFieldType>::get_field(
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
                    self::_puroro::internal::field_type::SingularStringField as self::_puroro::internal::field_type::FieldType
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
    // Singular, LengthDelimited(Message(Fqtn(".official_samples3.Test1")))
    c: self::_puroro::internal::field_type::SingularHeapMessageField<
        _puroro_root::official_samples3::Test1,
    >,

    _bitfield: self::_puroro::bitvec::BitArray<0>,
}

impl Test3 {
    // Singular, LengthDelimited(Message(Fqtn(".official_samples3.Test1")))
    pub fn c(&self) -> Option<&_puroro_root::official_samples3::Test1> {
        #[allow(unused)]
        use self::_puroro::internal::field_type::{NonRepeatedFieldType, RepeatedFieldType};
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::official_samples3::Test1,
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
                3 => <self::_puroro::internal::field_type::SingularHeapMessageField<
                    _puroro_root::official_samples3::Test1,
                > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
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
    d: self::_puroro::internal::field_type::RepeatedNumericalField<i32, self::_puroro::tags::Int32>,

    _bitfield: self::_puroro::bitvec::BitArray<0>,
}

impl Test4 {
    // Repeated, Variant(Int32)
    pub fn d(&self) -> &[i32] {
        #[allow(unused)]
        use self::_puroro::internal::field_type::{NonRepeatedFieldType, RepeatedFieldType};
        <self::_puroro::internal::field_type::RepeatedNumericalField<i32, self::_puroro::tags::Int32> as RepeatedFieldType>::get_field(
            &self.d, &self._bitfield, 
        )
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
                4 => <self::_puroro::internal::field_type::RepeatedNumericalField<
                    i32,
                    self::_puroro::tags::Int32,
                > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
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
