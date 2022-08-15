
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
    // Optional, Variant(Int32)
    g1_int32: self::_puroro::internal::field_types::OptionalVariantField<i32, self::_puroro::tags::Int32, 0>,
    // Optional, LengthDelimited(String)
    g1_string: self::_puroro::internal::field_types::OptionalStringField<1>,
    // Optional, Bits32(Float)
    g2_f32: self::_puroro::internal::field_types::,
    // Optional, LengthDelimited(String)
    g2_string: self::_puroro::internal::field_types::OptionalStringField<3>,
    // Optional, LengthDelimited(Message(Fqtn(".oneofs2.Submsg")))
    g2_submsg: self::_puroro::internal::field_types::SingularHeapMessageField<_puroro_root::oneofs2::Submsg>,
    // Optional, Variant(Int32)
    g3_int32: self::_puroro::internal::field_types::OptionalVariantField<i32, self::_puroro::tags::Int32, 5>,

    _bitfield: self::_puroro::bitvec::BitArray<1>,
}

impl Msg {
    // Optional, Variant(Int32)
    pub fn g1_int32(&self) -> i32 {
        <self::_puroro::internal::field_types::OptionalVariantField<i32, self::_puroro::tags::Int32, 0> as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.g1_int32, &self._bitfield,
        )
    }
    // Optional, LengthDelimited(String)
    pub fn g1_string(&self) -> &str {
        <self::_puroro::internal::field_types::OptionalStringField<1> as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.g1_string, &self._bitfield,
        )
    }
    // Optional, Bits32(Float)
    pub fn g2_f32(&self) -> f32 {
        <self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.g2_f32, &self._bitfield,
        )
    }
    // Optional, LengthDelimited(String)
    pub fn g2_string(&self) -> &str {
        <self::_puroro::internal::field_types::OptionalStringField<3> as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.g2_string, &self._bitfield,
        )
    }
    // Optional, LengthDelimited(Message(Fqtn(".oneofs2.Submsg")))
    pub fn g2_submsg(&self) -> Option<&_puroro_root::oneofs2::Submsg> {
        <self::_puroro::internal::field_types::SingularHeapMessageField<_puroro_root::oneofs2::Submsg> as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.g2_submsg, &self._bitfield,
        )
    }
    // Optional, Variant(Int32)
    pub fn g3_int32(&self) -> i32 {
        <self::_puroro::internal::field_types::OptionalVariantField<i32, self::_puroro::tags::Int32, 5> as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.g3_int32, &self._bitfield,
        )
    }
}

impl self::_puroro::Message for Msg {
    fn from_bytes_iter<I: ::std::iter::Iterator<Item=::std::io::Result<u8>>>(iter: I) -> self::_puroro::Result<Self> {
        let mut msg: Self = ::std::default::Default::default();
        let mut peekable = iter.peekable();
        while peekable.peek().is_some() {
            let (number, field_data) = self::_puroro::internal::ser::FieldData::from_bytes_iter(peekable.by_ref())?;
            match number {
                1 => <
                    self::_puroro::internal::field_types::OptionalVariantField<i32, self::_puroro::tags::Int32, 0> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.g1_int32,
                    &mut msg._bitfield,
                    field_data,
                )?,
                2 => <
                    self::_puroro::internal::field_types::OptionalStringField<1> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.g1_string,
                    &mut msg._bitfield,
                    field_data,
                )?,
                3 => <
                    self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.g2_f32,
                    &mut msg._bitfield,
                    field_data,
                )?,
                4 => <
                    self::_puroro::internal::field_types::OptionalStringField<3> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.g2_string,
                    &mut msg._bitfield,
                    field_data,
                )?,
                5 => <
                    self::_puroro::internal::field_types::SingularHeapMessageField<_puroro_root::oneofs2::Submsg> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.g2_submsg,
                    &mut msg._bitfield,
                    field_data,
                )?,
                6 => <
                    self::_puroro::internal::field_types::OptionalVariantField<i32, self::_puroro::tags::Int32, 5> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.g3_int32,
                    &mut msg._bitfield,
                    field_data,
                )?,
                _ => todo!(),
            }
        }
        Ok(msg)
    }
}

pub mod _msg {

    mod _puroro {
        pub use super::super::_puroro::*;
    }
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }

}

#[derive(Default, Clone)]
pub struct Submsg {
    // Optional, Variant(Int32)
    i32_optional: self::_puroro::internal::field_types::OptionalVariantField<i32, self::_puroro::tags::Int32, 0>,

    _bitfield: self::_puroro::bitvec::BitArray<1>,
}

impl Submsg {
    // Optional, Variant(Int32)
    pub fn i32_optional(&self) -> i32 {
        <self::_puroro::internal::field_types::OptionalVariantField<i32, self::_puroro::tags::Int32, 0> as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.i32_optional, &self._bitfield,
        )
    }
}

impl self::_puroro::Message for Submsg {
    fn from_bytes_iter<I: ::std::iter::Iterator<Item=::std::io::Result<u8>>>(iter: I) -> self::_puroro::Result<Self> {
        let mut msg: Self = ::std::default::Default::default();
        let mut peekable = iter.peekable();
        while peekable.peek().is_some() {
            let (number, field_data) = self::_puroro::internal::ser::FieldData::from_bytes_iter(peekable.by_ref())?;
            match number {
                1 => <
                    self::_puroro::internal::field_types::OptionalVariantField<i32, self::_puroro::tags::Int32, 0> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.i32_optional,
                    &mut msg._bitfield,
                    field_data,
                )?,
                _ => todo!(),
            }
        }
        Ok(msg)
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