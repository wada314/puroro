pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}
pub mod _puroro {
    pub use ::puroro::*;
}
pub(super) union GroupOne {
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
    type CaseRef<'a> = self::GroupOneCase::<i32, &str>;
    fn case_ref<B: self::_puroro::bitvec::BitSlice>(
        &self,
        bits: &B,
    ) -> ::std::option::Option<Self::CaseRef<'_>> {
        todo!()
    }
    fn clear<B: self::_puroro::bitvec::BitSlice>(&mut self, bits: &mut B) {
        todo!()
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
impl self::_puroro::internal::oneof_type::OneofCase for GroupOneCase {
    const BITFIELD_BEGIN: usize = 0usize;
    const BITFIELD_END: usize = 2usize;
    fn from_u32(x: u32) -> Option<Self> {
        todo!()
    }
    fn into_u32(self) -> u32 {
        todo!()
    }
}
impl<'a> self::_puroro::internal::oneof_type::OneofCaseRef<'a>
for GroupOneCase<i32, &str> {
    type Case = self::GroupOneCase;
    type Union = self::GroupOne;
    fn from_union_and_case(u: &'a Self::Union, case: Self::Case) -> Self {
        todo!()
    }
}
pub(super) union GroupTwo {
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
impl self::_puroro::internal::oneof_type::OneofUnion for GroupTwo {
    type Case = self::GroupTwoCase;
    type CaseRef<'a> = self::GroupTwoCase::<
        f32,
        &str,
        &self::_puroro_root::oneofs3::Submsg,
    >;
    fn case_ref<B: self::_puroro::bitvec::BitSlice>(
        &self,
        bits: &B,
    ) -> ::std::option::Option<Self::CaseRef<'_>> {
        todo!()
    }
    fn clear<B: self::_puroro::bitvec::BitSlice>(&mut self, bits: &mut B) {
        todo!()
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
impl self::_puroro::internal::oneof_type::OneofCase for GroupTwoCase {
    const BITFIELD_BEGIN: usize = 2usize;
    const BITFIELD_END: usize = 5usize;
    fn from_u32(x: u32) -> Option<Self> {
        todo!()
    }
    fn into_u32(self) -> u32 {
        todo!()
    }
}
impl<'a> self::_puroro::internal::oneof_type::OneofCaseRef<'a>
for GroupTwoCase<f32, &str, &self::_puroro_root::oneofs3::Submsg> {
    type Case = self::GroupTwoCase;
    type Union = self::GroupTwo;
    fn from_union_and_case(u: &'a Self::Union, case: Self::Case) -> Self {
        todo!()
    }
}
pub(super) union GroupThree {
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
        todo!()
    }
    fn clear<B: self::_puroro::bitvec::BitSlice>(&mut self, bits: &mut B) {
        todo!()
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
impl self::_puroro::internal::oneof_type::OneofCase for GroupThreeCase {
    const BITFIELD_BEGIN: usize = 3usize;
    const BITFIELD_END: usize = 5usize;
    fn from_u32(x: u32) -> Option<Self> {
        todo!()
    }
    fn into_u32(self) -> u32 {
        todo!()
    }
}
impl<'a> self::_puroro::internal::oneof_type::OneofCaseRef<'a> for GroupThreeCase<i32> {
    type Case = self::GroupThreeCase;
    type Union = self::GroupThree;
    fn from_union_and_case(u: &'a Self::Union, case: Self::Case) -> Self {
        todo!()
    }
}
