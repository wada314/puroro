// A generated source code by puroro library
// package library

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub mod _puroro {
    pub use ::puroro::*;
}

#[derive(Default, Clone)]
pub struct Book {
    // Singular, LengthDelimited(String)
    title: self::_puroro::internal::field_type::SingularStringField,
    // Singular, Variant(UInt32)
    num_pages: self::_puroro::internal::field_type::SingularNumericalField<
        u32,
        self::_puroro::tags::UInt32,
    >,
    // Singular, LengthDelimited(Message(Fqtn(".library.Author")))
    author: self::_puroro::internal::field_type::SingularHeapMessageField<
        _puroro_root::library::Author,
    >,

    _bitfield: self::_puroro::bitvec::BitArray<0>,
}

impl Book {
    // Singular, LengthDelimited(String)
    pub fn title(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularStringField as NonRepeatedFieldType>::get_field(
            &self.title, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn title_opt(&self) -> ::std::option::Option<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularStringField as NonRepeatedFieldType>::get_field_opt(
            &self.title, &self._bitfield,
        )
    }
    pub fn has_title(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularStringField as NonRepeatedFieldType>::get_field_opt(
            &self.title, &self._bitfield,
        ).is_some()
    }
    pub fn title_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularStringField as NonRepeatedFieldType>::mut_field(
            &mut self.title, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_title(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularStringField as NonRepeatedFieldType>::clear(
            &mut self.title,
            &mut self._bitfield,
        )
    }
    // Singular, Variant(UInt32)
    pub fn num_pages(&self) -> u32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            u32,
            self::_puroro::tags::UInt32,
        > as NonRepeatedFieldType>::get_field(
            &self.num_pages,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn num_pages_opt(&self) -> ::std::option::Option<u32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            u32,
            self::_puroro::tags::UInt32,
        > as NonRepeatedFieldType>::get_field_opt(&self.num_pages, &self._bitfield)
    }
    pub fn has_num_pages(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            u32,
            self::_puroro::tags::UInt32,
        > as NonRepeatedFieldType>::get_field_opt(&self.num_pages, &self._bitfield)
        .is_some()
    }
    pub fn num_pages_mut(&mut self) -> &mut u32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            u32,
            self::_puroro::tags::UInt32,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.num_pages,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_num_pages(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            u32,
            self::_puroro::tags::UInt32,
        > as NonRepeatedFieldType>::clear(&mut self.num_pages, &mut self._bitfield)
    }
    // Singular, LengthDelimited(Message(Fqtn(".library.Author")))
    pub fn author(&self) -> Option<&_puroro_root::library::Author> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<_puroro_root::library::Author> as NonRepeatedFieldType>::get_field(
            &self.author, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn author_opt(&self) -> ::std::option::Option<&_puroro_root::library::Author> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<_puroro_root::library::Author> as NonRepeatedFieldType>::get_field_opt(
            &self.author, &self._bitfield,
        )
    }
    pub fn has_author(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<_puroro_root::library::Author> as NonRepeatedFieldType>::get_field_opt(
            &self.author, &self._bitfield,
        ).is_some()
    }
    pub fn author_mut(&mut self) -> &mut _puroro_root::library::Author {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<_puroro_root::library::Author> as NonRepeatedFieldType>::mut_field(
            &mut self.author, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_author(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<_puroro_root::library::Author> as NonRepeatedFieldType>::clear(
            &mut self.author, &mut self._bitfield,
        )
    }
}

impl self::_puroro::Message for Book {
    fn from_bytes_iter<I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>>(
        iter: I,
    ) -> self::_puroro::Result<Self> {
        let mut msg: Self = ::std::default::Default::default();
        msg.merge_from_bytes_iter(iter)?;
        Ok(msg)
    }

    fn merge_from_bytes_iter<I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>>(
        &mut self,
        mut iter: I,
    ) -> self::_puroro::Result<()> {
        while let Some((number, _field_data)) =
            self::_puroro::internal::ser::FieldData::from_bytes_iter(iter.by_ref())?
        {
            match number {
                1 => <
                    self::_puroro::internal::field_type::SingularStringField as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.title,
                    &mut self._bitfield,
                    _field_data,
                )?,
                2 => <
                    self::_puroro::internal::field_type::SingularNumericalField<u32, self::_puroro::tags::UInt32> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.num_pages,
                    &mut self._bitfield,
                    _field_data,
                )?,
                3 => <
                    self::_puroro::internal::field_type::SingularHeapMessageField<_puroro_root::library::Author> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.author,
                    &mut self._bitfield,
                    _field_data,
                )?,
                _ => todo!(),
            }
        }
        Ok(())
    }
}
pub mod _book {

    mod _puroro {
        pub use super::super::_puroro::*;
    }
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
}

#[derive(Default, Clone)]
pub struct Author {
    // Singular, LengthDelimited(String)
    name: self::_puroro::internal::field_type::SingularStringField,

    _bitfield: self::_puroro::bitvec::BitArray<0>,
}

impl Author {
    // Singular, LengthDelimited(String)
    pub fn name(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularStringField as NonRepeatedFieldType>::get_field(
            &self.name, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn name_opt(&self) -> ::std::option::Option<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularStringField as NonRepeatedFieldType>::get_field_opt(
            &self.name, &self._bitfield,
        )
    }
    pub fn has_name(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularStringField as NonRepeatedFieldType>::get_field_opt(
            &self.name, &self._bitfield,
        ).is_some()
    }
    pub fn name_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularStringField as NonRepeatedFieldType>::mut_field(
            &mut self.name, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_name(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularStringField as NonRepeatedFieldType>::clear(
            &mut self.name,
            &mut self._bitfield,
        )
    }
}

impl self::_puroro::Message for Author {
    fn from_bytes_iter<I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>>(
        iter: I,
    ) -> self::_puroro::Result<Self> {
        let mut msg: Self = ::std::default::Default::default();
        msg.merge_from_bytes_iter(iter)?;
        Ok(msg)
    }

    fn merge_from_bytes_iter<I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>>(
        &mut self,
        mut iter: I,
    ) -> self::_puroro::Result<()> {
        while let Some((number, _field_data)) =
            self::_puroro::internal::ser::FieldData::from_bytes_iter(iter.by_ref())?
        {
            match number {
                1 => <
                    self::_puroro::internal::field_type::SingularStringField as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.name,
                    &mut self._bitfield,
                    _field_data,
                )?,
                _ => todo!(),
            }
        }
        Ok(())
    }
}
pub mod _author {

    mod _puroro {
        pub use super::super::_puroro::*;
    }
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
}
