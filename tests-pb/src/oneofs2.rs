pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}
pub mod _puroro {
    pub use ::puroro::*;
}
pub mod msg;
#[derive(::std::default::Default)]
pub struct Msg {
    group_one: self::_puroro_root::oneofs2::msg::GroupOne,
    group_two: self::_puroro_root::oneofs2::msg::GroupTwo,
    group_three: self::_puroro_root::oneofs2::msg::GroupThree,
    _bitfield: self::_puroro::bitvec::BitArray<1usize>,
}
impl Msg {
    pub fn group_one(
        &self,
    ) -> ::std::option::Option<
        <self::_puroro_root::oneofs2::msg::GroupOne as self::_puroro::internal::oneof_type::OneofUnion>::CaseRef<
            '_,
        >,
    > {
        use self::_puroro::internal::oneof_type::OneofUnion as _;
        self.group_one.case_ref(&self._bitfield)
    }
    pub fn clear_group_one(&mut self) {
        use self::_puroro::internal::oneof_type::OneofUnion as _;
        self.group_one.clear(&mut self._bitfield)
    }
    pub fn group_two(
        &self,
    ) -> ::std::option::Option<
        <self::_puroro_root::oneofs2::msg::GroupTwo as self::_puroro::internal::oneof_type::OneofUnion>::CaseRef<
            '_,
        >,
    > {
        use self::_puroro::internal::oneof_type::OneofUnion as _;
        self.group_two.case_ref(&self._bitfield)
    }
    pub fn clear_group_two(&mut self) {
        use self::_puroro::internal::oneof_type::OneofUnion as _;
        self.group_two.clear(&mut self._bitfield)
    }
    pub fn group_three(
        &self,
    ) -> ::std::option::Option<
        <self::_puroro_root::oneofs2::msg::GroupThree as self::_puroro::internal::oneof_type::OneofUnion>::CaseRef<
            '_,
        >,
    > {
        use self::_puroro::internal::oneof_type::OneofUnion as _;
        self.group_three.case_ref(&self._bitfield)
    }
    pub fn clear_group_three(&mut self) {
        use self::_puroro::internal::oneof_type::OneofUnion as _;
        self.group_three.clear(&mut self._bitfield)
    }
    pub fn g1_int32(&self) -> i32 {
        self.group_one.g1_int32(&self._bitfield)
    }
    pub fn g1_int32_opt(&self) -> ::std::option::Option::<i32> {
        self.group_one.g1_int32_opt(&self._bitfield)
    }
    pub fn g1_int32_mut(&mut self) -> &mut i32 {
        self.group_one.g1_int32_mut(&mut self._bitfield)
    }
    pub fn has_g1_int32(&self) -> bool {
        self.g1_int32_opt().is_some()
    }
    pub fn clear_g1_int32(&mut self) {
        #[allow(unused)]
        use ::std::option::Option::Some;
        use self::_puroro::internal::oneof_type::OneofCase;
        use self::_puroro::internal::oneof_type::OneofUnion;
        if let Some(self::_puroro_root::oneofs2::msg::GroupOneCase::G1Int32(_))
            = OneofCase::from_bitslice(&self._bitfield) {
            self.group_one.clear(&mut self._bitfield)
        }
    }
    pub fn g1_string(&self) -> &str {
        self.group_one.g1_string(&self._bitfield)
    }
    pub fn g1_string_opt(&self) -> ::std::option::Option::<&str> {
        self.group_one.g1_string_opt(&self._bitfield)
    }
    pub fn g1_string_mut(&mut self) -> &mut ::std::string::String {
        self.group_one.g1_string_mut(&mut self._bitfield)
    }
    pub fn has_g1_string(&self) -> bool {
        self.g1_string_opt().is_some()
    }
    pub fn clear_g1_string(&mut self) {
        #[allow(unused)]
        use ::std::option::Option::Some;
        use self::_puroro::internal::oneof_type::OneofCase;
        use self::_puroro::internal::oneof_type::OneofUnion;
        if let Some(self::_puroro_root::oneofs2::msg::GroupOneCase::G1String(_))
            = OneofCase::from_bitslice(&self._bitfield) {
            self.group_one.clear(&mut self._bitfield)
        }
    }
    pub fn g2_f32(&self) -> f32 {
        self.group_two.g2_f32(&self._bitfield)
    }
    pub fn g2_f32_opt(&self) -> ::std::option::Option::<f32> {
        self.group_two.g2_f32_opt(&self._bitfield)
    }
    pub fn g2_f32_mut(&mut self) -> &mut f32 {
        self.group_two.g2_f32_mut(&mut self._bitfield)
    }
    pub fn has_g2_f32(&self) -> bool {
        self.g2_f32_opt().is_some()
    }
    pub fn clear_g2_f32(&mut self) {
        #[allow(unused)]
        use ::std::option::Option::Some;
        use self::_puroro::internal::oneof_type::OneofCase;
        use self::_puroro::internal::oneof_type::OneofUnion;
        if let Some(self::_puroro_root::oneofs2::msg::GroupTwoCase::G2F32(_))
            = OneofCase::from_bitslice(&self._bitfield) {
            self.group_two.clear(&mut self._bitfield)
        }
    }
    pub fn g2_string(&self) -> &str {
        self.group_two.g2_string(&self._bitfield)
    }
    pub fn g2_string_opt(&self) -> ::std::option::Option::<&str> {
        self.group_two.g2_string_opt(&self._bitfield)
    }
    pub fn g2_string_mut(&mut self) -> &mut ::std::string::String {
        self.group_two.g2_string_mut(&mut self._bitfield)
    }
    pub fn has_g2_string(&self) -> bool {
        self.g2_string_opt().is_some()
    }
    pub fn clear_g2_string(&mut self) {
        #[allow(unused)]
        use ::std::option::Option::Some;
        use self::_puroro::internal::oneof_type::OneofCase;
        use self::_puroro::internal::oneof_type::OneofUnion;
        if let Some(self::_puroro_root::oneofs2::msg::GroupTwoCase::G2String(_))
            = OneofCase::from_bitslice(&self._bitfield) {
            self.group_two.clear(&mut self._bitfield)
        }
    }
    pub fn g2_submsg(
        &self,
    ) -> ::std::option::Option::<&self::_puroro_root::oneofs2::Submsg> {
        self.group_two.g2_submsg(&self._bitfield)
    }
    pub fn g2_submsg_opt(
        &self,
    ) -> ::std::option::Option::<&self::_puroro_root::oneofs2::Submsg> {
        self.group_two.g2_submsg_opt(&self._bitfield)
    }
    pub fn g2_submsg_mut(&mut self) -> &mut self::_puroro_root::oneofs2::Submsg {
        self.group_two.g2_submsg_mut(&mut self._bitfield)
    }
    pub fn has_g2_submsg(&self) -> bool {
        self.g2_submsg_opt().is_some()
    }
    pub fn clear_g2_submsg(&mut self) {
        #[allow(unused)]
        use ::std::option::Option::Some;
        use self::_puroro::internal::oneof_type::OneofCase;
        use self::_puroro::internal::oneof_type::OneofUnion;
        if let Some(self::_puroro_root::oneofs2::msg::GroupTwoCase::G2Submsg(_))
            = OneofCase::from_bitslice(&self._bitfield) {
            self.group_two.clear(&mut self._bitfield)
        }
    }
    pub fn g3_int32(&self) -> i32 {
        self.group_three.g3_int32(&self._bitfield)
    }
    pub fn g3_int32_opt(&self) -> ::std::option::Option::<i32> {
        self.group_three.g3_int32_opt(&self._bitfield)
    }
    pub fn g3_int32_mut(&mut self) -> &mut i32 {
        self.group_three.g3_int32_mut(&mut self._bitfield)
    }
    pub fn has_g3_int32(&self) -> bool {
        self.g3_int32_opt().is_some()
    }
    pub fn clear_g3_int32(&mut self) {
        #[allow(unused)]
        use ::std::option::Option::Some;
        use self::_puroro::internal::oneof_type::OneofCase;
        use self::_puroro::internal::oneof_type::OneofUnion;
        if let Some(self::_puroro_root::oneofs2::msg::GroupThreeCase::G3Int32(_))
            = OneofCase::from_bitslice(&self._bitfield) {
            self.group_three.clear(&mut self._bitfield)
        }
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
                    self
                        .group_one
                        .deser_from_iter(
                            &mut self._bitfield,
                            field_data,
                            self::_puroro_root::oneofs2::msg::GroupOneCase::G1Int32(()),
                        )?
                }
                2i32 => {
                    self
                        .group_one
                        .deser_from_iter(
                            &mut self._bitfield,
                            field_data,
                            self::_puroro_root::oneofs2::msg::GroupOneCase::G1String(()),
                        )?
                }
                3i32 => {
                    self
                        .group_two
                        .deser_from_iter(
                            &mut self._bitfield,
                            field_data,
                            self::_puroro_root::oneofs2::msg::GroupTwoCase::G2F32(()),
                        )?
                }
                4i32 => {
                    self
                        .group_two
                        .deser_from_iter(
                            &mut self._bitfield,
                            field_data,
                            self::_puroro_root::oneofs2::msg::GroupTwoCase::G2String(()),
                        )?
                }
                5i32 => {
                    self
                        .group_two
                        .deser_from_iter(
                            &mut self._bitfield,
                            field_data,
                            self::_puroro_root::oneofs2::msg::GroupTwoCase::G2Submsg(()),
                        )?
                }
                6i32 => {
                    self
                        .group_three
                        .deser_from_iter(
                            &mut self._bitfield,
                            field_data,
                            self::_puroro_root::oneofs2::msg::GroupThreeCase::G3Int32(()),
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
        self.group_one.ser_to_write(&self._bitfield, out)?;
        self.group_two.ser_to_write(&self._bitfield, out)?;
        self.group_three.ser_to_write(&self._bitfield, out)?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for Msg {
    fn clone(&self) -> Self {
        Self {
            group_one: <self::_puroro_root::oneofs2::msg::GroupOne as self::_puroro::internal::oneof_type::OneofUnion>::clone(
                &self.group_one,
                &self._bitfield,
            ),
            group_two: <self::_puroro_root::oneofs2::msg::GroupTwo as self::_puroro::internal::oneof_type::OneofUnion>::clone(
                &self.group_two,
                &self._bitfield,
            ),
            group_three: <self::_puroro_root::oneofs2::msg::GroupThree as self::_puroro::internal::oneof_type::OneofUnion>::clone(
                &self.group_three,
                &self._bitfield,
            ),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
impl ::std::ops::Drop for Msg {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion as _;
        self.group_one.clear(&mut self._bitfield);
        self.group_two.clear(&mut self._bitfield);
        self.group_three.clear(&mut self._bitfield);
    }
}
impl ::std::fmt::Debug for Msg {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct(stringify!(Msg))
            .field(stringify!(g1_int32), &self.g1_int32_opt())
            .field(stringify!(g1_string), &self.g1_string_opt())
            .field(stringify!(g2_f32), &self.g2_f32_opt())
            .field(stringify!(g2_string), &self.g2_string_opt())
            .field(stringify!(g2_submsg), &self.g2_submsg_opt())
            .field(stringify!(g3_int32), &self.g3_int32_opt())
            .finish()
    }
}
impl ::std::cmp::PartialEq for Msg {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion as _;
        true && self.group_one() == rhs.group_one()
            && self.group_two() == rhs.group_two()
            && self.group_three() == rhs.group_three()
    }
}
#[derive(::std::default::Default)]
pub struct Submsg {
    i32_optional: self::_puroro::internal::field_type::OptionalNumericalField::<
        i32,
        self::_puroro::tags::Int32,
        0usize,
    >,
    _bitfield: self::_puroro::bitvec::BitArray<1usize>,
}
impl Submsg {
    pub fn i32_optional(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::get_field(
            &self.i32_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn i32_optional_opt(&self) -> ::std::option::Option::<i32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.i32_optional, &self._bitfield)
    }
    pub fn i32_optional_mut(&mut self) -> &mut i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.i32_optional,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_i32_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.i32_optional, &self._bitfield)
            .is_some()
    }
    pub fn clear_i32_optional(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::clear(&mut self.i32_optional, &mut self._bitfield)
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
        use self::_puroro::internal::ser::FieldData;
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion as _;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            match number {
                1i32 => {
                    <self::_puroro::internal::field_type::OptionalNumericalField::<
                        i32,
                        self::_puroro::tags::Int32,
                        0usize,
                    > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.i32_optional,
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
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            0usize,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.i32_optional,
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
            i32_optional: <self::_puroro::internal::field_type::OptionalNumericalField::<
                i32,
                self::_puroro::tags::Int32,
                0usize,
            > as ::std::clone::Clone>::clone(&self.i32_optional),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
impl ::std::ops::Drop for Submsg {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for Submsg {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct(stringify!(Submsg))
            .field(stringify!(i32_optional), &self.i32_optional_opt())
            .finish()
    }
}
impl ::std::cmp::PartialEq for Submsg {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion as _;
        true && self.i32_optional_opt() == rhs.i32_optional_opt()
    }
}
