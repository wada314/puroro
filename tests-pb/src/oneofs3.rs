// A generated source code by puroro library
// package oneofs3
pub mod msg;

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub mod _puroro {
    pub use ::puroro::*;
}

#[derive(Default)]
pub struct Msg {
    // oneof GroupOne
    group_one: _puroro_root::oneofs3::msg::GroupOne,
    // oneof GroupTwo
    group_two: _puroro_root::oneofs3::msg::GroupTwo,
    // oneof GroupThree
    group_three: _puroro_root::oneofs3::msg::GroupThree,

    _bitfield: self::_puroro::bitvec::BitArray<1>,
}

impl Msg {}

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
                _ => todo!(),
            }
        }
        Ok(())
    }
}

impl ::std::clone::Clone for Msg {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use ::std::clone::Clone;
        Self {
            group_one: self.group_one._clone(&self._bitfield),
            group_two: self.group_two._clone(&self._bitfield),
            group_three: self.group_three._clone(&self._bitfield),

            _bitfield: Clone::clone(&self._bitfield),
        }
    }
}

impl ::std::fmt::Debug for Msg {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct("Msg").finish()
    }
}

#[derive(Default)]
pub struct Submsg {
    // Singular, Variant(Int32)
    i32_unlabeled: self::_puroro::internal::field_type::SingularNumericalField<
        i32,
        self::_puroro::tags::Int32,
    >,

    _bitfield: self::_puroro::bitvec::BitArray<0>,
}

impl Submsg {
    // Singular, Variant(Int32)
    pub fn i32_unlabeled(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<i32, self::_puroro::tags::Int32> as NonRepeatedFieldType>::get_field(
            &self.i32_unlabeled, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn i32_unlabeled_opt(&self) -> ::std::option::Option<i32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<i32, self::_puroro::tags::Int32> as NonRepeatedFieldType>::get_field_opt(
            &self.i32_unlabeled, &self._bitfield,
        )
    }
    pub fn has_i32_unlabeled(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<i32, self::_puroro::tags::Int32> as NonRepeatedFieldType>::get_field_opt(
            &self.i32_unlabeled, &self._bitfield,
        ).is_some()
    }
    pub fn i32_unlabeled_mut(&mut self) -> &mut i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<i32, self::_puroro::tags::Int32> as NonRepeatedFieldType>::mut_field(
            &mut self.i32_unlabeled, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_i32_unlabeled(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<i32, self::_puroro::tags::Int32> as NonRepeatedFieldType>::clear(
            &mut self.i32_unlabeled, &mut self._bitfield,
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
        use ::std::option::Option::Some;
        #[allow(unused)]
        use ::std::result::Result::Ok;
        while let Some((number, _field_data)) =
            self::_puroro::internal::ser::FieldData::from_bytes_iter(iter.by_ref())?
        {
            match number {
                1 => <self::_puroro::internal::field_type::SingularNumericalField<
                    i32,
                    self::_puroro::tags::Int32,
                > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                    &mut self.i32_unlabeled,
                    &mut self._bitfield,
                    _field_data,
                )?,
                _ => todo!(),
            }
        }
        Ok(())
    }
}

impl ::std::clone::Clone for Submsg {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use ::std::clone::Clone;
        Self {
            i32_unlabeled: Clone::clone(&self.i32_unlabeled),

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
            .finish()
    }
}
