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
        self::_puroro_root::full_coverage2::Enum,
        self::_puroro::tags::Enum2::<self::_puroro_root::full_coverage2::Enum>,
        8usize,
    >,
    enum_optional: self::_puroro::internal::field_type::OptionalNumericalField::<
        self::_puroro_root::full_coverage2::Enum,
        self::_puroro::tags::Enum2::<self::_puroro_root::full_coverage2::Enum>,
        9usize,
    >,
    enum_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        self::_puroro_root::full_coverage2::Enum,
        self::_puroro::tags::Enum2::<self::_puroro_root::full_coverage2::Enum>,
    >,
    submsg_required: self::_puroro::internal::field_type::SingularHeapMessageField::<
        self::_puroro_root::full_coverage2::msg::Submsg,
    >,
    submsg_optional: self::_puroro::internal::field_type::SingularHeapMessageField::<
        self::_puroro_root::full_coverage2::msg::Submsg,
    >,
    submsg_repeated: self::_puroro::internal::field_type::RepeatedMessageField::<
        self::_puroro_root::full_coverage2::msg::Submsg,
    >,
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
impl Msg {
    pub fn i32_required(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::get_field(
            &self.i32_required,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn i32_optional(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            1usize,
        > as NonRepeatedFieldType>::get_field(
            &self.i32_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn i32_repeated(&self) -> &[i32] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            i32,
            self::_puroro::tags::Int32,
        > as RepeatedFieldType>::get_field(&self.i32_repeated, &self._bitfield)
    }
    pub fn float_required(&self) -> f32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            f32,
            self::_puroro::tags::Float,
            2usize,
        > as NonRepeatedFieldType>::get_field(
            &self.float_required,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn float_optional(&self) -> f32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            f32,
            self::_puroro::tags::Float,
            3usize,
        > as NonRepeatedFieldType>::get_field(
            &self.float_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn float_repeated(&self) -> &[f32] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            f32,
            self::_puroro::tags::Float,
        > as RepeatedFieldType>::get_field(&self.float_repeated, &self._bitfield)
    }
    pub fn bytes_required(&self) -> &[u8] {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalBytesField::<
            4usize,
        > as NonRepeatedFieldType>::get_field(
            &self.bytes_required,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn bytes_optional(&self) -> &[u8] {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalBytesField::<
            5usize,
        > as NonRepeatedFieldType>::get_field(
            &self.bytes_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn bytes_repeated(&self) -> &[impl ::std::ops::Deref::<Target = &[u8]>] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedBytesField as RepeatedFieldType>::get_field(
            &self.bytes_repeated,
            &self._bitfield,
        )
    }
    pub fn string_required(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            6usize,
        > as NonRepeatedFieldType>::get_field(
            &self.string_required,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn string_optional(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            7usize,
        > as NonRepeatedFieldType>::get_field(
            &self.string_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn string_repeated(&self) -> &[impl ::std::ops::Deref::<Target = &str>] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedStringField as RepeatedFieldType>::get_field(
            &self.string_repeated,
            &self._bitfield,
        )
    }
    pub fn enum_required(&self) -> self::_puroro_root::full_coverage2::Enum {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            self::_puroro_root::full_coverage2::Enum,
            self::_puroro::tags::Enum2::<self::_puroro_root::full_coverage2::Enum>,
            8usize,
        > as NonRepeatedFieldType>::get_field(
            &self.enum_required,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn enum_optional(&self) -> self::_puroro_root::full_coverage2::Enum {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            self::_puroro_root::full_coverage2::Enum,
            self::_puroro::tags::Enum2::<self::_puroro_root::full_coverage2::Enum>,
            9usize,
        > as NonRepeatedFieldType>::get_field(
            &self.enum_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn enum_repeated(&self) -> &[self::_puroro_root::full_coverage2::Enum] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            self::_puroro_root::full_coverage2::Enum,
            self::_puroro::tags::Enum2::<self::_puroro_root::full_coverage2::Enum>,
        > as RepeatedFieldType>::get_field(&self.enum_repeated, &self._bitfield)
    }
    pub fn submsg_required(
        &self,
    ) -> ::std::option::Option::<&self::_puroro_root::full_coverage2::msg::Submsg> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField::<
            self::_puroro_root::full_coverage2::msg::Submsg,
        > as NonRepeatedFieldType>::get_field(
            &self.submsg_required,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn submsg_optional(
        &self,
    ) -> ::std::option::Option::<&self::_puroro_root::full_coverage2::msg::Submsg> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField::<
            self::_puroro_root::full_coverage2::msg::Submsg,
        > as NonRepeatedFieldType>::get_field(
            &self.submsg_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn submsg_repeated(&self) -> &[self::_puroro_root::full_coverage2::msg::Submsg] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField::<
            self::_puroro_root::full_coverage2::msg::Submsg,
        > as RepeatedFieldType>::get_field(&self.submsg_repeated, &self._bitfield)
    }
    pub fn i64_required(&self) -> i64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i64,
            self::_puroro::tags::Int64,
            10usize,
        > as NonRepeatedFieldType>::get_field(
            &self.i64_required,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn i64_optional(&self) -> i64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i64,
            self::_puroro::tags::Int64,
            11usize,
        > as NonRepeatedFieldType>::get_field(
            &self.i64_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn i64_repeated(&self) -> &[i64] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            i64,
            self::_puroro::tags::Int64,
        > as RepeatedFieldType>::get_field(&self.i64_repeated, &self._bitfield)
    }
    pub fn u32_required(&self) -> u32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            u32,
            self::_puroro::tags::UInt32,
            12usize,
        > as NonRepeatedFieldType>::get_field(
            &self.u32_required,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn u32_optional(&self) -> u32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            u32,
            self::_puroro::tags::UInt32,
            13usize,
        > as NonRepeatedFieldType>::get_field(
            &self.u32_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn u32_repeated(&self) -> &[u32] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            u32,
            self::_puroro::tags::UInt32,
        > as RepeatedFieldType>::get_field(&self.u32_repeated, &self._bitfield)
    }
    pub fn u64_required(&self) -> u64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            u64,
            self::_puroro::tags::UInt64,
            14usize,
        > as NonRepeatedFieldType>::get_field(
            &self.u64_required,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn u64_optional(&self) -> u64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            u64,
            self::_puroro::tags::UInt64,
            15usize,
        > as NonRepeatedFieldType>::get_field(
            &self.u64_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn u64_repeated(&self) -> &[u64] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            u64,
            self::_puroro::tags::UInt64,
        > as RepeatedFieldType>::get_field(&self.u64_repeated, &self._bitfield)
    }
    pub fn s32_required(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::SInt32,
            16usize,
        > as NonRepeatedFieldType>::get_field(
            &self.s32_required,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn s32_optional(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::SInt32,
            17usize,
        > as NonRepeatedFieldType>::get_field(
            &self.s32_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn s32_repeated(&self) -> &[i32] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            i32,
            self::_puroro::tags::SInt32,
        > as RepeatedFieldType>::get_field(&self.s32_repeated, &self._bitfield)
    }
    pub fn s64_required(&self) -> i64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i64,
            self::_puroro::tags::SInt64,
            18usize,
        > as NonRepeatedFieldType>::get_field(
            &self.s64_required,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn s64_optional(&self) -> i64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i64,
            self::_puroro::tags::SInt64,
            19usize,
        > as NonRepeatedFieldType>::get_field(
            &self.s64_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn s64_repeated(&self) -> &[i64] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            i64,
            self::_puroro::tags::SInt64,
        > as RepeatedFieldType>::get_field(&self.s64_repeated, &self._bitfield)
    }
    pub fn fixed32_required(&self) -> u32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            u32,
            self::_puroro::tags::Fixed32,
            20usize,
        > as NonRepeatedFieldType>::get_field(
            &self.fixed32_required,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn fixed32_optional(&self) -> u32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            u32,
            self::_puroro::tags::Fixed32,
            21usize,
        > as NonRepeatedFieldType>::get_field(
            &self.fixed32_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn fixed32_repeated(&self) -> &[u32] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            u32,
            self::_puroro::tags::Fixed32,
        > as RepeatedFieldType>::get_field(&self.fixed32_repeated, &self._bitfield)
    }
    pub fn fixed64_required(&self) -> u64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            u64,
            self::_puroro::tags::Fixed64,
            22usize,
        > as NonRepeatedFieldType>::get_field(
            &self.fixed64_required,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn fixed64_optional(&self) -> u64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            u64,
            self::_puroro::tags::Fixed64,
            23usize,
        > as NonRepeatedFieldType>::get_field(
            &self.fixed64_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn fixed64_repeated(&self) -> &[u64] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            u64,
            self::_puroro::tags::Fixed64,
        > as RepeatedFieldType>::get_field(&self.fixed64_repeated, &self._bitfield)
    }
    pub fn sfixed32_required(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::SFixed32,
            24usize,
        > as NonRepeatedFieldType>::get_field(
            &self.sfixed32_required,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn sfixed32_optional(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::SFixed32,
            25usize,
        > as NonRepeatedFieldType>::get_field(
            &self.sfixed32_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn sfixed32_repeated(&self) -> &[i32] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            i32,
            self::_puroro::tags::SFixed32,
        > as RepeatedFieldType>::get_field(&self.sfixed32_repeated, &self._bitfield)
    }
    pub fn sfixed64_required(&self) -> i64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i64,
            self::_puroro::tags::SFixed64,
            26usize,
        > as NonRepeatedFieldType>::get_field(
            &self.sfixed64_required,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn sfixed64_optional(&self) -> i64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i64,
            self::_puroro::tags::SFixed64,
            27usize,
        > as NonRepeatedFieldType>::get_field(
            &self.sfixed64_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn sfixed64_repeated(&self) -> &[i64] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            i64,
            self::_puroro::tags::SFixed64,
        > as RepeatedFieldType>::get_field(&self.sfixed64_repeated, &self._bitfield)
    }
    pub fn f64_required(&self) -> f64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            f64,
            self::_puroro::tags::Double,
            28usize,
        > as NonRepeatedFieldType>::get_field(
            &self.f64_required,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn f64_optional(&self) -> f64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            f64,
            self::_puroro::tags::Double,
            29usize,
        > as NonRepeatedFieldType>::get_field(
            &self.f64_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn f64_repeated(&self) -> &[f64] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            f64,
            self::_puroro::tags::Double,
        > as RepeatedFieldType>::get_field(&self.f64_repeated, &self._bitfield)
    }
}
#[derive(
    ::std::clone::Clone,
    ::std::marker::Copy,
    ::std::cmp::PartialEq,
    ::std::cmp::Eq,
    ::std::cmp::PartialOrd,
    ::std::cmp::Ord,
    ::std::hash::Hash,
)]
pub enum Enum {
    Zeroth,
    First,
    Tenth,
}
impl ::std::default::Default for Enum {
    fn default() -> Self {
        Self::Zeroth
    }
}
impl ::std::convert::From::<Enum> for i32 {
    fn from(val: Enum) -> i32 {
        match val {
            Enum::Zeroth => 0i32,
            Enum::First => 1i32,
            Enum::Tenth => 10i32,
        }
    }
}
impl ::std::convert::TryFrom::<i32> for Enum {
    type Error = self::_puroro::PuroroError;
    fn try_from(val: i32) -> ::std::result::Result<Self, Self::Error> {
        use ::std::result::Result::{Ok, Err};
        match val {
            0i32 => Ok(self::Enum::Zeroth),
            1i32 => Ok(self::Enum::First),
            10i32 => Ok(self::Enum::Tenth),
            _ => Err(self::_puroro::ErrorKind::UnknownEnumVariant(e))?,
        }
    }
}
