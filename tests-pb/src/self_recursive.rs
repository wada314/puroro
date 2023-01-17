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
    fields: self::_root::self_recursive::_fields::MsgFields<
        self::_pinternal::SingularHeapMessageField::<self::_root::self_recursive::Msg>,
    >,
    bitfield: self::_pinternal::BitArray<0usize>,
    unknown_fields: self::_pinternal::UnknownFieldsImpl,
}
impl Msg {
    pub fn recursive_unlabeled(
        &self,
    ) -> ::std::option::Option::<&self::_root::self_recursive::Msg> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.recursive_unlabeled,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn recursive_unlabeled_opt(
        &self,
    ) -> ::std::option::Option::<&self::_root::self_recursive::Msg> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
            &self.fields.recursive_unlabeled,
            &self.bitfield,
        )
    }
    pub fn recursive_unlabeled_mut(&mut self) -> &mut self::_root::self_recursive::Msg {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.recursive_unlabeled,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_recursive_unlabeled(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
                &self.fields.recursive_unlabeled,
                &self.bitfield,
            )
            .is_some()
    }
    pub fn clear_recursive_unlabeled(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(
            &mut self.fields.recursive_unlabeled,
            &mut self.bitfield,
        )
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
        use self::_pinternal::UnknownFields as _;
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        use self::_puroro::PuroroError;
        while let Some((number, mut field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    1i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.recursive_unlabeled,
                            &mut self.bitfield,
                            &mut field_data,
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
                    self.unknown_fields.push(number, field_data)?;
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
        use self::_pinternal::UnknownFields as _;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.recursive_unlabeled,
            &self.bitfield,
            1i32,
            out,
        )?;
        self.unknown_fields.ser_to_write(out)?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for Msg {
    fn clone(&self) -> Self {
        Self {
            fields: self::_fields::MsgFields {
                recursive_unlabeled: ::std::clone::Clone::clone(
                    &self.fields.recursive_unlabeled,
                ),
            },
            bitfield: ::std::clone::Clone::clone(&self.bitfield),
            unknown_fields: ::std::clone::Clone::clone(&self.unknown_fields),
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
        use self::_pinternal::UnknownFields as _;
        let mut debug_struct = fmt.debug_struct(stringify!(Msg));
        debug_struct
            .field(stringify!(recursive_unlabeled), &self.recursive_unlabeled_opt());
        self.unknown_fields.debug_struct_fields(&mut debug_struct)?;
        debug_struct.finish()
    }
}
impl ::std::cmp::PartialEq for Msg {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        true && self.recursive_unlabeled_opt() == rhs.recursive_unlabeled_opt()
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
    pub struct MsgFields<TRecursiveUnlabeled> {
        pub recursive_unlabeled: TRecursiveUnlabeled,
    }
}
pub use self::_fields::*;
