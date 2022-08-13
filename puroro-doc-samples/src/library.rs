// A generated source code by puroro library
// package library

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub mod _puroro {
    pub use ::puroro::*;
}

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

    _bitfield: self::_puroro::bitvec::array::BitArray<[u32; 0], self::_puroro::bitvec::order::Lsb0>,
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

pub mod _book {

    mod _puroro {
        pub use super::super::_puroro::*;
    }
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
}

pub struct Author {
    // Singular, LengthDelimited(String)
    name: self::_puroro::internal::field_types::SingularStringField,

    _bitfield: self::_puroro::bitvec::array::BitArray<[u32; 0], self::_puroro::bitvec::order::Lsb0>,
}

impl Author {
    // Singular, LengthDelimited(String)
    pub fn name(&self) -> &str {
        <self::_puroro::internal::field_types::SingularStringField as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.name, &self._bitfield,
        )
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
