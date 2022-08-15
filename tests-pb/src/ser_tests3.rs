
// A generated source code by puroro library
// package ser_tests3
pub mod _msg;

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub mod _puroro {
    pub use ::puroro::*;
}

#[derive(Default, Clone)]
pub struct Msg {
    // Singular, Variant(Int32)
    i32_unlabeled: self::_puroro::internal::field_types::SingularVariantField<i32, self::_puroro::tags::Int32>,
    // Singular, Bits32(Float)
    float_unlabeled: self::_puroro::internal::field_types::,
    // Singular, LengthDelimited(String)
    string_unlabeled: self::_puroro::internal::field_types::SingularStringField,
    // Singular, LengthDelimited(Message(Fqtn(".ser_tests3.Msg.Submsg")))
    submsg_unlabeled: self::_puroro::internal::field_types::SingularHeapMessageField<_puroro_root::ser_tests3::_msg::Submsg>,
    // Singular, Variant(Enum3(Fqtn(".ser_tests3.Enum")))
    enum_unlabeled: self::_puroro::internal::field_types::SingularVariantField<_puroro_root::ser_tests3::Enum, self::_puroro::tags::Enum3<_puroro_root::ser_tests3::Enum>>,
    // Singular, Variant(Int32)
    very_large_field_number: self::_puroro::internal::field_types::SingularVariantField<i32, self::_puroro::tags::Int32>,

    _bitfield: self::_puroro::bitvec::BitArray<0>,
}

impl Msg {
    // Singular, Variant(Int32)
    pub fn i32_unlabeled(&self) -> i32 {
        <self::_puroro::internal::field_types::SingularVariantField<i32, self::_puroro::tags::Int32> as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.i32_unlabeled, &self._bitfield,
        )
    }
    // Singular, Bits32(Float)
    pub fn float_unlabeled(&self) -> f32 {
        <self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.float_unlabeled, &self._bitfield,
        )
    }
    // Singular, LengthDelimited(String)
    pub fn string_unlabeled(&self) -> &str {
        <self::_puroro::internal::field_types::SingularStringField as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.string_unlabeled, &self._bitfield,
        )
    }
    // Singular, LengthDelimited(Message(Fqtn(".ser_tests3.Msg.Submsg")))
    pub fn submsg_unlabeled(&self) -> Option<&_puroro_root::ser_tests3::_msg::Submsg> {
        <self::_puroro::internal::field_types::SingularHeapMessageField<_puroro_root::ser_tests3::_msg::Submsg> as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.submsg_unlabeled, &self._bitfield,
        )
    }
    // Singular, Variant(Enum3(Fqtn(".ser_tests3.Enum")))
    pub fn enum_unlabeled(&self) -> _puroro_root::ser_tests3::Enum {
        <self::_puroro::internal::field_types::SingularVariantField<_puroro_root::ser_tests3::Enum, self::_puroro::tags::Enum3<_puroro_root::ser_tests3::Enum>> as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.enum_unlabeled, &self._bitfield,
        )
    }
    // Singular, Variant(Int32)
    pub fn very_large_field_number(&self) -> i32 {
        <self::_puroro::internal::field_types::SingularVariantField<i32, self::_puroro::tags::Int32> as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.very_large_field_number, &self._bitfield,
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
                    self::_puroro::internal::field_types::SingularVariantField<i32, self::_puroro::tags::Int32> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.i32_unlabeled,
                    &mut msg._bitfield,
                    field_data,
                )?,
                2 => <
                    self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.i32_repeated,
                    &mut msg._bitfield,
                    field_data,
                )?,
                3 => <
                    self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.float_unlabeled,
                    &mut msg._bitfield,
                    field_data,
                )?,
                4 => <
                    self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.float_repeated,
                    &mut msg._bitfield,
                    field_data,
                )?,
                5 => <
                    self::_puroro::internal::field_types::SingularStringField as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.string_unlabeled,
                    &mut msg._bitfield,
                    field_data,
                )?,
                6 => <
                    self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.string_repeated,
                    &mut msg._bitfield,
                    field_data,
                )?,
                7 => <
                    self::_puroro::internal::field_types::SingularHeapMessageField<_puroro_root::ser_tests3::_msg::Submsg> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.submsg_unlabeled,
                    &mut msg._bitfield,
                    field_data,
                )?,
                8 => <
                    self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.submsg_repeated,
                    &mut msg._bitfield,
                    field_data,
                )?,
                9 => <
                    self::_puroro::internal::field_types::SingularVariantField<_puroro_root::ser_tests3::Enum, self::_puroro::tags::Enum3<_puroro_root::ser_tests3::Enum>> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.enum_unlabeled,
                    &mut msg._bitfield,
                    field_data,
                )?,
                10 => <
                    self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.enum_repeated,
                    &mut msg._bitfield,
                    field_data,
                )?,
                536870911 => <
                    self::_puroro::internal::field_types::SingularVariantField<i32, self::_puroro::tags::Int32> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.very_large_field_number,
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
