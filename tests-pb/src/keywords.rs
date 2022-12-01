pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}
pub mod _puroro {
    pub use ::puroro::*;
}
#[derive(::std::default::Default)]
pub struct Msg {
    r#type: self::_puroro::internal::field_type::OptionalNumericalField::<
        i32,
        self::_puroro::tags::Int32,
        0usize,
    >,
    _bitfield: self::_puroro::bitvec::BitArray<1usize>,
}
impl Msg {
    pub fn r#type(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::get_field(
            &self.r#type,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn type_opt(&self) -> ::std::option::Option::<i32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.r#type, &self._bitfield)
    }
    pub fn type_mut(&mut self) -> &mut i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.r#type,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_type(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.r#type, &self._bitfield)
            .is_some()
    }
    pub fn clear_type(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::clear(&mut self.r#type, &mut self._bitfield)
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
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion as _;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            match number {
                1i32 => {
                    <self::_puroro::internal::field_type::OptionalNumericalField::<
                        i32,
                        self::_puroro::tags::Int32,
                        0usize,
                    > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.r#type,
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
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            0usize,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.r#type,
            &self._bitfield,
            1i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for Msg {
    fn clone(&self) -> Self {
        Self {
            r#type: <self::_puroro::internal::field_type::OptionalNumericalField::<
                i32,
                self::_puroro::tags::Int32,
                0usize,
            > as ::std::clone::Clone>::clone(&self.r#type),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
impl ::std::ops::Drop for Msg {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for Msg {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct(stringify!(Msg))
            .field(stringify!(r#type), &self.r#type())
            .finish()
    }
}
impl ::std::cmp::PartialEq for Msg {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion as _;
        true && self.type_opt() == rhs.type_opt()
    }
}
#[derive(::std::default::Default)]
pub struct _Self {
    r#type: self::_puroro::internal::field_type::OptionalNumericalField::<
        i32,
        self::_puroro::tags::Int32,
        0usize,
    >,
    _bitfield: self::_puroro::bitvec::BitArray<1usize>,
}
impl _Self {
    pub fn r#type(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::get_field(
            &self.r#type,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn type_opt(&self) -> ::std::option::Option::<i32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.r#type, &self._bitfield)
    }
    pub fn type_mut(&mut self) -> &mut i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.r#type,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_type(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.r#type, &self._bitfield)
            .is_some()
    }
    pub fn clear_type(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::clear(&mut self.r#type, &mut self._bitfield)
    }
}
impl self::_puroro::Message for _Self {
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
                    <self::_puroro::internal::field_type::OptionalNumericalField::<
                        i32,
                        self::_puroro::tags::Int32,
                        0usize,
                    > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.r#type,
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
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            0usize,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.r#type,
            &self._bitfield,
            1i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for _Self {
    fn clone(&self) -> Self {
        Self {
            r#type: <self::_puroro::internal::field_type::OptionalNumericalField::<
                i32,
                self::_puroro::tags::Int32,
                0usize,
            > as ::std::clone::Clone>::clone(&self.r#type),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
impl ::std::ops::Drop for _Self {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for _Self {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct(stringify!(_Self))
            .field(stringify!(r#type), &self.r#type())
            .finish()
    }
}
impl ::std::cmp::PartialEq for _Self {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion as _;
        true && self.type_opt() == rhs.type_opt()
    }
}
