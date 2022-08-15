// A generated source code by puroro library
// package ser_tests2
pub mod _msg;

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub mod _puroro {
    pub use ::puroro::*;
}

#[derive(Default, Clone)]
pub struct Msg {
    // Optional, Variant(Int32)
    i32_optional: self::_puroro::internal::field_types::OptionalVariantField<
        i32,
        self::_puroro::tags::Int32,
        0,
    >,
    // Optional, Bits32(Float)
    float_optional: self::_puroro::internal::field_types::Dummy,
    // Optional, LengthDelimited(String)
    string_optional: self::_puroro::internal::field_types::OptionalStringField<2>,
    // Optional, LengthDelimited(Message(Fqtn(".ser_tests2.Msg.Submsg")))
    submsg_optional: self::_puroro::internal::field_types::SingularHeapMessageField<
        _puroro_root::ser_tests2::_msg::Submsg,
    >,
    // Optional, Variant(Enum2(Fqtn(".ser_tests2.Enum")))
    enum_optional: self::_puroro::internal::field_types::OptionalVariantField<
        _puroro_root::ser_tests2::Enum,
        self::_puroro::tags::Enum2<_puroro_root::ser_tests2::Enum>,
        4,
    >,
    // Optional, Variant(Int32)
    very_large_field_number: self::_puroro::internal::field_types::OptionalVariantField<
        i32,
        self::_puroro::tags::Int32,
        5,
    >,

    _bitfield: self::_puroro::bitvec::BitArray<1>,
}

impl Msg {
    // Optional, Variant(Int32)
    pub fn i32_optional(&self) -> i32 {
        <self::_puroro::internal::field_types::OptionalVariantField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.i32_optional,
            &self._bitfield,
        )
    }
    // Optional, Bits32(Float)
    pub fn float_optional(&self) -> f32 {
        <self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.float_optional, &self._bitfield,
        )
    }
    // Optional, LengthDelimited(String)
    pub fn string_optional(&self) -> &str {
        <self::_puroro::internal::field_types::OptionalStringField<2> as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.string_optional, &self._bitfield,
        )
    }
    // Optional, LengthDelimited(Message(Fqtn(".ser_tests2.Msg.Submsg")))
    pub fn submsg_optional(&self) -> Option<&_puroro_root::ser_tests2::_msg::Submsg> {
        <self::_puroro::internal::field_types::SingularHeapMessageField<
            _puroro_root::ser_tests2::_msg::Submsg,
        > as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.submsg_optional,
            &self._bitfield,
        )
    }
    // Optional, Variant(Enum2(Fqtn(".ser_tests2.Enum")))
    pub fn enum_optional(&self) -> _puroro_root::ser_tests2::Enum {
        <self::_puroro::internal::field_types::OptionalVariantField<
            _puroro_root::ser_tests2::Enum,
            self::_puroro::tags::Enum2<_puroro_root::ser_tests2::Enum>,
            4,
        > as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.enum_optional,
            &self._bitfield,
        )
    }
    // Optional, Variant(Int32)
    pub fn very_large_field_number(&self) -> i32 {
        <self::_puroro::internal::field_types::OptionalVariantField<
            i32,
            self::_puroro::tags::Int32,
            5,
        > as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.very_large_field_number,
            &self._bitfield,
        )
    }
}

impl self::_puroro::Message for Msg {
    fn from_bytes_iter<I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>>(
        iter: I,
    ) -> self::_puroro::Result<Self> {
        let mut msg: Self = ::std::default::Default::default();
        let mut peekable = iter.peekable();
        while peekable.peek().is_some() {
            let (number, field_data) =
                self::_puroro::internal::ser::FieldData::from_bytes_iter(peekable.by_ref())?;
            match number {
                1 => <
                    self::_puroro::internal::field_types::OptionalVariantField<i32, self::_puroro::tags::Int32, 0> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.i32_optional,
                    &mut msg._bitfield,
                    field_data,
                )?,
                2 => <
                    self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.i32_repeated,
                    &mut msg._bitfield,
                    field_data,
                )?,
                3 => <
                    self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.float_optional,
                    &mut msg._bitfield,
                    field_data,
                )?,
                4 => <
                    self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.float_repeated,
                    &mut msg._bitfield,
                    field_data,
                )?,
                5 => <
                    self::_puroro::internal::field_types::OptionalStringField<2> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.string_optional,
                    &mut msg._bitfield,
                    field_data,
                )?,
                6 => <
                    self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.string_repeated,
                    &mut msg._bitfield,
                    field_data,
                )?,
                7 => <
                    self::_puroro::internal::field_types::SingularHeapMessageField<_puroro_root::ser_tests2::_msg::Submsg> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.submsg_optional,
                    &mut msg._bitfield,
                    field_data,
                )?,
                8 => <
                    self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.submsg_repeated,
                    &mut msg._bitfield,
                    field_data,
                )?,
                9 => <
                    self::_puroro::internal::field_types::OptionalVariantField<_puroro_root::ser_tests2::Enum, self::_puroro::tags::Enum2<_puroro_root::ser_tests2::Enum>, 4> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.enum_optional,
                    &mut msg._bitfield,
                    field_data,
                )?,
                10 => <
                    self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.enum_repeated,
                    &mut msg._bitfield,
                    field_data,
                )?,
                536870911 => <
                    self::_puroro::internal::field_types::OptionalVariantField<i32, self::_puroro::tags::Int32, 5> as self::_puroro::internal::field_types::FieldType
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
