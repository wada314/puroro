pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}
pub mod _puroro {
    pub use ::puroro::*;
}
pub struct Msg {
    i32_unlabeled: self::_puroro::internal::field_type::SingularNumericalField::<
        i32,
        self::_puroro::tags::Int32,
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
    float_unlabeled: self::_puroro::internal::field_type::SingularNumericalField::<
        f32,
        self::_puroro::tags::Float,
    >,
    float_optional: self::_puroro::internal::field_type::OptionalNumericalField::<
        f32,
        self::_puroro::tags::Float,
        1usize,
    >,
    float_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        f32,
        self::_puroro::tags::Float,
    >,
    bytes_unlabeled: self::_puroro::internal::field_type::SingularBytesField,
    bytes_optional: self::_puroro::internal::field_type::OptionalBytesField::<2usize>,
    bytes_repeated: self::_puroro::internal::field_type::RepeatedBytesField,
    string_unlabeled: self::_puroro::internal::field_type::SingularStringField,
    string_optional: self::_puroro::internal::field_type::OptionalStringField::<3usize>,
    string_repeated: self::_puroro::internal::field_type::RepeatedStringField,
    enum_unlabeled: self::_puroro::internal::field_type::SingularNumericalField::<
        self::_puroro_root::full_coverage3::Enum,
        self::_puroro::tags::Enum3::<self::_puroro_root::full_coverage3::Enum>,
    >,
    enum_optional: self::_puroro::internal::field_type::OptionalNumericalField::<
        self::_puroro_root::full_coverage3::Enum,
        self::_puroro::tags::Enum3::<self::_puroro_root::full_coverage3::Enum>,
        4usize,
    >,
    enum_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        self::_puroro_root::full_coverage3::Enum,
        self::_puroro::tags::Enum3::<self::_puroro_root::full_coverage3::Enum>,
    >,
    submsg_unlabeled: self::_puroro::internal::field_type::SingularHeapMessageField::<
        (),
    >,
    submsg_optional: self::_puroro::internal::field_type::SingularHeapMessageField::<()>,
    submsg_repeated: self::_puroro::internal::field_type::RepeatedMessageField::<()>,
    i64_unlabeled: self::_puroro::internal::field_type::SingularNumericalField::<
        i64,
        self::_puroro::tags::Int64,
    >,
    i64_optional: self::_puroro::internal::field_type::OptionalNumericalField::<
        i64,
        self::_puroro::tags::Int64,
        5usize,
    >,
    i64_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        i64,
        self::_puroro::tags::Int64,
    >,
    u32_unlabeled: self::_puroro::internal::field_type::SingularNumericalField::<
        u32,
        self::_puroro::tags::UInt32,
    >,
    u32_optional: self::_puroro::internal::field_type::OptionalNumericalField::<
        u32,
        self::_puroro::tags::UInt32,
        6usize,
    >,
    u32_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        u32,
        self::_puroro::tags::UInt32,
    >,
    u64_unlabeled: self::_puroro::internal::field_type::SingularNumericalField::<
        u64,
        self::_puroro::tags::UInt64,
    >,
    u64_optional: self::_puroro::internal::field_type::OptionalNumericalField::<
        u64,
        self::_puroro::tags::UInt64,
        7usize,
    >,
    u64_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        u64,
        self::_puroro::tags::UInt64,
    >,
    s32_unlabeled: self::_puroro::internal::field_type::SingularNumericalField::<
        i32,
        self::_puroro::tags::SInt32,
    >,
    s32_optional: self::_puroro::internal::field_type::OptionalNumericalField::<
        i32,
        self::_puroro::tags::SInt32,
        8usize,
    >,
    s32_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        i32,
        self::_puroro::tags::SInt32,
    >,
    s64_unlabeled: self::_puroro::internal::field_type::SingularNumericalField::<
        i64,
        self::_puroro::tags::SInt64,
    >,
    s64_optional: self::_puroro::internal::field_type::OptionalNumericalField::<
        i64,
        self::_puroro::tags::SInt64,
        9usize,
    >,
    s64_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        i64,
        self::_puroro::tags::SInt64,
    >,
    fixed32_unlabeled: self::_puroro::internal::field_type::SingularNumericalField::<
        u32,
        self::_puroro::tags::Fixed32,
    >,
    fixed32_optional: self::_puroro::internal::field_type::OptionalNumericalField::<
        u32,
        self::_puroro::tags::Fixed32,
        10usize,
    >,
    fixed32_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        u32,
        self::_puroro::tags::Fixed32,
    >,
    fixed64_unlabeled: self::_puroro::internal::field_type::SingularNumericalField::<
        u64,
        self::_puroro::tags::Fixed64,
    >,
    fixed64_optional: self::_puroro::internal::field_type::OptionalNumericalField::<
        u64,
        self::_puroro::tags::Fixed64,
        11usize,
    >,
    fixed64_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        u64,
        self::_puroro::tags::Fixed64,
    >,
    sfixed32_unlabeled: self::_puroro::internal::field_type::SingularNumericalField::<
        i32,
        self::_puroro::tags::SFixed32,
    >,
    sfixed32_optional: self::_puroro::internal::field_type::OptionalNumericalField::<
        i32,
        self::_puroro::tags::SFixed32,
        12usize,
    >,
    sfixed32_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        i32,
        self::_puroro::tags::SFixed32,
    >,
    sfixed64_unlabeled: self::_puroro::internal::field_type::SingularNumericalField::<
        i64,
        self::_puroro::tags::SFixed64,
    >,
    sfixed64_optional: self::_puroro::internal::field_type::OptionalNumericalField::<
        i64,
        self::_puroro::tags::SFixed64,
        13usize,
    >,
    sfixed64_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        i64,
        self::_puroro::tags::SFixed64,
    >,
    f64_unlabeled: self::_puroro::internal::field_type::SingularNumericalField::<
        f64,
        self::_puroro::tags::Double,
    >,
    f64_optional: self::_puroro::internal::field_type::OptionalNumericalField::<
        f64,
        self::_puroro::tags::Double,
        14usize,
    >,
    f64_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        f64,
        self::_puroro::tags::Double,
    >,
    _bitfield: self::_puroro::bitvec::BitArray<1usize>,
}
pub enum Enum {
    Zeroth,
    First,
    Tenth,
    _None(i32),
}
