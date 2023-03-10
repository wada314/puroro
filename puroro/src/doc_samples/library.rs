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
#[derive(::std::cmp::PartialEq)]
pub struct Book(::std::boxed::Box<self::_root::library::_view::BookView>);
impl Book {
    pub fn title_mut(
        &mut self,
    ) -> impl '_ + ::std::ops::Deref<
        Target = ::std::string::String,
    > + ::std::ops::DerefMut {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::library::_view::BookView = &mut self.0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.title,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_title(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::library::_view::BookView = &mut self.0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.title,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn num_pages_mut(&mut self) -> &mut u32 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::library::_view::BookView = &mut self.0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.num_pages,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_num_pages(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::library::_view::BookView = &mut self.0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.num_pages,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn author_mut(
        &mut self,
    ) -> impl '_ + ::std::ops::Deref<
        Target = self::_root::library::Author,
    > + ::std::ops::DerefMut {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::library::_view::BookView = &mut self.0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.author,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_author(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::library::_view::BookView = &mut self.0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.author,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub const TITLE_FIELD_NUMBER: i32 = 1i32;
    pub const NUM_PAGES_FIELD_NUMBER: i32 = 2i32;
    pub const AUTHOR_FIELD_NUMBER: i32 = 3i32;
}
impl self::_puroro::Message for Book {
    type ViewType = self::_root::library::_view::BookView;
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
}
impl self::_pinternal::MessageInternal for Book {
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
                        let view_ref: &mut self::_root::library::_view::BookView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.title,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    2i32 => {
                        let view_ref: &mut self::_root::library::_view::BookView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.num_pages,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    3i32 => {
                        let view_ref: &mut self::_root::library::_view::BookView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.author,
                            view_ref.shared.bitfield_mut(),
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
                    self.0.shared.unknown_fields_mut().push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
    fn from_boxed_view(
        v: ::std::boxed::Box<<Self as self::_puroro::Message>::ViewType>,
    ) -> Self {
        Self(v)
    }
}
impl ::std::borrow::Borrow<self::_root::library::_view::BookView> for Book {
    fn borrow(&self) -> &self::_root::library::_view::BookView {
        &self
    }
}
impl ::std::clone::Clone for Book {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use ::std::borrow::ToOwned;
        <self::_root::library::_view::BookView as ToOwned>::to_owned(&self)
    }
}
impl ::std::fmt::Debug for Book {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        <self::_root::library::_view::BookView as ::std::fmt::Debug>::fmt(&self, fmt)
    }
}
impl ::std::ops::Deref for Book {
    type Target = self::_root::library::_view::BookView;
    fn deref(&self) -> &Self::Target {
        <::std::boxed::Box<_> as ::std::ops::Deref>::deref(&self.0)
    }
}
#[derive(::std::default::Default)]
#[derive(::std::cmp::PartialEq)]
pub struct Author(::std::boxed::Box<self::_root::library::_view::AuthorView>);
impl Author {
    pub fn name_mut(
        &mut self,
    ) -> impl '_ + ::std::ops::Deref<
        Target = ::std::string::String,
    > + ::std::ops::DerefMut {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::library::_view::AuthorView = &mut self.0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.name,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_name(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::library::_view::AuthorView = &mut self.0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.name,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub const NAME_FIELD_NUMBER: i32 = 1i32;
}
impl self::_puroro::Message for Author {
    type ViewType = self::_root::library::_view::AuthorView;
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
}
impl self::_pinternal::MessageInternal for Author {
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
                        let view_ref: &mut self::_root::library::_view::AuthorView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.name,
                            view_ref.shared.bitfield_mut(),
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
                    self.0.shared.unknown_fields_mut().push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
    fn from_boxed_view(
        v: ::std::boxed::Box<<Self as self::_puroro::Message>::ViewType>,
    ) -> Self {
        Self(v)
    }
}
impl ::std::borrow::Borrow<self::_root::library::_view::AuthorView> for Author {
    fn borrow(&self) -> &self::_root::library::_view::AuthorView {
        &self
    }
}
impl ::std::clone::Clone for Author {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use ::std::borrow::ToOwned;
        <self::_root::library::_view::AuthorView as ToOwned>::to_owned(&self)
    }
}
impl ::std::fmt::Debug for Author {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        <self::_root::library::_view::AuthorView as ::std::fmt::Debug>::fmt(&self, fmt)
    }
}
impl ::std::ops::Deref for Author {
    type Target = self::_root::library::_view::AuthorView;
    fn deref(&self) -> &Self::Target {
        <::std::boxed::Box<_> as ::std::ops::Deref>::deref(&self.0)
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
    pub struct BookView {
        pub(super) fields: self::_root::library::_fields::BookFields::<
            self::_pinternal::SingularUnsizedField::<
                ::std::string::String,
                self::_pinternal::tags::String,
            >,
            self::_pinternal::SingularNumericalField::<
                u32,
                self::_pinternal::tags::UInt32,
            >,
            self::_pinternal::SingularMessageField::<
                self::_root::library::_view::AuthorView,
            >,
        >,
        pub(super) shared: self::_pinternal::SharedItemsImpl<0usize>,
    }
    impl BookView {
        pub fn title(&self) -> &str {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.title,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn title_opt(&self) -> ::std::option::Option::<&str> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.title,
                self.shared.bitfield(),
            )
        }
        pub fn has_title(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.title,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn num_pages(&self) -> u32 {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.num_pages,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn num_pages_opt(&self) -> ::std::option::Option::<u32> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.num_pages,
                self.shared.bitfield(),
            )
        }
        pub fn has_num_pages(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.num_pages,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn author(
            &self,
        ) -> ::std::option::Option::<&self::_root::library::_view::AuthorView> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.author,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn author_opt(
            &self,
        ) -> ::std::option::Option::<&self::_root::library::_view::AuthorView> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.author,
                self.shared.bitfield(),
            )
        }
        pub fn has_author(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.author,
                    self.shared.bitfield(),
                )
                .is_some()
        }
    }
    impl self::_puroro::MessageView for BookView {
        type MessageType = self::_root::library::Book;
        fn to_bytes<W: ::std::io::Write>(
            &self,
            #[allow(unused)]
            out: &mut W,
        ) -> self::_puroro::Result<()> {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.title,
                self.shared.bitfield(),
                1i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.num_pages,
                self.shared.bitfield(),
                2i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.author,
                self.shared.bitfield(),
                3i32,
                out,
            )?;
            self.shared.unknown_fields().ser_to_write(out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl ::std::ops::Drop for BookView {
        fn drop(&mut self) {
            #[allow(unused)]
            use self::_pinternal::{OneofUnion as _, SharedItems as _};
        }
    }
    impl ::std::fmt::Debug for BookView {
        fn fmt(
            &self,
            fmt: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            let mut debug_struct = fmt.debug_struct(stringify!(BookView));
            debug_struct
                .field(stringify!(title), &self.title_opt())
                .field(stringify!(num_pages), &self.num_pages_opt())
                .field(stringify!(author), &self.author_opt());
            self.shared.unknown_fields().debug_struct_fields(&mut debug_struct)?;
            debug_struct.finish()
        }
    }
    impl ::std::cmp::PartialEq for BookView {
        fn eq(&self, rhs: &Self) -> bool {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::SharedItems as _;
            true && self.title_opt() == rhs.title_opt()
                && self.num_pages_opt() == rhs.num_pages_opt()
                && self.author_opt() == rhs.author_opt()
                && self.shared.unknown_fields() == rhs.shared.unknown_fields()
        }
    }
    impl ::std::borrow::ToOwned for BookView {
        type Owned = self::_root::library::Book;
        fn to_owned(&self) -> Self::Owned {
            #[allow(unused)]
            use self::_pinternal::SharedItems;
            self::_root::library::Book(
                ::std::boxed::Box::new(Self {
                    fields: self::_root::library::_fields::BookFields {
                        title: ::std::clone::Clone::clone(&self.fields.title),
                        num_pages: ::std::clone::Clone::clone(&self.fields.num_pages),
                        author: ::std::clone::Clone::clone(&self.fields.author),
                    },
                    shared: ::std::clone::Clone::clone(&self.shared),
                }),
            )
        }
    }
    #[derive(::std::default::Default)]
    pub struct AuthorView {
        pub(super) fields: self::_root::library::_fields::AuthorFields::<
            self::_pinternal::SingularUnsizedField::<
                ::std::string::String,
                self::_pinternal::tags::String,
            >,
        >,
        pub(super) shared: self::_pinternal::SharedItemsImpl<0usize>,
    }
    impl AuthorView {
        pub fn name(&self) -> &str {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.name,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn name_opt(&self) -> ::std::option::Option::<&str> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.name,
                self.shared.bitfield(),
            )
        }
        pub fn has_name(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.name,
                    self.shared.bitfield(),
                )
                .is_some()
        }
    }
    impl self::_puroro::MessageView for AuthorView {
        type MessageType = self::_root::library::Author;
        fn to_bytes<W: ::std::io::Write>(
            &self,
            #[allow(unused)]
            out: &mut W,
        ) -> self::_puroro::Result<()> {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.name,
                self.shared.bitfield(),
                1i32,
                out,
            )?;
            self.shared.unknown_fields().ser_to_write(out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl ::std::ops::Drop for AuthorView {
        fn drop(&mut self) {
            #[allow(unused)]
            use self::_pinternal::{OneofUnion as _, SharedItems as _};
        }
    }
    impl ::std::fmt::Debug for AuthorView {
        fn fmt(
            &self,
            fmt: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            let mut debug_struct = fmt.debug_struct(stringify!(AuthorView));
            debug_struct.field(stringify!(name), &self.name_opt());
            self.shared.unknown_fields().debug_struct_fields(&mut debug_struct)?;
            debug_struct.finish()
        }
    }
    impl ::std::cmp::PartialEq for AuthorView {
        fn eq(&self, rhs: &Self) -> bool {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::SharedItems as _;
            true && self.name_opt() == rhs.name_opt()
                && self.shared.unknown_fields() == rhs.shared.unknown_fields()
        }
    }
    impl ::std::borrow::ToOwned for AuthorView {
        type Owned = self::_root::library::Author;
        fn to_owned(&self) -> Self::Owned {
            #[allow(unused)]
            use self::_pinternal::SharedItems;
            self::_root::library::Author(
                ::std::boxed::Box::new(Self {
                    fields: self::_root::library::_fields::AuthorFields {
                        name: ::std::clone::Clone::clone(&self.fields.name),
                    },
                    shared: ::std::clone::Clone::clone(&self.shared),
                }),
            )
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
    pub struct BookFields<TTitle, TNumPages, TAuthor> {
        pub title: TTitle,
        pub num_pages: TNumPages,
        pub author: TAuthor,
    }
    #[derive(::std::default::Default)]
    pub struct AuthorFields<TName> {
        pub name: TName,
    }
}
#[doc(hidden)]
pub use self::_fields::*;
