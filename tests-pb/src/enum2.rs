// A generated source code by puroro library
// package enum2

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub mod _puroro {
    pub use ::puroro::*;
}

#[derive(Default, Clone)]
pub struct Msg {
    // Optional, Variant(Enum2(Fqtn(".enum2.Enum")))
    enum_optional: self::_puroro::internal::field_type::OptionalNumericalField<
        _puroro_root::enum2::Enum,
        self::_puroro::tags::Enum2<_puroro_root::enum2::Enum>,
        0,
    >,
    // Repeated, Variant(Enum2(Fqtn(".enum2.Enum")))
    enum_repeated: self::_puroro::internal::field_type::RepeatedNumericalField<
        _puroro_root::enum2::Enum,
        self::_puroro::tags::Enum2<_puroro_root::enum2::Enum>,
    >,

    _bitfield: self::_puroro::bitvec::BitArray<1>,
}

impl Msg {
    // Optional, Variant(Enum2(Fqtn(".enum2.Enum")))
    pub fn enum_optional(&self) -> _puroro_root::enum2::Enum {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::enum2::Enum,
            self::_puroro::tags::Enum2<_puroro_root::enum2::Enum>,
            0,
        > as NonRepeatedFieldType>::get_field(
            &self.enum_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn enum_optional_opt(&self) -> ::std::option::Option<_puroro_root::enum2::Enum> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::enum2::Enum,
            self::_puroro::tags::Enum2<_puroro_root::enum2::Enum>,
            0,
        > as NonRepeatedFieldType>::get_field_opt(&self.enum_optional, &self._bitfield)
    }
    pub fn has_enum_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::enum2::Enum,
            self::_puroro::tags::Enum2<_puroro_root::enum2::Enum>,
            0,
        > as NonRepeatedFieldType>::get_field_opt(&self.enum_optional, &self._bitfield)
        .is_some()
    }
    pub fn enum_optional_mut(&mut self) -> &mut _puroro_root::enum2::Enum {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::enum2::Enum,
            self::_puroro::tags::Enum2<_puroro_root::enum2::Enum>,
            0,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.enum_optional,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_enum_optional(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::enum2::Enum,
            self::_puroro::tags::Enum2<_puroro_root::enum2::Enum>,
            0,
        > as NonRepeatedFieldType>::clear(&mut self.enum_optional, &mut self._bitfield)
    }
    // Repeated, Variant(Enum2(Fqtn(".enum2.Enum")))
    pub fn enum_repeated(&self) -> &[_puroro_root::enum2::Enum] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            _puroro_root::enum2::Enum,
            self::_puroro::tags::Enum2<_puroro_root::enum2::Enum>,
        > as RepeatedFieldType>::get_field(&self.enum_repeated, &self._bitfield)
    }
    pub fn enum_repeated_mut(&mut self) -> &mut ::std::vec::Vec<_puroro_root::enum2::Enum> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            _puroro_root::enum2::Enum,
            self::_puroro::tags::Enum2<_puroro_root::enum2::Enum>,
        > as RepeatedFieldType>::mut_field(&mut self.enum_repeated, &mut self._bitfield)
    }
    pub fn clear_enum_repeated(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            _puroro_root::enum2::Enum,
            self::_puroro::tags::Enum2<_puroro_root::enum2::Enum>,
        > as RepeatedFieldType>::clear(&mut self.enum_repeated, &mut self._bitfield)
    }
}

impl self::_puroro::Message for Msg {
    fn from_bytes_iter<I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>>(
        iter: I,
    ) -> self::_puroro::Result<Self> {
        #[allow(unused)]
        use ::std::result::Result::Ok;
        let mut msg: Self = ::std::default::Default::default();
        msg.merge_from_bytes_iter(iter)?;
        Ok(msg)
    }

    fn merge_from_bytes_iter<I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>>(
        &mut self,
        mut iter: I,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use ::std::option::Option::Some;
        #[allow(unused)]
        use ::std::result::Result::Ok;
        while let Some((number, _field_data)) =
            self::_puroro::internal::ser::FieldData::from_bytes_iter(iter.by_ref())?
        {
            match number {
                1 => <self::_puroro::internal::field_type::OptionalNumericalField<
                    _puroro_root::enum2::Enum,
                    self::_puroro::tags::Enum2<_puroro_root::enum2::Enum>,
                    0,
                > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                    &mut self.enum_optional,
                    &mut self._bitfield,
                    _field_data,
                )?,
                2 => <self::_puroro::internal::field_type::RepeatedNumericalField<
                    _puroro_root::enum2::Enum,
                    self::_puroro::tags::Enum2<_puroro_root::enum2::Enum>,
                > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                    &mut self.enum_repeated,
                    &mut self._bitfield,
                    _field_data,
                )?,
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
}

#[derive(Debug, Clone, PartialEq)]
pub enum Enum {
    ValueSeven,
    ValueZero,
    ValueOne,
    ValueFourtyTwo,
}

impl ::std::default::Default for Enum {
    fn default() -> Self {
        Enum::ValueSeven
    }
}

impl ::std::convert::TryFrom<i32> for Enum {
    type Error = self::_puroro::PuroroError;
    fn try_from(x: i32) -> ::std::result::Result<Self, Self::Error> {
        #[allow(unused)]
        use ::std::result::Result::{Err, Ok};
        match x {
            7 => Ok(self::Enum::ValueSeven),
            0 => Ok(self::Enum::ValueZero),
            1 => Ok(self::Enum::ValueOne),
            42 => Ok(self::Enum::ValueFourtyTwo),
            e => Err(self::_puroro::ErrorKind::UnknownEnumVariant(e))?,
        }
    }
}

impl ::std::convert::From<Enum> for i32 {
    fn from(x: Enum) -> i32 {
        match x {
            self::Enum::ValueSeven => 7,
            self::Enum::ValueZero => 0,
            self::Enum::ValueOne => 1,
            self::Enum::ValueFourtyTwo => 42,
        }
    }
}
