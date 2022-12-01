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
        todo!()
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
        todo!()
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
            self::_puroro_root::oneofs2::Submsg,
        >,
    >,
}
pub enum GroupTwoCase<G2F32 = (), G2String = (), G2Submsg = ()> {
    G2F32(G2F32),
    G2String(G2String),
    G2Submsg(G2Submsg),
}
impl self::_puroro::internal::oneof_type::OneofUnion for GroupTwo {
    type Case = self::GroupTwoCase;
    type CaseRef<'a> = self::GroupTwoCase::<
        f32,
        &'a str,
        &'a self::_puroro_root::oneofs2::Submsg,
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
        todo!()
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
        todo!()
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
        todo!()
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
        todo!()
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
