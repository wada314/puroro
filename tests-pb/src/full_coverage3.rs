mod _root {
    #[allow(unused)]
    pub(crate) use super::super::_root::*;
}
mod _puroro {
    #[allow(unused)]
    pub(crate) use ::puroro::*;
}
mod _pinternal {
    #[allow(unused)]
    pub(crate) use ::puroro::internal::*;
}
pub mod msg;
#[derive(::std::default::Default)]
pub struct Msg {
    i32_unlabeled: self::_pinternal::SingularNumericalField::<
        i32,
        self::_pinternal::tags::Int32,
    >,
    i32_optional: self::_pinternal::OptionalNumericalField::<
        i32,
        self::_pinternal::tags::Int32,
        0usize,
    >,
    i32_repeated: self::_pinternal::RepeatedNumericalField::<
        i32,
        self::_pinternal::tags::Int32,
    >,
    float_unlabeled: self::_pinternal::SingularNumericalField::<
        f32,
        self::_pinternal::tags::Float,
    >,
    float_optional: self::_pinternal::OptionalNumericalField::<
        f32,
        self::_pinternal::tags::Float,
        1usize,
    >,
    float_repeated: self::_pinternal::RepeatedNumericalField::<
        f32,
        self::_pinternal::tags::Float,
    >,
    bytes_unlabeled: self::_pinternal::SingularUnsizedField::<
        ::std::vec::Vec<u8>,
        self::_pinternal::tags::Bytes,
    >,
    bytes_optional: self::_pinternal::OptionalUnsizedField::<
        ::std::vec::Vec<u8>,
        self::_pinternal::tags::Bytes,
        2usize,
    >,
    bytes_repeated: self::_pinternal::RepeatedUnsizedField::<
        ::std::vec::Vec<u8>,
        self::_pinternal::tags::Bytes,
    >,
    string_unlabeled: self::_pinternal::SingularUnsizedField::<
        ::std::string::String,
        self::_pinternal::tags::String,
    >,
    string_optional: self::_pinternal::OptionalUnsizedField::<
        ::std::string::String,
        self::_pinternal::tags::String,
        3usize,
    >,
    string_repeated: self::_pinternal::RepeatedUnsizedField::<
        ::std::string::String,
        self::_pinternal::tags::String,
    >,
    enum_unlabeled: self::_pinternal::SingularNumericalField::<
        self::_root::full_coverage3::Enum,
        self::_pinternal::tags::Enum3::<self::_root::full_coverage3::Enum>,
    >,
    enum_optional: self::_pinternal::OptionalNumericalField::<
        self::_root::full_coverage3::Enum,
        self::_pinternal::tags::Enum3::<self::_root::full_coverage3::Enum>,
        4usize,
    >,
    enum_repeated: self::_pinternal::RepeatedNumericalField::<
        self::_root::full_coverage3::Enum,
        self::_pinternal::tags::Enum3::<self::_root::full_coverage3::Enum>,
    >,
    submsg_unlabeled: self::_pinternal::SingularHeapMessageField::<
        self::_root::full_coverage3::msg::Submsg,
    >,
    submsg_optional: self::_pinternal::SingularHeapMessageField::<
        self::_root::full_coverage3::msg::Submsg,
    >,
    submsg_repeated: self::_pinternal::RepeatedMessageField::<
        self::_root::full_coverage3::msg::Submsg,
    >,
    i64_unlabeled: self::_pinternal::SingularNumericalField::<
        i64,
        self::_pinternal::tags::Int64,
    >,
    i64_optional: self::_pinternal::OptionalNumericalField::<
        i64,
        self::_pinternal::tags::Int64,
        5usize,
    >,
    i64_repeated: self::_pinternal::RepeatedNumericalField::<
        i64,
        self::_pinternal::tags::Int64,
    >,
    u32_unlabeled: self::_pinternal::SingularNumericalField::<
        u32,
        self::_pinternal::tags::UInt32,
    >,
    u32_optional: self::_pinternal::OptionalNumericalField::<
        u32,
        self::_pinternal::tags::UInt32,
        6usize,
    >,
    u32_repeated: self::_pinternal::RepeatedNumericalField::<
        u32,
        self::_pinternal::tags::UInt32,
    >,
    u64_unlabeled: self::_pinternal::SingularNumericalField::<
        u64,
        self::_pinternal::tags::UInt64,
    >,
    u64_optional: self::_pinternal::OptionalNumericalField::<
        u64,
        self::_pinternal::tags::UInt64,
        7usize,
    >,
    u64_repeated: self::_pinternal::RepeatedNumericalField::<
        u64,
        self::_pinternal::tags::UInt64,
    >,
    s32_unlabeled: self::_pinternal::SingularNumericalField::<
        i32,
        self::_pinternal::tags::SInt32,
    >,
    s32_optional: self::_pinternal::OptionalNumericalField::<
        i32,
        self::_pinternal::tags::SInt32,
        8usize,
    >,
    s32_repeated: self::_pinternal::RepeatedNumericalField::<
        i32,
        self::_pinternal::tags::SInt32,
    >,
    s64_unlabeled: self::_pinternal::SingularNumericalField::<
        i64,
        self::_pinternal::tags::SInt64,
    >,
    s64_optional: self::_pinternal::OptionalNumericalField::<
        i64,
        self::_pinternal::tags::SInt64,
        9usize,
    >,
    s64_repeated: self::_pinternal::RepeatedNumericalField::<
        i64,
        self::_pinternal::tags::SInt64,
    >,
    fixed32_unlabeled: self::_pinternal::SingularNumericalField::<
        u32,
        self::_pinternal::tags::Fixed32,
    >,
    fixed32_optional: self::_pinternal::OptionalNumericalField::<
        u32,
        self::_pinternal::tags::Fixed32,
        10usize,
    >,
    fixed32_repeated: self::_pinternal::RepeatedNumericalField::<
        u32,
        self::_pinternal::tags::Fixed32,
    >,
    fixed64_unlabeled: self::_pinternal::SingularNumericalField::<
        u64,
        self::_pinternal::tags::Fixed64,
    >,
    fixed64_optional: self::_pinternal::OptionalNumericalField::<
        u64,
        self::_pinternal::tags::Fixed64,
        11usize,
    >,
    fixed64_repeated: self::_pinternal::RepeatedNumericalField::<
        u64,
        self::_pinternal::tags::Fixed64,
    >,
    sfixed32_unlabeled: self::_pinternal::SingularNumericalField::<
        i32,
        self::_pinternal::tags::SFixed32,
    >,
    sfixed32_optional: self::_pinternal::OptionalNumericalField::<
        i32,
        self::_pinternal::tags::SFixed32,
        12usize,
    >,
    sfixed32_repeated: self::_pinternal::RepeatedNumericalField::<
        i32,
        self::_pinternal::tags::SFixed32,
    >,
    sfixed64_unlabeled: self::_pinternal::SingularNumericalField::<
        i64,
        self::_pinternal::tags::SFixed64,
    >,
    sfixed64_optional: self::_pinternal::OptionalNumericalField::<
        i64,
        self::_pinternal::tags::SFixed64,
        13usize,
    >,
    sfixed64_repeated: self::_pinternal::RepeatedNumericalField::<
        i64,
        self::_pinternal::tags::SFixed64,
    >,
    f64_unlabeled: self::_pinternal::SingularNumericalField::<
        f64,
        self::_pinternal::tags::Double,
    >,
    f64_optional: self::_pinternal::OptionalNumericalField::<
        f64,
        self::_pinternal::tags::Double,
        14usize,
    >,
    f64_repeated: self::_pinternal::RepeatedNumericalField::<
        f64,
        self::_pinternal::tags::Double,
    >,
    _bitfield: self::_pinternal::BitArray<1usize>,
}
impl Msg {
    pub fn i32_unlabeled(&self) -> i32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.i32_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn i32_unlabeled_opt(&self) -> ::std::option::Option::<i32> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
        > as NonRepeatedFieldType>::get_field_opt(&self.i32_unlabeled, &self._bitfield)
    }
    pub fn i32_unlabeled_mut(&mut self) -> &mut i32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.i32_unlabeled,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_i32_unlabeled(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
        > as NonRepeatedFieldType>::get_field_opt(&self.i32_unlabeled, &self._bitfield)
            .is_some()
    }
    pub fn clear_i32_unlabeled(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
        > as NonRepeatedFieldType>::clear(&mut self.i32_unlabeled, &mut self._bitfield)
    }
    pub fn i32_optional(&self) -> i32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.i32_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn i32_optional_opt(&self) -> ::std::option::Option::<i32> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.i32_optional, &self._bitfield)
    }
    pub fn i32_optional_mut(&mut self) -> &mut i32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.i32_optional,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_i32_optional(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.i32_optional, &self._bitfield)
            .is_some()
    }
    pub fn clear_i32_optional(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::clear(&mut self.i32_optional, &mut self._bitfield)
    }
    pub fn i32_repeated(&self) -> &[i32] {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
        > as RepeatedFieldType>::get_field(&self.i32_repeated, &self._bitfield)
    }
    pub fn i32_repeated_mut(&mut self) -> &mut ::std::vec::Vec::<i32> {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
        > as RepeatedFieldType>::get_field_mut(
            &mut self.i32_repeated,
            &mut self._bitfield,
        )
    }
    pub fn clear_i32_repeated(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
        > as RepeatedFieldType>::clear(&mut self.i32_repeated, &mut self._bitfield)
    }
    pub fn float_unlabeled(&self) -> f32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            f32,
            self::_pinternal::tags::Float,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.float_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn float_unlabeled_opt(&self) -> ::std::option::Option::<f32> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            f32,
            self::_pinternal::tags::Float,
        > as NonRepeatedFieldType>::get_field_opt(&self.float_unlabeled, &self._bitfield)
    }
    pub fn float_unlabeled_mut(&mut self) -> &mut f32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            f32,
            self::_pinternal::tags::Float,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.float_unlabeled,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_float_unlabeled(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            f32,
            self::_pinternal::tags::Float,
        > as NonRepeatedFieldType>::get_field_opt(&self.float_unlabeled, &self._bitfield)
            .is_some()
    }
    pub fn clear_float_unlabeled(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            f32,
            self::_pinternal::tags::Float,
        > as NonRepeatedFieldType>::clear(&mut self.float_unlabeled, &mut self._bitfield)
    }
    pub fn float_optional(&self) -> f32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            f32,
            self::_pinternal::tags::Float,
            1usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.float_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn float_optional_opt(&self) -> ::std::option::Option::<f32> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            f32,
            self::_pinternal::tags::Float,
            1usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.float_optional, &self._bitfield)
    }
    pub fn float_optional_mut(&mut self) -> &mut f32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            f32,
            self::_pinternal::tags::Float,
            1usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.float_optional,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_float_optional(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            f32,
            self::_pinternal::tags::Float,
            1usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.float_optional, &self._bitfield)
            .is_some()
    }
    pub fn clear_float_optional(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            f32,
            self::_pinternal::tags::Float,
            1usize,
        > as NonRepeatedFieldType>::clear(&mut self.float_optional, &mut self._bitfield)
    }
    pub fn float_repeated(&self) -> &[f32] {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedNumericalField::<
            f32,
            self::_pinternal::tags::Float,
        > as RepeatedFieldType>::get_field(&self.float_repeated, &self._bitfield)
    }
    pub fn float_repeated_mut(&mut self) -> &mut ::std::vec::Vec::<f32> {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedNumericalField::<
            f32,
            self::_pinternal::tags::Float,
        > as RepeatedFieldType>::get_field_mut(
            &mut self.float_repeated,
            &mut self._bitfield,
        )
    }
    pub fn clear_float_repeated(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedNumericalField::<
            f32,
            self::_pinternal::tags::Float,
        > as RepeatedFieldType>::clear(&mut self.float_repeated, &mut self._bitfield)
    }
    pub fn bytes_unlabeled(&self) -> &[u8] {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularUnsizedField::<
            ::std::vec::Vec<u8>,
            self::_pinternal::tags::Bytes,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.bytes_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn bytes_unlabeled_opt(&self) -> ::std::option::Option::<&[u8]> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularUnsizedField::<
            ::std::vec::Vec<u8>,
            self::_pinternal::tags::Bytes,
        > as NonRepeatedFieldType>::get_field_opt(&self.bytes_unlabeled, &self._bitfield)
    }
    pub fn bytes_unlabeled_mut(&mut self) -> &mut ::std::vec::Vec::<u8> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularUnsizedField::<
            ::std::vec::Vec<u8>,
            self::_pinternal::tags::Bytes,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.bytes_unlabeled,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_bytes_unlabeled(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularUnsizedField::<
            ::std::vec::Vec<u8>,
            self::_pinternal::tags::Bytes,
        > as NonRepeatedFieldType>::get_field_opt(&self.bytes_unlabeled, &self._bitfield)
            .is_some()
    }
    pub fn clear_bytes_unlabeled(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularUnsizedField::<
            ::std::vec::Vec<u8>,
            self::_pinternal::tags::Bytes,
        > as NonRepeatedFieldType>::clear(&mut self.bytes_unlabeled, &mut self._bitfield)
    }
    pub fn bytes_optional(&self) -> &[u8] {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalUnsizedField::<
            ::std::vec::Vec<u8>,
            self::_pinternal::tags::Bytes,
            2usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.bytes_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn bytes_optional_opt(&self) -> ::std::option::Option::<&[u8]> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalUnsizedField::<
            ::std::vec::Vec<u8>,
            self::_pinternal::tags::Bytes,
            2usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.bytes_optional, &self._bitfield)
    }
    pub fn bytes_optional_mut(&mut self) -> &mut ::std::vec::Vec::<u8> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalUnsizedField::<
            ::std::vec::Vec<u8>,
            self::_pinternal::tags::Bytes,
            2usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.bytes_optional,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_bytes_optional(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalUnsizedField::<
            ::std::vec::Vec<u8>,
            self::_pinternal::tags::Bytes,
            2usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.bytes_optional, &self._bitfield)
            .is_some()
    }
    pub fn clear_bytes_optional(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalUnsizedField::<
            ::std::vec::Vec<u8>,
            self::_pinternal::tags::Bytes,
            2usize,
        > as NonRepeatedFieldType>::clear(&mut self.bytes_optional, &mut self._bitfield)
    }
    pub fn bytes_repeated(
        &self,
    ) -> &[impl ::std::ops::Deref::<
        Target = [u8],
    > + ::std::fmt::Debug + ::std::cmp::PartialEq] {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedUnsizedField::<
            ::std::vec::Vec<u8>,
            self::_pinternal::tags::Bytes,
        > as RepeatedFieldType>::get_field(&self.bytes_repeated, &self._bitfield)
    }
    pub fn bytes_repeated_mut(&mut self) -> &mut ::std::vec::Vec::<::std::vec::Vec<u8>> {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedUnsizedField::<
            ::std::vec::Vec<u8>,
            self::_pinternal::tags::Bytes,
        > as RepeatedFieldType>::get_field_mut(
            &mut self.bytes_repeated,
            &mut self._bitfield,
        )
    }
    pub fn clear_bytes_repeated(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedUnsizedField::<
            ::std::vec::Vec<u8>,
            self::_pinternal::tags::Bytes,
        > as RepeatedFieldType>::clear(&mut self.bytes_repeated, &mut self._bitfield)
    }
    pub fn string_unlabeled(&self) -> &str {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.string_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn string_unlabeled_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.string_unlabeled,
            &self._bitfield,
        )
    }
    pub fn string_unlabeled_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.string_unlabeled,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_string_unlabeled(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        > as NonRepeatedFieldType>::get_field_opt(
                &self.string_unlabeled,
                &self._bitfield,
            )
            .is_some()
    }
    pub fn clear_string_unlabeled(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        > as NonRepeatedFieldType>::clear(
            &mut self.string_unlabeled,
            &mut self._bitfield,
        )
    }
    pub fn string_optional(&self) -> &str {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            3usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.string_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn string_optional_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            3usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.string_optional, &self._bitfield)
    }
    pub fn string_optional_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            3usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.string_optional,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_string_optional(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            3usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.string_optional, &self._bitfield)
            .is_some()
    }
    pub fn clear_string_optional(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            3usize,
        > as NonRepeatedFieldType>::clear(&mut self.string_optional, &mut self._bitfield)
    }
    pub fn string_repeated(
        &self,
    ) -> &[impl ::std::ops::Deref::<
        Target = str,
    > + ::std::fmt::Debug + ::std::cmp::PartialEq] {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        > as RepeatedFieldType>::get_field(&self.string_repeated, &self._bitfield)
    }
    pub fn string_repeated_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<::std::string::String> {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        > as RepeatedFieldType>::get_field_mut(
            &mut self.string_repeated,
            &mut self._bitfield,
        )
    }
    pub fn clear_string_repeated(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        > as RepeatedFieldType>::clear(&mut self.string_repeated, &mut self._bitfield)
    }
    pub fn enum_unlabeled(&self) -> self::_root::full_coverage3::Enum {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            self::_root::full_coverage3::Enum,
            self::_pinternal::tags::Enum3::<self::_root::full_coverage3::Enum>,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.enum_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn enum_unlabeled_opt(
        &self,
    ) -> ::std::option::Option::<self::_root::full_coverage3::Enum> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            self::_root::full_coverage3::Enum,
            self::_pinternal::tags::Enum3::<self::_root::full_coverage3::Enum>,
        > as NonRepeatedFieldType>::get_field_opt(&self.enum_unlabeled, &self._bitfield)
    }
    pub fn enum_unlabeled_mut(&mut self) -> &mut self::_root::full_coverage3::Enum {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            self::_root::full_coverage3::Enum,
            self::_pinternal::tags::Enum3::<self::_root::full_coverage3::Enum>,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.enum_unlabeled,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_enum_unlabeled(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            self::_root::full_coverage3::Enum,
            self::_pinternal::tags::Enum3::<self::_root::full_coverage3::Enum>,
        > as NonRepeatedFieldType>::get_field_opt(&self.enum_unlabeled, &self._bitfield)
            .is_some()
    }
    pub fn clear_enum_unlabeled(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            self::_root::full_coverage3::Enum,
            self::_pinternal::tags::Enum3::<self::_root::full_coverage3::Enum>,
        > as NonRepeatedFieldType>::clear(&mut self.enum_unlabeled, &mut self._bitfield)
    }
    pub fn enum_optional(&self) -> self::_root::full_coverage3::Enum {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            self::_root::full_coverage3::Enum,
            self::_pinternal::tags::Enum3::<self::_root::full_coverage3::Enum>,
            4usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.enum_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn enum_optional_opt(
        &self,
    ) -> ::std::option::Option::<self::_root::full_coverage3::Enum> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            self::_root::full_coverage3::Enum,
            self::_pinternal::tags::Enum3::<self::_root::full_coverage3::Enum>,
            4usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.enum_optional, &self._bitfield)
    }
    pub fn enum_optional_mut(&mut self) -> &mut self::_root::full_coverage3::Enum {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            self::_root::full_coverage3::Enum,
            self::_pinternal::tags::Enum3::<self::_root::full_coverage3::Enum>,
            4usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.enum_optional,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_enum_optional(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            self::_root::full_coverage3::Enum,
            self::_pinternal::tags::Enum3::<self::_root::full_coverage3::Enum>,
            4usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.enum_optional, &self._bitfield)
            .is_some()
    }
    pub fn clear_enum_optional(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            self::_root::full_coverage3::Enum,
            self::_pinternal::tags::Enum3::<self::_root::full_coverage3::Enum>,
            4usize,
        > as NonRepeatedFieldType>::clear(&mut self.enum_optional, &mut self._bitfield)
    }
    pub fn enum_repeated(&self) -> &[self::_root::full_coverage3::Enum] {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedNumericalField::<
            self::_root::full_coverage3::Enum,
            self::_pinternal::tags::Enum3::<self::_root::full_coverage3::Enum>,
        > as RepeatedFieldType>::get_field(&self.enum_repeated, &self._bitfield)
    }
    pub fn enum_repeated_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::full_coverage3::Enum> {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedNumericalField::<
            self::_root::full_coverage3::Enum,
            self::_pinternal::tags::Enum3::<self::_root::full_coverage3::Enum>,
        > as RepeatedFieldType>::get_field_mut(
            &mut self.enum_repeated,
            &mut self._bitfield,
        )
    }
    pub fn clear_enum_repeated(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedNumericalField::<
            self::_root::full_coverage3::Enum,
            self::_pinternal::tags::Enum3::<self::_root::full_coverage3::Enum>,
        > as RepeatedFieldType>::clear(&mut self.enum_repeated, &mut self._bitfield)
    }
    pub fn submsg_unlabeled(
        &self,
    ) -> ::std::option::Option::<&self::_root::full_coverage3::msg::Submsg> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularHeapMessageField::<
            self::_root::full_coverage3::msg::Submsg,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.submsg_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn submsg_unlabeled_opt(
        &self,
    ) -> ::std::option::Option::<&self::_root::full_coverage3::msg::Submsg> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularHeapMessageField::<
            self::_root::full_coverage3::msg::Submsg,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.submsg_unlabeled,
            &self._bitfield,
        )
    }
    pub fn submsg_unlabeled_mut(
        &mut self,
    ) -> &mut self::_root::full_coverage3::msg::Submsg {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularHeapMessageField::<
            self::_root::full_coverage3::msg::Submsg,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.submsg_unlabeled,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_submsg_unlabeled(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularHeapMessageField::<
            self::_root::full_coverage3::msg::Submsg,
        > as NonRepeatedFieldType>::get_field_opt(
                &self.submsg_unlabeled,
                &self._bitfield,
            )
            .is_some()
    }
    pub fn clear_submsg_unlabeled(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularHeapMessageField::<
            self::_root::full_coverage3::msg::Submsg,
        > as NonRepeatedFieldType>::clear(
            &mut self.submsg_unlabeled,
            &mut self._bitfield,
        )
    }
    pub fn submsg_optional(
        &self,
    ) -> ::std::option::Option::<&self::_root::full_coverage3::msg::Submsg> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularHeapMessageField::<
            self::_root::full_coverage3::msg::Submsg,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.submsg_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn submsg_optional_opt(
        &self,
    ) -> ::std::option::Option::<&self::_root::full_coverage3::msg::Submsg> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularHeapMessageField::<
            self::_root::full_coverage3::msg::Submsg,
        > as NonRepeatedFieldType>::get_field_opt(&self.submsg_optional, &self._bitfield)
    }
    pub fn submsg_optional_mut(
        &mut self,
    ) -> &mut self::_root::full_coverage3::msg::Submsg {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularHeapMessageField::<
            self::_root::full_coverage3::msg::Submsg,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.submsg_optional,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_submsg_optional(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularHeapMessageField::<
            self::_root::full_coverage3::msg::Submsg,
        > as NonRepeatedFieldType>::get_field_opt(&self.submsg_optional, &self._bitfield)
            .is_some()
    }
    pub fn clear_submsg_optional(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularHeapMessageField::<
            self::_root::full_coverage3::msg::Submsg,
        > as NonRepeatedFieldType>::clear(&mut self.submsg_optional, &mut self._bitfield)
    }
    pub fn submsg_repeated(&self) -> &[self::_root::full_coverage3::msg::Submsg] {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedMessageField::<
            self::_root::full_coverage3::msg::Submsg,
        > as RepeatedFieldType>::get_field(&self.submsg_repeated, &self._bitfield)
    }
    pub fn submsg_repeated_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::full_coverage3::msg::Submsg> {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedMessageField::<
            self::_root::full_coverage3::msg::Submsg,
        > as RepeatedFieldType>::get_field_mut(
            &mut self.submsg_repeated,
            &mut self._bitfield,
        )
    }
    pub fn clear_submsg_repeated(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedMessageField::<
            self::_root::full_coverage3::msg::Submsg,
        > as RepeatedFieldType>::clear(&mut self.submsg_repeated, &mut self._bitfield)
    }
    pub fn i64_unlabeled(&self) -> i64 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            i64,
            self::_pinternal::tags::Int64,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.i64_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn i64_unlabeled_opt(&self) -> ::std::option::Option::<i64> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            i64,
            self::_pinternal::tags::Int64,
        > as NonRepeatedFieldType>::get_field_opt(&self.i64_unlabeled, &self._bitfield)
    }
    pub fn i64_unlabeled_mut(&mut self) -> &mut i64 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            i64,
            self::_pinternal::tags::Int64,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.i64_unlabeled,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_i64_unlabeled(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            i64,
            self::_pinternal::tags::Int64,
        > as NonRepeatedFieldType>::get_field_opt(&self.i64_unlabeled, &self._bitfield)
            .is_some()
    }
    pub fn clear_i64_unlabeled(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            i64,
            self::_pinternal::tags::Int64,
        > as NonRepeatedFieldType>::clear(&mut self.i64_unlabeled, &mut self._bitfield)
    }
    pub fn i64_optional(&self) -> i64 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i64,
            self::_pinternal::tags::Int64,
            5usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.i64_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn i64_optional_opt(&self) -> ::std::option::Option::<i64> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i64,
            self::_pinternal::tags::Int64,
            5usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.i64_optional, &self._bitfield)
    }
    pub fn i64_optional_mut(&mut self) -> &mut i64 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i64,
            self::_pinternal::tags::Int64,
            5usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.i64_optional,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_i64_optional(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i64,
            self::_pinternal::tags::Int64,
            5usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.i64_optional, &self._bitfield)
            .is_some()
    }
    pub fn clear_i64_optional(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i64,
            self::_pinternal::tags::Int64,
            5usize,
        > as NonRepeatedFieldType>::clear(&mut self.i64_optional, &mut self._bitfield)
    }
    pub fn i64_repeated(&self) -> &[i64] {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedNumericalField::<
            i64,
            self::_pinternal::tags::Int64,
        > as RepeatedFieldType>::get_field(&self.i64_repeated, &self._bitfield)
    }
    pub fn i64_repeated_mut(&mut self) -> &mut ::std::vec::Vec::<i64> {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedNumericalField::<
            i64,
            self::_pinternal::tags::Int64,
        > as RepeatedFieldType>::get_field_mut(
            &mut self.i64_repeated,
            &mut self._bitfield,
        )
    }
    pub fn clear_i64_repeated(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedNumericalField::<
            i64,
            self::_pinternal::tags::Int64,
        > as RepeatedFieldType>::clear(&mut self.i64_repeated, &mut self._bitfield)
    }
    pub fn u32_unlabeled(&self) -> u32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            u32,
            self::_pinternal::tags::UInt32,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.u32_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn u32_unlabeled_opt(&self) -> ::std::option::Option::<u32> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            u32,
            self::_pinternal::tags::UInt32,
        > as NonRepeatedFieldType>::get_field_opt(&self.u32_unlabeled, &self._bitfield)
    }
    pub fn u32_unlabeled_mut(&mut self) -> &mut u32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            u32,
            self::_pinternal::tags::UInt32,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.u32_unlabeled,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_u32_unlabeled(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            u32,
            self::_pinternal::tags::UInt32,
        > as NonRepeatedFieldType>::get_field_opt(&self.u32_unlabeled, &self._bitfield)
            .is_some()
    }
    pub fn clear_u32_unlabeled(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            u32,
            self::_pinternal::tags::UInt32,
        > as NonRepeatedFieldType>::clear(&mut self.u32_unlabeled, &mut self._bitfield)
    }
    pub fn u32_optional(&self) -> u32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            u32,
            self::_pinternal::tags::UInt32,
            6usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.u32_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn u32_optional_opt(&self) -> ::std::option::Option::<u32> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            u32,
            self::_pinternal::tags::UInt32,
            6usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.u32_optional, &self._bitfield)
    }
    pub fn u32_optional_mut(&mut self) -> &mut u32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            u32,
            self::_pinternal::tags::UInt32,
            6usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.u32_optional,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_u32_optional(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            u32,
            self::_pinternal::tags::UInt32,
            6usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.u32_optional, &self._bitfield)
            .is_some()
    }
    pub fn clear_u32_optional(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            u32,
            self::_pinternal::tags::UInt32,
            6usize,
        > as NonRepeatedFieldType>::clear(&mut self.u32_optional, &mut self._bitfield)
    }
    pub fn u32_repeated(&self) -> &[u32] {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedNumericalField::<
            u32,
            self::_pinternal::tags::UInt32,
        > as RepeatedFieldType>::get_field(&self.u32_repeated, &self._bitfield)
    }
    pub fn u32_repeated_mut(&mut self) -> &mut ::std::vec::Vec::<u32> {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedNumericalField::<
            u32,
            self::_pinternal::tags::UInt32,
        > as RepeatedFieldType>::get_field_mut(
            &mut self.u32_repeated,
            &mut self._bitfield,
        )
    }
    pub fn clear_u32_repeated(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedNumericalField::<
            u32,
            self::_pinternal::tags::UInt32,
        > as RepeatedFieldType>::clear(&mut self.u32_repeated, &mut self._bitfield)
    }
    pub fn u64_unlabeled(&self) -> u64 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            u64,
            self::_pinternal::tags::UInt64,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.u64_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn u64_unlabeled_opt(&self) -> ::std::option::Option::<u64> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            u64,
            self::_pinternal::tags::UInt64,
        > as NonRepeatedFieldType>::get_field_opt(&self.u64_unlabeled, &self._bitfield)
    }
    pub fn u64_unlabeled_mut(&mut self) -> &mut u64 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            u64,
            self::_pinternal::tags::UInt64,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.u64_unlabeled,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_u64_unlabeled(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            u64,
            self::_pinternal::tags::UInt64,
        > as NonRepeatedFieldType>::get_field_opt(&self.u64_unlabeled, &self._bitfield)
            .is_some()
    }
    pub fn clear_u64_unlabeled(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            u64,
            self::_pinternal::tags::UInt64,
        > as NonRepeatedFieldType>::clear(&mut self.u64_unlabeled, &mut self._bitfield)
    }
    pub fn u64_optional(&self) -> u64 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            u64,
            self::_pinternal::tags::UInt64,
            7usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.u64_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn u64_optional_opt(&self) -> ::std::option::Option::<u64> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            u64,
            self::_pinternal::tags::UInt64,
            7usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.u64_optional, &self._bitfield)
    }
    pub fn u64_optional_mut(&mut self) -> &mut u64 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            u64,
            self::_pinternal::tags::UInt64,
            7usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.u64_optional,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_u64_optional(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            u64,
            self::_pinternal::tags::UInt64,
            7usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.u64_optional, &self._bitfield)
            .is_some()
    }
    pub fn clear_u64_optional(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            u64,
            self::_pinternal::tags::UInt64,
            7usize,
        > as NonRepeatedFieldType>::clear(&mut self.u64_optional, &mut self._bitfield)
    }
    pub fn u64_repeated(&self) -> &[u64] {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedNumericalField::<
            u64,
            self::_pinternal::tags::UInt64,
        > as RepeatedFieldType>::get_field(&self.u64_repeated, &self._bitfield)
    }
    pub fn u64_repeated_mut(&mut self) -> &mut ::std::vec::Vec::<u64> {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedNumericalField::<
            u64,
            self::_pinternal::tags::UInt64,
        > as RepeatedFieldType>::get_field_mut(
            &mut self.u64_repeated,
            &mut self._bitfield,
        )
    }
    pub fn clear_u64_repeated(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedNumericalField::<
            u64,
            self::_pinternal::tags::UInt64,
        > as RepeatedFieldType>::clear(&mut self.u64_repeated, &mut self._bitfield)
    }
    pub fn s32_unlabeled(&self) -> i32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            i32,
            self::_pinternal::tags::SInt32,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.s32_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn s32_unlabeled_opt(&self) -> ::std::option::Option::<i32> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            i32,
            self::_pinternal::tags::SInt32,
        > as NonRepeatedFieldType>::get_field_opt(&self.s32_unlabeled, &self._bitfield)
    }
    pub fn s32_unlabeled_mut(&mut self) -> &mut i32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            i32,
            self::_pinternal::tags::SInt32,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.s32_unlabeled,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_s32_unlabeled(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            i32,
            self::_pinternal::tags::SInt32,
        > as NonRepeatedFieldType>::get_field_opt(&self.s32_unlabeled, &self._bitfield)
            .is_some()
    }
    pub fn clear_s32_unlabeled(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            i32,
            self::_pinternal::tags::SInt32,
        > as NonRepeatedFieldType>::clear(&mut self.s32_unlabeled, &mut self._bitfield)
    }
    pub fn s32_optional(&self) -> i32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::SInt32,
            8usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.s32_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn s32_optional_opt(&self) -> ::std::option::Option::<i32> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::SInt32,
            8usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.s32_optional, &self._bitfield)
    }
    pub fn s32_optional_mut(&mut self) -> &mut i32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::SInt32,
            8usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.s32_optional,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_s32_optional(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::SInt32,
            8usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.s32_optional, &self._bitfield)
            .is_some()
    }
    pub fn clear_s32_optional(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::SInt32,
            8usize,
        > as NonRepeatedFieldType>::clear(&mut self.s32_optional, &mut self._bitfield)
    }
    pub fn s32_repeated(&self) -> &[i32] {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedNumericalField::<
            i32,
            self::_pinternal::tags::SInt32,
        > as RepeatedFieldType>::get_field(&self.s32_repeated, &self._bitfield)
    }
    pub fn s32_repeated_mut(&mut self) -> &mut ::std::vec::Vec::<i32> {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedNumericalField::<
            i32,
            self::_pinternal::tags::SInt32,
        > as RepeatedFieldType>::get_field_mut(
            &mut self.s32_repeated,
            &mut self._bitfield,
        )
    }
    pub fn clear_s32_repeated(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedNumericalField::<
            i32,
            self::_pinternal::tags::SInt32,
        > as RepeatedFieldType>::clear(&mut self.s32_repeated, &mut self._bitfield)
    }
    pub fn s64_unlabeled(&self) -> i64 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            i64,
            self::_pinternal::tags::SInt64,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.s64_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn s64_unlabeled_opt(&self) -> ::std::option::Option::<i64> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            i64,
            self::_pinternal::tags::SInt64,
        > as NonRepeatedFieldType>::get_field_opt(&self.s64_unlabeled, &self._bitfield)
    }
    pub fn s64_unlabeled_mut(&mut self) -> &mut i64 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            i64,
            self::_pinternal::tags::SInt64,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.s64_unlabeled,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_s64_unlabeled(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            i64,
            self::_pinternal::tags::SInt64,
        > as NonRepeatedFieldType>::get_field_opt(&self.s64_unlabeled, &self._bitfield)
            .is_some()
    }
    pub fn clear_s64_unlabeled(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            i64,
            self::_pinternal::tags::SInt64,
        > as NonRepeatedFieldType>::clear(&mut self.s64_unlabeled, &mut self._bitfield)
    }
    pub fn s64_optional(&self) -> i64 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i64,
            self::_pinternal::tags::SInt64,
            9usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.s64_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn s64_optional_opt(&self) -> ::std::option::Option::<i64> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i64,
            self::_pinternal::tags::SInt64,
            9usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.s64_optional, &self._bitfield)
    }
    pub fn s64_optional_mut(&mut self) -> &mut i64 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i64,
            self::_pinternal::tags::SInt64,
            9usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.s64_optional,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_s64_optional(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i64,
            self::_pinternal::tags::SInt64,
            9usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.s64_optional, &self._bitfield)
            .is_some()
    }
    pub fn clear_s64_optional(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i64,
            self::_pinternal::tags::SInt64,
            9usize,
        > as NonRepeatedFieldType>::clear(&mut self.s64_optional, &mut self._bitfield)
    }
    pub fn s64_repeated(&self) -> &[i64] {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedNumericalField::<
            i64,
            self::_pinternal::tags::SInt64,
        > as RepeatedFieldType>::get_field(&self.s64_repeated, &self._bitfield)
    }
    pub fn s64_repeated_mut(&mut self) -> &mut ::std::vec::Vec::<i64> {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedNumericalField::<
            i64,
            self::_pinternal::tags::SInt64,
        > as RepeatedFieldType>::get_field_mut(
            &mut self.s64_repeated,
            &mut self._bitfield,
        )
    }
    pub fn clear_s64_repeated(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedNumericalField::<
            i64,
            self::_pinternal::tags::SInt64,
        > as RepeatedFieldType>::clear(&mut self.s64_repeated, &mut self._bitfield)
    }
    pub fn fixed32_unlabeled(&self) -> u32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            u32,
            self::_pinternal::tags::Fixed32,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.fixed32_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn fixed32_unlabeled_opt(&self) -> ::std::option::Option::<u32> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            u32,
            self::_pinternal::tags::Fixed32,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.fixed32_unlabeled,
            &self._bitfield,
        )
    }
    pub fn fixed32_unlabeled_mut(&mut self) -> &mut u32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            u32,
            self::_pinternal::tags::Fixed32,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.fixed32_unlabeled,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_fixed32_unlabeled(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            u32,
            self::_pinternal::tags::Fixed32,
        > as NonRepeatedFieldType>::get_field_opt(
                &self.fixed32_unlabeled,
                &self._bitfield,
            )
            .is_some()
    }
    pub fn clear_fixed32_unlabeled(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            u32,
            self::_pinternal::tags::Fixed32,
        > as NonRepeatedFieldType>::clear(
            &mut self.fixed32_unlabeled,
            &mut self._bitfield,
        )
    }
    pub fn fixed32_optional(&self) -> u32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            u32,
            self::_pinternal::tags::Fixed32,
            10usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.fixed32_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn fixed32_optional_opt(&self) -> ::std::option::Option::<u32> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            u32,
            self::_pinternal::tags::Fixed32,
            10usize,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.fixed32_optional,
            &self._bitfield,
        )
    }
    pub fn fixed32_optional_mut(&mut self) -> &mut u32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            u32,
            self::_pinternal::tags::Fixed32,
            10usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.fixed32_optional,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_fixed32_optional(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            u32,
            self::_pinternal::tags::Fixed32,
            10usize,
        > as NonRepeatedFieldType>::get_field_opt(
                &self.fixed32_optional,
                &self._bitfield,
            )
            .is_some()
    }
    pub fn clear_fixed32_optional(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            u32,
            self::_pinternal::tags::Fixed32,
            10usize,
        > as NonRepeatedFieldType>::clear(
            &mut self.fixed32_optional,
            &mut self._bitfield,
        )
    }
    pub fn fixed32_repeated(&self) -> &[u32] {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedNumericalField::<
            u32,
            self::_pinternal::tags::Fixed32,
        > as RepeatedFieldType>::get_field(&self.fixed32_repeated, &self._bitfield)
    }
    pub fn fixed32_repeated_mut(&mut self) -> &mut ::std::vec::Vec::<u32> {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedNumericalField::<
            u32,
            self::_pinternal::tags::Fixed32,
        > as RepeatedFieldType>::get_field_mut(
            &mut self.fixed32_repeated,
            &mut self._bitfield,
        )
    }
    pub fn clear_fixed32_repeated(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedNumericalField::<
            u32,
            self::_pinternal::tags::Fixed32,
        > as RepeatedFieldType>::clear(&mut self.fixed32_repeated, &mut self._bitfield)
    }
    pub fn fixed64_unlabeled(&self) -> u64 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            u64,
            self::_pinternal::tags::Fixed64,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.fixed64_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn fixed64_unlabeled_opt(&self) -> ::std::option::Option::<u64> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            u64,
            self::_pinternal::tags::Fixed64,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.fixed64_unlabeled,
            &self._bitfield,
        )
    }
    pub fn fixed64_unlabeled_mut(&mut self) -> &mut u64 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            u64,
            self::_pinternal::tags::Fixed64,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.fixed64_unlabeled,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_fixed64_unlabeled(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            u64,
            self::_pinternal::tags::Fixed64,
        > as NonRepeatedFieldType>::get_field_opt(
                &self.fixed64_unlabeled,
                &self._bitfield,
            )
            .is_some()
    }
    pub fn clear_fixed64_unlabeled(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            u64,
            self::_pinternal::tags::Fixed64,
        > as NonRepeatedFieldType>::clear(
            &mut self.fixed64_unlabeled,
            &mut self._bitfield,
        )
    }
    pub fn fixed64_optional(&self) -> u64 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            u64,
            self::_pinternal::tags::Fixed64,
            11usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.fixed64_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn fixed64_optional_opt(&self) -> ::std::option::Option::<u64> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            u64,
            self::_pinternal::tags::Fixed64,
            11usize,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.fixed64_optional,
            &self._bitfield,
        )
    }
    pub fn fixed64_optional_mut(&mut self) -> &mut u64 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            u64,
            self::_pinternal::tags::Fixed64,
            11usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.fixed64_optional,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_fixed64_optional(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            u64,
            self::_pinternal::tags::Fixed64,
            11usize,
        > as NonRepeatedFieldType>::get_field_opt(
                &self.fixed64_optional,
                &self._bitfield,
            )
            .is_some()
    }
    pub fn clear_fixed64_optional(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            u64,
            self::_pinternal::tags::Fixed64,
            11usize,
        > as NonRepeatedFieldType>::clear(
            &mut self.fixed64_optional,
            &mut self._bitfield,
        )
    }
    pub fn fixed64_repeated(&self) -> &[u64] {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedNumericalField::<
            u64,
            self::_pinternal::tags::Fixed64,
        > as RepeatedFieldType>::get_field(&self.fixed64_repeated, &self._bitfield)
    }
    pub fn fixed64_repeated_mut(&mut self) -> &mut ::std::vec::Vec::<u64> {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedNumericalField::<
            u64,
            self::_pinternal::tags::Fixed64,
        > as RepeatedFieldType>::get_field_mut(
            &mut self.fixed64_repeated,
            &mut self._bitfield,
        )
    }
    pub fn clear_fixed64_repeated(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedNumericalField::<
            u64,
            self::_pinternal::tags::Fixed64,
        > as RepeatedFieldType>::clear(&mut self.fixed64_repeated, &mut self._bitfield)
    }
    pub fn sfixed32_unlabeled(&self) -> i32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            i32,
            self::_pinternal::tags::SFixed32,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.sfixed32_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn sfixed32_unlabeled_opt(&self) -> ::std::option::Option::<i32> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            i32,
            self::_pinternal::tags::SFixed32,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.sfixed32_unlabeled,
            &self._bitfield,
        )
    }
    pub fn sfixed32_unlabeled_mut(&mut self) -> &mut i32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            i32,
            self::_pinternal::tags::SFixed32,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.sfixed32_unlabeled,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_sfixed32_unlabeled(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            i32,
            self::_pinternal::tags::SFixed32,
        > as NonRepeatedFieldType>::get_field_opt(
                &self.sfixed32_unlabeled,
                &self._bitfield,
            )
            .is_some()
    }
    pub fn clear_sfixed32_unlabeled(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            i32,
            self::_pinternal::tags::SFixed32,
        > as NonRepeatedFieldType>::clear(
            &mut self.sfixed32_unlabeled,
            &mut self._bitfield,
        )
    }
    pub fn sfixed32_optional(&self) -> i32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::SFixed32,
            12usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.sfixed32_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn sfixed32_optional_opt(&self) -> ::std::option::Option::<i32> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::SFixed32,
            12usize,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.sfixed32_optional,
            &self._bitfield,
        )
    }
    pub fn sfixed32_optional_mut(&mut self) -> &mut i32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::SFixed32,
            12usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.sfixed32_optional,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_sfixed32_optional(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::SFixed32,
            12usize,
        > as NonRepeatedFieldType>::get_field_opt(
                &self.sfixed32_optional,
                &self._bitfield,
            )
            .is_some()
    }
    pub fn clear_sfixed32_optional(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::SFixed32,
            12usize,
        > as NonRepeatedFieldType>::clear(
            &mut self.sfixed32_optional,
            &mut self._bitfield,
        )
    }
    pub fn sfixed32_repeated(&self) -> &[i32] {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedNumericalField::<
            i32,
            self::_pinternal::tags::SFixed32,
        > as RepeatedFieldType>::get_field(&self.sfixed32_repeated, &self._bitfield)
    }
    pub fn sfixed32_repeated_mut(&mut self) -> &mut ::std::vec::Vec::<i32> {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedNumericalField::<
            i32,
            self::_pinternal::tags::SFixed32,
        > as RepeatedFieldType>::get_field_mut(
            &mut self.sfixed32_repeated,
            &mut self._bitfield,
        )
    }
    pub fn clear_sfixed32_repeated(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedNumericalField::<
            i32,
            self::_pinternal::tags::SFixed32,
        > as RepeatedFieldType>::clear(&mut self.sfixed32_repeated, &mut self._bitfield)
    }
    pub fn sfixed64_unlabeled(&self) -> i64 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            i64,
            self::_pinternal::tags::SFixed64,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.sfixed64_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn sfixed64_unlabeled_opt(&self) -> ::std::option::Option::<i64> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            i64,
            self::_pinternal::tags::SFixed64,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.sfixed64_unlabeled,
            &self._bitfield,
        )
    }
    pub fn sfixed64_unlabeled_mut(&mut self) -> &mut i64 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            i64,
            self::_pinternal::tags::SFixed64,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.sfixed64_unlabeled,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_sfixed64_unlabeled(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            i64,
            self::_pinternal::tags::SFixed64,
        > as NonRepeatedFieldType>::get_field_opt(
                &self.sfixed64_unlabeled,
                &self._bitfield,
            )
            .is_some()
    }
    pub fn clear_sfixed64_unlabeled(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            i64,
            self::_pinternal::tags::SFixed64,
        > as NonRepeatedFieldType>::clear(
            &mut self.sfixed64_unlabeled,
            &mut self._bitfield,
        )
    }
    pub fn sfixed64_optional(&self) -> i64 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i64,
            self::_pinternal::tags::SFixed64,
            13usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.sfixed64_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn sfixed64_optional_opt(&self) -> ::std::option::Option::<i64> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i64,
            self::_pinternal::tags::SFixed64,
            13usize,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.sfixed64_optional,
            &self._bitfield,
        )
    }
    pub fn sfixed64_optional_mut(&mut self) -> &mut i64 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i64,
            self::_pinternal::tags::SFixed64,
            13usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.sfixed64_optional,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_sfixed64_optional(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i64,
            self::_pinternal::tags::SFixed64,
            13usize,
        > as NonRepeatedFieldType>::get_field_opt(
                &self.sfixed64_optional,
                &self._bitfield,
            )
            .is_some()
    }
    pub fn clear_sfixed64_optional(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i64,
            self::_pinternal::tags::SFixed64,
            13usize,
        > as NonRepeatedFieldType>::clear(
            &mut self.sfixed64_optional,
            &mut self._bitfield,
        )
    }
    pub fn sfixed64_repeated(&self) -> &[i64] {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedNumericalField::<
            i64,
            self::_pinternal::tags::SFixed64,
        > as RepeatedFieldType>::get_field(&self.sfixed64_repeated, &self._bitfield)
    }
    pub fn sfixed64_repeated_mut(&mut self) -> &mut ::std::vec::Vec::<i64> {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedNumericalField::<
            i64,
            self::_pinternal::tags::SFixed64,
        > as RepeatedFieldType>::get_field_mut(
            &mut self.sfixed64_repeated,
            &mut self._bitfield,
        )
    }
    pub fn clear_sfixed64_repeated(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedNumericalField::<
            i64,
            self::_pinternal::tags::SFixed64,
        > as RepeatedFieldType>::clear(&mut self.sfixed64_repeated, &mut self._bitfield)
    }
    pub fn f64_unlabeled(&self) -> f64 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            f64,
            self::_pinternal::tags::Double,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.f64_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn f64_unlabeled_opt(&self) -> ::std::option::Option::<f64> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            f64,
            self::_pinternal::tags::Double,
        > as NonRepeatedFieldType>::get_field_opt(&self.f64_unlabeled, &self._bitfield)
    }
    pub fn f64_unlabeled_mut(&mut self) -> &mut f64 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            f64,
            self::_pinternal::tags::Double,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.f64_unlabeled,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_f64_unlabeled(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            f64,
            self::_pinternal::tags::Double,
        > as NonRepeatedFieldType>::get_field_opt(&self.f64_unlabeled, &self._bitfield)
            .is_some()
    }
    pub fn clear_f64_unlabeled(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularNumericalField::<
            f64,
            self::_pinternal::tags::Double,
        > as NonRepeatedFieldType>::clear(&mut self.f64_unlabeled, &mut self._bitfield)
    }
    pub fn f64_optional(&self) -> f64 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            f64,
            self::_pinternal::tags::Double,
            14usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.f64_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn f64_optional_opt(&self) -> ::std::option::Option::<f64> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            f64,
            self::_pinternal::tags::Double,
            14usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.f64_optional, &self._bitfield)
    }
    pub fn f64_optional_mut(&mut self) -> &mut f64 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            f64,
            self::_pinternal::tags::Double,
            14usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.f64_optional,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_f64_optional(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            f64,
            self::_pinternal::tags::Double,
            14usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.f64_optional, &self._bitfield)
            .is_some()
    }
    pub fn clear_f64_optional(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            f64,
            self::_pinternal::tags::Double,
            14usize,
        > as NonRepeatedFieldType>::clear(&mut self.f64_optional, &mut self._bitfield)
    }
    pub fn f64_repeated(&self) -> &[f64] {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedNumericalField::<
            f64,
            self::_pinternal::tags::Double,
        > as RepeatedFieldType>::get_field(&self.f64_repeated, &self._bitfield)
    }
    pub fn f64_repeated_mut(&mut self) -> &mut ::std::vec::Vec::<f64> {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedNumericalField::<
            f64,
            self::_pinternal::tags::Double,
        > as RepeatedFieldType>::get_field_mut(
            &mut self.f64_repeated,
            &mut self._bitfield,
        )
    }
    pub fn clear_f64_repeated(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedNumericalField::<
            f64,
            self::_pinternal::tags::Double,
        > as RepeatedFieldType>::clear(&mut self.f64_repeated, &mut self._bitfield)
    }
}
impl self::_puroro::Message for Msg {
    fn from_bytes_iter<I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>>(
        iter: I,
    ) -> self::_puroro::Result<Self> {
        let mut msg = <Self as ::std::default::Default>::default();
        msg.merge_from_bytes_iter(iter)?;
        ::std::result::Result::Ok(msg)
    }
    fn merge_from_bytes_iter<I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>>(
        &mut self,
        mut iter: I,
    ) -> self::_puroro::Result<()> {
        use self::_pinternal::ser::FieldData;
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            match number {
                1i32 => {
                    <self::_pinternal::SingularNumericalField::<
                        i32,
                        self::_pinternal::tags::Int32,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.i32_unlabeled,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                2i32 => {
                    <self::_pinternal::OptionalNumericalField::<
                        i32,
                        self::_pinternal::tags::Int32,
                        0usize,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.i32_optional,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                3i32 => {
                    <self::_pinternal::RepeatedNumericalField::<
                        i32,
                        self::_pinternal::tags::Int32,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.i32_repeated,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                11i32 => {
                    <self::_pinternal::SingularNumericalField::<
                        f32,
                        self::_pinternal::tags::Float,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.float_unlabeled,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                12i32 => {
                    <self::_pinternal::OptionalNumericalField::<
                        f32,
                        self::_pinternal::tags::Float,
                        1usize,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.float_optional,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                13i32 => {
                    <self::_pinternal::RepeatedNumericalField::<
                        f32,
                        self::_pinternal::tags::Float,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.float_repeated,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                21i32 => {
                    <self::_pinternal::SingularUnsizedField::<
                        ::std::vec::Vec<u8>,
                        self::_pinternal::tags::Bytes,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.bytes_unlabeled,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                22i32 => {
                    <self::_pinternal::OptionalUnsizedField::<
                        ::std::vec::Vec<u8>,
                        self::_pinternal::tags::Bytes,
                        2usize,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.bytes_optional,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                23i32 => {
                    <self::_pinternal::RepeatedUnsizedField::<
                        ::std::vec::Vec<u8>,
                        self::_pinternal::tags::Bytes,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.bytes_repeated,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                31i32 => {
                    <self::_pinternal::SingularUnsizedField::<
                        ::std::string::String,
                        self::_pinternal::tags::String,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.string_unlabeled,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                32i32 => {
                    <self::_pinternal::OptionalUnsizedField::<
                        ::std::string::String,
                        self::_pinternal::tags::String,
                        3usize,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.string_optional,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                33i32 => {
                    <self::_pinternal::RepeatedUnsizedField::<
                        ::std::string::String,
                        self::_pinternal::tags::String,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.string_repeated,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                41i32 => {
                    <self::_pinternal::SingularNumericalField::<
                        self::_root::full_coverage3::Enum,
                        self::_pinternal::tags::Enum3::<
                            self::_root::full_coverage3::Enum,
                        >,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.enum_unlabeled,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                42i32 => {
                    <self::_pinternal::OptionalNumericalField::<
                        self::_root::full_coverage3::Enum,
                        self::_pinternal::tags::Enum3::<
                            self::_root::full_coverage3::Enum,
                        >,
                        4usize,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.enum_optional,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                43i32 => {
                    <self::_pinternal::RepeatedNumericalField::<
                        self::_root::full_coverage3::Enum,
                        self::_pinternal::tags::Enum3::<
                            self::_root::full_coverage3::Enum,
                        >,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.enum_repeated,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                51i32 => {
                    <self::_pinternal::SingularHeapMessageField::<
                        self::_root::full_coverage3::msg::Submsg,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.submsg_unlabeled,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                52i32 => {
                    <self::_pinternal::SingularHeapMessageField::<
                        self::_root::full_coverage3::msg::Submsg,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.submsg_optional,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                53i32 => {
                    <self::_pinternal::RepeatedMessageField::<
                        self::_root::full_coverage3::msg::Submsg,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.submsg_repeated,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                101i32 => {
                    <self::_pinternal::SingularNumericalField::<
                        i64,
                        self::_pinternal::tags::Int64,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.i64_unlabeled,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                102i32 => {
                    <self::_pinternal::OptionalNumericalField::<
                        i64,
                        self::_pinternal::tags::Int64,
                        5usize,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.i64_optional,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                103i32 => {
                    <self::_pinternal::RepeatedNumericalField::<
                        i64,
                        self::_pinternal::tags::Int64,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.i64_repeated,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                111i32 => {
                    <self::_pinternal::SingularNumericalField::<
                        u32,
                        self::_pinternal::tags::UInt32,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.u32_unlabeled,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                112i32 => {
                    <self::_pinternal::OptionalNumericalField::<
                        u32,
                        self::_pinternal::tags::UInt32,
                        6usize,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.u32_optional,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                113i32 => {
                    <self::_pinternal::RepeatedNumericalField::<
                        u32,
                        self::_pinternal::tags::UInt32,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.u32_repeated,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                121i32 => {
                    <self::_pinternal::SingularNumericalField::<
                        u64,
                        self::_pinternal::tags::UInt64,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.u64_unlabeled,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                122i32 => {
                    <self::_pinternal::OptionalNumericalField::<
                        u64,
                        self::_pinternal::tags::UInt64,
                        7usize,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.u64_optional,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                123i32 => {
                    <self::_pinternal::RepeatedNumericalField::<
                        u64,
                        self::_pinternal::tags::UInt64,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.u64_repeated,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                131i32 => {
                    <self::_pinternal::SingularNumericalField::<
                        i32,
                        self::_pinternal::tags::SInt32,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.s32_unlabeled,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                132i32 => {
                    <self::_pinternal::OptionalNumericalField::<
                        i32,
                        self::_pinternal::tags::SInt32,
                        8usize,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.s32_optional,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                133i32 => {
                    <self::_pinternal::RepeatedNumericalField::<
                        i32,
                        self::_pinternal::tags::SInt32,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.s32_repeated,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                141i32 => {
                    <self::_pinternal::SingularNumericalField::<
                        i64,
                        self::_pinternal::tags::SInt64,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.s64_unlabeled,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                142i32 => {
                    <self::_pinternal::OptionalNumericalField::<
                        i64,
                        self::_pinternal::tags::SInt64,
                        9usize,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.s64_optional,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                143i32 => {
                    <self::_pinternal::RepeatedNumericalField::<
                        i64,
                        self::_pinternal::tags::SInt64,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.s64_repeated,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                151i32 => {
                    <self::_pinternal::SingularNumericalField::<
                        u32,
                        self::_pinternal::tags::Fixed32,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.fixed32_unlabeled,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                152i32 => {
                    <self::_pinternal::OptionalNumericalField::<
                        u32,
                        self::_pinternal::tags::Fixed32,
                        10usize,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.fixed32_optional,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                153i32 => {
                    <self::_pinternal::RepeatedNumericalField::<
                        u32,
                        self::_pinternal::tags::Fixed32,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.fixed32_repeated,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                161i32 => {
                    <self::_pinternal::SingularNumericalField::<
                        u64,
                        self::_pinternal::tags::Fixed64,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.fixed64_unlabeled,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                162i32 => {
                    <self::_pinternal::OptionalNumericalField::<
                        u64,
                        self::_pinternal::tags::Fixed64,
                        11usize,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.fixed64_optional,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                163i32 => {
                    <self::_pinternal::RepeatedNumericalField::<
                        u64,
                        self::_pinternal::tags::Fixed64,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.fixed64_repeated,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                171i32 => {
                    <self::_pinternal::SingularNumericalField::<
                        i32,
                        self::_pinternal::tags::SFixed32,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.sfixed32_unlabeled,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                172i32 => {
                    <self::_pinternal::OptionalNumericalField::<
                        i32,
                        self::_pinternal::tags::SFixed32,
                        12usize,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.sfixed32_optional,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                173i32 => {
                    <self::_pinternal::RepeatedNumericalField::<
                        i32,
                        self::_pinternal::tags::SFixed32,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.sfixed32_repeated,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                181i32 => {
                    <self::_pinternal::SingularNumericalField::<
                        i64,
                        self::_pinternal::tags::SFixed64,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.sfixed64_unlabeled,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                182i32 => {
                    <self::_pinternal::OptionalNumericalField::<
                        i64,
                        self::_pinternal::tags::SFixed64,
                        13usize,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.sfixed64_optional,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                183i32 => {
                    <self::_pinternal::RepeatedNumericalField::<
                        i64,
                        self::_pinternal::tags::SFixed64,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.sfixed64_repeated,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                191i32 => {
                    <self::_pinternal::SingularNumericalField::<
                        f64,
                        self::_pinternal::tags::Double,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.f64_unlabeled,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                192i32 => {
                    <self::_pinternal::OptionalNumericalField::<
                        f64,
                        self::_pinternal::tags::Double,
                        14usize,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.f64_optional,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                193i32 => {
                    <self::_pinternal::RepeatedNumericalField::<
                        f64,
                        self::_pinternal::tags::Double,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.f64_repeated,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                _ => todo!(),
            }
        }
        ::std::result::Result::Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        <self::_pinternal::SingularNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.i32_unlabeled,
            &self._bitfield,
            1i32,
            out,
        )?;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.i32_optional,
            &self._bitfield,
            2i32,
            out,
        )?;
        <self::_pinternal::RepeatedNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.i32_repeated,
            &self._bitfield,
            3i32,
            out,
        )?;
        <self::_pinternal::SingularNumericalField::<
            f32,
            self::_pinternal::tags::Float,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.float_unlabeled,
            &self._bitfield,
            11i32,
            out,
        )?;
        <self::_pinternal::OptionalNumericalField::<
            f32,
            self::_pinternal::tags::Float,
            1usize,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.float_optional,
            &self._bitfield,
            12i32,
            out,
        )?;
        <self::_pinternal::RepeatedNumericalField::<
            f32,
            self::_pinternal::tags::Float,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.float_repeated,
            &self._bitfield,
            13i32,
            out,
        )?;
        <self::_pinternal::SingularUnsizedField::<
            ::std::vec::Vec<u8>,
            self::_pinternal::tags::Bytes,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.bytes_unlabeled,
            &self._bitfield,
            21i32,
            out,
        )?;
        <self::_pinternal::OptionalUnsizedField::<
            ::std::vec::Vec<u8>,
            self::_pinternal::tags::Bytes,
            2usize,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.bytes_optional,
            &self._bitfield,
            22i32,
            out,
        )?;
        <self::_pinternal::RepeatedUnsizedField::<
            ::std::vec::Vec<u8>,
            self::_pinternal::tags::Bytes,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.bytes_repeated,
            &self._bitfield,
            23i32,
            out,
        )?;
        <self::_pinternal::SingularUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.string_unlabeled,
            &self._bitfield,
            31i32,
            out,
        )?;
        <self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            3usize,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.string_optional,
            &self._bitfield,
            32i32,
            out,
        )?;
        <self::_pinternal::RepeatedUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.string_repeated,
            &self._bitfield,
            33i32,
            out,
        )?;
        <self::_pinternal::SingularNumericalField::<
            self::_root::full_coverage3::Enum,
            self::_pinternal::tags::Enum3::<self::_root::full_coverage3::Enum>,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.enum_unlabeled,
            &self._bitfield,
            41i32,
            out,
        )?;
        <self::_pinternal::OptionalNumericalField::<
            self::_root::full_coverage3::Enum,
            self::_pinternal::tags::Enum3::<self::_root::full_coverage3::Enum>,
            4usize,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.enum_optional,
            &self._bitfield,
            42i32,
            out,
        )?;
        <self::_pinternal::RepeatedNumericalField::<
            self::_root::full_coverage3::Enum,
            self::_pinternal::tags::Enum3::<self::_root::full_coverage3::Enum>,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.enum_repeated,
            &self._bitfield,
            43i32,
            out,
        )?;
        <self::_pinternal::SingularHeapMessageField::<
            self::_root::full_coverage3::msg::Submsg,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.submsg_unlabeled,
            &self._bitfield,
            51i32,
            out,
        )?;
        <self::_pinternal::SingularHeapMessageField::<
            self::_root::full_coverage3::msg::Submsg,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.submsg_optional,
            &self._bitfield,
            52i32,
            out,
        )?;
        <self::_pinternal::RepeatedMessageField::<
            self::_root::full_coverage3::msg::Submsg,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.submsg_repeated,
            &self._bitfield,
            53i32,
            out,
        )?;
        <self::_pinternal::SingularNumericalField::<
            i64,
            self::_pinternal::tags::Int64,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.i64_unlabeled,
            &self._bitfield,
            101i32,
            out,
        )?;
        <self::_pinternal::OptionalNumericalField::<
            i64,
            self::_pinternal::tags::Int64,
            5usize,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.i64_optional,
            &self._bitfield,
            102i32,
            out,
        )?;
        <self::_pinternal::RepeatedNumericalField::<
            i64,
            self::_pinternal::tags::Int64,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.i64_repeated,
            &self._bitfield,
            103i32,
            out,
        )?;
        <self::_pinternal::SingularNumericalField::<
            u32,
            self::_pinternal::tags::UInt32,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.u32_unlabeled,
            &self._bitfield,
            111i32,
            out,
        )?;
        <self::_pinternal::OptionalNumericalField::<
            u32,
            self::_pinternal::tags::UInt32,
            6usize,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.u32_optional,
            &self._bitfield,
            112i32,
            out,
        )?;
        <self::_pinternal::RepeatedNumericalField::<
            u32,
            self::_pinternal::tags::UInt32,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.u32_repeated,
            &self._bitfield,
            113i32,
            out,
        )?;
        <self::_pinternal::SingularNumericalField::<
            u64,
            self::_pinternal::tags::UInt64,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.u64_unlabeled,
            &self._bitfield,
            121i32,
            out,
        )?;
        <self::_pinternal::OptionalNumericalField::<
            u64,
            self::_pinternal::tags::UInt64,
            7usize,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.u64_optional,
            &self._bitfield,
            122i32,
            out,
        )?;
        <self::_pinternal::RepeatedNumericalField::<
            u64,
            self::_pinternal::tags::UInt64,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.u64_repeated,
            &self._bitfield,
            123i32,
            out,
        )?;
        <self::_pinternal::SingularNumericalField::<
            i32,
            self::_pinternal::tags::SInt32,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.s32_unlabeled,
            &self._bitfield,
            131i32,
            out,
        )?;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::SInt32,
            8usize,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.s32_optional,
            &self._bitfield,
            132i32,
            out,
        )?;
        <self::_pinternal::RepeatedNumericalField::<
            i32,
            self::_pinternal::tags::SInt32,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.s32_repeated,
            &self._bitfield,
            133i32,
            out,
        )?;
        <self::_pinternal::SingularNumericalField::<
            i64,
            self::_pinternal::tags::SInt64,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.s64_unlabeled,
            &self._bitfield,
            141i32,
            out,
        )?;
        <self::_pinternal::OptionalNumericalField::<
            i64,
            self::_pinternal::tags::SInt64,
            9usize,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.s64_optional,
            &self._bitfield,
            142i32,
            out,
        )?;
        <self::_pinternal::RepeatedNumericalField::<
            i64,
            self::_pinternal::tags::SInt64,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.s64_repeated,
            &self._bitfield,
            143i32,
            out,
        )?;
        <self::_pinternal::SingularNumericalField::<
            u32,
            self::_pinternal::tags::Fixed32,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.fixed32_unlabeled,
            &self._bitfield,
            151i32,
            out,
        )?;
        <self::_pinternal::OptionalNumericalField::<
            u32,
            self::_pinternal::tags::Fixed32,
            10usize,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.fixed32_optional,
            &self._bitfield,
            152i32,
            out,
        )?;
        <self::_pinternal::RepeatedNumericalField::<
            u32,
            self::_pinternal::tags::Fixed32,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.fixed32_repeated,
            &self._bitfield,
            153i32,
            out,
        )?;
        <self::_pinternal::SingularNumericalField::<
            u64,
            self::_pinternal::tags::Fixed64,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.fixed64_unlabeled,
            &self._bitfield,
            161i32,
            out,
        )?;
        <self::_pinternal::OptionalNumericalField::<
            u64,
            self::_pinternal::tags::Fixed64,
            11usize,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.fixed64_optional,
            &self._bitfield,
            162i32,
            out,
        )?;
        <self::_pinternal::RepeatedNumericalField::<
            u64,
            self::_pinternal::tags::Fixed64,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.fixed64_repeated,
            &self._bitfield,
            163i32,
            out,
        )?;
        <self::_pinternal::SingularNumericalField::<
            i32,
            self::_pinternal::tags::SFixed32,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.sfixed32_unlabeled,
            &self._bitfield,
            171i32,
            out,
        )?;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::SFixed32,
            12usize,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.sfixed32_optional,
            &self._bitfield,
            172i32,
            out,
        )?;
        <self::_pinternal::RepeatedNumericalField::<
            i32,
            self::_pinternal::tags::SFixed32,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.sfixed32_repeated,
            &self._bitfield,
            173i32,
            out,
        )?;
        <self::_pinternal::SingularNumericalField::<
            i64,
            self::_pinternal::tags::SFixed64,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.sfixed64_unlabeled,
            &self._bitfield,
            181i32,
            out,
        )?;
        <self::_pinternal::OptionalNumericalField::<
            i64,
            self::_pinternal::tags::SFixed64,
            13usize,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.sfixed64_optional,
            &self._bitfield,
            182i32,
            out,
        )?;
        <self::_pinternal::RepeatedNumericalField::<
            i64,
            self::_pinternal::tags::SFixed64,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.sfixed64_repeated,
            &self._bitfield,
            183i32,
            out,
        )?;
        <self::_pinternal::SingularNumericalField::<
            f64,
            self::_pinternal::tags::Double,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.f64_unlabeled,
            &self._bitfield,
            191i32,
            out,
        )?;
        <self::_pinternal::OptionalNumericalField::<
            f64,
            self::_pinternal::tags::Double,
            14usize,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.f64_optional,
            &self._bitfield,
            192i32,
            out,
        )?;
        <self::_pinternal::RepeatedNumericalField::<
            f64,
            self::_pinternal::tags::Double,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.f64_repeated,
            &self._bitfield,
            193i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for Msg {
    fn clone(&self) -> Self {
        Self {
            i32_unlabeled: <self::_pinternal::SingularNumericalField::<
                i32,
                self::_pinternal::tags::Int32,
            > as ::std::clone::Clone>::clone(&self.i32_unlabeled),
            i32_optional: <self::_pinternal::OptionalNumericalField::<
                i32,
                self::_pinternal::tags::Int32,
                0usize,
            > as ::std::clone::Clone>::clone(&self.i32_optional),
            i32_repeated: <self::_pinternal::RepeatedNumericalField::<
                i32,
                self::_pinternal::tags::Int32,
            > as ::std::clone::Clone>::clone(&self.i32_repeated),
            float_unlabeled: <self::_pinternal::SingularNumericalField::<
                f32,
                self::_pinternal::tags::Float,
            > as ::std::clone::Clone>::clone(&self.float_unlabeled),
            float_optional: <self::_pinternal::OptionalNumericalField::<
                f32,
                self::_pinternal::tags::Float,
                1usize,
            > as ::std::clone::Clone>::clone(&self.float_optional),
            float_repeated: <self::_pinternal::RepeatedNumericalField::<
                f32,
                self::_pinternal::tags::Float,
            > as ::std::clone::Clone>::clone(&self.float_repeated),
            bytes_unlabeled: <self::_pinternal::SingularUnsizedField::<
                ::std::vec::Vec<u8>,
                self::_pinternal::tags::Bytes,
            > as ::std::clone::Clone>::clone(&self.bytes_unlabeled),
            bytes_optional: <self::_pinternal::OptionalUnsizedField::<
                ::std::vec::Vec<u8>,
                self::_pinternal::tags::Bytes,
                2usize,
            > as ::std::clone::Clone>::clone(&self.bytes_optional),
            bytes_repeated: <self::_pinternal::RepeatedUnsizedField::<
                ::std::vec::Vec<u8>,
                self::_pinternal::tags::Bytes,
            > as ::std::clone::Clone>::clone(&self.bytes_repeated),
            string_unlabeled: <self::_pinternal::SingularUnsizedField::<
                ::std::string::String,
                self::_pinternal::tags::String,
            > as ::std::clone::Clone>::clone(&self.string_unlabeled),
            string_optional: <self::_pinternal::OptionalUnsizedField::<
                ::std::string::String,
                self::_pinternal::tags::String,
                3usize,
            > as ::std::clone::Clone>::clone(&self.string_optional),
            string_repeated: <self::_pinternal::RepeatedUnsizedField::<
                ::std::string::String,
                self::_pinternal::tags::String,
            > as ::std::clone::Clone>::clone(&self.string_repeated),
            enum_unlabeled: <self::_pinternal::SingularNumericalField::<
                self::_root::full_coverage3::Enum,
                self::_pinternal::tags::Enum3::<self::_root::full_coverage3::Enum>,
            > as ::std::clone::Clone>::clone(&self.enum_unlabeled),
            enum_optional: <self::_pinternal::OptionalNumericalField::<
                self::_root::full_coverage3::Enum,
                self::_pinternal::tags::Enum3::<self::_root::full_coverage3::Enum>,
                4usize,
            > as ::std::clone::Clone>::clone(&self.enum_optional),
            enum_repeated: <self::_pinternal::RepeatedNumericalField::<
                self::_root::full_coverage3::Enum,
                self::_pinternal::tags::Enum3::<self::_root::full_coverage3::Enum>,
            > as ::std::clone::Clone>::clone(&self.enum_repeated),
            submsg_unlabeled: <self::_pinternal::SingularHeapMessageField::<
                self::_root::full_coverage3::msg::Submsg,
            > as ::std::clone::Clone>::clone(&self.submsg_unlabeled),
            submsg_optional: <self::_pinternal::SingularHeapMessageField::<
                self::_root::full_coverage3::msg::Submsg,
            > as ::std::clone::Clone>::clone(&self.submsg_optional),
            submsg_repeated: <self::_pinternal::RepeatedMessageField::<
                self::_root::full_coverage3::msg::Submsg,
            > as ::std::clone::Clone>::clone(&self.submsg_repeated),
            i64_unlabeled: <self::_pinternal::SingularNumericalField::<
                i64,
                self::_pinternal::tags::Int64,
            > as ::std::clone::Clone>::clone(&self.i64_unlabeled),
            i64_optional: <self::_pinternal::OptionalNumericalField::<
                i64,
                self::_pinternal::tags::Int64,
                5usize,
            > as ::std::clone::Clone>::clone(&self.i64_optional),
            i64_repeated: <self::_pinternal::RepeatedNumericalField::<
                i64,
                self::_pinternal::tags::Int64,
            > as ::std::clone::Clone>::clone(&self.i64_repeated),
            u32_unlabeled: <self::_pinternal::SingularNumericalField::<
                u32,
                self::_pinternal::tags::UInt32,
            > as ::std::clone::Clone>::clone(&self.u32_unlabeled),
            u32_optional: <self::_pinternal::OptionalNumericalField::<
                u32,
                self::_pinternal::tags::UInt32,
                6usize,
            > as ::std::clone::Clone>::clone(&self.u32_optional),
            u32_repeated: <self::_pinternal::RepeatedNumericalField::<
                u32,
                self::_pinternal::tags::UInt32,
            > as ::std::clone::Clone>::clone(&self.u32_repeated),
            u64_unlabeled: <self::_pinternal::SingularNumericalField::<
                u64,
                self::_pinternal::tags::UInt64,
            > as ::std::clone::Clone>::clone(&self.u64_unlabeled),
            u64_optional: <self::_pinternal::OptionalNumericalField::<
                u64,
                self::_pinternal::tags::UInt64,
                7usize,
            > as ::std::clone::Clone>::clone(&self.u64_optional),
            u64_repeated: <self::_pinternal::RepeatedNumericalField::<
                u64,
                self::_pinternal::tags::UInt64,
            > as ::std::clone::Clone>::clone(&self.u64_repeated),
            s32_unlabeled: <self::_pinternal::SingularNumericalField::<
                i32,
                self::_pinternal::tags::SInt32,
            > as ::std::clone::Clone>::clone(&self.s32_unlabeled),
            s32_optional: <self::_pinternal::OptionalNumericalField::<
                i32,
                self::_pinternal::tags::SInt32,
                8usize,
            > as ::std::clone::Clone>::clone(&self.s32_optional),
            s32_repeated: <self::_pinternal::RepeatedNumericalField::<
                i32,
                self::_pinternal::tags::SInt32,
            > as ::std::clone::Clone>::clone(&self.s32_repeated),
            s64_unlabeled: <self::_pinternal::SingularNumericalField::<
                i64,
                self::_pinternal::tags::SInt64,
            > as ::std::clone::Clone>::clone(&self.s64_unlabeled),
            s64_optional: <self::_pinternal::OptionalNumericalField::<
                i64,
                self::_pinternal::tags::SInt64,
                9usize,
            > as ::std::clone::Clone>::clone(&self.s64_optional),
            s64_repeated: <self::_pinternal::RepeatedNumericalField::<
                i64,
                self::_pinternal::tags::SInt64,
            > as ::std::clone::Clone>::clone(&self.s64_repeated),
            fixed32_unlabeled: <self::_pinternal::SingularNumericalField::<
                u32,
                self::_pinternal::tags::Fixed32,
            > as ::std::clone::Clone>::clone(&self.fixed32_unlabeled),
            fixed32_optional: <self::_pinternal::OptionalNumericalField::<
                u32,
                self::_pinternal::tags::Fixed32,
                10usize,
            > as ::std::clone::Clone>::clone(&self.fixed32_optional),
            fixed32_repeated: <self::_pinternal::RepeatedNumericalField::<
                u32,
                self::_pinternal::tags::Fixed32,
            > as ::std::clone::Clone>::clone(&self.fixed32_repeated),
            fixed64_unlabeled: <self::_pinternal::SingularNumericalField::<
                u64,
                self::_pinternal::tags::Fixed64,
            > as ::std::clone::Clone>::clone(&self.fixed64_unlabeled),
            fixed64_optional: <self::_pinternal::OptionalNumericalField::<
                u64,
                self::_pinternal::tags::Fixed64,
                11usize,
            > as ::std::clone::Clone>::clone(&self.fixed64_optional),
            fixed64_repeated: <self::_pinternal::RepeatedNumericalField::<
                u64,
                self::_pinternal::tags::Fixed64,
            > as ::std::clone::Clone>::clone(&self.fixed64_repeated),
            sfixed32_unlabeled: <self::_pinternal::SingularNumericalField::<
                i32,
                self::_pinternal::tags::SFixed32,
            > as ::std::clone::Clone>::clone(&self.sfixed32_unlabeled),
            sfixed32_optional: <self::_pinternal::OptionalNumericalField::<
                i32,
                self::_pinternal::tags::SFixed32,
                12usize,
            > as ::std::clone::Clone>::clone(&self.sfixed32_optional),
            sfixed32_repeated: <self::_pinternal::RepeatedNumericalField::<
                i32,
                self::_pinternal::tags::SFixed32,
            > as ::std::clone::Clone>::clone(&self.sfixed32_repeated),
            sfixed64_unlabeled: <self::_pinternal::SingularNumericalField::<
                i64,
                self::_pinternal::tags::SFixed64,
            > as ::std::clone::Clone>::clone(&self.sfixed64_unlabeled),
            sfixed64_optional: <self::_pinternal::OptionalNumericalField::<
                i64,
                self::_pinternal::tags::SFixed64,
                13usize,
            > as ::std::clone::Clone>::clone(&self.sfixed64_optional),
            sfixed64_repeated: <self::_pinternal::RepeatedNumericalField::<
                i64,
                self::_pinternal::tags::SFixed64,
            > as ::std::clone::Clone>::clone(&self.sfixed64_repeated),
            f64_unlabeled: <self::_pinternal::SingularNumericalField::<
                f64,
                self::_pinternal::tags::Double,
            > as ::std::clone::Clone>::clone(&self.f64_unlabeled),
            f64_optional: <self::_pinternal::OptionalNumericalField::<
                f64,
                self::_pinternal::tags::Double,
                14usize,
            > as ::std::clone::Clone>::clone(&self.f64_optional),
            f64_repeated: <self::_pinternal::RepeatedNumericalField::<
                f64,
                self::_pinternal::tags::Double,
            > as ::std::clone::Clone>::clone(&self.f64_repeated),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
impl ::std::ops::Drop for Msg {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for Msg {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct(stringify!(Msg))
            .field(stringify!(i32_unlabeled), &self.i32_unlabeled_opt())
            .field(stringify!(i32_optional), &self.i32_optional_opt())
            .field(stringify!(i32_repeated), &self.i32_repeated())
            .field(stringify!(float_unlabeled), &self.float_unlabeled_opt())
            .field(stringify!(float_optional), &self.float_optional_opt())
            .field(stringify!(float_repeated), &self.float_repeated())
            .field(stringify!(bytes_unlabeled), &self.bytes_unlabeled_opt())
            .field(stringify!(bytes_optional), &self.bytes_optional_opt())
            .field(stringify!(bytes_repeated), &self.bytes_repeated())
            .field(stringify!(string_unlabeled), &self.string_unlabeled_opt())
            .field(stringify!(string_optional), &self.string_optional_opt())
            .field(stringify!(string_repeated), &self.string_repeated())
            .field(stringify!(enum_unlabeled), &self.enum_unlabeled_opt())
            .field(stringify!(enum_optional), &self.enum_optional_opt())
            .field(stringify!(enum_repeated), &self.enum_repeated())
            .field(stringify!(submsg_unlabeled), &self.submsg_unlabeled_opt())
            .field(stringify!(submsg_optional), &self.submsg_optional_opt())
            .field(stringify!(submsg_repeated), &self.submsg_repeated())
            .field(stringify!(i64_unlabeled), &self.i64_unlabeled_opt())
            .field(stringify!(i64_optional), &self.i64_optional_opt())
            .field(stringify!(i64_repeated), &self.i64_repeated())
            .field(stringify!(u32_unlabeled), &self.u32_unlabeled_opt())
            .field(stringify!(u32_optional), &self.u32_optional_opt())
            .field(stringify!(u32_repeated), &self.u32_repeated())
            .field(stringify!(u64_unlabeled), &self.u64_unlabeled_opt())
            .field(stringify!(u64_optional), &self.u64_optional_opt())
            .field(stringify!(u64_repeated), &self.u64_repeated())
            .field(stringify!(s32_unlabeled), &self.s32_unlabeled_opt())
            .field(stringify!(s32_optional), &self.s32_optional_opt())
            .field(stringify!(s32_repeated), &self.s32_repeated())
            .field(stringify!(s64_unlabeled), &self.s64_unlabeled_opt())
            .field(stringify!(s64_optional), &self.s64_optional_opt())
            .field(stringify!(s64_repeated), &self.s64_repeated())
            .field(stringify!(fixed32_unlabeled), &self.fixed32_unlabeled_opt())
            .field(stringify!(fixed32_optional), &self.fixed32_optional_opt())
            .field(stringify!(fixed32_repeated), &self.fixed32_repeated())
            .field(stringify!(fixed64_unlabeled), &self.fixed64_unlabeled_opt())
            .field(stringify!(fixed64_optional), &self.fixed64_optional_opt())
            .field(stringify!(fixed64_repeated), &self.fixed64_repeated())
            .field(stringify!(sfixed32_unlabeled), &self.sfixed32_unlabeled_opt())
            .field(stringify!(sfixed32_optional), &self.sfixed32_optional_opt())
            .field(stringify!(sfixed32_repeated), &self.sfixed32_repeated())
            .field(stringify!(sfixed64_unlabeled), &self.sfixed64_unlabeled_opt())
            .field(stringify!(sfixed64_optional), &self.sfixed64_optional_opt())
            .field(stringify!(sfixed64_repeated), &self.sfixed64_repeated())
            .field(stringify!(f64_unlabeled), &self.f64_unlabeled_opt())
            .field(stringify!(f64_optional), &self.f64_optional_opt())
            .field(stringify!(f64_repeated), &self.f64_repeated())
            .finish()
    }
}
impl ::std::cmp::PartialEq for Msg {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
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
pub mod _fields {
    mod _root {
        #[allow(unused)]
        pub use super::super::_root::*;
    }
    mod _puroro {
        #[allow(unused)]
        pub use ::puroro::*;
    }
    mod _pinternal {
        #[allow(unused)]
        pub use ::puroro::internal::*;
    }
    pub struct MsgFields {
        pub foo: i32,
    }
}
pub use self::_fields::*;
#[derive(
    ::std::clone::Clone,
    ::std::marker::Copy,
    ::std::cmp::PartialEq,
    ::std::cmp::Eq,
    ::std::cmp::PartialOrd,
    ::std::cmp::Ord,
    ::std::hash::Hash,
    ::std::fmt::Debug,
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
impl ::std::convert::From::<Enum> for i32 {
    fn from(val: Enum) -> i32 {
        match val {
            Enum::Zeroth => 0i32,
            Enum::First => 1i32,
            Enum::Tenth => 10i32,
            self::Enum::_None(i) => i,
        }
    }
}
impl ::std::convert::From::<i32> for Enum {
    fn from(val: i32) -> Self {
        match val {
            0i32 => self::Enum::Zeroth,
            1i32 => self::Enum::First,
            10i32 => self::Enum::Tenth,
            _ => Enum::_None(val),
        }
    }
}
