// A generated source code by puroro library
// package full_coverage3
pub mod _msg;

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub mod _puroro {
    pub use ::puroro::*;
}

#[derive(Default, Clone)]
pub struct Msg {
    // Singular, Variant(Int32)
    i32_unlabeled:
        self::_puroro::internal::field_types::SingularVariantField<i32, self::_puroro::tags::Int32>,
    // Optional, Variant(Int32)
    i32_optional: self::_puroro::internal::field_types::OptionalVariantField<
        i32,
        self::_puroro::tags::Int32,
        0,
    >,
    // Singular, Bits32(Float)
    float_unlabeled: self::_puroro::internal::field_types::Dummy,
    // Optional, Bits32(Float)
    float_optional: self::_puroro::internal::field_types::Dummy,
    // Singular, LengthDelimited(Bytes)
    bytes_unlabeled: self::_puroro::internal::field_types::Dummy,
    // Optional, LengthDelimited(Bytes)
    bytes_optional: self::_puroro::internal::field_types::Dummy,
    // Singular, LengthDelimited(String)
    string_unlabeled: self::_puroro::internal::field_types::SingularStringField,
    // Optional, LengthDelimited(String)
    string_optional: self::_puroro::internal::field_types::OptionalStringField<3>,
    // Singular, Variant(Enum3(Fqtn(".full_coverage3.Enum")))
    enum_unlabeled: self::_puroro::internal::field_types::SingularVariantField<
        _puroro_root::full_coverage3::Enum,
        self::_puroro::tags::Enum3<_puroro_root::full_coverage3::Enum>,
    >,
    // Optional, Variant(Enum3(Fqtn(".full_coverage3.Enum")))
    enum_optional: self::_puroro::internal::field_types::OptionalVariantField<
        _puroro_root::full_coverage3::Enum,
        self::_puroro::tags::Enum3<_puroro_root::full_coverage3::Enum>,
        4,
    >,
    // Singular, LengthDelimited(Message(Fqtn(".full_coverage3.Msg.Submsg")))
    submsg_unlabeled: self::_puroro::internal::field_types::SingularHeapMessageField<
        _puroro_root::full_coverage3::_msg::Submsg,
    >,
    // Optional, LengthDelimited(Message(Fqtn(".full_coverage3.Msg.Submsg")))
    submsg_optional: self::_puroro::internal::field_types::SingularHeapMessageField<
        _puroro_root::full_coverage3::_msg::Submsg,
    >,
    // Singular, Variant(Int64)
    i64_unlabeled:
        self::_puroro::internal::field_types::SingularVariantField<i64, self::_puroro::tags::Int64>,
    // Optional, Variant(Int64)
    i64_optional: self::_puroro::internal::field_types::OptionalVariantField<
        i64,
        self::_puroro::tags::Int64,
        6,
    >,
    // Singular, Variant(UInt32)
    u32_unlabeled: self::_puroro::internal::field_types::SingularVariantField<
        u32,
        self::_puroro::tags::UInt32,
    >,
    // Optional, Variant(UInt32)
    u32_optional: self::_puroro::internal::field_types::OptionalVariantField<
        u32,
        self::_puroro::tags::UInt32,
        7,
    >,
    // Singular, Variant(UInt64)
    u64_unlabeled: self::_puroro::internal::field_types::SingularVariantField<
        u64,
        self::_puroro::tags::UInt64,
    >,
    // Optional, Variant(UInt64)
    u64_optional: self::_puroro::internal::field_types::OptionalVariantField<
        u64,
        self::_puroro::tags::UInt64,
        8,
    >,
    // Singular, Variant(SInt32)
    s32_unlabeled: self::_puroro::internal::field_types::SingularVariantField<
        i32,
        self::_puroro::tags::SInt32,
    >,
    // Optional, Variant(SInt32)
    s32_optional: self::_puroro::internal::field_types::OptionalVariantField<
        i32,
        self::_puroro::tags::SInt32,
        9,
    >,
    // Singular, Variant(SInt64)
    s64_unlabeled: self::_puroro::internal::field_types::SingularVariantField<
        i64,
        self::_puroro::tags::SInt64,
    >,
    // Optional, Variant(SInt64)
    s64_optional: self::_puroro::internal::field_types::OptionalVariantField<
        i64,
        self::_puroro::tags::SInt64,
        10,
    >,
    // Singular, Bits32(Fixed32)
    fixed32_unlabeled: self::_puroro::internal::field_types::Dummy,
    // Optional, Bits32(Fixed32)
    fixed32_optional: self::_puroro::internal::field_types::Dummy,
    // Singular, Bits64(Fixed64)
    fixed64_unlabeled: self::_puroro::internal::field_types::Dummy,
    // Optional, Bits64(Fixed64)
    fixed64_optional: self::_puroro::internal::field_types::Dummy,
    // Singular, Bits32(SFixed32)
    sfixed32_unlabeled: self::_puroro::internal::field_types::Dummy,
    // Optional, Bits32(SFixed32)
    sfixed32_optional: self::_puroro::internal::field_types::Dummy,
    // Singular, Bits64(SFixed64)
    sfixed64_unlabeled: self::_puroro::internal::field_types::Dummy,
    // Optional, Bits64(SFixed64)
    sfixed64_optional: self::_puroro::internal::field_types::Dummy,
    // Singular, Bits64(Double)
    f64_unlabeled: self::_puroro::internal::field_types::Dummy,
    // Optional, Bits64(Double)
    f64_optional: self::_puroro::internal::field_types::Dummy,

    _bitfield: self::_puroro::bitvec::BitArray<2>,
}

impl Msg {
    // Singular, Variant(Int32)
    pub fn i32_unlabeled(&self) -> i32 {
        <self::_puroro::internal::field_types::SingularVariantField<i32, self::_puroro::tags::Int32> as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.i32_unlabeled, &self._bitfield,
        )
    }
    // Optional, Variant(Int32)
    pub fn i32_optional(&self) -> i32 {
        <self::_puroro::internal::field_types::OptionalVariantField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.i32_optional,
            &self._bitfield,
        )
    }
    // Singular, Bits32(Float)
    pub fn float_unlabeled(&self) -> f32 {
        <self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.float_unlabeled, &self._bitfield,
        )
    }
    // Optional, Bits32(Float)
    pub fn float_optional(&self) -> f32 {
        <self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.float_optional, &self._bitfield,
        )
    }
    // Singular, LengthDelimited(Bytes)
    pub fn bytes_unlabeled(&self) -> &[u8] {
        <self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.bytes_unlabeled, &self._bitfield,
        )
    }
    // Optional, LengthDelimited(Bytes)
    pub fn bytes_optional(&self) -> &[u8] {
        <self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.bytes_optional, &self._bitfield,
        )
    }
    // Singular, LengthDelimited(String)
    pub fn string_unlabeled(&self) -> &str {
        <self::_puroro::internal::field_types::SingularStringField as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.string_unlabeled, &self._bitfield,
        )
    }
    // Optional, LengthDelimited(String)
    pub fn string_optional(&self) -> &str {
        <self::_puroro::internal::field_types::OptionalStringField<3> as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.string_optional, &self._bitfield,
        )
    }
    // Singular, Variant(Enum3(Fqtn(".full_coverage3.Enum")))
    pub fn enum_unlabeled(&self) -> _puroro_root::full_coverage3::Enum {
        <self::_puroro::internal::field_types::SingularVariantField<
            _puroro_root::full_coverage3::Enum,
            self::_puroro::tags::Enum3<_puroro_root::full_coverage3::Enum>,
        > as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.enum_unlabeled,
            &self._bitfield,
        )
    }
    // Optional, Variant(Enum3(Fqtn(".full_coverage3.Enum")))
    pub fn enum_optional(&self) -> _puroro_root::full_coverage3::Enum {
        <self::_puroro::internal::field_types::OptionalVariantField<
            _puroro_root::full_coverage3::Enum,
            self::_puroro::tags::Enum3<_puroro_root::full_coverage3::Enum>,
            4,
        > as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.enum_optional,
            &self._bitfield,
        )
    }
    // Singular, LengthDelimited(Message(Fqtn(".full_coverage3.Msg.Submsg")))
    pub fn submsg_unlabeled(&self) -> Option<&_puroro_root::full_coverage3::_msg::Submsg> {
        <self::_puroro::internal::field_types::SingularHeapMessageField<
            _puroro_root::full_coverage3::_msg::Submsg,
        > as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.submsg_unlabeled,
            &self._bitfield,
        )
    }
    // Optional, LengthDelimited(Message(Fqtn(".full_coverage3.Msg.Submsg")))
    pub fn submsg_optional(&self) -> Option<&_puroro_root::full_coverage3::_msg::Submsg> {
        <self::_puroro::internal::field_types::SingularHeapMessageField<
            _puroro_root::full_coverage3::_msg::Submsg,
        > as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.submsg_optional,
            &self._bitfield,
        )
    }
    // Singular, Variant(Int64)
    pub fn i64_unlabeled(&self) -> i64 {
        <self::_puroro::internal::field_types::SingularVariantField<i64, self::_puroro::tags::Int64> as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.i64_unlabeled, &self._bitfield,
        )
    }
    // Optional, Variant(Int64)
    pub fn i64_optional(&self) -> i64 {
        <self::_puroro::internal::field_types::OptionalVariantField<
            i64,
            self::_puroro::tags::Int64,
            6,
        > as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.i64_optional,
            &self._bitfield,
        )
    }
    // Singular, Variant(UInt32)
    pub fn u32_unlabeled(&self) -> u32 {
        <self::_puroro::internal::field_types::SingularVariantField<u32, self::_puroro::tags::UInt32> as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.u32_unlabeled, &self._bitfield,
        )
    }
    // Optional, Variant(UInt32)
    pub fn u32_optional(&self) -> u32 {
        <self::_puroro::internal::field_types::OptionalVariantField<
            u32,
            self::_puroro::tags::UInt32,
            7,
        > as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.u32_optional,
            &self._bitfield,
        )
    }
    // Singular, Variant(UInt64)
    pub fn u64_unlabeled(&self) -> u64 {
        <self::_puroro::internal::field_types::SingularVariantField<u64, self::_puroro::tags::UInt64> as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.u64_unlabeled, &self._bitfield,
        )
    }
    // Optional, Variant(UInt64)
    pub fn u64_optional(&self) -> u64 {
        <self::_puroro::internal::field_types::OptionalVariantField<
            u64,
            self::_puroro::tags::UInt64,
            8,
        > as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.u64_optional,
            &self._bitfield,
        )
    }
    // Singular, Variant(SInt32)
    pub fn s32_unlabeled(&self) -> i32 {
        <self::_puroro::internal::field_types::SingularVariantField<i32, self::_puroro::tags::SInt32> as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.s32_unlabeled, &self._bitfield,
        )
    }
    // Optional, Variant(SInt32)
    pub fn s32_optional(&self) -> i32 {
        <self::_puroro::internal::field_types::OptionalVariantField<
            i32,
            self::_puroro::tags::SInt32,
            9,
        > as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.s32_optional,
            &self._bitfield,
        )
    }
    // Singular, Variant(SInt64)
    pub fn s64_unlabeled(&self) -> i64 {
        <self::_puroro::internal::field_types::SingularVariantField<i64, self::_puroro::tags::SInt64> as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.s64_unlabeled, &self._bitfield,
        )
    }
    // Optional, Variant(SInt64)
    pub fn s64_optional(&self) -> i64 {
        <self::_puroro::internal::field_types::OptionalVariantField<
            i64,
            self::_puroro::tags::SInt64,
            10,
        > as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.s64_optional,
            &self._bitfield,
        )
    }
    // Singular, Bits32(Fixed32)
    pub fn fixed32_unlabeled(&self) -> u32 {
        <self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.fixed32_unlabeled, &self._bitfield,
        )
    }
    // Optional, Bits32(Fixed32)
    pub fn fixed32_optional(&self) -> u32 {
        <self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.fixed32_optional, &self._bitfield,
        )
    }
    // Singular, Bits64(Fixed64)
    pub fn fixed64_unlabeled(&self) -> u64 {
        <self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.fixed64_unlabeled, &self._bitfield,
        )
    }
    // Optional, Bits64(Fixed64)
    pub fn fixed64_optional(&self) -> u64 {
        <self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.fixed64_optional, &self._bitfield,
        )
    }
    // Singular, Bits32(SFixed32)
    pub fn sfixed32_unlabeled(&self) -> i32 {
        <self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.sfixed32_unlabeled, &self._bitfield,
        )
    }
    // Optional, Bits32(SFixed32)
    pub fn sfixed32_optional(&self) -> i32 {
        <self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.sfixed32_optional, &self._bitfield,
        )
    }
    // Singular, Bits64(SFixed64)
    pub fn sfixed64_unlabeled(&self) -> i64 {
        <self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.sfixed64_unlabeled, &self._bitfield,
        )
    }
    // Optional, Bits64(SFixed64)
    pub fn sfixed64_optional(&self) -> i64 {
        <self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.sfixed64_optional, &self._bitfield,
        )
    }
    // Singular, Bits64(Double)
    pub fn f64_unlabeled(&self) -> f64 {
        <self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.f64_unlabeled, &self._bitfield,
        )
    }
    // Optional, Bits64(Double)
    pub fn f64_optional(&self) -> f64 {
        <self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType>::get_field(
            &self.f64_optional, &self._bitfield,
        )
    }
}

impl self::_puroro::Message for Msg {
    fn from_bytes_iter<I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>>(
        iter: I,
    ) -> self::_puroro::Result<Self> {
        let mut msg: Self = ::std::default::Default::default();
        let mut peekable = iter.peekable();
        while peekable.peek().is_some() {
            let (number, field_data) =
                self::_puroro::internal::ser::FieldData::from_bytes_iter(peekable.by_ref())?;
            match number {
                1 => <
                    self::_puroro::internal::field_types::SingularVariantField<i32, self::_puroro::tags::Int32> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.i32_unlabeled,
                    &mut msg._bitfield,
                    field_data,
                )?,
                2 => <
                    self::_puroro::internal::field_types::OptionalVariantField<i32, self::_puroro::tags::Int32, 0> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.i32_optional,
                    &mut msg._bitfield,
                    field_data,
                )?,
                3 => <
                    self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.i32_repeated,
                    &mut msg._bitfield,
                    field_data,
                )?,
                11 => <
                    self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.float_unlabeled,
                    &mut msg._bitfield,
                    field_data,
                )?,
                12 => <
                    self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.float_optional,
                    &mut msg._bitfield,
                    field_data,
                )?,
                13 => <
                    self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.float_repeated,
                    &mut msg._bitfield,
                    field_data,
                )?,
                21 => <
                    self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.bytes_unlabeled,
                    &mut msg._bitfield,
                    field_data,
                )?,
                22 => <
                    self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.bytes_optional,
                    &mut msg._bitfield,
                    field_data,
                )?,
                23 => <
                    self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.bytes_repeated,
                    &mut msg._bitfield,
                    field_data,
                )?,
                31 => <
                    self::_puroro::internal::field_types::SingularStringField as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.string_unlabeled,
                    &mut msg._bitfield,
                    field_data,
                )?,
                32 => <
                    self::_puroro::internal::field_types::OptionalStringField<3> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.string_optional,
                    &mut msg._bitfield,
                    field_data,
                )?,
                33 => <
                    self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.string_repeated,
                    &mut msg._bitfield,
                    field_data,
                )?,
                41 => <
                    self::_puroro::internal::field_types::SingularVariantField<_puroro_root::full_coverage3::Enum, self::_puroro::tags::Enum3<_puroro_root::full_coverage3::Enum>> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.enum_unlabeled,
                    &mut msg._bitfield,
                    field_data,
                )?,
                42 => <
                    self::_puroro::internal::field_types::OptionalVariantField<_puroro_root::full_coverage3::Enum, self::_puroro::tags::Enum3<_puroro_root::full_coverage3::Enum>, 4> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.enum_optional,
                    &mut msg._bitfield,
                    field_data,
                )?,
                43 => <
                    self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.enum_repeated,
                    &mut msg._bitfield,
                    field_data,
                )?,
                51 => <
                    self::_puroro::internal::field_types::SingularHeapMessageField<_puroro_root::full_coverage3::_msg::Submsg> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.submsg_unlabeled,
                    &mut msg._bitfield,
                    field_data,
                )?,
                52 => <
                    self::_puroro::internal::field_types::SingularHeapMessageField<_puroro_root::full_coverage3::_msg::Submsg> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.submsg_optional,
                    &mut msg._bitfield,
                    field_data,
                )?,
                53 => <
                    self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.submsg_repeated,
                    &mut msg._bitfield,
                    field_data,
                )?,
                101 => <
                    self::_puroro::internal::field_types::SingularVariantField<i64, self::_puroro::tags::Int64> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.i64_unlabeled,
                    &mut msg._bitfield,
                    field_data,
                )?,
                102 => <
                    self::_puroro::internal::field_types::OptionalVariantField<i64, self::_puroro::tags::Int64, 6> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.i64_optional,
                    &mut msg._bitfield,
                    field_data,
                )?,
                103 => <
                    self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.i64_repeated,
                    &mut msg._bitfield,
                    field_data,
                )?,
                111 => <
                    self::_puroro::internal::field_types::SingularVariantField<u32, self::_puroro::tags::UInt32> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.u32_unlabeled,
                    &mut msg._bitfield,
                    field_data,
                )?,
                112 => <
                    self::_puroro::internal::field_types::OptionalVariantField<u32, self::_puroro::tags::UInt32, 7> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.u32_optional,
                    &mut msg._bitfield,
                    field_data,
                )?,
                113 => <
                    self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.u32_repeated,
                    &mut msg._bitfield,
                    field_data,
                )?,
                121 => <
                    self::_puroro::internal::field_types::SingularVariantField<u64, self::_puroro::tags::UInt64> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.u64_unlabeled,
                    &mut msg._bitfield,
                    field_data,
                )?,
                122 => <
                    self::_puroro::internal::field_types::OptionalVariantField<u64, self::_puroro::tags::UInt64, 8> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.u64_optional,
                    &mut msg._bitfield,
                    field_data,
                )?,
                123 => <
                    self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.u64_repeated,
                    &mut msg._bitfield,
                    field_data,
                )?,
                131 => <
                    self::_puroro::internal::field_types::SingularVariantField<i32, self::_puroro::tags::SInt32> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.s32_unlabeled,
                    &mut msg._bitfield,
                    field_data,
                )?,
                132 => <
                    self::_puroro::internal::field_types::OptionalVariantField<i32, self::_puroro::tags::SInt32, 9> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.s32_optional,
                    &mut msg._bitfield,
                    field_data,
                )?,
                133 => <
                    self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.s32_repeated,
                    &mut msg._bitfield,
                    field_data,
                )?,
                141 => <
                    self::_puroro::internal::field_types::SingularVariantField<i64, self::_puroro::tags::SInt64> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.s64_unlabeled,
                    &mut msg._bitfield,
                    field_data,
                )?,
                142 => <
                    self::_puroro::internal::field_types::OptionalVariantField<i64, self::_puroro::tags::SInt64, 10> as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.s64_optional,
                    &mut msg._bitfield,
                    field_data,
                )?,
                143 => <
                    self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.s64_repeated,
                    &mut msg._bitfield,
                    field_data,
                )?,
                151 => <
                    self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.fixed32_unlabeled,
                    &mut msg._bitfield,
                    field_data,
                )?,
                152 => <
                    self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.fixed32_optional,
                    &mut msg._bitfield,
                    field_data,
                )?,
                153 => <
                    self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.fixed32_repeated,
                    &mut msg._bitfield,
                    field_data,
                )?,
                161 => <
                    self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.fixed64_unlabeled,
                    &mut msg._bitfield,
                    field_data,
                )?,
                162 => <
                    self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.fixed64_optional,
                    &mut msg._bitfield,
                    field_data,
                )?,
                163 => <
                    self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.fixed64_repeated,
                    &mut msg._bitfield,
                    field_data,
                )?,
                171 => <
                    self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.sfixed32_unlabeled,
                    &mut msg._bitfield,
                    field_data,
                )?,
                172 => <
                    self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.sfixed32_optional,
                    &mut msg._bitfield,
                    field_data,
                )?,
                173 => <
                    self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.sfixed32_repeated,
                    &mut msg._bitfield,
                    field_data,
                )?,
                181 => <
                    self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.sfixed64_unlabeled,
                    &mut msg._bitfield,
                    field_data,
                )?,
                182 => <
                    self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.sfixed64_optional,
                    &mut msg._bitfield,
                    field_data,
                )?,
                183 => <
                    self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.sfixed64_repeated,
                    &mut msg._bitfield,
                    field_data,
                )?,
                191 => <
                    self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.f64_unlabeled,
                    &mut msg._bitfield,
                    field_data,
                )?,
                192 => <
                    self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType
                >::deser_from_iter(
                    &mut msg.f64_optional,
                    &mut msg._bitfield,
                    field_data,
                )?,
                193 => <
                    self::_puroro::internal::field_types::Dummy as self::_puroro::internal::field_types::FieldType
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
