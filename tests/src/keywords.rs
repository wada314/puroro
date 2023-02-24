mod _root {
    #[allow(unused)]
    pub(crate) use super::super::_root::*;
}
mod _puroro {
    #[allow(unused)]
    pub(crate) use super::_root::_puroro::*;
}
mod _pinternal {
    #[allow(unused)]
    pub(crate) use super::_root::_pinternal::*;
}
#[derive(::std::default::Default)]
pub struct Msg<#[cfg(feature = "bumpalo")] B = ()> {
    body: self::_root::keywords::_view::MsgView,
    #[cfg(feature = "bumpalo")]
    bump: B,
}
impl Msg {
    pub fn type_mut(&mut self) -> &mut i32 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.body.fields.r#type,
            self.body.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_type(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(
            &mut self.body.fields.r#type,
            self.body.shared.bitfield_mut(),
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
        iter: I,
    ) -> self::_puroro::Result<()> {
        let mut pos_iter = self::_pinternal::PosIter::new(iter);
        let mut scoped_iter = self::_pinternal::ScopedIter::from_mut_pos_iter(
            &mut pos_iter,
        );
        <Self as self::_pinternal::MessageInternal>::merge_from_scoped_bytes_iter(
            self,
            &mut scoped_iter,
        )?;
        scoped_iter.drop_and_check_scope_completed()?;
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        self::_pinternal::FieldType::ser_to_write(
            &self.body.fields.r#type,
            self.body.shared.bitfield(),
            1i32,
            out,
        )?;
        self.shared.unknown_fields().ser_to_write(out)?;
        ::std::result::Result::Ok(())
    }
}
impl self::_pinternal::MessageInternal for Msg {
    fn merge_from_scoped_bytes_iter<
        'a,
        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
    >(
        &mut self,
        iter: &mut self::_pinternal::ScopedIter<'a, I>,
    ) -> self::_puroro::Result<()> {
        use self::_pinternal::ser::FieldData;
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        #[allow(unused)]
        use ::std::result::Result;
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        #[allow(unused)]
        use ::std::vec::Vec;
        use self::_puroro::PuroroError;
        while let Some((number, field_data))
            = FieldData::from_bytes_scoped_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    1i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.body.fields.r#type,
                            self.body.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    _ => {
                        let field_data = field_data
                            .map(|iter| { iter.collect::<Result<Vec<_>, _>>() })
                            .transpose()?;
                        Err(PuroroError::UnknownFieldNumber(field_data))?
                    }
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(PuroroError::UnknownFieldNumber(field_data)) => {
                    self.body.shared.unknown_fields_mut().push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
}
impl ::std::borrow::Borrow<self::_root::keywords::_view::MsgView> for Msg {
    fn borrow(&self) -> &self::_root::keywords::_view::MsgView {
        &self.body
    }
}
impl ::std::clone::Clone for Msg {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use ::std::borrow::ToOwned;
        ToOwned::to_owned(&self.body)
    }
}
impl ::std::fmt::Debug for Msg {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        <self::_root::keywords::_view::MsgView as ::std::fmt::Debug>::fmt(
            &self.body,
            fmt,
        )
    }
}
impl ::std::ops::Deref for Msg {
    type Target = self::_root::keywords::_view::MsgView;
    fn deref(&self) -> &Self::Target {
        &self.body
    }
}
impl ::std::cmp::PartialEq for Msg {
    fn eq(&self, rhs: &Self) -> bool {
        &self.body == &rhs.body
    }
}
#[derive(::std::default::Default)]
pub struct _Self<#[cfg(feature = "bumpalo")] B = ()> {
    body: self::_root::keywords::_view::SelfView,
    #[cfg(feature = "bumpalo")]
    bump: B,
}
impl _Self {
    pub fn type_mut(&mut self) -> &mut i32 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.body.fields.r#type,
            self.body.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_type(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(
            &mut self.body.fields.r#type,
            self.body.shared.bitfield_mut(),
        )
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
        iter: I,
    ) -> self::_puroro::Result<()> {
        let mut pos_iter = self::_pinternal::PosIter::new(iter);
        let mut scoped_iter = self::_pinternal::ScopedIter::from_mut_pos_iter(
            &mut pos_iter,
        );
        <Self as self::_pinternal::MessageInternal>::merge_from_scoped_bytes_iter(
            self,
            &mut scoped_iter,
        )?;
        scoped_iter.drop_and_check_scope_completed()?;
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        self::_pinternal::FieldType::ser_to_write(
            &self.body.fields.r#type,
            self.body.shared.bitfield(),
            1i32,
            out,
        )?;
        self.shared.unknown_fields().ser_to_write(out)?;
        ::std::result::Result::Ok(())
    }
}
impl self::_pinternal::MessageInternal for _Self {
    fn merge_from_scoped_bytes_iter<
        'a,
        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
    >(
        &mut self,
        iter: &mut self::_pinternal::ScopedIter<'a, I>,
    ) -> self::_puroro::Result<()> {
        use self::_pinternal::ser::FieldData;
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        #[allow(unused)]
        use ::std::result::Result;
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        #[allow(unused)]
        use ::std::vec::Vec;
        use self::_puroro::PuroroError;
        while let Some((number, field_data))
            = FieldData::from_bytes_scoped_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    1i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.body.fields.r#type,
                            self.body.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    _ => {
                        let field_data = field_data
                            .map(|iter| { iter.collect::<Result<Vec<_>, _>>() })
                            .transpose()?;
                        Err(PuroroError::UnknownFieldNumber(field_data))?
                    }
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(PuroroError::UnknownFieldNumber(field_data)) => {
                    self.body.shared.unknown_fields_mut().push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
}
impl ::std::borrow::Borrow<self::_root::keywords::_view::SelfView> for _Self {
    fn borrow(&self) -> &self::_root::keywords::_view::SelfView {
        &self.body
    }
}
impl ::std::clone::Clone for _Self {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use ::std::borrow::ToOwned;
        ToOwned::to_owned(&self.body)
    }
}
impl ::std::fmt::Debug for _Self {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        <self::_root::keywords::_view::SelfView as ::std::fmt::Debug>::fmt(
            &self.body,
            fmt,
        )
    }
}
impl ::std::ops::Deref for _Self {
    type Target = self::_root::keywords::_view::SelfView;
    fn deref(&self) -> &Self::Target {
        &self.body
    }
}
impl ::std::cmp::PartialEq for _Self {
    fn eq(&self, rhs: &Self) -> bool {
        &self.body == &rhs.body
    }
}
#[doc(hidden)]
pub mod _view {
    mod _root {
        #[allow(unused)]
        pub(crate) use super::super::_root::*;
    }
    mod _puroro {
        #[allow(unused)]
        pub(crate) use super::_root::_puroro::*;
    }
    mod _pinternal {
        #[allow(unused)]
        pub(crate) use super::_root::_pinternal::*;
    }
    #[derive(::std::default::Default)]
    pub struct MsgView {
        pub(super) fields: self::_root::keywords::_fields::MsgFields::<
            self::_pinternal::OptionalNumericalField::<
                i32,
                self::_pinternal::tags::Int32,
                0usize,
            >,
        >,
        pub(super) shared: self::_pinternal::SharedItemsImpl<1usize>,
    }
    impl MsgView {
        pub fn r#type(&self) -> i32 {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.r#type,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn type_opt(&self) -> ::std::option::Option::<i32> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.r#type,
                self.shared.bitfield(),
            )
        }
        pub fn has_type(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.r#type,
                    self.shared.bitfield(),
                )
                .is_some()
        }
    }
    impl ::std::ops::Drop for MsgView {
        fn drop(&mut self) {
            #[allow(unused)]
            use self::_pinternal::{OneofUnion as _, SharedItems as _};
        }
    }
    impl ::std::fmt::Debug for MsgView {
        fn fmt(
            &self,
            fmt: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            let mut debug_struct = fmt.debug_struct(stringify!(MsgView));
            debug_struct.field(stringify!(r#type), &self.type_opt());
            self.shared.unknown_fields().debug_struct_fields(&mut debug_struct)?;
            debug_struct.finish()
        }
    }
    impl ::std::cmp::PartialEq for MsgView {
        fn eq(&self, rhs: &Self) -> bool {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::SharedItems as _;
            true && self.type_opt() == rhs.type_opt()
                && self.shared.unknown_fields() == rhs.shared.unknown_fields()
        }
    }
    impl ::std::borrow::ToOwned for MsgView {
        type Owned = self::_root::keywords::Msg;
        fn to_owned(&self) -> Self::Owned {
            #[allow(unused)]
            use self::_pinternal::SharedItems;
            self::_root::keywords::Msg {
                body: Self {
                    fields: self::_root::keywords::_fields::MsgFields {
                        r#type: ::std::clone::Clone::clone(&self.fields.r#type),
                    },
                    shared: ::std::clone::Clone::clone(&self.shared),
                },
            }
        }
    }
    #[derive(::std::default::Default)]
    pub struct SelfView {
        pub(super) fields: self::_root::keywords::_fields::SelfFields::<
            self::_pinternal::OptionalNumericalField::<
                i32,
                self::_pinternal::tags::Int32,
                0usize,
            >,
        >,
        pub(super) shared: self::_pinternal::SharedItemsImpl<1usize>,
    }
    impl SelfView {
        pub fn r#type(&self) -> i32 {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.r#type,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn type_opt(&self) -> ::std::option::Option::<i32> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.r#type,
                self.shared.bitfield(),
            )
        }
        pub fn has_type(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.r#type,
                    self.shared.bitfield(),
                )
                .is_some()
        }
    }
    impl ::std::ops::Drop for SelfView {
        fn drop(&mut self) {
            #[allow(unused)]
            use self::_pinternal::{OneofUnion as _, SharedItems as _};
        }
    }
    impl ::std::fmt::Debug for SelfView {
        fn fmt(
            &self,
            fmt: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            let mut debug_struct = fmt.debug_struct(stringify!(SelfView));
            debug_struct.field(stringify!(r#type), &self.type_opt());
            self.shared.unknown_fields().debug_struct_fields(&mut debug_struct)?;
            debug_struct.finish()
        }
    }
    impl ::std::cmp::PartialEq for SelfView {
        fn eq(&self, rhs: &Self) -> bool {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::SharedItems as _;
            true && self.type_opt() == rhs.type_opt()
                && self.shared.unknown_fields() == rhs.shared.unknown_fields()
        }
    }
    impl ::std::borrow::ToOwned for SelfView {
        type Owned = self::_root::keywords::_Self;
        fn to_owned(&self) -> Self::Owned {
            #[allow(unused)]
            use self::_pinternal::SharedItems;
            self::_root::keywords::_Self {
                body: Self {
                    fields: self::_root::keywords::_fields::SelfFields {
                        r#type: ::std::clone::Clone::clone(&self.fields.r#type),
                    },
                    shared: ::std::clone::Clone::clone(&self.shared),
                },
            }
        }
    }
}
#[doc(inline)]
pub use self::_view::*;
#[doc(hidden)]
pub mod _fields {
    mod _root {
        #[allow(unused)]
        pub(crate) use super::super::_root::*;
    }
    mod _puroro {
        #[allow(unused)]
        pub(crate) use super::_root::_puroro::*;
    }
    mod _pinternal {
        #[allow(unused)]
        pub(crate) use super::_root::_pinternal::*;
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
#[doc(hidden)]
pub use self::_fields::*;
