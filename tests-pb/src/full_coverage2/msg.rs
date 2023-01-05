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
pub struct Submsg {
    fields: self::_root::full_coverage2::msg::_fields::SubmsgFields,
    _bitfield: self::_pinternal::BitArray<1usize>,
}
impl Submsg {
    pub fn i32_required(&self) -> i32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.i32_required,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn i32_required_opt(&self) -> ::std::option::Option::<i32> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.i32_required, &self._bitfield)
    }
    pub fn i32_required_mut(&mut self) -> &mut i32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.i32_required,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_i32_required(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.i32_required, &self._bitfield)
            .is_some()
    }
    pub fn clear_i32_required(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::clear(&mut self.i32_required, &mut self._bitfield)
    }
    pub fn i64_required(&self) -> i64 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i64,
            self::_pinternal::tags::Int64,
            1usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.i64_required,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn i64_required_opt(&self) -> ::std::option::Option::<i64> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i64,
            self::_pinternal::tags::Int64,
            1usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.i64_required, &self._bitfield)
    }
    pub fn i64_required_mut(&mut self) -> &mut i64 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i64,
            self::_pinternal::tags::Int64,
            1usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.i64_required,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_i64_required(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i64,
            self::_pinternal::tags::Int64,
            1usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.i64_required, &self._bitfield)
            .is_some()
    }
    pub fn clear_i64_required(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i64,
            self::_pinternal::tags::Int64,
            1usize,
        > as NonRepeatedFieldType>::clear(&mut self.i64_required, &mut self._bitfield)
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
        use self::_pinternal::ser::FieldData;
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            match number {
                1i32 => {
                    <self::_pinternal::OptionalNumericalField::<
                        i32,
                        self::_pinternal::tags::Int32,
                        0usize,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.i32_required,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                101i32 => {
                    <self::_pinternal::OptionalNumericalField::<
                        i64,
                        self::_pinternal::tags::Int64,
                        1usize,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.i64_required,
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
        use self::_pinternal::OneofUnion as _;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.i32_required,
            &self._bitfield,
            1i32,
            out,
        )?;
        <self::_pinternal::OptionalNumericalField::<
            i64,
            self::_pinternal::tags::Int64,
            1usize,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.i64_required,
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
            i32_required: <self::_pinternal::OptionalNumericalField::<
                i32,
                self::_pinternal::tags::Int32,
                0usize,
            > as ::std::clone::Clone>::clone(&self.i32_required),
            i64_required: <self::_pinternal::OptionalNumericalField::<
                i64,
                self::_pinternal::tags::Int64,
                1usize,
            > as ::std::clone::Clone>::clone(&self.i64_required),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
impl ::std::ops::Drop for Submsg {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for Submsg {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct(stringify!(Submsg))
            .field(stringify!(i32_required), &self.i32_required_opt())
            .field(stringify!(i64_required), &self.i64_required_opt())
            .finish()
    }
}
impl ::std::cmp::PartialEq for Submsg {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        true && self.i32_required_opt() == rhs.i32_required_opt()
            && self.i64_required_opt() == rhs.i64_required_opt()
    }
}
pub mod _fields {
    mod _root {
        #[allow(unused)]
        pub use super::super::_root::*;
    }
    mod _puroro {
        #[allow(unused)]
        pub use ::puroro::*;
    }
    mod _pinternal {
        #[allow(unused)]
        pub use ::puroro::internal::*;
    }
    pub struct SubmsgFields<TI32Required, TI64Required> {
        pub i32_required: TI32Required,
        pub i64_required: TI64Required,
    }
}
pub use self::_fields::*;
