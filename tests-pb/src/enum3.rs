pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}
pub mod _puroro {
    pub use ::puroro::*;
}
#[derive(::std::default::Default)]
pub struct Msg {
    enum_optional: self::_puroro::internal::field_type::OptionalNumericalField::<
        self::_puroro_root::enum3::Enum,
        self::_puroro::tags::Enum3::<self::_puroro_root::enum3::Enum>,
        0usize,
    >,
    enum_unlabeled: self::_puroro::internal::field_type::SingularNumericalField::<
        self::_puroro_root::enum3::Enum,
        self::_puroro::tags::Enum3::<self::_puroro_root::enum3::Enum>,
    >,
    enum_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        self::_puroro_root::enum3::Enum,
        self::_puroro::tags::Enum3::<self::_puroro_root::enum3::Enum>,
    >,
    _bitfield: self::_puroro::bitvec::BitArray<1usize>,
}
impl Msg {
    pub fn enum_optional(&self) -> self::_puroro_root::enum3::Enum {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            self::_puroro_root::enum3::Enum,
            self::_puroro::tags::Enum3::<self::_puroro_root::enum3::Enum>,
            0usize,
        > as NonRepeatedFieldType>::get_field(
            &self.enum_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn enum_optional_opt(
        &self,
    ) -> ::std::option::Option::<self::_puroro_root::enum3::Enum> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            self::_puroro_root::enum3::Enum,
            self::_puroro::tags::Enum3::<self::_puroro_root::enum3::Enum>,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.enum_optional, &self._bitfield)
    }
    pub fn enum_optional_mut(&mut self) -> &mut self::_puroro_root::enum3::Enum {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            self::_puroro_root::enum3::Enum,
            self::_puroro::tags::Enum3::<self::_puroro_root::enum3::Enum>,
            0usize,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.enum_optional,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_enum_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            self::_puroro_root::enum3::Enum,
            self::_puroro::tags::Enum3::<self::_puroro_root::enum3::Enum>,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.enum_optional, &self._bitfield)
            .is_some()
    }
    pub fn clear_enum_optional(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            self::_puroro_root::enum3::Enum,
            self::_puroro::tags::Enum3::<self::_puroro_root::enum3::Enum>,
            0usize,
        > as NonRepeatedFieldType>::clear(&mut self.enum_optional, &mut self._bitfield)
    }
    pub fn enum_unlabeled(&self) -> self::_puroro_root::enum3::Enum {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            self::_puroro_root::enum3::Enum,
            self::_puroro::tags::Enum3::<self::_puroro_root::enum3::Enum>,
        > as NonRepeatedFieldType>::get_field(
            &self.enum_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn enum_unlabeled_opt(
        &self,
    ) -> ::std::option::Option::<self::_puroro_root::enum3::Enum> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            self::_puroro_root::enum3::Enum,
            self::_puroro::tags::Enum3::<self::_puroro_root::enum3::Enum>,
        > as NonRepeatedFieldType>::get_field_opt(&self.enum_unlabeled, &self._bitfield)
    }
    pub fn enum_unlabeled_mut(&mut self) -> &mut self::_puroro_root::enum3::Enum {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            self::_puroro_root::enum3::Enum,
            self::_puroro::tags::Enum3::<self::_puroro_root::enum3::Enum>,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.enum_unlabeled,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_enum_unlabeled(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            self::_puroro_root::enum3::Enum,
            self::_puroro::tags::Enum3::<self::_puroro_root::enum3::Enum>,
        > as NonRepeatedFieldType>::get_field_opt(&self.enum_unlabeled, &self._bitfield)
            .is_some()
    }
    pub fn clear_enum_unlabeled(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            self::_puroro_root::enum3::Enum,
            self::_puroro::tags::Enum3::<self::_puroro_root::enum3::Enum>,
        > as NonRepeatedFieldType>::clear(&mut self.enum_unlabeled, &mut self._bitfield)
    }
    pub fn enum_repeated(&self) -> &[self::_puroro_root::enum3::Enum] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            self::_puroro_root::enum3::Enum,
            self::_puroro::tags::Enum3::<self::_puroro_root::enum3::Enum>,
        > as RepeatedFieldType>::get_field(&self.enum_repeated, &self._bitfield)
    }
    pub fn enum_repeated_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_puroro_root::enum3::Enum> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            self::_puroro_root::enum3::Enum,
            self::_puroro::tags::Enum3::<self::_puroro_root::enum3::Enum>,
        > as RepeatedFieldType>::mut_field(&mut self.enum_repeated, &mut self._bitfield)
    }
    pub fn clear_enum_repeated(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            self::_puroro_root::enum3::Enum,
            self::_puroro::tags::Enum3::<self::_puroro_root::enum3::Enum>,
        > as RepeatedFieldType>::clear(&mut self.enum_repeated, &mut self._bitfield)
    }
}
impl self::_puroro::Message for Msg {
    fn from_bytes_iter<I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>>(
        iter: I,
    ) -> self::_puroro::Result<Self> {
        let mut msg = <Self as ::std::default::Default>::default();
        msg.merge_from_bytes_iter(iter)?;
        ::std::result::Result::Ok(msg)
    }
    fn merge_from_bytes_iter<I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>>(
        &mut self,
        mut iter: I,
    ) -> self::_puroro::Result<()> {
        use self::_puroro::internal::ser::FieldData;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            match number {
                1i32 => {
                    <self::_puroro::internal::field_type::OptionalNumericalField::<
                        self::_puroro_root::enum3::Enum,
                        self::_puroro::tags::Enum3::<self::_puroro_root::enum3::Enum>,
                        0usize,
                    > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.enum_optional,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                2i32 => {
                    <self::_puroro::internal::field_type::SingularNumericalField::<
                        self::_puroro_root::enum3::Enum,
                        self::_puroro::tags::Enum3::<self::_puroro_root::enum3::Enum>,
                    > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.enum_unlabeled,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                3i32 => {
                    <self::_puroro::internal::field_type::RepeatedNumericalField::<
                        self::_puroro_root::enum3::Enum,
                        self::_puroro::tags::Enum3::<self::_puroro_root::enum3::Enum>,
                    > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.enum_repeated,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                _ => todo!(),
            }
        }
        ::std::result::Result::Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            self::_puroro_root::enum3::Enum,
            self::_puroro::tags::Enum3::<self::_puroro_root::enum3::Enum>,
            0usize,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.enum_optional,
            &self._bitfield,
            1i32,
            out,
        )?;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            self::_puroro_root::enum3::Enum,
            self::_puroro::tags::Enum3::<self::_puroro_root::enum3::Enum>,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.enum_unlabeled,
            &self._bitfield,
            2i32,
            out,
        )?;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            self::_puroro_root::enum3::Enum,
            self::_puroro::tags::Enum3::<self::_puroro_root::enum3::Enum>,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.enum_repeated,
            &self._bitfield,
            3i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for Msg {
    fn clone(&self) -> Self {
        Self {
            enum_optional: <self::_puroro::internal::field_type::OptionalNumericalField::<
                self::_puroro_root::enum3::Enum,
                self::_puroro::tags::Enum3::<self::_puroro_root::enum3::Enum>,
                0usize,
            > as ::std::clone::Clone>::clone(&self.enum_optional),
            enum_unlabeled: <self::_puroro::internal::field_type::SingularNumericalField::<
                self::_puroro_root::enum3::Enum,
                self::_puroro::tags::Enum3::<self::_puroro_root::enum3::Enum>,
            > as ::std::clone::Clone>::clone(&self.enum_unlabeled),
            enum_repeated: <self::_puroro::internal::field_type::RepeatedNumericalField::<
                self::_puroro_root::enum3::Enum,
                self::_puroro::tags::Enum3::<self::_puroro_root::enum3::Enum>,
            > as ::std::clone::Clone>::clone(&self.enum_repeated),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
#[derive(
    ::std::clone::Clone,
    ::std::marker::Copy,
    ::std::cmp::PartialEq,
    ::std::cmp::Eq,
    ::std::cmp::PartialOrd,
    ::std::cmp::Ord,
    ::std::hash::Hash,
)]
pub enum Enum {
    ValueZero,
    ValueSeven,
    ValueOne,
    ValueFourtyTwo,
    _None(i32),
}
impl ::std::default::Default for Enum {
    fn default() -> Self {
        Self::ValueZero
    }
}
impl ::std::convert::From::<Enum> for i32 {
    fn from(val: Enum) -> i32 {
        match val {
            Enum::ValueZero => 0i32,
            Enum::ValueSeven => 7i32,
            Enum::ValueOne => 1i32,
            Enum::ValueFourtyTwo => 42i32,
            self::Enum::_None(i) => i,
        }
    }
}
impl ::std::convert::From::<i32> for Enum {
    fn from(val: i32) -> Self {
        match val {
            0i32 => self::Enum::ValueZero,
            7i32 => self::Enum::ValueSeven,
            1i32 => self::Enum::ValueOne,
            42i32 => self::Enum::ValueFourtyTwo,
            _ => Enum::_None(val),
        }
    }
}
