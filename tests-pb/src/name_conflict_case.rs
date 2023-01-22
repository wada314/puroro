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
pub mod message;
#[derive(::std::default::Default)]
pub struct Message {
    fields: self::_root::name_conflict_case::_fields::MessageFields<
        self::_root::name_conflict_case::message::Conflict::<
            self::_pinternal::NumericalField::<i32, self::_pinternal::tags::Int32>,
        >,
    >,
    shared: self::_pinternal::SharedItems<1usize>,
}
impl Message {
    pub fn conflict(
        &self,
    ) -> ::std::option::Option<
        self::_root::name_conflict_case::message::_case::ConflictCase::<i32>,
    > {
        use self::_pinternal::{SharedItemsTrait as _, OneofUnion as _};
        self.fields.conflict.case_ref(self.shared.bitfield())
    }
    pub fn clear_conflict(&mut self) {
        use self::_pinternal::{SharedItemsTrait as _, OneofUnion as _};
        self.fields.conflict.clear(self.shared.bitfield_mut())
    }
    pub fn this_is_oneof_field(&self) -> i32 {
        use self::_pinternal::SharedItemsTrait as _;
        self.fields.conflict.this_is_oneof_field(self.shared.bitfield())
    }
    pub fn this_is_oneof_field_opt(&self) -> ::std::option::Option::<i32> {
        use self::_pinternal::SharedItemsTrait as _;
        self.fields.conflict.this_is_oneof_field_opt(self.shared.bitfield())
    }
    pub fn this_is_oneof_field_mut(&mut self) -> &mut i32 {
        use self::_pinternal::SharedItemsTrait as _;
        self.fields.conflict.this_is_oneof_field_mut(self.shared.bitfield_mut())
    }
    pub fn has_this_is_oneof_field(&self) -> bool {
        self.this_is_oneof_field_opt().is_some()
    }
    pub fn clear_this_is_oneof_field(&mut self) {
        #[allow(unused)]
        use ::std::option::Option::Some;
        use self::_pinternal::{OneofCase, OneofUnion as _, SharedItemsTrait as _};
        if let Some(
            self::_root::name_conflict_case::message::_case::ConflictCase::ThisIsOneofField(
                _,
            ),
        ) = OneofCase::from_bitslice(self.shared.bitfield()) {
            self.fields.conflict.clear(self.shared.bitfield_mut())
        }
    }
}
impl self::_puroro::Message for Message {
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
        use self::_pinternal::{SharedItemsTrait as _, UnknownFields as _};
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        use self::_puroro::PuroroError;
        while let Some((number, mut field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    1i32 => {
                        self
                            .fields
                            .conflict
                            .deser_from_iter(
                                self.shared.bitfield_mut(),
                                &mut field_data,
                                self::_root::name_conflict_case::message::_case::ConflictCase::ThisIsOneofField(()),
                            )?
                    }
                    _ => Err(PuroroError::UnknownFieldNumber)?,
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(
                    PuroroError::UnknownFieldNumber | PuroroError::UnknownEnumVariant(_),
                ) => {
                    self.shared.unknown_fields_mut().push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::{SharedItemsTrait as _, UnknownFields as _};
        self.fields.conflict.ser_to_write(self.shared.bitfield(), out)?;
        self.shared.unknown_fields().ser_to_write(out)?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for Message {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use self::_pinternal::SharedItemsTrait as _;
        Self {
            fields: self::_fields::MessageFields {
                conflict: self::_pinternal::OneofUnion::clone(
                    &self.fields.conflict,
                    self.shared.bitfield(),
                ),
            },
            shared: ::std::clone::Clone::clone(&self.shared),
        }
    }
}
impl ::std::ops::Drop for Message {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::SharedItemsTrait as _;
        self.fields.conflict.clear(self.shared.bitfield_mut());
    }
}
impl ::std::fmt::Debug for Message {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        use self::_pinternal::{SharedItemsTrait as _, UnknownFields as _};
        let mut debug_struct = fmt.debug_struct(stringify!(Message));
        debug_struct
            .field(stringify!(this_is_oneof_field), &self.this_is_oneof_field_opt());
        self.shared.unknown_fields().debug_struct_fields(&mut debug_struct)?;
        debug_struct.finish()
    }
}
impl ::std::cmp::PartialEq for Message {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::SharedItemsTrait as _;
        true && self.conflict() == rhs.conflict()
            && self.shared.unknown_fields() == rhs.shared.unknown_fields()
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
    pub struct MessageFields<TConflict> {
        pub conflict: TConflict,
    }
}
pub use self::_fields::*;
