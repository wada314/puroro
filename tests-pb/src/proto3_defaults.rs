// A generated source code by puroro library
// package proto3_defaults

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub mod _puroro {
    pub use ::puroro::*;
}

#[derive(Default, Clone)]
pub struct Msg {
    // Singular, Variant(Int32)
    i32_unlabeled:
        self::_puroro::internal::field_types::SingularVariantField<i32, self::_puroro::tags::Int32>,
    // Optional, Variant(Int32)
    i32_optional: self::_puroro::internal::field_types::OptionalVariantField<
        i32,
        self::_puroro::tags::Int32,
        0,
    >,
    // Singular, Bits32(Float)
    f32_unlabeled: self::_puroro::internal::field_types::Dummy,
    // Singular, LengthDelimited(String)
    string_unlabeled: self::_puroro::internal::field_types::SingularStringField,
    // Singular, LengthDelimited(Message(Fqtn(".proto3_defaults.Submsg")))
    submsg_unlabeled: self::_puroro::internal::field_types::SingularHeapMessageField<
        _puroro_root::proto3_defaults::Submsg,
    >,

    _bitfield: self::_puroro::bitvec::BitArray<1>,
}

impl Msg {
    // Singular, Variant(Int32)
    pub fn i32_unlabeled(&self) -> i32 {
        <self::_puroro::internal::field_types::SingularVariantField<i32, self::_puroro::tags::Int32> as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.i32_unlabeled, &self._bitfield,
        )
    }
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
    // Singular, Bits32(Float)
    pub fn f32_unlabeled(&self) -> f32 {
        <self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.f32_unlabeled, &self._bitfield,
        )
    }
    // Singular, LengthDelimited(String)
    pub fn string_unlabeled(&self) -> &str {
        <self::_puroro::internal::field_types::SingularStringField as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.string_unlabeled, &self._bitfield,
        )
    }
    // Singular, LengthDelimited(Message(Fqtn(".proto3_defaults.Submsg")))
    pub fn submsg_unlabeled(&self) -> Option<&_puroro_root::proto3_defaults::Submsg> {
        <self::_puroro::internal::field_types::SingularHeapMessageField<
            _puroro_root::proto3_defaults::Submsg,
        > as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.submsg_unlabeled,
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
                    self::_puroro::internal::field_types::SingularVariantField<i32, self::_puroro::tags::Int32> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.i32_unlabeled,
                    &mut msg._bitfield,
                    field_data,
                )?,
                2 => <
                    self::_puroro::internal::field_types::OptionalVariantField<i32, self::_puroro::tags::Int32, 0> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.i32_optional,
                    &mut msg._bitfield,
                    field_data,
                )?,
                3 => <
                    self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.i32_repeated,
                    &mut msg._bitfield,
                    field_data,
                )?,
                4 => <
                    self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.f32_unlabeled,
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
                    self::_puroro::internal::field_types::SingularHeapMessageField<_puroro_root::proto3_defaults::Submsg> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.submsg_unlabeled,
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
    // Singular, Variant(Int32)
    i32_unlabeled:
        self::_puroro::internal::field_types::SingularVariantField<i32, self::_puroro::tags::Int32>,

    _bitfield: self::_puroro::bitvec::BitArray<0>,
}

impl Submsg {
    // Singular, Variant(Int32)
    pub fn i32_unlabeled(&self) -> i32 {
        <self::_puroro::internal::field_types::SingularVariantField<i32, self::_puroro::tags::Int32> as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.i32_unlabeled, &self._bitfield,
        )
    }
}

impl self::_puroro::Message for Submsg {
    fn from_bytes_iter<I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>>(
        iter: I,
    ) -> self::_puroro::Result<Self> {
        let mut msg: Self = ::std::default::Default::default();
        let mut peekable = iter.peekable();
        while peekable.peek().is_some() {
            let (number, field_data) =
                self::_puroro::internal::ser::FieldData::from_bytes_iter(peekable.by_ref())?;
            match number {
                1 => <self::_puroro::internal::field_types::SingularVariantField<
                    i32,
                    self::_puroro::tags::Int32,
                > as self::_puroro::internal::field_types::FieldType>::deser_from_iter(
                    &mut msg.i32_unlabeled,
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
