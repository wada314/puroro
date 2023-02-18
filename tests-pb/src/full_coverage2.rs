mod _root {
    #[allow(unused)]
    pub(crate) use super::super::_root::*;
}
mod _puroro {
    #[allow(unused)]
    pub(crate) use super::_root::_puroro::*;
}
mod _pinternal {
    #[allow(unused)]
    pub(crate) use super::_root::_pinternal::*;
}
pub mod msg;
#[derive(::std::default::Default)]
pub struct Msg {
    view: self::_root::full_coverage2::_view::MsgView,
}
impl Msg {
    pub fn i32_required_mut(&mut self) -> &mut i32 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.view.fields.i32_required,
            self.view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_i32_required(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(
            &mut self.view.fields.i32_required,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn i32_optional_mut(&mut self) -> &mut i32 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.view.fields.i32_optional,
            self.view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_i32_optional(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(
            &mut self.view.fields.i32_optional,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn i32_repeated_mut(&mut self) -> &mut ::std::vec::Vec::<i32> {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.view.fields.i32_repeated,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn clear_i32_repeated(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(
            &mut self.view.fields.i32_repeated,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn float_required_mut(&mut self) -> &mut f32 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.view.fields.float_required,
            self.view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_float_required(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(
            &mut self.view.fields.float_required,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn float_optional_mut(&mut self) -> &mut f32 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.view.fields.float_optional,
            self.view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_float_optional(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(
            &mut self.view.fields.float_optional,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn float_repeated_mut(&mut self) -> &mut ::std::vec::Vec::<f32> {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.view.fields.float_repeated,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn clear_float_repeated(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(
            &mut self.view.fields.float_repeated,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn bytes_required_mut(&mut self) -> &mut ::std::vec::Vec::<u8> {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.view.fields.bytes_required,
            self.view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_bytes_required(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(
            &mut self.view.fields.bytes_required,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn bytes_optional_mut(&mut self) -> &mut ::std::vec::Vec::<u8> {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.view.fields.bytes_optional,
            self.view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_bytes_optional(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(
            &mut self.view.fields.bytes_optional,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn bytes_repeated_mut(&mut self) -> &mut ::std::vec::Vec::<::std::vec::Vec<u8>> {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.view.fields.bytes_repeated,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn clear_bytes_repeated(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(
            &mut self.view.fields.bytes_repeated,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn string_required_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.view.fields.string_required,
            self.view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_string_required(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(
            &mut self.view.fields.string_required,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn string_optional_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.view.fields.string_optional,
            self.view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_string_optional(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(
            &mut self.view.fields.string_optional,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn string_repeated_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<::std::string::String> {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.view.fields.string_repeated,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn clear_string_repeated(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(
            &mut self.view.fields.string_repeated,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn enum_required_mut(&mut self) -> &mut self::_root::full_coverage2::Enum {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.view.fields.enum_required,
            self.view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_enum_required(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(
            &mut self.view.fields.enum_required,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn enum_optional_mut(&mut self) -> &mut self::_root::full_coverage2::Enum {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.view.fields.enum_optional,
            self.view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_enum_optional(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(
            &mut self.view.fields.enum_optional,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn enum_repeated_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::full_coverage2::Enum> {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.view.fields.enum_repeated,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn clear_enum_repeated(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(
            &mut self.view.fields.enum_repeated,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn submsg_required_mut(
        &mut self,
    ) -> &mut self::_root::full_coverage2::msg::Submsg {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.view.fields.submsg_required,
            self.view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_submsg_required(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(
            &mut self.view.fields.submsg_required,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn submsg_optional_mut(
        &mut self,
    ) -> &mut self::_root::full_coverage2::msg::Submsg {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.view.fields.submsg_optional,
            self.view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_submsg_optional(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(
            &mut self.view.fields.submsg_optional,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn submsg_repeated_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::full_coverage2::msg::Submsg> {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.view.fields.submsg_repeated,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn clear_submsg_repeated(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(
            &mut self.view.fields.submsg_repeated,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn i64_required_mut(&mut self) -> &mut i64 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.view.fields.i64_required,
            self.view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_i64_required(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(
            &mut self.view.fields.i64_required,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn i64_optional_mut(&mut self) -> &mut i64 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.view.fields.i64_optional,
            self.view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_i64_optional(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(
            &mut self.view.fields.i64_optional,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn i64_repeated_mut(&mut self) -> &mut ::std::vec::Vec::<i64> {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.view.fields.i64_repeated,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn clear_i64_repeated(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(
            &mut self.view.fields.i64_repeated,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn u32_required_mut(&mut self) -> &mut u32 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.view.fields.u32_required,
            self.view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_u32_required(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(
            &mut self.view.fields.u32_required,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn u32_optional_mut(&mut self) -> &mut u32 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.view.fields.u32_optional,
            self.view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_u32_optional(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(
            &mut self.view.fields.u32_optional,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn u32_repeated_mut(&mut self) -> &mut ::std::vec::Vec::<u32> {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.view.fields.u32_repeated,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn clear_u32_repeated(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(
            &mut self.view.fields.u32_repeated,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn u64_required_mut(&mut self) -> &mut u64 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.view.fields.u64_required,
            self.view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_u64_required(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(
            &mut self.view.fields.u64_required,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn u64_optional_mut(&mut self) -> &mut u64 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.view.fields.u64_optional,
            self.view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_u64_optional(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(
            &mut self.view.fields.u64_optional,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn u64_repeated_mut(&mut self) -> &mut ::std::vec::Vec::<u64> {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.view.fields.u64_repeated,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn clear_u64_repeated(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(
            &mut self.view.fields.u64_repeated,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn s32_required_mut(&mut self) -> &mut i32 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.view.fields.s32_required,
            self.view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_s32_required(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(
            &mut self.view.fields.s32_required,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn s32_optional_mut(&mut self) -> &mut i32 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.view.fields.s32_optional,
            self.view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_s32_optional(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(
            &mut self.view.fields.s32_optional,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn s32_repeated_mut(&mut self) -> &mut ::std::vec::Vec::<i32> {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.view.fields.s32_repeated,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn clear_s32_repeated(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(
            &mut self.view.fields.s32_repeated,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn s64_required_mut(&mut self) -> &mut i64 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.view.fields.s64_required,
            self.view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_s64_required(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(
            &mut self.view.fields.s64_required,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn s64_optional_mut(&mut self) -> &mut i64 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.view.fields.s64_optional,
            self.view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_s64_optional(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(
            &mut self.view.fields.s64_optional,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn s64_repeated_mut(&mut self) -> &mut ::std::vec::Vec::<i64> {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.view.fields.s64_repeated,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn clear_s64_repeated(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(
            &mut self.view.fields.s64_repeated,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn fixed32_required_mut(&mut self) -> &mut u32 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.view.fields.fixed32_required,
            self.view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_fixed32_required(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(
            &mut self.view.fields.fixed32_required,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn fixed32_optional_mut(&mut self) -> &mut u32 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.view.fields.fixed32_optional,
            self.view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_fixed32_optional(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(
            &mut self.view.fields.fixed32_optional,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn fixed32_repeated_mut(&mut self) -> &mut ::std::vec::Vec::<u32> {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.view.fields.fixed32_repeated,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn clear_fixed32_repeated(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(
            &mut self.view.fields.fixed32_repeated,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn fixed64_required_mut(&mut self) -> &mut u64 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.view.fields.fixed64_required,
            self.view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_fixed64_required(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(
            &mut self.view.fields.fixed64_required,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn fixed64_optional_mut(&mut self) -> &mut u64 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.view.fields.fixed64_optional,
            self.view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_fixed64_optional(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(
            &mut self.view.fields.fixed64_optional,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn fixed64_repeated_mut(&mut self) -> &mut ::std::vec::Vec::<u64> {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.view.fields.fixed64_repeated,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn clear_fixed64_repeated(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(
            &mut self.view.fields.fixed64_repeated,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn sfixed32_required_mut(&mut self) -> &mut i32 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.view.fields.sfixed32_required,
            self.view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_sfixed32_required(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(
            &mut self.view.fields.sfixed32_required,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn sfixed32_optional_mut(&mut self) -> &mut i32 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.view.fields.sfixed32_optional,
            self.view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_sfixed32_optional(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(
            &mut self.view.fields.sfixed32_optional,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn sfixed32_repeated_mut(&mut self) -> &mut ::std::vec::Vec::<i32> {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.view.fields.sfixed32_repeated,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn clear_sfixed32_repeated(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(
            &mut self.view.fields.sfixed32_repeated,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn sfixed64_required_mut(&mut self) -> &mut i64 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.view.fields.sfixed64_required,
            self.view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_sfixed64_required(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(
            &mut self.view.fields.sfixed64_required,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn sfixed64_optional_mut(&mut self) -> &mut i64 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.view.fields.sfixed64_optional,
            self.view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_sfixed64_optional(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(
            &mut self.view.fields.sfixed64_optional,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn sfixed64_repeated_mut(&mut self) -> &mut ::std::vec::Vec::<i64> {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.view.fields.sfixed64_repeated,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn clear_sfixed64_repeated(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(
            &mut self.view.fields.sfixed64_repeated,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn f64_required_mut(&mut self) -> &mut f64 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.view.fields.f64_required,
            self.view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_f64_required(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(
            &mut self.view.fields.f64_required,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn f64_optional_mut(&mut self) -> &mut f64 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.view.fields.f64_optional,
            self.view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_f64_optional(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(
            &mut self.view.fields.f64_optional,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn f64_repeated_mut(&mut self) -> &mut ::std::vec::Vec::<f64> {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.view.fields.f64_repeated,
            self.view.shared.bitfield_mut(),
        )
    }
    pub fn clear_f64_repeated(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(
            &mut self.view.fields.f64_repeated,
            self.view.shared.bitfield_mut(),
        )
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
        iter: I,
    ) -> self::_puroro::Result<()> {
        let mut pos_iter = self::_pinternal::PosIter::new(iter);
        let mut scoped_iter = self::_pinternal::ScopedIter::from_mut_pos_iter(
            &mut pos_iter,
        );
        <Self as self::_pinternal::MessageInternal>::merge_from_scoped_bytes_iter(
            self,
            &mut scoped_iter,
        )?;
        scoped_iter.drop_and_check_scope_completed()?;
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        self::_pinternal::FieldType::ser_to_write(
            &self.view.fields.i32_required,
            self.view.shared.bitfield(),
            1i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.view.fields.i32_optional,
            self.view.shared.bitfield(),
            2i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.view.fields.i32_repeated,
            self.view.shared.bitfield(),
            3i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.view.fields.float_required,
            self.view.shared.bitfield(),
            11i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.view.fields.float_optional,
            self.view.shared.bitfield(),
            12i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.view.fields.float_repeated,
            self.view.shared.bitfield(),
            13i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.view.fields.bytes_required,
            self.view.shared.bitfield(),
            21i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.view.fields.bytes_optional,
            self.view.shared.bitfield(),
            22i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.view.fields.bytes_repeated,
            self.view.shared.bitfield(),
            23i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.view.fields.string_required,
            self.view.shared.bitfield(),
            31i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.view.fields.string_optional,
            self.view.shared.bitfield(),
            32i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.view.fields.string_repeated,
            self.view.shared.bitfield(),
            33i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.view.fields.enum_required,
            self.view.shared.bitfield(),
            41i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.view.fields.enum_optional,
            self.view.shared.bitfield(),
            42i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.view.fields.enum_repeated,
            self.view.shared.bitfield(),
            43i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.view.fields.submsg_required,
            self.view.shared.bitfield(),
            51i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.view.fields.submsg_optional,
            self.view.shared.bitfield(),
            52i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.view.fields.submsg_repeated,
            self.view.shared.bitfield(),
            53i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.view.fields.i64_required,
            self.view.shared.bitfield(),
            101i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.view.fields.i64_optional,
            self.view.shared.bitfield(),
            102i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.view.fields.i64_repeated,
            self.view.shared.bitfield(),
            103i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.view.fields.u32_required,
            self.view.shared.bitfield(),
            111i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.view.fields.u32_optional,
            self.view.shared.bitfield(),
            112i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.view.fields.u32_repeated,
            self.view.shared.bitfield(),
            113i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.view.fields.u64_required,
            self.view.shared.bitfield(),
            121i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.view.fields.u64_optional,
            self.view.shared.bitfield(),
            122i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.view.fields.u64_repeated,
            self.view.shared.bitfield(),
            123i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.view.fields.s32_required,
            self.view.shared.bitfield(),
            131i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.view.fields.s32_optional,
            self.view.shared.bitfield(),
            132i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.view.fields.s32_repeated,
            self.view.shared.bitfield(),
            133i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.view.fields.s64_required,
            self.view.shared.bitfield(),
            141i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.view.fields.s64_optional,
            self.view.shared.bitfield(),
            142i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.view.fields.s64_repeated,
            self.view.shared.bitfield(),
            143i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.view.fields.fixed32_required,
            self.view.shared.bitfield(),
            151i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.view.fields.fixed32_optional,
            self.view.shared.bitfield(),
            152i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.view.fields.fixed32_repeated,
            self.view.shared.bitfield(),
            153i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.view.fields.fixed64_required,
            self.view.shared.bitfield(),
            161i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.view.fields.fixed64_optional,
            self.view.shared.bitfield(),
            162i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.view.fields.fixed64_repeated,
            self.view.shared.bitfield(),
            163i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.view.fields.sfixed32_required,
            self.view.shared.bitfield(),
            171i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.view.fields.sfixed32_optional,
            self.view.shared.bitfield(),
            172i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.view.fields.sfixed32_repeated,
            self.view.shared.bitfield(),
            173i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.view.fields.sfixed64_required,
            self.view.shared.bitfield(),
            181i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.view.fields.sfixed64_optional,
            self.view.shared.bitfield(),
            182i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.view.fields.sfixed64_repeated,
            self.view.shared.bitfield(),
            183i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.view.fields.f64_required,
            self.view.shared.bitfield(),
            191i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.view.fields.f64_optional,
            self.view.shared.bitfield(),
            192i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.view.fields.f64_repeated,
            self.view.shared.bitfield(),
            193i32,
            out,
        )?;
        self.shared.unknown_fields().ser_to_write(out)?;
        ::std::result::Result::Ok(())
    }
}
impl self::_pinternal::MessageInternal for Msg {
    fn merge_from_scoped_bytes_iter<
        'a,
        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
    >(
        &mut self,
        iter: &mut self::_pinternal::ScopedIter<'a, I>,
    ) -> self::_puroro::Result<()> {
        use self::_pinternal::ser::FieldData;
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        #[allow(unused)]
        use ::std::result::Result;
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        #[allow(unused)]
        use ::std::vec::Vec;
        use self::_puroro::PuroroError;
        while let Some((number, field_data))
            = FieldData::from_bytes_scoped_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    1i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.view.fields.i32_required,
                            self.view.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    2i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.view.fields.i32_optional,
                            self.view.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    3i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.view.fields.i32_repeated,
                            self.view.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    11i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.view.fields.float_required,
                            self.view.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    12i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.view.fields.float_optional,
                            self.view.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    13i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.view.fields.float_repeated,
                            self.view.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    21i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.view.fields.bytes_required,
                            self.view.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    22i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.view.fields.bytes_optional,
                            self.view.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    23i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.view.fields.bytes_repeated,
                            self.view.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    31i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.view.fields.string_required,
                            self.view.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    32i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.view.fields.string_optional,
                            self.view.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    33i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.view.fields.string_repeated,
                            self.view.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    41i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.view.fields.enum_required,
                            self.view.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    42i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.view.fields.enum_optional,
                            self.view.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    43i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.view.fields.enum_repeated,
                            self.view.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    51i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.view.fields.submsg_required,
                            self.view.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    52i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.view.fields.submsg_optional,
                            self.view.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    53i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.view.fields.submsg_repeated,
                            self.view.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    101i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.view.fields.i64_required,
                            self.view.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    102i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.view.fields.i64_optional,
                            self.view.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    103i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.view.fields.i64_repeated,
                            self.view.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    111i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.view.fields.u32_required,
                            self.view.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    112i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.view.fields.u32_optional,
                            self.view.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    113i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.view.fields.u32_repeated,
                            self.view.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    121i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.view.fields.u64_required,
                            self.view.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    122i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.view.fields.u64_optional,
                            self.view.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    123i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.view.fields.u64_repeated,
                            self.view.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    131i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.view.fields.s32_required,
                            self.view.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    132i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.view.fields.s32_optional,
                            self.view.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    133i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.view.fields.s32_repeated,
                            self.view.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    141i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.view.fields.s64_required,
                            self.view.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    142i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.view.fields.s64_optional,
                            self.view.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    143i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.view.fields.s64_repeated,
                            self.view.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    151i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.view.fields.fixed32_required,
                            self.view.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    152i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.view.fields.fixed32_optional,
                            self.view.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    153i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.view.fields.fixed32_repeated,
                            self.view.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    161i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.view.fields.fixed64_required,
                            self.view.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    162i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.view.fields.fixed64_optional,
                            self.view.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    163i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.view.fields.fixed64_repeated,
                            self.view.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    171i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.view.fields.sfixed32_required,
                            self.view.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    172i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.view.fields.sfixed32_optional,
                            self.view.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    173i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.view.fields.sfixed32_repeated,
                            self.view.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    181i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.view.fields.sfixed64_required,
                            self.view.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    182i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.view.fields.sfixed64_optional,
                            self.view.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    183i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.view.fields.sfixed64_repeated,
                            self.view.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    191i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.view.fields.f64_required,
                            self.view.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    192i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.view.fields.f64_optional,
                            self.view.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    193i32 => {
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut self.view.fields.f64_repeated,
                            self.view.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    _ => {
                        let field_data = field_data
                            .map(|iter| { iter.collect::<Result<Vec<_>, _>>() })
                            .transpose()?;
                        Err(PuroroError::UnknownFieldNumber(field_data))?
                    }
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(PuroroError::UnknownFieldNumber(field_data)) => {
                    self.view.shared.unknown_fields_mut().push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
}
impl ::std::clone::Clone for Msg {
    fn clone(&self) -> Self {
        Self {
            view: ::std::clone::Clone::clone(&self.view),
        }
    }
}
impl ::std::fmt::Debug for Msg {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        <self::_root::full_coverage2::_view::MsgView as ::std::fmt::Debug>::fmt(
            &self.view,
            fmt,
        )
    }
}
impl ::std::ops::Deref for Msg {
    type Target = self::_root::full_coverage2::_view::MsgView;
    fn deref(&self) -> &Self::Target {
        &self.view
    }
}
impl ::std::cmp::PartialEq for Msg {
    fn eq(&self, rhs: &Self) -> bool {
        &self.view == &rhs.view
    }
}
#[doc(hidden)]
pub mod _view {
    mod _root {
        #[allow(unused)]
        pub(crate) use super::super::_root::*;
    }
    mod _puroro {
        #[allow(unused)]
        pub(crate) use super::_root::_puroro::*;
    }
    mod _pinternal {
        #[allow(unused)]
        pub(crate) use super::_root::_pinternal::*;
    }
    #[derive(::std::default::Default)]
    pub struct MsgView {
        pub(super) fields: self::_root::full_coverage2::_fields::MsgFields::<
            self::_pinternal::OptionalNumericalField::<
                i32,
                self::_pinternal::tags::Int32,
                0usize,
            >,
            self::_pinternal::OptionalNumericalField::<
                i32,
                self::_pinternal::tags::Int32,
                1usize,
            >,
            self::_pinternal::RepeatedNumericalField::<
                i32,
                self::_pinternal::tags::Int32,
            >,
            self::_pinternal::OptionalNumericalField::<
                f32,
                self::_pinternal::tags::Float,
                2usize,
            >,
            self::_pinternal::OptionalNumericalField::<
                f32,
                self::_pinternal::tags::Float,
                3usize,
            >,
            self::_pinternal::RepeatedNumericalField::<
                f32,
                self::_pinternal::tags::Float,
            >,
            self::_pinternal::OptionalUnsizedField::<
                ::std::vec::Vec<u8>,
                self::_pinternal::tags::Bytes,
                4usize,
            >,
            self::_pinternal::OptionalUnsizedField::<
                ::std::vec::Vec<u8>,
                self::_pinternal::tags::Bytes,
                5usize,
            >,
            self::_pinternal::RepeatedUnsizedField::<
                ::std::vec::Vec<u8>,
                self::_pinternal::tags::Bytes,
            >,
            self::_pinternal::OptionalUnsizedField::<
                ::std::string::String,
                self::_pinternal::tags::String,
                6usize,
            >,
            self::_pinternal::OptionalUnsizedField::<
                ::std::string::String,
                self::_pinternal::tags::String,
                7usize,
            >,
            self::_pinternal::RepeatedUnsizedField::<
                ::std::string::String,
                self::_pinternal::tags::String,
            >,
            self::_pinternal::OptionalNumericalField::<
                self::_root::full_coverage2::Enum,
                self::_pinternal::tags::Enum2::<self::_root::full_coverage2::Enum>,
                8usize,
            >,
            self::_pinternal::OptionalNumericalField::<
                self::_root::full_coverage2::Enum,
                self::_pinternal::tags::Enum2::<self::_root::full_coverage2::Enum>,
                9usize,
            >,
            self::_pinternal::RepeatedNumericalField::<
                self::_root::full_coverage2::Enum,
                self::_pinternal::tags::Enum2::<self::_root::full_coverage2::Enum>,
            >,
            self::_pinternal::SingularHeapMessageField::<
                self::_root::full_coverage2::msg::Submsg,
            >,
            self::_pinternal::SingularHeapMessageField::<
                self::_root::full_coverage2::msg::Submsg,
            >,
            self::_pinternal::RepeatedMessageField::<
                self::_root::full_coverage2::msg::Submsg,
            >,
            self::_pinternal::OptionalNumericalField::<
                i64,
                self::_pinternal::tags::Int64,
                10usize,
            >,
            self::_pinternal::OptionalNumericalField::<
                i64,
                self::_pinternal::tags::Int64,
                11usize,
            >,
            self::_pinternal::RepeatedNumericalField::<
                i64,
                self::_pinternal::tags::Int64,
            >,
            self::_pinternal::OptionalNumericalField::<
                u32,
                self::_pinternal::tags::UInt32,
                12usize,
            >,
            self::_pinternal::OptionalNumericalField::<
                u32,
                self::_pinternal::tags::UInt32,
                13usize,
            >,
            self::_pinternal::RepeatedNumericalField::<
                u32,
                self::_pinternal::tags::UInt32,
            >,
            self::_pinternal::OptionalNumericalField::<
                u64,
                self::_pinternal::tags::UInt64,
                14usize,
            >,
            self::_pinternal::OptionalNumericalField::<
                u64,
                self::_pinternal::tags::UInt64,
                15usize,
            >,
            self::_pinternal::RepeatedNumericalField::<
                u64,
                self::_pinternal::tags::UInt64,
            >,
            self::_pinternal::OptionalNumericalField::<
                i32,
                self::_pinternal::tags::SInt32,
                16usize,
            >,
            self::_pinternal::OptionalNumericalField::<
                i32,
                self::_pinternal::tags::SInt32,
                17usize,
            >,
            self::_pinternal::RepeatedNumericalField::<
                i32,
                self::_pinternal::tags::SInt32,
            >,
            self::_pinternal::OptionalNumericalField::<
                i64,
                self::_pinternal::tags::SInt64,
                18usize,
            >,
            self::_pinternal::OptionalNumericalField::<
                i64,
                self::_pinternal::tags::SInt64,
                19usize,
            >,
            self::_pinternal::RepeatedNumericalField::<
                i64,
                self::_pinternal::tags::SInt64,
            >,
            self::_pinternal::OptionalNumericalField::<
                u32,
                self::_pinternal::tags::Fixed32,
                20usize,
            >,
            self::_pinternal::OptionalNumericalField::<
                u32,
                self::_pinternal::tags::Fixed32,
                21usize,
            >,
            self::_pinternal::RepeatedNumericalField::<
                u32,
                self::_pinternal::tags::Fixed32,
            >,
            self::_pinternal::OptionalNumericalField::<
                u64,
                self::_pinternal::tags::Fixed64,
                22usize,
            >,
            self::_pinternal::OptionalNumericalField::<
                u64,
                self::_pinternal::tags::Fixed64,
                23usize,
            >,
            self::_pinternal::RepeatedNumericalField::<
                u64,
                self::_pinternal::tags::Fixed64,
            >,
            self::_pinternal::OptionalNumericalField::<
                i32,
                self::_pinternal::tags::SFixed32,
                24usize,
            >,
            self::_pinternal::OptionalNumericalField::<
                i32,
                self::_pinternal::tags::SFixed32,
                25usize,
            >,
            self::_pinternal::RepeatedNumericalField::<
                i32,
                self::_pinternal::tags::SFixed32,
            >,
            self::_pinternal::OptionalNumericalField::<
                i64,
                self::_pinternal::tags::SFixed64,
                26usize,
            >,
            self::_pinternal::OptionalNumericalField::<
                i64,
                self::_pinternal::tags::SFixed64,
                27usize,
            >,
            self::_pinternal::RepeatedNumericalField::<
                i64,
                self::_pinternal::tags::SFixed64,
            >,
            self::_pinternal::OptionalNumericalField::<
                f64,
                self::_pinternal::tags::Double,
                28usize,
            >,
            self::_pinternal::OptionalNumericalField::<
                f64,
                self::_pinternal::tags::Double,
                29usize,
            >,
            self::_pinternal::RepeatedNumericalField::<
                f64,
                self::_pinternal::tags::Double,
            >,
        >,
        pub(super) shared: self::_pinternal::SharedItemsImpl<1usize>,
    }
    impl MsgView {
        pub fn i32_required(&self) -> i32 {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.i32_required,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn i32_required_opt(&self) -> ::std::option::Option::<i32> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.i32_required,
                self.shared.bitfield(),
            )
        }
        pub fn has_i32_required(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.i32_required,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn i32_optional(&self) -> i32 {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.i32_optional,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn i32_optional_opt(&self) -> ::std::option::Option::<i32> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.i32_optional,
                self.shared.bitfield(),
            )
        }
        pub fn has_i32_optional(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.i32_optional,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn i32_repeated(
            &self,
        ) -> impl '_ + ::std::iter::IntoIterator<
            Item = i32,
        > + ::std::ops::Index<usize, Output = i32> {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field2(
                &self.fields.i32_repeated,
                self.shared.bitfield(),
            )
        }
        pub fn float_required(&self) -> f32 {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.float_required,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn float_required_opt(&self) -> ::std::option::Option::<f32> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.float_required,
                self.shared.bitfield(),
            )
        }
        pub fn has_float_required(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.float_required,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn float_optional(&self) -> f32 {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.float_optional,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn float_optional_opt(&self) -> ::std::option::Option::<f32> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.float_optional,
                self.shared.bitfield(),
            )
        }
        pub fn has_float_optional(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.float_optional,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn float_repeated(
            &self,
        ) -> impl '_ + ::std::iter::IntoIterator<
            Item = f32,
        > + ::std::ops::Index<usize, Output = f32> {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field2(
                &self.fields.float_repeated,
                self.shared.bitfield(),
            )
        }
        pub fn bytes_required(&self) -> &[u8] {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.bytes_required,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn bytes_required_opt(&self) -> ::std::option::Option::<&[u8]> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.bytes_required,
                self.shared.bitfield(),
            )
        }
        pub fn has_bytes_required(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.bytes_required,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn bytes_optional(&self) -> &[u8] {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.bytes_optional,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn bytes_optional_opt(&self) -> ::std::option::Option::<&[u8]> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.bytes_optional,
                self.shared.bitfield(),
            )
        }
        pub fn has_bytes_optional(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.bytes_optional,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn bytes_repeated(
            &self,
        ) -> impl '_ + ::std::iter::IntoIterator<
            Item = &[u8],
        > + ::std::ops::Index<usize, Output = [u8]> {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field2(
                &self.fields.bytes_repeated,
                self.shared.bitfield(),
            )
        }
        pub fn string_required(&self) -> &str {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.string_required,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn string_required_opt(&self) -> ::std::option::Option::<&str> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.string_required,
                self.shared.bitfield(),
            )
        }
        pub fn has_string_required(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.string_required,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn string_optional(&self) -> &str {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.string_optional,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn string_optional_opt(&self) -> ::std::option::Option::<&str> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.string_optional,
                self.shared.bitfield(),
            )
        }
        pub fn has_string_optional(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.string_optional,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn string_repeated(
            &self,
        ) -> impl '_ + ::std::iter::IntoIterator<
            Item = &str,
        > + ::std::ops::Index<usize, Output = str> {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field2(
                &self.fields.string_repeated,
                self.shared.bitfield(),
            )
        }
        pub fn enum_required(&self) -> self::_root::full_coverage2::Enum {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.enum_required,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn enum_required_opt(
            &self,
        ) -> ::std::option::Option::<self::_root::full_coverage2::Enum> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.enum_required,
                self.shared.bitfield(),
            )
        }
        pub fn has_enum_required(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.enum_required,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn enum_optional(&self) -> self::_root::full_coverage2::Enum {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.enum_optional,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn enum_optional_opt(
            &self,
        ) -> ::std::option::Option::<self::_root::full_coverage2::Enum> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.enum_optional,
                self.shared.bitfield(),
            )
        }
        pub fn has_enum_optional(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.enum_optional,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn enum_repeated(
            &self,
        ) -> impl '_ + ::std::iter::IntoIterator<
            Item = self::_root::full_coverage2::Enum,
        > + ::std::ops::Index<usize, Output = self::_root::full_coverage2::Enum> {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field2(
                &self.fields.enum_repeated,
                self.shared.bitfield(),
            )
        }
        pub fn submsg_required(
            &self,
        ) -> ::std::option::Option::<&self::_root::full_coverage2::msg::Submsg> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.submsg_required,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn submsg_required_opt(
            &self,
        ) -> ::std::option::Option::<&self::_root::full_coverage2::msg::Submsg> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.submsg_required,
                self.shared.bitfield(),
            )
        }
        pub fn has_submsg_required(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.submsg_required,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn submsg_optional(
            &self,
        ) -> ::std::option::Option::<&self::_root::full_coverage2::msg::Submsg> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.submsg_optional,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn submsg_optional_opt(
            &self,
        ) -> ::std::option::Option::<&self::_root::full_coverage2::msg::Submsg> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.submsg_optional,
                self.shared.bitfield(),
            )
        }
        pub fn has_submsg_optional(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.submsg_optional,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn submsg_repeated(
            &self,
        ) -> impl '_ + ::std::iter::IntoIterator<
            Item = &self::_root::full_coverage2::msg::_view::SubmsgView,
        > + ::std::ops::Index<
            usize,
            Output = self::_root::full_coverage2::msg::_view::SubmsgView,
        > {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field2(
                &self.fields.submsg_repeated,
                self.shared.bitfield(),
            )
        }
        pub fn i64_required(&self) -> i64 {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.i64_required,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn i64_required_opt(&self) -> ::std::option::Option::<i64> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.i64_required,
                self.shared.bitfield(),
            )
        }
        pub fn has_i64_required(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.i64_required,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn i64_optional(&self) -> i64 {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.i64_optional,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn i64_optional_opt(&self) -> ::std::option::Option::<i64> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.i64_optional,
                self.shared.bitfield(),
            )
        }
        pub fn has_i64_optional(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.i64_optional,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn i64_repeated(
            &self,
        ) -> impl '_ + ::std::iter::IntoIterator<
            Item = i64,
        > + ::std::ops::Index<usize, Output = i64> {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field2(
                &self.fields.i64_repeated,
                self.shared.bitfield(),
            )
        }
        pub fn u32_required(&self) -> u32 {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.u32_required,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn u32_required_opt(&self) -> ::std::option::Option::<u32> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.u32_required,
                self.shared.bitfield(),
            )
        }
        pub fn has_u32_required(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.u32_required,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn u32_optional(&self) -> u32 {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.u32_optional,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn u32_optional_opt(&self) -> ::std::option::Option::<u32> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.u32_optional,
                self.shared.bitfield(),
            )
        }
        pub fn has_u32_optional(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.u32_optional,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn u32_repeated(
            &self,
        ) -> impl '_ + ::std::iter::IntoIterator<
            Item = u32,
        > + ::std::ops::Index<usize, Output = u32> {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field2(
                &self.fields.u32_repeated,
                self.shared.bitfield(),
            )
        }
        pub fn u64_required(&self) -> u64 {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.u64_required,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn u64_required_opt(&self) -> ::std::option::Option::<u64> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.u64_required,
                self.shared.bitfield(),
            )
        }
        pub fn has_u64_required(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.u64_required,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn u64_optional(&self) -> u64 {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.u64_optional,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn u64_optional_opt(&self) -> ::std::option::Option::<u64> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.u64_optional,
                self.shared.bitfield(),
            )
        }
        pub fn has_u64_optional(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.u64_optional,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn u64_repeated(
            &self,
        ) -> impl '_ + ::std::iter::IntoIterator<
            Item = u64,
        > + ::std::ops::Index<usize, Output = u64> {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field2(
                &self.fields.u64_repeated,
                self.shared.bitfield(),
            )
        }
        pub fn s32_required(&self) -> i32 {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.s32_required,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn s32_required_opt(&self) -> ::std::option::Option::<i32> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.s32_required,
                self.shared.bitfield(),
            )
        }
        pub fn has_s32_required(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.s32_required,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn s32_optional(&self) -> i32 {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.s32_optional,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn s32_optional_opt(&self) -> ::std::option::Option::<i32> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.s32_optional,
                self.shared.bitfield(),
            )
        }
        pub fn has_s32_optional(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.s32_optional,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn s32_repeated(
            &self,
        ) -> impl '_ + ::std::iter::IntoIterator<
            Item = i32,
        > + ::std::ops::Index<usize, Output = i32> {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field2(
                &self.fields.s32_repeated,
                self.shared.bitfield(),
            )
        }
        pub fn s64_required(&self) -> i64 {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.s64_required,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn s64_required_opt(&self) -> ::std::option::Option::<i64> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.s64_required,
                self.shared.bitfield(),
            )
        }
        pub fn has_s64_required(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.s64_required,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn s64_optional(&self) -> i64 {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.s64_optional,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn s64_optional_opt(&self) -> ::std::option::Option::<i64> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.s64_optional,
                self.shared.bitfield(),
            )
        }
        pub fn has_s64_optional(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.s64_optional,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn s64_repeated(
            &self,
        ) -> impl '_ + ::std::iter::IntoIterator<
            Item = i64,
        > + ::std::ops::Index<usize, Output = i64> {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field2(
                &self.fields.s64_repeated,
                self.shared.bitfield(),
            )
        }
        pub fn fixed32_required(&self) -> u32 {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.fixed32_required,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn fixed32_required_opt(&self) -> ::std::option::Option::<u32> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.fixed32_required,
                self.shared.bitfield(),
            )
        }
        pub fn has_fixed32_required(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.fixed32_required,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn fixed32_optional(&self) -> u32 {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.fixed32_optional,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn fixed32_optional_opt(&self) -> ::std::option::Option::<u32> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.fixed32_optional,
                self.shared.bitfield(),
            )
        }
        pub fn has_fixed32_optional(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.fixed32_optional,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn fixed32_repeated(
            &self,
        ) -> impl '_ + ::std::iter::IntoIterator<
            Item = u32,
        > + ::std::ops::Index<usize, Output = u32> {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field2(
                &self.fields.fixed32_repeated,
                self.shared.bitfield(),
            )
        }
        pub fn fixed64_required(&self) -> u64 {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.fixed64_required,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn fixed64_required_opt(&self) -> ::std::option::Option::<u64> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.fixed64_required,
                self.shared.bitfield(),
            )
        }
        pub fn has_fixed64_required(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.fixed64_required,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn fixed64_optional(&self) -> u64 {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.fixed64_optional,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn fixed64_optional_opt(&self) -> ::std::option::Option::<u64> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.fixed64_optional,
                self.shared.bitfield(),
            )
        }
        pub fn has_fixed64_optional(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.fixed64_optional,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn fixed64_repeated(
            &self,
        ) -> impl '_ + ::std::iter::IntoIterator<
            Item = u64,
        > + ::std::ops::Index<usize, Output = u64> {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field2(
                &self.fields.fixed64_repeated,
                self.shared.bitfield(),
            )
        }
        pub fn sfixed32_required(&self) -> i32 {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.sfixed32_required,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn sfixed32_required_opt(&self) -> ::std::option::Option::<i32> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.sfixed32_required,
                self.shared.bitfield(),
            )
        }
        pub fn has_sfixed32_required(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.sfixed32_required,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn sfixed32_optional(&self) -> i32 {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.sfixed32_optional,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn sfixed32_optional_opt(&self) -> ::std::option::Option::<i32> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.sfixed32_optional,
                self.shared.bitfield(),
            )
        }
        pub fn has_sfixed32_optional(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.sfixed32_optional,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn sfixed32_repeated(
            &self,
        ) -> impl '_ + ::std::iter::IntoIterator<
            Item = i32,
        > + ::std::ops::Index<usize, Output = i32> {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field2(
                &self.fields.sfixed32_repeated,
                self.shared.bitfield(),
            )
        }
        pub fn sfixed64_required(&self) -> i64 {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.sfixed64_required,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn sfixed64_required_opt(&self) -> ::std::option::Option::<i64> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.sfixed64_required,
                self.shared.bitfield(),
            )
        }
        pub fn has_sfixed64_required(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.sfixed64_required,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn sfixed64_optional(&self) -> i64 {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.sfixed64_optional,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn sfixed64_optional_opt(&self) -> ::std::option::Option::<i64> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.sfixed64_optional,
                self.shared.bitfield(),
            )
        }
        pub fn has_sfixed64_optional(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.sfixed64_optional,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn sfixed64_repeated(
            &self,
        ) -> impl '_ + ::std::iter::IntoIterator<
            Item = i64,
        > + ::std::ops::Index<usize, Output = i64> {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field2(
                &self.fields.sfixed64_repeated,
                self.shared.bitfield(),
            )
        }
        pub fn f64_required(&self) -> f64 {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.f64_required,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn f64_required_opt(&self) -> ::std::option::Option::<f64> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.f64_required,
                self.shared.bitfield(),
            )
        }
        pub fn has_f64_required(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.f64_required,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn f64_optional(&self) -> f64 {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.f64_optional,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn f64_optional_opt(&self) -> ::std::option::Option::<f64> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.f64_optional,
                self.shared.bitfield(),
            )
        }
        pub fn has_f64_optional(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.f64_optional,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn f64_repeated(
            &self,
        ) -> impl '_ + ::std::iter::IntoIterator<
            Item = f64,
        > + ::std::ops::Index<usize, Output = f64> {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field2(
                &self.fields.f64_repeated,
                self.shared.bitfield(),
            )
        }
    }
    impl ::std::clone::Clone for MsgView {
        fn clone(&self) -> Self {
            #[allow(unused)]
            use self::_pinternal::SharedItems as _;
            Self {
                fields: self::_root::full_coverage2::_fields::MsgFields {
                    i32_required: ::std::clone::Clone::clone(&self.fields.i32_required),
                    i32_optional: ::std::clone::Clone::clone(&self.fields.i32_optional),
                    i32_repeated: ::std::clone::Clone::clone(&self.fields.i32_repeated),
                    float_required: ::std::clone::Clone::clone(
                        &self.fields.float_required,
                    ),
                    float_optional: ::std::clone::Clone::clone(
                        &self.fields.float_optional,
                    ),
                    float_repeated: ::std::clone::Clone::clone(
                        &self.fields.float_repeated,
                    ),
                    bytes_required: ::std::clone::Clone::clone(
                        &self.fields.bytes_required,
                    ),
                    bytes_optional: ::std::clone::Clone::clone(
                        &self.fields.bytes_optional,
                    ),
                    bytes_repeated: ::std::clone::Clone::clone(
                        &self.fields.bytes_repeated,
                    ),
                    string_required: ::std::clone::Clone::clone(
                        &self.fields.string_required,
                    ),
                    string_optional: ::std::clone::Clone::clone(
                        &self.fields.string_optional,
                    ),
                    string_repeated: ::std::clone::Clone::clone(
                        &self.fields.string_repeated,
                    ),
                    enum_required: ::std::clone::Clone::clone(
                        &self.fields.enum_required,
                    ),
                    enum_optional: ::std::clone::Clone::clone(
                        &self.fields.enum_optional,
                    ),
                    enum_repeated: ::std::clone::Clone::clone(
                        &self.fields.enum_repeated,
                    ),
                    submsg_required: ::std::clone::Clone::clone(
                        &self.fields.submsg_required,
                    ),
                    submsg_optional: ::std::clone::Clone::clone(
                        &self.fields.submsg_optional,
                    ),
                    submsg_repeated: ::std::clone::Clone::clone(
                        &self.fields.submsg_repeated,
                    ),
                    i64_required: ::std::clone::Clone::clone(&self.fields.i64_required),
                    i64_optional: ::std::clone::Clone::clone(&self.fields.i64_optional),
                    i64_repeated: ::std::clone::Clone::clone(&self.fields.i64_repeated),
                    u32_required: ::std::clone::Clone::clone(&self.fields.u32_required),
                    u32_optional: ::std::clone::Clone::clone(&self.fields.u32_optional),
                    u32_repeated: ::std::clone::Clone::clone(&self.fields.u32_repeated),
                    u64_required: ::std::clone::Clone::clone(&self.fields.u64_required),
                    u64_optional: ::std::clone::Clone::clone(&self.fields.u64_optional),
                    u64_repeated: ::std::clone::Clone::clone(&self.fields.u64_repeated),
                    s32_required: ::std::clone::Clone::clone(&self.fields.s32_required),
                    s32_optional: ::std::clone::Clone::clone(&self.fields.s32_optional),
                    s32_repeated: ::std::clone::Clone::clone(&self.fields.s32_repeated),
                    s64_required: ::std::clone::Clone::clone(&self.fields.s64_required),
                    s64_optional: ::std::clone::Clone::clone(&self.fields.s64_optional),
                    s64_repeated: ::std::clone::Clone::clone(&self.fields.s64_repeated),
                    fixed32_required: ::std::clone::Clone::clone(
                        &self.fields.fixed32_required,
                    ),
                    fixed32_optional: ::std::clone::Clone::clone(
                        &self.fields.fixed32_optional,
                    ),
                    fixed32_repeated: ::std::clone::Clone::clone(
                        &self.fields.fixed32_repeated,
                    ),
                    fixed64_required: ::std::clone::Clone::clone(
                        &self.fields.fixed64_required,
                    ),
                    fixed64_optional: ::std::clone::Clone::clone(
                        &self.fields.fixed64_optional,
                    ),
                    fixed64_repeated: ::std::clone::Clone::clone(
                        &self.fields.fixed64_repeated,
                    ),
                    sfixed32_required: ::std::clone::Clone::clone(
                        &self.fields.sfixed32_required,
                    ),
                    sfixed32_optional: ::std::clone::Clone::clone(
                        &self.fields.sfixed32_optional,
                    ),
                    sfixed32_repeated: ::std::clone::Clone::clone(
                        &self.fields.sfixed32_repeated,
                    ),
                    sfixed64_required: ::std::clone::Clone::clone(
                        &self.fields.sfixed64_required,
                    ),
                    sfixed64_optional: ::std::clone::Clone::clone(
                        &self.fields.sfixed64_optional,
                    ),
                    sfixed64_repeated: ::std::clone::Clone::clone(
                        &self.fields.sfixed64_repeated,
                    ),
                    f64_required: ::std::clone::Clone::clone(&self.fields.f64_required),
                    f64_optional: ::std::clone::Clone::clone(&self.fields.f64_optional),
                    f64_repeated: ::std::clone::Clone::clone(&self.fields.f64_repeated),
                },
                shared: ::std::clone::Clone::clone(&self.shared),
            }
        }
    }
    impl ::std::ops::Drop for MsgView {
        fn drop(&mut self) {
            #[allow(unused)]
            use self::_pinternal::{OneofUnion as _, SharedItems as _};
        }
    }
    impl ::std::fmt::Debug for MsgView {
        fn fmt(
            &self,
            fmt: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            let mut debug_struct = fmt.debug_struct(stringify!(MsgView));
            debug_struct
                .field(stringify!(i32_required), &self.i32_required_opt())
                .field(stringify!(i32_optional), &self.i32_optional_opt())
                .field(
                    stringify!(i32_repeated),
                    &self
                        .i32_repeated()
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>()
                        .as_slice(),
                )
                .field(stringify!(float_required), &self.float_required_opt())
                .field(stringify!(float_optional), &self.float_optional_opt())
                .field(
                    stringify!(float_repeated),
                    &self
                        .float_repeated()
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>()
                        .as_slice(),
                )
                .field(stringify!(bytes_required), &self.bytes_required_opt())
                .field(stringify!(bytes_optional), &self.bytes_optional_opt())
                .field(
                    stringify!(bytes_repeated),
                    &self
                        .bytes_repeated()
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>()
                        .as_slice(),
                )
                .field(stringify!(string_required), &self.string_required_opt())
                .field(stringify!(string_optional), &self.string_optional_opt())
                .field(
                    stringify!(string_repeated),
                    &self
                        .string_repeated()
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>()
                        .as_slice(),
                )
                .field(stringify!(enum_required), &self.enum_required_opt())
                .field(stringify!(enum_optional), &self.enum_optional_opt())
                .field(
                    stringify!(enum_repeated),
                    &self
                        .enum_repeated()
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>()
                        .as_slice(),
                )
                .field(stringify!(submsg_required), &self.submsg_required_opt())
                .field(stringify!(submsg_optional), &self.submsg_optional_opt())
                .field(
                    stringify!(submsg_repeated),
                    &self
                        .submsg_repeated()
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>()
                        .as_slice(),
                )
                .field(stringify!(i64_required), &self.i64_required_opt())
                .field(stringify!(i64_optional), &self.i64_optional_opt())
                .field(
                    stringify!(i64_repeated),
                    &self
                        .i64_repeated()
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>()
                        .as_slice(),
                )
                .field(stringify!(u32_required), &self.u32_required_opt())
                .field(stringify!(u32_optional), &self.u32_optional_opt())
                .field(
                    stringify!(u32_repeated),
                    &self
                        .u32_repeated()
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>()
                        .as_slice(),
                )
                .field(stringify!(u64_required), &self.u64_required_opt())
                .field(stringify!(u64_optional), &self.u64_optional_opt())
                .field(
                    stringify!(u64_repeated),
                    &self
                        .u64_repeated()
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>()
                        .as_slice(),
                )
                .field(stringify!(s32_required), &self.s32_required_opt())
                .field(stringify!(s32_optional), &self.s32_optional_opt())
                .field(
                    stringify!(s32_repeated),
                    &self
                        .s32_repeated()
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>()
                        .as_slice(),
                )
                .field(stringify!(s64_required), &self.s64_required_opt())
                .field(stringify!(s64_optional), &self.s64_optional_opt())
                .field(
                    stringify!(s64_repeated),
                    &self
                        .s64_repeated()
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>()
                        .as_slice(),
                )
                .field(stringify!(fixed32_required), &self.fixed32_required_opt())
                .field(stringify!(fixed32_optional), &self.fixed32_optional_opt())
                .field(
                    stringify!(fixed32_repeated),
                    &self
                        .fixed32_repeated()
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>()
                        .as_slice(),
                )
                .field(stringify!(fixed64_required), &self.fixed64_required_opt())
                .field(stringify!(fixed64_optional), &self.fixed64_optional_opt())
                .field(
                    stringify!(fixed64_repeated),
                    &self
                        .fixed64_repeated()
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>()
                        .as_slice(),
                )
                .field(stringify!(sfixed32_required), &self.sfixed32_required_opt())
                .field(stringify!(sfixed32_optional), &self.sfixed32_optional_opt())
                .field(
                    stringify!(sfixed32_repeated),
                    &self
                        .sfixed32_repeated()
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>()
                        .as_slice(),
                )
                .field(stringify!(sfixed64_required), &self.sfixed64_required_opt())
                .field(stringify!(sfixed64_optional), &self.sfixed64_optional_opt())
                .field(
                    stringify!(sfixed64_repeated),
                    &self
                        .sfixed64_repeated()
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>()
                        .as_slice(),
                )
                .field(stringify!(f64_required), &self.f64_required_opt())
                .field(stringify!(f64_optional), &self.f64_optional_opt())
                .field(
                    stringify!(f64_repeated),
                    &self
                        .f64_repeated()
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>()
                        .as_slice(),
                );
            self.shared.unknown_fields().debug_struct_fields(&mut debug_struct)?;
            debug_struct.finish()
        }
    }
    impl ::std::cmp::PartialEq for MsgView {
        fn eq(&self, rhs: &Self) -> bool {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::SharedItems as _;
            true && self.i32_required_opt() == rhs.i32_required_opt()
                && self.i32_optional_opt() == rhs.i32_optional_opt()
                && self.i32_repeated().into_iter().eq(rhs.i32_repeated())
                && self.float_required_opt() == rhs.float_required_opt()
                && self.float_optional_opt() == rhs.float_optional_opt()
                && self.float_repeated().into_iter().eq(rhs.float_repeated())
                && self.bytes_required_opt() == rhs.bytes_required_opt()
                && self.bytes_optional_opt() == rhs.bytes_optional_opt()
                && self.bytes_repeated().into_iter().eq(rhs.bytes_repeated())
                && self.string_required_opt() == rhs.string_required_opt()
                && self.string_optional_opt() == rhs.string_optional_opt()
                && self.string_repeated().into_iter().eq(rhs.string_repeated())
                && self.enum_required_opt() == rhs.enum_required_opt()
                && self.enum_optional_opt() == rhs.enum_optional_opt()
                && self.enum_repeated().into_iter().eq(rhs.enum_repeated())
                && self.submsg_required_opt() == rhs.submsg_required_opt()
                && self.submsg_optional_opt() == rhs.submsg_optional_opt()
                && self.submsg_repeated().into_iter().eq(rhs.submsg_repeated())
                && self.i64_required_opt() == rhs.i64_required_opt()
                && self.i64_optional_opt() == rhs.i64_optional_opt()
                && self.i64_repeated().into_iter().eq(rhs.i64_repeated())
                && self.u32_required_opt() == rhs.u32_required_opt()
                && self.u32_optional_opt() == rhs.u32_optional_opt()
                && self.u32_repeated().into_iter().eq(rhs.u32_repeated())
                && self.u64_required_opt() == rhs.u64_required_opt()
                && self.u64_optional_opt() == rhs.u64_optional_opt()
                && self.u64_repeated().into_iter().eq(rhs.u64_repeated())
                && self.s32_required_opt() == rhs.s32_required_opt()
                && self.s32_optional_opt() == rhs.s32_optional_opt()
                && self.s32_repeated().into_iter().eq(rhs.s32_repeated())
                && self.s64_required_opt() == rhs.s64_required_opt()
                && self.s64_optional_opt() == rhs.s64_optional_opt()
                && self.s64_repeated().into_iter().eq(rhs.s64_repeated())
                && self.fixed32_required_opt() == rhs.fixed32_required_opt()
                && self.fixed32_optional_opt() == rhs.fixed32_optional_opt()
                && self.fixed32_repeated().into_iter().eq(rhs.fixed32_repeated())
                && self.fixed64_required_opt() == rhs.fixed64_required_opt()
                && self.fixed64_optional_opt() == rhs.fixed64_optional_opt()
                && self.fixed64_repeated().into_iter().eq(rhs.fixed64_repeated())
                && self.sfixed32_required_opt() == rhs.sfixed32_required_opt()
                && self.sfixed32_optional_opt() == rhs.sfixed32_optional_opt()
                && self.sfixed32_repeated().into_iter().eq(rhs.sfixed32_repeated())
                && self.sfixed64_required_opt() == rhs.sfixed64_required_opt()
                && self.sfixed64_optional_opt() == rhs.sfixed64_optional_opt()
                && self.sfixed64_repeated().into_iter().eq(rhs.sfixed64_repeated())
                && self.f64_required_opt() == rhs.f64_required_opt()
                && self.f64_optional_opt() == rhs.f64_optional_opt()
                && self.f64_repeated().into_iter().eq(rhs.f64_repeated())
                && self.shared.unknown_fields() == rhs.shared.unknown_fields()
        }
    }
}
#[doc(inline)]
pub use self::_view::*;
#[doc(hidden)]
pub mod _fields {
    mod _root {
        #[allow(unused)]
        pub(crate) use super::super::_root::*;
    }
    mod _puroro {
        #[allow(unused)]
        pub(crate) use super::_root::_puroro::*;
    }
    mod _pinternal {
        #[allow(unused)]
        pub(crate) use super::_root::_pinternal::*;
    }
    #[derive(::std::default::Default)]
    pub struct MsgFields<
        TI32Required,
        TI32Optional,
        TI32Repeated,
        TFloatRequired,
        TFloatOptional,
        TFloatRepeated,
        TBytesRequired,
        TBytesOptional,
        TBytesRepeated,
        TStringRequired,
        TStringOptional,
        TStringRepeated,
        TEnumRequired,
        TEnumOptional,
        TEnumRepeated,
        TSubmsgRequired,
        TSubmsgOptional,
        TSubmsgRepeated,
        TI64Required,
        TI64Optional,
        TI64Repeated,
        TU32Required,
        TU32Optional,
        TU32Repeated,
        TU64Required,
        TU64Optional,
        TU64Repeated,
        TS32Required,
        TS32Optional,
        TS32Repeated,
        TS64Required,
        TS64Optional,
        TS64Repeated,
        TFixed32Required,
        TFixed32Optional,
        TFixed32Repeated,
        TFixed64Required,
        TFixed64Optional,
        TFixed64Repeated,
        TSfixed32Required,
        TSfixed32Optional,
        TSfixed32Repeated,
        TSfixed64Required,
        TSfixed64Optional,
        TSfixed64Repeated,
        TF64Required,
        TF64Optional,
        TF64Repeated,
    > {
        pub i32_required: TI32Required,
        pub i32_optional: TI32Optional,
        pub i32_repeated: TI32Repeated,
        pub float_required: TFloatRequired,
        pub float_optional: TFloatOptional,
        pub float_repeated: TFloatRepeated,
        pub bytes_required: TBytesRequired,
        pub bytes_optional: TBytesOptional,
        pub bytes_repeated: TBytesRepeated,
        pub string_required: TStringRequired,
        pub string_optional: TStringOptional,
        pub string_repeated: TStringRepeated,
        pub enum_required: TEnumRequired,
        pub enum_optional: TEnumOptional,
        pub enum_repeated: TEnumRepeated,
        pub submsg_required: TSubmsgRequired,
        pub submsg_optional: TSubmsgOptional,
        pub submsg_repeated: TSubmsgRepeated,
        pub i64_required: TI64Required,
        pub i64_optional: TI64Optional,
        pub i64_repeated: TI64Repeated,
        pub u32_required: TU32Required,
        pub u32_optional: TU32Optional,
        pub u32_repeated: TU32Repeated,
        pub u64_required: TU64Required,
        pub u64_optional: TU64Optional,
        pub u64_repeated: TU64Repeated,
        pub s32_required: TS32Required,
        pub s32_optional: TS32Optional,
        pub s32_repeated: TS32Repeated,
        pub s64_required: TS64Required,
        pub s64_optional: TS64Optional,
        pub s64_repeated: TS64Repeated,
        pub fixed32_required: TFixed32Required,
        pub fixed32_optional: TFixed32Optional,
        pub fixed32_repeated: TFixed32Repeated,
        pub fixed64_required: TFixed64Required,
        pub fixed64_optional: TFixed64Optional,
        pub fixed64_repeated: TFixed64Repeated,
        pub sfixed32_required: TSfixed32Required,
        pub sfixed32_optional: TSfixed32Optional,
        pub sfixed32_repeated: TSfixed32Repeated,
        pub sfixed64_required: TSfixed64Required,
        pub sfixed64_optional: TSfixed64Optional,
        pub sfixed64_repeated: TSfixed64Repeated,
        pub f64_required: TF64Required,
        pub f64_optional: TF64Optional,
        pub f64_repeated: TF64Repeated,
    }
}
#[doc(hidden)]
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
        match val {
            0i32 => ::std::result::Result::Ok(self::Enum::Zeroth),
            1i32 => ::std::result::Result::Ok(self::Enum::First),
            10i32 => ::std::result::Result::Ok(self::Enum::Tenth),
            _ => {
                ::std::result::Result::Err(
                    self::_puroro::PuroroError::UnknownEnumVariant(val),
                )?
            }
        }
    }
}
