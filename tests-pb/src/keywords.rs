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
pub struct Msg {
    fields: self::_root::keywords::_fields::MsgFields<
        self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        >,
    >,
    bitfield: self::_pinternal::BitArray<1usize>,
}
impl Msg {
    pub fn r#type(&self) -> i32 {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.r#type,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn type_opt(&self) -> ::std::option::Option::<i32> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.r#type, &self.bitfield)
    }
    pub fn type_mut(&mut self) -> &mut i32 {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.r#type,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_type(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.r#type, &self.bitfield)
            .is_some()
    }
    pub fn clear_type(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.r#type, &mut self.bitfield)
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
        use self::_pinternal::ser::FieldData;
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            match number {
                1i32 => {
                    self::_pinternal::FieldType::deser_from_iter(
                        &mut self.fields.r#type,
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
            &self.fields.r#type,
            &self.bitfield,
            1i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for Msg {
    fn clone(&self) -> Self {
        Self {
            fields: self::_fields::MsgFields {
                r#type: ::std::clone::Clone::clone(&self.fields.r#type),
            },
            bitfield: ::std::clone::Clone::clone(&self.bitfield),
        }
    }
}
impl ::std::ops::Drop for Msg {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for Msg {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct(stringify!(Msg))
            .field(stringify!(r#type), &self.type_opt())
            .finish()
    }
}
impl ::std::cmp::PartialEq for Msg {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        true && self.type_opt() == rhs.type_opt()
    }
}
#[derive(::std::default::Default)]
pub struct _Self {
    fields: self::_root::keywords::_fields::SelfFields<
        self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        >,
    >,
    bitfield: self::_pinternal::BitArray<1usize>,
}
impl _Self {
    pub fn r#type(&self) -> i32 {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.r#type,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn type_opt(&self) -> ::std::option::Option::<i32> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.r#type, &self.bitfield)
    }
    pub fn type_mut(&mut self) -> &mut i32 {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.r#type,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_type(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.r#type, &self.bitfield)
            .is_some()
    }
    pub fn clear_type(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.r#type, &mut self.bitfield)
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
        use self::_pinternal::ser::FieldData;
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            match number {
                1i32 => {
                    self::_pinternal::FieldType::deser_from_iter(
                        &mut self.fields.r#type,
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
            &self.fields.r#type,
            &self.bitfield,
            1i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for _Self {
    fn clone(&self) -> Self {
        Self {
            fields: self::_fields::SelfFields {
                r#type: ::std::clone::Clone::clone(&self.fields.r#type),
            },
            bitfield: ::std::clone::Clone::clone(&self.bitfield),
        }
    }
}
impl ::std::ops::Drop for _Self {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for _Self {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct(stringify!(_Self))
            .field(stringify!(r#type), &self.type_opt())
            .finish()
    }
}
impl ::std::cmp::PartialEq for _Self {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        true && self.type_opt() == rhs.type_opt()
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
    pub struct MsgFields<TType> {
        pub r#type: TType,
    }
    #[derive(::std::default::Default)]
    pub struct SelfFields<TType> {
        pub r#type: TType,
    }
}
pub use self::_fields::*;
