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
    num_pages: self::_puroro::internal::field_types::SingularNumericField<u32, ()>,

    _bitfield: self::_puroro::bitvec::array::BitArray<[u32; 0], self::_puroro::bitvec::order::Lsb0>,
}

pub struct BookOwned {}

pub mod book {

    mod _puroro {
        pub use super::super::_puroro::*;
    }
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
}
