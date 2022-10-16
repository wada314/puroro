// A generated source code by puroro library
// package .nested.Msg

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub mod _puroro {
    pub use ::puroro::*;
}

#[derive(Default)]
pub struct Submsg {
    // Singular, Variant(Int32)
    item_inner: self::_puroro::internal::field_type::SingularNumericalField<
        i32,
        self::_puroro::tags::Int32,
    >,

    _bitfield: self::_puroro::bitvec::BitArray<0>,
}

impl Submsg {
    // Singular, Variant(Int32)
    pub fn item_inner(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<i32, self::_puroro::tags::Int32> as NonRepeatedFieldType>::get_field(
            &self.item_inner, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn item_inner_opt(&self) -> ::std::option::Option<i32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<i32, self::_puroro::tags::Int32> as NonRepeatedFieldType>::get_field_opt(
            &self.item_inner, &self._bitfield,
        )
    }
    pub fn has_item_inner(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<i32, self::_puroro::tags::Int32> as NonRepeatedFieldType>::get_field_opt(
            &self.item_inner, &self._bitfield,
        ).is_some()
    }
    pub fn item_inner_mut(&mut self) -> &mut i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<i32, self::_puroro::tags::Int32> as NonRepeatedFieldType>::mut_field(
            &mut self.item_inner, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_item_inner(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<i32, self::_puroro::tags::Int32> as NonRepeatedFieldType>::clear(
            &mut self.item_inner, &mut self._bitfield,
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
                    &mut self.item_inner,
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
        > as FieldType>::ser_to_write(&self.item_inner, &self._bitfield, 1, out)?;

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
            item_inner: Clone::clone(&self.item_inner),

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
            .field("item_inner", &self.item_inner())
            .finish()
    }
}

impl ::std::cmp::PartialEq for Submsg {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;

        true && self.item_inner_opt() == rhs.item_inner_opt()
    }
}

impl ::std::ops::Drop for Submsg {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Enum {
    MyValue,
    _None(i32),
}

impl ::std::default::Default for Enum {
    fn default() -> Self {
        Enum::MyValue
    }
}

impl ::std::convert::From<i32> for Enum {
    fn from(x: i32) -> Self {
        match x {
            0 => self::Enum::MyValue,
            e => self::Enum::_None(e),
        }
    }
}

impl ::std::convert::From<Enum> for i32 {
    fn from(x: Enum) -> i32 {
        match x {
            self::Enum::MyValue => 0,
            self::Enum::_None(y) => y,
        }
    }
}
