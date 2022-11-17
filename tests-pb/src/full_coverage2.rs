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
        0usize,
    >,
    i32_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        i32,
        self::_puroro::tags::Int32,
    >,
    float_required: self::_puroro::internal::field_type::OptionalNumericalField::<
        f32,
        self::_puroro::tags::Float,
        0usize,
    >,
    float_optional: self::_puroro::internal::field_type::OptionalNumericalField::<
        f32,
        self::_puroro::tags::Float,
        0usize,
    >,
    float_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        f32,
        self::_puroro::tags::Float,
    >,
    bytes_required: self::_puroro::internal::field_type::OptionalBytesField::<0usize>,
    bytes_optional: self::_puroro::internal::field_type::OptionalBytesField::<0usize>,
    bytes_repeated: self::_puroro::internal::field_type::RepeatedBytesField,
    string_required: self::_puroro::internal::field_type::OptionalStringField::<0usize>,
    string_optional: self::_puroro::internal::field_type::OptionalStringField::<0usize>,
    string_repeated: self::_puroro::internal::field_type::RepeatedStringField,
    enum_required: self::_puroro::internal::field_type::OptionalNumericalField::<
        (),
        self::_puroro::tags::Enum2<()>,
        0usize,
    >,
    enum_optional: self::_puroro::internal::field_type::OptionalNumericalField::<
        (),
        self::_puroro::tags::Enum2<()>,
        0usize,
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
        0usize,
    >,
    i64_optional: self::_puroro::internal::field_type::OptionalNumericalField::<
        i64,
        self::_puroro::tags::Int64,
        0usize,
    >,
    i64_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        i64,
        self::_puroro::tags::Int64,
    >,
    u32_required: self::_puroro::internal::field_type::OptionalNumericalField::<
        u32,
        self::_puroro::tags::UInt32,
        0usize,
    >,
    u32_optional: self::_puroro::internal::field_type::OptionalNumericalField::<
        u32,
        self::_puroro::tags::UInt32,
        0usize,
    >,
    u32_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        u32,
        self::_puroro::tags::UInt32,
    >,
    u64_required: self::_puroro::internal::field_type::OptionalNumericalField::<
        u64,
        self::_puroro::tags::UInt64,
        0usize,
    >,
    u64_optional: self::_puroro::internal::field_type::OptionalNumericalField::<
        u64,
        self::_puroro::tags::UInt64,
        0usize,
    >,
    u64_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        u64,
        self::_puroro::tags::UInt64,
    >,
    s32_required: self::_puroro::internal::field_type::OptionalNumericalField::<
        i32,
        self::_puroro::tags::SInt32,
        0usize,
    >,
    s32_optional: self::_puroro::internal::field_type::OptionalNumericalField::<
        i32,
        self::_puroro::tags::SInt32,
        0usize,
    >,
    s32_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        i32,
        self::_puroro::tags::SInt32,
    >,
    s64_required: self::_puroro::internal::field_type::OptionalNumericalField::<
        i64,
        self::_puroro::tags::SInt64,
        0usize,
    >,
    s64_optional: self::_puroro::internal::field_type::OptionalNumericalField::<
        i64,
        self::_puroro::tags::SInt64,
        0usize,
    >,
    s64_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        i64,
        self::_puroro::tags::SInt64,
    >,
    fixed32_required: self::_puroro::internal::field_type::OptionalNumericalField::<
        u32,
        self::_puroro::tags::Fixed32,
        0usize,
    >,
    fixed32_optional: self::_puroro::internal::field_type::OptionalNumericalField::<
        u32,
        self::_puroro::tags::Fixed32,
        0usize,
    >,
    fixed32_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        u32,
        self::_puroro::tags::Fixed32,
    >,
    fixed64_required: self::_puroro::internal::field_type::OptionalNumericalField::<
        u64,
        self::_puroro::tags::Fixed64,
        0usize,
    >,
    fixed64_optional: self::_puroro::internal::field_type::OptionalNumericalField::<
        u64,
        self::_puroro::tags::Fixed64,
        0usize,
    >,
    fixed64_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        u64,
        self::_puroro::tags::Fixed64,
    >,
    sfixed32_required: self::_puroro::internal::field_type::OptionalNumericalField::<
        i32,
        self::_puroro::tags::SFixed32,
        0usize,
    >,
    sfixed32_optional: self::_puroro::internal::field_type::OptionalNumericalField::<
        i32,
        self::_puroro::tags::SFixed32,
        0usize,
    >,
    sfixed32_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        i32,
        self::_puroro::tags::SFixed32,
    >,
    sfixed64_required: self::_puroro::internal::field_type::OptionalNumericalField::<
        i64,
        self::_puroro::tags::SFixed64,
        0usize,
    >,
    sfixed64_optional: self::_puroro::internal::field_type::OptionalNumericalField::<
        i64,
        self::_puroro::tags::SFixed64,
        0usize,
    >,
    sfixed64_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        i64,
        self::_puroro::tags::SFixed64,
    >,
    f64_required: self::_puroro::internal::field_type::OptionalNumericalField::<
        f64,
        self::_puroro::tags::Double,
        0usize,
    >,
    f64_optional: self::_puroro::internal::field_type::OptionalNumericalField::<
        f64,
        self::_puroro::tags::Double,
        0usize,
    >,
    f64_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        f64,
        self::_puroro::tags::Double,
    >,
}
