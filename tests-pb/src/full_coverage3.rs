// A generated source code by puroro library
// package full_coverage3

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub mod _puroro {
    pub use ::puroro::*;
}

#[derive(Default)]
pub struct Msg {
    // Singular, Variant(Int32)
    i32_unlabeled: self::_puroro::internal::field_type::SingularNumericalField<
        i32,
        self::_puroro::tags::Int32,
    >,
    // Optional, Variant(Int32)
    i32_optional: self::_puroro::internal::field_type::OptionalNumericalField<
        i32,
        self::_puroro::tags::Int32,
        0,
    >,
    // Repeated, Variant(Int32)
    i32_repeated: self::_puroro::internal::field_type::RepeatedNumericalField<
        i32,
        self::_puroro::tags::Int32,
    >,
    // Singular, Bits32(Float)
    float_unlabeled: self::_puroro::internal::field_type::SingularNumericalField<
        f32,
        self::_puroro::tags::Float,
    >,
    // Optional, Bits32(Float)
    float_optional: self::_puroro::internal::field_type::OptionalNumericalField<
        f32,
        self::_puroro::tags::Float,
        1,
    >,
    // Repeated, Bits32(Float)
    float_repeated: self::_puroro::internal::field_type::RepeatedNumericalField<
        f32,
        self::_puroro::tags::Float,
    >,
    // Singular, LengthDelimited(Bytes)
    bytes_unlabeled: self::_puroro::internal::field_type::SingularBytesField,
    // Optional, LengthDelimited(Bytes)
    bytes_optional: self::_puroro::internal::field_type::OptionalBytesField<2>,
    // Repeated, LengthDelimited(Bytes)
    bytes_repeated: self::_puroro::internal::field_type::RepeatedBytesField,
    // Singular, LengthDelimited(String)
    string_unlabeled: self::_puroro::internal::field_type::SingularStringField,
    // Optional, LengthDelimited(String)
    string_optional: self::_puroro::internal::field_type::OptionalStringField<3>,
    // Repeated, LengthDelimited(String)
    string_repeated: self::_puroro::internal::field_type::RepeatedStringField,
    // Singular, Variant(Enum3(Fqtn(".full_coverage3.Enum")))
    enum_unlabeled: self::_puroro::internal::field_type::SingularNumericalField<
        _puroro_root::full_coverage3::Enum,
        self::_puroro::tags::Enum3<_puroro_root::full_coverage3::Enum>,
    >,
    // Optional, Variant(Enum3(Fqtn(".full_coverage3.Enum")))
    enum_optional: self::_puroro::internal::field_type::OptionalNumericalField<
        _puroro_root::full_coverage3::Enum,
        self::_puroro::tags::Enum3<_puroro_root::full_coverage3::Enum>,
        4,
    >,
    // Repeated, Variant(Enum3(Fqtn(".full_coverage3.Enum")))
    enum_repeated: self::_puroro::internal::field_type::RepeatedNumericalField<
        _puroro_root::full_coverage3::Enum,
        self::_puroro::tags::Enum3<_puroro_root::full_coverage3::Enum>,
    >,
    // Singular, LengthDelimited(Message(Fqtn(".full_coverage3.Msg.Submsg")))
    submsg_unlabeled: self::_puroro::internal::field_type::SingularHeapMessageField<
        _puroro_root::full_coverage3::msg::Submsg,
    >,
    // Optional, LengthDelimited(Message(Fqtn(".full_coverage3.Msg.Submsg")))
    submsg_optional: self::_puroro::internal::field_type::SingularHeapMessageField<
        _puroro_root::full_coverage3::msg::Submsg,
    >,
    // Repeated, LengthDelimited(Message(Fqtn(".full_coverage3.Msg.Submsg")))
    submsg_repeated: self::_puroro::internal::field_type::RepeatedMessageField<
        _puroro_root::full_coverage3::msg::Submsg,
    >,
    // Singular, Variant(Int64)
    i64_unlabeled: self::_puroro::internal::field_type::SingularNumericalField<
        i64,
        self::_puroro::tags::Int64,
    >,
    // Optional, Variant(Int64)
    i64_optional: self::_puroro::internal::field_type::OptionalNumericalField<
        i64,
        self::_puroro::tags::Int64,
        6,
    >,
    // Repeated, Variant(Int64)
    i64_repeated: self::_puroro::internal::field_type::RepeatedNumericalField<
        i64,
        self::_puroro::tags::Int64,
    >,
    // Singular, Variant(UInt32)
    u32_unlabeled: self::_puroro::internal::field_type::SingularNumericalField<
        u32,
        self::_puroro::tags::UInt32,
    >,
    // Optional, Variant(UInt32)
    u32_optional: self::_puroro::internal::field_type::OptionalNumericalField<
        u32,
        self::_puroro::tags::UInt32,
        7,
    >,
    // Repeated, Variant(UInt32)
    u32_repeated: self::_puroro::internal::field_type::RepeatedNumericalField<
        u32,
        self::_puroro::tags::UInt32,
    >,
    // Singular, Variant(UInt64)
    u64_unlabeled: self::_puroro::internal::field_type::SingularNumericalField<
        u64,
        self::_puroro::tags::UInt64,
    >,
    // Optional, Variant(UInt64)
    u64_optional: self::_puroro::internal::field_type::OptionalNumericalField<
        u64,
        self::_puroro::tags::UInt64,
        8,
    >,
    // Repeated, Variant(UInt64)
    u64_repeated: self::_puroro::internal::field_type::RepeatedNumericalField<
        u64,
        self::_puroro::tags::UInt64,
    >,
    // Singular, Variant(SInt32)
    s32_unlabeled: self::_puroro::internal::field_type::SingularNumericalField<
        i32,
        self::_puroro::tags::SInt32,
    >,
    // Optional, Variant(SInt32)
    s32_optional: self::_puroro::internal::field_type::OptionalNumericalField<
        i32,
        self::_puroro::tags::SInt32,
        9,
    >,
    // Repeated, Variant(SInt32)
    s32_repeated: self::_puroro::internal::field_type::RepeatedNumericalField<
        i32,
        self::_puroro::tags::SInt32,
    >,
    // Singular, Variant(SInt64)
    s64_unlabeled: self::_puroro::internal::field_type::SingularNumericalField<
        i64,
        self::_puroro::tags::SInt64,
    >,
    // Optional, Variant(SInt64)
    s64_optional: self::_puroro::internal::field_type::OptionalNumericalField<
        i64,
        self::_puroro::tags::SInt64,
        10,
    >,
    // Repeated, Variant(SInt64)
    s64_repeated: self::_puroro::internal::field_type::RepeatedNumericalField<
        i64,
        self::_puroro::tags::SInt64,
    >,
    // Singular, Bits32(Fixed32)
    fixed32_unlabeled: self::_puroro::internal::field_type::SingularNumericalField<
        u32,
        self::_puroro::tags::Fixed32,
    >,
    // Optional, Bits32(Fixed32)
    fixed32_optional: self::_puroro::internal::field_type::OptionalNumericalField<
        u32,
        self::_puroro::tags::Fixed32,
        11,
    >,
    // Repeated, Bits32(Fixed32)
    fixed32_repeated: self::_puroro::internal::field_type::RepeatedNumericalField<
        u32,
        self::_puroro::tags::Fixed32,
    >,
    // Singular, Bits64(Fixed64)
    fixed64_unlabeled: self::_puroro::internal::field_type::SingularNumericalField<
        u64,
        self::_puroro::tags::Fixed64,
    >,
    // Optional, Bits64(Fixed64)
    fixed64_optional: self::_puroro::internal::field_type::OptionalNumericalField<
        u64,
        self::_puroro::tags::Fixed64,
        12,
    >,
    // Repeated, Bits64(Fixed64)
    fixed64_repeated: self::_puroro::internal::field_type::RepeatedNumericalField<
        u64,
        self::_puroro::tags::Fixed64,
    >,
    // Singular, Bits32(SFixed32)
    sfixed32_unlabeled: self::_puroro::internal::field_type::SingularNumericalField<
        i32,
        self::_puroro::tags::SFixed32,
    >,
    // Optional, Bits32(SFixed32)
    sfixed32_optional: self::_puroro::internal::field_type::OptionalNumericalField<
        i32,
        self::_puroro::tags::SFixed32,
        13,
    >,
    // Repeated, Bits32(SFixed32)
    sfixed32_repeated: self::_puroro::internal::field_type::RepeatedNumericalField<
        i32,
        self::_puroro::tags::SFixed32,
    >,
    // Singular, Bits64(SFixed64)
    sfixed64_unlabeled: self::_puroro::internal::field_type::SingularNumericalField<
        i64,
        self::_puroro::tags::SFixed64,
    >,
    // Optional, Bits64(SFixed64)
    sfixed64_optional: self::_puroro::internal::field_type::OptionalNumericalField<
        i64,
        self::_puroro::tags::SFixed64,
        14,
    >,
    // Repeated, Bits64(SFixed64)
    sfixed64_repeated: self::_puroro::internal::field_type::RepeatedNumericalField<
        i64,
        self::_puroro::tags::SFixed64,
    >,
    // Singular, Bits64(Double)
    f64_unlabeled: self::_puroro::internal::field_type::SingularNumericalField<
        f64,
        self::_puroro::tags::Double,
    >,
    // Optional, Bits64(Double)
    f64_optional: self::_puroro::internal::field_type::OptionalNumericalField<
        f64,
        self::_puroro::tags::Double,
        15,
    >,
    // Repeated, Bits64(Double)
    f64_repeated: self::_puroro::internal::field_type::RepeatedNumericalField<
        f64,
        self::_puroro::tags::Double,
    >,

    _bitfield: self::_puroro::bitvec::BitArray<2>,
}

impl Msg {
    // Singular, Variant(Int32)
    pub fn i32_unlabeled(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<i32, self::_puroro::tags::Int32> as NonRepeatedFieldType>::get_field(
            &self.i32_unlabeled, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn i32_unlabeled_opt(&self) -> ::std::option::Option<i32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<i32, self::_puroro::tags::Int32> as NonRepeatedFieldType>::get_field_opt(
            &self.i32_unlabeled, &self._bitfield,
        )
    }
    pub fn has_i32_unlabeled(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<i32, self::_puroro::tags::Int32> as NonRepeatedFieldType>::get_field_opt(
            &self.i32_unlabeled, &self._bitfield,
        ).is_some()
    }
    pub fn i32_unlabeled_mut(&mut self) -> &mut i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<i32, self::_puroro::tags::Int32> as NonRepeatedFieldType>::mut_field(
            &mut self.i32_unlabeled, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_i32_unlabeled(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<i32, self::_puroro::tags::Int32> as NonRepeatedFieldType>::clear(
            &mut self.i32_unlabeled, &mut self._bitfield,
        )
    }
    // Optional, Variant(Int32)
    pub fn i32_optional(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
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
            0,
        > as NonRepeatedFieldType>::get_field_opt(&self.i32_optional, &self._bitfield)
    }
    pub fn has_i32_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as NonRepeatedFieldType>::get_field_opt(&self.i32_optional, &self._bitfield)
        .is_some()
    }
    pub fn i32_optional_mut(&mut self) -> &mut i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
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
            0,
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
    // Singular, Bits32(Float)
    pub fn float_unlabeled(&self) -> f32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<f32, self::_puroro::tags::Float> as NonRepeatedFieldType>::get_field(
            &self.float_unlabeled, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn float_unlabeled_opt(&self) -> ::std::option::Option<f32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<f32, self::_puroro::tags::Float> as NonRepeatedFieldType>::get_field_opt(
            &self.float_unlabeled, &self._bitfield,
        )
    }
    pub fn has_float_unlabeled(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<f32, self::_puroro::tags::Float> as NonRepeatedFieldType>::get_field_opt(
            &self.float_unlabeled, &self._bitfield,
        ).is_some()
    }
    pub fn float_unlabeled_mut(&mut self) -> &mut f32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<f32, self::_puroro::tags::Float> as NonRepeatedFieldType>::mut_field(
            &mut self.float_unlabeled, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_float_unlabeled(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<f32, self::_puroro::tags::Float> as NonRepeatedFieldType>::clear(
            &mut self.float_unlabeled, &mut self._bitfield,
        )
    }
    // Optional, Bits32(Float)
    pub fn float_optional(&self) -> f32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            f32,
            self::_puroro::tags::Float,
            1,
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
            1,
        > as NonRepeatedFieldType>::get_field_opt(&self.float_optional, &self._bitfield)
    }
    pub fn has_float_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            f32,
            self::_puroro::tags::Float,
            1,
        > as NonRepeatedFieldType>::get_field_opt(&self.float_optional, &self._bitfield)
        .is_some()
    }
    pub fn float_optional_mut(&mut self) -> &mut f32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            f32,
            self::_puroro::tags::Float,
            1,
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
            1,
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
    // Singular, LengthDelimited(Bytes)
    pub fn bytes_unlabeled(&self) -> &[u8] {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularBytesField as NonRepeatedFieldType>::get_field(
            &self.bytes_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn bytes_unlabeled_opt(&self) -> ::std::option::Option<&[u8]> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularBytesField as NonRepeatedFieldType>::get_field_opt(
            &self.bytes_unlabeled, &self._bitfield,
        )
    }
    pub fn has_bytes_unlabeled(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularBytesField as NonRepeatedFieldType>::get_field_opt(
            &self.bytes_unlabeled, &self._bitfield,
        ).is_some()
    }
    pub fn bytes_unlabeled_mut(&mut self) -> &mut ::std::vec::Vec<u8> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularBytesField as NonRepeatedFieldType>::mut_field(
            &mut self.bytes_unlabeled,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_bytes_unlabeled(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularBytesField as NonRepeatedFieldType>::clear(
            &mut self.bytes_unlabeled,
            &mut self._bitfield,
        )
    }
    // Optional, LengthDelimited(Bytes)
    pub fn bytes_optional(&self) -> &[u8] {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalBytesField<2> as NonRepeatedFieldType>::get_field(
            &self.bytes_optional, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn bytes_optional_opt(&self) -> ::std::option::Option<&[u8]> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalBytesField<2> as NonRepeatedFieldType>::get_field_opt(
            &self.bytes_optional, &self._bitfield,
        )
    }
    pub fn has_bytes_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalBytesField<2> as NonRepeatedFieldType>::get_field_opt(
            &self.bytes_optional, &self._bitfield,
        ).is_some()
    }
    pub fn bytes_optional_mut(&mut self) -> &mut ::std::vec::Vec<u8> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalBytesField<2> as NonRepeatedFieldType>::mut_field(
            &mut self.bytes_optional, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_bytes_optional(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalBytesField<2> as NonRepeatedFieldType>::clear(
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
    // Singular, LengthDelimited(String)
    pub fn string_unlabeled(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularStringField as NonRepeatedFieldType>::get_field(
            &self.string_unlabeled, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn string_unlabeled_opt(&self) -> ::std::option::Option<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularStringField as NonRepeatedFieldType>::get_field_opt(
            &self.string_unlabeled, &self._bitfield,
        )
    }
    pub fn has_string_unlabeled(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularStringField as NonRepeatedFieldType>::get_field_opt(
            &self.string_unlabeled, &self._bitfield,
        ).is_some()
    }
    pub fn string_unlabeled_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularStringField as NonRepeatedFieldType>::mut_field(
            &mut self.string_unlabeled, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_string_unlabeled(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularStringField as NonRepeatedFieldType>::clear(
            &mut self.string_unlabeled,
            &mut self._bitfield,
        )
    }
    // Optional, LengthDelimited(String)
    pub fn string_optional(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<3> as NonRepeatedFieldType>::get_field(
            &self.string_optional, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn string_optional_opt(&self) -> ::std::option::Option<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<3> as NonRepeatedFieldType>::get_field_opt(
            &self.string_optional, &self._bitfield,
        )
    }
    pub fn has_string_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<3> as NonRepeatedFieldType>::get_field_opt(
            &self.string_optional, &self._bitfield,
        ).is_some()
    }
    pub fn string_optional_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<3> as NonRepeatedFieldType>::mut_field(
            &mut self.string_optional, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_string_optional(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField<3> as NonRepeatedFieldType>::clear(
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
    // Singular, Variant(Enum3(Fqtn(".full_coverage3.Enum")))
    pub fn enum_unlabeled(&self) -> _puroro_root::full_coverage3::Enum {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            _puroro_root::full_coverage3::Enum,
            self::_puroro::tags::Enum3<_puroro_root::full_coverage3::Enum>,
        > as NonRepeatedFieldType>::get_field(
            &self.enum_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn enum_unlabeled_opt(&self) -> ::std::option::Option<_puroro_root::full_coverage3::Enum> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            _puroro_root::full_coverage3::Enum,
            self::_puroro::tags::Enum3<_puroro_root::full_coverage3::Enum>,
        > as NonRepeatedFieldType>::get_field_opt(&self.enum_unlabeled, &self._bitfield)
    }
    pub fn has_enum_unlabeled(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            _puroro_root::full_coverage3::Enum,
            self::_puroro::tags::Enum3<_puroro_root::full_coverage3::Enum>,
        > as NonRepeatedFieldType>::get_field_opt(&self.enum_unlabeled, &self._bitfield)
        .is_some()
    }
    pub fn enum_unlabeled_mut(&mut self) -> &mut _puroro_root::full_coverage3::Enum {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            _puroro_root::full_coverage3::Enum,
            self::_puroro::tags::Enum3<_puroro_root::full_coverage3::Enum>,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.enum_unlabeled,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_enum_unlabeled(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            _puroro_root::full_coverage3::Enum,
            self::_puroro::tags::Enum3<_puroro_root::full_coverage3::Enum>,
        > as NonRepeatedFieldType>::clear(&mut self.enum_unlabeled, &mut self._bitfield)
    }
    // Optional, Variant(Enum3(Fqtn(".full_coverage3.Enum")))
    pub fn enum_optional(&self) -> _puroro_root::full_coverage3::Enum {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::full_coverage3::Enum,
            self::_puroro::tags::Enum3<_puroro_root::full_coverage3::Enum>,
            4,
        > as NonRepeatedFieldType>::get_field(
            &self.enum_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn enum_optional_opt(&self) -> ::std::option::Option<_puroro_root::full_coverage3::Enum> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::full_coverage3::Enum,
            self::_puroro::tags::Enum3<_puroro_root::full_coverage3::Enum>,
            4,
        > as NonRepeatedFieldType>::get_field_opt(&self.enum_optional, &self._bitfield)
    }
    pub fn has_enum_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::full_coverage3::Enum,
            self::_puroro::tags::Enum3<_puroro_root::full_coverage3::Enum>,
            4,
        > as NonRepeatedFieldType>::get_field_opt(&self.enum_optional, &self._bitfield)
        .is_some()
    }
    pub fn enum_optional_mut(&mut self) -> &mut _puroro_root::full_coverage3::Enum {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::full_coverage3::Enum,
            self::_puroro::tags::Enum3<_puroro_root::full_coverage3::Enum>,
            4,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.enum_optional,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_enum_optional(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::full_coverage3::Enum,
            self::_puroro::tags::Enum3<_puroro_root::full_coverage3::Enum>,
            4,
        > as NonRepeatedFieldType>::clear(&mut self.enum_optional, &mut self._bitfield)
    }
    // Repeated, Variant(Enum3(Fqtn(".full_coverage3.Enum")))
    pub fn enum_repeated(&self) -> &[_puroro_root::full_coverage3::Enum] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            _puroro_root::full_coverage3::Enum,
            self::_puroro::tags::Enum3<_puroro_root::full_coverage3::Enum>,
        > as RepeatedFieldType>::get_field(&self.enum_repeated, &self._bitfield)
    }
    pub fn enum_repeated_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec<_puroro_root::full_coverage3::Enum> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            _puroro_root::full_coverage3::Enum,
            self::_puroro::tags::Enum3<_puroro_root::full_coverage3::Enum>,
        > as RepeatedFieldType>::mut_field(&mut self.enum_repeated, &mut self._bitfield)
    }
    pub fn clear_enum_repeated(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            _puroro_root::full_coverage3::Enum,
            self::_puroro::tags::Enum3<_puroro_root::full_coverage3::Enum>,
        > as RepeatedFieldType>::clear(&mut self.enum_repeated, &mut self._bitfield)
    }
    // Singular, LengthDelimited(Message(Fqtn(".full_coverage3.Msg.Submsg")))
    pub fn submsg_unlabeled(
        &self,
    ) -> ::std::option::Option<&_puroro_root::full_coverage3::msg::Submsg> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::full_coverage3::msg::Submsg,
        > as NonRepeatedFieldType>::get_field(
            &self.submsg_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn submsg_unlabeled_opt(
        &self,
    ) -> ::std::option::Option<&_puroro_root::full_coverage3::msg::Submsg> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::full_coverage3::msg::Submsg,
        > as NonRepeatedFieldType>::get_field_opt(&self.submsg_unlabeled, &self._bitfield)
    }
    pub fn has_submsg_unlabeled(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::full_coverage3::msg::Submsg,
        > as NonRepeatedFieldType>::get_field_opt(&self.submsg_unlabeled, &self._bitfield)
        .is_some()
    }
    pub fn submsg_unlabeled_mut(&mut self) -> &mut _puroro_root::full_coverage3::msg::Submsg {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::full_coverage3::msg::Submsg,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.submsg_unlabeled,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_submsg_unlabeled(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::full_coverage3::msg::Submsg,
        > as NonRepeatedFieldType>::clear(&mut self.submsg_unlabeled, &mut self._bitfield)
    }
    // Optional, LengthDelimited(Message(Fqtn(".full_coverage3.Msg.Submsg")))
    pub fn submsg_optional(
        &self,
    ) -> ::std::option::Option<&_puroro_root::full_coverage3::msg::Submsg> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::full_coverage3::msg::Submsg,
        > as NonRepeatedFieldType>::get_field(
            &self.submsg_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn submsg_optional_opt(
        &self,
    ) -> ::std::option::Option<&_puroro_root::full_coverage3::msg::Submsg> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::full_coverage3::msg::Submsg,
        > as NonRepeatedFieldType>::get_field_opt(&self.submsg_optional, &self._bitfield)
    }
    pub fn has_submsg_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::full_coverage3::msg::Submsg,
        > as NonRepeatedFieldType>::get_field_opt(&self.submsg_optional, &self._bitfield)
        .is_some()
    }
    pub fn submsg_optional_mut(&mut self) -> &mut _puroro_root::full_coverage3::msg::Submsg {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::full_coverage3::msg::Submsg,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.submsg_optional,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_submsg_optional(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::full_coverage3::msg::Submsg,
        > as NonRepeatedFieldType>::clear(&mut self.submsg_optional, &mut self._bitfield)
    }
    // Repeated, LengthDelimited(Message(Fqtn(".full_coverage3.Msg.Submsg")))
    pub fn submsg_repeated(&self) -> &[_puroro_root::full_coverage3::msg::Submsg] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::full_coverage3::msg::Submsg,
        > as RepeatedFieldType>::get_field(&self.submsg_repeated, &self._bitfield)
    }
    pub fn submsg_repeated_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec<_puroro_root::full_coverage3::msg::Submsg> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::full_coverage3::msg::Submsg,
        > as RepeatedFieldType>::mut_field(&mut self.submsg_repeated, &mut self._bitfield)
    }
    pub fn clear_submsg_repeated(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::full_coverage3::msg::Submsg,
        > as RepeatedFieldType>::clear(&mut self.submsg_repeated, &mut self._bitfield)
    }
    // Singular, Variant(Int64)
    pub fn i64_unlabeled(&self) -> i64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<i64, self::_puroro::tags::Int64> as NonRepeatedFieldType>::get_field(
            &self.i64_unlabeled, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn i64_unlabeled_opt(&self) -> ::std::option::Option<i64> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<i64, self::_puroro::tags::Int64> as NonRepeatedFieldType>::get_field_opt(
            &self.i64_unlabeled, &self._bitfield,
        )
    }
    pub fn has_i64_unlabeled(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<i64, self::_puroro::tags::Int64> as NonRepeatedFieldType>::get_field_opt(
            &self.i64_unlabeled, &self._bitfield,
        ).is_some()
    }
    pub fn i64_unlabeled_mut(&mut self) -> &mut i64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<i64, self::_puroro::tags::Int64> as NonRepeatedFieldType>::mut_field(
            &mut self.i64_unlabeled, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_i64_unlabeled(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<i64, self::_puroro::tags::Int64> as NonRepeatedFieldType>::clear(
            &mut self.i64_unlabeled, &mut self._bitfield,
        )
    }
    // Optional, Variant(Int64)
    pub fn i64_optional(&self) -> i64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::Int64,
            6,
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
            6,
        > as NonRepeatedFieldType>::get_field_opt(&self.i64_optional, &self._bitfield)
    }
    pub fn has_i64_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::Int64,
            6,
        > as NonRepeatedFieldType>::get_field_opt(&self.i64_optional, &self._bitfield)
        .is_some()
    }
    pub fn i64_optional_mut(&mut self) -> &mut i64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::Int64,
            6,
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
            6,
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
    // Singular, Variant(UInt32)
    pub fn u32_unlabeled(&self) -> u32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            u32,
            self::_puroro::tags::UInt32,
        > as NonRepeatedFieldType>::get_field(
            &self.u32_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn u32_unlabeled_opt(&self) -> ::std::option::Option<u32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            u32,
            self::_puroro::tags::UInt32,
        > as NonRepeatedFieldType>::get_field_opt(&self.u32_unlabeled, &self._bitfield)
    }
    pub fn has_u32_unlabeled(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            u32,
            self::_puroro::tags::UInt32,
        > as NonRepeatedFieldType>::get_field_opt(&self.u32_unlabeled, &self._bitfield)
        .is_some()
    }
    pub fn u32_unlabeled_mut(&mut self) -> &mut u32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            u32,
            self::_puroro::tags::UInt32,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.u32_unlabeled,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_u32_unlabeled(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            u32,
            self::_puroro::tags::UInt32,
        > as NonRepeatedFieldType>::clear(&mut self.u32_unlabeled, &mut self._bitfield)
    }
    // Optional, Variant(UInt32)
    pub fn u32_optional(&self) -> u32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u32,
            self::_puroro::tags::UInt32,
            7,
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
            7,
        > as NonRepeatedFieldType>::get_field_opt(&self.u32_optional, &self._bitfield)
    }
    pub fn has_u32_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u32,
            self::_puroro::tags::UInt32,
            7,
        > as NonRepeatedFieldType>::get_field_opt(&self.u32_optional, &self._bitfield)
        .is_some()
    }
    pub fn u32_optional_mut(&mut self) -> &mut u32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u32,
            self::_puroro::tags::UInt32,
            7,
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
            7,
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
    // Singular, Variant(UInt64)
    pub fn u64_unlabeled(&self) -> u64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            u64,
            self::_puroro::tags::UInt64,
        > as NonRepeatedFieldType>::get_field(
            &self.u64_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn u64_unlabeled_opt(&self) -> ::std::option::Option<u64> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            u64,
            self::_puroro::tags::UInt64,
        > as NonRepeatedFieldType>::get_field_opt(&self.u64_unlabeled, &self._bitfield)
    }
    pub fn has_u64_unlabeled(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            u64,
            self::_puroro::tags::UInt64,
        > as NonRepeatedFieldType>::get_field_opt(&self.u64_unlabeled, &self._bitfield)
        .is_some()
    }
    pub fn u64_unlabeled_mut(&mut self) -> &mut u64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            u64,
            self::_puroro::tags::UInt64,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.u64_unlabeled,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_u64_unlabeled(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            u64,
            self::_puroro::tags::UInt64,
        > as NonRepeatedFieldType>::clear(&mut self.u64_unlabeled, &mut self._bitfield)
    }
    // Optional, Variant(UInt64)
    pub fn u64_optional(&self) -> u64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u64,
            self::_puroro::tags::UInt64,
            8,
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
            8,
        > as NonRepeatedFieldType>::get_field_opt(&self.u64_optional, &self._bitfield)
    }
    pub fn has_u64_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u64,
            self::_puroro::tags::UInt64,
            8,
        > as NonRepeatedFieldType>::get_field_opt(&self.u64_optional, &self._bitfield)
        .is_some()
    }
    pub fn u64_optional_mut(&mut self) -> &mut u64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u64,
            self::_puroro::tags::UInt64,
            8,
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
            8,
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
    // Singular, Variant(SInt32)
    pub fn s32_unlabeled(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            i32,
            self::_puroro::tags::SInt32,
        > as NonRepeatedFieldType>::get_field(
            &self.s32_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn s32_unlabeled_opt(&self) -> ::std::option::Option<i32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            i32,
            self::_puroro::tags::SInt32,
        > as NonRepeatedFieldType>::get_field_opt(&self.s32_unlabeled, &self._bitfield)
    }
    pub fn has_s32_unlabeled(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            i32,
            self::_puroro::tags::SInt32,
        > as NonRepeatedFieldType>::get_field_opt(&self.s32_unlabeled, &self._bitfield)
        .is_some()
    }
    pub fn s32_unlabeled_mut(&mut self) -> &mut i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            i32,
            self::_puroro::tags::SInt32,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.s32_unlabeled,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_s32_unlabeled(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            i32,
            self::_puroro::tags::SInt32,
        > as NonRepeatedFieldType>::clear(&mut self.s32_unlabeled, &mut self._bitfield)
    }
    // Optional, Variant(SInt32)
    pub fn s32_optional(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::SInt32,
            9,
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
            9,
        > as NonRepeatedFieldType>::get_field_opt(&self.s32_optional, &self._bitfield)
    }
    pub fn has_s32_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::SInt32,
            9,
        > as NonRepeatedFieldType>::get_field_opt(&self.s32_optional, &self._bitfield)
        .is_some()
    }
    pub fn s32_optional_mut(&mut self) -> &mut i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::SInt32,
            9,
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
            9,
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
    // Singular, Variant(SInt64)
    pub fn s64_unlabeled(&self) -> i64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            i64,
            self::_puroro::tags::SInt64,
        > as NonRepeatedFieldType>::get_field(
            &self.s64_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn s64_unlabeled_opt(&self) -> ::std::option::Option<i64> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            i64,
            self::_puroro::tags::SInt64,
        > as NonRepeatedFieldType>::get_field_opt(&self.s64_unlabeled, &self._bitfield)
    }
    pub fn has_s64_unlabeled(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            i64,
            self::_puroro::tags::SInt64,
        > as NonRepeatedFieldType>::get_field_opt(&self.s64_unlabeled, &self._bitfield)
        .is_some()
    }
    pub fn s64_unlabeled_mut(&mut self) -> &mut i64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            i64,
            self::_puroro::tags::SInt64,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.s64_unlabeled,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_s64_unlabeled(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            i64,
            self::_puroro::tags::SInt64,
        > as NonRepeatedFieldType>::clear(&mut self.s64_unlabeled, &mut self._bitfield)
    }
    // Optional, Variant(SInt64)
    pub fn s64_optional(&self) -> i64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::SInt64,
            10,
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
            10,
        > as NonRepeatedFieldType>::get_field_opt(&self.s64_optional, &self._bitfield)
    }
    pub fn has_s64_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::SInt64,
            10,
        > as NonRepeatedFieldType>::get_field_opt(&self.s64_optional, &self._bitfield)
        .is_some()
    }
    pub fn s64_optional_mut(&mut self) -> &mut i64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::SInt64,
            10,
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
            10,
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
    // Singular, Bits32(Fixed32)
    pub fn fixed32_unlabeled(&self) -> u32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            u32,
            self::_puroro::tags::Fixed32,
        > as NonRepeatedFieldType>::get_field(
            &self.fixed32_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn fixed32_unlabeled_opt(&self) -> ::std::option::Option<u32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            u32,
            self::_puroro::tags::Fixed32,
        > as NonRepeatedFieldType>::get_field_opt(&self.fixed32_unlabeled, &self._bitfield)
    }
    pub fn has_fixed32_unlabeled(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            u32,
            self::_puroro::tags::Fixed32,
        > as NonRepeatedFieldType>::get_field_opt(&self.fixed32_unlabeled, &self._bitfield)
        .is_some()
    }
    pub fn fixed32_unlabeled_mut(&mut self) -> &mut u32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            u32,
            self::_puroro::tags::Fixed32,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.fixed32_unlabeled,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_fixed32_unlabeled(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            u32,
            self::_puroro::tags::Fixed32,
        > as NonRepeatedFieldType>::clear(&mut self.fixed32_unlabeled, &mut self._bitfield)
    }
    // Optional, Bits32(Fixed32)
    pub fn fixed32_optional(&self) -> u32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u32,
            self::_puroro::tags::Fixed32,
            11,
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
            11,
        > as NonRepeatedFieldType>::get_field_opt(&self.fixed32_optional, &self._bitfield)
    }
    pub fn has_fixed32_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u32,
            self::_puroro::tags::Fixed32,
            11,
        > as NonRepeatedFieldType>::get_field_opt(&self.fixed32_optional, &self._bitfield)
        .is_some()
    }
    pub fn fixed32_optional_mut(&mut self) -> &mut u32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u32,
            self::_puroro::tags::Fixed32,
            11,
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
            11,
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
    // Singular, Bits64(Fixed64)
    pub fn fixed64_unlabeled(&self) -> u64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            u64,
            self::_puroro::tags::Fixed64,
        > as NonRepeatedFieldType>::get_field(
            &self.fixed64_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn fixed64_unlabeled_opt(&self) -> ::std::option::Option<u64> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            u64,
            self::_puroro::tags::Fixed64,
        > as NonRepeatedFieldType>::get_field_opt(&self.fixed64_unlabeled, &self._bitfield)
    }
    pub fn has_fixed64_unlabeled(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            u64,
            self::_puroro::tags::Fixed64,
        > as NonRepeatedFieldType>::get_field_opt(&self.fixed64_unlabeled, &self._bitfield)
        .is_some()
    }
    pub fn fixed64_unlabeled_mut(&mut self) -> &mut u64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            u64,
            self::_puroro::tags::Fixed64,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.fixed64_unlabeled,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_fixed64_unlabeled(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            u64,
            self::_puroro::tags::Fixed64,
        > as NonRepeatedFieldType>::clear(&mut self.fixed64_unlabeled, &mut self._bitfield)
    }
    // Optional, Bits64(Fixed64)
    pub fn fixed64_optional(&self) -> u64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u64,
            self::_puroro::tags::Fixed64,
            12,
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
            12,
        > as NonRepeatedFieldType>::get_field_opt(&self.fixed64_optional, &self._bitfield)
    }
    pub fn has_fixed64_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u64,
            self::_puroro::tags::Fixed64,
            12,
        > as NonRepeatedFieldType>::get_field_opt(&self.fixed64_optional, &self._bitfield)
        .is_some()
    }
    pub fn fixed64_optional_mut(&mut self) -> &mut u64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u64,
            self::_puroro::tags::Fixed64,
            12,
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
            12,
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
    // Singular, Bits32(SFixed32)
    pub fn sfixed32_unlabeled(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            i32,
            self::_puroro::tags::SFixed32,
        > as NonRepeatedFieldType>::get_field(
            &self.sfixed32_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn sfixed32_unlabeled_opt(&self) -> ::std::option::Option<i32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            i32,
            self::_puroro::tags::SFixed32,
        > as NonRepeatedFieldType>::get_field_opt(&self.sfixed32_unlabeled, &self._bitfield)
    }
    pub fn has_sfixed32_unlabeled(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            i32,
            self::_puroro::tags::SFixed32,
        > as NonRepeatedFieldType>::get_field_opt(&self.sfixed32_unlabeled, &self._bitfield)
        .is_some()
    }
    pub fn sfixed32_unlabeled_mut(&mut self) -> &mut i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            i32,
            self::_puroro::tags::SFixed32,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.sfixed32_unlabeled,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_sfixed32_unlabeled(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            i32,
            self::_puroro::tags::SFixed32,
        > as NonRepeatedFieldType>::clear(&mut self.sfixed32_unlabeled, &mut self._bitfield)
    }
    // Optional, Bits32(SFixed32)
    pub fn sfixed32_optional(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::SFixed32,
            13,
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
            13,
        > as NonRepeatedFieldType>::get_field_opt(&self.sfixed32_optional, &self._bitfield)
    }
    pub fn has_sfixed32_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::SFixed32,
            13,
        > as NonRepeatedFieldType>::get_field_opt(&self.sfixed32_optional, &self._bitfield)
        .is_some()
    }
    pub fn sfixed32_optional_mut(&mut self) -> &mut i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::SFixed32,
            13,
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
            13,
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
    // Singular, Bits64(SFixed64)
    pub fn sfixed64_unlabeled(&self) -> i64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            i64,
            self::_puroro::tags::SFixed64,
        > as NonRepeatedFieldType>::get_field(
            &self.sfixed64_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn sfixed64_unlabeled_opt(&self) -> ::std::option::Option<i64> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            i64,
            self::_puroro::tags::SFixed64,
        > as NonRepeatedFieldType>::get_field_opt(&self.sfixed64_unlabeled, &self._bitfield)
    }
    pub fn has_sfixed64_unlabeled(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            i64,
            self::_puroro::tags::SFixed64,
        > as NonRepeatedFieldType>::get_field_opt(&self.sfixed64_unlabeled, &self._bitfield)
        .is_some()
    }
    pub fn sfixed64_unlabeled_mut(&mut self) -> &mut i64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            i64,
            self::_puroro::tags::SFixed64,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.sfixed64_unlabeled,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_sfixed64_unlabeled(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            i64,
            self::_puroro::tags::SFixed64,
        > as NonRepeatedFieldType>::clear(&mut self.sfixed64_unlabeled, &mut self._bitfield)
    }
    // Optional, Bits64(SFixed64)
    pub fn sfixed64_optional(&self) -> i64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::SFixed64,
            14,
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
            14,
        > as NonRepeatedFieldType>::get_field_opt(&self.sfixed64_optional, &self._bitfield)
    }
    pub fn has_sfixed64_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::SFixed64,
            14,
        > as NonRepeatedFieldType>::get_field_opt(&self.sfixed64_optional, &self._bitfield)
        .is_some()
    }
    pub fn sfixed64_optional_mut(&mut self) -> &mut i64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::SFixed64,
            14,
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
            14,
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
    // Singular, Bits64(Double)
    pub fn f64_unlabeled(&self) -> f64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            f64,
            self::_puroro::tags::Double,
        > as NonRepeatedFieldType>::get_field(
            &self.f64_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn f64_unlabeled_opt(&self) -> ::std::option::Option<f64> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            f64,
            self::_puroro::tags::Double,
        > as NonRepeatedFieldType>::get_field_opt(&self.f64_unlabeled, &self._bitfield)
    }
    pub fn has_f64_unlabeled(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            f64,
            self::_puroro::tags::Double,
        > as NonRepeatedFieldType>::get_field_opt(&self.f64_unlabeled, &self._bitfield)
        .is_some()
    }
    pub fn f64_unlabeled_mut(&mut self) -> &mut f64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            f64,
            self::_puroro::tags::Double,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.f64_unlabeled,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_f64_unlabeled(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField<
            f64,
            self::_puroro::tags::Double,
        > as NonRepeatedFieldType>::clear(&mut self.f64_unlabeled, &mut self._bitfield)
    }
    // Optional, Bits64(Double)
    pub fn f64_optional(&self) -> f64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            f64,
            self::_puroro::tags::Double,
            15,
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
            15,
        > as NonRepeatedFieldType>::get_field_opt(&self.f64_optional, &self._bitfield)
    }
    pub fn has_f64_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            f64,
            self::_puroro::tags::Double,
            15,
        > as NonRepeatedFieldType>::get_field_opt(&self.f64_optional, &self._bitfield)
        .is_some()
    }
    pub fn f64_optional_mut(&mut self) -> &mut f64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            f64,
            self::_puroro::tags::Double,
            15,
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
            15,
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
                    self::_puroro::internal::field_type::SingularNumericalField::<i32, self::_puroro::tags::Int32> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.i32_unlabeled,
                    &mut self._bitfield,
                    _field_data,
                )?,
                2 => <
                    self::_puroro::internal::field_type::OptionalNumericalField::<i32, self::_puroro::tags::Int32, 0> as self::_puroro::internal::field_type::FieldType
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
                    self::_puroro::internal::field_type::SingularNumericalField::<f32, self::_puroro::tags::Float> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.float_unlabeled,
                    &mut self._bitfield,
                    _field_data,
                )?,
                12 => <
                    self::_puroro::internal::field_type::OptionalNumericalField::<f32, self::_puroro::tags::Float, 1> as self::_puroro::internal::field_type::FieldType
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
                    self::_puroro::internal::field_type::SingularBytesField as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.bytes_unlabeled,
                    &mut self._bitfield,
                    _field_data,
                )?,
                22 => <
                    self::_puroro::internal::field_type::OptionalBytesField<2> as self::_puroro::internal::field_type::FieldType
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
                    self::_puroro::internal::field_type::SingularStringField as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.string_unlabeled,
                    &mut self._bitfield,
                    _field_data,
                )?,
                32 => <
                    self::_puroro::internal::field_type::OptionalStringField::<3> as self::_puroro::internal::field_type::FieldType
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
                    self::_puroro::internal::field_type::SingularNumericalField::<_puroro_root::full_coverage3::Enum, self::_puroro::tags::Enum3<_puroro_root::full_coverage3::Enum>> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.enum_unlabeled,
                    &mut self._bitfield,
                    _field_data,
                )?,
                42 => <
                    self::_puroro::internal::field_type::OptionalNumericalField::<_puroro_root::full_coverage3::Enum, self::_puroro::tags::Enum3<_puroro_root::full_coverage3::Enum>, 4> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.enum_optional,
                    &mut self._bitfield,
                    _field_data,
                )?,
                43 => <
                    self::_puroro::internal::field_type::RepeatedNumericalField::<_puroro_root::full_coverage3::Enum, self::_puroro::tags::Enum3<_puroro_root::full_coverage3::Enum>> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.enum_repeated,
                    &mut self._bitfield,
                    _field_data,
                )?,
                51 => <
                    self::_puroro::internal::field_type::SingularHeapMessageField::<_puroro_root::full_coverage3::msg::Submsg> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.submsg_unlabeled,
                    &mut self._bitfield,
                    _field_data,
                )?,
                52 => <
                    self::_puroro::internal::field_type::SingularHeapMessageField::<_puroro_root::full_coverage3::msg::Submsg> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.submsg_optional,
                    &mut self._bitfield,
                    _field_data,
                )?,
                53 => <
                    self::_puroro::internal::field_type::RepeatedMessageField::<_puroro_root::full_coverage3::msg::Submsg> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.submsg_repeated,
                    &mut self._bitfield,
                    _field_data,
                )?,
                101 => <
                    self::_puroro::internal::field_type::SingularNumericalField::<i64, self::_puroro::tags::Int64> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.i64_unlabeled,
                    &mut self._bitfield,
                    _field_data,
                )?,
                102 => <
                    self::_puroro::internal::field_type::OptionalNumericalField::<i64, self::_puroro::tags::Int64, 6> as self::_puroro::internal::field_type::FieldType
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
                    self::_puroro::internal::field_type::SingularNumericalField::<u32, self::_puroro::tags::UInt32> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.u32_unlabeled,
                    &mut self._bitfield,
                    _field_data,
                )?,
                112 => <
                    self::_puroro::internal::field_type::OptionalNumericalField::<u32, self::_puroro::tags::UInt32, 7> as self::_puroro::internal::field_type::FieldType
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
                    self::_puroro::internal::field_type::SingularNumericalField::<u64, self::_puroro::tags::UInt64> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.u64_unlabeled,
                    &mut self._bitfield,
                    _field_data,
                )?,
                122 => <
                    self::_puroro::internal::field_type::OptionalNumericalField::<u64, self::_puroro::tags::UInt64, 8> as self::_puroro::internal::field_type::FieldType
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
                    self::_puroro::internal::field_type::SingularNumericalField::<i32, self::_puroro::tags::SInt32> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.s32_unlabeled,
                    &mut self._bitfield,
                    _field_data,
                )?,
                132 => <
                    self::_puroro::internal::field_type::OptionalNumericalField::<i32, self::_puroro::tags::SInt32, 9> as self::_puroro::internal::field_type::FieldType
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
                    self::_puroro::internal::field_type::SingularNumericalField::<i64, self::_puroro::tags::SInt64> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.s64_unlabeled,
                    &mut self._bitfield,
                    _field_data,
                )?,
                142 => <
                    self::_puroro::internal::field_type::OptionalNumericalField::<i64, self::_puroro::tags::SInt64, 10> as self::_puroro::internal::field_type::FieldType
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
                    self::_puroro::internal::field_type::SingularNumericalField::<u32, self::_puroro::tags::Fixed32> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.fixed32_unlabeled,
                    &mut self._bitfield,
                    _field_data,
                )?,
                152 => <
                    self::_puroro::internal::field_type::OptionalNumericalField::<u32, self::_puroro::tags::Fixed32, 11> as self::_puroro::internal::field_type::FieldType
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
                    self::_puroro::internal::field_type::SingularNumericalField::<u64, self::_puroro::tags::Fixed64> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.fixed64_unlabeled,
                    &mut self._bitfield,
                    _field_data,
                )?,
                162 => <
                    self::_puroro::internal::field_type::OptionalNumericalField::<u64, self::_puroro::tags::Fixed64, 12> as self::_puroro::internal::field_type::FieldType
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
                    self::_puroro::internal::field_type::SingularNumericalField::<i32, self::_puroro::tags::SFixed32> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.sfixed32_unlabeled,
                    &mut self._bitfield,
                    _field_data,
                )?,
                172 => <
                    self::_puroro::internal::field_type::OptionalNumericalField::<i32, self::_puroro::tags::SFixed32, 13> as self::_puroro::internal::field_type::FieldType
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
                    self::_puroro::internal::field_type::SingularNumericalField::<i64, self::_puroro::tags::SFixed64> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.sfixed64_unlabeled,
                    &mut self._bitfield,
                    _field_data,
                )?,
                182 => <
                    self::_puroro::internal::field_type::OptionalNumericalField::<i64, self::_puroro::tags::SFixed64, 14> as self::_puroro::internal::field_type::FieldType
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
                    self::_puroro::internal::field_type::SingularNumericalField::<f64, self::_puroro::tags::Double> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.f64_unlabeled,
                    &mut self._bitfield,
                    _field_data,
                )?,
                192 => <
                    self::_puroro::internal::field_type::OptionalNumericalField::<f64, self::_puroro::tags::Double, 15> as self::_puroro::internal::field_type::FieldType
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
        <self::_puroro::internal::field_type::SingularNumericalField<
            i32,
            self::_puroro::tags::Int32,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.i32_unlabeled,
            &self._bitfield,
            1,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
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
        <self::_puroro::internal::field_type::SingularNumericalField<
            f32,
            self::_puroro::tags::Float,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.float_unlabeled,
            &self._bitfield,
            11,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            f32,
            self::_puroro::tags::Float,
            1,
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
        <self::_puroro::internal::field_type::SingularBytesField as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.bytes_unlabeled,
            &self._bitfield,
            21,
            out
        )?;
        <self::_puroro::internal::field_type::OptionalBytesField<2> as self::_puroro::internal::field_type::FieldType>::ser_to_write(
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
        <self::_puroro::internal::field_type::SingularStringField as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.string_unlabeled,
            &self._bitfield,
            31,
            out
        )?;
        <self::_puroro::internal::field_type::OptionalStringField::<3> as self::_puroro::internal::field_type::FieldType>::ser_to_write(
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
        <self::_puroro::internal::field_type::SingularNumericalField<
            _puroro_root::full_coverage3::Enum,
            self::_puroro::tags::Enum3<_puroro_root::full_coverage3::Enum>,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.enum_unlabeled,
            &self._bitfield,
            41,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::full_coverage3::Enum,
            self::_puroro::tags::Enum3<_puroro_root::full_coverage3::Enum>,
            4,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.enum_optional,
            &self._bitfield,
            42,
            out,
        )?;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            _puroro_root::full_coverage3::Enum,
            self::_puroro::tags::Enum3<_puroro_root::full_coverage3::Enum>,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.enum_repeated,
            &self._bitfield,
            43,
            out,
        )?;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::full_coverage3::msg::Submsg,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.submsg_unlabeled,
            &self._bitfield,
            51,
            out,
        )?;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::full_coverage3::msg::Submsg,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.submsg_optional,
            &self._bitfield,
            52,
            out,
        )?;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::full_coverage3::msg::Submsg,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.submsg_repeated,
            &self._bitfield,
            53,
            out,
        )?;
        <self::_puroro::internal::field_type::SingularNumericalField<
            i64,
            self::_puroro::tags::Int64,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.i64_unlabeled,
            &self._bitfield,
            101,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::Int64,
            6,
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
        <self::_puroro::internal::field_type::SingularNumericalField<
            u32,
            self::_puroro::tags::UInt32,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.u32_unlabeled,
            &self._bitfield,
            111,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u32,
            self::_puroro::tags::UInt32,
            7,
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
        <self::_puroro::internal::field_type::SingularNumericalField<
            u64,
            self::_puroro::tags::UInt64,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.u64_unlabeled,
            &self._bitfield,
            121,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u64,
            self::_puroro::tags::UInt64,
            8,
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
        <self::_puroro::internal::field_type::SingularNumericalField<
            i32,
            self::_puroro::tags::SInt32,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.s32_unlabeled,
            &self._bitfield,
            131,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::SInt32,
            9,
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
        <self::_puroro::internal::field_type::SingularNumericalField<
            i64,
            self::_puroro::tags::SInt64,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.s64_unlabeled,
            &self._bitfield,
            141,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::SInt64,
            10,
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
        <self::_puroro::internal::field_type::SingularNumericalField<
            u32,
            self::_puroro::tags::Fixed32,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.fixed32_unlabeled,
            &self._bitfield,
            151,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u32,
            self::_puroro::tags::Fixed32,
            11,
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
        <self::_puroro::internal::field_type::SingularNumericalField<
            u64,
            self::_puroro::tags::Fixed64,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.fixed64_unlabeled,
            &self._bitfield,
            161,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u64,
            self::_puroro::tags::Fixed64,
            12,
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
        <self::_puroro::internal::field_type::SingularNumericalField<
            i32,
            self::_puroro::tags::SFixed32,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.sfixed32_unlabeled,
            &self._bitfield,
            171,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::SFixed32,
            13,
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
        <self::_puroro::internal::field_type::SingularNumericalField<
            i64,
            self::_puroro::tags::SFixed64,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.sfixed64_unlabeled,
            &self._bitfield,
            181,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::SFixed64,
            14,
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
        <self::_puroro::internal::field_type::SingularNumericalField<
            f64,
            self::_puroro::tags::Double,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.f64_unlabeled,
            &self._bitfield,
            191,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            f64,
            self::_puroro::tags::Double,
            15,
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
            i32_unlabeled: Clone::clone(&self.i32_unlabeled),
            i32_optional: Clone::clone(&self.i32_optional),
            i32_repeated: Clone::clone(&self.i32_repeated),
            float_unlabeled: Clone::clone(&self.float_unlabeled),
            float_optional: Clone::clone(&self.float_optional),
            float_repeated: Clone::clone(&self.float_repeated),
            bytes_unlabeled: Clone::clone(&self.bytes_unlabeled),
            bytes_optional: Clone::clone(&self.bytes_optional),
            bytes_repeated: Clone::clone(&self.bytes_repeated),
            string_unlabeled: Clone::clone(&self.string_unlabeled),
            string_optional: Clone::clone(&self.string_optional),
            string_repeated: Clone::clone(&self.string_repeated),
            enum_unlabeled: Clone::clone(&self.enum_unlabeled),
            enum_optional: Clone::clone(&self.enum_optional),
            enum_repeated: Clone::clone(&self.enum_repeated),
            submsg_unlabeled: Clone::clone(&self.submsg_unlabeled),
            submsg_optional: Clone::clone(&self.submsg_optional),
            submsg_repeated: Clone::clone(&self.submsg_repeated),
            i64_unlabeled: Clone::clone(&self.i64_unlabeled),
            i64_optional: Clone::clone(&self.i64_optional),
            i64_repeated: Clone::clone(&self.i64_repeated),
            u32_unlabeled: Clone::clone(&self.u32_unlabeled),
            u32_optional: Clone::clone(&self.u32_optional),
            u32_repeated: Clone::clone(&self.u32_repeated),
            u64_unlabeled: Clone::clone(&self.u64_unlabeled),
            u64_optional: Clone::clone(&self.u64_optional),
            u64_repeated: Clone::clone(&self.u64_repeated),
            s32_unlabeled: Clone::clone(&self.s32_unlabeled),
            s32_optional: Clone::clone(&self.s32_optional),
            s32_repeated: Clone::clone(&self.s32_repeated),
            s64_unlabeled: Clone::clone(&self.s64_unlabeled),
            s64_optional: Clone::clone(&self.s64_optional),
            s64_repeated: Clone::clone(&self.s64_repeated),
            fixed32_unlabeled: Clone::clone(&self.fixed32_unlabeled),
            fixed32_optional: Clone::clone(&self.fixed32_optional),
            fixed32_repeated: Clone::clone(&self.fixed32_repeated),
            fixed64_unlabeled: Clone::clone(&self.fixed64_unlabeled),
            fixed64_optional: Clone::clone(&self.fixed64_optional),
            fixed64_repeated: Clone::clone(&self.fixed64_repeated),
            sfixed32_unlabeled: Clone::clone(&self.sfixed32_unlabeled),
            sfixed32_optional: Clone::clone(&self.sfixed32_optional),
            sfixed32_repeated: Clone::clone(&self.sfixed32_repeated),
            sfixed64_unlabeled: Clone::clone(&self.sfixed64_unlabeled),
            sfixed64_optional: Clone::clone(&self.sfixed64_optional),
            sfixed64_repeated: Clone::clone(&self.sfixed64_repeated),
            f64_unlabeled: Clone::clone(&self.f64_unlabeled),
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
            .field("i32_unlabeled", &self.i32_unlabeled())
            .field("i32_optional", &self.i32_optional())
            .field("i32_repeated", &self.i32_repeated())
            .field("float_unlabeled", &self.float_unlabeled())
            .field("float_optional", &self.float_optional())
            .field("float_repeated", &self.float_repeated())
            .field("bytes_unlabeled", &self.bytes_unlabeled())
            .field("bytes_optional", &self.bytes_optional())
            .field("bytes_repeated", &self.bytes_repeated())
            .field("string_unlabeled", &self.string_unlabeled())
            .field("string_optional", &self.string_optional())
            .field("string_repeated", &self.string_repeated())
            .field("enum_unlabeled", &self.enum_unlabeled())
            .field("enum_optional", &self.enum_optional())
            .field("enum_repeated", &self.enum_repeated())
            .field("submsg_unlabeled", &self.submsg_unlabeled())
            .field("submsg_optional", &self.submsg_optional())
            .field("submsg_repeated", &self.submsg_repeated())
            .field("i64_unlabeled", &self.i64_unlabeled())
            .field("i64_optional", &self.i64_optional())
            .field("i64_repeated", &self.i64_repeated())
            .field("u32_unlabeled", &self.u32_unlabeled())
            .field("u32_optional", &self.u32_optional())
            .field("u32_repeated", &self.u32_repeated())
            .field("u64_unlabeled", &self.u64_unlabeled())
            .field("u64_optional", &self.u64_optional())
            .field("u64_repeated", &self.u64_repeated())
            .field("s32_unlabeled", &self.s32_unlabeled())
            .field("s32_optional", &self.s32_optional())
            .field("s32_repeated", &self.s32_repeated())
            .field("s64_unlabeled", &self.s64_unlabeled())
            .field("s64_optional", &self.s64_optional())
            .field("s64_repeated", &self.s64_repeated())
            .field("fixed32_unlabeled", &self.fixed32_unlabeled())
            .field("fixed32_optional", &self.fixed32_optional())
            .field("fixed32_repeated", &self.fixed32_repeated())
            .field("fixed64_unlabeled", &self.fixed64_unlabeled())
            .field("fixed64_optional", &self.fixed64_optional())
            .field("fixed64_repeated", &self.fixed64_repeated())
            .field("sfixed32_unlabeled", &self.sfixed32_unlabeled())
            .field("sfixed32_optional", &self.sfixed32_optional())
            .field("sfixed32_repeated", &self.sfixed32_repeated())
            .field("sfixed64_unlabeled", &self.sfixed64_unlabeled())
            .field("sfixed64_optional", &self.sfixed64_optional())
            .field("sfixed64_repeated", &self.sfixed64_repeated())
            .field("f64_unlabeled", &self.f64_unlabeled())
            .field("f64_optional", &self.f64_optional())
            .field("f64_repeated", &self.f64_repeated())
            .finish()
    }
}

impl ::std::cmp::PartialEq for Msg {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;

        true && self.i32_unlabeled_opt() == rhs.i32_unlabeled_opt()
            && self.i32_optional_opt() == rhs.i32_optional_opt()
            && self.i32_repeated() == rhs.i32_repeated()
            && self.float_unlabeled_opt() == rhs.float_unlabeled_opt()
            && self.float_optional_opt() == rhs.float_optional_opt()
            && self.float_repeated() == rhs.float_repeated()
            && self.bytes_unlabeled_opt() == rhs.bytes_unlabeled_opt()
            && self.bytes_optional_opt() == rhs.bytes_optional_opt()
            && self.bytes_repeated() == rhs.bytes_repeated()
            && self.string_unlabeled_opt() == rhs.string_unlabeled_opt()
            && self.string_optional_opt() == rhs.string_optional_opt()
            && self.string_repeated() == rhs.string_repeated()
            && self.enum_unlabeled_opt() == rhs.enum_unlabeled_opt()
            && self.enum_optional_opt() == rhs.enum_optional_opt()
            && self.enum_repeated() == rhs.enum_repeated()
            && self.submsg_unlabeled_opt() == rhs.submsg_unlabeled_opt()
            && self.submsg_optional_opt() == rhs.submsg_optional_opt()
            && self.submsg_repeated() == rhs.submsg_repeated()
            && self.i64_unlabeled_opt() == rhs.i64_unlabeled_opt()
            && self.i64_optional_opt() == rhs.i64_optional_opt()
            && self.i64_repeated() == rhs.i64_repeated()
            && self.u32_unlabeled_opt() == rhs.u32_unlabeled_opt()
            && self.u32_optional_opt() == rhs.u32_optional_opt()
            && self.u32_repeated() == rhs.u32_repeated()
            && self.u64_unlabeled_opt() == rhs.u64_unlabeled_opt()
            && self.u64_optional_opt() == rhs.u64_optional_opt()
            && self.u64_repeated() == rhs.u64_repeated()
            && self.s32_unlabeled_opt() == rhs.s32_unlabeled_opt()
            && self.s32_optional_opt() == rhs.s32_optional_opt()
            && self.s32_repeated() == rhs.s32_repeated()
            && self.s64_unlabeled_opt() == rhs.s64_unlabeled_opt()
            && self.s64_optional_opt() == rhs.s64_optional_opt()
            && self.s64_repeated() == rhs.s64_repeated()
            && self.fixed32_unlabeled_opt() == rhs.fixed32_unlabeled_opt()
            && self.fixed32_optional_opt() == rhs.fixed32_optional_opt()
            && self.fixed32_repeated() == rhs.fixed32_repeated()
            && self.fixed64_unlabeled_opt() == rhs.fixed64_unlabeled_opt()
            && self.fixed64_optional_opt() == rhs.fixed64_optional_opt()
            && self.fixed64_repeated() == rhs.fixed64_repeated()
            && self.sfixed32_unlabeled_opt() == rhs.sfixed32_unlabeled_opt()
            && self.sfixed32_optional_opt() == rhs.sfixed32_optional_opt()
            && self.sfixed32_repeated() == rhs.sfixed32_repeated()
            && self.sfixed64_unlabeled_opt() == rhs.sfixed64_unlabeled_opt()
            && self.sfixed64_optional_opt() == rhs.sfixed64_optional_opt()
            && self.sfixed64_repeated() == rhs.sfixed64_repeated()
            && self.f64_unlabeled_opt() == rhs.f64_unlabeled_opt()
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
    _None(i32),
}

impl ::std::default::Default for Enum {
    fn default() -> Self {
        Enum::ZEROTH
    }
}

impl ::std::convert::From<i32> for Enum {
    fn from(x: i32) -> Self {
        match x {
            0 => self::Enum::ZEROTH,
            1 => self::Enum::FIRST,
            10 => self::Enum::TENTH,
            e => self::Enum::_None(e),
        }
    }
}

impl ::std::convert::From<Enum> for i32 {
    fn from(x: Enum) -> i32 {
        match x {
            self::Enum::ZEROTH => 0,
            self::Enum::FIRST => 1,
            self::Enum::TENTH => 10,
            self::Enum::_None(y) => y,
        }
    }
}
pub mod msg;
