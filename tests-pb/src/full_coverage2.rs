// A generated source code by puroro library
// package full_coverage2

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub mod _puroro {
    pub use ::puroro::*;
}

#[derive(Default)]
pub struct Msg {
    // Optional, Variant(Int32)
    i32_required: self::_puroro::internal::field_type::OptionalNumericalField<
        i32,
        self::_puroro::tags::Int32,
        0,
    >,
    // Optional, Variant(Int32)
    i32_optional: self::_puroro::internal::field_type::OptionalNumericalField<
        i32,
        self::_puroro::tags::Int32,
        1,
    >,
    // Repeated, Variant(Int32)
    i32_repeated: self::_puroro::internal::field_type::RepeatedNumericalField<
        i32,
        self::_puroro::tags::Int32,
    >,
    // Optional, Bits32(Float)
    float_required: self::_puroro::internal::field_type::OptionalNumericalField<
        f32,
        self::_puroro::tags::Float,
        2,
    >,
    // Optional, Bits32(Float)
    float_optional: self::_puroro::internal::field_type::OptionalNumericalField<
        f32,
        self::_puroro::tags::Float,
        3,
    >,
    // Repeated, Bits32(Float)
    float_repeated: self::_puroro::internal::field_type::RepeatedNumericalField<
        f32,
        self::_puroro::tags::Float,
    >,
    // Optional, LengthDelimited(Bytes)
    bytes_required: self::_puroro::internal::field_type::OptionalBytesField<4>,
    // Optional, LengthDelimited(Bytes)
    bytes_optional: self::_puroro::internal::field_type::OptionalBytesField<5>,
    // Repeated, LengthDelimited(Bytes)
    bytes_repeated: self::_puroro::internal::field_type::RepeatedBytesField,
    // Optional, LengthDelimited(String)
    string_required: self::_puroro::internal::field_type::OptionalStringField<6>,
    // Optional, LengthDelimited(String)
    string_optional: self::_puroro::internal::field_type::OptionalStringField<7>,
    // Repeated, LengthDelimited(String)
    string_repeated: self::_puroro::internal::field_type::RepeatedStringField,
    // Optional, Variant(Enum2(Fqtn(".full_coverage2.Enum")))
    enum_required: self::_puroro::internal::field_type::OptionalNumericalField<
        _puroro_root::full_coverage2::Enum,
        self::_puroro::tags::Enum2<_puroro_root::full_coverage2::Enum>,
        8,
    >,
    // Optional, Variant(Enum2(Fqtn(".full_coverage2.Enum")))
    enum_optional: self::_puroro::internal::field_type::OptionalNumericalField<
        _puroro_root::full_coverage2::Enum,
        self::_puroro::tags::Enum2<_puroro_root::full_coverage2::Enum>,
        9,
    >,
    // Repeated, Variant(Enum2(Fqtn(".full_coverage2.Enum")))
    enum_repeated: self::_puroro::internal::field_type::RepeatedNumericalField<
        _puroro_root::full_coverage2::Enum,
        self::_puroro::tags::Enum2<_puroro_root::full_coverage2::Enum>,
    >,
    // Optional, LengthDelimited(Message(Fqtn(".full_coverage2.Msg.Submsg")))
    submsg_required: self::_puroro::internal::field_type::SingularHeapMessageField<
        _puroro_root::full_coverage2::msg::Submsg,
    >,
    // Optional, LengthDelimited(Message(Fqtn(".full_coverage2.Msg.Submsg")))
    submsg_optional: self::_puroro::internal::field_type::SingularHeapMessageField<
        _puroro_root::full_coverage2::msg::Submsg,
    >,
    // Repeated, LengthDelimited(Message(Fqtn(".full_coverage2.Msg.Submsg")))
    submsg_repeated: self::_puroro::internal::field_type::RepeatedMessageField<
        _puroro_root::full_coverage2::msg::Submsg,
    >,
    // Optional, Variant(Int64)
    i64_required: self::_puroro::internal::field_type::OptionalNumericalField<
        i64,
        self::_puroro::tags::Int64,
        12,
    >,
    // Optional, Variant(Int64)
    i64_optional: self::_puroro::internal::field_type::OptionalNumericalField<
        i64,
        self::_puroro::tags::Int64,
        13,
    >,
    // Repeated, Variant(Int64)
    i64_repeated: self::_puroro::internal::field_type::RepeatedNumericalField<
        i64,
        self::_puroro::tags::Int64,
    >,
    // Optional, Variant(UInt32)
    u32_required: self::_puroro::internal::field_type::OptionalNumericalField<
        u32,
        self::_puroro::tags::UInt32,
        14,
    >,
    // Optional, Variant(UInt32)
    u32_optional: self::_puroro::internal::field_type::OptionalNumericalField<
        u32,
        self::_puroro::tags::UInt32,
        15,
    >,
    // Repeated, Variant(UInt32)
    u32_repeated: self::_puroro::internal::field_type::RepeatedNumericalField<
        u32,
        self::_puroro::tags::UInt32,
    >,
    // Optional, Variant(UInt64)
    u64_required: self::_puroro::internal::field_type::OptionalNumericalField<
        u64,
        self::_puroro::tags::UInt64,
        16,
    >,
    // Optional, Variant(UInt64)
    u64_optional: self::_puroro::internal::field_type::OptionalNumericalField<
        u64,
        self::_puroro::tags::UInt64,
        17,
    >,
    // Repeated, Variant(UInt64)
    u64_repeated: self::_puroro::internal::field_type::RepeatedNumericalField<
        u64,
        self::_puroro::tags::UInt64,
    >,
    // Optional, Variant(SInt32)
    s32_required: self::_puroro::internal::field_type::OptionalNumericalField<
        i32,
        self::_puroro::tags::SInt32,
        18,
    >,
    // Optional, Variant(SInt32)
    s32_optional: self::_puroro::internal::field_type::OptionalNumericalField<
        i32,
        self::_puroro::tags::SInt32,
        19,
    >,
    // Repeated, Variant(SInt32)
    s32_repeated: self::_puroro::internal::field_type::RepeatedNumericalField<
        i32,
        self::_puroro::tags::SInt32,
    >,
    // Optional, Variant(SInt64)
    s64_required: self::_puroro::internal::field_type::OptionalNumericalField<
        i64,
        self::_puroro::tags::SInt64,
        20,
    >,
    // Optional, Variant(SInt64)
    s64_optional: self::_puroro::internal::field_type::OptionalNumericalField<
        i64,
        self::_puroro::tags::SInt64,
        21,
    >,
    // Repeated, Variant(SInt64)
    s64_repeated: self::_puroro::internal::field_type::RepeatedNumericalField<
        i64,
        self::_puroro::tags::SInt64,
    >,
    // Optional, Bits32(Fixed32)
    fixed32_required: self::_puroro::internal::field_type::OptionalNumericalField<
        u32,
        self::_puroro::tags::Fixed32,
        22,
    >,
    // Optional, Bits32(Fixed32)
    fixed32_optional: self::_puroro::internal::field_type::OptionalNumericalField<
        u32,
        self::_puroro::tags::Fixed32,
        23,
    >,
    // Repeated, Bits32(Fixed32)
    fixed32_repeated: self::_puroro::internal::field_type::RepeatedNumericalField<
        u32,
        self::_puroro::tags::Fixed32,
    >,
    // Optional, Bits64(Fixed64)
    fixed64_required: self::_puroro::internal::field_type::OptionalNumericalField<
        u64,
        self::_puroro::tags::Fixed64,
        24,
    >,
    // Optional, Bits64(Fixed64)
    fixed64_optional: self::_puroro::internal::field_type::OptionalNumericalField<
        u64,
        self::_puroro::tags::Fixed64,
        25,
    >,
    // Repeated, Bits64(Fixed64)
    fixed64_repeated: self::_puroro::internal::field_type::RepeatedNumericalField<
        u64,
        self::_puroro::tags::Fixed64,
    >,
    // Optional, Bits32(SFixed32)
    sfixed32_required: self::_puroro::internal::field_type::OptionalNumericalField<
        i32,
        self::_puroro::tags::SFixed32,
        26,
    >,
    // Optional, Bits32(SFixed32)
    sfixed32_optional: self::_puroro::internal::field_type::OptionalNumericalField<
        i32,
        self::_puroro::tags::SFixed32,
        27,
    >,
    // Repeated, Bits32(SFixed32)
    sfixed32_repeated: self::_puroro::internal::field_type::RepeatedNumericalField<
        i32,
        self::_puroro::tags::SFixed32,
    >,
    // Optional, Bits64(SFixed64)
    sfixed64_required: self::_puroro::internal::field_type::OptionalNumericalField<
        i64,
        self::_puroro::tags::SFixed64,
        28,
    >,
    // Optional, Bits64(SFixed64)
    sfixed64_optional: self::_puroro::internal::field_type::OptionalNumericalField<
        i64,
        self::_puroro::tags::SFixed64,
        29,
    >,
    // Repeated, Bits64(SFixed64)
    sfixed64_repeated: self::_puroro::internal::field_type::RepeatedNumericalField<
        i64,
        self::_puroro::tags::SFixed64,
    >,
    // Optional, Bits64(Double)
    f64_required: self::_puroro::internal::field_type::OptionalNumericalField<
        f64,
        self::_puroro::tags::Double,
        30,
    >,
    // Optional, Bits64(Double)
    f64_optional: self::_puroro::internal::field_type::OptionalNumericalField<
        f64,
        self::_puroro::tags::Double,
        31,
    >,
    // Repeated, Bits64(Double)
    f64_repeated: self::_puroro::internal::field_type::RepeatedNumericalField<
        f64,
        self::_puroro::tags::Double,
    >,

    _bitfield: self::_puroro::bitvec::BitArray<4>,
}

impl Msg {
    // Optional, Variant(Int32)
    pub fn i32_required(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as NonRepeatedFieldType>::get_field(
            &self.i32_required,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn i32_required_opt(&self) -> ::std::option::Option<i32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as NonRepeatedFieldType>::get_field_opt(&self.i32_required, &self._bitfield)
    }
    pub fn has_i32_required(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as NonRepeatedFieldType>::get_field_opt(&self.i32_required, &self._bitfield)
        .is_some()
    }
    pub fn i32_required_mut(&mut self) -> &mut i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.i32_required,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_i32_required(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as NonRepeatedFieldType>::clear(&mut self.i32_required, &mut self._bitfield)
    }
    // Optional, Variant(Int32)
    pub fn i32_optional(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            1,
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
            1,
        > as NonRepeatedFieldType>::get_field_opt(&self.i32_optional, &self._bitfield)
    }
    pub fn has_i32_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            1,
        > as NonRepeatedFieldType>::get_field_opt(&self.i32_optional, &self._bitfield)
        .is_some()
    }
    pub fn i32_optional_mut(&mut self) -> &mut i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            1,
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
            1,
        > as NonRepeatedFieldType>::clear(&mut self.i32_optional, &mut self._bitfield)
    }
    // Repeated, Variant(Int32)
    pub fn i32_repeated(&self) -> &[i32] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<i32, self::_puroro::tags::Int32> as RepeatedFieldType>::get_field(
            &self.i32_repeated, &self._bitfield, 
        )
    }
    pub fn i32_repeated_mut(&mut self) -> &mut ::std::vec::Vec<i32> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<i32, self::_puroro::tags::Int32> as RepeatedFieldType>::mut_field(
            &mut self.i32_repeated, &mut self._bitfield, 
        )
    }
    pub fn clear_i32_repeated(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<i32, self::_puroro::tags::Int32> as RepeatedFieldType>::clear(
            &mut self.i32_repeated, &mut self._bitfield, 
        )
    }
    // Optional, Bits32(Float)
    pub fn float_required(&self) -> f32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            f32,
            self::_puroro::tags::Float,
            2,
        > as NonRepeatedFieldType>::get_field(
            &self.float_required,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn float_required_opt(&self) -> ::std::option::Option<f32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            f32,
            self::_puroro::tags::Float,
            2,
        > as NonRepeatedFieldType>::get_field_opt(&self.float_required, &self._bitfield)
    }
    pub fn has_float_required(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            f32,
            self::_puroro::tags::Float,
            2,
        > as NonRepeatedFieldType>::get_field_opt(&self.float_required, &self._bitfield)
        .is_some()
    }
    pub fn float_required_mut(&mut self) -> &mut f32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            f32,
            self::_puroro::tags::Float,
            2,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.float_required,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_float_required(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            f32,
            self::_puroro::tags::Float,
            2,
        > as NonRepeatedFieldType>::clear(&mut self.float_required, &mut self._bitfield)
    }
    // Optional, Bits32(Float)
    pub fn float_optional(&self) -> f32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            f32,
            self::_puroro::tags::Float,
            3,
        > as NonRepeatedFieldType>::get_field(
            &self.float_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn float_optional_opt(&self) -> ::std::option::Option<f32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            f32,
            self::_puroro::tags::Float,
            3,
        > as NonRepeatedFieldType>::get_field_opt(&self.float_optional, &self._bitfield)
    }
    pub fn has_float_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            f32,
            self::_puroro::tags::Float,
            3,
        > as NonRepeatedFieldType>::get_field_opt(&self.float_optional, &self._bitfield)
        .is_some()
    }
    pub fn float_optional_mut(&mut self) -> &mut f32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            f32,
            self::_puroro::tags::Float,
            3,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.float_optional,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_float_optional(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            f32,
            self::_puroro::tags::Float,
            3,
        > as NonRepeatedFieldType>::clear(&mut self.float_optional, &mut self._bitfield)
    }
    // Repeated, Bits32(Float)
    pub fn float_repeated(&self) -> &[f32] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<f32, self::_puroro::tags::Float> as RepeatedFieldType>::get_field(
            &self.float_repeated, &self._bitfield, 
        )
    }
    pub fn float_repeated_mut(&mut self) -> &mut ::std::vec::Vec<f32> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<f32, self::_puroro::tags::Float> as RepeatedFieldType>::mut_field(
            &mut self.float_repeated, &mut self._bitfield, 
        )
    }
    pub fn clear_float_repeated(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<f32, self::_puroro::tags::Float> as RepeatedFieldType>::clear(
            &mut self.float_repeated, &mut self._bitfield, 
        )
    }
    // Optional, LengthDelimited(Bytes)
    pub fn bytes_required(&self) -> &[u8] {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalBytesField<4> as NonRepeatedFieldType>::get_field(
            &self.bytes_required, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn bytes_required_opt(&self) -> ::std::option::Option<&[u8]> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalBytesField<4> as NonRepeatedFieldType>::get_field_opt(
            &self.bytes_required, &self._bitfield,
        )
    }
    pub fn has_bytes_required(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalBytesField<4> as NonRepeatedFieldType>::get_field_opt(
            &self.bytes_required, &self._bitfield,
        ).is_some()
    }
    pub fn bytes_required_mut(&mut self) -> &mut ::std::vec::Vec<u8> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalBytesField<4> as NonRepeatedFieldType>::mut_field(
            &mut self.bytes_required, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_bytes_required(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalBytesField<4> as NonRepeatedFieldType>::clear(
            &mut self.bytes_required,
            &mut self._bitfield,
        )
    }
    // Optional, LengthDelimited(Bytes)
    pub fn bytes_optional(&self) -> &[u8] {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalBytesField<5> as NonRepeatedFieldType>::get_field(
            &self.bytes_optional, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn bytes_optional_opt(&self) -> ::std::option::Option<&[u8]> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalBytesField<5> as NonRepeatedFieldType>::get_field_opt(
            &self.bytes_optional, &self._bitfield,
        )
    }
    pub fn has_bytes_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalBytesField<5> as NonRepeatedFieldType>::get_field_opt(
            &self.bytes_optional, &self._bitfield,
        ).is_some()
    }
    pub fn bytes_optional_mut(&mut self) -> &mut ::std::vec::Vec<u8> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalBytesField<5> as NonRepeatedFieldType>::mut_field(
            &mut self.bytes_optional, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_bytes_optional(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalBytesField<5> as NonRepeatedFieldType>::clear(
            &mut self.bytes_optional,
            &mut self._bitfield,
        )
    }
    // Repeated, LengthDelimited(Bytes)
    pub fn bytes_repeated(&self) -> &[::std::vec::Vec<u8>] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedBytesField as RepeatedFieldType>::get_field(
            &self.bytes_repeated,
            &self._bitfield,
        )
    }
    pub fn bytes_repeated_mut(&mut self) -> &mut ::std::vec::Vec<::std::vec::Vec<u8>> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedBytesField as RepeatedFieldType>::mut_field(
            &mut self.bytes_repeated,
            &mut self._bitfield,
        )
    }
    pub fn clear_bytes_repeated(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedBytesField as RepeatedFieldType>::clear(
            &mut self.bytes_repeated,
            &mut self._bitfield,
        )
    }
    // Optional, LengthDelimited(String)
    pub fn string_required(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<6> as NonRepeatedFieldType>::get_field(
            &self.string_required, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn string_required_opt(&self) -> ::std::option::Option<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<6> as NonRepeatedFieldType>::get_field_opt(
            &self.string_required, &self._bitfield,
        )
    }
    pub fn has_string_required(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<6> as NonRepeatedFieldType>::get_field_opt(
            &self.string_required, &self._bitfield,
        ).is_some()
    }
    pub fn string_required_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<6> as NonRepeatedFieldType>::mut_field(
            &mut self.string_required, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_string_required(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField<6> as NonRepeatedFieldType>::clear(
            &mut self.string_required,
            &mut self._bitfield,
        )
    }
    // Optional, LengthDelimited(String)
    pub fn string_optional(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<7> as NonRepeatedFieldType>::get_field(
            &self.string_optional, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn string_optional_opt(&self) -> ::std::option::Option<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<7> as NonRepeatedFieldType>::get_field_opt(
            &self.string_optional, &self._bitfield,
        )
    }
    pub fn has_string_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<7> as NonRepeatedFieldType>::get_field_opt(
            &self.string_optional, &self._bitfield,
        ).is_some()
    }
    pub fn string_optional_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<7> as NonRepeatedFieldType>::mut_field(
            &mut self.string_optional, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_string_optional(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField<7> as NonRepeatedFieldType>::clear(
            &mut self.string_optional,
            &mut self._bitfield,
        )
    }
    // Repeated, LengthDelimited(String)
    pub fn string_repeated(&self) -> &[::std::string::String] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedStringField as RepeatedFieldType>::get_field(
            &self.string_repeated,
            &self._bitfield,
        )
    }
    pub fn string_repeated_mut(&mut self) -> &mut ::std::vec::Vec<::std::string::String> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedStringField as RepeatedFieldType>::mut_field(
            &mut self.string_repeated,
            &mut self._bitfield,
        )
    }
    pub fn clear_string_repeated(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedStringField as RepeatedFieldType>::clear(
            &mut self.string_repeated,
            &mut self._bitfield,
        )
    }
    // Optional, Variant(Enum2(Fqtn(".full_coverage2.Enum")))
    pub fn enum_required(&self) -> _puroro_root::full_coverage2::Enum {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::full_coverage2::Enum,
            self::_puroro::tags::Enum2<_puroro_root::full_coverage2::Enum>,
            8,
        > as NonRepeatedFieldType>::get_field(
            &self.enum_required,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn enum_required_opt(&self) -> ::std::option::Option<_puroro_root::full_coverage2::Enum> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::full_coverage2::Enum,
            self::_puroro::tags::Enum2<_puroro_root::full_coverage2::Enum>,
            8,
        > as NonRepeatedFieldType>::get_field_opt(&self.enum_required, &self._bitfield)
    }
    pub fn has_enum_required(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::full_coverage2::Enum,
            self::_puroro::tags::Enum2<_puroro_root::full_coverage2::Enum>,
            8,
        > as NonRepeatedFieldType>::get_field_opt(&self.enum_required, &self._bitfield)
        .is_some()
    }
    pub fn enum_required_mut(&mut self) -> &mut _puroro_root::full_coverage2::Enum {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::full_coverage2::Enum,
            self::_puroro::tags::Enum2<_puroro_root::full_coverage2::Enum>,
            8,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.enum_required,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_enum_required(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::full_coverage2::Enum,
            self::_puroro::tags::Enum2<_puroro_root::full_coverage2::Enum>,
            8,
        > as NonRepeatedFieldType>::clear(&mut self.enum_required, &mut self._bitfield)
    }
    // Optional, Variant(Enum2(Fqtn(".full_coverage2.Enum")))
    pub fn enum_optional(&self) -> _puroro_root::full_coverage2::Enum {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::full_coverage2::Enum,
            self::_puroro::tags::Enum2<_puroro_root::full_coverage2::Enum>,
            9,
        > as NonRepeatedFieldType>::get_field(
            &self.enum_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn enum_optional_opt(&self) -> ::std::option::Option<_puroro_root::full_coverage2::Enum> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::full_coverage2::Enum,
            self::_puroro::tags::Enum2<_puroro_root::full_coverage2::Enum>,
            9,
        > as NonRepeatedFieldType>::get_field_opt(&self.enum_optional, &self._bitfield)
    }
    pub fn has_enum_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::full_coverage2::Enum,
            self::_puroro::tags::Enum2<_puroro_root::full_coverage2::Enum>,
            9,
        > as NonRepeatedFieldType>::get_field_opt(&self.enum_optional, &self._bitfield)
        .is_some()
    }
    pub fn enum_optional_mut(&mut self) -> &mut _puroro_root::full_coverage2::Enum {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::full_coverage2::Enum,
            self::_puroro::tags::Enum2<_puroro_root::full_coverage2::Enum>,
            9,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.enum_optional,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_enum_optional(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::full_coverage2::Enum,
            self::_puroro::tags::Enum2<_puroro_root::full_coverage2::Enum>,
            9,
        > as NonRepeatedFieldType>::clear(&mut self.enum_optional, &mut self._bitfield)
    }
    // Repeated, Variant(Enum2(Fqtn(".full_coverage2.Enum")))
    pub fn enum_repeated(&self) -> &[_puroro_root::full_coverage2::Enum] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            _puroro_root::full_coverage2::Enum,
            self::_puroro::tags::Enum2<_puroro_root::full_coverage2::Enum>,
        > as RepeatedFieldType>::get_field(&self.enum_repeated, &self._bitfield)
    }
    pub fn enum_repeated_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec<_puroro_root::full_coverage2::Enum> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            _puroro_root::full_coverage2::Enum,
            self::_puroro::tags::Enum2<_puroro_root::full_coverage2::Enum>,
        > as RepeatedFieldType>::mut_field(&mut self.enum_repeated, &mut self._bitfield)
    }
    pub fn clear_enum_repeated(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            _puroro_root::full_coverage2::Enum,
            self::_puroro::tags::Enum2<_puroro_root::full_coverage2::Enum>,
        > as RepeatedFieldType>::clear(&mut self.enum_repeated, &mut self._bitfield)
    }
    // Optional, LengthDelimited(Message(Fqtn(".full_coverage2.Msg.Submsg")))
    pub fn submsg_required(
        &self,
    ) -> ::std::option::Option<&_puroro_root::full_coverage2::msg::Submsg> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::full_coverage2::msg::Submsg,
        > as NonRepeatedFieldType>::get_field(
            &self.submsg_required,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn submsg_required_opt(
        &self,
    ) -> ::std::option::Option<&_puroro_root::full_coverage2::msg::Submsg> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::full_coverage2::msg::Submsg,
        > as NonRepeatedFieldType>::get_field_opt(&self.submsg_required, &self._bitfield)
    }
    pub fn has_submsg_required(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::full_coverage2::msg::Submsg,
        > as NonRepeatedFieldType>::get_field_opt(&self.submsg_required, &self._bitfield)
        .is_some()
    }
    pub fn submsg_required_mut(&mut self) -> &mut _puroro_root::full_coverage2::msg::Submsg {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::full_coverage2::msg::Submsg,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.submsg_required,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_submsg_required(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::full_coverage2::msg::Submsg,
        > as NonRepeatedFieldType>::clear(&mut self.submsg_required, &mut self._bitfield)
    }
    // Optional, LengthDelimited(Message(Fqtn(".full_coverage2.Msg.Submsg")))
    pub fn submsg_optional(
        &self,
    ) -> ::std::option::Option<&_puroro_root::full_coverage2::msg::Submsg> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::full_coverage2::msg::Submsg,
        > as NonRepeatedFieldType>::get_field(
            &self.submsg_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn submsg_optional_opt(
        &self,
    ) -> ::std::option::Option<&_puroro_root::full_coverage2::msg::Submsg> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::full_coverage2::msg::Submsg,
        > as NonRepeatedFieldType>::get_field_opt(&self.submsg_optional, &self._bitfield)
    }
    pub fn has_submsg_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::full_coverage2::msg::Submsg,
        > as NonRepeatedFieldType>::get_field_opt(&self.submsg_optional, &self._bitfield)
        .is_some()
    }
    pub fn submsg_optional_mut(&mut self) -> &mut _puroro_root::full_coverage2::msg::Submsg {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::full_coverage2::msg::Submsg,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.submsg_optional,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_submsg_optional(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::full_coverage2::msg::Submsg,
        > as NonRepeatedFieldType>::clear(&mut self.submsg_optional, &mut self._bitfield)
    }
    // Repeated, LengthDelimited(Message(Fqtn(".full_coverage2.Msg.Submsg")))
    pub fn submsg_repeated(&self) -> &[_puroro_root::full_coverage2::msg::Submsg] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::full_coverage2::msg::Submsg,
        > as RepeatedFieldType>::get_field(&self.submsg_repeated, &self._bitfield)
    }
    pub fn submsg_repeated_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec<_puroro_root::full_coverage2::msg::Submsg> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::full_coverage2::msg::Submsg,
        > as RepeatedFieldType>::mut_field(&mut self.submsg_repeated, &mut self._bitfield)
    }
    pub fn clear_submsg_repeated(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::full_coverage2::msg::Submsg,
        > as RepeatedFieldType>::clear(&mut self.submsg_repeated, &mut self._bitfield)
    }
    // Optional, Variant(Int64)
    pub fn i64_required(&self) -> i64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::Int64,
            12,
        > as NonRepeatedFieldType>::get_field(
            &self.i64_required,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn i64_required_opt(&self) -> ::std::option::Option<i64> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::Int64,
            12,
        > as NonRepeatedFieldType>::get_field_opt(&self.i64_required, &self._bitfield)
    }
    pub fn has_i64_required(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::Int64,
            12,
        > as NonRepeatedFieldType>::get_field_opt(&self.i64_required, &self._bitfield)
        .is_some()
    }
    pub fn i64_required_mut(&mut self) -> &mut i64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::Int64,
            12,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.i64_required,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_i64_required(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::Int64,
            12,
        > as NonRepeatedFieldType>::clear(&mut self.i64_required, &mut self._bitfield)
    }
    // Optional, Variant(Int64)
    pub fn i64_optional(&self) -> i64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::Int64,
            13,
        > as NonRepeatedFieldType>::get_field(
            &self.i64_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn i64_optional_opt(&self) -> ::std::option::Option<i64> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::Int64,
            13,
        > as NonRepeatedFieldType>::get_field_opt(&self.i64_optional, &self._bitfield)
    }
    pub fn has_i64_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::Int64,
            13,
        > as NonRepeatedFieldType>::get_field_opt(&self.i64_optional, &self._bitfield)
        .is_some()
    }
    pub fn i64_optional_mut(&mut self) -> &mut i64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::Int64,
            13,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.i64_optional,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_i64_optional(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::Int64,
            13,
        > as NonRepeatedFieldType>::clear(&mut self.i64_optional, &mut self._bitfield)
    }
    // Repeated, Variant(Int64)
    pub fn i64_repeated(&self) -> &[i64] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<i64, self::_puroro::tags::Int64> as RepeatedFieldType>::get_field(
            &self.i64_repeated, &self._bitfield, 
        )
    }
    pub fn i64_repeated_mut(&mut self) -> &mut ::std::vec::Vec<i64> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<i64, self::_puroro::tags::Int64> as RepeatedFieldType>::mut_field(
            &mut self.i64_repeated, &mut self._bitfield, 
        )
    }
    pub fn clear_i64_repeated(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<i64, self::_puroro::tags::Int64> as RepeatedFieldType>::clear(
            &mut self.i64_repeated, &mut self._bitfield, 
        )
    }
    // Optional, Variant(UInt32)
    pub fn u32_required(&self) -> u32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u32,
            self::_puroro::tags::UInt32,
            14,
        > as NonRepeatedFieldType>::get_field(
            &self.u32_required,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn u32_required_opt(&self) -> ::std::option::Option<u32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u32,
            self::_puroro::tags::UInt32,
            14,
        > as NonRepeatedFieldType>::get_field_opt(&self.u32_required, &self._bitfield)
    }
    pub fn has_u32_required(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u32,
            self::_puroro::tags::UInt32,
            14,
        > as NonRepeatedFieldType>::get_field_opt(&self.u32_required, &self._bitfield)
        .is_some()
    }
    pub fn u32_required_mut(&mut self) -> &mut u32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u32,
            self::_puroro::tags::UInt32,
            14,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.u32_required,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_u32_required(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u32,
            self::_puroro::tags::UInt32,
            14,
        > as NonRepeatedFieldType>::clear(&mut self.u32_required, &mut self._bitfield)
    }
    // Optional, Variant(UInt32)
    pub fn u32_optional(&self) -> u32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u32,
            self::_puroro::tags::UInt32,
            15,
        > as NonRepeatedFieldType>::get_field(
            &self.u32_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn u32_optional_opt(&self) -> ::std::option::Option<u32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u32,
            self::_puroro::tags::UInt32,
            15,
        > as NonRepeatedFieldType>::get_field_opt(&self.u32_optional, &self._bitfield)
    }
    pub fn has_u32_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u32,
            self::_puroro::tags::UInt32,
            15,
        > as NonRepeatedFieldType>::get_field_opt(&self.u32_optional, &self._bitfield)
        .is_some()
    }
    pub fn u32_optional_mut(&mut self) -> &mut u32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u32,
            self::_puroro::tags::UInt32,
            15,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.u32_optional,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_u32_optional(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u32,
            self::_puroro::tags::UInt32,
            15,
        > as NonRepeatedFieldType>::clear(&mut self.u32_optional, &mut self._bitfield)
    }
    // Repeated, Variant(UInt32)
    pub fn u32_repeated(&self) -> &[u32] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            u32,
            self::_puroro::tags::UInt32,
        > as RepeatedFieldType>::get_field(&self.u32_repeated, &self._bitfield)
    }
    pub fn u32_repeated_mut(&mut self) -> &mut ::std::vec::Vec<u32> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            u32,
            self::_puroro::tags::UInt32,
        > as RepeatedFieldType>::mut_field(&mut self.u32_repeated, &mut self._bitfield)
    }
    pub fn clear_u32_repeated(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            u32,
            self::_puroro::tags::UInt32,
        > as RepeatedFieldType>::clear(&mut self.u32_repeated, &mut self._bitfield)
    }
    // Optional, Variant(UInt64)
    pub fn u64_required(&self) -> u64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u64,
            self::_puroro::tags::UInt64,
            16,
        > as NonRepeatedFieldType>::get_field(
            &self.u64_required,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn u64_required_opt(&self) -> ::std::option::Option<u64> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u64,
            self::_puroro::tags::UInt64,
            16,
        > as NonRepeatedFieldType>::get_field_opt(&self.u64_required, &self._bitfield)
    }
    pub fn has_u64_required(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u64,
            self::_puroro::tags::UInt64,
            16,
        > as NonRepeatedFieldType>::get_field_opt(&self.u64_required, &self._bitfield)
        .is_some()
    }
    pub fn u64_required_mut(&mut self) -> &mut u64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u64,
            self::_puroro::tags::UInt64,
            16,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.u64_required,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_u64_required(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u64,
            self::_puroro::tags::UInt64,
            16,
        > as NonRepeatedFieldType>::clear(&mut self.u64_required, &mut self._bitfield)
    }
    // Optional, Variant(UInt64)
    pub fn u64_optional(&self) -> u64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u64,
            self::_puroro::tags::UInt64,
            17,
        > as NonRepeatedFieldType>::get_field(
            &self.u64_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn u64_optional_opt(&self) -> ::std::option::Option<u64> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u64,
            self::_puroro::tags::UInt64,
            17,
        > as NonRepeatedFieldType>::get_field_opt(&self.u64_optional, &self._bitfield)
    }
    pub fn has_u64_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u64,
            self::_puroro::tags::UInt64,
            17,
        > as NonRepeatedFieldType>::get_field_opt(&self.u64_optional, &self._bitfield)
        .is_some()
    }
    pub fn u64_optional_mut(&mut self) -> &mut u64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u64,
            self::_puroro::tags::UInt64,
            17,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.u64_optional,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_u64_optional(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u64,
            self::_puroro::tags::UInt64,
            17,
        > as NonRepeatedFieldType>::clear(&mut self.u64_optional, &mut self._bitfield)
    }
    // Repeated, Variant(UInt64)
    pub fn u64_repeated(&self) -> &[u64] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            u64,
            self::_puroro::tags::UInt64,
        > as RepeatedFieldType>::get_field(&self.u64_repeated, &self._bitfield)
    }
    pub fn u64_repeated_mut(&mut self) -> &mut ::std::vec::Vec<u64> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            u64,
            self::_puroro::tags::UInt64,
        > as RepeatedFieldType>::mut_field(&mut self.u64_repeated, &mut self._bitfield)
    }
    pub fn clear_u64_repeated(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            u64,
            self::_puroro::tags::UInt64,
        > as RepeatedFieldType>::clear(&mut self.u64_repeated, &mut self._bitfield)
    }
    // Optional, Variant(SInt32)
    pub fn s32_required(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::SInt32,
            18,
        > as NonRepeatedFieldType>::get_field(
            &self.s32_required,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn s32_required_opt(&self) -> ::std::option::Option<i32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::SInt32,
            18,
        > as NonRepeatedFieldType>::get_field_opt(&self.s32_required, &self._bitfield)
    }
    pub fn has_s32_required(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::SInt32,
            18,
        > as NonRepeatedFieldType>::get_field_opt(&self.s32_required, &self._bitfield)
        .is_some()
    }
    pub fn s32_required_mut(&mut self) -> &mut i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::SInt32,
            18,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.s32_required,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_s32_required(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::SInt32,
            18,
        > as NonRepeatedFieldType>::clear(&mut self.s32_required, &mut self._bitfield)
    }
    // Optional, Variant(SInt32)
    pub fn s32_optional(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::SInt32,
            19,
        > as NonRepeatedFieldType>::get_field(
            &self.s32_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn s32_optional_opt(&self) -> ::std::option::Option<i32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::SInt32,
            19,
        > as NonRepeatedFieldType>::get_field_opt(&self.s32_optional, &self._bitfield)
    }
    pub fn has_s32_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::SInt32,
            19,
        > as NonRepeatedFieldType>::get_field_opt(&self.s32_optional, &self._bitfield)
        .is_some()
    }
    pub fn s32_optional_mut(&mut self) -> &mut i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::SInt32,
            19,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.s32_optional,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_s32_optional(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::SInt32,
            19,
        > as NonRepeatedFieldType>::clear(&mut self.s32_optional, &mut self._bitfield)
    }
    // Repeated, Variant(SInt32)
    pub fn s32_repeated(&self) -> &[i32] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            i32,
            self::_puroro::tags::SInt32,
        > as RepeatedFieldType>::get_field(&self.s32_repeated, &self._bitfield)
    }
    pub fn s32_repeated_mut(&mut self) -> &mut ::std::vec::Vec<i32> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            i32,
            self::_puroro::tags::SInt32,
        > as RepeatedFieldType>::mut_field(&mut self.s32_repeated, &mut self._bitfield)
    }
    pub fn clear_s32_repeated(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            i32,
            self::_puroro::tags::SInt32,
        > as RepeatedFieldType>::clear(&mut self.s32_repeated, &mut self._bitfield)
    }
    // Optional, Variant(SInt64)
    pub fn s64_required(&self) -> i64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::SInt64,
            20,
        > as NonRepeatedFieldType>::get_field(
            &self.s64_required,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn s64_required_opt(&self) -> ::std::option::Option<i64> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::SInt64,
            20,
        > as NonRepeatedFieldType>::get_field_opt(&self.s64_required, &self._bitfield)
    }
    pub fn has_s64_required(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::SInt64,
            20,
        > as NonRepeatedFieldType>::get_field_opt(&self.s64_required, &self._bitfield)
        .is_some()
    }
    pub fn s64_required_mut(&mut self) -> &mut i64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::SInt64,
            20,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.s64_required,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_s64_required(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::SInt64,
            20,
        > as NonRepeatedFieldType>::clear(&mut self.s64_required, &mut self._bitfield)
    }
    // Optional, Variant(SInt64)
    pub fn s64_optional(&self) -> i64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::SInt64,
            21,
        > as NonRepeatedFieldType>::get_field(
            &self.s64_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn s64_optional_opt(&self) -> ::std::option::Option<i64> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::SInt64,
            21,
        > as NonRepeatedFieldType>::get_field_opt(&self.s64_optional, &self._bitfield)
    }
    pub fn has_s64_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::SInt64,
            21,
        > as NonRepeatedFieldType>::get_field_opt(&self.s64_optional, &self._bitfield)
        .is_some()
    }
    pub fn s64_optional_mut(&mut self) -> &mut i64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::SInt64,
            21,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.s64_optional,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_s64_optional(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::SInt64,
            21,
        > as NonRepeatedFieldType>::clear(&mut self.s64_optional, &mut self._bitfield)
    }
    // Repeated, Variant(SInt64)
    pub fn s64_repeated(&self) -> &[i64] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            i64,
            self::_puroro::tags::SInt64,
        > as RepeatedFieldType>::get_field(&self.s64_repeated, &self._bitfield)
    }
    pub fn s64_repeated_mut(&mut self) -> &mut ::std::vec::Vec<i64> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            i64,
            self::_puroro::tags::SInt64,
        > as RepeatedFieldType>::mut_field(&mut self.s64_repeated, &mut self._bitfield)
    }
    pub fn clear_s64_repeated(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            i64,
            self::_puroro::tags::SInt64,
        > as RepeatedFieldType>::clear(&mut self.s64_repeated, &mut self._bitfield)
    }
    // Optional, Bits32(Fixed32)
    pub fn fixed32_required(&self) -> u32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u32,
            self::_puroro::tags::Fixed32,
            22,
        > as NonRepeatedFieldType>::get_field(
            &self.fixed32_required,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn fixed32_required_opt(&self) -> ::std::option::Option<u32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u32,
            self::_puroro::tags::Fixed32,
            22,
        > as NonRepeatedFieldType>::get_field_opt(&self.fixed32_required, &self._bitfield)
    }
    pub fn has_fixed32_required(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u32,
            self::_puroro::tags::Fixed32,
            22,
        > as NonRepeatedFieldType>::get_field_opt(&self.fixed32_required, &self._bitfield)
        .is_some()
    }
    pub fn fixed32_required_mut(&mut self) -> &mut u32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u32,
            self::_puroro::tags::Fixed32,
            22,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.fixed32_required,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_fixed32_required(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u32,
            self::_puroro::tags::Fixed32,
            22,
        > as NonRepeatedFieldType>::clear(&mut self.fixed32_required, &mut self._bitfield)
    }
    // Optional, Bits32(Fixed32)
    pub fn fixed32_optional(&self) -> u32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u32,
            self::_puroro::tags::Fixed32,
            23,
        > as NonRepeatedFieldType>::get_field(
            &self.fixed32_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn fixed32_optional_opt(&self) -> ::std::option::Option<u32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u32,
            self::_puroro::tags::Fixed32,
            23,
        > as NonRepeatedFieldType>::get_field_opt(&self.fixed32_optional, &self._bitfield)
    }
    pub fn has_fixed32_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u32,
            self::_puroro::tags::Fixed32,
            23,
        > as NonRepeatedFieldType>::get_field_opt(&self.fixed32_optional, &self._bitfield)
        .is_some()
    }
    pub fn fixed32_optional_mut(&mut self) -> &mut u32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u32,
            self::_puroro::tags::Fixed32,
            23,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.fixed32_optional,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_fixed32_optional(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u32,
            self::_puroro::tags::Fixed32,
            23,
        > as NonRepeatedFieldType>::clear(&mut self.fixed32_optional, &mut self._bitfield)
    }
    // Repeated, Bits32(Fixed32)
    pub fn fixed32_repeated(&self) -> &[u32] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            u32,
            self::_puroro::tags::Fixed32,
        > as RepeatedFieldType>::get_field(&self.fixed32_repeated, &self._bitfield)
    }
    pub fn fixed32_repeated_mut(&mut self) -> &mut ::std::vec::Vec<u32> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            u32,
            self::_puroro::tags::Fixed32,
        > as RepeatedFieldType>::mut_field(&mut self.fixed32_repeated, &mut self._bitfield)
    }
    pub fn clear_fixed32_repeated(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            u32,
            self::_puroro::tags::Fixed32,
        > as RepeatedFieldType>::clear(&mut self.fixed32_repeated, &mut self._bitfield)
    }
    // Optional, Bits64(Fixed64)
    pub fn fixed64_required(&self) -> u64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u64,
            self::_puroro::tags::Fixed64,
            24,
        > as NonRepeatedFieldType>::get_field(
            &self.fixed64_required,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn fixed64_required_opt(&self) -> ::std::option::Option<u64> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u64,
            self::_puroro::tags::Fixed64,
            24,
        > as NonRepeatedFieldType>::get_field_opt(&self.fixed64_required, &self._bitfield)
    }
    pub fn has_fixed64_required(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u64,
            self::_puroro::tags::Fixed64,
            24,
        > as NonRepeatedFieldType>::get_field_opt(&self.fixed64_required, &self._bitfield)
        .is_some()
    }
    pub fn fixed64_required_mut(&mut self) -> &mut u64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u64,
            self::_puroro::tags::Fixed64,
            24,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.fixed64_required,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_fixed64_required(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u64,
            self::_puroro::tags::Fixed64,
            24,
        > as NonRepeatedFieldType>::clear(&mut self.fixed64_required, &mut self._bitfield)
    }
    // Optional, Bits64(Fixed64)
    pub fn fixed64_optional(&self) -> u64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u64,
            self::_puroro::tags::Fixed64,
            25,
        > as NonRepeatedFieldType>::get_field(
            &self.fixed64_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn fixed64_optional_opt(&self) -> ::std::option::Option<u64> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u64,
            self::_puroro::tags::Fixed64,
            25,
        > as NonRepeatedFieldType>::get_field_opt(&self.fixed64_optional, &self._bitfield)
    }
    pub fn has_fixed64_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u64,
            self::_puroro::tags::Fixed64,
            25,
        > as NonRepeatedFieldType>::get_field_opt(&self.fixed64_optional, &self._bitfield)
        .is_some()
    }
    pub fn fixed64_optional_mut(&mut self) -> &mut u64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u64,
            self::_puroro::tags::Fixed64,
            25,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.fixed64_optional,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_fixed64_optional(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u64,
            self::_puroro::tags::Fixed64,
            25,
        > as NonRepeatedFieldType>::clear(&mut self.fixed64_optional, &mut self._bitfield)
    }
    // Repeated, Bits64(Fixed64)
    pub fn fixed64_repeated(&self) -> &[u64] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            u64,
            self::_puroro::tags::Fixed64,
        > as RepeatedFieldType>::get_field(&self.fixed64_repeated, &self._bitfield)
    }
    pub fn fixed64_repeated_mut(&mut self) -> &mut ::std::vec::Vec<u64> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            u64,
            self::_puroro::tags::Fixed64,
        > as RepeatedFieldType>::mut_field(&mut self.fixed64_repeated, &mut self._bitfield)
    }
    pub fn clear_fixed64_repeated(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            u64,
            self::_puroro::tags::Fixed64,
        > as RepeatedFieldType>::clear(&mut self.fixed64_repeated, &mut self._bitfield)
    }
    // Optional, Bits32(SFixed32)
    pub fn sfixed32_required(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::SFixed32,
            26,
        > as NonRepeatedFieldType>::get_field(
            &self.sfixed32_required,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn sfixed32_required_opt(&self) -> ::std::option::Option<i32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::SFixed32,
            26,
        > as NonRepeatedFieldType>::get_field_opt(&self.sfixed32_required, &self._bitfield)
    }
    pub fn has_sfixed32_required(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::SFixed32,
            26,
        > as NonRepeatedFieldType>::get_field_opt(&self.sfixed32_required, &self._bitfield)
        .is_some()
    }
    pub fn sfixed32_required_mut(&mut self) -> &mut i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::SFixed32,
            26,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.sfixed32_required,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_sfixed32_required(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::SFixed32,
            26,
        > as NonRepeatedFieldType>::clear(&mut self.sfixed32_required, &mut self._bitfield)
    }
    // Optional, Bits32(SFixed32)
    pub fn sfixed32_optional(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::SFixed32,
            27,
        > as NonRepeatedFieldType>::get_field(
            &self.sfixed32_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn sfixed32_optional_opt(&self) -> ::std::option::Option<i32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::SFixed32,
            27,
        > as NonRepeatedFieldType>::get_field_opt(&self.sfixed32_optional, &self._bitfield)
    }
    pub fn has_sfixed32_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::SFixed32,
            27,
        > as NonRepeatedFieldType>::get_field_opt(&self.sfixed32_optional, &self._bitfield)
        .is_some()
    }
    pub fn sfixed32_optional_mut(&mut self) -> &mut i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::SFixed32,
            27,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.sfixed32_optional,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_sfixed32_optional(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::SFixed32,
            27,
        > as NonRepeatedFieldType>::clear(&mut self.sfixed32_optional, &mut self._bitfield)
    }
    // Repeated, Bits32(SFixed32)
    pub fn sfixed32_repeated(&self) -> &[i32] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            i32,
            self::_puroro::tags::SFixed32,
        > as RepeatedFieldType>::get_field(&self.sfixed32_repeated, &self._bitfield)
    }
    pub fn sfixed32_repeated_mut(&mut self) -> &mut ::std::vec::Vec<i32> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            i32,
            self::_puroro::tags::SFixed32,
        > as RepeatedFieldType>::mut_field(&mut self.sfixed32_repeated, &mut self._bitfield)
    }
    pub fn clear_sfixed32_repeated(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            i32,
            self::_puroro::tags::SFixed32,
        > as RepeatedFieldType>::clear(&mut self.sfixed32_repeated, &mut self._bitfield)
    }
    // Optional, Bits64(SFixed64)
    pub fn sfixed64_required(&self) -> i64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::SFixed64,
            28,
        > as NonRepeatedFieldType>::get_field(
            &self.sfixed64_required,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn sfixed64_required_opt(&self) -> ::std::option::Option<i64> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::SFixed64,
            28,
        > as NonRepeatedFieldType>::get_field_opt(&self.sfixed64_required, &self._bitfield)
    }
    pub fn has_sfixed64_required(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::SFixed64,
            28,
        > as NonRepeatedFieldType>::get_field_opt(&self.sfixed64_required, &self._bitfield)
        .is_some()
    }
    pub fn sfixed64_required_mut(&mut self) -> &mut i64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::SFixed64,
            28,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.sfixed64_required,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_sfixed64_required(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::SFixed64,
            28,
        > as NonRepeatedFieldType>::clear(&mut self.sfixed64_required, &mut self._bitfield)
    }
    // Optional, Bits64(SFixed64)
    pub fn sfixed64_optional(&self) -> i64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::SFixed64,
            29,
        > as NonRepeatedFieldType>::get_field(
            &self.sfixed64_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn sfixed64_optional_opt(&self) -> ::std::option::Option<i64> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::SFixed64,
            29,
        > as NonRepeatedFieldType>::get_field_opt(&self.sfixed64_optional, &self._bitfield)
    }
    pub fn has_sfixed64_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::SFixed64,
            29,
        > as NonRepeatedFieldType>::get_field_opt(&self.sfixed64_optional, &self._bitfield)
        .is_some()
    }
    pub fn sfixed64_optional_mut(&mut self) -> &mut i64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::SFixed64,
            29,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.sfixed64_optional,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_sfixed64_optional(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::SFixed64,
            29,
        > as NonRepeatedFieldType>::clear(&mut self.sfixed64_optional, &mut self._bitfield)
    }
    // Repeated, Bits64(SFixed64)
    pub fn sfixed64_repeated(&self) -> &[i64] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            i64,
            self::_puroro::tags::SFixed64,
        > as RepeatedFieldType>::get_field(&self.sfixed64_repeated, &self._bitfield)
    }
    pub fn sfixed64_repeated_mut(&mut self) -> &mut ::std::vec::Vec<i64> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            i64,
            self::_puroro::tags::SFixed64,
        > as RepeatedFieldType>::mut_field(&mut self.sfixed64_repeated, &mut self._bitfield)
    }
    pub fn clear_sfixed64_repeated(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            i64,
            self::_puroro::tags::SFixed64,
        > as RepeatedFieldType>::clear(&mut self.sfixed64_repeated, &mut self._bitfield)
    }
    // Optional, Bits64(Double)
    pub fn f64_required(&self) -> f64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            f64,
            self::_puroro::tags::Double,
            30,
        > as NonRepeatedFieldType>::get_field(
            &self.f64_required,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn f64_required_opt(&self) -> ::std::option::Option<f64> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            f64,
            self::_puroro::tags::Double,
            30,
        > as NonRepeatedFieldType>::get_field_opt(&self.f64_required, &self._bitfield)
    }
    pub fn has_f64_required(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            f64,
            self::_puroro::tags::Double,
            30,
        > as NonRepeatedFieldType>::get_field_opt(&self.f64_required, &self._bitfield)
        .is_some()
    }
    pub fn f64_required_mut(&mut self) -> &mut f64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            f64,
            self::_puroro::tags::Double,
            30,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.f64_required,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_f64_required(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            f64,
            self::_puroro::tags::Double,
            30,
        > as NonRepeatedFieldType>::clear(&mut self.f64_required, &mut self._bitfield)
    }
    // Optional, Bits64(Double)
    pub fn f64_optional(&self) -> f64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            f64,
            self::_puroro::tags::Double,
            31,
        > as NonRepeatedFieldType>::get_field(
            &self.f64_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn f64_optional_opt(&self) -> ::std::option::Option<f64> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            f64,
            self::_puroro::tags::Double,
            31,
        > as NonRepeatedFieldType>::get_field_opt(&self.f64_optional, &self._bitfield)
    }
    pub fn has_f64_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            f64,
            self::_puroro::tags::Double,
            31,
        > as NonRepeatedFieldType>::get_field_opt(&self.f64_optional, &self._bitfield)
        .is_some()
    }
    pub fn f64_optional_mut(&mut self) -> &mut f64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            f64,
            self::_puroro::tags::Double,
            31,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.f64_optional,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_f64_optional(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            f64,
            self::_puroro::tags::Double,
            31,
        > as NonRepeatedFieldType>::clear(&mut self.f64_optional, &mut self._bitfield)
    }
    // Repeated, Bits64(Double)
    pub fn f64_repeated(&self) -> &[f64] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            f64,
            self::_puroro::tags::Double,
        > as RepeatedFieldType>::get_field(&self.f64_repeated, &self._bitfield)
    }
    pub fn f64_repeated_mut(&mut self) -> &mut ::std::vec::Vec<f64> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            f64,
            self::_puroro::tags::Double,
        > as RepeatedFieldType>::mut_field(&mut self.f64_repeated, &mut self._bitfield)
    }
    pub fn clear_f64_repeated(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            f64,
            self::_puroro::tags::Double,
        > as RepeatedFieldType>::clear(&mut self.f64_repeated, &mut self._bitfield)
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
                1 => <
                    self::_puroro::internal::field_type::OptionalNumericalField::<i32, self::_puroro::tags::Int32, 0> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.i32_required,
                    &mut self._bitfield,
                    _field_data,
                )?,
                2 => <
                    self::_puroro::internal::field_type::OptionalNumericalField::<i32, self::_puroro::tags::Int32, 1> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.i32_optional,
                    &mut self._bitfield,
                    _field_data,
                )?,
                3 => <
                    self::_puroro::internal::field_type::RepeatedNumericalField::<i32, self::_puroro::tags::Int32> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.i32_repeated,
                    &mut self._bitfield,
                    _field_data,
                )?,
                11 => <
                    self::_puroro::internal::field_type::OptionalNumericalField::<f32, self::_puroro::tags::Float, 2> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.float_required,
                    &mut self._bitfield,
                    _field_data,
                )?,
                12 => <
                    self::_puroro::internal::field_type::OptionalNumericalField::<f32, self::_puroro::tags::Float, 3> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.float_optional,
                    &mut self._bitfield,
                    _field_data,
                )?,
                13 => <
                    self::_puroro::internal::field_type::RepeatedNumericalField::<f32, self::_puroro::tags::Float> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.float_repeated,
                    &mut self._bitfield,
                    _field_data,
                )?,
                21 => <
                    self::_puroro::internal::field_type::OptionalBytesField<4> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.bytes_required,
                    &mut self._bitfield,
                    _field_data,
                )?,
                22 => <
                    self::_puroro::internal::field_type::OptionalBytesField<5> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.bytes_optional,
                    &mut self._bitfield,
                    _field_data,
                )?,
                23 => <
                    self::_puroro::internal::field_type::RepeatedBytesField as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.bytes_repeated,
                    &mut self._bitfield,
                    _field_data,
                )?,
                31 => <
                    self::_puroro::internal::field_type::OptionalStringField::<6> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.string_required,
                    &mut self._bitfield,
                    _field_data,
                )?,
                32 => <
                    self::_puroro::internal::field_type::OptionalStringField::<7> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.string_optional,
                    &mut self._bitfield,
                    _field_data,
                )?,
                33 => <
                    self::_puroro::internal::field_type::RepeatedStringField as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.string_repeated,
                    &mut self._bitfield,
                    _field_data,
                )?,
                41 => <
                    self::_puroro::internal::field_type::OptionalNumericalField::<_puroro_root::full_coverage2::Enum, self::_puroro::tags::Enum2<_puroro_root::full_coverage2::Enum>, 8> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.enum_required,
                    &mut self._bitfield,
                    _field_data,
                )?,
                42 => <
                    self::_puroro::internal::field_type::OptionalNumericalField::<_puroro_root::full_coverage2::Enum, self::_puroro::tags::Enum2<_puroro_root::full_coverage2::Enum>, 9> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.enum_optional,
                    &mut self._bitfield,
                    _field_data,
                )?,
                43 => <
                    self::_puroro::internal::field_type::RepeatedNumericalField::<_puroro_root::full_coverage2::Enum, self::_puroro::tags::Enum2<_puroro_root::full_coverage2::Enum>> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.enum_repeated,
                    &mut self._bitfield,
                    _field_data,
                )?,
                51 => <
                    self::_puroro::internal::field_type::SingularHeapMessageField::<_puroro_root::full_coverage2::msg::Submsg> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.submsg_required,
                    &mut self._bitfield,
                    _field_data,
                )?,
                52 => <
                    self::_puroro::internal::field_type::SingularHeapMessageField::<_puroro_root::full_coverage2::msg::Submsg> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.submsg_optional,
                    &mut self._bitfield,
                    _field_data,
                )?,
                53 => <
                    self::_puroro::internal::field_type::RepeatedMessageField::<_puroro_root::full_coverage2::msg::Submsg> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.submsg_repeated,
                    &mut self._bitfield,
                    _field_data,
                )?,
                101 => <
                    self::_puroro::internal::field_type::OptionalNumericalField::<i64, self::_puroro::tags::Int64, 12> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.i64_required,
                    &mut self._bitfield,
                    _field_data,
                )?,
                102 => <
                    self::_puroro::internal::field_type::OptionalNumericalField::<i64, self::_puroro::tags::Int64, 13> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.i64_optional,
                    &mut self._bitfield,
                    _field_data,
                )?,
                103 => <
                    self::_puroro::internal::field_type::RepeatedNumericalField::<i64, self::_puroro::tags::Int64> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.i64_repeated,
                    &mut self._bitfield,
                    _field_data,
                )?,
                111 => <
                    self::_puroro::internal::field_type::OptionalNumericalField::<u32, self::_puroro::tags::UInt32, 14> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.u32_required,
                    &mut self._bitfield,
                    _field_data,
                )?,
                112 => <
                    self::_puroro::internal::field_type::OptionalNumericalField::<u32, self::_puroro::tags::UInt32, 15> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.u32_optional,
                    &mut self._bitfield,
                    _field_data,
                )?,
                113 => <
                    self::_puroro::internal::field_type::RepeatedNumericalField::<u32, self::_puroro::tags::UInt32> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.u32_repeated,
                    &mut self._bitfield,
                    _field_data,
                )?,
                121 => <
                    self::_puroro::internal::field_type::OptionalNumericalField::<u64, self::_puroro::tags::UInt64, 16> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.u64_required,
                    &mut self._bitfield,
                    _field_data,
                )?,
                122 => <
                    self::_puroro::internal::field_type::OptionalNumericalField::<u64, self::_puroro::tags::UInt64, 17> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.u64_optional,
                    &mut self._bitfield,
                    _field_data,
                )?,
                123 => <
                    self::_puroro::internal::field_type::RepeatedNumericalField::<u64, self::_puroro::tags::UInt64> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.u64_repeated,
                    &mut self._bitfield,
                    _field_data,
                )?,
                131 => <
                    self::_puroro::internal::field_type::OptionalNumericalField::<i32, self::_puroro::tags::SInt32, 18> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.s32_required,
                    &mut self._bitfield,
                    _field_data,
                )?,
                132 => <
                    self::_puroro::internal::field_type::OptionalNumericalField::<i32, self::_puroro::tags::SInt32, 19> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.s32_optional,
                    &mut self._bitfield,
                    _field_data,
                )?,
                133 => <
                    self::_puroro::internal::field_type::RepeatedNumericalField::<i32, self::_puroro::tags::SInt32> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.s32_repeated,
                    &mut self._bitfield,
                    _field_data,
                )?,
                141 => <
                    self::_puroro::internal::field_type::OptionalNumericalField::<i64, self::_puroro::tags::SInt64, 20> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.s64_required,
                    &mut self._bitfield,
                    _field_data,
                )?,
                142 => <
                    self::_puroro::internal::field_type::OptionalNumericalField::<i64, self::_puroro::tags::SInt64, 21> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.s64_optional,
                    &mut self._bitfield,
                    _field_data,
                )?,
                143 => <
                    self::_puroro::internal::field_type::RepeatedNumericalField::<i64, self::_puroro::tags::SInt64> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.s64_repeated,
                    &mut self._bitfield,
                    _field_data,
                )?,
                151 => <
                    self::_puroro::internal::field_type::OptionalNumericalField::<u32, self::_puroro::tags::Fixed32, 22> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.fixed32_required,
                    &mut self._bitfield,
                    _field_data,
                )?,
                152 => <
                    self::_puroro::internal::field_type::OptionalNumericalField::<u32, self::_puroro::tags::Fixed32, 23> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.fixed32_optional,
                    &mut self._bitfield,
                    _field_data,
                )?,
                153 => <
                    self::_puroro::internal::field_type::RepeatedNumericalField::<u32, self::_puroro::tags::Fixed32> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.fixed32_repeated,
                    &mut self._bitfield,
                    _field_data,
                )?,
                161 => <
                    self::_puroro::internal::field_type::OptionalNumericalField::<u64, self::_puroro::tags::Fixed64, 24> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.fixed64_required,
                    &mut self._bitfield,
                    _field_data,
                )?,
                162 => <
                    self::_puroro::internal::field_type::OptionalNumericalField::<u64, self::_puroro::tags::Fixed64, 25> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.fixed64_optional,
                    &mut self._bitfield,
                    _field_data,
                )?,
                163 => <
                    self::_puroro::internal::field_type::RepeatedNumericalField::<u64, self::_puroro::tags::Fixed64> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.fixed64_repeated,
                    &mut self._bitfield,
                    _field_data,
                )?,
                171 => <
                    self::_puroro::internal::field_type::OptionalNumericalField::<i32, self::_puroro::tags::SFixed32, 26> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.sfixed32_required,
                    &mut self._bitfield,
                    _field_data,
                )?,
                172 => <
                    self::_puroro::internal::field_type::OptionalNumericalField::<i32, self::_puroro::tags::SFixed32, 27> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.sfixed32_optional,
                    &mut self._bitfield,
                    _field_data,
                )?,
                173 => <
                    self::_puroro::internal::field_type::RepeatedNumericalField::<i32, self::_puroro::tags::SFixed32> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.sfixed32_repeated,
                    &mut self._bitfield,
                    _field_data,
                )?,
                181 => <
                    self::_puroro::internal::field_type::OptionalNumericalField::<i64, self::_puroro::tags::SFixed64, 28> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.sfixed64_required,
                    &mut self._bitfield,
                    _field_data,
                )?,
                182 => <
                    self::_puroro::internal::field_type::OptionalNumericalField::<i64, self::_puroro::tags::SFixed64, 29> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.sfixed64_optional,
                    &mut self._bitfield,
                    _field_data,
                )?,
                183 => <
                    self::_puroro::internal::field_type::RepeatedNumericalField::<i64, self::_puroro::tags::SFixed64> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.sfixed64_repeated,
                    &mut self._bitfield,
                    _field_data,
                )?,
                191 => <
                    self::_puroro::internal::field_type::OptionalNumericalField::<f64, self::_puroro::tags::Double, 30> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.f64_required,
                    &mut self._bitfield,
                    _field_data,
                )?,
                192 => <
                    self::_puroro::internal::field_type::OptionalNumericalField::<f64, self::_puroro::tags::Double, 31> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.f64_optional,
                    &mut self._bitfield,
                    _field_data,
                )?,
                193 => <
                    self::_puroro::internal::field_type::RepeatedNumericalField::<f64, self::_puroro::tags::Double> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.f64_repeated,
                    &mut self._bitfield,
                    _field_data,
                )?,
                _ => todo!(),
            }
        }
        Ok(())
    }

    fn to_bytes<W: ::std::io::Write>(&self, out: &mut W) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use ::std::result::Result::Ok;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.i32_required,
            &self._bitfield,
            1,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            1,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.i32_optional,
            &self._bitfield,
            2,
            out,
        )?;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            i32,
            self::_puroro::tags::Int32,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.i32_repeated,
            &self._bitfield,
            3,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            f32,
            self::_puroro::tags::Float,
            2,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.float_required,
            &self._bitfield,
            11,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            f32,
            self::_puroro::tags::Float,
            3,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.float_optional,
            &self._bitfield,
            12,
            out,
        )?;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            f32,
            self::_puroro::tags::Float,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.float_repeated,
            &self._bitfield,
            13,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalBytesField<4> as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.bytes_required,
            &self._bitfield,
            21,
            out
        )?;
        <self::_puroro::internal::field_type::OptionalBytesField<5> as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.bytes_optional,
            &self._bitfield,
            22,
            out
        )?;
        <self::_puroro::internal::field_type::RepeatedBytesField as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.bytes_repeated,
            &self._bitfield,
            23,
            out
        )?;
        <self::_puroro::internal::field_type::OptionalStringField::<6> as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.string_required,
            &self._bitfield,
            31,
            out
        )?;
        <self::_puroro::internal::field_type::OptionalStringField::<7> as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.string_optional,
            &self._bitfield,
            32,
            out
        )?;
        <self::_puroro::internal::field_type::RepeatedStringField as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.string_repeated,
            &self._bitfield,
            33,
            out
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::full_coverage2::Enum,
            self::_puroro::tags::Enum2<_puroro_root::full_coverage2::Enum>,
            8,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.enum_required,
            &self._bitfield,
            41,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::full_coverage2::Enum,
            self::_puroro::tags::Enum2<_puroro_root::full_coverage2::Enum>,
            9,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.enum_optional,
            &self._bitfield,
            42,
            out,
        )?;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            _puroro_root::full_coverage2::Enum,
            self::_puroro::tags::Enum2<_puroro_root::full_coverage2::Enum>,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.enum_repeated,
            &self._bitfield,
            43,
            out,
        )?;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::full_coverage2::msg::Submsg,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.submsg_required,
            &self._bitfield,
            51,
            out,
        )?;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::full_coverage2::msg::Submsg,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.submsg_optional,
            &self._bitfield,
            52,
            out,
        )?;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::full_coverage2::msg::Submsg,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.submsg_repeated,
            &self._bitfield,
            53,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::Int64,
            12,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.i64_required,
            &self._bitfield,
            101,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::Int64,
            13,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.i64_optional,
            &self._bitfield,
            102,
            out,
        )?;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            i64,
            self::_puroro::tags::Int64,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.i64_repeated,
            &self._bitfield,
            103,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u32,
            self::_puroro::tags::UInt32,
            14,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.u32_required,
            &self._bitfield,
            111,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u32,
            self::_puroro::tags::UInt32,
            15,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.u32_optional,
            &self._bitfield,
            112,
            out,
        )?;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            u32,
            self::_puroro::tags::UInt32,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.u32_repeated,
            &self._bitfield,
            113,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u64,
            self::_puroro::tags::UInt64,
            16,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.u64_required,
            &self._bitfield,
            121,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u64,
            self::_puroro::tags::UInt64,
            17,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.u64_optional,
            &self._bitfield,
            122,
            out,
        )?;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            u64,
            self::_puroro::tags::UInt64,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.u64_repeated,
            &self._bitfield,
            123,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::SInt32,
            18,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.s32_required,
            &self._bitfield,
            131,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::SInt32,
            19,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.s32_optional,
            &self._bitfield,
            132,
            out,
        )?;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            i32,
            self::_puroro::tags::SInt32,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.s32_repeated,
            &self._bitfield,
            133,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::SInt64,
            20,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.s64_required,
            &self._bitfield,
            141,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::SInt64,
            21,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.s64_optional,
            &self._bitfield,
            142,
            out,
        )?;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            i64,
            self::_puroro::tags::SInt64,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.s64_repeated,
            &self._bitfield,
            143,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u32,
            self::_puroro::tags::Fixed32,
            22,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.fixed32_required,
            &self._bitfield,
            151,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u32,
            self::_puroro::tags::Fixed32,
            23,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.fixed32_optional,
            &self._bitfield,
            152,
            out,
        )?;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            u32,
            self::_puroro::tags::Fixed32,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.fixed32_repeated,
            &self._bitfield,
            153,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u64,
            self::_puroro::tags::Fixed64,
            24,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.fixed64_required,
            &self._bitfield,
            161,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u64,
            self::_puroro::tags::Fixed64,
            25,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.fixed64_optional,
            &self._bitfield,
            162,
            out,
        )?;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            u64,
            self::_puroro::tags::Fixed64,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.fixed64_repeated,
            &self._bitfield,
            163,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::SFixed32,
            26,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.sfixed32_required,
            &self._bitfield,
            171,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::SFixed32,
            27,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.sfixed32_optional,
            &self._bitfield,
            172,
            out,
        )?;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            i32,
            self::_puroro::tags::SFixed32,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.sfixed32_repeated,
            &self._bitfield,
            173,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::SFixed64,
            28,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.sfixed64_required,
            &self._bitfield,
            181,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::SFixed64,
            29,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.sfixed64_optional,
            &self._bitfield,
            182,
            out,
        )?;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            i64,
            self::_puroro::tags::SFixed64,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.sfixed64_repeated,
            &self._bitfield,
            183,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            f64,
            self::_puroro::tags::Double,
            30,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.f64_required,
            &self._bitfield,
            191,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            f64,
            self::_puroro::tags::Double,
            31,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.f64_optional,
            &self._bitfield,
            192,
            out,
        )?;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            f64,
            self::_puroro::tags::Double,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.f64_repeated,
            &self._bitfield,
            193,
            out,
        )?;
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
            i32_required: Clone::clone(&self.i32_required),
            i32_optional: Clone::clone(&self.i32_optional),
            i32_repeated: Clone::clone(&self.i32_repeated),
            float_required: Clone::clone(&self.float_required),
            float_optional: Clone::clone(&self.float_optional),
            float_repeated: Clone::clone(&self.float_repeated),
            bytes_required: Clone::clone(&self.bytes_required),
            bytes_optional: Clone::clone(&self.bytes_optional),
            bytes_repeated: Clone::clone(&self.bytes_repeated),
            string_required: Clone::clone(&self.string_required),
            string_optional: Clone::clone(&self.string_optional),
            string_repeated: Clone::clone(&self.string_repeated),
            enum_required: Clone::clone(&self.enum_required),
            enum_optional: Clone::clone(&self.enum_optional),
            enum_repeated: Clone::clone(&self.enum_repeated),
            submsg_required: Clone::clone(&self.submsg_required),
            submsg_optional: Clone::clone(&self.submsg_optional),
            submsg_repeated: Clone::clone(&self.submsg_repeated),
            i64_required: Clone::clone(&self.i64_required),
            i64_optional: Clone::clone(&self.i64_optional),
            i64_repeated: Clone::clone(&self.i64_repeated),
            u32_required: Clone::clone(&self.u32_required),
            u32_optional: Clone::clone(&self.u32_optional),
            u32_repeated: Clone::clone(&self.u32_repeated),
            u64_required: Clone::clone(&self.u64_required),
            u64_optional: Clone::clone(&self.u64_optional),
            u64_repeated: Clone::clone(&self.u64_repeated),
            s32_required: Clone::clone(&self.s32_required),
            s32_optional: Clone::clone(&self.s32_optional),
            s32_repeated: Clone::clone(&self.s32_repeated),
            s64_required: Clone::clone(&self.s64_required),
            s64_optional: Clone::clone(&self.s64_optional),
            s64_repeated: Clone::clone(&self.s64_repeated),
            fixed32_required: Clone::clone(&self.fixed32_required),
            fixed32_optional: Clone::clone(&self.fixed32_optional),
            fixed32_repeated: Clone::clone(&self.fixed32_repeated),
            fixed64_required: Clone::clone(&self.fixed64_required),
            fixed64_optional: Clone::clone(&self.fixed64_optional),
            fixed64_repeated: Clone::clone(&self.fixed64_repeated),
            sfixed32_required: Clone::clone(&self.sfixed32_required),
            sfixed32_optional: Clone::clone(&self.sfixed32_optional),
            sfixed32_repeated: Clone::clone(&self.sfixed32_repeated),
            sfixed64_required: Clone::clone(&self.sfixed64_required),
            sfixed64_optional: Clone::clone(&self.sfixed64_optional),
            sfixed64_repeated: Clone::clone(&self.sfixed64_repeated),
            f64_required: Clone::clone(&self.f64_required),
            f64_optional: Clone::clone(&self.f64_optional),
            f64_repeated: Clone::clone(&self.f64_repeated),

            _bitfield: Clone::clone(&self._bitfield),
        }
    }
}

impl ::std::fmt::Debug for Msg {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct("Msg")
            .field("i32_required", &self.i32_required())
            .field("i32_optional", &self.i32_optional())
            .field("i32_repeated", &self.i32_repeated())
            .field("float_required", &self.float_required())
            .field("float_optional", &self.float_optional())
            .field("float_repeated", &self.float_repeated())
            .field("bytes_required", &self.bytes_required())
            .field("bytes_optional", &self.bytes_optional())
            .field("bytes_repeated", &self.bytes_repeated())
            .field("string_required", &self.string_required())
            .field("string_optional", &self.string_optional())
            .field("string_repeated", &self.string_repeated())
            .field("enum_required", &self.enum_required())
            .field("enum_optional", &self.enum_optional())
            .field("enum_repeated", &self.enum_repeated())
            .field("submsg_required", &self.submsg_required())
            .field("submsg_optional", &self.submsg_optional())
            .field("submsg_repeated", &self.submsg_repeated())
            .field("i64_required", &self.i64_required())
            .field("i64_optional", &self.i64_optional())
            .field("i64_repeated", &self.i64_repeated())
            .field("u32_required", &self.u32_required())
            .field("u32_optional", &self.u32_optional())
            .field("u32_repeated", &self.u32_repeated())
            .field("u64_required", &self.u64_required())
            .field("u64_optional", &self.u64_optional())
            .field("u64_repeated", &self.u64_repeated())
            .field("s32_required", &self.s32_required())
            .field("s32_optional", &self.s32_optional())
            .field("s32_repeated", &self.s32_repeated())
            .field("s64_required", &self.s64_required())
            .field("s64_optional", &self.s64_optional())
            .field("s64_repeated", &self.s64_repeated())
            .field("fixed32_required", &self.fixed32_required())
            .field("fixed32_optional", &self.fixed32_optional())
            .field("fixed32_repeated", &self.fixed32_repeated())
            .field("fixed64_required", &self.fixed64_required())
            .field("fixed64_optional", &self.fixed64_optional())
            .field("fixed64_repeated", &self.fixed64_repeated())
            .field("sfixed32_required", &self.sfixed32_required())
            .field("sfixed32_optional", &self.sfixed32_optional())
            .field("sfixed32_repeated", &self.sfixed32_repeated())
            .field("sfixed64_required", &self.sfixed64_required())
            .field("sfixed64_optional", &self.sfixed64_optional())
            .field("sfixed64_repeated", &self.sfixed64_repeated())
            .field("f64_required", &self.f64_required())
            .field("f64_optional", &self.f64_optional())
            .field("f64_repeated", &self.f64_repeated())
            .finish()
    }
}

impl ::std::cmp::PartialEq for Msg {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;

        true && self.i32_required_opt() == rhs.i32_required_opt()
            && self.i32_optional_opt() == rhs.i32_optional_opt()
            && self.i32_repeated() == rhs.i32_repeated()
            && self.float_required_opt() == rhs.float_required_opt()
            && self.float_optional_opt() == rhs.float_optional_opt()
            && self.float_repeated() == rhs.float_repeated()
            && self.bytes_required_opt() == rhs.bytes_required_opt()
            && self.bytes_optional_opt() == rhs.bytes_optional_opt()
            && self.bytes_repeated() == rhs.bytes_repeated()
            && self.string_required_opt() == rhs.string_required_opt()
            && self.string_optional_opt() == rhs.string_optional_opt()
            && self.string_repeated() == rhs.string_repeated()
            && self.enum_required_opt() == rhs.enum_required_opt()
            && self.enum_optional_opt() == rhs.enum_optional_opt()
            && self.enum_repeated() == rhs.enum_repeated()
            && self.submsg_required_opt() == rhs.submsg_required_opt()
            && self.submsg_optional_opt() == rhs.submsg_optional_opt()
            && self.submsg_repeated() == rhs.submsg_repeated()
            && self.i64_required_opt() == rhs.i64_required_opt()
            && self.i64_optional_opt() == rhs.i64_optional_opt()
            && self.i64_repeated() == rhs.i64_repeated()
            && self.u32_required_opt() == rhs.u32_required_opt()
            && self.u32_optional_opt() == rhs.u32_optional_opt()
            && self.u32_repeated() == rhs.u32_repeated()
            && self.u64_required_opt() == rhs.u64_required_opt()
            && self.u64_optional_opt() == rhs.u64_optional_opt()
            && self.u64_repeated() == rhs.u64_repeated()
            && self.s32_required_opt() == rhs.s32_required_opt()
            && self.s32_optional_opt() == rhs.s32_optional_opt()
            && self.s32_repeated() == rhs.s32_repeated()
            && self.s64_required_opt() == rhs.s64_required_opt()
            && self.s64_optional_opt() == rhs.s64_optional_opt()
            && self.s64_repeated() == rhs.s64_repeated()
            && self.fixed32_required_opt() == rhs.fixed32_required_opt()
            && self.fixed32_optional_opt() == rhs.fixed32_optional_opt()
            && self.fixed32_repeated() == rhs.fixed32_repeated()
            && self.fixed64_required_opt() == rhs.fixed64_required_opt()
            && self.fixed64_optional_opt() == rhs.fixed64_optional_opt()
            && self.fixed64_repeated() == rhs.fixed64_repeated()
            && self.sfixed32_required_opt() == rhs.sfixed32_required_opt()
            && self.sfixed32_optional_opt() == rhs.sfixed32_optional_opt()
            && self.sfixed32_repeated() == rhs.sfixed32_repeated()
            && self.sfixed64_required_opt() == rhs.sfixed64_required_opt()
            && self.sfixed64_optional_opt() == rhs.sfixed64_optional_opt()
            && self.sfixed64_repeated() == rhs.sfixed64_repeated()
            && self.f64_required_opt() == rhs.f64_required_opt()
            && self.f64_optional_opt() == rhs.f64_optional_opt()
            && self.f64_repeated() == rhs.f64_repeated()
    }
}

impl ::std::ops::Drop for Msg {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Enum {
    ZEROTH,
    FIRST,
    TENTH,
}

impl ::std::default::Default for Enum {
    fn default() -> Self {
        Enum::ZEROTH
    }
}

impl ::std::convert::TryFrom<i32> for Enum {
    type Error = self::_puroro::PuroroError;
    fn try_from(x: i32) -> ::std::result::Result<Self, Self::Error> {
        #[allow(unused)]
        use ::std::result::Result::{Err, Ok};
        match x {
            0 => Ok(self::Enum::ZEROTH),
            1 => Ok(self::Enum::FIRST),
            10 => Ok(self::Enum::TENTH),
            e => Err(self::_puroro::ErrorKind::UnknownEnumVariant(e))?,
        }
    }
}

impl ::std::convert::From<Enum> for i32 {
    fn from(x: Enum) -> i32 {
        match x {
            self::Enum::ZEROTH => 0,
            self::Enum::FIRST => 1,
            self::Enum::TENTH => 10,
        }
    }
}
pub mod msg;
