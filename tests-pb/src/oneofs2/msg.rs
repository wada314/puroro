mod _puroro_root {
    #[allow(unused)]
    pub(crate) use super::super::_puroro_root::*;
}
mod _puroro {
    #[allow(unused)]
    pub(crate) use ::puroro::*;
}
pub union GroupOne<TG1Int32, TG1String> {
    _none: (),
    g1_int32: ::std::mem::ManuallyDrop::<TG1Int32>,
    g1_string: ::std::mem::ManuallyDrop::<TG1String>,
}
#[derive(::std::fmt::Debug, ::std::cmp::PartialEq)]
pub enum GroupOneCase<TG1Int32 = (), TG1String = ()> {
    G1Int32(TG1Int32),
    G1String(TG1String),
}
impl<TG1Int32, TG1String> GroupOne<TG1Int32, TG1String>
where
    TG1Int32: _puroro::internal::oneof_field_type::OneofFieldType,
    TG1String: _puroro::internal::oneof_field_type::OneofFieldType,
{
    pub(crate) fn g1_int32<B: self::_puroro::internal::bitvec::BitSlice>(
        &self,
        bits: &B,
    ) -> <TG1Int32 as self::_puroro::internal::oneof_field_type::OneofFieldType>::GetterOrElseType<
        '_,
    > {
        #[allow(unused)]
        use ::std::option::Option::{None, Some};
        #[allow(unused)]
        use ::std::default::Default;
        use self::_puroro::internal::oneof_field_type::OneofFieldType;
        use ::std::ops::Deref as _;
        use self::_puroro::internal::oneof_type::OneofCase as _;
        let case_opt = self::GroupOneCase::from_bitslice(bits);
        let item_opt = matches!(case_opt, Some(self::GroupOneCase::G1Int32(())))
            .then(|| { unsafe { self.g1_int32.deref() } });
        OneofFieldType::get_field_or_else(item_opt, ::std::default::Default::default)
    }
    pub(crate) fn g1_int32_opt<B: self::_puroro::internal::bitvec::BitSlice>(
        &self,
        bits: &B,
    ) -> <TG1Int32 as self::_puroro::internal::oneof_field_type::OneofFieldType>::GetterOptType<
        '_,
    > {
        #[allow(unused)]
        use ::std::option::Option::{None, Some};
        use self::_puroro::internal::oneof_field_type::OneofFieldType;
        use ::std::ops::Deref as _;
        use self::_puroro::internal::oneof_type::OneofCase as _;
        let case_opt = self::GroupOneCase::from_bitslice(bits);
        let item_opt = matches!(case_opt, Some(self::GroupOneCase::G1Int32(())))
            .then(|| { unsafe { self.g1_int32.deref() } });
        OneofFieldType::get_field_opt(item_opt)
    }
    pub(crate) fn g1_int32_mut<B: self::_puroro::internal::bitvec::BitSlice>(
        &mut self,
        bits: &mut B,
    ) -> <TG1Int32 as self::_puroro::internal::oneof_field_type::OneofFieldType>::GetterMutType<
        '_,
    > {
        #[allow(unused)]
        use ::std::option::Option::Some;
        #[allow(unused)]
        use ::std::default::Default;
        use ::std::mem::ManuallyDrop;
        use self::_puroro::internal::oneof_type::{OneofCase as _, OneofUnion};
        use self::_puroro::internal::oneof_field_type::OneofFieldType as _;
        let case_opt = self::GroupOneCase::from_bitslice(bits);
        if let Some(self::GroupOneCase::G1Int32(())) = case_opt {} else {
            <Self as OneofUnion>::clear(self, bits);
            let index = self::GroupOneCase::into_u32(self::GroupOneCase::G1Int32(()));
            bits.set_range(0usize..2usize, index);
            *self = self::GroupOne {
                g1_int32: ManuallyDrop::new((::std::default::Default::default)()),
            };
        }
        unsafe { &mut self.g1_int32 }.get_field_mut()
    }
    pub(crate) fn g1_string<B: self::_puroro::internal::bitvec::BitSlice>(
        &self,
        bits: &B,
    ) -> <TG1String as self::_puroro::internal::oneof_field_type::OneofFieldType>::GetterOrElseType<
        '_,
    > {
        #[allow(unused)]
        use ::std::option::Option::{None, Some};
        #[allow(unused)]
        use ::std::default::Default;
        use self::_puroro::internal::oneof_field_type::OneofFieldType;
        use ::std::ops::Deref as _;
        use self::_puroro::internal::oneof_type::OneofCase as _;
        let case_opt = self::GroupOneCase::from_bitslice(bits);
        let item_opt = matches!(case_opt, Some(self::GroupOneCase::G1String(())))
            .then(|| { unsafe { self.g1_string.deref() } });
        OneofFieldType::get_field_or_else(item_opt, ::std::default::Default::default)
    }
    pub(crate) fn g1_string_opt<B: self::_puroro::internal::bitvec::BitSlice>(
        &self,
        bits: &B,
    ) -> <TG1String as self::_puroro::internal::oneof_field_type::OneofFieldType>::GetterOptType<
        '_,
    > {
        #[allow(unused)]
        use ::std::option::Option::{None, Some};
        use self::_puroro::internal::oneof_field_type::OneofFieldType;
        use ::std::ops::Deref as _;
        use self::_puroro::internal::oneof_type::OneofCase as _;
        let case_opt = self::GroupOneCase::from_bitslice(bits);
        let item_opt = matches!(case_opt, Some(self::GroupOneCase::G1String(())))
            .then(|| { unsafe { self.g1_string.deref() } });
        OneofFieldType::get_field_opt(item_opt)
    }
    pub(crate) fn g1_string_mut<B: self::_puroro::internal::bitvec::BitSlice>(
        &mut self,
        bits: &mut B,
    ) -> <TG1String as self::_puroro::internal::oneof_field_type::OneofFieldType>::GetterMutType<
        '_,
    > {
        #[allow(unused)]
        use ::std::option::Option::Some;
        #[allow(unused)]
        use ::std::default::Default;
        use ::std::mem::ManuallyDrop;
        use self::_puroro::internal::oneof_type::{OneofCase as _, OneofUnion};
        use self::_puroro::internal::oneof_field_type::OneofFieldType as _;
        let case_opt = self::GroupOneCase::from_bitslice(bits);
        if let Some(self::GroupOneCase::G1String(())) = case_opt {} else {
            <Self as OneofUnion>::clear(self, bits);
            let index = self::GroupOneCase::into_u32(self::GroupOneCase::G1String(()));
            bits.set_range(0usize..2usize, index);
            *self = self::GroupOne {
                g1_string: ManuallyDrop::new((::std::default::Default::default)()),
            };
        }
        unsafe { &mut self.g1_string }.get_field_mut()
    }
}
impl<TG1Int32, TG1String> self::_puroro::internal::oneof_type::OneofUnion
for GroupOne<TG1Int32, TG1String>
where
    TG1Int32: _puroro::internal::oneof_field_type::OneofFieldType,
    TG1String: _puroro::internal::oneof_field_type::OneofFieldType,
{
    type Case = self::GroupOneCase;
    type CaseRef<'a> where Self: 'a = self::GroupOneCase::<i32, &'a str>;
    fn case_ref<B: self::_puroro::internal::bitvec::BitSlice>(
        &self,
        bits: &B,
    ) -> ::std::option::Option<Self::CaseRef<'_>> {
        use self::_puroro::internal::oneof_type::OneofCase;
        use ::std::mem::ManuallyDrop;
        use self::_puroro::internal::oneof_field_type::OneofFieldType as _;
        use ::std::ops::Deref as _;
        let case_opt = <self::GroupOneCase as OneofCase>::from_bitslice(bits);
        case_opt
            .map(|case| {
                match case {
                    self::GroupOneCase::G1Int32(_) => {
                        self::GroupOneCase::G1Int32(
                            ManuallyDrop::deref(unsafe { &self.g1_int32 }).get_field(),
                        )
                    }
                    self::GroupOneCase::G1String(_) => {
                        self::GroupOneCase::G1String(
                            ManuallyDrop::deref(unsafe { &self.g1_string }).get_field(),
                        )
                    }
                }
            })
    }
    fn clear<B: self::_puroro::internal::bitvec::BitSlice>(&mut self, bits: &mut B) {
        use self::_puroro::internal::oneof_type::OneofCase;
        use ::std::mem::ManuallyDrop;
        #[allow(unused)]
        use ::std::option::Option::Some;
        match <self::GroupOneCase as OneofCase>::from_bitslice(bits) {
            Some(self::GroupOneCase::G1Int32(())) => {
                unsafe { ManuallyDrop::take(&mut self.g1_int32) };
            }
            Some(self::GroupOneCase::G1String(())) => {
                unsafe { ManuallyDrop::take(&mut self.g1_string) };
            }
            _ => {}
        }
        bits.set_range(0usize..2usize, 0);
    }
    fn clone<B: self::_puroro::internal::bitvec::BitSlice>(&self, bits: &B) -> Self {
        use self::_puroro::internal::oneof_type::OneofCase;
        #[allow(unused)]
        use ::std::option::Option::Some;
        #[allow(unused)]
        use ::std::clone::Clone;
        match <self::GroupOneCase as OneofCase>::from_bitslice(bits) {
            Some(self::GroupOneCase::G1Int32(())) => {
                Self {
                    g1_int32: Clone::clone(unsafe { &self.g1_int32 }),
                }
            }
            Some(self::GroupOneCase::G1String(())) => {
                Self {
                    g1_string: Clone::clone(unsafe { &self.g1_string }),
                }
            }
            _ => Self { _none: () },
        }
    }
    fn deser_from_iter<I, B>(
        &mut self,
        bitvec: &mut B,
        field_data: self::_puroro::internal::ser::FieldData<I>,
        case: Self::Case,
    ) -> self::_puroro::Result<()>
    where
        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        B: self::_puroro::internal::bitvec::BitSlice,
    {
        use self::_puroro::internal::oneof_field_type::OneofFieldType as _;
        #[allow(unused)]
        use ::std::result::Result::Ok;
        match case {
            Self::Case::G1Int32(_) => {
                let _ = <Self>::g1_int32_mut(self, bitvec);
                unsafe { &mut self.g1_int32 }.deser_from_iter(field_data)?;
            }
            Self::Case::G1String(_) => {
                let _ = <Self>::g1_string_mut(self, bitvec);
                unsafe { &mut self.g1_string }.deser_from_iter(field_data)?;
            }
        }
        Ok(())
    }
    fn ser_to_write<W, B>(&self, bitvec: &B, out: &mut W) -> self::_puroro::Result<()>
    where
        W: ::std::io::Write,
        B: self::_puroro::internal::bitvec::BitSlice,
    {
        #[allow(unused)]
        use ::std::option::Option::Some;
        #[allow(unused)]
        use ::std::result::Result::Ok;
        use self::_puroro::internal::oneof_type::OneofCase as _;
        use self::_puroro::internal::oneof_field_type::OneofFieldType as _;
        match self::GroupOneCase::from_bitslice(bitvec) {
            Some(self::GroupOneCase::G1Int32(_)) => {
                unsafe { &self.g1_int32 }.ser_to_write(1i32, out)?;
            }
            Some(self::GroupOneCase::G1String(_)) => {
                unsafe { &self.g1_string }.ser_to_write(2i32, out)?;
            }
            _ => {}
        }
        Ok(())
    }
}
impl self::_puroro::internal::oneof_type::OneofCase for GroupOneCase {
    const BITFIELD_BEGIN: usize = 0usize;
    const BITFIELD_END: usize = 2usize;
    fn from_u32(x: u32) -> ::std::option::Option<Self> {
        match x {
            1u32 => ::std::option::Option::Some(Self::G1Int32(())),
            2u32 => ::std::option::Option::Some(Self::G1String(())),
            _ => ::std::option::Option::None,
        }
    }
    fn into_u32(self) -> u32 {
        match self {
            Self::G1Int32(_) => 1u32,
            Self::G1String(_) => 2u32,
        }
    }
}
impl<TG1Int32, TG1String> ::std::default::Default for GroupOne<TG1Int32, TG1String> {
    fn default() -> Self {
        Self { _none: () }
    }
}
pub union GroupTwo<TG2F32, TG2String, TG2Submsg> {
    _none: (),
    g2_f32: ::std::mem::ManuallyDrop::<TG2F32>,
    g2_string: ::std::mem::ManuallyDrop::<TG2String>,
    g2_submsg: ::std::mem::ManuallyDrop::<TG2Submsg>,
}
#[derive(::std::fmt::Debug, ::std::cmp::PartialEq)]
pub enum GroupTwoCase<TG2F32 = (), TG2String = (), TG2Submsg = ()> {
    G2F32(TG2F32),
    G2String(TG2String),
    G2Submsg(TG2Submsg),
}
impl<TG2F32, TG2String, TG2Submsg> GroupTwo<TG2F32, TG2String, TG2Submsg>
where
    TG2F32: _puroro::internal::oneof_field_type::OneofFieldType,
    TG2String: _puroro::internal::oneof_field_type::OneofFieldType,
    TG2Submsg: _puroro::internal::oneof_field_type::OneofFieldType,
{
    pub(crate) fn g2_f32<B: self::_puroro::internal::bitvec::BitSlice>(
        &self,
        bits: &B,
    ) -> <TG2F32 as self::_puroro::internal::oneof_field_type::OneofFieldType>::GetterOrElseType<
        '_,
    > {
        #[allow(unused)]
        use ::std::option::Option::{None, Some};
        #[allow(unused)]
        use ::std::default::Default;
        use self::_puroro::internal::oneof_field_type::OneofFieldType;
        use ::std::ops::Deref as _;
        use self::_puroro::internal::oneof_type::OneofCase as _;
        let case_opt = self::GroupTwoCase::from_bitslice(bits);
        let item_opt = matches!(case_opt, Some(self::GroupTwoCase::G2F32(())))
            .then(|| { unsafe { self.g2_f32.deref() } });
        OneofFieldType::get_field_or_else(item_opt, ::std::default::Default::default)
    }
    pub(crate) fn g2_f32_opt<B: self::_puroro::internal::bitvec::BitSlice>(
        &self,
        bits: &B,
    ) -> <TG2F32 as self::_puroro::internal::oneof_field_type::OneofFieldType>::GetterOptType<
        '_,
    > {
        #[allow(unused)]
        use ::std::option::Option::{None, Some};
        use self::_puroro::internal::oneof_field_type::OneofFieldType;
        use ::std::ops::Deref as _;
        use self::_puroro::internal::oneof_type::OneofCase as _;
        let case_opt = self::GroupTwoCase::from_bitslice(bits);
        let item_opt = matches!(case_opt, Some(self::GroupTwoCase::G2F32(())))
            .then(|| { unsafe { self.g2_f32.deref() } });
        OneofFieldType::get_field_opt(item_opt)
    }
    pub(crate) fn g2_f32_mut<B: self::_puroro::internal::bitvec::BitSlice>(
        &mut self,
        bits: &mut B,
    ) -> <TG2F32 as self::_puroro::internal::oneof_field_type::OneofFieldType>::GetterMutType<
        '_,
    > {
        #[allow(unused)]
        use ::std::option::Option::Some;
        #[allow(unused)]
        use ::std::default::Default;
        use ::std::mem::ManuallyDrop;
        use self::_puroro::internal::oneof_type::{OneofCase as _, OneofUnion};
        use self::_puroro::internal::oneof_field_type::OneofFieldType as _;
        let case_opt = self::GroupTwoCase::from_bitslice(bits);
        if let Some(self::GroupTwoCase::G2F32(())) = case_opt {} else {
            <Self as OneofUnion>::clear(self, bits);
            let index = self::GroupTwoCase::into_u32(self::GroupTwoCase::G2F32(()));
            bits.set_range(2usize..5usize, index);
            *self = self::GroupTwo {
                g2_f32: ManuallyDrop::new((::std::default::Default::default)()),
            };
        }
        unsafe { &mut self.g2_f32 }.get_field_mut()
    }
    pub(crate) fn g2_string<B: self::_puroro::internal::bitvec::BitSlice>(
        &self,
        bits: &B,
    ) -> <TG2String as self::_puroro::internal::oneof_field_type::OneofFieldType>::GetterOrElseType<
        '_,
    > {
        #[allow(unused)]
        use ::std::option::Option::{None, Some};
        #[allow(unused)]
        use ::std::default::Default;
        use self::_puroro::internal::oneof_field_type::OneofFieldType;
        use ::std::ops::Deref as _;
        use self::_puroro::internal::oneof_type::OneofCase as _;
        let case_opt = self::GroupTwoCase::from_bitslice(bits);
        let item_opt = matches!(case_opt, Some(self::GroupTwoCase::G2String(())))
            .then(|| { unsafe { self.g2_string.deref() } });
        OneofFieldType::get_field_or_else(item_opt, ::std::default::Default::default)
    }
    pub(crate) fn g2_string_opt<B: self::_puroro::internal::bitvec::BitSlice>(
        &self,
        bits: &B,
    ) -> <TG2String as self::_puroro::internal::oneof_field_type::OneofFieldType>::GetterOptType<
        '_,
    > {
        #[allow(unused)]
        use ::std::option::Option::{None, Some};
        use self::_puroro::internal::oneof_field_type::OneofFieldType;
        use ::std::ops::Deref as _;
        use self::_puroro::internal::oneof_type::OneofCase as _;
        let case_opt = self::GroupTwoCase::from_bitslice(bits);
        let item_opt = matches!(case_opt, Some(self::GroupTwoCase::G2String(())))
            .then(|| { unsafe { self.g2_string.deref() } });
        OneofFieldType::get_field_opt(item_opt)
    }
    pub(crate) fn g2_string_mut<B: self::_puroro::internal::bitvec::BitSlice>(
        &mut self,
        bits: &mut B,
    ) -> <TG2String as self::_puroro::internal::oneof_field_type::OneofFieldType>::GetterMutType<
        '_,
    > {
        #[allow(unused)]
        use ::std::option::Option::Some;
        #[allow(unused)]
        use ::std::default::Default;
        use ::std::mem::ManuallyDrop;
        use self::_puroro::internal::oneof_type::{OneofCase as _, OneofUnion};
        use self::_puroro::internal::oneof_field_type::OneofFieldType as _;
        let case_opt = self::GroupTwoCase::from_bitslice(bits);
        if let Some(self::GroupTwoCase::G2String(())) = case_opt {} else {
            <Self as OneofUnion>::clear(self, bits);
            let index = self::GroupTwoCase::into_u32(self::GroupTwoCase::G2String(()));
            bits.set_range(2usize..5usize, index);
            *self = self::GroupTwo {
                g2_string: ManuallyDrop::new((::std::default::Default::default)()),
            };
        }
        unsafe { &mut self.g2_string }.get_field_mut()
    }
    pub(crate) fn g2_submsg<B: self::_puroro::internal::bitvec::BitSlice>(
        &self,
        bits: &B,
    ) -> <TG2Submsg as self::_puroro::internal::oneof_field_type::OneofFieldType>::GetterOrElseType<
        '_,
    > {
        #[allow(unused)]
        use ::std::option::Option::{None, Some};
        #[allow(unused)]
        use ::std::default::Default;
        use self::_puroro::internal::oneof_field_type::OneofFieldType;
        use ::std::ops::Deref as _;
        use self::_puroro::internal::oneof_type::OneofCase as _;
        let case_opt = self::GroupTwoCase::from_bitslice(bits);
        let item_opt = matches!(case_opt, Some(self::GroupTwoCase::G2Submsg(())))
            .then(|| { unsafe { self.g2_submsg.deref() } });
        OneofFieldType::get_field_or_else(item_opt, ::std::default::Default::default)
    }
    pub(crate) fn g2_submsg_opt<B: self::_puroro::internal::bitvec::BitSlice>(
        &self,
        bits: &B,
    ) -> <TG2Submsg as self::_puroro::internal::oneof_field_type::OneofFieldType>::GetterOptType<
        '_,
    > {
        #[allow(unused)]
        use ::std::option::Option::{None, Some};
        use self::_puroro::internal::oneof_field_type::OneofFieldType;
        use ::std::ops::Deref as _;
        use self::_puroro::internal::oneof_type::OneofCase as _;
        let case_opt = self::GroupTwoCase::from_bitslice(bits);
        let item_opt = matches!(case_opt, Some(self::GroupTwoCase::G2Submsg(())))
            .then(|| { unsafe { self.g2_submsg.deref() } });
        OneofFieldType::get_field_opt(item_opt)
    }
    pub(crate) fn g2_submsg_mut<B: self::_puroro::internal::bitvec::BitSlice>(
        &mut self,
        bits: &mut B,
    ) -> <TG2Submsg as self::_puroro::internal::oneof_field_type::OneofFieldType>::GetterMutType<
        '_,
    > {
        #[allow(unused)]
        use ::std::option::Option::Some;
        #[allow(unused)]
        use ::std::default::Default;
        use ::std::mem::ManuallyDrop;
        use self::_puroro::internal::oneof_type::{OneofCase as _, OneofUnion};
        use self::_puroro::internal::oneof_field_type::OneofFieldType as _;
        let case_opt = self::GroupTwoCase::from_bitslice(bits);
        if let Some(self::GroupTwoCase::G2Submsg(())) = case_opt {} else {
            <Self as OneofUnion>::clear(self, bits);
            let index = self::GroupTwoCase::into_u32(self::GroupTwoCase::G2Submsg(()));
            bits.set_range(2usize..5usize, index);
            *self = self::GroupTwo {
                g2_submsg: ManuallyDrop::new((::std::default::Default::default)()),
            };
        }
        unsafe { &mut self.g2_submsg }.get_field_mut()
    }
}
impl<TG2F32, TG2String, TG2Submsg> self::_puroro::internal::oneof_type::OneofUnion
for GroupTwo<TG2F32, TG2String, TG2Submsg>
where
    TG2F32: _puroro::internal::oneof_field_type::OneofFieldType,
    TG2String: _puroro::internal::oneof_field_type::OneofFieldType,
    TG2Submsg: _puroro::internal::oneof_field_type::OneofFieldType,
{
    type Case = self::GroupTwoCase;
    type CaseRef<'a>
    where
        Self: 'a,
    = self::GroupTwoCase::<f32, &'a str, &'a self::_puroro_root::oneofs2::Submsg>;
    fn case_ref<B: self::_puroro::internal::bitvec::BitSlice>(
        &self,
        bits: &B,
    ) -> ::std::option::Option<Self::CaseRef<'_>> {
        use self::_puroro::internal::oneof_type::OneofCase;
        use ::std::mem::ManuallyDrop;
        use self::_puroro::internal::oneof_field_type::OneofFieldType as _;
        use ::std::ops::Deref as _;
        let case_opt = <self::GroupTwoCase as OneofCase>::from_bitslice(bits);
        case_opt
            .map(|case| {
                match case {
                    self::GroupTwoCase::G2F32(_) => {
                        self::GroupTwoCase::G2F32(
                            ManuallyDrop::deref(unsafe { &self.g2_f32 }).get_field(),
                        )
                    }
                    self::GroupTwoCase::G2String(_) => {
                        self::GroupTwoCase::G2String(
                            ManuallyDrop::deref(unsafe { &self.g2_string }).get_field(),
                        )
                    }
                    self::GroupTwoCase::G2Submsg(_) => {
                        self::GroupTwoCase::G2Submsg(
                            ManuallyDrop::deref(unsafe { &self.g2_submsg }).get_field(),
                        )
                    }
                }
            })
    }
    fn clear<B: self::_puroro::internal::bitvec::BitSlice>(&mut self, bits: &mut B) {
        use self::_puroro::internal::oneof_type::OneofCase;
        use ::std::mem::ManuallyDrop;
        #[allow(unused)]
        use ::std::option::Option::Some;
        match <self::GroupTwoCase as OneofCase>::from_bitslice(bits) {
            Some(self::GroupTwoCase::G2F32(())) => {
                unsafe { ManuallyDrop::take(&mut self.g2_f32) };
            }
            Some(self::GroupTwoCase::G2String(())) => {
                unsafe { ManuallyDrop::take(&mut self.g2_string) };
            }
            Some(self::GroupTwoCase::G2Submsg(())) => {
                unsafe { ManuallyDrop::take(&mut self.g2_submsg) };
            }
            _ => {}
        }
        bits.set_range(2usize..5usize, 0);
    }
    fn clone<B: self::_puroro::internal::bitvec::BitSlice>(&self, bits: &B) -> Self {
        use self::_puroro::internal::oneof_type::OneofCase;
        #[allow(unused)]
        use ::std::option::Option::Some;
        #[allow(unused)]
        use ::std::clone::Clone;
        match <self::GroupTwoCase as OneofCase>::from_bitslice(bits) {
            Some(self::GroupTwoCase::G2F32(())) => {
                Self {
                    g2_f32: Clone::clone(unsafe { &self.g2_f32 }),
                }
            }
            Some(self::GroupTwoCase::G2String(())) => {
                Self {
                    g2_string: Clone::clone(unsafe { &self.g2_string }),
                }
            }
            Some(self::GroupTwoCase::G2Submsg(())) => {
                Self {
                    g2_submsg: Clone::clone(unsafe { &self.g2_submsg }),
                }
            }
            _ => Self { _none: () },
        }
    }
    fn deser_from_iter<I, B>(
        &mut self,
        bitvec: &mut B,
        field_data: self::_puroro::internal::ser::FieldData<I>,
        case: Self::Case,
    ) -> self::_puroro::Result<()>
    where
        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        B: self::_puroro::internal::bitvec::BitSlice,
    {
        use self::_puroro::internal::oneof_field_type::OneofFieldType as _;
        #[allow(unused)]
        use ::std::result::Result::Ok;
        match case {
            Self::Case::G2F32(_) => {
                let _ = <Self>::g2_f32_mut(self, bitvec);
                unsafe { &mut self.g2_f32 }.deser_from_iter(field_data)?;
            }
            Self::Case::G2String(_) => {
                let _ = <Self>::g2_string_mut(self, bitvec);
                unsafe { &mut self.g2_string }.deser_from_iter(field_data)?;
            }
            Self::Case::G2Submsg(_) => {
                let _ = <Self>::g2_submsg_mut(self, bitvec);
                unsafe { &mut self.g2_submsg }.deser_from_iter(field_data)?;
            }
        }
        Ok(())
    }
    fn ser_to_write<W, B>(&self, bitvec: &B, out: &mut W) -> self::_puroro::Result<()>
    where
        W: ::std::io::Write,
        B: self::_puroro::internal::bitvec::BitSlice,
    {
        #[allow(unused)]
        use ::std::option::Option::Some;
        #[allow(unused)]
        use ::std::result::Result::Ok;
        use self::_puroro::internal::oneof_type::OneofCase as _;
        use self::_puroro::internal::oneof_field_type::OneofFieldType as _;
        match self::GroupTwoCase::from_bitslice(bitvec) {
            Some(self::GroupTwoCase::G2F32(_)) => {
                unsafe { &self.g2_f32 }.ser_to_write(3i32, out)?;
            }
            Some(self::GroupTwoCase::G2String(_)) => {
                unsafe { &self.g2_string }.ser_to_write(4i32, out)?;
            }
            Some(self::GroupTwoCase::G2Submsg(_)) => {
                unsafe { &self.g2_submsg }.ser_to_write(5i32, out)?;
            }
            _ => {}
        }
        Ok(())
    }
}
impl self::_puroro::internal::oneof_type::OneofCase for GroupTwoCase {
    const BITFIELD_BEGIN: usize = 2usize;
    const BITFIELD_END: usize = 5usize;
    fn from_u32(x: u32) -> ::std::option::Option<Self> {
        match x {
            1u32 => ::std::option::Option::Some(Self::G2F32(())),
            2u32 => ::std::option::Option::Some(Self::G2String(())),
            3u32 => ::std::option::Option::Some(Self::G2Submsg(())),
            _ => ::std::option::Option::None,
        }
    }
    fn into_u32(self) -> u32 {
        match self {
            Self::G2F32(_) => 1u32,
            Self::G2String(_) => 2u32,
            Self::G2Submsg(_) => 3u32,
        }
    }
}
impl<TG2F32, TG2String, TG2Submsg> ::std::default::Default
for GroupTwo<TG2F32, TG2String, TG2Submsg> {
    fn default() -> Self {
        Self { _none: () }
    }
}
pub union GroupThree<TG3Int32> {
    _none: (),
    g3_int32: ::std::mem::ManuallyDrop::<TG3Int32>,
}
#[derive(::std::fmt::Debug, ::std::cmp::PartialEq)]
pub enum GroupThreeCase<TG3Int32 = ()> {
    G3Int32(TG3Int32),
}
impl<TG3Int32> GroupThree<TG3Int32>
where
    TG3Int32: _puroro::internal::oneof_field_type::OneofFieldType,
{
    pub(crate) fn g3_int32<B: self::_puroro::internal::bitvec::BitSlice>(
        &self,
        bits: &B,
    ) -> <TG3Int32 as self::_puroro::internal::oneof_field_type::OneofFieldType>::GetterOrElseType<
        '_,
    > {
        #[allow(unused)]
        use ::std::option::Option::{None, Some};
        #[allow(unused)]
        use ::std::default::Default;
        use self::_puroro::internal::oneof_field_type::OneofFieldType;
        use ::std::ops::Deref as _;
        use self::_puroro::internal::oneof_type::OneofCase as _;
        let case_opt = self::GroupThreeCase::from_bitslice(bits);
        let item_opt = matches!(case_opt, Some(self::GroupThreeCase::G3Int32(())))
            .then(|| { unsafe { self.g3_int32.deref() } });
        OneofFieldType::get_field_or_else(item_opt, ::std::default::Default::default)
    }
    pub(crate) fn g3_int32_opt<B: self::_puroro::internal::bitvec::BitSlice>(
        &self,
        bits: &B,
    ) -> <TG3Int32 as self::_puroro::internal::oneof_field_type::OneofFieldType>::GetterOptType<
        '_,
    > {
        #[allow(unused)]
        use ::std::option::Option::{None, Some};
        use self::_puroro::internal::oneof_field_type::OneofFieldType;
        use ::std::ops::Deref as _;
        use self::_puroro::internal::oneof_type::OneofCase as _;
        let case_opt = self::GroupThreeCase::from_bitslice(bits);
        let item_opt = matches!(case_opt, Some(self::GroupThreeCase::G3Int32(())))
            .then(|| { unsafe { self.g3_int32.deref() } });
        OneofFieldType::get_field_opt(item_opt)
    }
    pub(crate) fn g3_int32_mut<B: self::_puroro::internal::bitvec::BitSlice>(
        &mut self,
        bits: &mut B,
    ) -> <TG3Int32 as self::_puroro::internal::oneof_field_type::OneofFieldType>::GetterMutType<
        '_,
    > {
        #[allow(unused)]
        use ::std::option::Option::Some;
        #[allow(unused)]
        use ::std::default::Default;
        use ::std::mem::ManuallyDrop;
        use self::_puroro::internal::oneof_type::{OneofCase as _, OneofUnion};
        use self::_puroro::internal::oneof_field_type::OneofFieldType as _;
        let case_opt = self::GroupThreeCase::from_bitslice(bits);
        if let Some(self::GroupThreeCase::G3Int32(())) = case_opt {} else {
            <Self as OneofUnion>::clear(self, bits);
            let index = self::GroupThreeCase::into_u32(
                self::GroupThreeCase::G3Int32(()),
            );
            bits.set_range(3usize..5usize, index);
            *self = self::GroupThree {
                g3_int32: ManuallyDrop::new((::std::default::Default::default)()),
            };
        }
        unsafe { &mut self.g3_int32 }.get_field_mut()
    }
}
impl<TG3Int32> self::_puroro::internal::oneof_type::OneofUnion for GroupThree<TG3Int32>
where
    TG3Int32: _puroro::internal::oneof_field_type::OneofFieldType,
{
    type Case = self::GroupThreeCase;
    type CaseRef<'a> where Self: 'a = self::GroupThreeCase::<i32>;
    fn case_ref<B: self::_puroro::internal::bitvec::BitSlice>(
        &self,
        bits: &B,
    ) -> ::std::option::Option<Self::CaseRef<'_>> {
        use self::_puroro::internal::oneof_type::OneofCase;
        use ::std::mem::ManuallyDrop;
        use self::_puroro::internal::oneof_field_type::OneofFieldType as _;
        use ::std::ops::Deref as _;
        let case_opt = <self::GroupThreeCase as OneofCase>::from_bitslice(bits);
        case_opt
            .map(|case| {
                match case {
                    self::GroupThreeCase::G3Int32(_) => {
                        self::GroupThreeCase::G3Int32(
                            ManuallyDrop::deref(unsafe { &self.g3_int32 }).get_field(),
                        )
                    }
                }
            })
    }
    fn clear<B: self::_puroro::internal::bitvec::BitSlice>(&mut self, bits: &mut B) {
        use self::_puroro::internal::oneof_type::OneofCase;
        use ::std::mem::ManuallyDrop;
        #[allow(unused)]
        use ::std::option::Option::Some;
        match <self::GroupThreeCase as OneofCase>::from_bitslice(bits) {
            Some(self::GroupThreeCase::G3Int32(())) => {
                unsafe { ManuallyDrop::take(&mut self.g3_int32) };
            }
            _ => {}
        }
        bits.set_range(3usize..5usize, 0);
    }
    fn clone<B: self::_puroro::internal::bitvec::BitSlice>(&self, bits: &B) -> Self {
        use self::_puroro::internal::oneof_type::OneofCase;
        #[allow(unused)]
        use ::std::option::Option::Some;
        #[allow(unused)]
        use ::std::clone::Clone;
        match <self::GroupThreeCase as OneofCase>::from_bitslice(bits) {
            Some(self::GroupThreeCase::G3Int32(())) => {
                Self {
                    g3_int32: Clone::clone(unsafe { &self.g3_int32 }),
                }
            }
            _ => Self { _none: () },
        }
    }
    fn deser_from_iter<I, B>(
        &mut self,
        bitvec: &mut B,
        field_data: self::_puroro::internal::ser::FieldData<I>,
        case: Self::Case,
    ) -> self::_puroro::Result<()>
    where
        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        B: self::_puroro::internal::bitvec::BitSlice,
    {
        use self::_puroro::internal::oneof_field_type::OneofFieldType as _;
        #[allow(unused)]
        use ::std::result::Result::Ok;
        match case {
            Self::Case::G3Int32(_) => {
                let _ = <Self>::g3_int32_mut(self, bitvec);
                unsafe { &mut self.g3_int32 }.deser_from_iter(field_data)?;
            }
        }
        Ok(())
    }
    fn ser_to_write<W, B>(&self, bitvec: &B, out: &mut W) -> self::_puroro::Result<()>
    where
        W: ::std::io::Write,
        B: self::_puroro::internal::bitvec::BitSlice,
    {
        #[allow(unused)]
        use ::std::option::Option::Some;
        #[allow(unused)]
        use ::std::result::Result::Ok;
        use self::_puroro::internal::oneof_type::OneofCase as _;
        use self::_puroro::internal::oneof_field_type::OneofFieldType as _;
        match self::GroupThreeCase::from_bitslice(bitvec) {
            Some(self::GroupThreeCase::G3Int32(_)) => {
                unsafe { &self.g3_int32 }.ser_to_write(6i32, out)?;
            }
            _ => {}
        }
        Ok(())
    }
}
impl self::_puroro::internal::oneof_type::OneofCase for GroupThreeCase {
    const BITFIELD_BEGIN: usize = 3usize;
    const BITFIELD_END: usize = 5usize;
    fn from_u32(x: u32) -> ::std::option::Option<Self> {
        match x {
            1u32 => ::std::option::Option::Some(Self::G3Int32(())),
            _ => ::std::option::Option::None,
        }
    }
    fn into_u32(self) -> u32 {
        match self {
            Self::G3Int32(_) => 1u32,
        }
    }
}
impl<TG3Int32> ::std::default::Default for GroupThree<TG3Int32> {
    fn default() -> Self {
        Self { _none: () }
    }
}
