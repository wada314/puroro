// A generated source code by puroro library
// package .google.protobuf.DescriptorProto

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub mod _puroro {
    pub use ::puroro::*;
}

#[derive(Default)]
pub struct ExtensionRange {
    // Optional, Variant(Int32)
    start: self::_puroro::internal::field_type::OptionalNumericalField<
        i32,
        self::_puroro::tags::Int32,
        0,
    >,
    // Optional, Variant(Int32)
    end: self::_puroro::internal::field_type::OptionalNumericalField<
        i32,
        self::_puroro::tags::Int32,
        1,
    >,
    // Optional, LengthDelimited(Message(Fqtn(".google.protobuf.ExtensionRangeOptions")))
    options: self::_puroro::internal::field_type::SingularHeapMessageField<
        _puroro_root::google::protobuf::ExtensionRangeOptions,
    >,

    _bitfield: self::_puroro::bitvec::BitArray<1>,
}

impl ExtensionRange {
    // Optional, Variant(Int32)
    pub fn start(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as NonRepeatedFieldType>::get_field(
            &self.start,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn start_opt(&self) -> ::std::option::Option<i32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as NonRepeatedFieldType>::get_field_opt(&self.start, &self._bitfield)
    }
    pub fn has_start(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as NonRepeatedFieldType>::get_field_opt(&self.start, &self._bitfield)
        .is_some()
    }
    pub fn start_mut(&mut self) -> &mut i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.start,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_start(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as NonRepeatedFieldType>::clear(&mut self.start, &mut self._bitfield)
    }
    // Optional, Variant(Int32)
    pub fn end(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            1,
        > as NonRepeatedFieldType>::get_field(
            &self.end,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn end_opt(&self) -> ::std::option::Option<i32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            1,
        > as NonRepeatedFieldType>::get_field_opt(&self.end, &self._bitfield)
    }
    pub fn has_end(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            1,
        > as NonRepeatedFieldType>::get_field_opt(&self.end, &self._bitfield)
        .is_some()
    }
    pub fn end_mut(&mut self) -> &mut i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            1,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.end,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_end(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            1,
        > as NonRepeatedFieldType>::clear(&mut self.end, &mut self._bitfield)
    }
    // Optional, LengthDelimited(Message(Fqtn(".google.protobuf.ExtensionRangeOptions")))
    pub fn options(
        &self,
    ) -> ::std::option::Option<&_puroro_root::google::protobuf::ExtensionRangeOptions> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::ExtensionRangeOptions,
        > as NonRepeatedFieldType>::get_field(
            &self.options,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn options_opt(
        &self,
    ) -> ::std::option::Option<&_puroro_root::google::protobuf::ExtensionRangeOptions> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::ExtensionRangeOptions,
        > as NonRepeatedFieldType>::get_field_opt(&self.options, &self._bitfield)
    }
    pub fn has_options(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::ExtensionRangeOptions,
        > as NonRepeatedFieldType>::get_field_opt(&self.options, &self._bitfield)
        .is_some()
    }
    pub fn options_mut(&mut self) -> &mut _puroro_root::google::protobuf::ExtensionRangeOptions {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::ExtensionRangeOptions,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.options,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_options(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::ExtensionRangeOptions,
        > as NonRepeatedFieldType>::clear(&mut self.options, &mut self._bitfield)
    }
}

impl self::_puroro::Message for ExtensionRange {
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
        use self::_puroro::internal::field_type::FieldType;
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        #[allow(unused)]
        use ::std::option::Option::Some;
        #[allow(unused)]
        use ::std::result::Result::Ok;
        while let Some((number, field_data)) =
            self::_puroro::internal::ser::FieldData::from_bytes_iter(iter.by_ref())?
        {
            match number {
                1 => <self::_puroro::internal::field_type::OptionalNumericalField<
                    i32,
                    self::_puroro::tags::Int32,
                    0,
                > as FieldType>::deser_from_iter(
                    &mut self.start, &mut self._bitfield, field_data
                )?,
                2 => <self::_puroro::internal::field_type::OptionalNumericalField<
                    i32,
                    self::_puroro::tags::Int32,
                    1,
                > as FieldType>::deser_from_iter(
                    &mut self.end, &mut self._bitfield, field_data
                )?,
                3 => <self::_puroro::internal::field_type::SingularHeapMessageField<
                    _puroro_root::google::protobuf::ExtensionRangeOptions,
                > as FieldType>::deser_from_iter(
                    &mut self.options,
                    &mut self._bitfield,
                    field_data,
                )?,
                _ => todo!(), // Unknown field...
            }
        }
        Ok(())
    }

    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)] out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use self::_puroro::internal::field_type::FieldType;
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        #[allow(unused)]
        use ::std::result::Result::Ok;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as FieldType>::ser_to_write(&self.start, &self._bitfield, 1, out)?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            1,
        > as FieldType>::ser_to_write(&self.end, &self._bitfield, 2, out)?;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::ExtensionRangeOptions,
        > as FieldType>::ser_to_write(&self.options, &self._bitfield, 3, out)?;

        Ok(())
    }
}

impl ::std::clone::Clone for ExtensionRange {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        #[allow(unused)]
        use ::std::clone::Clone;
        Self {
            start: Clone::clone(&self.start),
            end: Clone::clone(&self.end),
            options: Clone::clone(&self.options),

            _bitfield: Clone::clone(&self._bitfield),
        }
    }
}

impl ::std::fmt::Debug for ExtensionRange {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct("ExtensionRange")
            .field("start", &self.start())
            .field("end", &self.end())
            .field("options", &self.options())
            .finish()
    }
}

impl ::std::cmp::PartialEq for ExtensionRange {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;

        true && self.start_opt() == rhs.start_opt()
            && self.end_opt() == rhs.end_opt()
            && self.options_opt() == rhs.options_opt()
    }
}

impl ::std::ops::Drop for ExtensionRange {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
    }
}

#[derive(Default)]
pub struct ReservedRange {
    // Optional, Variant(Int32)
    start: self::_puroro::internal::field_type::OptionalNumericalField<
        i32,
        self::_puroro::tags::Int32,
        0,
    >,
    // Optional, Variant(Int32)
    end: self::_puroro::internal::field_type::OptionalNumericalField<
        i32,
        self::_puroro::tags::Int32,
        1,
    >,

    _bitfield: self::_puroro::bitvec::BitArray<1>,
}

impl ReservedRange {
    // Optional, Variant(Int32)
    pub fn start(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as NonRepeatedFieldType>::get_field(
            &self.start,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn start_opt(&self) -> ::std::option::Option<i32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as NonRepeatedFieldType>::get_field_opt(&self.start, &self._bitfield)
    }
    pub fn has_start(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as NonRepeatedFieldType>::get_field_opt(&self.start, &self._bitfield)
        .is_some()
    }
    pub fn start_mut(&mut self) -> &mut i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.start,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_start(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as NonRepeatedFieldType>::clear(&mut self.start, &mut self._bitfield)
    }
    // Optional, Variant(Int32)
    pub fn end(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            1,
        > as NonRepeatedFieldType>::get_field(
            &self.end,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn end_opt(&self) -> ::std::option::Option<i32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            1,
        > as NonRepeatedFieldType>::get_field_opt(&self.end, &self._bitfield)
    }
    pub fn has_end(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            1,
        > as NonRepeatedFieldType>::get_field_opt(&self.end, &self._bitfield)
        .is_some()
    }
    pub fn end_mut(&mut self) -> &mut i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            1,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.end,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_end(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            1,
        > as NonRepeatedFieldType>::clear(&mut self.end, &mut self._bitfield)
    }
}

impl self::_puroro::Message for ReservedRange {
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
        use self::_puroro::internal::field_type::FieldType;
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        #[allow(unused)]
        use ::std::option::Option::Some;
        #[allow(unused)]
        use ::std::result::Result::Ok;
        while let Some((number, field_data)) =
            self::_puroro::internal::ser::FieldData::from_bytes_iter(iter.by_ref())?
        {
            match number {
                1 => <self::_puroro::internal::field_type::OptionalNumericalField<
                    i32,
                    self::_puroro::tags::Int32,
                    0,
                > as FieldType>::deser_from_iter(
                    &mut self.start, &mut self._bitfield, field_data
                )?,
                2 => <self::_puroro::internal::field_type::OptionalNumericalField<
                    i32,
                    self::_puroro::tags::Int32,
                    1,
                > as FieldType>::deser_from_iter(
                    &mut self.end, &mut self._bitfield, field_data
                )?,
                _ => todo!(), // Unknown field...
            }
        }
        Ok(())
    }

    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)] out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use self::_puroro::internal::field_type::FieldType;
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        #[allow(unused)]
        use ::std::result::Result::Ok;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as FieldType>::ser_to_write(&self.start, &self._bitfield, 1, out)?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            1,
        > as FieldType>::ser_to_write(&self.end, &self._bitfield, 2, out)?;

        Ok(())
    }
}

impl ::std::clone::Clone for ReservedRange {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        #[allow(unused)]
        use ::std::clone::Clone;
        Self {
            start: Clone::clone(&self.start),
            end: Clone::clone(&self.end),

            _bitfield: Clone::clone(&self._bitfield),
        }
    }
}

impl ::std::fmt::Debug for ReservedRange {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct("ReservedRange")
            .field("start", &self.start())
            .field("end", &self.end())
            .finish()
    }
}

impl ::std::cmp::PartialEq for ReservedRange {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;

        true && self.start_opt() == rhs.start_opt() && self.end_opt() == rhs.end_opt()
    }
}

impl ::std::ops::Drop for ReservedRange {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
    }
}
