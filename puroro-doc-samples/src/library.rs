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
pub struct Book {
    fields: self::_root::library::_fields::BookFields<
        self::_pinternal::SingularUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        >,
        self::_pinternal::SingularNumericalField::<u32, self::_pinternal::tags::UInt32>,
        self::_pinternal::SingularHeapMessageField::<self::_root::library::Author>,
    >,
    bitfield: self::_pinternal::BitArray<0usize>,
}
impl Book {
    pub fn title(&self) -> &str {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.fields.title,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn title_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        > as NonRepeatedFieldType>::get_field_opt(&self.fields.title, &self.bitfield)
    }
    pub fn title_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.fields.title,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_title(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        > as NonRepeatedFieldType>::get_field_opt(&self.fields.title, &self.bitfield)
            .is_some()
    }
    pub fn clear_title(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        > as NonRepeatedFieldType>::clear(&mut self.fields.title, &mut self.bitfield)
    }
    pub fn num_pages(&self) -> u32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            u32,
            self::_pinternal::tags::UInt32,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.fields.num_pages,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn num_pages_opt(&self) -> ::std::option::Option::<u32> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            u32,
            self::_pinternal::tags::UInt32,
        > as NonRepeatedFieldType>::get_field_opt(&self.fields.num_pages, &self.bitfield)
    }
    pub fn num_pages_mut(&mut self) -> &mut u32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            u32,
            self::_pinternal::tags::UInt32,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.fields.num_pages,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_num_pages(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            u32,
            self::_pinternal::tags::UInt32,
        > as NonRepeatedFieldType>::get_field_opt(&self.fields.num_pages, &self.bitfield)
            .is_some()
    }
    pub fn clear_num_pages(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            u32,
            self::_pinternal::tags::UInt32,
        > as NonRepeatedFieldType>::clear(&mut self.fields.num_pages, &mut self.bitfield)
    }
    pub fn author(&self) -> ::std::option::Option::<&self::_root::library::Author> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularHeapMessageField::<
            self::_root::library::Author,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.fields.author,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn author_opt(&self) -> ::std::option::Option::<&self::_root::library::Author> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularHeapMessageField::<
            self::_root::library::Author,
        > as NonRepeatedFieldType>::get_field_opt(&self.fields.author, &self.bitfield)
    }
    pub fn author_mut(&mut self) -> &mut self::_root::library::Author {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularHeapMessageField::<
            self::_root::library::Author,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.fields.author,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_author(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularHeapMessageField::<
            self::_root::library::Author,
        > as NonRepeatedFieldType>::get_field_opt(&self.fields.author, &self.bitfield)
            .is_some()
    }
    pub fn clear_author(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularHeapMessageField::<
            self::_root::library::Author,
        > as NonRepeatedFieldType>::clear(&mut self.fields.author, &mut self.bitfield)
    }
}
impl self::_puroro::Message for Book {
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
                    <self::_pinternal::SingularUnsizedField::<
                        ::std::string::String,
                        self::_pinternal::tags::String,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.fields.title,
                        &mut self.bitfield,
                        field_data,
                    )?
                }
                2i32 => {
                    <self::_pinternal::SingularNumericalField::<
                        u32,
                        self::_pinternal::tags::UInt32,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.fields.num_pages,
                        &mut self.bitfield,
                        field_data,
                    )?
                }
                3i32 => {
                    <self::_pinternal::SingularHeapMessageField::<
                        self::_root::library::Author,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.fields.author,
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
        <self::_pinternal::SingularUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.fields.title,
            &self.bitfield,
            1i32,
            out,
        )?;
        <self::_pinternal::SingularNumericalField::<
            u32,
            self::_pinternal::tags::UInt32,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.fields.num_pages,
            &self.bitfield,
            2i32,
            out,
        )?;
        <self::_pinternal::SingularHeapMessageField::<
            self::_root::library::Author,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.fields.author,
            &self.bitfield,
            3i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for Book {
    fn clone(&self) -> Self {
        Self {
            fields: self::_fields::BookFields {
                title: <self::_pinternal::SingularUnsizedField::<
                    ::std::string::String,
                    self::_pinternal::tags::String,
                > as ::std::clone::Clone>::clone(&self.fields.title),
                num_pages: <self::_pinternal::SingularNumericalField::<
                    u32,
                    self::_pinternal::tags::UInt32,
                > as ::std::clone::Clone>::clone(&self.fields.num_pages),
                author: <self::_pinternal::SingularHeapMessageField::<
                    self::_root::library::Author,
                > as ::std::clone::Clone>::clone(&self.fields.author),
            },
            bitfield: ::std::clone::Clone::clone(&self.bitfield),
        }
    }
}
impl ::std::ops::Drop for Book {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for Book {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct(stringify!(Book))
            .field(stringify!(title), &self.title_opt())
            .field(stringify!(num_pages), &self.num_pages_opt())
            .field(stringify!(author), &self.author_opt())
            .finish()
    }
}
impl ::std::cmp::PartialEq for Book {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        true && self.title_opt() == rhs.title_opt()
            && self.num_pages_opt() == rhs.num_pages_opt()
            && self.author_opt() == rhs.author_opt()
    }
}
#[derive(::std::default::Default)]
pub struct Author {
    fields: self::_root::library::_fields::AuthorFields<
        self::_pinternal::SingularUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        >,
    >,
    bitfield: self::_pinternal::BitArray<0usize>,
}
impl Author {
    pub fn name(&self) -> &str {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.fields.name,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn name_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        > as NonRepeatedFieldType>::get_field_opt(&self.fields.name, &self.bitfield)
    }
    pub fn name_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.fields.name,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_name(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        > as NonRepeatedFieldType>::get_field_opt(&self.fields.name, &self.bitfield)
            .is_some()
    }
    pub fn clear_name(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        > as NonRepeatedFieldType>::clear(&mut self.fields.name, &mut self.bitfield)
    }
}
impl self::_puroro::Message for Author {
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
                    <self::_pinternal::SingularUnsizedField::<
                        ::std::string::String,
                        self::_pinternal::tags::String,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.fields.name,
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
        <self::_pinternal::SingularUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.fields.name,
            &self.bitfield,
            1i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for Author {
    fn clone(&self) -> Self {
        Self {
            fields: self::_fields::AuthorFields {
                name: <self::_pinternal::SingularUnsizedField::<
                    ::std::string::String,
                    self::_pinternal::tags::String,
                > as ::std::clone::Clone>::clone(&self.fields.name),
            },
            bitfield: ::std::clone::Clone::clone(&self.bitfield),
        }
    }
}
impl ::std::ops::Drop for Author {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for Author {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct(stringify!(Author))
            .field(stringify!(name), &self.name_opt())
            .finish()
    }
}
impl ::std::cmp::PartialEq for Author {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        true && self.name_opt() == rhs.name_opt()
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
pub use self::_fields::*;
