//!Generated from package "full_coverage2"
pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}
pub mod _puroro {
    pub use ::puroro::*;
}
pub struct Msg {
    i32_required: self::_puroro::internal::field_type::OptionalNumericalField::<
        i32,
        self::_puroro::tags::Int32,
        0usize,
    >,
    i32_optional: self::_puroro::internal::field_type::OptionalNumericalField::<
        i32,
        self::_puroro::tags::Int32,
        1usize,
    >,
    i32_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        i32,
        self::_puroro::tags::Int32,
    >,
    float_required: self::_puroro::internal::field_type::OptionalNumericalField::<
        f32,
        self::_puroro::tags::Float,
        2usize,
    >,
    float_optional: self::_puroro::internal::field_type::OptionalNumericalField::<
        f32,
        self::_puroro::tags::Float,
        3usize,
    >,
    float_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        f32,
        self::_puroro::tags::Float,
    >,
    bytes_required: self::_puroro::internal::field_type::OptionalBytesField::<4usize>,
    bytes_optional: self::_puroro::internal::field_type::OptionalBytesField::<5usize>,
    bytes_repeated: self::_puroro::internal::field_type::RepeatedBytesField,
    string_required: self::_puroro::internal::field_type::OptionalStringField::<6usize>,
    string_optional: self::_puroro::internal::field_type::OptionalStringField::<7usize>,
    string_repeated: self::_puroro::internal::field_type::RepeatedStringField,
    enum_required: self::_puroro::internal::field_type::OptionalNumericalField::<
        (),
        self::_puroro::tags::Enum2<()>,
        8usize,
    >,
    enum_optional: self::_puroro::internal::field_type::OptionalNumericalField::<
        (),
        self::_puroro::tags::Enum2<()>,
        9usize,
    >,
    enum_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        (),
        self::_puroro::tags::Enum2<()>,
    >,
    submsg_required: self::_puroro::internal::field_type::SingularHeapMessageField::<()>,
    submsg_optional: self::_puroro::internal::field_type::SingularHeapMessageField::<()>,
    submsg_repeated: self::_puroro::internal::field_type::RepeatedMessageField::<()>,
    i64_required: self::_puroro::internal::field_type::OptionalNumericalField::<
        i64,
        self::_puroro::tags::Int64,
        10usize,
    >,
    i64_optional: self::_puroro::internal::field_type::OptionalNumericalField::<
        i64,
        self::_puroro::tags::Int64,
        11usize,
    >,
    i64_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        i64,
        self::_puroro::tags::Int64,
    >,
    u32_required: self::_puroro::internal::field_type::OptionalNumericalField::<
        u32,
        self::_puroro::tags::UInt32,
        12usize,
    >,
    u32_optional: self::_puroro::internal::field_type::OptionalNumericalField::<
        u32,
        self::_puroro::tags::UInt32,
        13usize,
    >,
    u32_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        u32,
        self::_puroro::tags::UInt32,
    >,
    u64_required: self::_puroro::internal::field_type::OptionalNumericalField::<
        u64,
        self::_puroro::tags::UInt64,
        14usize,
    >,
    u64_optional: self::_puroro::internal::field_type::OptionalNumericalField::<
        u64,
        self::_puroro::tags::UInt64,
        15usize,
    >,
    u64_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        u64,
        self::_puroro::tags::UInt64,
    >,
    s32_required: self::_puroro::internal::field_type::OptionalNumericalField::<
        i32,
        self::_puroro::tags::SInt32,
        16usize,
    >,
    s32_optional: self::_puroro::internal::field_type::OptionalNumericalField::<
        i32,
        self::_puroro::tags::SInt32,
        17usize,
    >,
    s32_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        i32,
        self::_puroro::tags::SInt32,
    >,
    s64_required: self::_puroro::internal::field_type::OptionalNumericalField::<
        i64,
        self::_puroro::tags::SInt64,
        18usize,
    >,
    s64_optional: self::_puroro::internal::field_type::OptionalNumericalField::<
        i64,
        self::_puroro::tags::SInt64,
        19usize,
    >,
    s64_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        i64,
        self::_puroro::tags::SInt64,
    >,
    fixed32_required: self::_puroro::internal::field_type::OptionalNumericalField::<
        u32,
        self::_puroro::tags::Fixed32,
        20usize,
    >,
    fixed32_optional: self::_puroro::internal::field_type::OptionalNumericalField::<
        u32,
        self::_puroro::tags::Fixed32,
        21usize,
    >,
    fixed32_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        u32,
        self::_puroro::tags::Fixed32,
    >,
    fixed64_required: self::_puroro::internal::field_type::OptionalNumericalField::<
        u64,
        self::_puroro::tags::Fixed64,
        22usize,
    >,
    fixed64_optional: self::_puroro::internal::field_type::OptionalNumericalField::<
        u64,
        self::_puroro::tags::Fixed64,
        23usize,
    >,
    fixed64_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        u64,
        self::_puroro::tags::Fixed64,
    >,
    sfixed32_required: self::_puroro::internal::field_type::OptionalNumericalField::<
        i32,
        self::_puroro::tags::SFixed32,
        24usize,
    >,
    sfixed32_optional: self::_puroro::internal::field_type::OptionalNumericalField::<
        i32,
        self::_puroro::tags::SFixed32,
        25usize,
    >,
    sfixed32_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        i32,
        self::_puroro::tags::SFixed32,
    >,
    sfixed64_required: self::_puroro::internal::field_type::OptionalNumericalField::<
        i64,
        self::_puroro::tags::SFixed64,
        26usize,
    >,
    sfixed64_optional: self::_puroro::internal::field_type::OptionalNumericalField::<
        i64,
        self::_puroro::tags::SFixed64,
        27usize,
    >,
    sfixed64_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        i64,
        self::_puroro::tags::SFixed64,
    >,
    f64_required: self::_puroro::internal::field_type::OptionalNumericalField::<
        f64,
        self::_puroro::tags::Double,
        28usize,
    >,
    f64_optional: self::_puroro::internal::field_type::OptionalNumericalField::<
        f64,
        self::_puroro::tags::Double,
        29usize,
    >,
    f64_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        f64,
        self::_puroro::tags::Double,
    >,
    _bitfield: self::_puroro::bitvec::BitArray<1usize>,
}
