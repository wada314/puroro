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
pub struct ConflictFields {
    fields: self::_root::name_conflict_fields::conflict::_fields::ConflictFieldsFields<
        self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        >,
    >,
    _bitfield: self::_pinternal::BitArray<1usize>,
}
impl ConflictFields {
    pub fn this_is_child_message_field(&self) -> i32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.this_is_child_message_field,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn this_is_child_message_field_opt(&self) -> ::std::option::Option::<i32> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.this_is_child_message_field,
            &self._bitfield,
        )
    }
    pub fn this_is_child_message_field_mut(&mut self) -> &mut i32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.this_is_child_message_field,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_this_is_child_message_field(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(
                &self.this_is_child_message_field,
                &self._bitfield,
            )
            .is_some()
    }
    pub fn clear_this_is_child_message_field(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::clear(
            &mut self.this_is_child_message_field,
            &mut self._bitfield,
        )
    }
}
impl self::_puroro::Message for ConflictFields {
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
                        &mut self.this_is_child_message_field,
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
            &self.this_is_child_message_field,
            &self._bitfield,
            1i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for ConflictFields {
    fn clone(&self) -> Self {
        Self {
            this_is_child_message_field: <self::_pinternal::OptionalNumericalField::<
                i32,
                self::_pinternal::tags::Int32,
                0usize,
            > as ::std::clone::Clone>::clone(&self.this_is_child_message_field),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
impl ::std::ops::Drop for ConflictFields {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for ConflictFields {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct(stringify!(ConflictFields))
            .field(
                stringify!(this_is_child_message_field),
                &self.this_is_child_message_field_opt(),
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for ConflictFields {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        true
            && self.this_is_child_message_field_opt()
                == rhs.this_is_child_message_field_opt()
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
    pub struct ConflictFieldsFields<TThisIsChildMessageField> {
        pub this_is_child_message_field: TThisIsChildMessageField,
    }
}
pub use self::_fields::*;
