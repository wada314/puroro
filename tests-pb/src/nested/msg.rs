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
#[derive(::std::default::Default)]
pub struct Submsg {
    fields: self::_root::nested::msg::_fields::SubmsgFields<
        self::_pinternal::SingularNumericalField::<i32, self::_pinternal::tags::Int32>,
    >,
    _bitfield: self::_pinternal::BitArray<0usize>,
}
impl Submsg {
    pub fn item_inner(&self) -> i32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.item_inner,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn item_inner_opt(&self) -> ::std::option::Option::<i32> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
        > as NonRepeatedFieldType>::get_field_opt(&self.item_inner, &self._bitfield)
    }
    pub fn item_inner_mut(&mut self) -> &mut i32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.item_inner,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_item_inner(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
        > as NonRepeatedFieldType>::get_field_opt(&self.item_inner, &self._bitfield)
            .is_some()
    }
    pub fn clear_item_inner(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
        > as NonRepeatedFieldType>::clear(&mut self.item_inner, &mut self._bitfield)
    }
}
impl self::_puroro::Message for Submsg {
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
                        &mut self.item_inner,
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
            &self.item_inner,
            &self._bitfield,
            1i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for Submsg {
    fn clone(&self) -> Self {
        Self {
            item_inner: <self::_pinternal::SingularNumericalField::<
                i32,
                self::_pinternal::tags::Int32,
            > as ::std::clone::Clone>::clone(&self.item_inner),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
impl ::std::ops::Drop for Submsg {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for Submsg {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct(stringify!(Submsg))
            .field(stringify!(item_inner), &self.item_inner_opt())
            .finish()
    }
}
impl ::std::cmp::PartialEq for Submsg {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        true && self.item_inner_opt() == rhs.item_inner_opt()
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
    pub struct SubmsgFields<TItemInner> {
        pub item_inner: TItemInner,
    }
}
pub use self::_fields::*;
#[derive(
    ::std::clone::Clone,
    ::std::marker::Copy,
    ::std::cmp::PartialEq,
    ::std::cmp::Eq,
    ::std::cmp::PartialOrd,
    ::std::cmp::Ord,
    ::std::hash::Hash,
    ::std::fmt::Debug,
)]
pub enum Enum {
    MyValue,
    _None(i32),
}
impl ::std::default::Default for Enum {
    fn default() -> Self {
        Self::MyValue
    }
}
impl ::std::convert::From::<Enum> for i32 {
    fn from(val: Enum) -> i32 {
        match val {
            Enum::MyValue => 0i32,
            self::Enum::_None(i) => i,
        }
    }
}
impl ::std::convert::From::<i32> for Enum {
    fn from(val: i32) -> Self {
        match val {
            0i32 => self::Enum::MyValue,
            _ => Enum::_None(val),
        }
    }
}
