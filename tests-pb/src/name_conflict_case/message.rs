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
pub struct TestCase {
    this_is_message: self::_pinternal::OptionalNumericalField::<
        i32,
        self::_pinternal::tags::Int32,
        0usize,
    >,
    _bitfield: self::_pinternal::BitArray<1usize>,
}
impl TestCase {
    pub fn this_is_message(&self) -> i32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.this_is_message,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn this_is_message_opt(&self) -> ::std::option::Option::<i32> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.this_is_message, &self._bitfield)
    }
    pub fn this_is_message_mut(&mut self) -> &mut i32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.this_is_message,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_this_is_message(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.this_is_message, &self._bitfield)
            .is_some()
    }
    pub fn clear_this_is_message(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::clear(&mut self.this_is_message, &mut self._bitfield)
    }
}
impl self::_puroro::Message for TestCase {
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
                    <self::_pinternal::OptionalNumericalField::<
                        i32,
                        self::_pinternal::tags::Int32,
                        0usize,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.this_is_message,
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
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.this_is_message,
            &self._bitfield,
            1i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for TestCase {
    fn clone(&self) -> Self {
        Self {
            this_is_message: <self::_pinternal::OptionalNumericalField::<
                i32,
                self::_pinternal::tags::Int32,
                0usize,
            > as ::std::clone::Clone>::clone(&self.this_is_message),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
impl ::std::ops::Drop for TestCase {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for TestCase {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct(stringify!(TestCase))
            .field(stringify!(this_is_message), &self.this_is_message_opt())
            .finish()
    }
}
impl ::std::cmp::PartialEq for TestCase {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        true && self.this_is_message_opt() == rhs.this_is_message_opt()
    }
}
pub union Test<TThisIsOneof> {
    _none: (),
    this_is_oneof: ::std::mem::ManuallyDrop::<TThisIsOneof>,
}
impl<TThisIsOneof> Test<TThisIsOneof>
where
    Self: self::_pinternal::OneofUnion,
    TThisIsOneof: self::_pinternal::OneofFieldType,
{
    pub(crate) fn this_is_oneof<B: self::_pinternal::BitSlice>(
        &self,
        bits: &B,
    ) -> <TThisIsOneof as self::_pinternal::OneofFieldType>::GetterOrElseType<'_> {
        #[allow(unused)]
        use ::std::option::Option::{None, Some};
        #[allow(unused)]
        use ::std::default::Default;
        use self::_pinternal::OneofFieldType;
        use ::std::ops::Deref as _;
        use self::_pinternal::OneofCase;
        let case_opt = OneofCase::from_bitslice(bits);
        let field_opt = matches!(
            case_opt, Some(self::_root::name_conflict_case::message::_case::TestCase:: <
            > ::ThisIsOneof(()))
        )
            .then(|| { unsafe { self.this_is_oneof.deref() } });
        OneofFieldType::get_field_or_else(field_opt, ::std::default::Default::default)
    }
    pub(crate) fn this_is_oneof_opt<B: self::_pinternal::BitSlice>(
        &self,
        bits: &B,
    ) -> <TThisIsOneof as self::_pinternal::OneofFieldType>::GetterOptType<'_> {
        #[allow(unused)]
        use ::std::option::Option::{None, Some};
        use self::_pinternal::OneofFieldType;
        use ::std::ops::Deref as _;
        use self::_pinternal::OneofCase;
        let case_opt = OneofCase::from_bitslice(bits);
        let field_opt = matches!(
            case_opt, Some(self::_root::name_conflict_case::message::_case::TestCase:: <
            > ::ThisIsOneof(()))
        )
            .then(|| { unsafe { self.this_is_oneof.deref() } });
        OneofFieldType::get_field_opt(field_opt)
    }
    pub(crate) fn this_is_oneof_mut<B: self::_pinternal::BitSlice>(
        &mut self,
        bits: &mut B,
    ) -> <TThisIsOneof as self::_pinternal::OneofFieldType>::GetterMutType<'_> {
        #[allow(unused)]
        use ::std::option::Option::Some;
        #[allow(unused)]
        use ::std::default::Default;
        use ::std::mem::ManuallyDrop;
        use ::std::ops::DerefMut as _;
        use self::_pinternal::{OneofCase, OneofUnion};
        use self::_pinternal::OneofFieldType;
        let case_opt = OneofCase::from_bitslice(bits);
        if let Some(
            self::_root::name_conflict_case::message::_case::TestCase::ThisIsOneof(()),
        ) = case_opt {} else {
            <Self as OneofUnion>::clear(self, bits);
            let index = OneofCase::into_u32(
                self::_root::name_conflict_case::message::_case::TestCase::ThisIsOneof(()),
            );
            bits.set_range(0usize..2usize, index);
            *self = Self {
                this_is_oneof: ManuallyDrop::new((::std::default::Default::default)()),
            };
        }
        let field_mut = unsafe { self.this_is_oneof.deref_mut() };
        OneofFieldType::get_field_mut(field_mut)
    }
}
impl<TThisIsOneof> self::_pinternal::OneofUnion for Test<TThisIsOneof>
where
    TThisIsOneof: self::_pinternal::OneofFieldType,
{
    type Case = self::_root::name_conflict_case::message::_case::TestCase;
    type CaseRef<'a>
    where
        Self: 'a,
    = self::_root::name_conflict_case::message::_case::TestCase::<
        <TThisIsOneof as self::_pinternal::OneofFieldType>::GetterType::<'a>,
    >;
    fn case_ref<B: self::_pinternal::BitSlice>(
        &self,
        bits: &B,
    ) -> ::std::option::Option<Self::CaseRef<'_>> {
        use self::_pinternal::OneofCase;
        use ::std::mem::ManuallyDrop;
        use ::std::ops::Deref as _;
        let case_opt = OneofCase::from_bitslice(bits);
        case_opt
            .map(|case| {
                match case {
                    Self::Case::ThisIsOneof(_) => {
                        Self::CaseRef::ThisIsOneof(
                            ManuallyDrop::deref(unsafe { &self.this_is_oneof })
                                .get_field(),
                        )
                    }
                }
            })
    }
    fn clear<B: self::_pinternal::BitSlice>(&mut self, bits: &mut B) {
        use self::_pinternal::OneofCase;
        use ::std::mem::ManuallyDrop;
        #[allow(unused)]
        use ::std::option::Option::Some;
        match OneofCase::from_bitslice(bits) {
            Some(Self::Case::ThisIsOneof(())) => {
                unsafe { ManuallyDrop::take(&mut self.this_is_oneof) };
            }
            _ => {}
        }
        bits.set_range(0usize..2usize, 0);
    }
    fn clone<B: self::_pinternal::BitSlice>(&self, bits: &B) -> Self {
        use self::_pinternal::OneofCase;
        #[allow(unused)]
        use ::std::option::Option::Some;
        #[allow(unused)]
        use ::std::clone::Clone;
        match OneofCase::from_bitslice(bits) {
            Some(Self::Case::ThisIsOneof(())) => {
                Self {
                    this_is_oneof: Clone::clone(unsafe { &self.this_is_oneof }),
                }
            }
            _ => Self { _none: () },
        }
    }
    fn deser_from_iter<I, B>(
        &mut self,
        bitvec: &mut B,
        field_data: self::_pinternal::ser::FieldData<I>,
        case: Self::Case,
    ) -> self::_puroro::Result<()>
    where
        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        B: self::_pinternal::BitSlice,
    {
        #[allow(unused)]
        use ::std::result::Result::Ok;
        match case {
            Self::Case::ThisIsOneof(_) => {
                let _ = <Self>::this_is_oneof_mut(self, bitvec);
                unsafe { &mut self.this_is_oneof }.deser_from_iter(field_data)?;
            }
        }
        Ok(())
    }
    fn ser_to_write<W, B>(&self, bitvec: &B, out: &mut W) -> self::_puroro::Result<()>
    where
        W: ::std::io::Write,
        B: self::_pinternal::BitSlice,
    {
        #[allow(unused)]
        use ::std::option::Option::Some;
        #[allow(unused)]
        use ::std::result::Result::Ok;
        use self::_pinternal::OneofCase;
        match OneofCase::from_bitslice(bitvec) {
            Some(Self::Case::ThisIsOneof(_)) => {
                unsafe { &self.this_is_oneof }.ser_to_write(1i32, out)?;
            }
            _ => {}
        }
        Ok(())
    }
}
impl<TThisIsOneof> ::std::default::Default for Test<TThisIsOneof> {
    fn default() -> Self {
        Self { _none: () }
    }
}
pub mod _case {
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
    #[derive(::std::fmt::Debug, ::std::cmp::PartialEq)]
    pub enum TestCase<TThisIsOneof = ()> {
        ThisIsOneof(TThisIsOneof),
    }
    impl self::_pinternal::OneofCase for TestCase {
        const BITFIELD_BEGIN: usize = 0usize;
        const BITFIELD_END: usize = 2usize;
        fn from_u32(x: u32) -> ::std::option::Option<Self> {
            match x {
                1u32 => ::std::option::Option::Some(Self::ThisIsOneof(())),
                _ => ::std::option::Option::None,
            }
        }
        fn into_u32(self) -> u32 {
            match self {
                Self::ThisIsOneof(_) => 1u32,
            }
        }
    }
}
pub use self::_case::*;
