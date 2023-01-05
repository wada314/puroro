mod _root {
    #[allow(unused)]
    pub(crate) use super::super::_root::*;
}
mod _puroro {
    #[allow(unused)]
    pub(crate) use ::puroro::*;
}
mod _pinternal {
    #[allow(unused)]
    pub(crate) use ::puroro::internal::*;
}
pub mod msg;
#[derive(::std::default::Default)]
pub struct Msg {
    fields: self::_root::nested::_fields::MsgFields<
        self::_pinternal::SingularNumericalField::<i32, self::_pinternal::tags::Int32>,
    >,
    _bitfield: self::_pinternal::BitArray<0usize>,
}
impl Msg {
    pub fn item_outer(&self) -> i32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.item_outer,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn item_outer_opt(&self) -> ::std::option::Option::<i32> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
        > as NonRepeatedFieldType>::get_field_opt(&self.item_outer, &self._bitfield)
    }
    pub fn item_outer_mut(&mut self) -> &mut i32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.item_outer,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_item_outer(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
        > as NonRepeatedFieldType>::get_field_opt(&self.item_outer, &self._bitfield)
            .is_some()
    }
    pub fn clear_item_outer(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
        > as NonRepeatedFieldType>::clear(&mut self.item_outer, &mut self._bitfield)
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
        use self::_pinternal::ser::FieldData;
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            match number {
                1i32 => {
                    <self::_pinternal::SingularNumericalField::<
                        i32,
                        self::_pinternal::tags::Int32,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.item_outer,
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
        use self::_pinternal::OneofUnion as _;
        <self::_pinternal::SingularNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.item_outer,
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
            item_outer: <self::_pinternal::SingularNumericalField::<
                i32,
                self::_pinternal::tags::Int32,
            > as ::std::clone::Clone>::clone(&self.item_outer),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
impl ::std::ops::Drop for Msg {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for Msg {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct(stringify!(Msg))
            .field(stringify!(item_outer), &self.item_outer_opt())
            .finish()
    }
}
impl ::std::cmp::PartialEq for Msg {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        true && self.item_outer_opt() == rhs.item_outer_opt()
    }
}
pub mod _fields {
    mod _root {
        #[allow(unused)]
        pub use super::super::_root::*;
    }
    mod _puroro {
        #[allow(unused)]
        pub use ::puroro::*;
    }
    mod _pinternal {
        #[allow(unused)]
        pub use ::puroro::internal::*;
    }
    pub struct MsgFields<TItemOuter> {
        pub item_outer: TItemOuter,
    }
}
pub use self::_fields::*;
