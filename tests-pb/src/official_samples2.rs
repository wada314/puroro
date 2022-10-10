// A generated source code by puroro library
// package official_samples2

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub mod _puroro {
    pub use ::puroro::*;
}

#[derive(Default)]
pub struct Test1 {
    // Optional, Variant(Int32)
    a: self::_puroro::internal::field_type::OptionalNumericalField<
        i32,
        self::_puroro::tags::Int32,
        0,
    >,

    _bitfield: self::_puroro::bitvec::BitArray<1>,
}

impl Test1 {
    // Optional, Variant(Int32)
    pub fn a(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as NonRepeatedFieldType>::get_field(
            &self.a,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn a_opt(&self) -> ::std::option::Option<i32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as NonRepeatedFieldType>::get_field_opt(&self.a, &self._bitfield)
    }
    pub fn has_a(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as NonRepeatedFieldType>::get_field_opt(&self.a, &self._bitfield)
        .is_some()
    }
    pub fn a_mut(&mut self) -> &mut i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.a,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_a(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as NonRepeatedFieldType>::clear(&mut self.a, &mut self._bitfield)
    }
}

impl self::_puroro::Message for Test1 {
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
                    &mut self.a,
                    &mut self._bitfield,
                    _field_data,
                )?,
                _ => todo!(),
            }
        }
        Ok(())
    }

    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)] out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use ::std::result::Result::Ok;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.a,
            &self._bitfield,
            1,
            out,
        )?;

        Ok(())
    }
}

impl ::std::clone::Clone for Test1 {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        #[allow(unused)]
        use ::std::clone::Clone;
        Self {
            a: Clone::clone(&self.a),

            _bitfield: Clone::clone(&self._bitfield),
        }
    }
}

impl ::std::fmt::Debug for Test1 {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct("Test1").field("a", &self.a()).finish()
    }
}

impl ::std::cmp::PartialEq for Test1 {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;

        true && self.a_opt() == rhs.a_opt()
    }
}

impl ::std::ops::Drop for Test1 {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
    }
}

#[derive(Default)]
pub struct Test2 {
    // Optional, LengthDelimited(String)
    b: self::_puroro::internal::field_type::OptionalStringField<0>,

    _bitfield: self::_puroro::bitvec::BitArray<1>,
}

impl Test2 {
    // Optional, LengthDelimited(String)
    pub fn b(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field(
            &self.b, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn b_opt(&self) -> ::std::option::Option<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field_opt(
            &self.b, &self._bitfield,
        )
    }
    pub fn has_b(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field_opt(
            &self.b, &self._bitfield,
        ).is_some()
    }
    pub fn b_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::mut_field(
            &mut self.b, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_b(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField<0> as NonRepeatedFieldType>::clear(
            &mut self.b,
            &mut self._bitfield,
        )
    }
}

impl self::_puroro::Message for Test2 {
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
                2 => <
                    self::_puroro::internal::field_type::OptionalStringField::<0> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.b,
                    &mut self._bitfield,
                    _field_data,
                )?,
                _ => todo!(),
            }
        }
        Ok(())
    }

    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)] out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use ::std::result::Result::Ok;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.b,
            &self._bitfield,
            2,
            out
        )?;

        Ok(())
    }
}

impl ::std::clone::Clone for Test2 {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        #[allow(unused)]
        use ::std::clone::Clone;
        Self {
            b: Clone::clone(&self.b),

            _bitfield: Clone::clone(&self._bitfield),
        }
    }
}

impl ::std::fmt::Debug for Test2 {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct("Test2").field("b", &self.b()).finish()
    }
}

impl ::std::cmp::PartialEq for Test2 {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;

        true && self.b_opt() == rhs.b_opt()
    }
}

impl ::std::ops::Drop for Test2 {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
    }
}

#[derive(Default)]
pub struct Test3 {
    // Optional, LengthDelimited(Message(Fqtn(".official_samples2.Test1")))
    c: self::_puroro::internal::field_type::SingularHeapMessageField<
        _puroro_root::official_samples2::Test1,
    >,

    _bitfield: self::_puroro::bitvec::BitArray<1>,
}

impl Test3 {
    // Optional, LengthDelimited(Message(Fqtn(".official_samples2.Test1")))
    pub fn c(&self) -> ::std::option::Option<&_puroro_root::official_samples2::Test1> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::official_samples2::Test1,
        > as NonRepeatedFieldType>::get_field(
            &self.c,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn c_opt(&self) -> ::std::option::Option<&_puroro_root::official_samples2::Test1> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::official_samples2::Test1,
        > as NonRepeatedFieldType>::get_field_opt(&self.c, &self._bitfield)
    }
    pub fn has_c(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::official_samples2::Test1,
        > as NonRepeatedFieldType>::get_field_opt(&self.c, &self._bitfield)
        .is_some()
    }
    pub fn c_mut(&mut self) -> &mut _puroro_root::official_samples2::Test1 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::official_samples2::Test1,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.c,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_c(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::official_samples2::Test1,
        > as NonRepeatedFieldType>::clear(&mut self.c, &mut self._bitfield)
    }
}

impl self::_puroro::Message for Test3 {
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
                3 => <self::_puroro::internal::field_type::SingularHeapMessageField<
                    _puroro_root::official_samples2::Test1,
                > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                    &mut self.c,
                    &mut self._bitfield,
                    _field_data,
                )?,
                _ => todo!(),
            }
        }
        Ok(())
    }

    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)] out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use ::std::result::Result::Ok;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::official_samples2::Test1,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.c,
            &self._bitfield,
            3,
            out,
        )?;

        Ok(())
    }
}

impl ::std::clone::Clone for Test3 {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        #[allow(unused)]
        use ::std::clone::Clone;
        Self {
            c: Clone::clone(&self.c),

            _bitfield: Clone::clone(&self._bitfield),
        }
    }
}

impl ::std::fmt::Debug for Test3 {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct("Test3").field("c", &self.c()).finish()
    }
}

impl ::std::cmp::PartialEq for Test3 {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;

        true && self.c_opt() == rhs.c_opt()
    }
}

impl ::std::ops::Drop for Test3 {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
    }
}

#[derive(Default)]
pub struct Test4 {
    // Repeated, Variant(Int32)
    d: self::_puroro::internal::field_type::RepeatedNumericalField<i32, self::_puroro::tags::Int32>,

    _bitfield: self::_puroro::bitvec::BitArray<0>,
}

impl Test4 {
    // Repeated, Variant(Int32)
    pub fn d(&self) -> &[i32] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<i32, self::_puroro::tags::Int32> as RepeatedFieldType>::get_field(
            &self.d, &self._bitfield, 
        )
    }
    pub fn d_mut(&mut self) -> &mut ::std::vec::Vec<i32> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<i32, self::_puroro::tags::Int32> as RepeatedFieldType>::mut_field(
            &mut self.d, &mut self._bitfield, 
        )
    }
    pub fn clear_d(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<i32, self::_puroro::tags::Int32> as RepeatedFieldType>::clear(
            &mut self.d, &mut self._bitfield, 
        )
    }
}

impl self::_puroro::Message for Test4 {
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
                4 => <self::_puroro::internal::field_type::RepeatedNumericalField<
                    i32,
                    self::_puroro::tags::Int32,
                > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                    &mut self.d,
                    &mut self._bitfield,
                    _field_data,
                )?,
                _ => todo!(),
            }
        }
        Ok(())
    }

    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)] out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use ::std::result::Result::Ok;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            i32,
            self::_puroro::tags::Int32,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.d,
            &self._bitfield,
            4,
            out,
        )?;

        Ok(())
    }
}

impl ::std::clone::Clone for Test4 {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        #[allow(unused)]
        use ::std::clone::Clone;
        Self {
            d: Clone::clone(&self.d),

            _bitfield: Clone::clone(&self._bitfield),
        }
    }
}

impl ::std::fmt::Debug for Test4 {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct("Test4").field("d", &self.d()).finish()
    }
}

impl ::std::cmp::PartialEq for Test4 {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;

        true && self.d() == rhs.d()
    }
}

impl ::std::ops::Drop for Test4 {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
    }
}
