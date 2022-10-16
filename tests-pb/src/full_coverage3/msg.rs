// A generated source code by puroro library
// package .full_coverage3.Msg

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub mod _puroro {
    pub use ::puroro::*;
}

#[derive(Default)]
pub struct Submsg {
    // Singular, Variant(Int32)
    i32_unlabeled: self::_puroro::internal::field_type::SingularNumericalField<
        i32,
        self::_puroro::tags::Int32,
    >,
    // Singular, Variant(Int32)
    i32_optional: self::_puroro::internal::field_type::SingularNumericalField<
        i32,
        self::_puroro::tags::Int32,
    >,
    // Singular, Variant(Int64)
    i64_unlabeled: self::_puroro::internal::field_type::SingularNumericalField<
        i64,
        self::_puroro::tags::Int64,
    >,

    _bitfield: self::_puroro::bitvec::BitArray<0>,
}

impl Submsg {
    // Singular, Variant(Int32)
    pub fn i32_unlabeled(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<i32, self::_puroro::tags::Int32> as NonRepeatedFieldType>::get_field(
            &self.i32_unlabeled, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn i32_unlabeled_opt(&self) -> ::std::option::Option<i32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<i32, self::_puroro::tags::Int32> as NonRepeatedFieldType>::get_field_opt(
            &self.i32_unlabeled, &self._bitfield,
        )
    }
    pub fn has_i32_unlabeled(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<i32, self::_puroro::tags::Int32> as NonRepeatedFieldType>::get_field_opt(
            &self.i32_unlabeled, &self._bitfield,
        ).is_some()
    }
    pub fn i32_unlabeled_mut(&mut self) -> &mut i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<i32, self::_puroro::tags::Int32> as NonRepeatedFieldType>::mut_field(
            &mut self.i32_unlabeled, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_i32_unlabeled(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<i32, self::_puroro::tags::Int32> as NonRepeatedFieldType>::clear(
            &mut self.i32_unlabeled, &mut self._bitfield,
        )
    }
    // Singular, Variant(Int32)
    pub fn i32_optional(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<i32, self::_puroro::tags::Int32> as NonRepeatedFieldType>::get_field(
            &self.i32_optional, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn i32_optional_opt(&self) -> ::std::option::Option<i32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<i32, self::_puroro::tags::Int32> as NonRepeatedFieldType>::get_field_opt(
            &self.i32_optional, &self._bitfield,
        )
    }
    pub fn has_i32_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<i32, self::_puroro::tags::Int32> as NonRepeatedFieldType>::get_field_opt(
            &self.i32_optional, &self._bitfield,
        ).is_some()
    }
    pub fn i32_optional_mut(&mut self) -> &mut i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<i32, self::_puroro::tags::Int32> as NonRepeatedFieldType>::mut_field(
            &mut self.i32_optional, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_i32_optional(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<i32, self::_puroro::tags::Int32> as NonRepeatedFieldType>::clear(
            &mut self.i32_optional, &mut self._bitfield,
        )
    }
    // Singular, Variant(Int64)
    pub fn i64_unlabeled(&self) -> i64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<i64, self::_puroro::tags::Int64> as NonRepeatedFieldType>::get_field(
            &self.i64_unlabeled, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn i64_unlabeled_opt(&self) -> ::std::option::Option<i64> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<i64, self::_puroro::tags::Int64> as NonRepeatedFieldType>::get_field_opt(
            &self.i64_unlabeled, &self._bitfield,
        )
    }
    pub fn has_i64_unlabeled(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<i64, self::_puroro::tags::Int64> as NonRepeatedFieldType>::get_field_opt(
            &self.i64_unlabeled, &self._bitfield,
        ).is_some()
    }
    pub fn i64_unlabeled_mut(&mut self) -> &mut i64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<i64, self::_puroro::tags::Int64> as NonRepeatedFieldType>::mut_field(
            &mut self.i64_unlabeled, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_i64_unlabeled(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<i64, self::_puroro::tags::Int64> as NonRepeatedFieldType>::clear(
            &mut self.i64_unlabeled, &mut self._bitfield,
        )
    }
}

impl self::_puroro::Message for Submsg {
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
                1 => <self::_puroro::internal::field_type::SingularNumericalField<
                    i32,
                    self::_puroro::tags::Int32,
                > as FieldType>::deser_from_iter(
                    &mut self.i32_unlabeled,
                    &mut self._bitfield,
                    field_data,
                )?,
                2 => <self::_puroro::internal::field_type::SingularNumericalField<
                    i32,
                    self::_puroro::tags::Int32,
                > as FieldType>::deser_from_iter(
                    &mut self.i32_optional,
                    &mut self._bitfield,
                    field_data,
                )?,
                101 => <self::_puroro::internal::field_type::SingularNumericalField<
                    i64,
                    self::_puroro::tags::Int64,
                > as FieldType>::deser_from_iter(
                    &mut self.i64_unlabeled,
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
        <self::_puroro::internal::field_type::SingularNumericalField<
            i32,
            self::_puroro::tags::Int32,
        > as FieldType>::ser_to_write(&self.i32_unlabeled, &self._bitfield, 1, out)?;
        <self::_puroro::internal::field_type::SingularNumericalField<
            i32,
            self::_puroro::tags::Int32,
        > as FieldType>::ser_to_write(&self.i32_optional, &self._bitfield, 2, out)?;
        <self::_puroro::internal::field_type::SingularNumericalField<
            i64,
            self::_puroro::tags::Int64,
        > as FieldType>::ser_to_write(&self.i64_unlabeled, &self._bitfield, 101, out)?;

        Ok(())
    }
}

impl ::std::clone::Clone for Submsg {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        #[allow(unused)]
        use ::std::clone::Clone;
        Self {
            i32_unlabeled: Clone::clone(&self.i32_unlabeled),
            i32_optional: Clone::clone(&self.i32_optional),
            i64_unlabeled: Clone::clone(&self.i64_unlabeled),

            _bitfield: Clone::clone(&self._bitfield),
        }
    }
}

impl ::std::fmt::Debug for Submsg {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct("Submsg")
            .field("i32_unlabeled", &self.i32_unlabeled())
            .field("i32_optional", &self.i32_optional())
            .field("i64_unlabeled", &self.i64_unlabeled())
            .finish()
    }
}

impl ::std::cmp::PartialEq for Submsg {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;

        true && self.i32_unlabeled_opt() == rhs.i32_unlabeled_opt()
            && self.i32_optional_opt() == rhs.i32_optional_opt()
            && self.i64_unlabeled_opt() == rhs.i64_unlabeled_opt()
    }
}

impl ::std::ops::Drop for Submsg {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
    }
}
