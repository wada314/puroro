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
    title: self::_puroro::internal::field_types::SingularStringField,
    // Singular, Variant(UInt32)
    num_pages: self::_puroro::internal::field_types::SingularVariantField<
        u32,
        self::_puroro::tags::UInt32,
    >,
    // Singular, LengthDelimited(Message(Fqtn(".library.Author")))
    author: self::_puroro::internal::field_types::SingularHeapMessageField<
        _puroro_root::library::Author,
    >,

    _bitfield: self::_puroro::bitvec::BitArray<0>,
}

impl Book {
    // Singular, LengthDelimited(String)
    pub fn title(&self) -> &str {
        <self::_puroro::internal::field_types::SingularStringField as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.title, &self._bitfield,
        )
    }
    // Singular, Variant(UInt32)
    pub fn num_pages(&self) -> u32 {
        <self::_puroro::internal::field_types::SingularVariantField<u32, self::_puroro::tags::UInt32> as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.num_pages, &self._bitfield,
        )
    }
    // Singular, LengthDelimited(Message(Fqtn(".library.Author")))
    pub fn author(&self) -> Option<&_puroro_root::library::Author> {
        <self::_puroro::internal::field_types::SingularHeapMessageField<
            _puroro_root::library::Author,
        > as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.author,
            &self._bitfield,
        )
    }
}

impl self::_puroro::Message for Book {
    fn from_bytes_iter<I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>>(
        iter: I,
    ) -> self::_puroro::Result<Self> {
        let mut msg: Self = ::std::default::Default::default();
        let mut peekable = iter.peekable();
        while peekable.peek().is_some() {
            let (number, field_data) =
                self::_puroro::internal::ser::FieldData::from_bytes_iter(peekable.by_ref())?;
            match number {
                1 => <
                    self::_puroro::internal::field_types::SingularStringField as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.title,
                    &mut msg._bitfield,
                    field_data,
                )?,
                2 => <
                    self::_puroro::internal::field_types::SingularVariantField<u32, self::_puroro::tags::UInt32> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.num_pages,
                    &mut msg._bitfield,
                    field_data,
                )?,
                3 => <
                    self::_puroro::internal::field_types::SingularHeapMessageField<_puroro_root::library::Author> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.author,
                    &mut msg._bitfield,
                    field_data,
                )?,
                _ => todo!(),
            }
        }
        Ok(msg)
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
    name: self::_puroro::internal::field_types::SingularStringField,

    _bitfield: self::_puroro::bitvec::BitArray<0>,
}

impl Author {
    // Singular, LengthDelimited(String)
    pub fn name(&self) -> &str {
        <self::_puroro::internal::field_types::SingularStringField as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.name, &self._bitfield,
        )
    }
}

impl self::_puroro::Message for Author {
    fn from_bytes_iter<I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>>(
        iter: I,
    ) -> self::_puroro::Result<Self> {
        let mut msg: Self = ::std::default::Default::default();
        let mut peekable = iter.peekable();
        while peekable.peek().is_some() {
            let (number, field_data) =
                self::_puroro::internal::ser::FieldData::from_bytes_iter(peekable.by_ref())?;
            match number {
                1 => <
                    self::_puroro::internal::field_types::SingularStringField as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.name,
                    &mut msg._bitfield,
                    field_data,
                )?,
                _ => todo!(),
            }
        }
        Ok(msg)
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
