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
    fields: self::_root::google::protobuf::enum_descriptor_proto::_fields::EnumReservedRangeFields<
        self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        >,
        self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            1usize,
        >,
    >,
    bitfield: self::_pinternal::BitArray<1usize>,
    unknown_fields: self::_pinternal::UnknownFieldsImpl,
}
impl EnumReservedRange {
    pub fn start(&self) -> i32 {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.start,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn start_opt(&self) -> ::std::option::Option::<i32> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.start, &self.bitfield)
    }
    pub fn start_mut(&mut self) -> &mut i32 {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.start,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_start(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.start, &self.bitfield).is_some()
    }
    pub fn clear_start(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.start, &mut self.bitfield)
    }
    pub fn end(&self) -> i32 {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.end,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn end_opt(&self) -> ::std::option::Option::<i32> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.end, &self.bitfield)
    }
    pub fn end_mut(&mut self) -> &mut i32 {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.end,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_end(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.end, &self.bitfield).is_some()
    }
    pub fn clear_end(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.end, &mut self.bitfield)
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
        use self::_pinternal::OneofUnion as _;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            match number {
                1i32 => {
                    self::_pinternal::FieldType::deser_from_iter(
                        &mut self.fields.start,
                        &mut self.bitfield,
                        field_data,
                    )?
                }
                2i32 => {
                    self::_pinternal::FieldType::deser_from_iter(
                        &mut self.fields.end,
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
            &self.fields.start,
            &self.bitfield,
            1i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.end,
            &self.bitfield,
            2i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for EnumReservedRange {
    fn clone(&self) -> Self {
        Self {
            fields: self::_fields::EnumReservedRangeFields {
                start: ::std::clone::Clone::clone(&self.fields.start),
                end: ::std::clone::Clone::clone(&self.fields.end),
            },
            bitfield: ::std::clone::Clone::clone(&self.bitfield),
            unknown_fields: ::std::clone::Clone::clone(&self.unknown_fields),
        }
    }
}
impl ::std::ops::Drop for EnumReservedRange {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for EnumReservedRange {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        use self::_pinternal::UnknownFields as _;
        let mut debug_struct = fmt.debug_struct(stringify!(EnumReservedRange));
        debug_struct
            .field(stringify!(start), &self.start_opt())
            .field(stringify!(end), &self.end_opt());
        self.unknown_fields.debug_struct_fields(&mut debug_struct)?;
        debug_struct.finish()
    }
}
impl ::std::cmp::PartialEq for EnumReservedRange {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        true && self.start_opt() == rhs.start_opt() && self.end_opt() == rhs.end_opt()
            && self.unknown_fields == rhs.unknown_fields
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
    pub struct EnumReservedRangeFields<TStart, TEnd> {
        pub start: TStart,
        pub end: TEnd,
    }
}
pub use self::_fields::*;
