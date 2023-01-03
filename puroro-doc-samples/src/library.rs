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
    title: self::_pinternal::field_type::SingularUnsizedField::<
        ::std::string::String,
        self::_pinternal::tags::String,
    >,
    num_pages: self::_pinternal::field_type::SingularNumericalField::<
        u32,
        self::_pinternal::tags::UInt32,
    >,
    author: self::_pinternal::field_type::SingularHeapMessageField::<
        self::_root::library::Author,
    >,
    _bitfield: self::_pinternal::bitvec::BitArray<0usize>,
}
impl Book {
    pub fn title(&self) -> &str {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.title,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn title_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        > as NonRepeatedFieldType>::get_field_opt(&self.title, &self._bitfield)
    }
    pub fn title_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.title,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_title(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        > as NonRepeatedFieldType>::get_field_opt(&self.title, &self._bitfield)
            .is_some()
    }
    pub fn clear_title(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        > as NonRepeatedFieldType>::clear(&mut self.title, &mut self._bitfield)
    }
    pub fn num_pages(&self) -> u32 {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularNumericalField::<
            u32,
            self::_pinternal::tags::UInt32,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.num_pages,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn num_pages_opt(&self) -> ::std::option::Option::<u32> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularNumericalField::<
            u32,
            self::_pinternal::tags::UInt32,
        > as NonRepeatedFieldType>::get_field_opt(&self.num_pages, &self._bitfield)
    }
    pub fn num_pages_mut(&mut self) -> &mut u32 {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularNumericalField::<
            u32,
            self::_pinternal::tags::UInt32,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.num_pages,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_num_pages(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularNumericalField::<
            u32,
            self::_pinternal::tags::UInt32,
        > as NonRepeatedFieldType>::get_field_opt(&self.num_pages, &self._bitfield)
            .is_some()
    }
    pub fn clear_num_pages(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularNumericalField::<
            u32,
            self::_pinternal::tags::UInt32,
        > as NonRepeatedFieldType>::clear(&mut self.num_pages, &mut self._bitfield)
    }
    pub fn author(&self) -> ::std::option::Option::<&self::_root::library::Author> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::library::Author,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.author,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn author_opt(&self) -> ::std::option::Option::<&self::_root::library::Author> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::library::Author,
        > as NonRepeatedFieldType>::get_field_opt(&self.author, &self._bitfield)
    }
    pub fn author_mut(&mut self) -> &mut self::_root::library::Author {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::library::Author,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.author,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_author(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::library::Author,
        > as NonRepeatedFieldType>::get_field_opt(&self.author, &self._bitfield)
            .is_some()
    }
    pub fn clear_author(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::library::Author,
        > as NonRepeatedFieldType>::clear(&mut self.author, &mut self._bitfield)
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
        use self::_pinternal::oneof_type::OneofUnion as _;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            match number {
                1i32 => {
                    <self::_pinternal::field_type::SingularUnsizedField::<
                        ::std::string::String,
                        self::_pinternal::tags::String,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.title,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                2i32 => {
                    <self::_pinternal::field_type::SingularNumericalField::<
                        u32,
                        self::_pinternal::tags::UInt32,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.num_pages,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                3i32 => {
                    <self::_pinternal::field_type::SingularHeapMessageField::<
                        self::_root::library::Author,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.author,
                        &mut self._bitfield,
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
        use self::_pinternal::oneof_type::OneofUnion as _;
        <self::_pinternal::field_type::SingularUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.title,
            &self._bitfield,
            1i32,
            out,
        )?;
        <self::_pinternal::field_type::SingularNumericalField::<
            u32,
            self::_pinternal::tags::UInt32,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.num_pages,
            &self._bitfield,
            2i32,
            out,
        )?;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::library::Author,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.author,
            &self._bitfield,
            3i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for Book {
    fn clone(&self) -> Self {
        Self {
            title: <self::_pinternal::field_type::SingularUnsizedField::<
                ::std::string::String,
                self::_pinternal::tags::String,
            > as ::std::clone::Clone>::clone(&self.title),
            num_pages: <self::_pinternal::field_type::SingularNumericalField::<
                u32,
                self::_pinternal::tags::UInt32,
            > as ::std::clone::Clone>::clone(&self.num_pages),
            author: <self::_pinternal::field_type::SingularHeapMessageField::<
                self::_root::library::Author,
            > as ::std::clone::Clone>::clone(&self.author),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
impl ::std::ops::Drop for Book {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
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
        use self::_pinternal::oneof_type::OneofUnion as _;
        true && self.title_opt() == rhs.title_opt()
            && self.num_pages_opt() == rhs.num_pages_opt()
            && self.author_opt() == rhs.author_opt()
    }
}
#[derive(::std::default::Default)]
pub struct Author {
    name: self::_pinternal::field_type::SingularUnsizedField::<
        ::std::string::String,
        self::_pinternal::tags::String,
    >,
    _bitfield: self::_pinternal::bitvec::BitArray<0usize>,
}
impl Author {
    pub fn name(&self) -> &str {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.name,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn name_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        > as NonRepeatedFieldType>::get_field_opt(&self.name, &self._bitfield)
    }
    pub fn name_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.name,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_name(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        > as NonRepeatedFieldType>::get_field_opt(&self.name, &self._bitfield)
            .is_some()
    }
    pub fn clear_name(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        > as NonRepeatedFieldType>::clear(&mut self.name, &mut self._bitfield)
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
        use self::_pinternal::oneof_type::OneofUnion as _;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            match number {
                1i32 => {
                    <self::_pinternal::field_type::SingularUnsizedField::<
                        ::std::string::String,
                        self::_pinternal::tags::String,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.name,
                        &mut self._bitfield,
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
        use self::_pinternal::oneof_type::OneofUnion as _;
        <self::_pinternal::field_type::SingularUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.name,
            &self._bitfield,
            1i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for Author {
    fn clone(&self) -> Self {
        Self {
            name: <self::_pinternal::field_type::SingularUnsizedField::<
                ::std::string::String,
                self::_pinternal::tags::String,
            > as ::std::clone::Clone>::clone(&self.name),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
impl ::std::ops::Drop for Author {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
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
        use self::_pinternal::oneof_type::OneofUnion as _;
        true && self.name_opt() == rhs.name_opt()
    }
}
