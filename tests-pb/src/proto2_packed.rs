// A generated source code by puroro library
// package proto2_packed

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub mod _puroro {
    pub use ::puroro::*;
}

#[derive(Default, Clone)]
pub struct Msg {
    _bitfield: self::_puroro::bitvec::BitArray<0>,
}

impl Msg {}

impl self::_puroro::Message for Msg {
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
                    self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.explicitly_packed,
                    &mut msg._bitfield,
                    field_data,
                )?,
                2 => <
                    self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.explicitly_not_packed,
                    &mut msg._bitfield,
                    field_data,
                )?,
                3 => <
                    self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.not_annotated,
                    &mut msg._bitfield,
                    field_data,
                )?,
                _ => todo!(),
            }
        }
        Ok(msg)
    }
}

pub mod _msg {

    mod _puroro {
        pub use super::super::_puroro::*;
    }
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
}
