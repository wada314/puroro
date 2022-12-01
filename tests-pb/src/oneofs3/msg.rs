pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}
pub mod _puroro {
    pub use ::puroro::*;
}
pub union GroupOne {
    _none: (),
    g1_int32: ::std::mem::ManuallyDrop::<
        self::_puroro::internal::oneof_field_type::NumericalField::<
            i32,
            self::_puroro::tags::Int32,
        >,
    >,
    g1_string: ::std::mem::ManuallyDrop::<
        self::_puroro::internal::oneof_field_type::StringField,
    >,
}
pub enum GroupOneCase<G1Int32 = (), G1String = ()> {
    G1Int32(G1Int32),
    G1String(G1String),
}
impl GroupOne {
    pub(crate) fn g1_int32<B: self::_puroro::bitvec::BitSlice>(&self, bits: &B) -> i32 {
        #[allow(unused)]
        use ::std::option::Option::{None, Some};
        #[allow(unused)]
        use ::std::default::Default;
        use self::_puroro::internal::oneof_field_type::OneofFieldTypeOpt;
        use ::std::ops::Deref as _;
        use self::_puroro::internal::oneof_type::OneofCase as _;
        let case_opt = self::GroupOneCase::from_bitslice(bits);
        let item_opt = matches!(case_opt, Some(self::GroupOneCase::G1Int32(())))
            .then(|| { unsafe { self.g1_int32.deref() } });
        OneofFieldTypeOpt::get_field(item_opt, Default::default)
    }
    pub(crate) fn g1_int32_opt<B: self::_puroro::bitvec::BitSlice>(
        &self,
        bits: &B,
    ) -> ::std::option::Option::<i32> {
        #[allow(unused)]
        use ::std::option::Option::{None, Some};
        use self::_puroro::internal::oneof_field_type::OneofFieldTypeOpt;
        use ::std::ops::Deref as _;
        use self::_puroro::internal::oneof_type::OneofCase as _;
        let case_opt = self::GroupOneCase::from_bitslice(bits);
        let item_opt = matches!(case_opt, Some(self::GroupOneCase::G1Int32(())))
            .then(|| { unsafe { self.g1_int32.deref() } });
        OneofFieldTypeOpt::get_field_opt(item_opt)
    }
    pub(crate) fn g1_int32_mut<B: self::_puroro::bitvec::BitSlice>(
        &mut self,
        bits: &mut B,
    ) -> &mut i32 {
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
                g1_int32: ManuallyDrop::new(Default::default()),
            };
        }
        unsafe { &mut self.g1_int32 }.mut_field()
    }
    pub(crate) fn g1_string<B: self::_puroro::bitvec::BitSlice>(
        &self,
        bits: &B,
    ) -> &str {
        #[allow(unused)]
        use ::std::option::Option::{None, Some};
        #[allow(unused)]
        use ::std::default::Default;
        use self::_puroro::internal::oneof_field_type::OneofFieldTypeOpt;
        use ::std::ops::Deref as _;
        use self::_puroro::internal::oneof_type::OneofCase as _;
        let case_opt = self::GroupOneCase::from_bitslice(bits);
        let item_opt = matches!(case_opt, Some(self::GroupOneCase::G1String(())))
            .then(|| { unsafe { self.g1_string.deref() } });
        OneofFieldTypeOpt::get_field(item_opt, Default::default)
    }
    pub(crate) fn g1_string_opt<B: self::_puroro::bitvec::BitSlice>(
        &self,
        bits: &B,
    ) -> ::std::option::Option::<&str> {
        #[allow(unused)]
        use ::std::option::Option::{None, Some};
        use self::_puroro::internal::oneof_field_type::OneofFieldTypeOpt;
        use ::std::ops::Deref as _;
        use self::_puroro::internal::oneof_type::OneofCase as _;
        let case_opt = self::GroupOneCase::from_bitslice(bits);
        let item_opt = matches!(case_opt, Some(self::GroupOneCase::G1String(())))
            .then(|| { unsafe { self.g1_string.deref() } });
        OneofFieldTypeOpt::get_field_opt(item_opt)
    }
    pub(crate) fn g1_string_mut<B: self::_puroro::bitvec::BitSlice>(
        &mut self,
        bits: &mut B,
    ) -> &mut ::std::string::String {
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
                g1_string: ManuallyDrop::new(Default::default()),
            };
        }
        unsafe { &mut self.g1_string }.mut_field()
    }
}
impl self::_puroro::internal::oneof_type::OneofUnion for GroupOne {
    type Case = self::GroupOneCase;
    type CaseRef<'a> = self::GroupOneCase::<i32, &'a str>;
    fn case_ref<B: self::_puroro::bitvec::BitSlice>(
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
    fn clear<B: self::_puroro::bitvec::BitSlice>(&mut self, bits: &mut B) {
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
    fn clone<B: self::_puroro::bitvec::BitSlice>(&self, bits: &B) -> Self {
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
        B: self::_puroro::bitvec::BitSlice,
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
        B: self::_puroro::bitvec::BitSlice,
    {
        todo!()
    }
}
impl ::std::default::Default for GroupOne {
    fn default() -> Self {
        Self { _none: () }
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
pub union GroupTwo {
    _none: (),
    g2_f32: ::std::mem::ManuallyDrop::<
        self::_puroro::internal::oneof_field_type::NumericalField::<
            f32,
            self::_puroro::tags::Float,
        >,
    >,
    g2_string: ::std::mem::ManuallyDrop::<
        self::_puroro::internal::oneof_field_type::StringField,
    >,
    g2_submsg: ::std::mem::ManuallyDrop::<
        self::_puroro::internal::oneof_field_type::HeapMessageField::<
            self::_puroro_root::oneofs3::Submsg,
        >,
    >,
}
pub enum GroupTwoCase<G2F32 = (), G2String = (), G2Submsg = ()> {
    G2F32(G2F32),
    G2String(G2String),
    G2Submsg(G2Submsg),
}
impl GroupTwo {
    pub(crate) fn g2_f32<B: self::_puroro::bitvec::BitSlice>(&self, bits: &B) -> f32 {
        #[allow(unused)]
        use ::std::option::Option::{None, Some};
        #[allow(unused)]
        use ::std::default::Default;
        use self::_puroro::internal::oneof_field_type::OneofFieldTypeOpt;
        use ::std::ops::Deref as _;
        use self::_puroro::internal::oneof_type::OneofCase as _;
        let case_opt = self::GroupTwoCase::from_bitslice(bits);
        let item_opt = matches!(case_opt, Some(self::GroupTwoCase::G2F32(())))
            .then(|| { unsafe { self.g2_f32.deref() } });
        OneofFieldTypeOpt::get_field(item_opt, Default::default)
    }
    pub(crate) fn g2_f32_opt<B: self::_puroro::bitvec::BitSlice>(
        &self,
        bits: &B,
    ) -> ::std::option::Option::<f32> {
        #[allow(unused)]
        use ::std::option::Option::{None, Some};
        use self::_puroro::internal::oneof_field_type::OneofFieldTypeOpt;
        use ::std::ops::Deref as _;
        use self::_puroro::internal::oneof_type::OneofCase as _;
        let case_opt = self::GroupTwoCase::from_bitslice(bits);
        let item_opt = matches!(case_opt, Some(self::GroupTwoCase::G2F32(())))
            .then(|| { unsafe { self.g2_f32.deref() } });
        OneofFieldTypeOpt::get_field_opt(item_opt)
    }
    pub(crate) fn g2_f32_mut<B: self::_puroro::bitvec::BitSlice>(
        &mut self,
        bits: &mut B,
    ) -> &mut f32 {
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
                g2_f32: ManuallyDrop::new(Default::default()),
            };
        }
        unsafe { &mut self.g2_f32 }.mut_field()
    }
    pub(crate) fn g2_string<B: self::_puroro::bitvec::BitSlice>(
        &self,
        bits: &B,
    ) -> &str {
        #[allow(unused)]
        use ::std::option::Option::{None, Some};
        #[allow(unused)]
        use ::std::default::Default;
        use self::_puroro::internal::oneof_field_type::OneofFieldTypeOpt;
        use ::std::ops::Deref as _;
        use self::_puroro::internal::oneof_type::OneofCase as _;
        let case_opt = self::GroupTwoCase::from_bitslice(bits);
        let item_opt = matches!(case_opt, Some(self::GroupTwoCase::G2String(())))
            .then(|| { unsafe { self.g2_string.deref() } });
        OneofFieldTypeOpt::get_field(item_opt, Default::default)
    }
    pub(crate) fn g2_string_opt<B: self::_puroro::bitvec::BitSlice>(
        &self,
        bits: &B,
    ) -> ::std::option::Option::<&str> {
        #[allow(unused)]
        use ::std::option::Option::{None, Some};
        use self::_puroro::internal::oneof_field_type::OneofFieldTypeOpt;
        use ::std::ops::Deref as _;
        use self::_puroro::internal::oneof_type::OneofCase as _;
        let case_opt = self::GroupTwoCase::from_bitslice(bits);
        let item_opt = matches!(case_opt, Some(self::GroupTwoCase::G2String(())))
            .then(|| { unsafe { self.g2_string.deref() } });
        OneofFieldTypeOpt::get_field_opt(item_opt)
    }
    pub(crate) fn g2_string_mut<B: self::_puroro::bitvec::BitSlice>(
        &mut self,
        bits: &mut B,
    ) -> &mut ::std::string::String {
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
                g2_string: ManuallyDrop::new(Default::default()),
            };
        }
        unsafe { &mut self.g2_string }.mut_field()
    }
    pub(crate) fn g2_submsg<B: self::_puroro::bitvec::BitSlice>(
        &self,
        bits: &B,
    ) -> ::std::option::Option::<&self::_puroro_root::oneofs3::Submsg> {
        #[allow(unused)]
        use ::std::option::Option::{None, Some};
        #[allow(unused)]
        use ::std::default::Default;
        use self::_puroro::internal::oneof_field_type::OneofFieldTypeOpt;
        use ::std::ops::Deref as _;
        use self::_puroro::internal::oneof_type::OneofCase as _;
        let case_opt = self::GroupTwoCase::from_bitslice(bits);
        let item_opt = matches!(case_opt, Some(self::GroupTwoCase::G2Submsg(())))
            .then(|| { unsafe { self.g2_submsg.deref() } });
        OneofFieldTypeOpt::get_field(item_opt, Default::default)
    }
    pub(crate) fn g2_submsg_opt<B: self::_puroro::bitvec::BitSlice>(
        &self,
        bits: &B,
    ) -> ::std::option::Option::<&self::_puroro_root::oneofs3::Submsg> {
        #[allow(unused)]
        use ::std::option::Option::{None, Some};
        use self::_puroro::internal::oneof_field_type::OneofFieldTypeOpt;
        use ::std::ops::Deref as _;
        use self::_puroro::internal::oneof_type::OneofCase as _;
        let case_opt = self::GroupTwoCase::from_bitslice(bits);
        let item_opt = matches!(case_opt, Some(self::GroupTwoCase::G2Submsg(())))
            .then(|| { unsafe { self.g2_submsg.deref() } });
        OneofFieldTypeOpt::get_field_opt(item_opt)
    }
    pub(crate) fn g2_submsg_mut<B: self::_puroro::bitvec::BitSlice>(
        &mut self,
        bits: &mut B,
    ) -> &mut self::_puroro_root::oneofs3::Submsg {
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
                g2_submsg: ManuallyDrop::new(Default::default()),
            };
        }
        unsafe { &mut self.g2_submsg }.mut_field()
    }
}
impl self::_puroro::internal::oneof_type::OneofUnion for GroupTwo {
    type Case = self::GroupTwoCase;
    type CaseRef<'a> = self::GroupTwoCase::<
        f32,
        &'a str,
        &'a self::_puroro_root::oneofs3::Submsg,
    >;
    fn case_ref<B: self::_puroro::bitvec::BitSlice>(
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
    fn clear<B: self::_puroro::bitvec::BitSlice>(&mut self, bits: &mut B) {
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
    fn clone<B: self::_puroro::bitvec::BitSlice>(&self, bits: &B) -> Self {
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
        B: self::_puroro::bitvec::BitSlice,
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
        B: self::_puroro::bitvec::BitSlice,
    {
        todo!()
    }
}
impl ::std::default::Default for GroupTwo {
    fn default() -> Self {
        Self { _none: () }
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
pub union GroupThree {
    _none: (),
    g3_int32: ::std::mem::ManuallyDrop::<
        self::_puroro::internal::oneof_field_type::NumericalField::<
            i32,
            self::_puroro::tags::Int32,
        >,
    >,
}
pub enum GroupThreeCase<G3Int32 = ()> {
    G3Int32(G3Int32),
}
impl GroupThree {
    pub(crate) fn g3_int32<B: self::_puroro::bitvec::BitSlice>(&self, bits: &B) -> i32 {
        #[allow(unused)]
        use ::std::option::Option::{None, Some};
        #[allow(unused)]
        use ::std::default::Default;
        use self::_puroro::internal::oneof_field_type::OneofFieldTypeOpt;
        use ::std::ops::Deref as _;
        use self::_puroro::internal::oneof_type::OneofCase as _;
        let case_opt = self::GroupThreeCase::from_bitslice(bits);
        let item_opt = matches!(case_opt, Some(self::GroupThreeCase::G3Int32(())))
            .then(|| { unsafe { self.g3_int32.deref() } });
        OneofFieldTypeOpt::get_field(item_opt, Default::default)
    }
    pub(crate) fn g3_int32_opt<B: self::_puroro::bitvec::BitSlice>(
        &self,
        bits: &B,
    ) -> ::std::option::Option::<i32> {
        #[allow(unused)]
        use ::std::option::Option::{None, Some};
        use self::_puroro::internal::oneof_field_type::OneofFieldTypeOpt;
        use ::std::ops::Deref as _;
        use self::_puroro::internal::oneof_type::OneofCase as _;
        let case_opt = self::GroupThreeCase::from_bitslice(bits);
        let item_opt = matches!(case_opt, Some(self::GroupThreeCase::G3Int32(())))
            .then(|| { unsafe { self.g3_int32.deref() } });
        OneofFieldTypeOpt::get_field_opt(item_opt)
    }
    pub(crate) fn g3_int32_mut<B: self::_puroro::bitvec::BitSlice>(
        &mut self,
        bits: &mut B,
    ) -> &mut i32 {
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
                g3_int32: ManuallyDrop::new(Default::default()),
            };
        }
        unsafe { &mut self.g3_int32 }.mut_field()
    }
}
impl self::_puroro::internal::oneof_type::OneofUnion for GroupThree {
    type Case = self::GroupThreeCase;
    type CaseRef<'a> = self::GroupThreeCase::<i32>;
    fn case_ref<B: self::_puroro::bitvec::BitSlice>(
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
    fn clear<B: self::_puroro::bitvec::BitSlice>(&mut self, bits: &mut B) {
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
    fn clone<B: self::_puroro::bitvec::BitSlice>(&self, bits: &B) -> Self {
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
        B: self::_puroro::bitvec::BitSlice,
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
        B: self::_puroro::bitvec::BitSlice,
    {
        todo!()
    }
}
impl ::std::default::Default for GroupThree {
    fn default() -> Self {
        Self { _none: () }
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
