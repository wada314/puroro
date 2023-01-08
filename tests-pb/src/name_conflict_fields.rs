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
pub struct Conflict {
    fields: self::_root::name_conflict_fields::_fields::ConflictFields<
        self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        >,
    >,
    bitfield: self::_pinternal::BitArray<1usize>,
}
impl Conflict {
    pub fn this_is_original_message_field(&self) -> i32 {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.this_is_original_message_field,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn this_is_original_message_field_opt(&self) -> ::std::option::Option::<i32> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
            &self.fields.this_is_original_message_field,
            &self.bitfield,
        )
    }
    pub fn this_is_original_message_field_mut(&mut self) -> &mut i32 {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.this_is_original_message_field,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_this_is_original_message_field(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
                &self.fields.this_is_original_message_field,
                &self.bitfield,
            )
            .is_some()
    }
    pub fn clear_this_is_original_message_field(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(
            &mut self.fields.this_is_original_message_field,
            &mut self.bitfield,
        )
    }
}
impl self::_puroro::Message for Conflict {
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
                    self::_pinternal::FieldType::deser_from_iter(
                        &mut self.fields.this_is_original_message_field,
                        &mut self.bitfield,
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
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.this_is_original_message_field,
            &self.bitfield,
            1i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for Conflict {
    fn clone(&self) -> Self {
        Self {
            fields: self::_fields::ConflictFields {
                this_is_original_message_field: ::std::clone::Clone::clone(
                    &self.fields.this_is_original_message_field,
                ),
            },
            bitfield: ::std::clone::Clone::clone(&self.bitfield),
        }
    }
}
impl ::std::ops::Drop for Conflict {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for Conflict {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct(stringify!(Conflict))
            .field(
                stringify!(this_is_original_message_field),
                &self.this_is_original_message_field_opt(),
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for Conflict {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        true
            && self.this_is_original_message_field_opt()
                == rhs.this_is_original_message_field_opt()
    }
}
#[derive(::std::default::Default)]
pub struct ConflictFields {
    fields: self::_root::name_conflict_fields::_fields::ConflictFieldsFields<
        self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        >,
    >,
    bitfield: self::_pinternal::BitArray<1usize>,
}
impl ConflictFields {
    pub fn this_is_fields_message_field(&self) -> i32 {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.this_is_fields_message_field,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn this_is_fields_message_field_opt(&self) -> ::std::option::Option::<i32> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
            &self.fields.this_is_fields_message_field,
            &self.bitfield,
        )
    }
    pub fn this_is_fields_message_field_mut(&mut self) -> &mut i32 {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.this_is_fields_message_field,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_this_is_fields_message_field(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
                &self.fields.this_is_fields_message_field,
                &self.bitfield,
            )
            .is_some()
    }
    pub fn clear_this_is_fields_message_field(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(
            &mut self.fields.this_is_fields_message_field,
            &mut self.bitfield,
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
                    self::_pinternal::FieldType::deser_from_iter(
                        &mut self.fields.this_is_fields_message_field,
                        &mut self.bitfield,
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
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.this_is_fields_message_field,
            &self.bitfield,
            1i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for ConflictFields {
    fn clone(&self) -> Self {
        Self {
            fields: self::_fields::ConflictFieldsFields {
                this_is_fields_message_field: ::std::clone::Clone::clone(
                    &self.fields.this_is_fields_message_field,
                ),
            },
            bitfield: ::std::clone::Clone::clone(&self.bitfield),
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
                stringify!(this_is_fields_message_field),
                &self.this_is_fields_message_field_opt(),
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for ConflictFields {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        true
            && self.this_is_fields_message_field_opt()
                == rhs.this_is_fields_message_field_opt()
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
    #[derive(::std::default::Default)]
    pub struct ConflictFields<TThisIsOriginalMessageField = ()> {
        pub this_is_original_message_field: TThisIsOriginalMessageField,
    }
    #[derive(::std::default::Default)]
    pub struct ConflictFieldsFields<TThisIsFieldsMessageField = ()> {
        pub this_is_fields_message_field: TThisIsFieldsMessageField,
    }
}
pub use self::_fields::*;
