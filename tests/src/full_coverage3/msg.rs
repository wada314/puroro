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
pub struct Submsg {
    body: self::_root::full_coverage3::msg::_view::SubmsgView,
}
impl Submsg {
    pub fn i32_unlabeled_mut(&mut self) -> &mut i32 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.body.fields.i32_unlabeled,
            self.body.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_i32_unlabeled(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(
            &mut self.body.fields.i32_unlabeled,
            self.body.shared.bitfield_mut(),
        )
    }
    pub fn i32_optional_mut(&mut self) -> &mut i32 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.body.fields.i32_optional,
            self.body.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_i32_optional(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(
            &mut self.body.fields.i32_optional,
            self.body.shared.bitfield_mut(),
        )
    }
    pub fn i64_unlabeled_mut(&mut self) -> &mut i64 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.body.fields.i64_unlabeled,
            self.body.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_i64_unlabeled(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(
            &mut self.body.fields.i64_unlabeled,
            self.body.shared.bitfield_mut(),
        )
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
            &self.body.fields.i32_unlabeled,
            self.body.shared.bitfield(),
            1i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.body.fields.i32_optional,
            self.body.shared.bitfield(),
            2i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.body.fields.i64_unlabeled,
            self.body.shared.bitfield(),
            101i32,
            out,
        )?;
        self.shared.unknown_fields().ser_to_write(out)?;
        ::std::result::Result::Ok(())
    }
}
impl self::_pinternal::MessageInternal for Submsg {
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
                            &mut self.body.fields.i32_unlabeled,
                            self.body.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    2i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.body.fields.i32_optional,
                            self.body.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    101i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.body.fields.i64_unlabeled,
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
impl ::std::borrow::Borrow<self::_root::full_coverage3::msg::_view::SubmsgView>
for Submsg {
    fn borrow(&self) -> &self::_root::full_coverage3::msg::_view::SubmsgView {
        &self.body
    }
}
impl ::std::clone::Clone for Submsg {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use ::std::borrow::ToOwned;
        ToOwned::to_owned(&self.body)
    }
}
impl ::std::fmt::Debug for Submsg {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        <self::_root::full_coverage3::msg::_view::SubmsgView as ::std::fmt::Debug>::fmt(
            &self.body,
            fmt,
        )
    }
}
impl ::std::ops::Deref for Submsg {
    type Target = self::_root::full_coverage3::msg::_view::SubmsgView;
    fn deref(&self) -> &Self::Target {
        &self.body
    }
}
impl ::std::cmp::PartialEq for Submsg {
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
    pub struct SubmsgView {
        pub(super) fields: self::_root::full_coverage3::msg::_fields::SubmsgFields::<
            self::_pinternal::SingularNumericalField::<
                i32,
                self::_pinternal::tags::Int32,
            >,
            self::_pinternal::SingularNumericalField::<
                i32,
                self::_pinternal::tags::Int32,
            >,
            self::_pinternal::SingularNumericalField::<
                i64,
                self::_pinternal::tags::Int64,
            >,
        >,
        pub(super) shared: self::_pinternal::SharedItemsImpl<0usize>,
    }
    impl SubmsgView {
        pub fn i32_unlabeled(&self) -> i32 {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.i32_unlabeled,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn i32_unlabeled_opt(&self) -> ::std::option::Option::<i32> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.i32_unlabeled,
                self.shared.bitfield(),
            )
        }
        pub fn has_i32_unlabeled(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.i32_unlabeled,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn i32_optional(&self) -> i32 {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.i32_optional,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn i32_optional_opt(&self) -> ::std::option::Option::<i32> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.i32_optional,
                self.shared.bitfield(),
            )
        }
        pub fn has_i32_optional(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.i32_optional,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn i64_unlabeled(&self) -> i64 {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.i64_unlabeled,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn i64_unlabeled_opt(&self) -> ::std::option::Option::<i64> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.i64_unlabeled,
                self.shared.bitfield(),
            )
        }
        pub fn has_i64_unlabeled(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.i64_unlabeled,
                    self.shared.bitfield(),
                )
                .is_some()
        }
    }
    impl ::std::ops::Drop for SubmsgView {
        fn drop(&mut self) {
            #[allow(unused)]
            use self::_pinternal::{OneofUnion as _, SharedItems as _};
        }
    }
    impl ::std::fmt::Debug for SubmsgView {
        fn fmt(
            &self,
            fmt: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            let mut debug_struct = fmt.debug_struct(stringify!(SubmsgView));
            debug_struct
                .field(stringify!(i32_unlabeled), &self.i32_unlabeled_opt())
                .field(stringify!(i32_optional), &self.i32_optional_opt())
                .field(stringify!(i64_unlabeled), &self.i64_unlabeled_opt());
            self.shared.unknown_fields().debug_struct_fields(&mut debug_struct)?;
            debug_struct.finish()
        }
    }
    impl ::std::cmp::PartialEq for SubmsgView {
        fn eq(&self, rhs: &Self) -> bool {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::SharedItems as _;
            true && self.i32_unlabeled_opt() == rhs.i32_unlabeled_opt()
                && self.i32_optional_opt() == rhs.i32_optional_opt()
                && self.i64_unlabeled_opt() == rhs.i64_unlabeled_opt()
                && self.shared.unknown_fields() == rhs.shared.unknown_fields()
        }
    }
    impl ::std::borrow::ToOwned for SubmsgView {
        type Owned = self::_root::full_coverage3::msg::Submsg;
        fn to_owned(&self) -> Self::Owned {
            #[allow(unused)]
            use self::_pinternal::SharedItems;
            self::_root::full_coverage3::msg::Submsg {
                body: Self {
                    fields: self::_root::full_coverage3::msg::_fields::SubmsgFields {
                        i32_unlabeled: ::std::clone::Clone::clone(
                            &self.fields.i32_unlabeled,
                        ),
                        i32_optional: ::std::clone::Clone::clone(
                            &self.fields.i32_optional,
                        ),
                        i64_unlabeled: ::std::clone::Clone::clone(
                            &self.fields.i64_unlabeled,
                        ),
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
    pub struct SubmsgFields<TI32Unlabeled, TI32Optional, TI64Unlabeled> {
        pub i32_unlabeled: TI32Unlabeled,
        pub i32_optional: TI32Optional,
        pub i64_unlabeled: TI64Unlabeled,
    }
}
#[doc(hidden)]
pub use self::_fields::*;
