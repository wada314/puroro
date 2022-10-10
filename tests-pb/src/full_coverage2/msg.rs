// A generated source code by puroro library
// package .full_coverage2.Msg

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub mod _puroro {
    pub use ::puroro::*;
}

#[derive(Default)]
pub struct Submsg {
    // Optional, Variant(Int32)
    i32_required: self::_puroro::internal::field_type::OptionalNumericalField<
        i32,
        self::_puroro::tags::Int32,
        0,
    >,
    // Optional, Variant(Int64)
    i64_required: self::_puroro::internal::field_type::OptionalNumericalField<
        i64,
        self::_puroro::tags::Int64,
        1,
    >,

    _bitfield: self::_puroro::bitvec::BitArray<1>,
}

impl Submsg {
    // Optional, Variant(Int32)
    pub fn i32_required(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as NonRepeatedFieldType>::get_field(
            &self.i32_required,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn i32_required_opt(&self) -> ::std::option::Option<i32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as NonRepeatedFieldType>::get_field_opt(&self.i32_required, &self._bitfield)
    }
    pub fn has_i32_required(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as NonRepeatedFieldType>::get_field_opt(&self.i32_required, &self._bitfield)
        .is_some()
    }
    pub fn i32_required_mut(&mut self) -> &mut i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.i32_required,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_i32_required(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as NonRepeatedFieldType>::clear(&mut self.i32_required, &mut self._bitfield)
    }
    // Optional, Variant(Int64)
    pub fn i64_required(&self) -> i64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::Int64,
            1,
        > as NonRepeatedFieldType>::get_field(
            &self.i64_required,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn i64_required_opt(&self) -> ::std::option::Option<i64> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::Int64,
            1,
        > as NonRepeatedFieldType>::get_field_opt(&self.i64_required, &self._bitfield)
    }
    pub fn has_i64_required(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::Int64,
            1,
        > as NonRepeatedFieldType>::get_field_opt(&self.i64_required, &self._bitfield)
        .is_some()
    }
    pub fn i64_required_mut(&mut self) -> &mut i64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::Int64,
            1,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.i64_required,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_i64_required(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::Int64,
            1,
        > as NonRepeatedFieldType>::clear(&mut self.i64_required, &mut self._bitfield)
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
        use ::std::option::Option::Some;
        #[allow(unused)]
        use ::std::result::Result::Ok;
        while let Some((number, _field_data)) =
            self::_puroro::internal::ser::FieldData::from_bytes_iter(iter.by_ref())?
        {
            match number {
                1 => <self::_puroro::internal::field_type::OptionalNumericalField<
                    i32,
                    self::_puroro::tags::Int32,
                    0,
                > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                    &mut self.i32_required,
                    &mut self._bitfield,
                    _field_data,
                )?,
                101 => <self::_puroro::internal::field_type::OptionalNumericalField<
                    i64,
                    self::_puroro::tags::Int64,
                    1,
                > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                    &mut self.i64_required,
                    &mut self._bitfield,
                    _field_data,
                )?,
                _ => todo!(),
            }
        }
        Ok(())
    }

    fn to_bytes<W: ::std::io::Write>(&self, out: &mut W) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use ::std::result::Result::Ok;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.i32_required,
            &self._bitfield,
            1,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::Int64,
            1,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.i64_required,
            &self._bitfield,
            101,
            out,
        )?;
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
            i32_required: Clone::clone(&self.i32_required),
            i64_required: Clone::clone(&self.i64_required),

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
            .field("i32_required", &self.i32_required())
            .field("i64_required", &self.i64_required())
            .finish()
    }
}

impl ::std::cmp::PartialEq for Submsg {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;

        true && self.i32_required_opt() == rhs.i32_required_opt()
            && self.i64_required_opt() == rhs.i64_required_opt()
    }
}

impl ::std::ops::Drop for Submsg {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
    }
}
pub mod submsg;
