// A generated source code by puroro library
// package library

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub use _puroro_simple_impl::Book;
pub mod _puroro_simple_impl {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
    pub struct Book {
        _bitfield:
            ::puroro::bitvec::array::BitArray<::puroro::bitvec::order::Lsb0, [u32; (0 + 31) / 32]>,
        title: ::puroro::internal::Bare<::std::string::String>,
        num_pages: ::puroro::internal::Bare<u32>,
    }
    impl ::puroro::Message<Book> for Book {}

    impl Book {
        pub fn new() -> Self {
            Self {
                _bitfield: ::std::default::Default::default(),
                title: ::std::default::Default::default(),
                num_pages: ::std::default::Default::default(),
            }
        }
        pub fn title_opt(&self) -> ::std::option::Option<&'_ str> {
            if !::puroro::internal::IsDefault::is_default(&*self.title) {
                ::std::option::Option::Some(&self.title)
            } else {
                ::std::option::Option::None
            }
        }

        pub fn has_title(&self) -> bool {
            Self::title_opt(self).is_some()
        }

        pub fn title(&self) -> &'_ str {
            self.title_opt()
                .unwrap_or(::std::default::Default::default())
        }
        pub fn num_pages_opt(&self) -> ::std::option::Option<u32> {
            if !::puroro::internal::IsDefault::is_default(&*self.num_pages) {
                ::std::option::Option::Some(self.num_pages.inner())
            } else {
                ::std::option::Option::None
            }
        }

        pub fn has_num_pages(&self) -> bool {
            Self::num_pages_opt(self).is_some()
        }

        pub fn num_pages(&self) -> u32 {
            self.num_pages_opt()
                .unwrap_or(::std::default::Default::default())
        }
        pub fn clear_title(&mut self) {
            self.title = ::std::default::Default::default();
        }
        pub fn title_mut(&mut self) -> &'_ mut ::std::string::String {
            if !self.has_title() {
                self.title = ::std::default::Default::default();
            }
            &mut self.title
        }
        pub fn clear_num_pages(&mut self) {
            self.num_pages = ::std::default::Default::default();
        }
        pub fn num_pages_mut(&mut self) -> &'_ mut u32 {
            if !self.has_num_pages() {
                self.num_pages = ::std::default::Default::default();
            }
            &mut self.num_pages
        }
    }

    impl super::_puroro_traits::BookTrait for Book {
        fn title_opt<'this>(&'this self) -> Option<&'this str> {
            <self::Book>::title_opt(self)
        }
        fn num_pages_opt<'this>(&'this self) -> Option<u32> {
            <self::Book>::num_pages_opt(self)
        }
    }

    impl ::puroro::MessageRepresentativeImpl for Book {}

    impl ::puroro::internal::de::DeserMessageFromBytesIter for Book {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::internal::types::FieldData<
                &mut ::puroro::internal::de::from_iter::ScopedIter<I>,
            >,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro::internal::impls::simple::de::DeserFieldFromBytesIter;
            match field_number {
            1 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::String
                >::deser_field(&mut self.title, data)
            }
            2 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::UInt32
                >::deser_field(&mut self.num_pages, data)
            }

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::internal::se::SerMessageToIoWrite for Book
    where
        Self: super::_puroro_traits::BookTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::BookTrait>::title_opt(self),
                1,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::UInt32,
            >::ser_field(
                <Self as super::_puroro_traits::BookTrait>::num_pages_opt(self),
                2,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::default::Default for Book {
        fn default() -> Self {
            Self::new()
        }
    }

    impl ::std::fmt::Debug for Book
    where
        Self: super::_puroro_traits::BookTrait,
    {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.debug_struct("Book")
                .field("title", &self.title())
                .field("num_pages", &self.num_pages())
                .finish()
        }
    }

    impl ::std::clone::Clone for Book {
        fn clone(&self) -> Self {
            Self {
                _bitfield: ::std::clone::Clone::clone(&self._bitfield),
                title: ::std::clone::Clone::clone(&self.title),
                num_pages: ::std::clone::Clone::clone(&self.num_pages),
            }
        }
    }

    impl ::std::cmp::PartialEq for Book {
        fn eq(&self, rhs: &Self) -> bool {
            self._bitfield == rhs._bitfield
                && self.title == rhs.title
                && self.num_pages == rhs.num_pages
                && true
        }
    }
}

pub use _puroro_impls::*;
pub mod _puroro_impls {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
    use super::_puroro_traits::*;

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct BookSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub title: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Book> for BookSingleField1<ScalarType> where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::BookTrait for BookSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn title_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::Some(self.title.as_ref())
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for BookSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        Self: super::_puroro_traits::BookTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::BookTrait>::title_opt(self),
                1,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for BookSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self { title: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct BookSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::Into<u32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub num_pages: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Book> for BookSingleField2<ScalarType> where
        ScalarType: ::std::convert::Into<u32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::BookTrait for BookSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::Into<u32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn num_pages_opt<'this>(&'this self) -> ::std::option::Option<u32> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.num_pages,
            )))
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for BookSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::Into<u32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        Self: super::_puroro_traits::BookTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::UInt32,
            >::ser_field(
                <Self as super::_puroro_traits::BookTrait>::num_pages_opt(self),
                2,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for BookSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::Into<u32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self { num_pages: value }
        }
    }
    pub struct BookBumpalo<'bump> {
        _bump: &'bump ::puroro::bumpalo::Bump,
        _bitfield:
            ::puroro::bitvec::array::BitArray<::puroro::bitvec::order::Lsb0, [u32; (0 + 31) / 32]>,
        title: ::puroro::internal::Bare<::puroro::internal::NoAllocBumpString>,
        num_pages: ::puroro::internal::Bare<u32>,
    }

    pub type BookBumpaloOwned = ::puroro::BumpaloOwned<BookBumpalo<'static>>;
    impl<'bump> BookBumpalo<'bump> {
        pub fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
            #[allow(unused)]
            let bump_ref: &::puroro::bumpalo::Bump =
                unsafe { ::std::mem::transmute(::std::ops::Deref::deref(&bump)) };

            Self {
                _bump: bump,
                _bitfield: ::std::default::Default::default(),
                title: ::std::default::Default::default(),
                num_pages: ::std::default::Default::default(),
            }
        }
        pub fn title_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            if !::puroro::internal::IsDefault::is_default(&*self.title) {
                ::std::option::Option::Some(&self.title)
            } else {
                ::std::option::Option::None
            }
        }
        pub fn title<'this>(&'this self) -> &'this str {
            match self.title_opt() {
                ::std::option::Option::Some(x) => x,
                _ => ::std::default::Default::default(),
            }
        }

        pub fn has_title(&self) -> bool {
            self.title_opt().is_some()
        }
        pub fn num_pages_opt<'this>(&'this self) -> ::std::option::Option<u32> {
            if !::puroro::internal::IsDefault::is_default(&*self.num_pages) {
                ::std::option::Option::Some(self.num_pages.inner())
            } else {
                ::std::option::Option::None
            }
        }
        pub fn num_pages<'this>(&'this self) -> u32 {
            match self.num_pages_opt() {
                ::std::option::Option::Some(x) => x,
                _ => ::std::default::Default::default(),
            }
        }

        pub fn has_num_pages(&self) -> bool {
            self.num_pages_opt().is_some()
        }
        pub fn clear_title(&mut self) {
            self.title = ::std::default::Default::default();
        }
        pub fn title_mut<'this>(
            &'this mut self,
        ) -> ::puroro::internal::RefMutBumpString<'bump, 'this> {
            if !self.has_title() {
                self.title = ::std::default::Default::default();
            }
            unsafe { self.title.as_mut_string_in(self._bump) }
        }
        pub fn clear_num_pages(&mut self) {
            self.num_pages = ::std::default::Default::default();
        }
        pub fn num_pages_mut<'this>(&'this mut self) -> &'this mut u32 {
            if !self.has_num_pages() {
                self.num_pages = ::std::default::Default::default();
            }
            &mut self.num_pages
        }
    }
    impl<'bump> ::puroro::Message<super::_puroro_simple_impl::Book> for BookBumpalo<'bump> {}

    impl<'bump> ::puroro::BumpaloMessage<'bump> for BookBumpalo<'bump> {
        fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
            Self::new_in(bump)
        }
    }

    impl<'bump> ::puroro::internal::BumpDefault<'bump> for BookBumpalo<'bump> {
        fn default_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
            Self::new_in(bump)
        }
    }

    impl<'bump> super::_puroro_traits::BookTrait for BookBumpalo<'bump> {
        fn title_opt<'this>(&'this self) -> Option<&'this str> {
            <Self>::title_opt(self)
        }
        fn num_pages_opt<'this>(&'this self) -> Option<u32> {
            <Self>::num_pages_opt(self)
        }
    }

    impl<'bump> ::puroro::internal::de::DeserMessageFromBytesIter for BookBumpalo<'bump> {
        fn deser_field<'this, I>(
            &'this mut self,
            field_number: i32,
            data: ::puroro::internal::types::FieldData<
                &mut ::puroro::internal::de::from_iter::ScopedIter<I>,
            >,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro::internal::impls::bumpalo::de::DeserFieldFromBytesIter;
            match field_number {
            1 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::String
                >::deser_field(&mut self.title, data, self._bump)
            }
            2 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::UInt32
                >::deser_field(&mut self.num_pages, data, self._bump)
            }

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl<'bump> ::puroro::internal::se::SerMessageToIoWrite for BookBumpalo<'bump>
    where
        Self: super::_puroro_traits::BookTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::BookTrait>::title_opt(self),
                1,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::UInt32,
            >::ser_field(
                <Self as super::_puroro_traits::BookTrait>::num_pages_opt(self),
                2,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }
    pub struct BookBuilder<T>(T);

    impl<T> BookBuilder<T>
    where
        T: BookTrait,
    {
        pub fn append_title<ScalarType>(
            self,
            value: ScalarType,
        ) -> BookBuilder<(T, BookSingleField1<ScalarType>)>
        where
            ScalarType: ::std::convert::AsRef<str>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            BookBuilder((self.0, BookSingleField1 { title: value }))
        }

        pub fn append_num_pages<ScalarType>(
            self,
            value: ScalarType,
        ) -> BookBuilder<(T, BookSingleField2<ScalarType>)>
        where
            ScalarType: ::std::convert::Into<u32>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            BookBuilder((self.0, BookSingleField2 { num_pages: value }))
        }

        pub fn build(self) -> T {
            self.0
        }
    }

    impl BookBuilder<()> {
        pub fn new() -> Self {
            Self(())
        }
    }
}
pub use _puroro_traits::*;
pub mod _puroro_traits {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }

    pub trait BookTrait {
        fn title<'this>(&'this self) -> &'this str {
            self.title_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_title<'this>(&'this self) -> bool {
            self.title_opt().is_some()
        }
        fn title_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::None
        }
        fn num_pages<'this>(&'this self) -> u32 {
            self.num_pages_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_num_pages<'this>(&'this self) -> bool {
            self.num_pages_opt().is_some()
        }
        fn num_pages_opt<'this>(&'this self) -> ::std::option::Option<u32> {
            ::std::option::Option::None
        }
    }

    macro_rules! book_delegate {
        ($ty:ty) => {
            fn title_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                (**self).title_opt()
            }
            fn num_pages_opt<'this>(&'this self) -> ::std::option::Option<u32> {
                (**self).num_pages_opt()
            }
        };
    }

    impl<T> BookTrait for &'_ T
    where
        T: BookTrait,
    {
        book_delegate!(T);
    }

    impl<T> BookTrait for &'_ mut T
    where
        T: BookTrait,
    {
        book_delegate!(T);
    }

    impl<T> BookTrait for ::std::boxed::Box<T>
    where
        T: BookTrait,
    {
        book_delegate!(T);
    }

    impl<'bump, T> BookTrait for ::puroro::bumpalo::boxed::Box<'bump, T>
    where
        T: BookTrait,
    {
        book_delegate!(T);
    }

    impl<T> BookTrait for ::puroro::BumpaloOwned<T>
    where
        T: BookTrait,
    {
        book_delegate!(T);
    }
    impl BookTrait for () {}
    impl<T, U> BookTrait for (T, U)
    where
        T: BookTrait,
        U: BookTrait,
    {
        fn title_opt<'this>(&'this self) -> Option<&'this str> {
            <U as BookTrait>::title_opt(&self.1).or_else(|| <T as BookTrait>::title_opt(&self.0))
        }
        fn num_pages_opt<'this>(&'this self) -> Option<u32> {
            <U as BookTrait>::num_pages_opt(&self.1)
                .or_else(|| <T as BookTrait>::num_pages_opt(&self.0))
        }
    }
    impl<T, U> BookTrait for ::puroro::Either<T, U>
    where
        T: BookTrait,
        U: BookTrait,
    {
        fn title_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().either(
                |t| <T as BookTrait>::title_opt(t),
                |u| <U as BookTrait>::title_opt(u),
            )
        }
        fn num_pages_opt<'this>(&'this self) -> ::std::option::Option<u32> {
            self.as_ref().either(
                |t| <T as BookTrait>::num_pages_opt(t),
                |u| <U as BookTrait>::num_pages_opt(u),
            )
        }
    }
    impl<T> BookTrait for ::std::option::Option<T>
    where
        T: BookTrait,
    {
        fn title_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().and_then(|msg| msg.title_opt())
        }
        fn num_pages_opt<'this>(&'this self) -> ::std::option::Option<u32> {
            self.as_ref().and_then(|msg| msg.num_pages_opt())
        }
    }
}
pub use _puroro_nested::*;
pub mod _puroro_nested {
    pub mod book {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }
    }
}
