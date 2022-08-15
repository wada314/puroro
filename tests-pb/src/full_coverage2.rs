
// A generated source code by puroro library
// package full_coverage2
pub mod _msg;

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub mod _puroro {
    pub use ::puroro::*;
}

#[derive(Default, Clone)]
pub struct Msg {
    // Optional, Variant(Int32)
    i32_required: self::_puroro::internal::field_types::OptionalVariantField<i32, self::_puroro::tags::Int32, 0>,
    // Optional, Variant(Int32)
    i32_optional: self::_puroro::internal::field_types::OptionalVariantField<i32, self::_puroro::tags::Int32, 1>,
    // Optional, Bits32(Float)
    float_required: self::_puroro::internal::field_types::,
    // Optional, Bits32(Float)
    float_optional: self::_puroro::internal::field_types::,
    // Optional, LengthDelimited(Bytes)
    bytes_required: self::_puroro::internal::field_types::,
    // Optional, LengthDelimited(Bytes)
    bytes_optional: self::_puroro::internal::field_types::,
    // Optional, LengthDelimited(String)
    string_required: self::_puroro::internal::field_types::OptionalStringField<6>,
    // Optional, LengthDelimited(String)
    string_optional: self::_puroro::internal::field_types::OptionalStringField<7>,
    // Optional, Variant(Enum2(Fqtn(".full_coverage2.Enum")))
    enum_required: self::_puroro::internal::field_types::OptionalVariantField<_puroro_root::full_coverage2::Enum, self::_puroro::tags::Enum2<_puroro_root::full_coverage2::Enum>, 8>,
    // Optional, Variant(Enum2(Fqtn(".full_coverage2.Enum")))
    enum_optional: self::_puroro::internal::field_types::OptionalVariantField<_puroro_root::full_coverage2::Enum, self::_puroro::tags::Enum2<_puroro_root::full_coverage2::Enum>, 9>,
    // Optional, LengthDelimited(Message(Fqtn(".full_coverage2.Msg.Submsg")))
    submsg_required: self::_puroro::internal::field_types::SingularHeapMessageField<_puroro_root::full_coverage2::_msg::Submsg>,
    // Optional, LengthDelimited(Message(Fqtn(".full_coverage2.Msg.Submsg")))
    submsg_optional: self::_puroro::internal::field_types::SingularHeapMessageField<_puroro_root::full_coverage2::_msg::Submsg>,
    // Optional, Variant(Int64)
    i64_required: self::_puroro::internal::field_types::OptionalVariantField<i64, self::_puroro::tags::Int64, 12>,
    // Optional, Variant(Int64)
    i64_optional: self::_puroro::internal::field_types::OptionalVariantField<i64, self::_puroro::tags::Int64, 13>,
    // Optional, Variant(UInt32)
    u32_required: self::_puroro::internal::field_types::OptionalVariantField<u32, self::_puroro::tags::UInt32, 14>,
    // Optional, Variant(UInt32)
    u32_optional: self::_puroro::internal::field_types::OptionalVariantField<u32, self::_puroro::tags::UInt32, 15>,
    // Optional, Variant(UInt64)
    u64_required: self::_puroro::internal::field_types::OptionalVariantField<u64, self::_puroro::tags::UInt64, 16>,
    // Optional, Variant(UInt64)
    u64_optional: self::_puroro::internal::field_types::OptionalVariantField<u64, self::_puroro::tags::UInt64, 17>,
    // Optional, Variant(SInt32)
    s32_required: self::_puroro::internal::field_types::OptionalVariantField<i32, self::_puroro::tags::SInt32, 18>,
    // Optional, Variant(SInt32)
    s32_optional: self::_puroro::internal::field_types::OptionalVariantField<i32, self::_puroro::tags::SInt32, 19>,
    // Optional, Variant(SInt64)
    s64_required: self::_puroro::internal::field_types::OptionalVariantField<i64, self::_puroro::tags::SInt64, 20>,
    // Optional, Variant(SInt64)
    s64_optional: self::_puroro::internal::field_types::OptionalVariantField<i64, self::_puroro::tags::SInt64, 21>,
    // Optional, Bits32(Fixed32)
    fixed32_required: self::_puroro::internal::field_types::,
    // Optional, Bits32(Fixed32)
    fixed32_optional: self::_puroro::internal::field_types::,
    // Optional, Bits64(Fixed64)
    fixed64_required: self::_puroro::internal::field_types::,
    // Optional, Bits64(Fixed64)
    fixed64_optional: self::_puroro::internal::field_types::,
    // Optional, Bits32(SFixed32)
    sfixed32_required: self::_puroro::internal::field_types::,
    // Optional, Bits32(SFixed32)
    sfixed32_optional: self::_puroro::internal::field_types::,
    // Optional, Bits64(SFixed64)
    sfixed64_required: self::_puroro::internal::field_types::,
    // Optional, Bits64(SFixed64)
    sfixed64_optional: self::_puroro::internal::field_types::,
    // Optional, Bits64(Double)
    f64_required: self::_puroro::internal::field_types::,
    // Optional, Bits64(Double)
    f64_optional: self::_puroro::internal::field_types::,

    _bitfield: self::_puroro::bitvec::BitArray<4>,
}

impl Msg {
    // Optional, Variant(Int32)
    pub fn i32_required(&self) -> i32 {
        <self::_puroro::internal::field_types::OptionalVariantField<i32, self::_puroro::tags::Int32, 0> as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.i32_required, &self._bitfield,
        )
    }
    // Optional, Variant(Int32)
    pub fn i32_optional(&self) -> i32 {
        <self::_puroro::internal::field_types::OptionalVariantField<i32, self::_puroro::tags::Int32, 1> as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.i32_optional, &self._bitfield,
        )
    }
    // Optional, Bits32(Float)
    pub fn float_required(&self) -> f32 {
        <self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.float_required, &self._bitfield,
        )
    }
    // Optional, Bits32(Float)
    pub fn float_optional(&self) -> f32 {
        <self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.float_optional, &self._bitfield,
        )
    }
    // Optional, LengthDelimited(Bytes)
    pub fn bytes_required(&self) -> &[u8] {
        <self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.bytes_required, &self._bitfield,
        )
    }
    // Optional, LengthDelimited(Bytes)
    pub fn bytes_optional(&self) -> &[u8] {
        <self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.bytes_optional, &self._bitfield,
        )
    }
    // Optional, LengthDelimited(String)
    pub fn string_required(&self) -> &str {
        <self::_puroro::internal::field_types::OptionalStringField<6> as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.string_required, &self._bitfield,
        )
    }
    // Optional, LengthDelimited(String)
    pub fn string_optional(&self) -> &str {
        <self::_puroro::internal::field_types::OptionalStringField<7> as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.string_optional, &self._bitfield,
        )
    }
    // Optional, Variant(Enum2(Fqtn(".full_coverage2.Enum")))
    pub fn enum_required(&self) -> _puroro_root::full_coverage2::Enum {
        <self::_puroro::internal::field_types::OptionalVariantField<_puroro_root::full_coverage2::Enum, self::_puroro::tags::Enum2<_puroro_root::full_coverage2::Enum>, 8> as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.enum_required, &self._bitfield,
        )
    }
    // Optional, Variant(Enum2(Fqtn(".full_coverage2.Enum")))
    pub fn enum_optional(&self) -> _puroro_root::full_coverage2::Enum {
        <self::_puroro::internal::field_types::OptionalVariantField<_puroro_root::full_coverage2::Enum, self::_puroro::tags::Enum2<_puroro_root::full_coverage2::Enum>, 9> as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.enum_optional, &self._bitfield,
        )
    }
    // Optional, LengthDelimited(Message(Fqtn(".full_coverage2.Msg.Submsg")))
    pub fn submsg_required(&self) -> Option<&_puroro_root::full_coverage2::_msg::Submsg> {
        <self::_puroro::internal::field_types::SingularHeapMessageField<_puroro_root::full_coverage2::_msg::Submsg> as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.submsg_required, &self._bitfield,
        )
    }
    // Optional, LengthDelimited(Message(Fqtn(".full_coverage2.Msg.Submsg")))
    pub fn submsg_optional(&self) -> Option<&_puroro_root::full_coverage2::_msg::Submsg> {
        <self::_puroro::internal::field_types::SingularHeapMessageField<_puroro_root::full_coverage2::_msg::Submsg> as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.submsg_optional, &self._bitfield,
        )
    }
    // Optional, Variant(Int64)
    pub fn i64_required(&self) -> i64 {
        <self::_puroro::internal::field_types::OptionalVariantField<i64, self::_puroro::tags::Int64, 12> as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.i64_required, &self._bitfield,
        )
    }
    // Optional, Variant(Int64)
    pub fn i64_optional(&self) -> i64 {
        <self::_puroro::internal::field_types::OptionalVariantField<i64, self::_puroro::tags::Int64, 13> as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.i64_optional, &self._bitfield,
        )
    }
    // Optional, Variant(UInt32)
    pub fn u32_required(&self) -> u32 {
        <self::_puroro::internal::field_types::OptionalVariantField<u32, self::_puroro::tags::UInt32, 14> as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.u32_required, &self._bitfield,
        )
    }
    // Optional, Variant(UInt32)
    pub fn u32_optional(&self) -> u32 {
        <self::_puroro::internal::field_types::OptionalVariantField<u32, self::_puroro::tags::UInt32, 15> as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.u32_optional, &self._bitfield,
        )
    }
    // Optional, Variant(UInt64)
    pub fn u64_required(&self) -> u64 {
        <self::_puroro::internal::field_types::OptionalVariantField<u64, self::_puroro::tags::UInt64, 16> as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.u64_required, &self._bitfield,
        )
    }
    // Optional, Variant(UInt64)
    pub fn u64_optional(&self) -> u64 {
        <self::_puroro::internal::field_types::OptionalVariantField<u64, self::_puroro::tags::UInt64, 17> as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.u64_optional, &self._bitfield,
        )
    }
    // Optional, Variant(SInt32)
    pub fn s32_required(&self) -> i32 {
        <self::_puroro::internal::field_types::OptionalVariantField<i32, self::_puroro::tags::SInt32, 18> as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.s32_required, &self._bitfield,
        )
    }
    // Optional, Variant(SInt32)
    pub fn s32_optional(&self) -> i32 {
        <self::_puroro::internal::field_types::OptionalVariantField<i32, self::_puroro::tags::SInt32, 19> as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.s32_optional, &self._bitfield,
        )
    }
    // Optional, Variant(SInt64)
    pub fn s64_required(&self) -> i64 {
        <self::_puroro::internal::field_types::OptionalVariantField<i64, self::_puroro::tags::SInt64, 20> as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.s64_required, &self._bitfield,
        )
    }
    // Optional, Variant(SInt64)
    pub fn s64_optional(&self) -> i64 {
        <self::_puroro::internal::field_types::OptionalVariantField<i64, self::_puroro::tags::SInt64, 21> as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.s64_optional, &self._bitfield,
        )
    }
    // Optional, Bits32(Fixed32)
    pub fn fixed32_required(&self) -> u32 {
        <self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.fixed32_required, &self._bitfield,
        )
    }
    // Optional, Bits32(Fixed32)
    pub fn fixed32_optional(&self) -> u32 {
        <self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.fixed32_optional, &self._bitfield,
        )
    }
    // Optional, Bits64(Fixed64)
    pub fn fixed64_required(&self) -> u64 {
        <self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.fixed64_required, &self._bitfield,
        )
    }
    // Optional, Bits64(Fixed64)
    pub fn fixed64_optional(&self) -> u64 {
        <self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.fixed64_optional, &self._bitfield,
        )
    }
    // Optional, Bits32(SFixed32)
    pub fn sfixed32_required(&self) -> i32 {
        <self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.sfixed32_required, &self._bitfield,
        )
    }
    // Optional, Bits32(SFixed32)
    pub fn sfixed32_optional(&self) -> i32 {
        <self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.sfixed32_optional, &self._bitfield,
        )
    }
    // Optional, Bits64(SFixed64)
    pub fn sfixed64_required(&self) -> i64 {
        <self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.sfixed64_required, &self._bitfield,
        )
    }
    // Optional, Bits64(SFixed64)
    pub fn sfixed64_optional(&self) -> i64 {
        <self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.sfixed64_optional, &self._bitfield,
        )
    }
    // Optional, Bits64(Double)
    pub fn f64_required(&self) -> f64 {
        <self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.f64_required, &self._bitfield,
        )
    }
    // Optional, Bits64(Double)
    pub fn f64_optional(&self) -> f64 {
        <self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.f64_optional, &self._bitfield,
        )
    }
}

impl self::_puroro::Message for Msg {
    fn from_bytes_iter<I: ::std::iter::Iterator<Item=::std::io::Result<u8>>>(iter: I) -> self::_puroro::Result<Self> {
        let mut msg: Self = ::std::default::Default::default();
        let mut peekable = iter.peekable();
        while peekable.peek().is_some() {
            let (number, field_data) = self::_puroro::internal::ser::FieldData::from_bytes_iter(peekable.by_ref())?;
            match number {
                1 => <
                    self::_puroro::internal::field_types::OptionalVariantField<i32, self::_puroro::tags::Int32, 0> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.i32_required,
                    &mut msg._bitfield,
                    field_data,
                )?,
                2 => <
                    self::_puroro::internal::field_types::OptionalVariantField<i32, self::_puroro::tags::Int32, 1> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.i32_optional,
                    &mut msg._bitfield,
                    field_data,
                )?,
                3 => <
                    self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.i32_repeated,
                    &mut msg._bitfield,
                    field_data,
                )?,
                11 => <
                    self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.float_required,
                    &mut msg._bitfield,
                    field_data,
                )?,
                12 => <
                    self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.float_optional,
                    &mut msg._bitfield,
                    field_data,
                )?,
                13 => <
                    self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.float_repeated,
                    &mut msg._bitfield,
                    field_data,
                )?,
                21 => <
                    self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.bytes_required,
                    &mut msg._bitfield,
                    field_data,
                )?,
                22 => <
                    self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.bytes_optional,
                    &mut msg._bitfield,
                    field_data,
                )?,
                23 => <
                    self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.bytes_repeated,
                    &mut msg._bitfield,
                    field_data,
                )?,
                31 => <
                    self::_puroro::internal::field_types::OptionalStringField<6> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.string_required,
                    &mut msg._bitfield,
                    field_data,
                )?,
                32 => <
                    self::_puroro::internal::field_types::OptionalStringField<7> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.string_optional,
                    &mut msg._bitfield,
                    field_data,
                )?,
                33 => <
                    self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.string_repeated,
                    &mut msg._bitfield,
                    field_data,
                )?,
                41 => <
                    self::_puroro::internal::field_types::OptionalVariantField<_puroro_root::full_coverage2::Enum, self::_puroro::tags::Enum2<_puroro_root::full_coverage2::Enum>, 8> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.enum_required,
                    &mut msg._bitfield,
                    field_data,
                )?,
                42 => <
                    self::_puroro::internal::field_types::OptionalVariantField<_puroro_root::full_coverage2::Enum, self::_puroro::tags::Enum2<_puroro_root::full_coverage2::Enum>, 9> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.enum_optional,
                    &mut msg._bitfield,
                    field_data,
                )?,
                43 => <
                    self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.enum_repeated,
                    &mut msg._bitfield,
                    field_data,
                )?,
                51 => <
                    self::_puroro::internal::field_types::SingularHeapMessageField<_puroro_root::full_coverage2::_msg::Submsg> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.submsg_required,
                    &mut msg._bitfield,
                    field_data,
                )?,
                52 => <
                    self::_puroro::internal::field_types::SingularHeapMessageField<_puroro_root::full_coverage2::_msg::Submsg> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.submsg_optional,
                    &mut msg._bitfield,
                    field_data,
                )?,
                53 => <
                    self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.submsg_repeated,
                    &mut msg._bitfield,
                    field_data,
                )?,
                101 => <
                    self::_puroro::internal::field_types::OptionalVariantField<i64, self::_puroro::tags::Int64, 12> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.i64_required,
                    &mut msg._bitfield,
                    field_data,
                )?,
                102 => <
                    self::_puroro::internal::field_types::OptionalVariantField<i64, self::_puroro::tags::Int64, 13> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.i64_optional,
                    &mut msg._bitfield,
                    field_data,
                )?,
                103 => <
                    self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.i64_repeated,
                    &mut msg._bitfield,
                    field_data,
                )?,
                111 => <
                    self::_puroro::internal::field_types::OptionalVariantField<u32, self::_puroro::tags::UInt32, 14> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.u32_required,
                    &mut msg._bitfield,
                    field_data,
                )?,
                112 => <
                    self::_puroro::internal::field_types::OptionalVariantField<u32, self::_puroro::tags::UInt32, 15> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.u32_optional,
                    &mut msg._bitfield,
                    field_data,
                )?,
                113 => <
                    self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.u32_repeated,
                    &mut msg._bitfield,
                    field_data,
                )?,
                121 => <
                    self::_puroro::internal::field_types::OptionalVariantField<u64, self::_puroro::tags::UInt64, 16> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.u64_required,
                    &mut msg._bitfield,
                    field_data,
                )?,
                122 => <
                    self::_puroro::internal::field_types::OptionalVariantField<u64, self::_puroro::tags::UInt64, 17> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.u64_optional,
                    &mut msg._bitfield,
                    field_data,
                )?,
                123 => <
                    self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.u64_repeated,
                    &mut msg._bitfield,
                    field_data,
                )?,
                131 => <
                    self::_puroro::internal::field_types::OptionalVariantField<i32, self::_puroro::tags::SInt32, 18> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.s32_required,
                    &mut msg._bitfield,
                    field_data,
                )?,
                132 => <
                    self::_puroro::internal::field_types::OptionalVariantField<i32, self::_puroro::tags::SInt32, 19> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.s32_optional,
                    &mut msg._bitfield,
                    field_data,
                )?,
                133 => <
                    self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.s32_repeated,
                    &mut msg._bitfield,
                    field_data,
                )?,
                141 => <
                    self::_puroro::internal::field_types::OptionalVariantField<i64, self::_puroro::tags::SInt64, 20> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.s64_required,
                    &mut msg._bitfield,
                    field_data,
                )?,
                142 => <
                    self::_puroro::internal::field_types::OptionalVariantField<i64, self::_puroro::tags::SInt64, 21> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.s64_optional,
                    &mut msg._bitfield,
                    field_data,
                )?,
                143 => <
                    self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.s64_repeated,
                    &mut msg._bitfield,
                    field_data,
                )?,
                151 => <
                    self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.fixed32_required,
                    &mut msg._bitfield,
                    field_data,
                )?,
                152 => <
                    self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.fixed32_optional,
                    &mut msg._bitfield,
                    field_data,
                )?,
                153 => <
                    self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.fixed32_repeated,
                    &mut msg._bitfield,
                    field_data,
                )?,
                161 => <
                    self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.fixed64_required,
                    &mut msg._bitfield,
                    field_data,
                )?,
                162 => <
                    self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.fixed64_optional,
                    &mut msg._bitfield,
                    field_data,
                )?,
                163 => <
                    self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.fixed64_repeated,
                    &mut msg._bitfield,
                    field_data,
                )?,
                171 => <
                    self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.sfixed32_required,
                    &mut msg._bitfield,
                    field_data,
                )?,
                172 => <
                    self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.sfixed32_optional,
                    &mut msg._bitfield,
                    field_data,
                )?,
                173 => <
                    self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.sfixed32_repeated,
                    &mut msg._bitfield,
                    field_data,
                )?,
                181 => <
                    self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.sfixed64_required,
                    &mut msg._bitfield,
                    field_data,
                )?,
                182 => <
                    self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.sfixed64_optional,
                    &mut msg._bitfield,
                    field_data,
                )?,
                183 => <
                    self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.sfixed64_repeated,
                    &mut msg._bitfield,
                    field_data,
                )?,
                191 => <
                    self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.f64_required,
                    &mut msg._bitfield,
                    field_data,
                )?,
                192 => <
                    self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.f64_optional,
                    &mut msg._bitfield,
                    field_data,
                )?,
                193 => <
                    self::_puroro::internal::field_types:: as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.f64_repeated,
                    &mut msg._bitfield,
                    field_data,
                )?,
                _ => todo!(),
            }
        }
        Ok(msg)
    }
}

pub mod _msg {

    mod _puroro {
        pub use super::super::_puroro::*;
    }
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }

}
