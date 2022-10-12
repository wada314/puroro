// A generated source code by puroro library
// package enum3

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub mod _puroro {
    pub use ::puroro::*;
}

#[derive(Default)]
pub struct Msg {
    // Optional, Variant(Enum3(Fqtn(".enum3.Enum")))
    enum_optional: self::_puroro::internal::field_type::OptionalNumericalField<
        _puroro_root::enum3::Enum,
        self::_puroro::tags::Enum3<_puroro_root::enum3::Enum>,
        0,
    >,
    // Singular, Variant(Enum3(Fqtn(".enum3.Enum")))
    enum_unlabeled: self::_puroro::internal::field_type::SingularNumericalField<
        _puroro_root::enum3::Enum,
        self::_puroro::tags::Enum3<_puroro_root::enum3::Enum>,
    >,
    // Repeated, Variant(Enum3(Fqtn(".enum3.Enum")))
    enum_repeated: self::_puroro::internal::field_type::RepeatedNumericalField<
        _puroro_root::enum3::Enum,
        self::_puroro::tags::Enum3<_puroro_root::enum3::Enum>,
    >,

    _bitfield: self::_puroro::bitvec::BitArray<1>,
}

impl Msg {
    // Optional, Variant(Enum3(Fqtn(".enum3.Enum")))
    pub fn enum_optional(&self) -> _puroro_root::enum3::Enum {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::enum3::Enum,
            self::_puroro::tags::Enum3<_puroro_root::enum3::Enum>,
            0,
        > as NonRepeatedFieldType>::get_field(
            &self.enum_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn enum_optional_opt(&self) -> ::std::option::Option<_puroro_root::enum3::Enum> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::enum3::Enum,
            self::_puroro::tags::Enum3<_puroro_root::enum3::Enum>,
            0,
        > as NonRepeatedFieldType>::get_field_opt(&self.enum_optional, &self._bitfield)
    }
    pub fn has_enum_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::enum3::Enum,
            self::_puroro::tags::Enum3<_puroro_root::enum3::Enum>,
            0,
        > as NonRepeatedFieldType>::get_field_opt(&self.enum_optional, &self._bitfield)
        .is_some()
    }
    pub fn enum_optional_mut(&mut self) -> &mut _puroro_root::enum3::Enum {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::enum3::Enum,
            self::_puroro::tags::Enum3<_puroro_root::enum3::Enum>,
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
            _puroro_root::enum3::Enum,
            self::_puroro::tags::Enum3<_puroro_root::enum3::Enum>,
            0,
        > as NonRepeatedFieldType>::clear(&mut self.enum_optional, &mut self._bitfield)
    }
    // Singular, Variant(Enum3(Fqtn(".enum3.Enum")))
    pub fn enum_unlabeled(&self) -> _puroro_root::enum3::Enum {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            _puroro_root::enum3::Enum,
            self::_puroro::tags::Enum3<_puroro_root::enum3::Enum>,
        > as NonRepeatedFieldType>::get_field(
            &self.enum_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn enum_unlabeled_opt(&self) -> ::std::option::Option<_puroro_root::enum3::Enum> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            _puroro_root::enum3::Enum,
            self::_puroro::tags::Enum3<_puroro_root::enum3::Enum>,
        > as NonRepeatedFieldType>::get_field_opt(&self.enum_unlabeled, &self._bitfield)
    }
    pub fn has_enum_unlabeled(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            _puroro_root::enum3::Enum,
            self::_puroro::tags::Enum3<_puroro_root::enum3::Enum>,
        > as NonRepeatedFieldType>::get_field_opt(&self.enum_unlabeled, &self._bitfield)
        .is_some()
    }
    pub fn enum_unlabeled_mut(&mut self) -> &mut _puroro_root::enum3::Enum {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            _puroro_root::enum3::Enum,
            self::_puroro::tags::Enum3<_puroro_root::enum3::Enum>,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.enum_unlabeled,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_enum_unlabeled(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            _puroro_root::enum3::Enum,
            self::_puroro::tags::Enum3<_puroro_root::enum3::Enum>,
        > as NonRepeatedFieldType>::clear(&mut self.enum_unlabeled, &mut self._bitfield)
    }
    // Repeated, Variant(Enum3(Fqtn(".enum3.Enum")))
    pub fn enum_repeated(&self) -> &[_puroro_root::enum3::Enum] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            _puroro_root::enum3::Enum,
            self::_puroro::tags::Enum3<_puroro_root::enum3::Enum>,
        > as RepeatedFieldType>::get_field(&self.enum_repeated, &self._bitfield)
    }
    pub fn enum_repeated_mut(&mut self) -> &mut ::std::vec::Vec<_puroro_root::enum3::Enum> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            _puroro_root::enum3::Enum,
            self::_puroro::tags::Enum3<_puroro_root::enum3::Enum>,
        > as RepeatedFieldType>::mut_field(&mut self.enum_repeated, &mut self._bitfield)
    }
    pub fn clear_enum_repeated(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            _puroro_root::enum3::Enum,
            self::_puroro::tags::Enum3<_puroro_root::enum3::Enum>,
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
        use self::_puroro::internal::field_type::FieldType;
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        #[allow(unused)]
        use ::std::option::Option::Some;
        #[allow(unused)]
        use ::std::result::Result::Ok;
        while let Some((number, field_data)) =
            self::_puroro::internal::ser::FieldData::from_bytes_iter(iter.by_ref())?
        {
            match number {
                1 => <self::_puroro::internal::field_type::OptionalNumericalField<
                    _puroro_root::enum3::Enum,
                    self::_puroro::tags::Enum3<_puroro_root::enum3::Enum>,
                    0,
                > as FieldType>::deser_from_iter(
                    &mut self.enum_optional,
                    &mut self._bitfield,
                    field_data,
                )?,
                2 => <self::_puroro::internal::field_type::SingularNumericalField<
                    _puroro_root::enum3::Enum,
                    self::_puroro::tags::Enum3<_puroro_root::enum3::Enum>,
                > as FieldType>::deser_from_iter(
                    &mut self.enum_unlabeled,
                    &mut self._bitfield,
                    field_data,
                )?,
                3 => <self::_puroro::internal::field_type::RepeatedNumericalField<
                    _puroro_root::enum3::Enum,
                    self::_puroro::tags::Enum3<_puroro_root::enum3::Enum>,
                > as FieldType>::deser_from_iter(
                    &mut self.enum_repeated,
                    &mut self._bitfield,
                    field_data,
                )?,
                _ => todo!(), // Unknown field...
            }
        }
        Ok(())
    }

    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)] out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use self::_puroro::internal::field_type::FieldType;
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        #[allow(unused)]
        use ::std::result::Result::Ok;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::enum3::Enum,
            self::_puroro::tags::Enum3<_puroro_root::enum3::Enum>,
            0,
        > as FieldType>::ser_to_write(&self.enum_optional, &self._bitfield, 1, out)?;
        <self::_puroro::internal::field_type::SingularNumericalField<
            _puroro_root::enum3::Enum,
            self::_puroro::tags::Enum3<_puroro_root::enum3::Enum>,
        > as FieldType>::ser_to_write(&self.enum_unlabeled, &self._bitfield, 2, out)?;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            _puroro_root::enum3::Enum,
            self::_puroro::tags::Enum3<_puroro_root::enum3::Enum>,
        > as FieldType>::ser_to_write(&self.enum_repeated, &self._bitfield, 3, out)?;

        Ok(())
    }
}

impl ::std::clone::Clone for Msg {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        #[allow(unused)]
        use ::std::clone::Clone;
        Self {
            enum_optional: Clone::clone(&self.enum_optional),
            enum_unlabeled: Clone::clone(&self.enum_unlabeled),
            enum_repeated: Clone::clone(&self.enum_repeated),

            _bitfield: Clone::clone(&self._bitfield),
        }
    }
}

impl ::std::fmt::Debug for Msg {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct("Msg")
            .field("enum_optional", &self.enum_optional())
            .field("enum_unlabeled", &self.enum_unlabeled())
            .field("enum_repeated", &self.enum_repeated())
            .finish()
    }
}

impl ::std::cmp::PartialEq for Msg {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;

        true && self.enum_optional_opt() == rhs.enum_optional_opt()
            && self.enum_unlabeled_opt() == rhs.enum_unlabeled_opt()
            && self.enum_repeated() == rhs.enum_repeated()
    }
}

impl ::std::ops::Drop for Msg {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Enum {
    ValueZero,
    ValueSeven,
    ValueOne,
    ValueFourtyTwo,
    _None(i32),
}

impl ::std::default::Default for Enum {
    fn default() -> Self {
        Enum::ValueZero
    }
}

impl ::std::convert::From<i32> for Enum {
    fn from(x: i32) -> Self {
        match x {
            0 => self::Enum::ValueZero,
            7 => self::Enum::ValueSeven,
            1 => self::Enum::ValueOne,
            42 => self::Enum::ValueFourtyTwo,
            e => self::Enum::_None(e),
        }
    }
}

impl ::std::convert::From<Enum> for i32 {
    fn from(x: Enum) -> i32 {
        match x {
            self::Enum::ValueZero => 0,
            self::Enum::ValueSeven => 7,
            self::Enum::ValueOne => 1,
            self::Enum::ValueFourtyTwo => 42,
            self::Enum::_None(y) => y,
        }
    }
}
