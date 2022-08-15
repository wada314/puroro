// A generated source code by puroro library
// package oneofs3

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub mod _puroro {
    pub use ::puroro::*;
}

#[derive(Default, Clone)]
pub struct Msg {
    // Singular, Variant(Int32)
    g1_int32:
        self::_puroro::internal::field_types::SingularVariantField<i32, self::_puroro::tags::Int32>,
    // Singular, LengthDelimited(String)
    g1_string: self::_puroro::internal::field_types::SingularStringField,
    // Singular, Bits32(Float)
    g2_f32: self::_puroro::internal::field_types::Dummy,
    // Singular, LengthDelimited(String)
    g2_string: self::_puroro::internal::field_types::SingularStringField,
    // Singular, LengthDelimited(Message(Fqtn(".oneofs3.Submsg")))
    g2_submsg: self::_puroro::internal::field_types::SingularHeapMessageField<
        _puroro_root::oneofs3::Submsg,
    >,
    // Singular, Variant(Int32)
    g3_int32:
        self::_puroro::internal::field_types::SingularVariantField<i32, self::_puroro::tags::Int32>,

    _bitfield: self::_puroro::bitvec::BitArray<0>,
}

impl Msg {
    // Singular, Variant(Int32)
    pub fn g1_int32(&self) -> i32 {
        <self::_puroro::internal::field_types::SingularVariantField<i32, self::_puroro::tags::Int32> as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.g1_int32, &self._bitfield,
        )
    }
    // Singular, LengthDelimited(String)
    pub fn g1_string(&self) -> &str {
        <self::_puroro::internal::field_types::SingularStringField as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.g1_string, &self._bitfield,
        )
    }
    // Singular, Bits32(Float)
    pub fn g2_f32(&self) -> f32 {
        <self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.g2_f32, &self._bitfield,
        )
    }
    // Singular, LengthDelimited(String)
    pub fn g2_string(&self) -> &str {
        <self::_puroro::internal::field_types::SingularStringField as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.g2_string, &self._bitfield,
        )
    }
    // Singular, LengthDelimited(Message(Fqtn(".oneofs3.Submsg")))
    pub fn g2_submsg(&self) -> Option<&_puroro_root::oneofs3::Submsg> {
        <self::_puroro::internal::field_types::SingularHeapMessageField<
            _puroro_root::oneofs3::Submsg,
        > as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.g2_submsg,
            &self._bitfield,
        )
    }
    // Singular, Variant(Int32)
    pub fn g3_int32(&self) -> i32 {
        <self::_puroro::internal::field_types::SingularVariantField<i32, self::_puroro::tags::Int32> as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.g3_int32, &self._bitfield,
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
                    &mut msg.g1_int32,
                    &mut msg._bitfield,
                    field_data,
                )?,
                2 => <
                    self::_puroro::internal::field_types::SingularStringField as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.g1_string,
                    &mut msg._bitfield,
                    field_data,
                )?,
                3 => <
                    self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.g2_f32,
                    &mut msg._bitfield,
                    field_data,
                )?,
                4 => <
                    self::_puroro::internal::field_types::SingularStringField as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.g2_string,
                    &mut msg._bitfield,
                    field_data,
                )?,
                5 => <
                    self::_puroro::internal::field_types::SingularHeapMessageField<_puroro_root::oneofs3::Submsg> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.g2_submsg,
                    &mut msg._bitfield,
                    field_data,
                )?,
                6 => <
                    self::_puroro::internal::field_types::SingularVariantField<i32, self::_puroro::tags::Int32> as self::_puroro::internal::field_types::FieldType
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
