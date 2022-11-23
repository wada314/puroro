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
        self::_puroro_root::full_coverage3::msg::Submsg,
    >,
    submsg_optional: self::_puroro::internal::field_type::SingularHeapMessageField::<
        self::_puroro_root::full_coverage3::msg::Submsg,
    >,
    submsg_repeated: self::_puroro::internal::field_type::RepeatedMessageField::<
        self::_puroro_root::full_coverage3::msg::Submsg,
    >,
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
impl Msg {
    pub fn i32_unlabeled(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            i32,
            self::_puroro::tags::Int32,
        > as NonRepeatedFieldType>::get_field(
            &self.i32_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn i32_optional(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            0usize,
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
    pub fn float_unlabeled(&self) -> f32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            f32,
            self::_puroro::tags::Float,
        > as NonRepeatedFieldType>::get_field(
            &self.float_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn float_optional(&self) -> f32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            f32,
            self::_puroro::tags::Float,
            1usize,
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
    pub fn bytes_unlabeled(&self) -> &[u8] {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularBytesField as NonRepeatedFieldType>::get_field(
            &self.bytes_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn bytes_optional(&self) -> &[u8] {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalBytesField::<
            2usize,
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
    pub fn string_unlabeled(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularStringField as NonRepeatedFieldType>::get_field(
            &self.string_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn string_optional(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            3usize,
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
    pub fn enum_unlabeled(&self) -> self::_puroro_root::full_coverage3::Enum {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            self::_puroro_root::full_coverage3::Enum,
            self::_puroro::tags::Enum3::<self::_puroro_root::full_coverage3::Enum>,
        > as NonRepeatedFieldType>::get_field(
            &self.enum_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn enum_optional(&self) -> self::_puroro_root::full_coverage3::Enum {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            self::_puroro_root::full_coverage3::Enum,
            self::_puroro::tags::Enum3::<self::_puroro_root::full_coverage3::Enum>,
            4usize,
        > as NonRepeatedFieldType>::get_field(
            &self.enum_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn enum_repeated(&self) -> &[self::_puroro_root::full_coverage3::Enum] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            self::_puroro_root::full_coverage3::Enum,
            self::_puroro::tags::Enum3::<self::_puroro_root::full_coverage3::Enum>,
        > as RepeatedFieldType>::get_field(&self.enum_repeated, &self._bitfield)
    }
    pub fn submsg_unlabeled(
        &self,
    ) -> ::std::option::Option::<&self::_puroro_root::full_coverage3::msg::Submsg> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField::<
            self::_puroro_root::full_coverage3::msg::Submsg,
        > as NonRepeatedFieldType>::get_field(
            &self.submsg_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn submsg_optional(
        &self,
    ) -> ::std::option::Option::<&self::_puroro_root::full_coverage3::msg::Submsg> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField::<
            self::_puroro_root::full_coverage3::msg::Submsg,
        > as NonRepeatedFieldType>::get_field(
            &self.submsg_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn submsg_repeated(&self) -> &[self::_puroro_root::full_coverage3::msg::Submsg] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField::<
            self::_puroro_root::full_coverage3::msg::Submsg,
        > as RepeatedFieldType>::get_field(&self.submsg_repeated, &self._bitfield)
    }
    pub fn i64_unlabeled(&self) -> i64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            i64,
            self::_puroro::tags::Int64,
        > as NonRepeatedFieldType>::get_field(
            &self.i64_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn i64_optional(&self) -> i64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i64,
            self::_puroro::tags::Int64,
            5usize,
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
    pub fn u32_unlabeled(&self) -> u32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            u32,
            self::_puroro::tags::UInt32,
        > as NonRepeatedFieldType>::get_field(
            &self.u32_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn u32_optional(&self) -> u32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            u32,
            self::_puroro::tags::UInt32,
            6usize,
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
    pub fn u64_unlabeled(&self) -> u64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            u64,
            self::_puroro::tags::UInt64,
        > as NonRepeatedFieldType>::get_field(
            &self.u64_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn u64_optional(&self) -> u64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            u64,
            self::_puroro::tags::UInt64,
            7usize,
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
    pub fn s32_unlabeled(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            i32,
            self::_puroro::tags::SInt32,
        > as NonRepeatedFieldType>::get_field(
            &self.s32_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn s32_optional(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::SInt32,
            8usize,
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
    pub fn s64_unlabeled(&self) -> i64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            i64,
            self::_puroro::tags::SInt64,
        > as NonRepeatedFieldType>::get_field(
            &self.s64_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn s64_optional(&self) -> i64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i64,
            self::_puroro::tags::SInt64,
            9usize,
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
    pub fn fixed32_unlabeled(&self) -> u32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            u32,
            self::_puroro::tags::Fixed32,
        > as NonRepeatedFieldType>::get_field(
            &self.fixed32_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn fixed32_optional(&self) -> u32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            u32,
            self::_puroro::tags::Fixed32,
            10usize,
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
    pub fn fixed64_unlabeled(&self) -> u64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            u64,
            self::_puroro::tags::Fixed64,
        > as NonRepeatedFieldType>::get_field(
            &self.fixed64_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn fixed64_optional(&self) -> u64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            u64,
            self::_puroro::tags::Fixed64,
            11usize,
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
    pub fn sfixed32_unlabeled(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            i32,
            self::_puroro::tags::SFixed32,
        > as NonRepeatedFieldType>::get_field(
            &self.sfixed32_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn sfixed32_optional(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::SFixed32,
            12usize,
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
    pub fn sfixed64_unlabeled(&self) -> i64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            i64,
            self::_puroro::tags::SFixed64,
        > as NonRepeatedFieldType>::get_field(
            &self.sfixed64_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn sfixed64_optional(&self) -> i64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i64,
            self::_puroro::tags::SFixed64,
            13usize,
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
    pub fn f64_unlabeled(&self) -> f64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            f64,
            self::_puroro::tags::Double,
        > as NonRepeatedFieldType>::get_field(
            &self.f64_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn f64_optional(&self) -> f64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            f64,
            self::_puroro::tags::Double,
            14usize,
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
    _None(i32),
}
impl ::std::default::Default for Enum {
    fn default() -> Self {
        Self::Zeroth
    }
}
