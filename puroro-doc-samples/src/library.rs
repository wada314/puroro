// A generated source code by puroro library
// package library

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub mod _puroro {
    pub use ::puroro::*;
}

pub struct Book<T>(T);

impl<T: self::_puroro::GenericMessage> Book<T> {
    pub fn title(&self) -> () {
        todo!()
    }

    // title

    pub fn num_pages(&self) -> () {
        todo!()
    }

    // num_pages
}

pub struct BookOwned {
    _bitfield: self::_puroro::bitvec::array::BitArray<[u32; 4], self::_puroro::bitvec::order::Lsb0>,
}
