// A generated source code by puroro library
// package library

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub mod _puroro {
    pub use ::puroro::*;
}

pub struct Book {
    // Optional, LengthDelimited(String)
    title: self::_puroro::internal::field_types::OptionalStringField<0>,
    // Optional, Variant(UInt32)
    num_pages: self::_puroro::internal::field_types::OptionalNumericField<u32, (), 1>,

    _bitfield: self::_puroro::bitvec::array::BitArray<[u32; 1], self::_puroro::bitvec::order::Lsb0>,
}

impl Book {
    // Optional, LengthDelimited(String)
    pub fn title(&self) -> &str {
        <self::_puroro::internal::field_types::OptionalStringField<0> as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.title, &self._bitfield,
        )
    }
    // Optional, Variant(UInt32)
    pub fn num_pages(&self) -> u32 {
        <self::_puroro::internal::field_types::OptionalNumericField<u32, (), 1> as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.num_pages, &self._bitfield,
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
