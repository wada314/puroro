// A generated source code by puroro library
// package oneofs2
pub mod msg;

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub mod _puroro {
    pub use ::puroro::*;
}

#[derive(Default)]
pub struct Msg {
    // oneof GroupOne
    group_one: _puroro_root::oneofs2::msg::GroupOne,
    // oneof GroupTwo
    group_two: _puroro_root::oneofs2::msg::GroupTwo,
    // oneof GroupThree
    group_three: _puroro_root::oneofs2::msg::GroupThree,

    _bitfield: self::_puroro::bitvec::BitArray<1>,
}

impl Msg {
    pub fn group_one(
        &self,
    ) -> ::std::option::Option<_puroro_root::oneofs2::msg::GroupOneCaseRef<'_>> {
        use self::_puroro::internal::oneof_type::{OneofCase, OneofCaseRef};
        let case_opt =
            <_puroro_root::oneofs2::msg::GroupOneCase as OneofCase>::from_bitslice(&self._bitfield);
        case_opt.map(|case| OneofCaseRef::from_union_and_case(&self.group_one, case))
    }

    pub fn clear_group_one(&mut self) {
        use self::_puroro::internal::oneof_type::OneofUnion;
        <_puroro_root::oneofs2::msg::GroupOne as OneofUnion>::clear(
            &mut self.group_one,
            &mut self._bitfield,
        );
    }
    pub fn g1_int32_opt(&self) -> ::std::option::Option<i32> {
        self.group_one.g1_int32_opt(&self._bitfield)
    }

    pub fn has_g1_int32(&self) -> bool {
        self.g1_int32_opt.is_some()
    }

    pub fn clear_g1_int32(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofCase;
        #[allow(unused)]
        use ::std::option::Option::Some;
        match <_puroro_root::oneofs2::msg::GroupOneCase as OneofCase>::from_bitslice(
            &self._bitfield,
        ) {
            Some(_puroro_root::oneofs2::msg::GroupOne::G1Int32) => {
                self.clear_group_one();
            }
            _ => (),
        }
    }
    pub fn g1_string_opt(&self) -> ::std::option::Option<&str> {
        self.group_one.g1_string_opt(&self._bitfield)
    }

    pub fn has_g1_string(&self) -> bool {
        self.g1_string_opt.is_some()
    }

    pub fn clear_g1_string(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofCase;
        #[allow(unused)]
        use ::std::option::Option::Some;
        match <_puroro_root::oneofs2::msg::GroupOneCase as OneofCase>::from_bitslice(
            &self._bitfield,
        ) {
            Some(_puroro_root::oneofs2::msg::GroupOne::G1String) => {
                self.clear_group_one();
            }
            _ => (),
        }
    }
    pub fn group_two(
        &self,
    ) -> ::std::option::Option<_puroro_root::oneofs2::msg::GroupTwoCaseRef<'_>> {
        use self::_puroro::internal::oneof_type::{OneofCase, OneofCaseRef};
        let case_opt =
            <_puroro_root::oneofs2::msg::GroupTwoCase as OneofCase>::from_bitslice(&self._bitfield);
        case_opt.map(|case| OneofCaseRef::from_union_and_case(&self.group_two, case))
    }

    pub fn clear_group_two(&mut self) {
        use self::_puroro::internal::oneof_type::OneofUnion;
        <_puroro_root::oneofs2::msg::GroupTwo as OneofUnion>::clear(
            &mut self.group_two,
            &mut self._bitfield,
        );
    }
    pub fn g2_f32_opt(&self) -> ::std::option::Option<f32> {
        self.group_two.g2_f32_opt(&self._bitfield)
    }

    pub fn has_g2_f32(&self) -> bool {
        self.g2_f32_opt.is_some()
    }

    pub fn clear_g2_f32(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofCase;
        #[allow(unused)]
        use ::std::option::Option::Some;
        match <_puroro_root::oneofs2::msg::GroupTwoCase as OneofCase>::from_bitslice(
            &self._bitfield,
        ) {
            Some(_puroro_root::oneofs2::msg::GroupTwo::G2F32) => {
                self.clear_group_two();
            }
            _ => (),
        }
    }
    pub fn g2_string_opt(&self) -> ::std::option::Option<&str> {
        self.group_two.g2_string_opt(&self._bitfield)
    }

    pub fn has_g2_string(&self) -> bool {
        self.g2_string_opt.is_some()
    }

    pub fn clear_g2_string(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofCase;
        #[allow(unused)]
        use ::std::option::Option::Some;
        match <_puroro_root::oneofs2::msg::GroupTwoCase as OneofCase>::from_bitslice(
            &self._bitfield,
        ) {
            Some(_puroro_root::oneofs2::msg::GroupTwo::G2String) => {
                self.clear_group_two();
            }
            _ => (),
        }
    }
    pub fn g2_submsg_opt(&self) -> ::std::option::Option<&_puroro_root::oneofs2::Submsg> {
        self.group_two.g2_submsg_opt(&self._bitfield)
    }

    pub fn has_g2_submsg(&self) -> bool {
        self.g2_submsg_opt.is_some()
    }

    pub fn clear_g2_submsg(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofCase;
        #[allow(unused)]
        use ::std::option::Option::Some;
        match <_puroro_root::oneofs2::msg::GroupTwoCase as OneofCase>::from_bitslice(
            &self._bitfield,
        ) {
            Some(_puroro_root::oneofs2::msg::GroupTwo::G2Submsg) => {
                self.clear_group_two();
            }
            _ => (),
        }
    }
    pub fn group_three(
        &self,
    ) -> ::std::option::Option<_puroro_root::oneofs2::msg::GroupThreeCaseRef> {
        use self::_puroro::internal::oneof_type::{OneofCase, OneofCaseRef};
        let case_opt = <_puroro_root::oneofs2::msg::GroupThreeCase as OneofCase>::from_bitslice(
            &self._bitfield,
        );
        case_opt.map(|case| OneofCaseRef::from_union_and_case(&self.group_three, case))
    }

    pub fn clear_group_three(&mut self) {
        use self::_puroro::internal::oneof_type::OneofUnion;
        <_puroro_root::oneofs2::msg::GroupThree as OneofUnion>::clear(
            &mut self.group_three,
            &mut self._bitfield,
        );
    }
    pub fn g3_int32_opt(&self) -> ::std::option::Option<i32> {
        self.group_three.g3_int32_opt(&self._bitfield)
    }

    pub fn has_g3_int32(&self) -> bool {
        self.g3_int32_opt.is_some()
    }

    pub fn clear_g3_int32(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofCase;
        #[allow(unused)]
        use ::std::option::Option::Some;
        match <_puroro_root::oneofs2::msg::GroupThreeCase as OneofCase>::from_bitslice(
            &self._bitfield,
        ) {
            Some(_puroro_root::oneofs2::msg::GroupThree::G3Int32) => {
                self.clear_group_three();
            }
            _ => (),
        }
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
                _ => todo!(),
            }
        }
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
            group_one: <_puroro_root::oneofs2::msg::GroupOne as OneofUnion>::clone(
                &self.group_one,
                &self._bitfield,
            ),
            group_two: <_puroro_root::oneofs2::msg::GroupTwo as OneofUnion>::clone(
                &self.group_two,
                &self._bitfield,
            ),
            group_three: <_puroro_root::oneofs2::msg::GroupThree as OneofUnion>::clone(
                &self.group_three,
                &self._bitfield,
            ),

            _bitfield: Clone::clone(&self._bitfield),
        }
    }
}

impl ::std::fmt::Debug for Msg {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct("Msg").finish()
    }
}

impl ::std::ops::Drop for Msg {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        <_puroro_root::oneofs2::msg::GroupOne as OneofUnion>::clear(
            &mut self.group_one,
            &mut self._bitfield,
        );
        <_puroro_root::oneofs2::msg::GroupTwo as OneofUnion>::clear(
            &mut self.group_two,
            &mut self._bitfield,
        );
        <_puroro_root::oneofs2::msg::GroupThree as OneofUnion>::clear(
            &mut self.group_three,
            &mut self._bitfield,
        );
    }
}

#[derive(Default)]
pub struct Submsg {
    // Optional, Variant(Int32)
    i32_optional: self::_puroro::internal::field_type::OptionalNumericalField<
        i32,
        self::_puroro::tags::Int32,
        0,
    >,

    _bitfield: self::_puroro::bitvec::BitArray<1>,
}

impl Submsg {
    // Optional, Variant(Int32)
    pub fn i32_optional(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as NonRepeatedFieldType>::get_field(
            &self.i32_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn i32_optional_opt(&self) -> ::std::option::Option<i32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as NonRepeatedFieldType>::get_field_opt(&self.i32_optional, &self._bitfield)
    }
    pub fn has_i32_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as NonRepeatedFieldType>::get_field_opt(&self.i32_optional, &self._bitfield)
        .is_some()
    }
    pub fn i32_optional_mut(&mut self) -> &mut i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.i32_optional,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_i32_optional(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as NonRepeatedFieldType>::clear(&mut self.i32_optional, &mut self._bitfield)
    }
}

impl self::_puroro::Message for Submsg {
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
                1 => <self::_puroro::internal::field_type::OptionalNumericalField<
                    i32,
                    self::_puroro::tags::Int32,
                    0,
                > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                    &mut self.i32_optional,
                    &mut self._bitfield,
                    _field_data,
                )?,
                _ => todo!(),
            }
        }
        Ok(())
    }
}

impl ::std::clone::Clone for Submsg {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        #[allow(unused)]
        use ::std::clone::Clone;
        Self {
            i32_optional: Clone::clone(&self.i32_optional),

            _bitfield: Clone::clone(&self._bitfield),
        }
    }
}

impl ::std::fmt::Debug for Submsg {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct("Submsg")
            .field("i32_optional", &self.i32_optional())
            .finish()
    }
}

impl ::std::ops::Drop for Submsg {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
    }
}
