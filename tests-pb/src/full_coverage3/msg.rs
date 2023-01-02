mod _puroro_root {
    #[allow(unused)]
    pub(crate) use super::super::_puroro_root::*;
}
mod _puroro {
    #[allow(unused)]
    pub(crate) use ::puroro::*;
}
#[derive(::std::default::Default)]
pub struct Submsg {
    i32_unlabeled: self::_puroro::internal::field_type::SingularNumericalField::<
        i32,
        self::_puroro::internal::tags::Int32,
    >,
    i32_optional: self::_puroro::internal::field_type::SingularNumericalField::<
        i32,
        self::_puroro::internal::tags::Int32,
    >,
    i64_unlabeled: self::_puroro::internal::field_type::SingularNumericalField::<
        i64,
        self::_puroro::internal::tags::Int64,
    >,
    _bitfield: self::_puroro::internal::bitvec::BitArray<0usize>,
}
impl Submsg {
    pub fn i32_unlabeled(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            i32,
            self::_puroro::internal::tags::Int32,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.i32_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn i32_unlabeled_opt(&self) -> ::std::option::Option::<i32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            i32,
            self::_puroro::internal::tags::Int32,
        > as NonRepeatedFieldType>::get_field_opt(&self.i32_unlabeled, &self._bitfield)
    }
    pub fn i32_unlabeled_mut(&mut self) -> &mut i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            i32,
            self::_puroro::internal::tags::Int32,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.i32_unlabeled,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_i32_unlabeled(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            i32,
            self::_puroro::internal::tags::Int32,
        > as NonRepeatedFieldType>::get_field_opt(&self.i32_unlabeled, &self._bitfield)
            .is_some()
    }
    pub fn clear_i32_unlabeled(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            i32,
            self::_puroro::internal::tags::Int32,
        > as NonRepeatedFieldType>::clear(&mut self.i32_unlabeled, &mut self._bitfield)
    }
    pub fn i32_optional(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            i32,
            self::_puroro::internal::tags::Int32,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.i32_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn i32_optional_opt(&self) -> ::std::option::Option::<i32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            i32,
            self::_puroro::internal::tags::Int32,
        > as NonRepeatedFieldType>::get_field_opt(&self.i32_optional, &self._bitfield)
    }
    pub fn i32_optional_mut(&mut self) -> &mut i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            i32,
            self::_puroro::internal::tags::Int32,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.i32_optional,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_i32_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            i32,
            self::_puroro::internal::tags::Int32,
        > as NonRepeatedFieldType>::get_field_opt(&self.i32_optional, &self._bitfield)
            .is_some()
    }
    pub fn clear_i32_optional(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            i32,
            self::_puroro::internal::tags::Int32,
        > as NonRepeatedFieldType>::clear(&mut self.i32_optional, &mut self._bitfield)
    }
    pub fn i64_unlabeled(&self) -> i64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            i64,
            self::_puroro::internal::tags::Int64,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.i64_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn i64_unlabeled_opt(&self) -> ::std::option::Option::<i64> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            i64,
            self::_puroro::internal::tags::Int64,
        > as NonRepeatedFieldType>::get_field_opt(&self.i64_unlabeled, &self._bitfield)
    }
    pub fn i64_unlabeled_mut(&mut self) -> &mut i64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            i64,
            self::_puroro::internal::tags::Int64,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.i64_unlabeled,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_i64_unlabeled(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            i64,
            self::_puroro::internal::tags::Int64,
        > as NonRepeatedFieldType>::get_field_opt(&self.i64_unlabeled, &self._bitfield)
            .is_some()
    }
    pub fn clear_i64_unlabeled(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            i64,
            self::_puroro::internal::tags::Int64,
        > as NonRepeatedFieldType>::clear(&mut self.i64_unlabeled, &mut self._bitfield)
    }
}
impl self::_puroro::Message for Submsg {
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
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion as _;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            match number {
                1i32 => {
                    <self::_puroro::internal::field_type::SingularNumericalField::<
                        i32,
                        self::_puroro::internal::tags::Int32,
                    > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.i32_unlabeled,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                2i32 => {
                    <self::_puroro::internal::field_type::SingularNumericalField::<
                        i32,
                        self::_puroro::internal::tags::Int32,
                    > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.i32_optional,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                101i32 => {
                    <self::_puroro::internal::field_type::SingularNumericalField::<
                        i64,
                        self::_puroro::internal::tags::Int64,
                    > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.i64_unlabeled,
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
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion as _;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            i32,
            self::_puroro::internal::tags::Int32,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.i32_unlabeled,
            &self._bitfield,
            1i32,
            out,
        )?;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            i32,
            self::_puroro::internal::tags::Int32,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.i32_optional,
            &self._bitfield,
            2i32,
            out,
        )?;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            i64,
            self::_puroro::internal::tags::Int64,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.i64_unlabeled,
            &self._bitfield,
            101i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for Submsg {
    fn clone(&self) -> Self {
        Self {
            i32_unlabeled: <self::_puroro::internal::field_type::SingularNumericalField::<
                i32,
                self::_puroro::internal::tags::Int32,
            > as ::std::clone::Clone>::clone(&self.i32_unlabeled),
            i32_optional: <self::_puroro::internal::field_type::SingularNumericalField::<
                i32,
                self::_puroro::internal::tags::Int32,
            > as ::std::clone::Clone>::clone(&self.i32_optional),
            i64_unlabeled: <self::_puroro::internal::field_type::SingularNumericalField::<
                i64,
                self::_puroro::internal::tags::Int64,
            > as ::std::clone::Clone>::clone(&self.i64_unlabeled),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
impl ::std::ops::Drop for Submsg {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for Submsg {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct(stringify!(Submsg))
            .field(stringify!(i32_unlabeled), &self.i32_unlabeled_opt())
            .field(stringify!(i32_optional), &self.i32_optional_opt())
            .field(stringify!(i64_unlabeled), &self.i64_unlabeled_opt())
            .finish()
    }
}
impl ::std::cmp::PartialEq for Submsg {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion as _;
        true && self.i32_unlabeled_opt() == rhs.i32_unlabeled_opt()
            && self.i32_optional_opt() == rhs.i32_optional_opt()
            && self.i64_unlabeled_opt() == rhs.i64_unlabeled_opt()
    }
}
