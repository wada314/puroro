mod _root {
    #[allow(unused)]
    pub(crate) use super::super::_root::*;
}
mod _puroro {
    #[allow(unused)]
    pub(crate) use ::puroro::*;
}
mod _pinternal {
    #[allow(unused)]
    pub(crate) use ::puroro::internal::*;
}
#[derive(::std::default::Default)]
pub struct EnumReservedRange {
    start: self::_pinternal::field_type::OptionalNumericalField::<
        i32,
        self::_pinternal::tags::Int32,
        0usize,
    >,
    end: self::_pinternal::field_type::OptionalNumericalField::<
        i32,
        self::_pinternal::tags::Int32,
        1usize,
    >,
    _bitfield: self::_pinternal::bitvec::BitArray<1usize>,
}
impl EnumReservedRange {
    pub fn start(&self) -> i32 {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.start,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn start_opt(&self) -> ::std::option::Option::<i32> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.start, &self._bitfield)
    }
    pub fn start_mut(&mut self) -> &mut i32 {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.start,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_start(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.start, &self._bitfield)
            .is_some()
    }
    pub fn clear_start(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::clear(&mut self.start, &mut self._bitfield)
    }
    pub fn end(&self) -> i32 {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            1usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.end,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn end_opt(&self) -> ::std::option::Option::<i32> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            1usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.end, &self._bitfield)
    }
    pub fn end_mut(&mut self) -> &mut i32 {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            1usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.end,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_end(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            1usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.end, &self._bitfield)
            .is_some()
    }
    pub fn clear_end(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            1usize,
        > as NonRepeatedFieldType>::clear(&mut self.end, &mut self._bitfield)
    }
}
impl self::_puroro::Message for EnumReservedRange {
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
        use self::_pinternal::ser::FieldData;
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            match number {
                1i32 => {
                    <self::_pinternal::field_type::OptionalNumericalField::<
                        i32,
                        self::_pinternal::tags::Int32,
                        0usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.start,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                2i32 => {
                    <self::_pinternal::field_type::OptionalNumericalField::<
                        i32,
                        self::_pinternal::tags::Int32,
                        1usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.end,
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
        use self::_pinternal::oneof_type::OneofUnion as _;
        <self::_pinternal::field_type::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.start,
            &self._bitfield,
            1i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            1usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.end,
            &self._bitfield,
            2i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for EnumReservedRange {
    fn clone(&self) -> Self {
        Self {
            start: <self::_pinternal::field_type::OptionalNumericalField::<
                i32,
                self::_pinternal::tags::Int32,
                0usize,
            > as ::std::clone::Clone>::clone(&self.start),
            end: <self::_pinternal::field_type::OptionalNumericalField::<
                i32,
                self::_pinternal::tags::Int32,
                1usize,
            > as ::std::clone::Clone>::clone(&self.end),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
impl ::std::ops::Drop for EnumReservedRange {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for EnumReservedRange {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct(stringify!(EnumReservedRange))
            .field(stringify!(start), &self.start_opt())
            .field(stringify!(end), &self.end_opt())
            .finish()
    }
}
impl ::std::cmp::PartialEq for EnumReservedRange {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
        true && self.start_opt() == rhs.start_opt() && self.end_opt() == rhs.end_opt()
    }
}
