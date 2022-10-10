// A generated source code by puroro library
// package self_recursive

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub mod _puroro {
    pub use ::puroro::*;
}

#[derive(Default)]
pub struct Msg {
    // Singular, LengthDelimited(Message(Fqtn(".self_recursive.Msg")))
    recursive_unlabeled: self::_puroro::internal::field_type::SingularHeapMessageField<
        _puroro_root::self_recursive::Msg,
    >,

    _bitfield: self::_puroro::bitvec::BitArray<0>,
}

impl Msg {
    // Singular, LengthDelimited(Message(Fqtn(".self_recursive.Msg")))
    pub fn recursive_unlabeled(&self) -> ::std::option::Option<&_puroro_root::self_recursive::Msg> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::self_recursive::Msg,
        > as NonRepeatedFieldType>::get_field(
            &self.recursive_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn recursive_unlabeled_opt(
        &self,
    ) -> ::std::option::Option<&_puroro_root::self_recursive::Msg> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::self_recursive::Msg,
        > as NonRepeatedFieldType>::get_field_opt(&self.recursive_unlabeled, &self._bitfield)
    }
    pub fn has_recursive_unlabeled(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::self_recursive::Msg,
        > as NonRepeatedFieldType>::get_field_opt(&self.recursive_unlabeled, &self._bitfield)
        .is_some()
    }
    pub fn recursive_unlabeled_mut(&mut self) -> &mut _puroro_root::self_recursive::Msg {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::self_recursive::Msg,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.recursive_unlabeled,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_recursive_unlabeled(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::self_recursive::Msg,
        > as NonRepeatedFieldType>::clear(&mut self.recursive_unlabeled, &mut self._bitfield)
    }
}

impl self::_puroro::Message for Msg {
    fn from_bytes_iter<I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>>(
        iter: I,
    ) -> self::_puroro::Result<Self> {
        #[allow(unused)]
        use ::std::result::Result::Ok;
        let mut msg: Self = ::std::default::Default::default();
        msg.merge_from_bytes_iter(iter)?;
        Ok(msg)
    }

    fn merge_from_bytes_iter<I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>>(
        &mut self,
        mut iter: I,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use ::std::option::Option::Some;
        #[allow(unused)]
        use ::std::result::Result::Ok;
        while let Some((number, _field_data)) =
            self::_puroro::internal::ser::FieldData::from_bytes_iter(iter.by_ref())?
        {
            match number {
                1 => <self::_puroro::internal::field_type::SingularHeapMessageField<
                    _puroro_root::self_recursive::Msg,
                > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                    &mut self.recursive_unlabeled,
                    &mut self._bitfield,
                    _field_data,
                )?,
                _ => todo!(),
            }
        }
        Ok(())
    }

    fn to_bytes<W: ::std::io::Write>(&self, out: &mut W) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use ::std::result::Result::Ok;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::self_recursive::Msg,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.recursive_unlabeled,
            &self._bitfield,
            1,
            out,
        )?;
        Ok(())
    }
}

impl ::std::clone::Clone for Msg {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        #[allow(unused)]
        use ::std::clone::Clone;
        Self {
            recursive_unlabeled: Clone::clone(&self.recursive_unlabeled),

            _bitfield: Clone::clone(&self._bitfield),
        }
    }
}

impl ::std::fmt::Debug for Msg {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct("Msg")
            .field("recursive_unlabeled", &self.recursive_unlabeled())
            .finish()
    }
}

impl ::std::cmp::PartialEq for Msg {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;

        true && self.recursive_unlabeled_opt() == rhs.recursive_unlabeled_opt()
    }
}

impl ::std::ops::Drop for Msg {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
    }
}
