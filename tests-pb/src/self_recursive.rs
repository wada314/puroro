mod _puroro_root {
    #[allow(unused)]
    pub(crate) use super::super::_puroro_root::*;
}
mod _puroro {
    #[allow(unused)]
    pub(crate) use ::puroro::*;
}
#[derive(::std::default::Default)]
pub struct Msg {
    recursive_unlabeled: self::_puroro::internal::field_type::SingularHeapMessageField::<
        self::_puroro_root::self_recursive::Msg,
    >,
    _bitfield: self::_puroro::internal::bitvec::BitArray<0usize>,
}
impl Msg {
    pub fn recursive_unlabeled(
        &self,
    ) -> ::std::option::Option::<&self::_puroro_root::self_recursive::Msg> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField::<
            self::_puroro_root::self_recursive::Msg,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.recursive_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn recursive_unlabeled_opt(
        &self,
    ) -> ::std::option::Option::<&self::_puroro_root::self_recursive::Msg> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField::<
            self::_puroro_root::self_recursive::Msg,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.recursive_unlabeled,
            &self._bitfield,
        )
    }
    pub fn recursive_unlabeled_mut(
        &mut self,
    ) -> &mut self::_puroro_root::self_recursive::Msg {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField::<
            self::_puroro_root::self_recursive::Msg,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.recursive_unlabeled,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_recursive_unlabeled(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField::<
            self::_puroro_root::self_recursive::Msg,
        > as NonRepeatedFieldType>::get_field_opt(
                &self.recursive_unlabeled,
                &self._bitfield,
            )
            .is_some()
    }
    pub fn clear_recursive_unlabeled(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField::<
            self::_puroro_root::self_recursive::Msg,
        > as NonRepeatedFieldType>::clear(
            &mut self.recursive_unlabeled,
            &mut self._bitfield,
        )
    }
}
impl self::_puroro::Message for Msg {
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
        use self::_puroro::internal::ser::FieldData;
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion as _;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            match number {
                1i32 => {
                    <self::_puroro::internal::field_type::SingularHeapMessageField::<
                        self::_puroro_root::self_recursive::Msg,
                    > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.recursive_unlabeled,
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
        use self::_puroro::internal::oneof_type::OneofUnion as _;
        <self::_puroro::internal::field_type::SingularHeapMessageField::<
            self::_puroro_root::self_recursive::Msg,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.recursive_unlabeled,
            &self._bitfield,
            1i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for Msg {
    fn clone(&self) -> Self {
        Self {
            recursive_unlabeled: <self::_puroro::internal::field_type::SingularHeapMessageField::<
                self::_puroro_root::self_recursive::Msg,
            > as ::std::clone::Clone>::clone(&self.recursive_unlabeled),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
impl ::std::ops::Drop for Msg {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for Msg {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct(stringify!(Msg))
            .field(stringify!(recursive_unlabeled), &self.recursive_unlabeled_opt())
            .finish()
    }
}
impl ::std::cmp::PartialEq for Msg {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion as _;
        true && self.recursive_unlabeled_opt() == rhs.recursive_unlabeled_opt()
    }
}
