// A generated source code by puroro library
// package self_recursive

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub mod _puroro {
    pub use ::puroro::*;
}

#[derive(Default, Clone)]
pub struct Msg {
    // Singular, LengthDelimited(Message(Fqtn(".self_recursive.Msg")))
    recursive_unlabeled: self::_puroro::internal::field_types::SingularHeapMessageField<
        _puroro_root::self_recursive::Msg,
    >,

    _bitfield: self::_puroro::bitvec::BitArray<0>,
}

impl Msg {
    // Singular, LengthDelimited(Message(Fqtn(".self_recursive.Msg")))
    pub fn recursive_unlabeled(&self) -> Option<&_puroro_root::self_recursive::Msg> {
        <self::_puroro::internal::field_types::SingularHeapMessageField<
            _puroro_root::self_recursive::Msg,
        > as self::_puroro::internal::field_types::NonRepeatedFieldType>::get_field(
            &self.recursive_unlabeled,
            &self._bitfield,
            ::std::default::Default::default(),
        )
    }
}

impl self::_puroro::Message for Msg {
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
        while let Some((number, field_data)) =
            self::_puroro::internal::ser::FieldData::from_bytes_iter(iter.by_ref())?
        {
            match number {
                1 => <self::_puroro::internal::field_types::SingularHeapMessageField<
                    _puroro_root::self_recursive::Msg,
                > as self::_puroro::internal::field_types::FieldType>::deser_from_iter(
                    &mut self.recursive_unlabeled,
                    &mut self._bitfield,
                    field_data,
                )?,
                _ => todo!(),
            }
        }
        Ok(())
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
