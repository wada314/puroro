// A generated source code by puroro library
// package full_coverage3

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub use _puroro_simple_impl::Msg;
pub mod _puroro_simple_impl {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
    pub struct Msg {
    _bitfield: ::puroro::bitvec::array::BitArray<
        ::puroro::bitvec::order::Lsb0,
        [u32; (15 + 31) / 32],
    >,
    i32_unlabeled: ::puroro::internal::Bare<i32>,
    i32_optional: ::puroro::internal::Bare<i32>,
    i32_repeated: ::std::vec::Vec<i32>,
    float_unlabeled: ::puroro::internal::Bare<f32>,
    float_optional: ::puroro::internal::Bare<f32>,
    float_repeated: ::std::vec::Vec<f32>,
    bytes_unlabeled: ::puroro::internal::Bare<::std::vec::Vec<u8>>,
    bytes_optional: ::puroro::internal::Bare<::std::vec::Vec<u8>>,
    bytes_repeated: ::std::vec::Vec<::std::vec::Vec<u8>>,
    string_unlabeled: ::puroro::internal::Bare<::std::string::String>,
    string_optional: ::puroro::internal::Bare<::std::string::String>,
    string_repeated: ::std::vec::Vec<::std::string::String>,
    enum_unlabeled: ::puroro::internal::Bare<self::_puroro_root::full_coverage3::Enum>,
    enum_optional: ::puroro::internal::Bare<self::_puroro_root::full_coverage3::Enum>,
    enum_repeated: ::std::vec::Vec<self::_puroro_root::full_coverage3::Enum>,
    submsg_unlabeled: ::std::option::Option<::std::boxed::Box<self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_simple_impl::Submsg>>,
    submsg_optional: ::std::option::Option<::std::boxed::Box<self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_simple_impl::Submsg>>,
    submsg_repeated: ::std::vec::Vec<self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_simple_impl::Submsg>,
    i64_unlabeled: ::puroro::internal::Bare<i64>,
    i64_optional: ::puroro::internal::Bare<i64>,
    i64_repeated: ::std::vec::Vec<i64>,
    u32_unlabeled: ::puroro::internal::Bare<u32>,
    u32_optional: ::puroro::internal::Bare<u32>,
    u32_repeated: ::std::vec::Vec<u32>,
    u64_unlabeled: ::puroro::internal::Bare<u64>,
    u64_optional: ::puroro::internal::Bare<u64>,
    u64_repeated: ::std::vec::Vec<u64>,
    s32_unlabeled: ::puroro::internal::Bare<i32>,
    s32_optional: ::puroro::internal::Bare<i32>,
    s32_repeated: ::std::vec::Vec<i32>,
    s64_unlabeled: ::puroro::internal::Bare<i64>,
    s64_optional: ::puroro::internal::Bare<i64>,
    s64_repeated: ::std::vec::Vec<i64>,
    fixed32_unlabeled: ::puroro::internal::Bare<u32>,
    fixed32_optional: ::puroro::internal::Bare<u32>,
    fixed32_repeated: ::std::vec::Vec<u32>,
    fixed64_unlabeled: ::puroro::internal::Bare<u64>,
    fixed64_optional: ::puroro::internal::Bare<u64>,
    fixed64_repeated: ::std::vec::Vec<u64>,
    sfixed32_unlabeled: ::puroro::internal::Bare<i32>,
    sfixed32_optional: ::puroro::internal::Bare<i32>,
    sfixed32_repeated: ::std::vec::Vec<i32>,
    sfixed64_unlabeled: ::puroro::internal::Bare<i64>,
    sfixed64_optional: ::puroro::internal::Bare<i64>,
    sfixed64_repeated: ::std::vec::Vec<i64>,
    f64_unlabeled: ::puroro::internal::Bare<f64>,
    f64_optional: ::puroro::internal::Bare<f64>,
    f64_repeated: ::std::vec::Vec<f64>,
}
    impl ::puroro::Message<Msg> for Msg {}

    impl Msg {
        pub fn new() -> Self {
            Self {
                _bitfield: ::std::default::Default::default(),
                i32_unlabeled: ::std::default::Default::default(),
                i32_optional: ::std::default::Default::default(),
                i32_repeated: ::std::default::Default::default(),
                float_unlabeled: ::std::default::Default::default(),
                float_optional: ::std::default::Default::default(),
                float_repeated: ::std::default::Default::default(),
                bytes_unlabeled: ::std::default::Default::default(),
                bytes_optional: ::std::default::Default::default(),
                bytes_repeated: ::std::default::Default::default(),
                string_unlabeled: ::std::default::Default::default(),
                string_optional: ::std::default::Default::default(),
                string_repeated: ::std::default::Default::default(),
                enum_unlabeled: ::std::default::Default::default(),
                enum_optional: ::std::default::Default::default(),
                enum_repeated: ::std::default::Default::default(),
                submsg_unlabeled: ::std::default::Default::default(),
                submsg_optional: ::std::default::Default::default(),
                submsg_repeated: ::std::default::Default::default(),
                i64_unlabeled: ::std::default::Default::default(),
                i64_optional: ::std::default::Default::default(),
                i64_repeated: ::std::default::Default::default(),
                u32_unlabeled: ::std::default::Default::default(),
                u32_optional: ::std::default::Default::default(),
                u32_repeated: ::std::default::Default::default(),
                u64_unlabeled: ::std::default::Default::default(),
                u64_optional: ::std::default::Default::default(),
                u64_repeated: ::std::default::Default::default(),
                s32_unlabeled: ::std::default::Default::default(),
                s32_optional: ::std::default::Default::default(),
                s32_repeated: ::std::default::Default::default(),
                s64_unlabeled: ::std::default::Default::default(),
                s64_optional: ::std::default::Default::default(),
                s64_repeated: ::std::default::Default::default(),
                fixed32_unlabeled: ::std::default::Default::default(),
                fixed32_optional: ::std::default::Default::default(),
                fixed32_repeated: ::std::default::Default::default(),
                fixed64_unlabeled: ::std::default::Default::default(),
                fixed64_optional: ::std::default::Default::default(),
                fixed64_repeated: ::std::default::Default::default(),
                sfixed32_unlabeled: ::std::default::Default::default(),
                sfixed32_optional: ::std::default::Default::default(),
                sfixed32_repeated: ::std::default::Default::default(),
                sfixed64_unlabeled: ::std::default::Default::default(),
                sfixed64_optional: ::std::default::Default::default(),
                sfixed64_repeated: ::std::default::Default::default(),
                f64_unlabeled: ::std::default::Default::default(),
                f64_optional: ::std::default::Default::default(),
                f64_repeated: ::std::default::Default::default(),
            }
        }
        pub fn i32_unlabeled_opt(&self) -> ::std::option::Option<i32> {
            if !::puroro::internal::IsDefault::is_default(&*self.i32_unlabeled) {
                ::std::option::Option::Some(self.i32_unlabeled.inner())
            } else {
                ::std::option::Option::None
            }
        }

        pub fn has_i32_unlabeled(&self) -> bool {
            Self::i32_unlabeled_opt(self).is_some()
        }

        pub fn i32_unlabeled(&self) -> i32 {
            self.i32_unlabeled_opt()
                .unwrap_or(::std::default::Default::default())
        }
        pub fn i32_optional_opt(&self) -> ::std::option::Option<i32> {
            if self._bitfield.get(0).map_or(false, |v| *v) {
                ::std::option::Option::Some(self.i32_optional.clone().inner())
            } else {
                ::std::option::Option::None
            }
        }

        pub fn has_i32_optional(&self) -> bool {
            Self::i32_optional_opt(self).is_some()
        }

        pub fn i32_optional(&self) -> i32 {
            self.i32_optional_opt()
                .unwrap_or(::std::default::Default::default())
        }
        pub fn i32_repeated(&self) -> &'_ [i32] {
            &self.i32_repeated
        }
        pub fn float_unlabeled_opt(&self) -> ::std::option::Option<f32> {
            if !::puroro::internal::IsDefault::is_default(&*self.float_unlabeled) {
                ::std::option::Option::Some(self.float_unlabeled.inner())
            } else {
                ::std::option::Option::None
            }
        }

        pub fn has_float_unlabeled(&self) -> bool {
            Self::float_unlabeled_opt(self).is_some()
        }

        pub fn float_unlabeled(&self) -> f32 {
            self.float_unlabeled_opt()
                .unwrap_or(::std::default::Default::default())
        }
        pub fn float_optional_opt(&self) -> ::std::option::Option<f32> {
            if self._bitfield.get(1).map_or(false, |v| *v) {
                ::std::option::Option::Some(self.float_optional.clone().inner())
            } else {
                ::std::option::Option::None
            }
        }

        pub fn has_float_optional(&self) -> bool {
            Self::float_optional_opt(self).is_some()
        }

        pub fn float_optional(&self) -> f32 {
            self.float_optional_opt()
                .unwrap_or(::std::default::Default::default())
        }
        pub fn float_repeated(&self) -> &'_ [f32] {
            &self.float_repeated
        }
        pub fn bytes_unlabeled_opt(&self) -> ::std::option::Option<&'_ [u8]> {
            if !::puroro::internal::IsDefault::is_default(&*self.bytes_unlabeled) {
                ::std::option::Option::Some(&self.bytes_unlabeled)
            } else {
                ::std::option::Option::None
            }
        }

        pub fn has_bytes_unlabeled(&self) -> bool {
            Self::bytes_unlabeled_opt(self).is_some()
        }

        pub fn bytes_unlabeled(&self) -> &'_ [u8] {
            self.bytes_unlabeled_opt()
                .unwrap_or(::std::default::Default::default())
        }
        pub fn bytes_optional_opt(&self) -> ::std::option::Option<&'_ [u8]> {
            if self._bitfield.get(2).map_or(false, |v| *v) {
                ::std::option::Option::Some(&self.bytes_optional)
            } else {
                ::std::option::Option::None
            }
        }

        pub fn has_bytes_optional(&self) -> bool {
            Self::bytes_optional_opt(self).is_some()
        }

        pub fn bytes_optional(&self) -> &'_ [u8] {
            self.bytes_optional_opt()
                .unwrap_or(::std::default::Default::default())
        }
        pub fn bytes_repeated(
            &self,
        ) -> &'_ [impl ::std::ops::Deref<Target = [u8]> + ::std::fmt::Debug] {
            &self.bytes_repeated
        }
        pub fn string_unlabeled_opt(&self) -> ::std::option::Option<&'_ str> {
            if !::puroro::internal::IsDefault::is_default(&*self.string_unlabeled) {
                ::std::option::Option::Some(&self.string_unlabeled)
            } else {
                ::std::option::Option::None
            }
        }

        pub fn has_string_unlabeled(&self) -> bool {
            Self::string_unlabeled_opt(self).is_some()
        }

        pub fn string_unlabeled(&self) -> &'_ str {
            self.string_unlabeled_opt()
                .unwrap_or(::std::default::Default::default())
        }
        pub fn string_optional_opt(&self) -> ::std::option::Option<&'_ str> {
            if self._bitfield.get(3).map_or(false, |v| *v) {
                ::std::option::Option::Some(&self.string_optional)
            } else {
                ::std::option::Option::None
            }
        }

        pub fn has_string_optional(&self) -> bool {
            Self::string_optional_opt(self).is_some()
        }

        pub fn string_optional(&self) -> &'_ str {
            self.string_optional_opt()
                .unwrap_or(::std::default::Default::default())
        }
        pub fn string_repeated(
            &self,
        ) -> &'_ [impl ::std::ops::Deref<Target = str> + ::std::fmt::Debug] {
            &self.string_repeated
        }
        pub fn enum_unlabeled_opt(
            &self,
        ) -> ::std::option::Option<self::_puroro_root::full_coverage3::Enum> {
            if !::puroro::internal::IsDefault::is_default(&*self.enum_unlabeled) {
                ::std::option::Option::Some(self.enum_unlabeled.inner())
            } else {
                ::std::option::Option::None
            }
        }

        pub fn has_enum_unlabeled(&self) -> bool {
            Self::enum_unlabeled_opt(self).is_some()
        }

        pub fn enum_unlabeled(&self) -> self::_puroro_root::full_coverage3::Enum {
            self.enum_unlabeled_opt()
                .unwrap_or(::std::default::Default::default())
        }
        pub fn enum_optional_opt(
            &self,
        ) -> ::std::option::Option<self::_puroro_root::full_coverage3::Enum> {
            if self._bitfield.get(4).map_or(false, |v| *v) {
                ::std::option::Option::Some(self.enum_optional.clone().inner())
            } else {
                ::std::option::Option::None
            }
        }

        pub fn has_enum_optional(&self) -> bool {
            Self::enum_optional_opt(self).is_some()
        }

        pub fn enum_optional(&self) -> self::_puroro_root::full_coverage3::Enum {
            self.enum_optional_opt()
                .unwrap_or(::std::default::Default::default())
        }
        pub fn enum_repeated(&self) -> &'_ [self::_puroro_root::full_coverage3::Enum] {
            &self.enum_repeated
        }
        pub fn submsg_unlabeled_opt(&self) -> ::std::option::Option<&'_ self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_simple_impl::Submsg>{
            self.submsg_unlabeled.as_deref()
        }

        pub fn has_submsg_unlabeled(&self) -> bool {
            Self::submsg_unlabeled_opt(self).is_some()
        }

        pub fn submsg_unlabeled(&self) -> ::std::option::Option<&'_ self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_simple_impl::Submsg>{
            self.submsg_unlabeled_opt()
        }
        pub fn submsg_optional_opt(&self) -> ::std::option::Option<&'_ self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_simple_impl::Submsg>{
            self.submsg_optional.as_deref()
        }

        pub fn has_submsg_optional(&self) -> bool {
            Self::submsg_optional_opt(self).is_some()
        }

        pub fn submsg_optional(&self) -> ::std::option::Option<&'_ self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_simple_impl::Submsg>{
            self.submsg_optional_opt()
        }
        pub fn submsg_repeated(&self) -> &'_[self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_simple_impl::Submsg]{
            &self.submsg_repeated
        }
        pub fn i64_unlabeled_opt(&self) -> ::std::option::Option<i64> {
            if !::puroro::internal::IsDefault::is_default(&*self.i64_unlabeled) {
                ::std::option::Option::Some(self.i64_unlabeled.inner())
            } else {
                ::std::option::Option::None
            }
        }

        pub fn has_i64_unlabeled(&self) -> bool {
            Self::i64_unlabeled_opt(self).is_some()
        }

        pub fn i64_unlabeled(&self) -> i64 {
            self.i64_unlabeled_opt()
                .unwrap_or(::std::default::Default::default())
        }
        pub fn i64_optional_opt(&self) -> ::std::option::Option<i64> {
            if self._bitfield.get(5).map_or(false, |v| *v) {
                ::std::option::Option::Some(self.i64_optional.clone().inner())
            } else {
                ::std::option::Option::None
            }
        }

        pub fn has_i64_optional(&self) -> bool {
            Self::i64_optional_opt(self).is_some()
        }

        pub fn i64_optional(&self) -> i64 {
            self.i64_optional_opt()
                .unwrap_or(::std::default::Default::default())
        }
        pub fn i64_repeated(&self) -> &'_ [i64] {
            &self.i64_repeated
        }
        pub fn u32_unlabeled_opt(&self) -> ::std::option::Option<u32> {
            if !::puroro::internal::IsDefault::is_default(&*self.u32_unlabeled) {
                ::std::option::Option::Some(self.u32_unlabeled.inner())
            } else {
                ::std::option::Option::None
            }
        }

        pub fn has_u32_unlabeled(&self) -> bool {
            Self::u32_unlabeled_opt(self).is_some()
        }

        pub fn u32_unlabeled(&self) -> u32 {
            self.u32_unlabeled_opt()
                .unwrap_or(::std::default::Default::default())
        }
        pub fn u32_optional_opt(&self) -> ::std::option::Option<u32> {
            if self._bitfield.get(6).map_or(false, |v| *v) {
                ::std::option::Option::Some(self.u32_optional.clone().inner())
            } else {
                ::std::option::Option::None
            }
        }

        pub fn has_u32_optional(&self) -> bool {
            Self::u32_optional_opt(self).is_some()
        }

        pub fn u32_optional(&self) -> u32 {
            self.u32_optional_opt()
                .unwrap_or(::std::default::Default::default())
        }
        pub fn u32_repeated(&self) -> &'_ [u32] {
            &self.u32_repeated
        }
        pub fn u64_unlabeled_opt(&self) -> ::std::option::Option<u64> {
            if !::puroro::internal::IsDefault::is_default(&*self.u64_unlabeled) {
                ::std::option::Option::Some(self.u64_unlabeled.inner())
            } else {
                ::std::option::Option::None
            }
        }

        pub fn has_u64_unlabeled(&self) -> bool {
            Self::u64_unlabeled_opt(self).is_some()
        }

        pub fn u64_unlabeled(&self) -> u64 {
            self.u64_unlabeled_opt()
                .unwrap_or(::std::default::Default::default())
        }
        pub fn u64_optional_opt(&self) -> ::std::option::Option<u64> {
            if self._bitfield.get(7).map_or(false, |v| *v) {
                ::std::option::Option::Some(self.u64_optional.clone().inner())
            } else {
                ::std::option::Option::None
            }
        }

        pub fn has_u64_optional(&self) -> bool {
            Self::u64_optional_opt(self).is_some()
        }

        pub fn u64_optional(&self) -> u64 {
            self.u64_optional_opt()
                .unwrap_or(::std::default::Default::default())
        }
        pub fn u64_repeated(&self) -> &'_ [u64] {
            &self.u64_repeated
        }
        pub fn s32_unlabeled_opt(&self) -> ::std::option::Option<i32> {
            if !::puroro::internal::IsDefault::is_default(&*self.s32_unlabeled) {
                ::std::option::Option::Some(self.s32_unlabeled.inner())
            } else {
                ::std::option::Option::None
            }
        }

        pub fn has_s32_unlabeled(&self) -> bool {
            Self::s32_unlabeled_opt(self).is_some()
        }

        pub fn s32_unlabeled(&self) -> i32 {
            self.s32_unlabeled_opt()
                .unwrap_or(::std::default::Default::default())
        }
        pub fn s32_optional_opt(&self) -> ::std::option::Option<i32> {
            if self._bitfield.get(8).map_or(false, |v| *v) {
                ::std::option::Option::Some(self.s32_optional.clone().inner())
            } else {
                ::std::option::Option::None
            }
        }

        pub fn has_s32_optional(&self) -> bool {
            Self::s32_optional_opt(self).is_some()
        }

        pub fn s32_optional(&self) -> i32 {
            self.s32_optional_opt()
                .unwrap_or(::std::default::Default::default())
        }
        pub fn s32_repeated(&self) -> &'_ [i32] {
            &self.s32_repeated
        }
        pub fn s64_unlabeled_opt(&self) -> ::std::option::Option<i64> {
            if !::puroro::internal::IsDefault::is_default(&*self.s64_unlabeled) {
                ::std::option::Option::Some(self.s64_unlabeled.inner())
            } else {
                ::std::option::Option::None
            }
        }

        pub fn has_s64_unlabeled(&self) -> bool {
            Self::s64_unlabeled_opt(self).is_some()
        }

        pub fn s64_unlabeled(&self) -> i64 {
            self.s64_unlabeled_opt()
                .unwrap_or(::std::default::Default::default())
        }
        pub fn s64_optional_opt(&self) -> ::std::option::Option<i64> {
            if self._bitfield.get(9).map_or(false, |v| *v) {
                ::std::option::Option::Some(self.s64_optional.clone().inner())
            } else {
                ::std::option::Option::None
            }
        }

        pub fn has_s64_optional(&self) -> bool {
            Self::s64_optional_opt(self).is_some()
        }

        pub fn s64_optional(&self) -> i64 {
            self.s64_optional_opt()
                .unwrap_or(::std::default::Default::default())
        }
        pub fn s64_repeated(&self) -> &'_ [i64] {
            &self.s64_repeated
        }
        pub fn fixed32_unlabeled_opt(&self) -> ::std::option::Option<u32> {
            if !::puroro::internal::IsDefault::is_default(&*self.fixed32_unlabeled) {
                ::std::option::Option::Some(self.fixed32_unlabeled.inner())
            } else {
                ::std::option::Option::None
            }
        }

        pub fn has_fixed32_unlabeled(&self) -> bool {
            Self::fixed32_unlabeled_opt(self).is_some()
        }

        pub fn fixed32_unlabeled(&self) -> u32 {
            self.fixed32_unlabeled_opt()
                .unwrap_or(::std::default::Default::default())
        }
        pub fn fixed32_optional_opt(&self) -> ::std::option::Option<u32> {
            if self._bitfield.get(10).map_or(false, |v| *v) {
                ::std::option::Option::Some(self.fixed32_optional.clone().inner())
            } else {
                ::std::option::Option::None
            }
        }

        pub fn has_fixed32_optional(&self) -> bool {
            Self::fixed32_optional_opt(self).is_some()
        }

        pub fn fixed32_optional(&self) -> u32 {
            self.fixed32_optional_opt()
                .unwrap_or(::std::default::Default::default())
        }
        pub fn fixed32_repeated(&self) -> &'_ [u32] {
            &self.fixed32_repeated
        }
        pub fn fixed64_unlabeled_opt(&self) -> ::std::option::Option<u64> {
            if !::puroro::internal::IsDefault::is_default(&*self.fixed64_unlabeled) {
                ::std::option::Option::Some(self.fixed64_unlabeled.inner())
            } else {
                ::std::option::Option::None
            }
        }

        pub fn has_fixed64_unlabeled(&self) -> bool {
            Self::fixed64_unlabeled_opt(self).is_some()
        }

        pub fn fixed64_unlabeled(&self) -> u64 {
            self.fixed64_unlabeled_opt()
                .unwrap_or(::std::default::Default::default())
        }
        pub fn fixed64_optional_opt(&self) -> ::std::option::Option<u64> {
            if self._bitfield.get(11).map_or(false, |v| *v) {
                ::std::option::Option::Some(self.fixed64_optional.clone().inner())
            } else {
                ::std::option::Option::None
            }
        }

        pub fn has_fixed64_optional(&self) -> bool {
            Self::fixed64_optional_opt(self).is_some()
        }

        pub fn fixed64_optional(&self) -> u64 {
            self.fixed64_optional_opt()
                .unwrap_or(::std::default::Default::default())
        }
        pub fn fixed64_repeated(&self) -> &'_ [u64] {
            &self.fixed64_repeated
        }
        pub fn sfixed32_unlabeled_opt(&self) -> ::std::option::Option<i32> {
            if !::puroro::internal::IsDefault::is_default(&*self.sfixed32_unlabeled) {
                ::std::option::Option::Some(self.sfixed32_unlabeled.inner())
            } else {
                ::std::option::Option::None
            }
        }

        pub fn has_sfixed32_unlabeled(&self) -> bool {
            Self::sfixed32_unlabeled_opt(self).is_some()
        }

        pub fn sfixed32_unlabeled(&self) -> i32 {
            self.sfixed32_unlabeled_opt()
                .unwrap_or(::std::default::Default::default())
        }
        pub fn sfixed32_optional_opt(&self) -> ::std::option::Option<i32> {
            if self._bitfield.get(12).map_or(false, |v| *v) {
                ::std::option::Option::Some(self.sfixed32_optional.clone().inner())
            } else {
                ::std::option::Option::None
            }
        }

        pub fn has_sfixed32_optional(&self) -> bool {
            Self::sfixed32_optional_opt(self).is_some()
        }

        pub fn sfixed32_optional(&self) -> i32 {
            self.sfixed32_optional_opt()
                .unwrap_or(::std::default::Default::default())
        }
        pub fn sfixed32_repeated(&self) -> &'_ [i32] {
            &self.sfixed32_repeated
        }
        pub fn sfixed64_unlabeled_opt(&self) -> ::std::option::Option<i64> {
            if !::puroro::internal::IsDefault::is_default(&*self.sfixed64_unlabeled) {
                ::std::option::Option::Some(self.sfixed64_unlabeled.inner())
            } else {
                ::std::option::Option::None
            }
        }

        pub fn has_sfixed64_unlabeled(&self) -> bool {
            Self::sfixed64_unlabeled_opt(self).is_some()
        }

        pub fn sfixed64_unlabeled(&self) -> i64 {
            self.sfixed64_unlabeled_opt()
                .unwrap_or(::std::default::Default::default())
        }
        pub fn sfixed64_optional_opt(&self) -> ::std::option::Option<i64> {
            if self._bitfield.get(13).map_or(false, |v| *v) {
                ::std::option::Option::Some(self.sfixed64_optional.clone().inner())
            } else {
                ::std::option::Option::None
            }
        }

        pub fn has_sfixed64_optional(&self) -> bool {
            Self::sfixed64_optional_opt(self).is_some()
        }

        pub fn sfixed64_optional(&self) -> i64 {
            self.sfixed64_optional_opt()
                .unwrap_or(::std::default::Default::default())
        }
        pub fn sfixed64_repeated(&self) -> &'_ [i64] {
            &self.sfixed64_repeated
        }
        pub fn f64_unlabeled_opt(&self) -> ::std::option::Option<f64> {
            if !::puroro::internal::IsDefault::is_default(&*self.f64_unlabeled) {
                ::std::option::Option::Some(self.f64_unlabeled.inner())
            } else {
                ::std::option::Option::None
            }
        }

        pub fn has_f64_unlabeled(&self) -> bool {
            Self::f64_unlabeled_opt(self).is_some()
        }

        pub fn f64_unlabeled(&self) -> f64 {
            self.f64_unlabeled_opt()
                .unwrap_or(::std::default::Default::default())
        }
        pub fn f64_optional_opt(&self) -> ::std::option::Option<f64> {
            if self._bitfield.get(14).map_or(false, |v| *v) {
                ::std::option::Option::Some(self.f64_optional.clone().inner())
            } else {
                ::std::option::Option::None
            }
        }

        pub fn has_f64_optional(&self) -> bool {
            Self::f64_optional_opt(self).is_some()
        }

        pub fn f64_optional(&self) -> f64 {
            self.f64_optional_opt()
                .unwrap_or(::std::default::Default::default())
        }
        pub fn f64_repeated(&self) -> &'_ [f64] {
            &self.f64_repeated
        }
        pub fn clear_i32_unlabeled(&mut self) {
            self.i32_unlabeled = ::std::default::Default::default();
        }
        pub fn i32_unlabeled_mut(&mut self) -> &'_ mut i32 {
            if !self.has_i32_unlabeled() {
                self.i32_unlabeled = ::std::default::Default::default();
            }
            &mut self.i32_unlabeled
        }
        pub fn clear_i32_optional(&mut self) {
            self._bitfield.set(0, false);
        }
        pub fn i32_optional_mut(&mut self) -> &'_ mut i32 {
            if !self.has_i32_optional() {
                self.i32_optional = ::std::default::Default::default();
                self._bitfield.set(0, true);
            }
            &mut self.i32_optional
        }
        pub fn i32_repeated_mut(&mut self) -> &'_ mut ::std::vec::Vec<i32> {
            &mut self.i32_repeated
        }
        pub fn clear_float_unlabeled(&mut self) {
            self.float_unlabeled = ::std::default::Default::default();
        }
        pub fn float_unlabeled_mut(&mut self) -> &'_ mut f32 {
            if !self.has_float_unlabeled() {
                self.float_unlabeled = ::std::default::Default::default();
            }
            &mut self.float_unlabeled
        }
        pub fn clear_float_optional(&mut self) {
            self._bitfield.set(1, false);
        }
        pub fn float_optional_mut(&mut self) -> &'_ mut f32 {
            if !self.has_float_optional() {
                self.float_optional = ::std::default::Default::default();
                self._bitfield.set(1, true);
            }
            &mut self.float_optional
        }
        pub fn float_repeated_mut(&mut self) -> &'_ mut ::std::vec::Vec<f32> {
            &mut self.float_repeated
        }
        pub fn clear_bytes_unlabeled(&mut self) {
            self.bytes_unlabeled = ::std::default::Default::default();
        }
        pub fn bytes_unlabeled_mut(&mut self) -> &'_ mut ::std::vec::Vec<u8> {
            if !self.has_bytes_unlabeled() {
                self.bytes_unlabeled = ::std::default::Default::default();
            }
            &mut self.bytes_unlabeled
        }
        pub fn clear_bytes_optional(&mut self) {
            self._bitfield.set(2, false);
        }
        pub fn bytes_optional_mut(&mut self) -> &'_ mut ::std::vec::Vec<u8> {
            if !self.has_bytes_optional() {
                self.bytes_optional = ::std::default::Default::default();
                self._bitfield.set(2, true);
            }
            &mut self.bytes_optional
        }
        pub fn bytes_repeated_mut(&mut self) -> &'_ mut ::std::vec::Vec<::std::vec::Vec<u8>> {
            &mut self.bytes_repeated
        }
        pub fn clear_string_unlabeled(&mut self) {
            self.string_unlabeled = ::std::default::Default::default();
        }
        pub fn string_unlabeled_mut(&mut self) -> &'_ mut ::std::string::String {
            if !self.has_string_unlabeled() {
                self.string_unlabeled = ::std::default::Default::default();
            }
            &mut self.string_unlabeled
        }
        pub fn clear_string_optional(&mut self) {
            self._bitfield.set(3, false);
        }
        pub fn string_optional_mut(&mut self) -> &'_ mut ::std::string::String {
            if !self.has_string_optional() {
                self.string_optional = ::std::default::Default::default();
                self._bitfield.set(3, true);
            }
            &mut self.string_optional
        }
        pub fn string_repeated_mut(&mut self) -> &'_ mut ::std::vec::Vec<::std::string::String> {
            &mut self.string_repeated
        }
        pub fn clear_enum_unlabeled(&mut self) {
            self.enum_unlabeled = ::std::default::Default::default();
        }
        pub fn enum_unlabeled_mut(&mut self) -> &'_ mut self::_puroro_root::full_coverage3::Enum {
            if !self.has_enum_unlabeled() {
                self.enum_unlabeled = ::std::default::Default::default();
            }
            &mut self.enum_unlabeled
        }
        pub fn clear_enum_optional(&mut self) {
            self._bitfield.set(4, false);
        }
        pub fn enum_optional_mut(&mut self) -> &'_ mut self::_puroro_root::full_coverage3::Enum {
            if !self.has_enum_optional() {
                self.enum_optional = ::std::default::Default::default();
                self._bitfield.set(4, true);
            }
            &mut self.enum_optional
        }
        pub fn enum_repeated_mut(
            &mut self,
        ) -> &'_ mut ::std::vec::Vec<self::_puroro_root::full_coverage3::Enum> {
            &mut self.enum_repeated
        }
        pub fn clear_submsg_unlabeled(&mut self) {
            self.submsg_unlabeled = ::std::default::Default::default();
        }
        pub fn submsg_unlabeled_mut(&mut self) -> &'_ mut self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_simple_impl::Submsg{
            if !self.has_submsg_unlabeled() {
                self.submsg_unlabeled = ::std::default::Default::default();
            }
            self.submsg_unlabeled
                .get_or_insert_with(::std::default::Default::default)
        }
        pub fn clear_submsg_optional(&mut self) {
            self.submsg_optional = ::std::default::Default::default();
        }
        pub fn submsg_optional_mut(&mut self) -> &'_ mut self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_simple_impl::Submsg{
            if !self.has_submsg_optional() {
                self.submsg_optional = ::std::default::Default::default();
            }
            self.submsg_optional
                .get_or_insert_with(::std::default::Default::default)
        }
        pub fn submsg_repeated_mut(
            &mut self,
        ) -> &'_ mut ::std::vec::Vec<
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_simple_impl::Submsg,
        > {
            &mut self.submsg_repeated
        }
        pub fn clear_i64_unlabeled(&mut self) {
            self.i64_unlabeled = ::std::default::Default::default();
        }
        pub fn i64_unlabeled_mut(&mut self) -> &'_ mut i64 {
            if !self.has_i64_unlabeled() {
                self.i64_unlabeled = ::std::default::Default::default();
            }
            &mut self.i64_unlabeled
        }
        pub fn clear_i64_optional(&mut self) {
            self._bitfield.set(5, false);
        }
        pub fn i64_optional_mut(&mut self) -> &'_ mut i64 {
            if !self.has_i64_optional() {
                self.i64_optional = ::std::default::Default::default();
                self._bitfield.set(5, true);
            }
            &mut self.i64_optional
        }
        pub fn i64_repeated_mut(&mut self) -> &'_ mut ::std::vec::Vec<i64> {
            &mut self.i64_repeated
        }
        pub fn clear_u32_unlabeled(&mut self) {
            self.u32_unlabeled = ::std::default::Default::default();
        }
        pub fn u32_unlabeled_mut(&mut self) -> &'_ mut u32 {
            if !self.has_u32_unlabeled() {
                self.u32_unlabeled = ::std::default::Default::default();
            }
            &mut self.u32_unlabeled
        }
        pub fn clear_u32_optional(&mut self) {
            self._bitfield.set(6, false);
        }
        pub fn u32_optional_mut(&mut self) -> &'_ mut u32 {
            if !self.has_u32_optional() {
                self.u32_optional = ::std::default::Default::default();
                self._bitfield.set(6, true);
            }
            &mut self.u32_optional
        }
        pub fn u32_repeated_mut(&mut self) -> &'_ mut ::std::vec::Vec<u32> {
            &mut self.u32_repeated
        }
        pub fn clear_u64_unlabeled(&mut self) {
            self.u64_unlabeled = ::std::default::Default::default();
        }
        pub fn u64_unlabeled_mut(&mut self) -> &'_ mut u64 {
            if !self.has_u64_unlabeled() {
                self.u64_unlabeled = ::std::default::Default::default();
            }
            &mut self.u64_unlabeled
        }
        pub fn clear_u64_optional(&mut self) {
            self._bitfield.set(7, false);
        }
        pub fn u64_optional_mut(&mut self) -> &'_ mut u64 {
            if !self.has_u64_optional() {
                self.u64_optional = ::std::default::Default::default();
                self._bitfield.set(7, true);
            }
            &mut self.u64_optional
        }
        pub fn u64_repeated_mut(&mut self) -> &'_ mut ::std::vec::Vec<u64> {
            &mut self.u64_repeated
        }
        pub fn clear_s32_unlabeled(&mut self) {
            self.s32_unlabeled = ::std::default::Default::default();
        }
        pub fn s32_unlabeled_mut(&mut self) -> &'_ mut i32 {
            if !self.has_s32_unlabeled() {
                self.s32_unlabeled = ::std::default::Default::default();
            }
            &mut self.s32_unlabeled
        }
        pub fn clear_s32_optional(&mut self) {
            self._bitfield.set(8, false);
        }
        pub fn s32_optional_mut(&mut self) -> &'_ mut i32 {
            if !self.has_s32_optional() {
                self.s32_optional = ::std::default::Default::default();
                self._bitfield.set(8, true);
            }
            &mut self.s32_optional
        }
        pub fn s32_repeated_mut(&mut self) -> &'_ mut ::std::vec::Vec<i32> {
            &mut self.s32_repeated
        }
        pub fn clear_s64_unlabeled(&mut self) {
            self.s64_unlabeled = ::std::default::Default::default();
        }
        pub fn s64_unlabeled_mut(&mut self) -> &'_ mut i64 {
            if !self.has_s64_unlabeled() {
                self.s64_unlabeled = ::std::default::Default::default();
            }
            &mut self.s64_unlabeled
        }
        pub fn clear_s64_optional(&mut self) {
            self._bitfield.set(9, false);
        }
        pub fn s64_optional_mut(&mut self) -> &'_ mut i64 {
            if !self.has_s64_optional() {
                self.s64_optional = ::std::default::Default::default();
                self._bitfield.set(9, true);
            }
            &mut self.s64_optional
        }
        pub fn s64_repeated_mut(&mut self) -> &'_ mut ::std::vec::Vec<i64> {
            &mut self.s64_repeated
        }
        pub fn clear_fixed32_unlabeled(&mut self) {
            self.fixed32_unlabeled = ::std::default::Default::default();
        }
        pub fn fixed32_unlabeled_mut(&mut self) -> &'_ mut u32 {
            if !self.has_fixed32_unlabeled() {
                self.fixed32_unlabeled = ::std::default::Default::default();
            }
            &mut self.fixed32_unlabeled
        }
        pub fn clear_fixed32_optional(&mut self) {
            self._bitfield.set(10, false);
        }
        pub fn fixed32_optional_mut(&mut self) -> &'_ mut u32 {
            if !self.has_fixed32_optional() {
                self.fixed32_optional = ::std::default::Default::default();
                self._bitfield.set(10, true);
            }
            &mut self.fixed32_optional
        }
        pub fn fixed32_repeated_mut(&mut self) -> &'_ mut ::std::vec::Vec<u32> {
            &mut self.fixed32_repeated
        }
        pub fn clear_fixed64_unlabeled(&mut self) {
            self.fixed64_unlabeled = ::std::default::Default::default();
        }
        pub fn fixed64_unlabeled_mut(&mut self) -> &'_ mut u64 {
            if !self.has_fixed64_unlabeled() {
                self.fixed64_unlabeled = ::std::default::Default::default();
            }
            &mut self.fixed64_unlabeled
        }
        pub fn clear_fixed64_optional(&mut self) {
            self._bitfield.set(11, false);
        }
        pub fn fixed64_optional_mut(&mut self) -> &'_ mut u64 {
            if !self.has_fixed64_optional() {
                self.fixed64_optional = ::std::default::Default::default();
                self._bitfield.set(11, true);
            }
            &mut self.fixed64_optional
        }
        pub fn fixed64_repeated_mut(&mut self) -> &'_ mut ::std::vec::Vec<u64> {
            &mut self.fixed64_repeated
        }
        pub fn clear_sfixed32_unlabeled(&mut self) {
            self.sfixed32_unlabeled = ::std::default::Default::default();
        }
        pub fn sfixed32_unlabeled_mut(&mut self) -> &'_ mut i32 {
            if !self.has_sfixed32_unlabeled() {
                self.sfixed32_unlabeled = ::std::default::Default::default();
            }
            &mut self.sfixed32_unlabeled
        }
        pub fn clear_sfixed32_optional(&mut self) {
            self._bitfield.set(12, false);
        }
        pub fn sfixed32_optional_mut(&mut self) -> &'_ mut i32 {
            if !self.has_sfixed32_optional() {
                self.sfixed32_optional = ::std::default::Default::default();
                self._bitfield.set(12, true);
            }
            &mut self.sfixed32_optional
        }
        pub fn sfixed32_repeated_mut(&mut self) -> &'_ mut ::std::vec::Vec<i32> {
            &mut self.sfixed32_repeated
        }
        pub fn clear_sfixed64_unlabeled(&mut self) {
            self.sfixed64_unlabeled = ::std::default::Default::default();
        }
        pub fn sfixed64_unlabeled_mut(&mut self) -> &'_ mut i64 {
            if !self.has_sfixed64_unlabeled() {
                self.sfixed64_unlabeled = ::std::default::Default::default();
            }
            &mut self.sfixed64_unlabeled
        }
        pub fn clear_sfixed64_optional(&mut self) {
            self._bitfield.set(13, false);
        }
        pub fn sfixed64_optional_mut(&mut self) -> &'_ mut i64 {
            if !self.has_sfixed64_optional() {
                self.sfixed64_optional = ::std::default::Default::default();
                self._bitfield.set(13, true);
            }
            &mut self.sfixed64_optional
        }
        pub fn sfixed64_repeated_mut(&mut self) -> &'_ mut ::std::vec::Vec<i64> {
            &mut self.sfixed64_repeated
        }
        pub fn clear_f64_unlabeled(&mut self) {
            self.f64_unlabeled = ::std::default::Default::default();
        }
        pub fn f64_unlabeled_mut(&mut self) -> &'_ mut f64 {
            if !self.has_f64_unlabeled() {
                self.f64_unlabeled = ::std::default::Default::default();
            }
            &mut self.f64_unlabeled
        }
        pub fn clear_f64_optional(&mut self) {
            self._bitfield.set(14, false);
        }
        pub fn f64_optional_mut(&mut self) -> &'_ mut f64 {
            if !self.has_f64_optional() {
                self.f64_optional = ::std::default::Default::default();
                self._bitfield.set(14, true);
            }
            &mut self.f64_optional
        }
        pub fn f64_repeated_mut(&mut self) -> &'_ mut ::std::vec::Vec<f64> {
            &mut self.f64_repeated
        }
    }

    impl super::_puroro_traits::MsgTrait for Msg {
        fn i32_unlabeled_opt<'this>(&'this self) -> Option<i32> {
            <self::Msg>::i32_unlabeled_opt(self)
        }
        fn i32_optional_opt<'this>(&'this self) -> Option<i32> {
            <self::Msg>::i32_optional_opt(self)
        }
        type I32RepeatedRepeatedType<'this> =
            ::puroro::CloneThenIntoRepeatedField<'this, ::std::vec::Vec<i32>, i32, i32>;

        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.i32_repeated)
        }
        fn float_unlabeled_opt<'this>(&'this self) -> Option<f32> {
            <self::Msg>::float_unlabeled_opt(self)
        }
        fn float_optional_opt<'this>(&'this self) -> Option<f32> {
            <self::Msg>::float_optional_opt(self)
        }
        type FloatRepeatedRepeatedType<'this> =
            ::puroro::CloneThenIntoRepeatedField<'this, ::std::vec::Vec<f32>, f32, f32>;

        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.float_repeated)
        }
        fn bytes_unlabeled_opt<'this>(&'this self) -> Option<&'this [u8]> {
            <self::Msg>::bytes_unlabeled_opt(self)
        }
        fn bytes_optional_opt<'this>(&'this self) -> Option<&'this [u8]> {
            <self::Msg>::bytes_optional_opt(self)
        }
        type BytesRepeatedRepeatedType<'this> = ::puroro::AsRefRepeatedField<
            'this,
            ::std::vec::Vec<::std::vec::Vec<u8>>,
            ::std::vec::Vec<u8>,
            [u8],
        >;

        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::AsRefRepeatedField::new(&self.bytes_repeated)
        }
        fn string_unlabeled_opt<'this>(&'this self) -> Option<&'this str> {
            <self::Msg>::string_unlabeled_opt(self)
        }
        fn string_optional_opt<'this>(&'this self) -> Option<&'this str> {
            <self::Msg>::string_optional_opt(self)
        }
        type StringRepeatedRepeatedType<'this> = ::puroro::AsRefRepeatedField<
            'this,
            ::std::vec::Vec<::std::string::String>,
            ::std::string::String,
            str,
        >;

        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::AsRefRepeatedField::new(&self.string_repeated)
        }
        fn enum_unlabeled_opt<'this>(
            &'this self,
        ) -> Option<self::_puroro_root::full_coverage3::Enum> {
            <self::Msg>::enum_unlabeled_opt(self)
        }
        fn enum_optional_opt<'this>(
            &'this self,
        ) -> Option<self::_puroro_root::full_coverage3::Enum> {
            <self::Msg>::enum_optional_opt(self)
        }
        type EnumRepeatedRepeatedType<'this> = ::puroro::CloneThenIntoRepeatedField<
            'this,
            ::std::vec::Vec<self::_puroro_root::full_coverage3::Enum>,
            self::_puroro_root::full_coverage3::Enum,
            self::_puroro_root::full_coverage3::Enum,
        >;

        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.enum_repeated)
        }
        type SubmsgUnlabeledMessageType<'this> where Self: 'this = &'this self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_simple_impl::Submsg;
        fn submsg_unlabeled_opt<'this>(
            &'this self,
        ) -> Option<Self::SubmsgUnlabeledMessageType<'this>> {
            <self::Msg>::submsg_unlabeled_opt(self)
        }
        type SubmsgOptionalMessageType<'this> where Self: 'this = &'this self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_simple_impl::Submsg;
        fn submsg_optional_opt<'this>(
            &'this self,
        ) -> Option<Self::SubmsgOptionalMessageType<'this>> {
            <self::Msg>::submsg_optional_opt(self)
        }
        type SubmsgRepeatedMessageType<'this> where Self: 'this = &'this self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_simple_impl::Submsg;
        type SubmsgRepeatedRepeatedType<'this> =
    &'this [self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_simple_impl::Submsg];

        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            &self.submsg_repeated
        }
        fn i64_unlabeled_opt<'this>(&'this self) -> Option<i64> {
            <self::Msg>::i64_unlabeled_opt(self)
        }
        fn i64_optional_opt<'this>(&'this self) -> Option<i64> {
            <self::Msg>::i64_optional_opt(self)
        }
        type I64RepeatedRepeatedType<'this> =
            ::puroro::CloneThenIntoRepeatedField<'this, ::std::vec::Vec<i64>, i64, i64>;

        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.i64_repeated)
        }
        fn u32_unlabeled_opt<'this>(&'this self) -> Option<u32> {
            <self::Msg>::u32_unlabeled_opt(self)
        }
        fn u32_optional_opt<'this>(&'this self) -> Option<u32> {
            <self::Msg>::u32_optional_opt(self)
        }
        type U32RepeatedRepeatedType<'this> =
            ::puroro::CloneThenIntoRepeatedField<'this, ::std::vec::Vec<u32>, u32, u32>;

        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.u32_repeated)
        }
        fn u64_unlabeled_opt<'this>(&'this self) -> Option<u64> {
            <self::Msg>::u64_unlabeled_opt(self)
        }
        fn u64_optional_opt<'this>(&'this self) -> Option<u64> {
            <self::Msg>::u64_optional_opt(self)
        }
        type U64RepeatedRepeatedType<'this> =
            ::puroro::CloneThenIntoRepeatedField<'this, ::std::vec::Vec<u64>, u64, u64>;

        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.u64_repeated)
        }
        fn s32_unlabeled_opt<'this>(&'this self) -> Option<i32> {
            <self::Msg>::s32_unlabeled_opt(self)
        }
        fn s32_optional_opt<'this>(&'this self) -> Option<i32> {
            <self::Msg>::s32_optional_opt(self)
        }
        type S32RepeatedRepeatedType<'this> =
            ::puroro::CloneThenIntoRepeatedField<'this, ::std::vec::Vec<i32>, i32, i32>;

        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.s32_repeated)
        }
        fn s64_unlabeled_opt<'this>(&'this self) -> Option<i64> {
            <self::Msg>::s64_unlabeled_opt(self)
        }
        fn s64_optional_opt<'this>(&'this self) -> Option<i64> {
            <self::Msg>::s64_optional_opt(self)
        }
        type S64RepeatedRepeatedType<'this> =
            ::puroro::CloneThenIntoRepeatedField<'this, ::std::vec::Vec<i64>, i64, i64>;

        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.s64_repeated)
        }
        fn fixed32_unlabeled_opt<'this>(&'this self) -> Option<u32> {
            <self::Msg>::fixed32_unlabeled_opt(self)
        }
        fn fixed32_optional_opt<'this>(&'this self) -> Option<u32> {
            <self::Msg>::fixed32_optional_opt(self)
        }
        type Fixed32RepeatedRepeatedType<'this> =
            ::puroro::CloneThenIntoRepeatedField<'this, ::std::vec::Vec<u32>, u32, u32>;

        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.fixed32_repeated)
        }
        fn fixed64_unlabeled_opt<'this>(&'this self) -> Option<u64> {
            <self::Msg>::fixed64_unlabeled_opt(self)
        }
        fn fixed64_optional_opt<'this>(&'this self) -> Option<u64> {
            <self::Msg>::fixed64_optional_opt(self)
        }
        type Fixed64RepeatedRepeatedType<'this> =
            ::puroro::CloneThenIntoRepeatedField<'this, ::std::vec::Vec<u64>, u64, u64>;

        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.fixed64_repeated)
        }
        fn sfixed32_unlabeled_opt<'this>(&'this self) -> Option<i32> {
            <self::Msg>::sfixed32_unlabeled_opt(self)
        }
        fn sfixed32_optional_opt<'this>(&'this self) -> Option<i32> {
            <self::Msg>::sfixed32_optional_opt(self)
        }
        type Sfixed32RepeatedRepeatedType<'this> =
            ::puroro::CloneThenIntoRepeatedField<'this, ::std::vec::Vec<i32>, i32, i32>;

        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.sfixed32_repeated)
        }
        fn sfixed64_unlabeled_opt<'this>(&'this self) -> Option<i64> {
            <self::Msg>::sfixed64_unlabeled_opt(self)
        }
        fn sfixed64_optional_opt<'this>(&'this self) -> Option<i64> {
            <self::Msg>::sfixed64_optional_opt(self)
        }
        type Sfixed64RepeatedRepeatedType<'this> =
            ::puroro::CloneThenIntoRepeatedField<'this, ::std::vec::Vec<i64>, i64, i64>;

        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.sfixed64_repeated)
        }
        fn f64_unlabeled_opt<'this>(&'this self) -> Option<f64> {
            <self::Msg>::f64_unlabeled_opt(self)
        }
        fn f64_optional_opt<'this>(&'this self) -> Option<f64> {
            <self::Msg>::f64_optional_opt(self)
        }
        type F64RepeatedRepeatedType<'this> =
            ::puroro::CloneThenIntoRepeatedField<'this, ::std::vec::Vec<f64>, f64, f64>;

        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.f64_repeated)
        }
    }

    impl ::puroro::MessageRepresentativeImpl for Msg {}

    impl ::puroro::internal::de::DeserMessageFromBytesIter for Msg {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::internal::types::FieldData<
                &mut ::puroro::internal::de::from_iter::ScopedIter<I>,
            >,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro::internal::impls::simple::de::DeserFieldFromBytesIter;
            match field_number {
            1 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::Int32
                >::deser_field(&mut self.i32_unlabeled, data)
            }
            2 => {
                self._bitfield.set(0, true);
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Int32
                >::deser_field(&mut self.i32_optional, data)
            }
            3 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Int32
                >::deser_field(&mut self.i32_repeated, data)
            }
            11 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::Float
                >::deser_field(&mut self.float_unlabeled, data)
            }
            12 => {
                self._bitfield.set(1, true);
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Float
                >::deser_field(&mut self.float_optional, data)
            }
            13 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Float
                >::deser_field(&mut self.float_repeated, data)
            }
            21 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::Bytes
                >::deser_field(&mut self.bytes_unlabeled, data)
            }
            22 => {
                self._bitfield.set(2, true);
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Bytes
                >::deser_field(&mut self.bytes_optional, data)
            }
            23 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Bytes
                >::deser_field(&mut self.bytes_repeated, data)
            }
            31 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::String
                >::deser_field(&mut self.string_unlabeled, data)
            }
            32 => {
                self._bitfield.set(3, true);
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::String
                >::deser_field(&mut self.string_optional, data)
            }
            33 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::String
                >::deser_field(&mut self.string_repeated, data)
            }
            41 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::Enum3<self::_puroro_root::full_coverage3::Enum>
                >::deser_field(&mut self.enum_unlabeled, data)
            }
            42 => {
                self._bitfield.set(4, true);
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Enum3<self::_puroro_root::full_coverage3::Enum>
                >::deser_field(&mut self.enum_optional, data)
            }
            43 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Enum3<self::_puroro_root::full_coverage3::Enum>
                >::deser_field(&mut self.enum_repeated, data)
            }
            51 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::Message<::std::boxed::Box<self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_simple_impl::Submsg>>
                >::deser_field(&mut self.submsg_unlabeled, data)
            }
            52 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Message<::std::boxed::Box<self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_simple_impl::Submsg>>
                >::deser_field(&mut self.submsg_optional, data)
            }
            53 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_simple_impl::Submsg>
                >::deser_field(&mut self.submsg_repeated, data)
            }
            101 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::Int64
                >::deser_field(&mut self.i64_unlabeled, data)
            }
            102 => {
                self._bitfield.set(5, true);
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Int64
                >::deser_field(&mut self.i64_optional, data)
            }
            103 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Int64
                >::deser_field(&mut self.i64_repeated, data)
            }
            111 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::UInt32
                >::deser_field(&mut self.u32_unlabeled, data)
            }
            112 => {
                self._bitfield.set(6, true);
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::UInt32
                >::deser_field(&mut self.u32_optional, data)
            }
            113 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::UInt32
                >::deser_field(&mut self.u32_repeated, data)
            }
            121 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::UInt64
                >::deser_field(&mut self.u64_unlabeled, data)
            }
            122 => {
                self._bitfield.set(7, true);
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::UInt64
                >::deser_field(&mut self.u64_optional, data)
            }
            123 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::UInt64
                >::deser_field(&mut self.u64_repeated, data)
            }
            131 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::SInt32
                >::deser_field(&mut self.s32_unlabeled, data)
            }
            132 => {
                self._bitfield.set(8, true);
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::SInt32
                >::deser_field(&mut self.s32_optional, data)
            }
            133 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::SInt32
                >::deser_field(&mut self.s32_repeated, data)
            }
            141 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::SInt64
                >::deser_field(&mut self.s64_unlabeled, data)
            }
            142 => {
                self._bitfield.set(9, true);
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::SInt64
                >::deser_field(&mut self.s64_optional, data)
            }
            143 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::SInt64
                >::deser_field(&mut self.s64_repeated, data)
            }
            151 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::Fixed32
                >::deser_field(&mut self.fixed32_unlabeled, data)
            }
            152 => {
                self._bitfield.set(10, true);
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Fixed32
                >::deser_field(&mut self.fixed32_optional, data)
            }
            153 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Fixed32
                >::deser_field(&mut self.fixed32_repeated, data)
            }
            161 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::Fixed64
                >::deser_field(&mut self.fixed64_unlabeled, data)
            }
            162 => {
                self._bitfield.set(11, true);
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Fixed64
                >::deser_field(&mut self.fixed64_optional, data)
            }
            163 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Fixed64
                >::deser_field(&mut self.fixed64_repeated, data)
            }
            171 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::SFixed32
                >::deser_field(&mut self.sfixed32_unlabeled, data)
            }
            172 => {
                self._bitfield.set(12, true);
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::SFixed32
                >::deser_field(&mut self.sfixed32_optional, data)
            }
            173 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::SFixed32
                >::deser_field(&mut self.sfixed32_repeated, data)
            }
            181 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::SFixed64
                >::deser_field(&mut self.sfixed64_unlabeled, data)
            }
            182 => {
                self._bitfield.set(13, true);
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::SFixed64
                >::deser_field(&mut self.sfixed64_optional, data)
            }
            183 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::SFixed64
                >::deser_field(&mut self.sfixed64_repeated, data)
            }
            191 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::Double
                >::deser_field(&mut self.f64_unlabeled, data)
            }
            192 => {
                self._bitfield.set(14, true);
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Double
                >::deser_field(&mut self.f64_optional, data)
            }
            193 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Double
                >::deser_field(&mut self.f64_repeated, data)
            }

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::internal::se::SerMessageToIoWrite for Msg
    where
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::i32_unlabeled_opt(self),
                1,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::i32_optional_opt(self),
                2,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::i32_repeated(self),
                3,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Float,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::float_unlabeled_opt(self),
                11,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Float,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::float_optional_opt(self),
                12,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Float,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::float_repeated(self),
                13,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Bytes,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::bytes_unlabeled_opt(self),
                21,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bytes,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::bytes_optional_opt(self),
                22,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Bytes,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::bytes_repeated(self),
                23,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::string_unlabeled_opt(self),
                31,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::string_optional_opt(self),
                32,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::string_repeated(self),
                33,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Enum3<self::_puroro_root::full_coverage3::Enum>,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::enum_unlabeled_opt(self),
                41,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Enum3<self::_puroro_root::full_coverage3::Enum>,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::enum_optional_opt(self),
                42,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Enum3<self::_puroro_root::full_coverage3::Enum>,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::enum_repeated(self),
                43,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Message<
                    <Self as super::_puroro_traits::MsgTrait>::SubmsgUnlabeledMessageType<'_>,
                >,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::submsg_unlabeled_opt(self),
                51,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    <Self as super::_puroro_traits::MsgTrait>::SubmsgOptionalMessageType<'_>,
                >,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::submsg_optional_opt(self),
                52,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    <Self as super::_puroro_traits::MsgTrait>::SubmsgRepeatedMessageType<'_>,
                >,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::submsg_repeated(self),
                53,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Int64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::i64_unlabeled_opt(self),
                101,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::i64_optional_opt(self),
                102,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Int64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::i64_repeated(self),
                103,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::UInt32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::u32_unlabeled_opt(self),
                111,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::UInt32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::u32_optional_opt(self),
                112,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::UInt32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::u32_repeated(self),
                113,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::UInt64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::u64_unlabeled_opt(self),
                121,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::UInt64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::u64_optional_opt(self),
                122,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::UInt64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::u64_repeated(self),
                123,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::SInt32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::s32_unlabeled_opt(self),
                131,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::SInt32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::s32_optional_opt(self),
                132,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::SInt32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::s32_repeated(self),
                133,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::SInt64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::s64_unlabeled_opt(self),
                141,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::SInt64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::s64_optional_opt(self),
                142,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::SInt64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::s64_repeated(self),
                143,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Fixed32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::fixed32_unlabeled_opt(self),
                151,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Fixed32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::fixed32_optional_opt(self),
                152,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Fixed32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::fixed32_repeated(self),
                153,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Fixed64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::fixed64_unlabeled_opt(self),
                161,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Fixed64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::fixed64_optional_opt(self),
                162,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Fixed64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::fixed64_repeated(self),
                163,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::SFixed32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::sfixed32_unlabeled_opt(self),
                171,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::SFixed32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::sfixed32_optional_opt(self),
                172,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::SFixed32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::sfixed32_repeated(self),
                173,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::SFixed64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::sfixed64_unlabeled_opt(self),
                181,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::SFixed64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::sfixed64_optional_opt(self),
                182,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::SFixed64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::sfixed64_repeated(self),
                183,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Double,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::f64_unlabeled_opt(self),
                191,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Double,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::f64_optional_opt(self),
                192,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Double,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::f64_repeated(self),
                193,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::default::Default for Msg {
        fn default() -> Self {
            Self::new()
        }
    }

    impl ::std::fmt::Debug for Msg
    where
        Self: super::_puroro_traits::MsgTrait,
    {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.debug_struct("Msg")
                .field("i32_unlabeled", &self.i32_unlabeled())
                .field("i32_optional", &self.i32_optional_opt())
                .field("i32_repeated", &self.i32_repeated())
                .field("float_unlabeled", &self.float_unlabeled())
                .field("float_optional", &self.float_optional_opt())
                .field("float_repeated", &self.float_repeated())
                .field("bytes_unlabeled", &self.bytes_unlabeled())
                .field("bytes_optional", &self.bytes_optional_opt())
                .field("bytes_repeated", &self.bytes_repeated())
                .field("string_unlabeled", &self.string_unlabeled())
                .field("string_optional", &self.string_optional_opt())
                .field("string_repeated", &self.string_repeated())
                .field("enum_unlabeled", &self.enum_unlabeled())
                .field("enum_optional", &self.enum_optional_opt())
                .field("enum_repeated", &self.enum_repeated())
                .field("submsg_unlabeled", &self.submsg_unlabeled())
                .field("submsg_optional", &self.submsg_optional())
                .field("submsg_repeated", &self.submsg_repeated())
                .field("i64_unlabeled", &self.i64_unlabeled())
                .field("i64_optional", &self.i64_optional_opt())
                .field("i64_repeated", &self.i64_repeated())
                .field("u32_unlabeled", &self.u32_unlabeled())
                .field("u32_optional", &self.u32_optional_opt())
                .field("u32_repeated", &self.u32_repeated())
                .field("u64_unlabeled", &self.u64_unlabeled())
                .field("u64_optional", &self.u64_optional_opt())
                .field("u64_repeated", &self.u64_repeated())
                .field("s32_unlabeled", &self.s32_unlabeled())
                .field("s32_optional", &self.s32_optional_opt())
                .field("s32_repeated", &self.s32_repeated())
                .field("s64_unlabeled", &self.s64_unlabeled())
                .field("s64_optional", &self.s64_optional_opt())
                .field("s64_repeated", &self.s64_repeated())
                .field("fixed32_unlabeled", &self.fixed32_unlabeled())
                .field("fixed32_optional", &self.fixed32_optional_opt())
                .field("fixed32_repeated", &self.fixed32_repeated())
                .field("fixed64_unlabeled", &self.fixed64_unlabeled())
                .field("fixed64_optional", &self.fixed64_optional_opt())
                .field("fixed64_repeated", &self.fixed64_repeated())
                .field("sfixed32_unlabeled", &self.sfixed32_unlabeled())
                .field("sfixed32_optional", &self.sfixed32_optional_opt())
                .field("sfixed32_repeated", &self.sfixed32_repeated())
                .field("sfixed64_unlabeled", &self.sfixed64_unlabeled())
                .field("sfixed64_optional", &self.sfixed64_optional_opt())
                .field("sfixed64_repeated", &self.sfixed64_repeated())
                .field("f64_unlabeled", &self.f64_unlabeled())
                .field("f64_optional", &self.f64_optional_opt())
                .field("f64_repeated", &self.f64_repeated())
                .finish()
        }
    }

    impl ::std::clone::Clone for Msg {
        fn clone(&self) -> Self {
            Self {
                _bitfield: ::std::clone::Clone::clone(&self._bitfield),
                i32_unlabeled: ::std::clone::Clone::clone(&self.i32_unlabeled),
                i32_optional: ::std::clone::Clone::clone(&self.i32_optional),
                i32_repeated: ::std::clone::Clone::clone(&self.i32_repeated),
                float_unlabeled: ::std::clone::Clone::clone(&self.float_unlabeled),
                float_optional: ::std::clone::Clone::clone(&self.float_optional),
                float_repeated: ::std::clone::Clone::clone(&self.float_repeated),
                bytes_unlabeled: ::std::clone::Clone::clone(&self.bytes_unlabeled),
                bytes_optional: ::std::clone::Clone::clone(&self.bytes_optional),
                bytes_repeated: ::std::clone::Clone::clone(&self.bytes_repeated),
                string_unlabeled: ::std::clone::Clone::clone(&self.string_unlabeled),
                string_optional: ::std::clone::Clone::clone(&self.string_optional),
                string_repeated: ::std::clone::Clone::clone(&self.string_repeated),
                enum_unlabeled: ::std::clone::Clone::clone(&self.enum_unlabeled),
                enum_optional: ::std::clone::Clone::clone(&self.enum_optional),
                enum_repeated: ::std::clone::Clone::clone(&self.enum_repeated),
                submsg_unlabeled: ::std::clone::Clone::clone(&self.submsg_unlabeled),
                submsg_optional: ::std::clone::Clone::clone(&self.submsg_optional),
                submsg_repeated: ::std::clone::Clone::clone(&self.submsg_repeated),
                i64_unlabeled: ::std::clone::Clone::clone(&self.i64_unlabeled),
                i64_optional: ::std::clone::Clone::clone(&self.i64_optional),
                i64_repeated: ::std::clone::Clone::clone(&self.i64_repeated),
                u32_unlabeled: ::std::clone::Clone::clone(&self.u32_unlabeled),
                u32_optional: ::std::clone::Clone::clone(&self.u32_optional),
                u32_repeated: ::std::clone::Clone::clone(&self.u32_repeated),
                u64_unlabeled: ::std::clone::Clone::clone(&self.u64_unlabeled),
                u64_optional: ::std::clone::Clone::clone(&self.u64_optional),
                u64_repeated: ::std::clone::Clone::clone(&self.u64_repeated),
                s32_unlabeled: ::std::clone::Clone::clone(&self.s32_unlabeled),
                s32_optional: ::std::clone::Clone::clone(&self.s32_optional),
                s32_repeated: ::std::clone::Clone::clone(&self.s32_repeated),
                s64_unlabeled: ::std::clone::Clone::clone(&self.s64_unlabeled),
                s64_optional: ::std::clone::Clone::clone(&self.s64_optional),
                s64_repeated: ::std::clone::Clone::clone(&self.s64_repeated),
                fixed32_unlabeled: ::std::clone::Clone::clone(&self.fixed32_unlabeled),
                fixed32_optional: ::std::clone::Clone::clone(&self.fixed32_optional),
                fixed32_repeated: ::std::clone::Clone::clone(&self.fixed32_repeated),
                fixed64_unlabeled: ::std::clone::Clone::clone(&self.fixed64_unlabeled),
                fixed64_optional: ::std::clone::Clone::clone(&self.fixed64_optional),
                fixed64_repeated: ::std::clone::Clone::clone(&self.fixed64_repeated),
                sfixed32_unlabeled: ::std::clone::Clone::clone(&self.sfixed32_unlabeled),
                sfixed32_optional: ::std::clone::Clone::clone(&self.sfixed32_optional),
                sfixed32_repeated: ::std::clone::Clone::clone(&self.sfixed32_repeated),
                sfixed64_unlabeled: ::std::clone::Clone::clone(&self.sfixed64_unlabeled),
                sfixed64_optional: ::std::clone::Clone::clone(&self.sfixed64_optional),
                sfixed64_repeated: ::std::clone::Clone::clone(&self.sfixed64_repeated),
                f64_unlabeled: ::std::clone::Clone::clone(&self.f64_unlabeled),
                f64_optional: ::std::clone::Clone::clone(&self.f64_optional),
                f64_repeated: ::std::clone::Clone::clone(&self.f64_repeated),
            }
        }
    }

    impl ::std::cmp::PartialEq for Msg {
        fn eq(&self, rhs: &Self) -> bool {
            self._bitfield == rhs._bitfield
                && self.i32_unlabeled == rhs.i32_unlabeled
                && (self._bitfield.get(0).as_deref() != Some(&true)
                    || self.i32_optional == rhs.i32_optional)
                && self.i32_repeated == rhs.i32_repeated
                && self.float_unlabeled == rhs.float_unlabeled
                && (self._bitfield.get(1).as_deref() != Some(&true)
                    || self.float_optional == rhs.float_optional)
                && self.float_repeated == rhs.float_repeated
                && self.bytes_unlabeled == rhs.bytes_unlabeled
                && (self._bitfield.get(2).as_deref() != Some(&true)
                    || self.bytes_optional == rhs.bytes_optional)
                && self.bytes_repeated == rhs.bytes_repeated
                && self.string_unlabeled == rhs.string_unlabeled
                && (self._bitfield.get(3).as_deref() != Some(&true)
                    || self.string_optional == rhs.string_optional)
                && self.string_repeated == rhs.string_repeated
                && self.enum_unlabeled == rhs.enum_unlabeled
                && (self._bitfield.get(4).as_deref() != Some(&true)
                    || self.enum_optional == rhs.enum_optional)
                && self.enum_repeated == rhs.enum_repeated
                && self.submsg_unlabeled == rhs.submsg_unlabeled
                && self.submsg_optional == rhs.submsg_optional
                && self.submsg_repeated == rhs.submsg_repeated
                && self.i64_unlabeled == rhs.i64_unlabeled
                && (self._bitfield.get(5).as_deref() != Some(&true)
                    || self.i64_optional == rhs.i64_optional)
                && self.i64_repeated == rhs.i64_repeated
                && self.u32_unlabeled == rhs.u32_unlabeled
                && (self._bitfield.get(6).as_deref() != Some(&true)
                    || self.u32_optional == rhs.u32_optional)
                && self.u32_repeated == rhs.u32_repeated
                && self.u64_unlabeled == rhs.u64_unlabeled
                && (self._bitfield.get(7).as_deref() != Some(&true)
                    || self.u64_optional == rhs.u64_optional)
                && self.u64_repeated == rhs.u64_repeated
                && self.s32_unlabeled == rhs.s32_unlabeled
                && (self._bitfield.get(8).as_deref() != Some(&true)
                    || self.s32_optional == rhs.s32_optional)
                && self.s32_repeated == rhs.s32_repeated
                && self.s64_unlabeled == rhs.s64_unlabeled
                && (self._bitfield.get(9).as_deref() != Some(&true)
                    || self.s64_optional == rhs.s64_optional)
                && self.s64_repeated == rhs.s64_repeated
                && self.fixed32_unlabeled == rhs.fixed32_unlabeled
                && (self._bitfield.get(10).as_deref() != Some(&true)
                    || self.fixed32_optional == rhs.fixed32_optional)
                && self.fixed32_repeated == rhs.fixed32_repeated
                && self.fixed64_unlabeled == rhs.fixed64_unlabeled
                && (self._bitfield.get(11).as_deref() != Some(&true)
                    || self.fixed64_optional == rhs.fixed64_optional)
                && self.fixed64_repeated == rhs.fixed64_repeated
                && self.sfixed32_unlabeled == rhs.sfixed32_unlabeled
                && (self._bitfield.get(12).as_deref() != Some(&true)
                    || self.sfixed32_optional == rhs.sfixed32_optional)
                && self.sfixed32_repeated == rhs.sfixed32_repeated
                && self.sfixed64_unlabeled == rhs.sfixed64_unlabeled
                && (self._bitfield.get(13).as_deref() != Some(&true)
                    || self.sfixed64_optional == rhs.sfixed64_optional)
                && self.sfixed64_repeated == rhs.sfixed64_repeated
                && self.f64_unlabeled == rhs.f64_unlabeled
                && (self._bitfield.get(14).as_deref() != Some(&true)
                    || self.f64_optional == rhs.f64_optional)
                && self.f64_repeated == rhs.f64_repeated
                && true
        }
    }
}

pub use _puroro_impls::*;
pub mod _puroro_impls {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
    use super::_puroro_traits::*;

    pub struct MsgSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
    {
        pub i32_unlabeled: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField1<ScalarType> where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
    {
        fn i32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.i32_unlabeled,
            )))
        }
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            Self::SubmsgRepeatedMessageType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::i32_unlabeled_opt(self),
                1,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                i32_unlabeled: value,
            }
        }
    }

    impl<ScalarType> ::std::clone::Clone for MsgSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        ScalarType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                i32_unlabeled: ::std::clone::Clone::clone(&self.i32_unlabeled),
            }
        }
    }

    pub struct MsgSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
    {
        pub i32_optional: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField2<ScalarType> where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
    {
        fn i32_optional_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.i32_optional,
            )))
        }
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            Self::SubmsgRepeatedMessageType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::i32_optional_opt(self),
                2,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                i32_optional: value,
            }
        }
    }

    impl<ScalarType> ::std::clone::Clone for MsgSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        ScalarType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                i32_optional: ::std::clone::Clone::clone(&self.i32_optional),
            }
        }
    }

    pub struct MsgSingleField3<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub i32_repeated: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::Msg>
        for MsgSingleField3<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::MsgTrait
        for MsgSingleField3<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<'this, RepeatedType, ScalarType, i32>;

        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.i32_repeated)
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            Self::SubmsgRepeatedMessageType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::se::SerMessageToIoWrite
        for MsgSingleField3<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::i32_repeated(self),
                3,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for MsgSingleField3<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                i32_repeated: value,
            }
        }
    }

    impl<ScalarType, RepeatedType> ::std::clone::Clone for MsgSingleField3<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        RepeatedType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                i32_repeated: ::std::clone::Clone::clone(&self.i32_repeated),
            }
        }
    }

    pub struct MsgSingleField11<ScalarType>
    where
        ScalarType: ::std::convert::Into<f32> + ::std::clone::Clone,
    {
        pub float_unlabeled: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField11<ScalarType> where
        ScalarType: ::std::convert::Into<f32> + ::std::clone::Clone
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField11<ScalarType>
    where
        ScalarType: ::std::convert::Into<f32> + ::std::clone::Clone,
    {
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn float_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<f32> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.float_unlabeled,
            )))
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            Self::SubmsgRepeatedMessageType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField11<ScalarType>
    where
        ScalarType: ::std::convert::Into<f32> + ::std::clone::Clone,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Float,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::float_unlabeled_opt(self),
                11,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField11<ScalarType>
    where
        ScalarType: ::std::convert::Into<f32> + ::std::clone::Clone,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                float_unlabeled: value,
            }
        }
    }

    impl<ScalarType> ::std::clone::Clone for MsgSingleField11<ScalarType>
    where
        ScalarType: ::std::convert::Into<f32> + ::std::clone::Clone,
        ScalarType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                float_unlabeled: ::std::clone::Clone::clone(&self.float_unlabeled),
            }
        }
    }

    pub struct MsgSingleField12<ScalarType>
    where
        ScalarType: ::std::convert::Into<f32> + ::std::clone::Clone,
    {
        pub float_optional: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField12<ScalarType> where
        ScalarType: ::std::convert::Into<f32> + ::std::clone::Clone
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField12<ScalarType>
    where
        ScalarType: ::std::convert::Into<f32> + ::std::clone::Clone,
    {
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn float_optional_opt<'this>(&'this self) -> ::std::option::Option<f32> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.float_optional,
            )))
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            Self::SubmsgRepeatedMessageType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField12<ScalarType>
    where
        ScalarType: ::std::convert::Into<f32> + ::std::clone::Clone,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Float,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::float_optional_opt(self),
                12,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField12<ScalarType>
    where
        ScalarType: ::std::convert::Into<f32> + ::std::clone::Clone,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                float_optional: value,
            }
        }
    }

    impl<ScalarType> ::std::clone::Clone for MsgSingleField12<ScalarType>
    where
        ScalarType: ::std::convert::Into<f32> + ::std::clone::Clone,
        ScalarType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                float_optional: ::std::clone::Clone::clone(&self.float_optional),
            }
        }
    }

    pub struct MsgSingleField13<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<f32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub float_repeated: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::Msg>
        for MsgSingleField13<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<f32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::MsgTrait
        for MsgSingleField13<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<f32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<'this, RepeatedType, ScalarType, f32>;

        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.float_repeated)
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            Self::SubmsgRepeatedMessageType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::se::SerMessageToIoWrite
        for MsgSingleField13<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<f32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Float,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::float_repeated(self),
                13,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for MsgSingleField13<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<f32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                float_repeated: value,
            }
        }
    }

    impl<ScalarType, RepeatedType> ::std::clone::Clone for MsgSingleField13<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<f32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        RepeatedType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                float_repeated: ::std::clone::Clone::clone(&self.float_repeated),
            }
        }
    }

    pub struct MsgSingleField21<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<[u8]>,
    {
        pub bytes_unlabeled: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField21<ScalarType> where
        ScalarType: ::std::convert::AsRef<[u8]>
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField21<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<[u8]>,
    {
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn bytes_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<&'this [u8]> {
            ::std::option::Option::Some(self.bytes_unlabeled.as_ref())
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            Self::SubmsgRepeatedMessageType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField21<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<[u8]>,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Bytes,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::bytes_unlabeled_opt(self),
                21,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField21<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<[u8]>,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                bytes_unlabeled: value,
            }
        }
    }

    impl<ScalarType> ::std::clone::Clone for MsgSingleField21<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<[u8]>,
        ScalarType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                bytes_unlabeled: ::std::clone::Clone::clone(&self.bytes_unlabeled),
            }
        }
    }

    pub struct MsgSingleField22<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<[u8]>,
    {
        pub bytes_optional: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField22<ScalarType> where
        ScalarType: ::std::convert::AsRef<[u8]>
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField22<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<[u8]>,
    {
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn bytes_optional_opt<'this>(&'this self) -> ::std::option::Option<&'this [u8]> {
            ::std::option::Option::Some(self.bytes_optional.as_ref())
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            Self::SubmsgRepeatedMessageType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField22<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<[u8]>,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bytes,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::bytes_optional_opt(self),
                22,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField22<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<[u8]>,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                bytes_optional: value,
            }
        }
    }

    impl<ScalarType> ::std::clone::Clone for MsgSingleField22<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<[u8]>,
        ScalarType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                bytes_optional: ::std::clone::Clone::clone(&self.bytes_optional),
            }
        }
    }

    pub struct MsgSingleField23<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::AsRef<[u8]>,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub bytes_repeated: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::Msg>
        for MsgSingleField23<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::AsRef<[u8]>,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::MsgTrait
        for MsgSingleField23<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::AsRef<[u8]>,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::AsRefRepeatedField<'this, RepeatedType, ScalarType, [u8]>;

        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::AsRefRepeatedField::new(&self.bytes_repeated)
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            Self::SubmsgRepeatedMessageType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::se::SerMessageToIoWrite
        for MsgSingleField23<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::AsRef<[u8]>,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Bytes,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::bytes_repeated(self),
                23,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for MsgSingleField23<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::AsRef<[u8]>,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                bytes_repeated: value,
            }
        }
    }

    impl<ScalarType, RepeatedType> ::std::clone::Clone for MsgSingleField23<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::AsRef<[u8]>,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        RepeatedType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                bytes_repeated: ::std::clone::Clone::clone(&self.bytes_repeated),
            }
        }
    }

    pub struct MsgSingleField31<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>,
    {
        pub string_unlabeled: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField31<ScalarType> where
        ScalarType: ::std::convert::AsRef<str>
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField31<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>,
    {
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn string_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::Some(self.string_unlabeled.as_ref())
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            Self::SubmsgRepeatedMessageType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField31<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::string_unlabeled_opt(self),
                31,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField31<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                string_unlabeled: value,
            }
        }
    }

    impl<ScalarType> ::std::clone::Clone for MsgSingleField31<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>,
        ScalarType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                string_unlabeled: ::std::clone::Clone::clone(&self.string_unlabeled),
            }
        }
    }

    pub struct MsgSingleField32<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>,
    {
        pub string_optional: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField32<ScalarType> where
        ScalarType: ::std::convert::AsRef<str>
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField32<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>,
    {
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn string_optional_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::Some(self.string_optional.as_ref())
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            Self::SubmsgRepeatedMessageType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField32<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::string_optional_opt(self),
                32,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField32<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                string_optional: value,
            }
        }
    }

    impl<ScalarType> ::std::clone::Clone for MsgSingleField32<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>,
        ScalarType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                string_optional: ::std::clone::Clone::clone(&self.string_optional),
            }
        }
    }

    pub struct MsgSingleField33<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::AsRef<str>,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub string_repeated: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::Msg>
        for MsgSingleField33<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::AsRef<str>,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::MsgTrait
        for MsgSingleField33<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::AsRef<str>,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::AsRefRepeatedField<'this, RepeatedType, ScalarType, str>;

        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::AsRefRepeatedField::new(&self.string_repeated)
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            Self::SubmsgRepeatedMessageType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::se::SerMessageToIoWrite
        for MsgSingleField33<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::AsRef<str>,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::string_repeated(self),
                33,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for MsgSingleField33<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::AsRef<str>,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                string_repeated: value,
            }
        }
    }

    impl<ScalarType, RepeatedType> ::std::clone::Clone for MsgSingleField33<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::AsRef<str>,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        RepeatedType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                string_repeated: ::std::clone::Clone::clone(&self.string_repeated),
            }
        }
    }

    pub struct MsgSingleField41<ScalarType>
    where
        ScalarType:
            ::std::convert::Into<self::_puroro_root::full_coverage3::Enum> + ::std::clone::Clone,
    {
        pub enum_unlabeled: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField41<ScalarType> where
        ScalarType:
            ::std::convert::Into<self::_puroro_root::full_coverage3::Enum> + ::std::clone::Clone
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField41<ScalarType>
    where
        ScalarType:
            ::std::convert::Into<self::_puroro_root::full_coverage3::Enum> + ::std::clone::Clone,
    {
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn enum_unlabeled_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::full_coverage3::Enum> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.enum_unlabeled,
            )))
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            Self::SubmsgRepeatedMessageType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField41<ScalarType>
    where
        ScalarType:
            ::std::convert::Into<self::_puroro_root::full_coverage3::Enum> + ::std::clone::Clone,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Enum3<self::_puroro_root::full_coverage3::Enum>,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::enum_unlabeled_opt(self),
                41,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField41<ScalarType>
    where
        ScalarType:
            ::std::convert::Into<self::_puroro_root::full_coverage3::Enum> + ::std::clone::Clone,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                enum_unlabeled: value,
            }
        }
    }

    impl<ScalarType> ::std::clone::Clone for MsgSingleField41<ScalarType>
    where
        ScalarType:
            ::std::convert::Into<self::_puroro_root::full_coverage3::Enum> + ::std::clone::Clone,
        ScalarType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                enum_unlabeled: ::std::clone::Clone::clone(&self.enum_unlabeled),
            }
        }
    }

    pub struct MsgSingleField42<ScalarType>
    where
        ScalarType:
            ::std::convert::Into<self::_puroro_root::full_coverage3::Enum> + ::std::clone::Clone,
    {
        pub enum_optional: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField42<ScalarType> where
        ScalarType:
            ::std::convert::Into<self::_puroro_root::full_coverage3::Enum> + ::std::clone::Clone
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField42<ScalarType>
    where
        ScalarType:
            ::std::convert::Into<self::_puroro_root::full_coverage3::Enum> + ::std::clone::Clone,
    {
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn enum_optional_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::full_coverage3::Enum> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.enum_optional,
            )))
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            Self::SubmsgRepeatedMessageType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField42<ScalarType>
    where
        ScalarType:
            ::std::convert::Into<self::_puroro_root::full_coverage3::Enum> + ::std::clone::Clone,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Enum3<self::_puroro_root::full_coverage3::Enum>,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::enum_optional_opt(self),
                42,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField42<ScalarType>
    where
        ScalarType:
            ::std::convert::Into<self::_puroro_root::full_coverage3::Enum> + ::std::clone::Clone,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                enum_optional: value,
            }
        }
    }

    impl<ScalarType> ::std::clone::Clone for MsgSingleField42<ScalarType>
    where
        ScalarType:
            ::std::convert::Into<self::_puroro_root::full_coverage3::Enum> + ::std::clone::Clone,
        ScalarType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                enum_optional: ::std::clone::Clone::clone(&self.enum_optional),
            }
        }
    }

    pub struct MsgSingleField43<ScalarType, RepeatedType>
    where
        ScalarType:
            ::std::convert::Into<self::_puroro_root::full_coverage3::Enum> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub enum_repeated: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::Msg>
        for MsgSingleField43<ScalarType, RepeatedType>
    where
        ScalarType:
            ::std::convert::Into<self::_puroro_root::full_coverage3::Enum> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::MsgTrait
        for MsgSingleField43<ScalarType, RepeatedType>
    where
        ScalarType:
            ::std::convert::Into<self::_puroro_root::full_coverage3::Enum> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<
            'this,
            RepeatedType,
            ScalarType,
            self::_puroro_root::full_coverage3::Enum,
        >;

        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.enum_repeated)
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            Self::SubmsgRepeatedMessageType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::se::SerMessageToIoWrite
        for MsgSingleField43<ScalarType, RepeatedType>
    where
        ScalarType:
            ::std::convert::Into<self::_puroro_root::full_coverage3::Enum> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Enum3<self::_puroro_root::full_coverage3::Enum>,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::enum_repeated(self),
                43,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for MsgSingleField43<ScalarType, RepeatedType>
    where
        ScalarType:
            ::std::convert::Into<self::_puroro_root::full_coverage3::Enum> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                enum_repeated: value,
            }
        }
    }

    impl<ScalarType, RepeatedType> ::std::clone::Clone for MsgSingleField43<ScalarType, RepeatedType>
    where
        ScalarType:
            ::std::convert::Into<self::_puroro_root::full_coverage3::Enum> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        RepeatedType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                enum_repeated: ::std::clone::Clone::clone(&self.enum_repeated),
            }
        }
    }

    pub struct MsgSingleField51<ScalarType>
    where
        ScalarType:
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait,
    {
        pub submsg_unlabeled: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField51<ScalarType> where
        ScalarType:
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField51<ScalarType>
    where
        ScalarType:
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait,
    {
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = &'this ScalarType;

        fn submsg_unlabeled_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::SubmsgUnlabeledMessageType<'this>> {
            ::std::option::Option::Some(&self.submsg_unlabeled)
        }
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            Self::SubmsgRepeatedMessageType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField51<ScalarType>
    where
        ScalarType:
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait,
        Self: super::_puroro_traits::MsgTrait,
        for<'a> <Self as super::_puroro_traits::MsgTrait>::SubmsgUnlabeledMessageType<'a>:
            ::puroro::internal::se::SerMessageToIoWrite,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Message<
                    <Self as super::_puroro_traits::MsgTrait>::SubmsgUnlabeledMessageType<'_>,
                >,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::submsg_unlabeled_opt(self),
                51,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField51<ScalarType>
    where
        ScalarType:
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                submsg_unlabeled: value,
            }
        }
    }

    impl<ScalarType> ::std::clone::Clone for MsgSingleField51<ScalarType>
    where
        ScalarType:
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait,
        ScalarType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                submsg_unlabeled: ::std::clone::Clone::clone(&self.submsg_unlabeled),
            }
        }
    }

    pub struct MsgSingleField52<ScalarType>
    where
        ScalarType:
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait,
    {
        pub submsg_optional: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField52<ScalarType> where
        ScalarType:
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField52<ScalarType>
    where
        ScalarType:
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait,
    {
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = &'this ScalarType;

        fn submsg_optional_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::SubmsgOptionalMessageType<'this>> {
            ::std::option::Option::Some(&self.submsg_optional)
        }
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            Self::SubmsgRepeatedMessageType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField52<ScalarType>
    where
        ScalarType:
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait,
        Self: super::_puroro_traits::MsgTrait,
        for<'a> <Self as super::_puroro_traits::MsgTrait>::SubmsgOptionalMessageType<'a>:
            ::puroro::internal::se::SerMessageToIoWrite,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    <Self as super::_puroro_traits::MsgTrait>::SubmsgOptionalMessageType<'_>,
                >,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::submsg_optional_opt(self),
                52,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField52<ScalarType>
    where
        ScalarType:
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                submsg_optional: value,
            }
        }
    }

    impl<ScalarType> ::std::clone::Clone for MsgSingleField52<ScalarType>
    where
        ScalarType:
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait,
        ScalarType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                submsg_optional: ::std::clone::Clone::clone(&self.submsg_optional),
            }
        }
    }

    pub struct MsgSingleField53<ScalarType, RepeatedType>
    where
        ScalarType:
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub submsg_repeated: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::Msg>
        for MsgSingleField53<ScalarType, RepeatedType>
    where
        ScalarType:
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::MsgTrait
        for MsgSingleField53<ScalarType, RepeatedType>
    where
        ScalarType:
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = &'this ScalarType;

        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = &'this RepeatedType;

        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            &self.submsg_repeated
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::se::SerMessageToIoWrite
        for MsgSingleField53<ScalarType, RepeatedType>
    where
        ScalarType:
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        Self: super::_puroro_traits::MsgTrait,
        for<'a> <Self as super::_puroro_traits::MsgTrait>::SubmsgRepeatedMessageType<'a>:
            ::puroro::internal::se::SerMessageToIoWrite,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    <Self as super::_puroro_traits::MsgTrait>::SubmsgRepeatedMessageType<'_>,
                >,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::submsg_repeated(self),
                53,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for MsgSingleField53<ScalarType, RepeatedType>
    where
        ScalarType:
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                submsg_repeated: value,
            }
        }
    }

    impl<ScalarType, RepeatedType> ::std::clone::Clone for MsgSingleField53<ScalarType, RepeatedType>
    where
        ScalarType:
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        RepeatedType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                submsg_repeated: ::std::clone::Clone::clone(&self.submsg_repeated),
            }
        }
    }

    pub struct MsgSingleField101<ScalarType>
    where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
    {
        pub i64_unlabeled: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField101<ScalarType> where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField101<ScalarType>
    where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
    {
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            Self::SubmsgRepeatedMessageType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn i64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.i64_unlabeled,
            )))
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField101<ScalarType>
    where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Int64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::i64_unlabeled_opt(self),
                101,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField101<ScalarType>
    where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                i64_unlabeled: value,
            }
        }
    }

    impl<ScalarType> ::std::clone::Clone for MsgSingleField101<ScalarType>
    where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
        ScalarType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                i64_unlabeled: ::std::clone::Clone::clone(&self.i64_unlabeled),
            }
        }
    }

    pub struct MsgSingleField102<ScalarType>
    where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
    {
        pub i64_optional: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField102<ScalarType> where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField102<ScalarType>
    where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
    {
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            Self::SubmsgRepeatedMessageType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn i64_optional_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.i64_optional,
            )))
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField102<ScalarType>
    where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::i64_optional_opt(self),
                102,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField102<ScalarType>
    where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                i64_optional: value,
            }
        }
    }

    impl<ScalarType> ::std::clone::Clone for MsgSingleField102<ScalarType>
    where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
        ScalarType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                i64_optional: ::std::clone::Clone::clone(&self.i64_optional),
            }
        }
    }

    pub struct MsgSingleField103<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub i64_repeated: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::Msg>
        for MsgSingleField103<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::MsgTrait
        for MsgSingleField103<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            Self::SubmsgRepeatedMessageType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<'this, RepeatedType, ScalarType, i64>;

        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.i64_repeated)
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::se::SerMessageToIoWrite
        for MsgSingleField103<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Int64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::i64_repeated(self),
                103,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for MsgSingleField103<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                i64_repeated: value,
            }
        }
    }

    impl<ScalarType, RepeatedType> ::std::clone::Clone for MsgSingleField103<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        RepeatedType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                i64_repeated: ::std::clone::Clone::clone(&self.i64_repeated),
            }
        }
    }

    pub struct MsgSingleField111<ScalarType>
    where
        ScalarType: ::std::convert::Into<u32> + ::std::clone::Clone,
    {
        pub u32_unlabeled: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField111<ScalarType> where
        ScalarType: ::std::convert::Into<u32> + ::std::clone::Clone
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField111<ScalarType>
    where
        ScalarType: ::std::convert::Into<u32> + ::std::clone::Clone,
    {
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            Self::SubmsgRepeatedMessageType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn u32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<u32> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.u32_unlabeled,
            )))
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField111<ScalarType>
    where
        ScalarType: ::std::convert::Into<u32> + ::std::clone::Clone,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::UInt32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::u32_unlabeled_opt(self),
                111,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField111<ScalarType>
    where
        ScalarType: ::std::convert::Into<u32> + ::std::clone::Clone,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                u32_unlabeled: value,
            }
        }
    }

    impl<ScalarType> ::std::clone::Clone for MsgSingleField111<ScalarType>
    where
        ScalarType: ::std::convert::Into<u32> + ::std::clone::Clone,
        ScalarType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                u32_unlabeled: ::std::clone::Clone::clone(&self.u32_unlabeled),
            }
        }
    }

    pub struct MsgSingleField112<ScalarType>
    where
        ScalarType: ::std::convert::Into<u32> + ::std::clone::Clone,
    {
        pub u32_optional: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField112<ScalarType> where
        ScalarType: ::std::convert::Into<u32> + ::std::clone::Clone
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField112<ScalarType>
    where
        ScalarType: ::std::convert::Into<u32> + ::std::clone::Clone,
    {
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            Self::SubmsgRepeatedMessageType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn u32_optional_opt<'this>(&'this self) -> ::std::option::Option<u32> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.u32_optional,
            )))
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField112<ScalarType>
    where
        ScalarType: ::std::convert::Into<u32> + ::std::clone::Clone,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::UInt32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::u32_optional_opt(self),
                112,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField112<ScalarType>
    where
        ScalarType: ::std::convert::Into<u32> + ::std::clone::Clone,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                u32_optional: value,
            }
        }
    }

    impl<ScalarType> ::std::clone::Clone for MsgSingleField112<ScalarType>
    where
        ScalarType: ::std::convert::Into<u32> + ::std::clone::Clone,
        ScalarType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                u32_optional: ::std::clone::Clone::clone(&self.u32_optional),
            }
        }
    }

    pub struct MsgSingleField113<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<u32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub u32_repeated: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::Msg>
        for MsgSingleField113<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<u32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::MsgTrait
        for MsgSingleField113<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<u32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            Self::SubmsgRepeatedMessageType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<'this, RepeatedType, ScalarType, u32>;

        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.u32_repeated)
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::se::SerMessageToIoWrite
        for MsgSingleField113<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<u32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::UInt32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::u32_repeated(self),
                113,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for MsgSingleField113<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<u32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                u32_repeated: value,
            }
        }
    }

    impl<ScalarType, RepeatedType> ::std::clone::Clone for MsgSingleField113<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<u32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        RepeatedType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                u32_repeated: ::std::clone::Clone::clone(&self.u32_repeated),
            }
        }
    }

    pub struct MsgSingleField121<ScalarType>
    where
        ScalarType: ::std::convert::Into<u64> + ::std::clone::Clone,
    {
        pub u64_unlabeled: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField121<ScalarType> where
        ScalarType: ::std::convert::Into<u64> + ::std::clone::Clone
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField121<ScalarType>
    where
        ScalarType: ::std::convert::Into<u64> + ::std::clone::Clone,
    {
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            Self::SubmsgRepeatedMessageType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn u64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<u64> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.u64_unlabeled,
            )))
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField121<ScalarType>
    where
        ScalarType: ::std::convert::Into<u64> + ::std::clone::Clone,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::UInt64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::u64_unlabeled_opt(self),
                121,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField121<ScalarType>
    where
        ScalarType: ::std::convert::Into<u64> + ::std::clone::Clone,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                u64_unlabeled: value,
            }
        }
    }

    impl<ScalarType> ::std::clone::Clone for MsgSingleField121<ScalarType>
    where
        ScalarType: ::std::convert::Into<u64> + ::std::clone::Clone,
        ScalarType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                u64_unlabeled: ::std::clone::Clone::clone(&self.u64_unlabeled),
            }
        }
    }

    pub struct MsgSingleField122<ScalarType>
    where
        ScalarType: ::std::convert::Into<u64> + ::std::clone::Clone,
    {
        pub u64_optional: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField122<ScalarType> where
        ScalarType: ::std::convert::Into<u64> + ::std::clone::Clone
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField122<ScalarType>
    where
        ScalarType: ::std::convert::Into<u64> + ::std::clone::Clone,
    {
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            Self::SubmsgRepeatedMessageType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn u64_optional_opt<'this>(&'this self) -> ::std::option::Option<u64> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.u64_optional,
            )))
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField122<ScalarType>
    where
        ScalarType: ::std::convert::Into<u64> + ::std::clone::Clone,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::UInt64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::u64_optional_opt(self),
                122,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField122<ScalarType>
    where
        ScalarType: ::std::convert::Into<u64> + ::std::clone::Clone,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                u64_optional: value,
            }
        }
    }

    impl<ScalarType> ::std::clone::Clone for MsgSingleField122<ScalarType>
    where
        ScalarType: ::std::convert::Into<u64> + ::std::clone::Clone,
        ScalarType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                u64_optional: ::std::clone::Clone::clone(&self.u64_optional),
            }
        }
    }

    pub struct MsgSingleField123<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<u64> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub u64_repeated: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::Msg>
        for MsgSingleField123<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<u64> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::MsgTrait
        for MsgSingleField123<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<u64> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            Self::SubmsgRepeatedMessageType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<'this, RepeatedType, ScalarType, u64>;

        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.u64_repeated)
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::se::SerMessageToIoWrite
        for MsgSingleField123<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<u64> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::UInt64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::u64_repeated(self),
                123,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for MsgSingleField123<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<u64> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                u64_repeated: value,
            }
        }
    }

    impl<ScalarType, RepeatedType> ::std::clone::Clone for MsgSingleField123<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<u64> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        RepeatedType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                u64_repeated: ::std::clone::Clone::clone(&self.u64_repeated),
            }
        }
    }

    pub struct MsgSingleField131<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
    {
        pub s32_unlabeled: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField131<ScalarType> where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField131<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
    {
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            Self::SubmsgRepeatedMessageType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn s32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.s32_unlabeled,
            )))
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField131<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::SInt32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::s32_unlabeled_opt(self),
                131,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField131<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                s32_unlabeled: value,
            }
        }
    }

    impl<ScalarType> ::std::clone::Clone for MsgSingleField131<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        ScalarType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                s32_unlabeled: ::std::clone::Clone::clone(&self.s32_unlabeled),
            }
        }
    }

    pub struct MsgSingleField132<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
    {
        pub s32_optional: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField132<ScalarType> where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField132<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
    {
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            Self::SubmsgRepeatedMessageType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn s32_optional_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.s32_optional,
            )))
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField132<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::SInt32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::s32_optional_opt(self),
                132,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField132<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                s32_optional: value,
            }
        }
    }

    impl<ScalarType> ::std::clone::Clone for MsgSingleField132<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        ScalarType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                s32_optional: ::std::clone::Clone::clone(&self.s32_optional),
            }
        }
    }

    pub struct MsgSingleField133<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub s32_repeated: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::Msg>
        for MsgSingleField133<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::MsgTrait
        for MsgSingleField133<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            Self::SubmsgRepeatedMessageType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<'this, RepeatedType, ScalarType, i32>;

        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.s32_repeated)
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::se::SerMessageToIoWrite
        for MsgSingleField133<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::SInt32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::s32_repeated(self),
                133,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for MsgSingleField133<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                s32_repeated: value,
            }
        }
    }

    impl<ScalarType, RepeatedType> ::std::clone::Clone for MsgSingleField133<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        RepeatedType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                s32_repeated: ::std::clone::Clone::clone(&self.s32_repeated),
            }
        }
    }

    pub struct MsgSingleField141<ScalarType>
    where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
    {
        pub s64_unlabeled: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField141<ScalarType> where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField141<ScalarType>
    where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
    {
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            Self::SubmsgRepeatedMessageType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn s64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.s64_unlabeled,
            )))
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField141<ScalarType>
    where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::SInt64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::s64_unlabeled_opt(self),
                141,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField141<ScalarType>
    where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                s64_unlabeled: value,
            }
        }
    }

    impl<ScalarType> ::std::clone::Clone for MsgSingleField141<ScalarType>
    where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
        ScalarType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                s64_unlabeled: ::std::clone::Clone::clone(&self.s64_unlabeled),
            }
        }
    }

    pub struct MsgSingleField142<ScalarType>
    where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
    {
        pub s64_optional: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField142<ScalarType> where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField142<ScalarType>
    where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
    {
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            Self::SubmsgRepeatedMessageType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn s64_optional_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.s64_optional,
            )))
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField142<ScalarType>
    where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::SInt64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::s64_optional_opt(self),
                142,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField142<ScalarType>
    where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                s64_optional: value,
            }
        }
    }

    impl<ScalarType> ::std::clone::Clone for MsgSingleField142<ScalarType>
    where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
        ScalarType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                s64_optional: ::std::clone::Clone::clone(&self.s64_optional),
            }
        }
    }

    pub struct MsgSingleField143<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub s64_repeated: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::Msg>
        for MsgSingleField143<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::MsgTrait
        for MsgSingleField143<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            Self::SubmsgRepeatedMessageType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<'this, RepeatedType, ScalarType, i64>;

        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.s64_repeated)
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::se::SerMessageToIoWrite
        for MsgSingleField143<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::SInt64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::s64_repeated(self),
                143,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for MsgSingleField143<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                s64_repeated: value,
            }
        }
    }

    impl<ScalarType, RepeatedType> ::std::clone::Clone for MsgSingleField143<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        RepeatedType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                s64_repeated: ::std::clone::Clone::clone(&self.s64_repeated),
            }
        }
    }

    pub struct MsgSingleField151<ScalarType>
    where
        ScalarType: ::std::convert::Into<u32> + ::std::clone::Clone,
    {
        pub fixed32_unlabeled: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField151<ScalarType> where
        ScalarType: ::std::convert::Into<u32> + ::std::clone::Clone
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField151<ScalarType>
    where
        ScalarType: ::std::convert::Into<u32> + ::std::clone::Clone,
    {
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            Self::SubmsgRepeatedMessageType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn fixed32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<u32> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.fixed32_unlabeled,
            )))
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField151<ScalarType>
    where
        ScalarType: ::std::convert::Into<u32> + ::std::clone::Clone,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Fixed32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::fixed32_unlabeled_opt(self),
                151,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField151<ScalarType>
    where
        ScalarType: ::std::convert::Into<u32> + ::std::clone::Clone,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                fixed32_unlabeled: value,
            }
        }
    }

    impl<ScalarType> ::std::clone::Clone for MsgSingleField151<ScalarType>
    where
        ScalarType: ::std::convert::Into<u32> + ::std::clone::Clone,
        ScalarType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                fixed32_unlabeled: ::std::clone::Clone::clone(&self.fixed32_unlabeled),
            }
        }
    }

    pub struct MsgSingleField152<ScalarType>
    where
        ScalarType: ::std::convert::Into<u32> + ::std::clone::Clone,
    {
        pub fixed32_optional: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField152<ScalarType> where
        ScalarType: ::std::convert::Into<u32> + ::std::clone::Clone
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField152<ScalarType>
    where
        ScalarType: ::std::convert::Into<u32> + ::std::clone::Clone,
    {
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            Self::SubmsgRepeatedMessageType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn fixed32_optional_opt<'this>(&'this self) -> ::std::option::Option<u32> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.fixed32_optional,
            )))
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField152<ScalarType>
    where
        ScalarType: ::std::convert::Into<u32> + ::std::clone::Clone,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Fixed32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::fixed32_optional_opt(self),
                152,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField152<ScalarType>
    where
        ScalarType: ::std::convert::Into<u32> + ::std::clone::Clone,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                fixed32_optional: value,
            }
        }
    }

    impl<ScalarType> ::std::clone::Clone for MsgSingleField152<ScalarType>
    where
        ScalarType: ::std::convert::Into<u32> + ::std::clone::Clone,
        ScalarType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                fixed32_optional: ::std::clone::Clone::clone(&self.fixed32_optional),
            }
        }
    }

    pub struct MsgSingleField153<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<u32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub fixed32_repeated: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::Msg>
        for MsgSingleField153<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<u32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::MsgTrait
        for MsgSingleField153<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<u32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            Self::SubmsgRepeatedMessageType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<'this, RepeatedType, ScalarType, u32>;

        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.fixed32_repeated)
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::se::SerMessageToIoWrite
        for MsgSingleField153<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<u32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Fixed32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::fixed32_repeated(self),
                153,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for MsgSingleField153<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<u32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                fixed32_repeated: value,
            }
        }
    }

    impl<ScalarType, RepeatedType> ::std::clone::Clone for MsgSingleField153<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<u32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        RepeatedType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                fixed32_repeated: ::std::clone::Clone::clone(&self.fixed32_repeated),
            }
        }
    }

    pub struct MsgSingleField161<ScalarType>
    where
        ScalarType: ::std::convert::Into<u64> + ::std::clone::Clone,
    {
        pub fixed64_unlabeled: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField161<ScalarType> where
        ScalarType: ::std::convert::Into<u64> + ::std::clone::Clone
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField161<ScalarType>
    where
        ScalarType: ::std::convert::Into<u64> + ::std::clone::Clone,
    {
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            Self::SubmsgRepeatedMessageType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn fixed64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<u64> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.fixed64_unlabeled,
            )))
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField161<ScalarType>
    where
        ScalarType: ::std::convert::Into<u64> + ::std::clone::Clone,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Fixed64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::fixed64_unlabeled_opt(self),
                161,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField161<ScalarType>
    where
        ScalarType: ::std::convert::Into<u64> + ::std::clone::Clone,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                fixed64_unlabeled: value,
            }
        }
    }

    impl<ScalarType> ::std::clone::Clone for MsgSingleField161<ScalarType>
    where
        ScalarType: ::std::convert::Into<u64> + ::std::clone::Clone,
        ScalarType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                fixed64_unlabeled: ::std::clone::Clone::clone(&self.fixed64_unlabeled),
            }
        }
    }

    pub struct MsgSingleField162<ScalarType>
    where
        ScalarType: ::std::convert::Into<u64> + ::std::clone::Clone,
    {
        pub fixed64_optional: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField162<ScalarType> where
        ScalarType: ::std::convert::Into<u64> + ::std::clone::Clone
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField162<ScalarType>
    where
        ScalarType: ::std::convert::Into<u64> + ::std::clone::Clone,
    {
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            Self::SubmsgRepeatedMessageType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn fixed64_optional_opt<'this>(&'this self) -> ::std::option::Option<u64> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.fixed64_optional,
            )))
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField162<ScalarType>
    where
        ScalarType: ::std::convert::Into<u64> + ::std::clone::Clone,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Fixed64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::fixed64_optional_opt(self),
                162,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField162<ScalarType>
    where
        ScalarType: ::std::convert::Into<u64> + ::std::clone::Clone,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                fixed64_optional: value,
            }
        }
    }

    impl<ScalarType> ::std::clone::Clone for MsgSingleField162<ScalarType>
    where
        ScalarType: ::std::convert::Into<u64> + ::std::clone::Clone,
        ScalarType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                fixed64_optional: ::std::clone::Clone::clone(&self.fixed64_optional),
            }
        }
    }

    pub struct MsgSingleField163<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<u64> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub fixed64_repeated: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::Msg>
        for MsgSingleField163<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<u64> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::MsgTrait
        for MsgSingleField163<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<u64> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            Self::SubmsgRepeatedMessageType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<'this, RepeatedType, ScalarType, u64>;

        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.fixed64_repeated)
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::se::SerMessageToIoWrite
        for MsgSingleField163<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<u64> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Fixed64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::fixed64_repeated(self),
                163,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for MsgSingleField163<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<u64> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                fixed64_repeated: value,
            }
        }
    }

    impl<ScalarType, RepeatedType> ::std::clone::Clone for MsgSingleField163<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<u64> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        RepeatedType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                fixed64_repeated: ::std::clone::Clone::clone(&self.fixed64_repeated),
            }
        }
    }

    pub struct MsgSingleField171<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
    {
        pub sfixed32_unlabeled: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField171<ScalarType> where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField171<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
    {
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            Self::SubmsgRepeatedMessageType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn sfixed32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.sfixed32_unlabeled,
            )))
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField171<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::SFixed32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::sfixed32_unlabeled_opt(self),
                171,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField171<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                sfixed32_unlabeled: value,
            }
        }
    }

    impl<ScalarType> ::std::clone::Clone for MsgSingleField171<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        ScalarType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                sfixed32_unlabeled: ::std::clone::Clone::clone(&self.sfixed32_unlabeled),
            }
        }
    }

    pub struct MsgSingleField172<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
    {
        pub sfixed32_optional: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField172<ScalarType> where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField172<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
    {
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            Self::SubmsgRepeatedMessageType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn sfixed32_optional_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.sfixed32_optional,
            )))
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField172<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::SFixed32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::sfixed32_optional_opt(self),
                172,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField172<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                sfixed32_optional: value,
            }
        }
    }

    impl<ScalarType> ::std::clone::Clone for MsgSingleField172<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        ScalarType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                sfixed32_optional: ::std::clone::Clone::clone(&self.sfixed32_optional),
            }
        }
    }

    pub struct MsgSingleField173<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub sfixed32_repeated: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::Msg>
        for MsgSingleField173<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::MsgTrait
        for MsgSingleField173<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            Self::SubmsgRepeatedMessageType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<'this, RepeatedType, ScalarType, i32>;

        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.sfixed32_repeated)
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::se::SerMessageToIoWrite
        for MsgSingleField173<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::SFixed32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::sfixed32_repeated(self),
                173,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for MsgSingleField173<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                sfixed32_repeated: value,
            }
        }
    }

    impl<ScalarType, RepeatedType> ::std::clone::Clone for MsgSingleField173<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        RepeatedType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                sfixed32_repeated: ::std::clone::Clone::clone(&self.sfixed32_repeated),
            }
        }
    }

    pub struct MsgSingleField181<ScalarType>
    where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
    {
        pub sfixed64_unlabeled: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField181<ScalarType> where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField181<ScalarType>
    where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
    {
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            Self::SubmsgRepeatedMessageType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn sfixed64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.sfixed64_unlabeled,
            )))
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField181<ScalarType>
    where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::SFixed64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::sfixed64_unlabeled_opt(self),
                181,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField181<ScalarType>
    where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                sfixed64_unlabeled: value,
            }
        }
    }

    impl<ScalarType> ::std::clone::Clone for MsgSingleField181<ScalarType>
    where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
        ScalarType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                sfixed64_unlabeled: ::std::clone::Clone::clone(&self.sfixed64_unlabeled),
            }
        }
    }

    pub struct MsgSingleField182<ScalarType>
    where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
    {
        pub sfixed64_optional: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField182<ScalarType> where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField182<ScalarType>
    where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
    {
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            Self::SubmsgRepeatedMessageType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn sfixed64_optional_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.sfixed64_optional,
            )))
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField182<ScalarType>
    where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::SFixed64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::sfixed64_optional_opt(self),
                182,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField182<ScalarType>
    where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                sfixed64_optional: value,
            }
        }
    }

    impl<ScalarType> ::std::clone::Clone for MsgSingleField182<ScalarType>
    where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
        ScalarType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                sfixed64_optional: ::std::clone::Clone::clone(&self.sfixed64_optional),
            }
        }
    }

    pub struct MsgSingleField183<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub sfixed64_repeated: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::Msg>
        for MsgSingleField183<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::MsgTrait
        for MsgSingleField183<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            Self::SubmsgRepeatedMessageType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<'this, RepeatedType, ScalarType, i64>;

        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.sfixed64_repeated)
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::se::SerMessageToIoWrite
        for MsgSingleField183<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::SFixed64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::sfixed64_repeated(self),
                183,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for MsgSingleField183<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                sfixed64_repeated: value,
            }
        }
    }

    impl<ScalarType, RepeatedType> ::std::clone::Clone for MsgSingleField183<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        RepeatedType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                sfixed64_repeated: ::std::clone::Clone::clone(&self.sfixed64_repeated),
            }
        }
    }

    pub struct MsgSingleField191<ScalarType>
    where
        ScalarType: ::std::convert::Into<f64> + ::std::clone::Clone,
    {
        pub f64_unlabeled: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField191<ScalarType> where
        ScalarType: ::std::convert::Into<f64> + ::std::clone::Clone
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField191<ScalarType>
    where
        ScalarType: ::std::convert::Into<f64> + ::std::clone::Clone,
    {
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            Self::SubmsgRepeatedMessageType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn f64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<f64> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.f64_unlabeled,
            )))
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField191<ScalarType>
    where
        ScalarType: ::std::convert::Into<f64> + ::std::clone::Clone,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Double,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::f64_unlabeled_opt(self),
                191,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField191<ScalarType>
    where
        ScalarType: ::std::convert::Into<f64> + ::std::clone::Clone,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                f64_unlabeled: value,
            }
        }
    }

    impl<ScalarType> ::std::clone::Clone for MsgSingleField191<ScalarType>
    where
        ScalarType: ::std::convert::Into<f64> + ::std::clone::Clone,
        ScalarType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                f64_unlabeled: ::std::clone::Clone::clone(&self.f64_unlabeled),
            }
        }
    }

    pub struct MsgSingleField192<ScalarType>
    where
        ScalarType: ::std::convert::Into<f64> + ::std::clone::Clone,
    {
        pub f64_optional: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField192<ScalarType> where
        ScalarType: ::std::convert::Into<f64> + ::std::clone::Clone
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField192<ScalarType>
    where
        ScalarType: ::std::convert::Into<f64> + ::std::clone::Clone,
    {
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            Self::SubmsgRepeatedMessageType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn f64_optional_opt<'this>(&'this self) -> ::std::option::Option<f64> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.f64_optional,
            )))
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField192<ScalarType>
    where
        ScalarType: ::std::convert::Into<f64> + ::std::clone::Clone,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Double,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::f64_optional_opt(self),
                192,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField192<ScalarType>
    where
        ScalarType: ::std::convert::Into<f64> + ::std::clone::Clone,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                f64_optional: value,
            }
        }
    }

    impl<ScalarType> ::std::clone::Clone for MsgSingleField192<ScalarType>
    where
        ScalarType: ::std::convert::Into<f64> + ::std::clone::Clone,
        ScalarType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                f64_optional: ::std::clone::Clone::clone(&self.f64_optional),
            }
        }
    }

    pub struct MsgSingleField193<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<f64> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub f64_repeated: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::Msg>
        for MsgSingleField193<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<f64> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::MsgTrait
        for MsgSingleField193<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<f64> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            Self::SubmsgRepeatedMessageType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<'this, RepeatedType, ScalarType, f64>;

        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.f64_repeated)
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::se::SerMessageToIoWrite
        for MsgSingleField193<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<f64> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Double,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::f64_repeated(self),
                193,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for MsgSingleField193<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<f64> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                f64_repeated: value,
            }
        }
    }

    impl<ScalarType, RepeatedType> ::std::clone::Clone for MsgSingleField193<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<f64> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        RepeatedType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                f64_repeated: ::std::clone::Clone::clone(&self.f64_repeated),
            }
        }
    }
    pub struct MsgBumpalo<'bump> {
    _bump: &'bump ::puroro::bumpalo::Bump,
    _bitfield: ::puroro::bitvec::array::BitArray<
        ::puroro::bitvec::order::Lsb0,
        [u32; (15 + 31) / 32],
    >,
    i32_unlabeled: ::puroro::internal::Bare<i32>,
    i32_optional: ::puroro::internal::Bare<i32>,
    i32_repeated: ::puroro::internal::NoAllocBumpVec<i32>,
    float_unlabeled: ::puroro::internal::Bare<f32>,
    float_optional: ::puroro::internal::Bare<f32>,
    float_repeated: ::puroro::internal::NoAllocBumpVec<f32>,
    bytes_unlabeled: ::puroro::internal::Bare<::puroro::internal::NoAllocBumpVec<u8>>,
    bytes_optional: ::puroro::internal::Bare<::puroro::internal::NoAllocBumpVec<u8>>,
    bytes_repeated: ::puroro::internal::NoAllocBumpVec<::puroro::internal::NoAllocBumpVec<u8>>,
    string_unlabeled: ::puroro::internal::Bare<::puroro::internal::NoAllocBumpString>,
    string_optional: ::puroro::internal::Bare<::puroro::internal::NoAllocBumpString>,
    string_repeated: ::puroro::internal::NoAllocBumpVec<::puroro::internal::NoAllocBumpString>,
    enum_unlabeled: ::puroro::internal::Bare<self::_puroro_root::full_coverage3::Enum>,
    enum_optional: ::puroro::internal::Bare<self::_puroro_root::full_coverage3::Enum>,
    enum_repeated: ::puroro::internal::NoAllocBumpVec<self::_puroro_root::full_coverage3::Enum>,
    submsg_unlabeled: ::std::option::Option<::puroro::internal::NoAllocBumpBox<self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgBumpalo<'bump>>>,
    submsg_optional: ::std::option::Option<::puroro::internal::NoAllocBumpBox<self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgBumpalo<'bump>>>,
    submsg_repeated: ::puroro::internal::NoAllocBumpVec<self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgBumpalo<'bump>>,
    i64_unlabeled: ::puroro::internal::Bare<i64>,
    i64_optional: ::puroro::internal::Bare<i64>,
    i64_repeated: ::puroro::internal::NoAllocBumpVec<i64>,
    u32_unlabeled: ::puroro::internal::Bare<u32>,
    u32_optional: ::puroro::internal::Bare<u32>,
    u32_repeated: ::puroro::internal::NoAllocBumpVec<u32>,
    u64_unlabeled: ::puroro::internal::Bare<u64>,
    u64_optional: ::puroro::internal::Bare<u64>,
    u64_repeated: ::puroro::internal::NoAllocBumpVec<u64>,
    s32_unlabeled: ::puroro::internal::Bare<i32>,
    s32_optional: ::puroro::internal::Bare<i32>,
    s32_repeated: ::puroro::internal::NoAllocBumpVec<i32>,
    s64_unlabeled: ::puroro::internal::Bare<i64>,
    s64_optional: ::puroro::internal::Bare<i64>,
    s64_repeated: ::puroro::internal::NoAllocBumpVec<i64>,
    fixed32_unlabeled: ::puroro::internal::Bare<u32>,
    fixed32_optional: ::puroro::internal::Bare<u32>,
    fixed32_repeated: ::puroro::internal::NoAllocBumpVec<u32>,
    fixed64_unlabeled: ::puroro::internal::Bare<u64>,
    fixed64_optional: ::puroro::internal::Bare<u64>,
    fixed64_repeated: ::puroro::internal::NoAllocBumpVec<u64>,
    sfixed32_unlabeled: ::puroro::internal::Bare<i32>,
    sfixed32_optional: ::puroro::internal::Bare<i32>,
    sfixed32_repeated: ::puroro::internal::NoAllocBumpVec<i32>,
    sfixed64_unlabeled: ::puroro::internal::Bare<i64>,
    sfixed64_optional: ::puroro::internal::Bare<i64>,
    sfixed64_repeated: ::puroro::internal::NoAllocBumpVec<i64>,
    f64_unlabeled: ::puroro::internal::Bare<f64>,
    f64_optional: ::puroro::internal::Bare<f64>,
    f64_repeated: ::puroro::internal::NoAllocBumpVec<f64>,
}

    pub type MsgBumpaloOwned = ::puroro::BumpaloOwned<MsgBumpalo<'static>>;
    impl<'bump> MsgBumpalo<'bump> {
        pub fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
            #[allow(unused)]
            let bump_ref: &::puroro::bumpalo::Bump =
                unsafe { ::std::mem::transmute(::std::ops::Deref::deref(&bump)) };

            Self {
                _bump: bump,
                _bitfield: ::std::default::Default::default(),
                i32_unlabeled: ::std::default::Default::default(),
                i32_optional: ::std::default::Default::default(),
                i32_repeated: ::std::default::Default::default(),
                float_unlabeled: ::std::default::Default::default(),
                float_optional: ::std::default::Default::default(),
                float_repeated: ::std::default::Default::default(),
                bytes_unlabeled: ::std::default::Default::default(),
                bytes_optional: ::std::default::Default::default(),
                bytes_repeated: ::std::default::Default::default(),
                string_unlabeled: ::std::default::Default::default(),
                string_optional: ::std::default::Default::default(),
                string_repeated: ::std::default::Default::default(),
                enum_unlabeled: ::std::default::Default::default(),
                enum_optional: ::std::default::Default::default(),
                enum_repeated: ::std::default::Default::default(),
                submsg_unlabeled: ::std::default::Default::default(),
                submsg_optional: ::std::default::Default::default(),
                submsg_repeated: ::std::default::Default::default(),
                i64_unlabeled: ::std::default::Default::default(),
                i64_optional: ::std::default::Default::default(),
                i64_repeated: ::std::default::Default::default(),
                u32_unlabeled: ::std::default::Default::default(),
                u32_optional: ::std::default::Default::default(),
                u32_repeated: ::std::default::Default::default(),
                u64_unlabeled: ::std::default::Default::default(),
                u64_optional: ::std::default::Default::default(),
                u64_repeated: ::std::default::Default::default(),
                s32_unlabeled: ::std::default::Default::default(),
                s32_optional: ::std::default::Default::default(),
                s32_repeated: ::std::default::Default::default(),
                s64_unlabeled: ::std::default::Default::default(),
                s64_optional: ::std::default::Default::default(),
                s64_repeated: ::std::default::Default::default(),
                fixed32_unlabeled: ::std::default::Default::default(),
                fixed32_optional: ::std::default::Default::default(),
                fixed32_repeated: ::std::default::Default::default(),
                fixed64_unlabeled: ::std::default::Default::default(),
                fixed64_optional: ::std::default::Default::default(),
                fixed64_repeated: ::std::default::Default::default(),
                sfixed32_unlabeled: ::std::default::Default::default(),
                sfixed32_optional: ::std::default::Default::default(),
                sfixed32_repeated: ::std::default::Default::default(),
                sfixed64_unlabeled: ::std::default::Default::default(),
                sfixed64_optional: ::std::default::Default::default(),
                sfixed64_repeated: ::std::default::Default::default(),
                f64_unlabeled: ::std::default::Default::default(),
                f64_optional: ::std::default::Default::default(),
                f64_repeated: ::std::default::Default::default(),
            }
        }
        pub fn i32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            if !::puroro::internal::IsDefault::is_default(&*self.i32_unlabeled) {
                ::std::option::Option::Some(self.i32_unlabeled.inner())
            } else {
                ::std::option::Option::None
            }
        }
        pub fn i32_unlabeled<'this>(&'this self) -> i32 {
            match self.i32_unlabeled_opt() {
                ::std::option::Option::Some(x) => x,
                _ => ::std::default::Default::default(),
            }
        }

        pub fn has_i32_unlabeled(&self) -> bool {
            self.i32_unlabeled_opt().is_some()
        }
        pub fn i32_optional_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            if self._bitfield.get(0).map_or(false, |v| *v) {
                ::std::option::Option::Some(self.i32_optional.inner())
            } else {
                ::std::option::Option::None
            }
        }
        pub fn i32_optional<'this>(&'this self) -> i32 {
            match self.i32_optional_opt() {
                ::std::option::Option::Some(x) => x,
                _ => ::std::default::Default::default(),
            }
        }

        pub fn has_i32_optional(&self) -> bool {
            self.i32_optional_opt().is_some()
        }
        pub fn i32_repeated<'this>(&'this self) -> &'this [i32] {
            &self.i32_repeated
        }
        pub fn float_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<f32> {
            if !::puroro::internal::IsDefault::is_default(&*self.float_unlabeled) {
                ::std::option::Option::Some(self.float_unlabeled.inner())
            } else {
                ::std::option::Option::None
            }
        }
        pub fn float_unlabeled<'this>(&'this self) -> f32 {
            match self.float_unlabeled_opt() {
                ::std::option::Option::Some(x) => x,
                _ => ::std::default::Default::default(),
            }
        }

        pub fn has_float_unlabeled(&self) -> bool {
            self.float_unlabeled_opt().is_some()
        }
        pub fn float_optional_opt<'this>(&'this self) -> ::std::option::Option<f32> {
            if self._bitfield.get(1).map_or(false, |v| *v) {
                ::std::option::Option::Some(self.float_optional.inner())
            } else {
                ::std::option::Option::None
            }
        }
        pub fn float_optional<'this>(&'this self) -> f32 {
            match self.float_optional_opt() {
                ::std::option::Option::Some(x) => x,
                _ => ::std::default::Default::default(),
            }
        }

        pub fn has_float_optional(&self) -> bool {
            self.float_optional_opt().is_some()
        }
        pub fn float_repeated<'this>(&'this self) -> &'this [f32] {
            &self.float_repeated
        }
        pub fn bytes_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<&'this [u8]> {
            if !::puroro::internal::IsDefault::is_default(&*self.bytes_unlabeled) {
                ::std::option::Option::Some(&self.bytes_unlabeled)
            } else {
                ::std::option::Option::None
            }
        }
        pub fn bytes_unlabeled<'this>(&'this self) -> &'this [u8] {
            match self.bytes_unlabeled_opt() {
                ::std::option::Option::Some(x) => x,
                _ => ::std::default::Default::default(),
            }
        }

        pub fn has_bytes_unlabeled(&self) -> bool {
            self.bytes_unlabeled_opt().is_some()
        }
        pub fn bytes_optional_opt<'this>(&'this self) -> ::std::option::Option<&'this [u8]> {
            if self._bitfield.get(2).map_or(false, |v| *v) {
                ::std::option::Option::Some(&self.bytes_optional)
            } else {
                ::std::option::Option::None
            }
        }
        pub fn bytes_optional<'this>(&'this self) -> &'this [u8] {
            match self.bytes_optional_opt() {
                ::std::option::Option::Some(x) => x,
                _ => ::std::default::Default::default(),
            }
        }

        pub fn has_bytes_optional(&self) -> bool {
            self.bytes_optional_opt().is_some()
        }
        pub fn bytes_repeated<'this>(
            &'this self,
        ) -> &'this [impl ::std::ops::Deref<Target = [u8]>] {
            &self.bytes_repeated
        }
        pub fn string_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            if !::puroro::internal::IsDefault::is_default(&*self.string_unlabeled) {
                ::std::option::Option::Some(&self.string_unlabeled)
            } else {
                ::std::option::Option::None
            }
        }
        pub fn string_unlabeled<'this>(&'this self) -> &'this str {
            match self.string_unlabeled_opt() {
                ::std::option::Option::Some(x) => x,
                _ => ::std::default::Default::default(),
            }
        }

        pub fn has_string_unlabeled(&self) -> bool {
            self.string_unlabeled_opt().is_some()
        }
        pub fn string_optional_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            if self._bitfield.get(3).map_or(false, |v| *v) {
                ::std::option::Option::Some(&self.string_optional)
            } else {
                ::std::option::Option::None
            }
        }
        pub fn string_optional<'this>(&'this self) -> &'this str {
            match self.string_optional_opt() {
                ::std::option::Option::Some(x) => x,
                _ => ::std::default::Default::default(),
            }
        }

        pub fn has_string_optional(&self) -> bool {
            self.string_optional_opt().is_some()
        }
        pub fn string_repeated<'this>(
            &'this self,
        ) -> &'this [impl ::std::ops::Deref<Target = str>] {
            &self.string_repeated
        }
        pub fn enum_unlabeled_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::full_coverage3::Enum> {
            if !::puroro::internal::IsDefault::is_default(&*self.enum_unlabeled) {
                ::std::option::Option::Some(self.enum_unlabeled.inner())
            } else {
                ::std::option::Option::None
            }
        }
        pub fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::full_coverage3::Enum {
            match self.enum_unlabeled_opt() {
                ::std::option::Option::Some(x) => x,
                _ => ::std::default::Default::default(),
            }
        }

        pub fn has_enum_unlabeled(&self) -> bool {
            self.enum_unlabeled_opt().is_some()
        }
        pub fn enum_optional_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::full_coverage3::Enum> {
            if self._bitfield.get(4).map_or(false, |v| *v) {
                ::std::option::Option::Some(self.enum_optional.inner())
            } else {
                ::std::option::Option::None
            }
        }
        pub fn enum_optional<'this>(&'this self) -> self::_puroro_root::full_coverage3::Enum {
            match self.enum_optional_opt() {
                ::std::option::Option::Some(x) => x,
                _ => ::std::default::Default::default(),
            }
        }

        pub fn has_enum_optional(&self) -> bool {
            self.enum_optional_opt().is_some()
        }
        pub fn enum_repeated<'this>(
            &'this self,
        ) -> &'this [self::_puroro_root::full_coverage3::Enum] {
            &self.enum_repeated
        }
        pub fn submsg_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<&'this self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgBumpalo<'this>>{
            self.submsg_unlabeled
                .as_ref()
                .map(|x| unsafe { ::std::mem::transmute(::std::ops::Deref::deref(x)) })
        }
        pub fn submsg_unlabeled<'this>(&'this self) -> ::std::option::Option<&'this self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgBumpalo<'this>>{
            self.submsg_unlabeled_opt()
        }

        pub fn has_submsg_unlabeled(&self) -> bool {
            self.submsg_unlabeled_opt().is_some()
        }
        pub fn submsg_optional_opt<'this>(&'this self) -> ::std::option::Option<&'this self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgBumpalo<'this>>{
            self.submsg_optional
                .as_ref()
                .map(|x| unsafe { ::std::mem::transmute(::std::ops::Deref::deref(x)) })
        }
        pub fn submsg_optional<'this>(&'this self) -> ::std::option::Option<&'this self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgBumpalo<'this>>{
            self.submsg_optional_opt()
        }

        pub fn has_submsg_optional(&self) -> bool {
            self.submsg_optional_opt().is_some()
        }
        pub fn submsg_repeated<'this>(&'this self) -> &'this[self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgBumpalo<'this>]{
            unsafe { self.submsg_repeated.cast_item_unchecked() }
        }
        pub fn i64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            if !::puroro::internal::IsDefault::is_default(&*self.i64_unlabeled) {
                ::std::option::Option::Some(self.i64_unlabeled.inner())
            } else {
                ::std::option::Option::None
            }
        }
        pub fn i64_unlabeled<'this>(&'this self) -> i64 {
            match self.i64_unlabeled_opt() {
                ::std::option::Option::Some(x) => x,
                _ => ::std::default::Default::default(),
            }
        }

        pub fn has_i64_unlabeled(&self) -> bool {
            self.i64_unlabeled_opt().is_some()
        }
        pub fn i64_optional_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            if self._bitfield.get(5).map_or(false, |v| *v) {
                ::std::option::Option::Some(self.i64_optional.inner())
            } else {
                ::std::option::Option::None
            }
        }
        pub fn i64_optional<'this>(&'this self) -> i64 {
            match self.i64_optional_opt() {
                ::std::option::Option::Some(x) => x,
                _ => ::std::default::Default::default(),
            }
        }

        pub fn has_i64_optional(&self) -> bool {
            self.i64_optional_opt().is_some()
        }
        pub fn i64_repeated<'this>(&'this self) -> &'this [i64] {
            &self.i64_repeated
        }
        pub fn u32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<u32> {
            if !::puroro::internal::IsDefault::is_default(&*self.u32_unlabeled) {
                ::std::option::Option::Some(self.u32_unlabeled.inner())
            } else {
                ::std::option::Option::None
            }
        }
        pub fn u32_unlabeled<'this>(&'this self) -> u32 {
            match self.u32_unlabeled_opt() {
                ::std::option::Option::Some(x) => x,
                _ => ::std::default::Default::default(),
            }
        }

        pub fn has_u32_unlabeled(&self) -> bool {
            self.u32_unlabeled_opt().is_some()
        }
        pub fn u32_optional_opt<'this>(&'this self) -> ::std::option::Option<u32> {
            if self._bitfield.get(6).map_or(false, |v| *v) {
                ::std::option::Option::Some(self.u32_optional.inner())
            } else {
                ::std::option::Option::None
            }
        }
        pub fn u32_optional<'this>(&'this self) -> u32 {
            match self.u32_optional_opt() {
                ::std::option::Option::Some(x) => x,
                _ => ::std::default::Default::default(),
            }
        }

        pub fn has_u32_optional(&self) -> bool {
            self.u32_optional_opt().is_some()
        }
        pub fn u32_repeated<'this>(&'this self) -> &'this [u32] {
            &self.u32_repeated
        }
        pub fn u64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<u64> {
            if !::puroro::internal::IsDefault::is_default(&*self.u64_unlabeled) {
                ::std::option::Option::Some(self.u64_unlabeled.inner())
            } else {
                ::std::option::Option::None
            }
        }
        pub fn u64_unlabeled<'this>(&'this self) -> u64 {
            match self.u64_unlabeled_opt() {
                ::std::option::Option::Some(x) => x,
                _ => ::std::default::Default::default(),
            }
        }

        pub fn has_u64_unlabeled(&self) -> bool {
            self.u64_unlabeled_opt().is_some()
        }
        pub fn u64_optional_opt<'this>(&'this self) -> ::std::option::Option<u64> {
            if self._bitfield.get(7).map_or(false, |v| *v) {
                ::std::option::Option::Some(self.u64_optional.inner())
            } else {
                ::std::option::Option::None
            }
        }
        pub fn u64_optional<'this>(&'this self) -> u64 {
            match self.u64_optional_opt() {
                ::std::option::Option::Some(x) => x,
                _ => ::std::default::Default::default(),
            }
        }

        pub fn has_u64_optional(&self) -> bool {
            self.u64_optional_opt().is_some()
        }
        pub fn u64_repeated<'this>(&'this self) -> &'this [u64] {
            &self.u64_repeated
        }
        pub fn s32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            if !::puroro::internal::IsDefault::is_default(&*self.s32_unlabeled) {
                ::std::option::Option::Some(self.s32_unlabeled.inner())
            } else {
                ::std::option::Option::None
            }
        }
        pub fn s32_unlabeled<'this>(&'this self) -> i32 {
            match self.s32_unlabeled_opt() {
                ::std::option::Option::Some(x) => x,
                _ => ::std::default::Default::default(),
            }
        }

        pub fn has_s32_unlabeled(&self) -> bool {
            self.s32_unlabeled_opt().is_some()
        }
        pub fn s32_optional_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            if self._bitfield.get(8).map_or(false, |v| *v) {
                ::std::option::Option::Some(self.s32_optional.inner())
            } else {
                ::std::option::Option::None
            }
        }
        pub fn s32_optional<'this>(&'this self) -> i32 {
            match self.s32_optional_opt() {
                ::std::option::Option::Some(x) => x,
                _ => ::std::default::Default::default(),
            }
        }

        pub fn has_s32_optional(&self) -> bool {
            self.s32_optional_opt().is_some()
        }
        pub fn s32_repeated<'this>(&'this self) -> &'this [i32] {
            &self.s32_repeated
        }
        pub fn s64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            if !::puroro::internal::IsDefault::is_default(&*self.s64_unlabeled) {
                ::std::option::Option::Some(self.s64_unlabeled.inner())
            } else {
                ::std::option::Option::None
            }
        }
        pub fn s64_unlabeled<'this>(&'this self) -> i64 {
            match self.s64_unlabeled_opt() {
                ::std::option::Option::Some(x) => x,
                _ => ::std::default::Default::default(),
            }
        }

        pub fn has_s64_unlabeled(&self) -> bool {
            self.s64_unlabeled_opt().is_some()
        }
        pub fn s64_optional_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            if self._bitfield.get(9).map_or(false, |v| *v) {
                ::std::option::Option::Some(self.s64_optional.inner())
            } else {
                ::std::option::Option::None
            }
        }
        pub fn s64_optional<'this>(&'this self) -> i64 {
            match self.s64_optional_opt() {
                ::std::option::Option::Some(x) => x,
                _ => ::std::default::Default::default(),
            }
        }

        pub fn has_s64_optional(&self) -> bool {
            self.s64_optional_opt().is_some()
        }
        pub fn s64_repeated<'this>(&'this self) -> &'this [i64] {
            &self.s64_repeated
        }
        pub fn fixed32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<u32> {
            if !::puroro::internal::IsDefault::is_default(&*self.fixed32_unlabeled) {
                ::std::option::Option::Some(self.fixed32_unlabeled.inner())
            } else {
                ::std::option::Option::None
            }
        }
        pub fn fixed32_unlabeled<'this>(&'this self) -> u32 {
            match self.fixed32_unlabeled_opt() {
                ::std::option::Option::Some(x) => x,
                _ => ::std::default::Default::default(),
            }
        }

        pub fn has_fixed32_unlabeled(&self) -> bool {
            self.fixed32_unlabeled_opt().is_some()
        }
        pub fn fixed32_optional_opt<'this>(&'this self) -> ::std::option::Option<u32> {
            if self._bitfield.get(10).map_or(false, |v| *v) {
                ::std::option::Option::Some(self.fixed32_optional.inner())
            } else {
                ::std::option::Option::None
            }
        }
        pub fn fixed32_optional<'this>(&'this self) -> u32 {
            match self.fixed32_optional_opt() {
                ::std::option::Option::Some(x) => x,
                _ => ::std::default::Default::default(),
            }
        }

        pub fn has_fixed32_optional(&self) -> bool {
            self.fixed32_optional_opt().is_some()
        }
        pub fn fixed32_repeated<'this>(&'this self) -> &'this [u32] {
            &self.fixed32_repeated
        }
        pub fn fixed64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<u64> {
            if !::puroro::internal::IsDefault::is_default(&*self.fixed64_unlabeled) {
                ::std::option::Option::Some(self.fixed64_unlabeled.inner())
            } else {
                ::std::option::Option::None
            }
        }
        pub fn fixed64_unlabeled<'this>(&'this self) -> u64 {
            match self.fixed64_unlabeled_opt() {
                ::std::option::Option::Some(x) => x,
                _ => ::std::default::Default::default(),
            }
        }

        pub fn has_fixed64_unlabeled(&self) -> bool {
            self.fixed64_unlabeled_opt().is_some()
        }
        pub fn fixed64_optional_opt<'this>(&'this self) -> ::std::option::Option<u64> {
            if self._bitfield.get(11).map_or(false, |v| *v) {
                ::std::option::Option::Some(self.fixed64_optional.inner())
            } else {
                ::std::option::Option::None
            }
        }
        pub fn fixed64_optional<'this>(&'this self) -> u64 {
            match self.fixed64_optional_opt() {
                ::std::option::Option::Some(x) => x,
                _ => ::std::default::Default::default(),
            }
        }

        pub fn has_fixed64_optional(&self) -> bool {
            self.fixed64_optional_opt().is_some()
        }
        pub fn fixed64_repeated<'this>(&'this self) -> &'this [u64] {
            &self.fixed64_repeated
        }
        pub fn sfixed32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            if !::puroro::internal::IsDefault::is_default(&*self.sfixed32_unlabeled) {
                ::std::option::Option::Some(self.sfixed32_unlabeled.inner())
            } else {
                ::std::option::Option::None
            }
        }
        pub fn sfixed32_unlabeled<'this>(&'this self) -> i32 {
            match self.sfixed32_unlabeled_opt() {
                ::std::option::Option::Some(x) => x,
                _ => ::std::default::Default::default(),
            }
        }

        pub fn has_sfixed32_unlabeled(&self) -> bool {
            self.sfixed32_unlabeled_opt().is_some()
        }
        pub fn sfixed32_optional_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            if self._bitfield.get(12).map_or(false, |v| *v) {
                ::std::option::Option::Some(self.sfixed32_optional.inner())
            } else {
                ::std::option::Option::None
            }
        }
        pub fn sfixed32_optional<'this>(&'this self) -> i32 {
            match self.sfixed32_optional_opt() {
                ::std::option::Option::Some(x) => x,
                _ => ::std::default::Default::default(),
            }
        }

        pub fn has_sfixed32_optional(&self) -> bool {
            self.sfixed32_optional_opt().is_some()
        }
        pub fn sfixed32_repeated<'this>(&'this self) -> &'this [i32] {
            &self.sfixed32_repeated
        }
        pub fn sfixed64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            if !::puroro::internal::IsDefault::is_default(&*self.sfixed64_unlabeled) {
                ::std::option::Option::Some(self.sfixed64_unlabeled.inner())
            } else {
                ::std::option::Option::None
            }
        }
        pub fn sfixed64_unlabeled<'this>(&'this self) -> i64 {
            match self.sfixed64_unlabeled_opt() {
                ::std::option::Option::Some(x) => x,
                _ => ::std::default::Default::default(),
            }
        }

        pub fn has_sfixed64_unlabeled(&self) -> bool {
            self.sfixed64_unlabeled_opt().is_some()
        }
        pub fn sfixed64_optional_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            if self._bitfield.get(13).map_or(false, |v| *v) {
                ::std::option::Option::Some(self.sfixed64_optional.inner())
            } else {
                ::std::option::Option::None
            }
        }
        pub fn sfixed64_optional<'this>(&'this self) -> i64 {
            match self.sfixed64_optional_opt() {
                ::std::option::Option::Some(x) => x,
                _ => ::std::default::Default::default(),
            }
        }

        pub fn has_sfixed64_optional(&self) -> bool {
            self.sfixed64_optional_opt().is_some()
        }
        pub fn sfixed64_repeated<'this>(&'this self) -> &'this [i64] {
            &self.sfixed64_repeated
        }
        pub fn f64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<f64> {
            if !::puroro::internal::IsDefault::is_default(&*self.f64_unlabeled) {
                ::std::option::Option::Some(self.f64_unlabeled.inner())
            } else {
                ::std::option::Option::None
            }
        }
        pub fn f64_unlabeled<'this>(&'this self) -> f64 {
            match self.f64_unlabeled_opt() {
                ::std::option::Option::Some(x) => x,
                _ => ::std::default::Default::default(),
            }
        }

        pub fn has_f64_unlabeled(&self) -> bool {
            self.f64_unlabeled_opt().is_some()
        }
        pub fn f64_optional_opt<'this>(&'this self) -> ::std::option::Option<f64> {
            if self._bitfield.get(14).map_or(false, |v| *v) {
                ::std::option::Option::Some(self.f64_optional.inner())
            } else {
                ::std::option::Option::None
            }
        }
        pub fn f64_optional<'this>(&'this self) -> f64 {
            match self.f64_optional_opt() {
                ::std::option::Option::Some(x) => x,
                _ => ::std::default::Default::default(),
            }
        }

        pub fn has_f64_optional(&self) -> bool {
            self.f64_optional_opt().is_some()
        }
        pub fn f64_repeated<'this>(&'this self) -> &'this [f64] {
            &self.f64_repeated
        }
        pub fn clear_i32_unlabeled(&mut self) {
            self.i32_unlabeled = ::std::default::Default::default();
        }
        pub fn i32_unlabeled_mut<'this>(&'this mut self) -> &'this mut i32 {
            if !self.has_i32_unlabeled() {
                self.i32_unlabeled = ::std::default::Default::default();
            }
            &mut self.i32_unlabeled
        }
        pub fn clear_i32_optional(&mut self) {
            self._bitfield.set(0, false);
        }
        pub fn i32_optional_mut<'this>(&'this mut self) -> &'this mut i32 {
            if !self.has_i32_optional() {
                self.i32_optional = ::std::default::Default::default();
                self._bitfield.set(0, true);
            }
            &mut self.i32_optional
        }
        pub fn i32_repeated_mut<'this>(
            &'this mut self,
        ) -> ::puroro::internal::RefMutBumpVec<'bump, 'this, i32> {
            unsafe { self.i32_repeated.as_mut_vec_in(self._bump) }
        }
        pub fn clear_float_unlabeled(&mut self) {
            self.float_unlabeled = ::std::default::Default::default();
        }
        pub fn float_unlabeled_mut<'this>(&'this mut self) -> &'this mut f32 {
            if !self.has_float_unlabeled() {
                self.float_unlabeled = ::std::default::Default::default();
            }
            &mut self.float_unlabeled
        }
        pub fn clear_float_optional(&mut self) {
            self._bitfield.set(1, false);
        }
        pub fn float_optional_mut<'this>(&'this mut self) -> &'this mut f32 {
            if !self.has_float_optional() {
                self.float_optional = ::std::default::Default::default();
                self._bitfield.set(1, true);
            }
            &mut self.float_optional
        }
        pub fn float_repeated_mut<'this>(
            &'this mut self,
        ) -> ::puroro::internal::RefMutBumpVec<'bump, 'this, f32> {
            unsafe { self.float_repeated.as_mut_vec_in(self._bump) }
        }
        pub fn clear_bytes_unlabeled(&mut self) {
            self.bytes_unlabeled = ::std::default::Default::default();
        }
        pub fn bytes_unlabeled_mut<'this>(
            &'this mut self,
        ) -> impl 'this + ::std::ops::DerefMut<Target = ::puroro::bumpalo::collections::Vec<'bump, u8>>
        {
            if !self.has_bytes_unlabeled() {
                self.bytes_unlabeled = ::std::default::Default::default();
            }
            unsafe { self.bytes_unlabeled.as_mut_vec_in(self._bump) }
        }
        pub fn clear_bytes_optional(&mut self) {
            self._bitfield.set(2, false);
        }
        pub fn bytes_optional_mut<'this>(
            &'this mut self,
        ) -> impl 'this + ::std::ops::DerefMut<Target = ::puroro::bumpalo::collections::Vec<'bump, u8>>
        {
            if !self.has_bytes_optional() {
                self.bytes_optional = ::std::default::Default::default();
                self._bitfield.set(2, true);
            }
            unsafe { self.bytes_optional.as_mut_vec_in(self._bump) }
        }
        pub fn bytes_repeated_mut<'this>(
            &'this mut self,
        ) -> ::puroro::internal::AddBumpVecView<'bump, 'this, ::puroro::internal::NoAllocBumpVec<u8>>
        {
            unsafe { self.bytes_repeated.as_add_bump_vec_view_in(self._bump) }
        }
        pub fn clear_string_unlabeled(&mut self) {
            self.string_unlabeled = ::std::default::Default::default();
        }
        pub fn string_unlabeled_mut<'this>(
            &'this mut self,
        ) -> impl 'this + ::std::ops::DerefMut<Target = ::puroro::bumpalo::collections::String<'bump>>
        {
            if !self.has_string_unlabeled() {
                self.string_unlabeled = ::std::default::Default::default();
            }
            unsafe { self.string_unlabeled.as_mut_string_in(self._bump) }
        }
        pub fn clear_string_optional(&mut self) {
            self._bitfield.set(3, false);
        }
        pub fn string_optional_mut<'this>(
            &'this mut self,
        ) -> impl 'this + ::std::ops::DerefMut<Target = ::puroro::bumpalo::collections::String<'bump>>
        {
            if !self.has_string_optional() {
                self.string_optional = ::std::default::Default::default();
                self._bitfield.set(3, true);
            }
            unsafe { self.string_optional.as_mut_string_in(self._bump) }
        }
        pub fn string_repeated_mut<'this>(
            &'this mut self,
        ) -> ::puroro::internal::AddBumpVecView<'bump, 'this, ::puroro::internal::NoAllocBumpString>
        {
            unsafe { self.string_repeated.as_add_bump_vec_view_in(self._bump) }
        }
        pub fn clear_enum_unlabeled(&mut self) {
            self.enum_unlabeled = ::std::default::Default::default();
        }
        pub fn enum_unlabeled_mut<'this>(
            &'this mut self,
        ) -> &'this mut self::_puroro_root::full_coverage3::Enum {
            if !self.has_enum_unlabeled() {
                self.enum_unlabeled = ::std::default::Default::default();
            }
            &mut self.enum_unlabeled
        }
        pub fn clear_enum_optional(&mut self) {
            self._bitfield.set(4, false);
        }
        pub fn enum_optional_mut<'this>(
            &'this mut self,
        ) -> &'this mut self::_puroro_root::full_coverage3::Enum {
            if !self.has_enum_optional() {
                self.enum_optional = ::std::default::Default::default();
                self._bitfield.set(4, true);
            }
            &mut self.enum_optional
        }
        pub fn enum_repeated_mut<'this>(
            &'this mut self,
        ) -> ::puroro::internal::RefMutBumpVec<'bump, 'this, self::_puroro_root::full_coverage3::Enum>
        {
            unsafe { self.enum_repeated.as_mut_vec_in(self._bump) }
        }
        pub fn clear_submsg_unlabeled(&mut self) {
            self.submsg_unlabeled = ::std::default::Default::default();
        }
        pub fn submsg_unlabeled_mut<'this>(&'this mut self) -> &'this mut self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgBumpalo<'bump>{
            if !self.has_submsg_unlabeled() {
                self.submsg_unlabeled = ::std::default::Default::default();
            }
            let bump = self._bump;
            self.submsg_unlabeled.get_or_insert_with(|| {
                ::puroro::internal::NoAllocBumpBox::new_in(
                    ::puroro::internal::BumpDefault::default_in(bump),
                    bump,
                )
            })
        }
        pub fn clear_submsg_optional(&mut self) {
            self.submsg_optional = ::std::default::Default::default();
        }
        pub fn submsg_optional_mut<'this>(&'this mut self) -> &'this mut self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgBumpalo<'bump>{
            if !self.has_submsg_optional() {
                self.submsg_optional = ::std::default::Default::default();
            }
            let bump = self._bump;
            self.submsg_optional.get_or_insert_with(|| {
                ::puroro::internal::NoAllocBumpBox::new_in(
                    ::puroro::internal::BumpDefault::default_in(bump),
                    bump,
                )
            })
        }
        pub fn submsg_repeated_mut<'this>(
            &'this mut self,
        ) -> ::puroro::internal::RefMutBumpVec<
            'bump,
            'this,
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgBumpalo<
                'bump,
            >,
        > {
            unsafe { self.submsg_repeated.as_mut_vec_in(self._bump) }
        }
        pub fn clear_i64_unlabeled(&mut self) {
            self.i64_unlabeled = ::std::default::Default::default();
        }
        pub fn i64_unlabeled_mut<'this>(&'this mut self) -> &'this mut i64 {
            if !self.has_i64_unlabeled() {
                self.i64_unlabeled = ::std::default::Default::default();
            }
            &mut self.i64_unlabeled
        }
        pub fn clear_i64_optional(&mut self) {
            self._bitfield.set(5, false);
        }
        pub fn i64_optional_mut<'this>(&'this mut self) -> &'this mut i64 {
            if !self.has_i64_optional() {
                self.i64_optional = ::std::default::Default::default();
                self._bitfield.set(5, true);
            }
            &mut self.i64_optional
        }
        pub fn i64_repeated_mut<'this>(
            &'this mut self,
        ) -> ::puroro::internal::RefMutBumpVec<'bump, 'this, i64> {
            unsafe { self.i64_repeated.as_mut_vec_in(self._bump) }
        }
        pub fn clear_u32_unlabeled(&mut self) {
            self.u32_unlabeled = ::std::default::Default::default();
        }
        pub fn u32_unlabeled_mut<'this>(&'this mut self) -> &'this mut u32 {
            if !self.has_u32_unlabeled() {
                self.u32_unlabeled = ::std::default::Default::default();
            }
            &mut self.u32_unlabeled
        }
        pub fn clear_u32_optional(&mut self) {
            self._bitfield.set(6, false);
        }
        pub fn u32_optional_mut<'this>(&'this mut self) -> &'this mut u32 {
            if !self.has_u32_optional() {
                self.u32_optional = ::std::default::Default::default();
                self._bitfield.set(6, true);
            }
            &mut self.u32_optional
        }
        pub fn u32_repeated_mut<'this>(
            &'this mut self,
        ) -> ::puroro::internal::RefMutBumpVec<'bump, 'this, u32> {
            unsafe { self.u32_repeated.as_mut_vec_in(self._bump) }
        }
        pub fn clear_u64_unlabeled(&mut self) {
            self.u64_unlabeled = ::std::default::Default::default();
        }
        pub fn u64_unlabeled_mut<'this>(&'this mut self) -> &'this mut u64 {
            if !self.has_u64_unlabeled() {
                self.u64_unlabeled = ::std::default::Default::default();
            }
            &mut self.u64_unlabeled
        }
        pub fn clear_u64_optional(&mut self) {
            self._bitfield.set(7, false);
        }
        pub fn u64_optional_mut<'this>(&'this mut self) -> &'this mut u64 {
            if !self.has_u64_optional() {
                self.u64_optional = ::std::default::Default::default();
                self._bitfield.set(7, true);
            }
            &mut self.u64_optional
        }
        pub fn u64_repeated_mut<'this>(
            &'this mut self,
        ) -> ::puroro::internal::RefMutBumpVec<'bump, 'this, u64> {
            unsafe { self.u64_repeated.as_mut_vec_in(self._bump) }
        }
        pub fn clear_s32_unlabeled(&mut self) {
            self.s32_unlabeled = ::std::default::Default::default();
        }
        pub fn s32_unlabeled_mut<'this>(&'this mut self) -> &'this mut i32 {
            if !self.has_s32_unlabeled() {
                self.s32_unlabeled = ::std::default::Default::default();
            }
            &mut self.s32_unlabeled
        }
        pub fn clear_s32_optional(&mut self) {
            self._bitfield.set(8, false);
        }
        pub fn s32_optional_mut<'this>(&'this mut self) -> &'this mut i32 {
            if !self.has_s32_optional() {
                self.s32_optional = ::std::default::Default::default();
                self._bitfield.set(8, true);
            }
            &mut self.s32_optional
        }
        pub fn s32_repeated_mut<'this>(
            &'this mut self,
        ) -> ::puroro::internal::RefMutBumpVec<'bump, 'this, i32> {
            unsafe { self.s32_repeated.as_mut_vec_in(self._bump) }
        }
        pub fn clear_s64_unlabeled(&mut self) {
            self.s64_unlabeled = ::std::default::Default::default();
        }
        pub fn s64_unlabeled_mut<'this>(&'this mut self) -> &'this mut i64 {
            if !self.has_s64_unlabeled() {
                self.s64_unlabeled = ::std::default::Default::default();
            }
            &mut self.s64_unlabeled
        }
        pub fn clear_s64_optional(&mut self) {
            self._bitfield.set(9, false);
        }
        pub fn s64_optional_mut<'this>(&'this mut self) -> &'this mut i64 {
            if !self.has_s64_optional() {
                self.s64_optional = ::std::default::Default::default();
                self._bitfield.set(9, true);
            }
            &mut self.s64_optional
        }
        pub fn s64_repeated_mut<'this>(
            &'this mut self,
        ) -> ::puroro::internal::RefMutBumpVec<'bump, 'this, i64> {
            unsafe { self.s64_repeated.as_mut_vec_in(self._bump) }
        }
        pub fn clear_fixed32_unlabeled(&mut self) {
            self.fixed32_unlabeled = ::std::default::Default::default();
        }
        pub fn fixed32_unlabeled_mut<'this>(&'this mut self) -> &'this mut u32 {
            if !self.has_fixed32_unlabeled() {
                self.fixed32_unlabeled = ::std::default::Default::default();
            }
            &mut self.fixed32_unlabeled
        }
        pub fn clear_fixed32_optional(&mut self) {
            self._bitfield.set(10, false);
        }
        pub fn fixed32_optional_mut<'this>(&'this mut self) -> &'this mut u32 {
            if !self.has_fixed32_optional() {
                self.fixed32_optional = ::std::default::Default::default();
                self._bitfield.set(10, true);
            }
            &mut self.fixed32_optional
        }
        pub fn fixed32_repeated_mut<'this>(
            &'this mut self,
        ) -> ::puroro::internal::RefMutBumpVec<'bump, 'this, u32> {
            unsafe { self.fixed32_repeated.as_mut_vec_in(self._bump) }
        }
        pub fn clear_fixed64_unlabeled(&mut self) {
            self.fixed64_unlabeled = ::std::default::Default::default();
        }
        pub fn fixed64_unlabeled_mut<'this>(&'this mut self) -> &'this mut u64 {
            if !self.has_fixed64_unlabeled() {
                self.fixed64_unlabeled = ::std::default::Default::default();
            }
            &mut self.fixed64_unlabeled
        }
        pub fn clear_fixed64_optional(&mut self) {
            self._bitfield.set(11, false);
        }
        pub fn fixed64_optional_mut<'this>(&'this mut self) -> &'this mut u64 {
            if !self.has_fixed64_optional() {
                self.fixed64_optional = ::std::default::Default::default();
                self._bitfield.set(11, true);
            }
            &mut self.fixed64_optional
        }
        pub fn fixed64_repeated_mut<'this>(
            &'this mut self,
        ) -> ::puroro::internal::RefMutBumpVec<'bump, 'this, u64> {
            unsafe { self.fixed64_repeated.as_mut_vec_in(self._bump) }
        }
        pub fn clear_sfixed32_unlabeled(&mut self) {
            self.sfixed32_unlabeled = ::std::default::Default::default();
        }
        pub fn sfixed32_unlabeled_mut<'this>(&'this mut self) -> &'this mut i32 {
            if !self.has_sfixed32_unlabeled() {
                self.sfixed32_unlabeled = ::std::default::Default::default();
            }
            &mut self.sfixed32_unlabeled
        }
        pub fn clear_sfixed32_optional(&mut self) {
            self._bitfield.set(12, false);
        }
        pub fn sfixed32_optional_mut<'this>(&'this mut self) -> &'this mut i32 {
            if !self.has_sfixed32_optional() {
                self.sfixed32_optional = ::std::default::Default::default();
                self._bitfield.set(12, true);
            }
            &mut self.sfixed32_optional
        }
        pub fn sfixed32_repeated_mut<'this>(
            &'this mut self,
        ) -> ::puroro::internal::RefMutBumpVec<'bump, 'this, i32> {
            unsafe { self.sfixed32_repeated.as_mut_vec_in(self._bump) }
        }
        pub fn clear_sfixed64_unlabeled(&mut self) {
            self.sfixed64_unlabeled = ::std::default::Default::default();
        }
        pub fn sfixed64_unlabeled_mut<'this>(&'this mut self) -> &'this mut i64 {
            if !self.has_sfixed64_unlabeled() {
                self.sfixed64_unlabeled = ::std::default::Default::default();
            }
            &mut self.sfixed64_unlabeled
        }
        pub fn clear_sfixed64_optional(&mut self) {
            self._bitfield.set(13, false);
        }
        pub fn sfixed64_optional_mut<'this>(&'this mut self) -> &'this mut i64 {
            if !self.has_sfixed64_optional() {
                self.sfixed64_optional = ::std::default::Default::default();
                self._bitfield.set(13, true);
            }
            &mut self.sfixed64_optional
        }
        pub fn sfixed64_repeated_mut<'this>(
            &'this mut self,
        ) -> ::puroro::internal::RefMutBumpVec<'bump, 'this, i64> {
            unsafe { self.sfixed64_repeated.as_mut_vec_in(self._bump) }
        }
        pub fn clear_f64_unlabeled(&mut self) {
            self.f64_unlabeled = ::std::default::Default::default();
        }
        pub fn f64_unlabeled_mut<'this>(&'this mut self) -> &'this mut f64 {
            if !self.has_f64_unlabeled() {
                self.f64_unlabeled = ::std::default::Default::default();
            }
            &mut self.f64_unlabeled
        }
        pub fn clear_f64_optional(&mut self) {
            self._bitfield.set(14, false);
        }
        pub fn f64_optional_mut<'this>(&'this mut self) -> &'this mut f64 {
            if !self.has_f64_optional() {
                self.f64_optional = ::std::default::Default::default();
                self._bitfield.set(14, true);
            }
            &mut self.f64_optional
        }
        pub fn f64_repeated_mut<'this>(
            &'this mut self,
        ) -> ::puroro::internal::RefMutBumpVec<'bump, 'this, f64> {
            unsafe { self.f64_repeated.as_mut_vec_in(self._bump) }
        }
    }
    impl<'bump> ::puroro::Message<super::_puroro_simple_impl::Msg> for MsgBumpalo<'bump> {}

    impl<'bump> ::puroro::BumpaloMessage<'bump> for MsgBumpalo<'bump> {
        fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
            Self::new_in(bump)
        }
    }

    impl<'bump> ::puroro::internal::BumpDefault<'bump> for MsgBumpalo<'bump> {
        fn default_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
            Self::new_in(bump)
        }
    }

    impl<'bump> super::_puroro_traits::MsgTrait for MsgBumpalo<'bump> {
        fn i32_unlabeled_opt<'this>(&'this self) -> Option<i32> {
            <Self>::i32_unlabeled_opt(self)
        }
        fn i32_optional_opt<'this>(&'this self) -> Option<i32> {
            <Self>::i32_optional_opt(self)
        }
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<
            'this,
            ::puroro::internal::NoAllocBumpVec<i32>,
            i32,
            i32,
        >;

        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.i32_repeated)
        }
        fn float_unlabeled_opt<'this>(&'this self) -> Option<f32> {
            <Self>::float_unlabeled_opt(self)
        }
        fn float_optional_opt<'this>(&'this self) -> Option<f32> {
            <Self>::float_optional_opt(self)
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<
            'this,
            ::puroro::internal::NoAllocBumpVec<f32>,
            f32,
            f32,
        >;

        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.float_repeated)
        }
        fn bytes_unlabeled_opt<'this>(&'this self) -> Option<&'this [u8]> {
            <Self>::bytes_unlabeled_opt(self)
        }
        fn bytes_optional_opt<'this>(&'this self) -> Option<&'this [u8]> {
            <Self>::bytes_optional_opt(self)
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::AsRefRepeatedField<
            'this,
            ::puroro::internal::NoAllocBumpVec<::puroro::internal::NoAllocBumpVec<u8>>,
            ::puroro::internal::NoAllocBumpVec<u8>,
            [u8],
        >;

        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::AsRefRepeatedField::new(&self.bytes_repeated)
        }
        fn string_unlabeled_opt<'this>(&'this self) -> Option<&'this str> {
            <Self>::string_unlabeled_opt(self)
        }
        fn string_optional_opt<'this>(&'this self) -> Option<&'this str> {
            <Self>::string_optional_opt(self)
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::AsRefRepeatedField<
            'this,
            ::puroro::internal::NoAllocBumpVec<::puroro::internal::NoAllocBumpString>,
            ::puroro::internal::NoAllocBumpString,
            str,
        >;

        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::AsRefRepeatedField::new(&self.string_repeated)
        }
        fn enum_unlabeled_opt<'this>(
            &'this self,
        ) -> Option<self::_puroro_root::full_coverage3::Enum> {
            <Self>::enum_unlabeled_opt(self)
        }
        fn enum_optional_opt<'this>(
            &'this self,
        ) -> Option<self::_puroro_root::full_coverage3::Enum> {
            <Self>::enum_optional_opt(self)
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<
            'this,
            ::puroro::internal::NoAllocBumpVec<self::_puroro_root::full_coverage3::Enum>,
            self::_puroro_root::full_coverage3::Enum,
            self::_puroro_root::full_coverage3::Enum,
        >;

        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.enum_repeated)
        }
        type SubmsgUnlabeledMessageType<'this> where Self: 'this = &'this self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgBumpalo<'this>;
        fn submsg_unlabeled_opt<'this>(
            &'this self,
        ) -> Option<Self::SubmsgUnlabeledMessageType<'this>> {
            <Self>::submsg_unlabeled_opt(self)
        }
        type SubmsgOptionalMessageType<'this> where Self: 'this = &'this self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgBumpalo<'this>;
        fn submsg_optional_opt<'this>(
            &'this self,
        ) -> Option<Self::SubmsgOptionalMessageType<'this>> {
            <Self>::submsg_optional_opt(self)
        }
        type SubmsgRepeatedMessageType<'this> where Self: 'this = &'this self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgBumpalo<'this>;
        type SubmsgRepeatedRepeatedType<'this> where Self: 'this =
    &'this [self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgBumpalo<'this>];

        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            unsafe { self.submsg_repeated.cast_item_unchecked() }
        }
        fn i64_unlabeled_opt<'this>(&'this self) -> Option<i64> {
            <Self>::i64_unlabeled_opt(self)
        }
        fn i64_optional_opt<'this>(&'this self) -> Option<i64> {
            <Self>::i64_optional_opt(self)
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<
            'this,
            ::puroro::internal::NoAllocBumpVec<i64>,
            i64,
            i64,
        >;

        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.i64_repeated)
        }
        fn u32_unlabeled_opt<'this>(&'this self) -> Option<u32> {
            <Self>::u32_unlabeled_opt(self)
        }
        fn u32_optional_opt<'this>(&'this self) -> Option<u32> {
            <Self>::u32_optional_opt(self)
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<
            'this,
            ::puroro::internal::NoAllocBumpVec<u32>,
            u32,
            u32,
        >;

        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.u32_repeated)
        }
        fn u64_unlabeled_opt<'this>(&'this self) -> Option<u64> {
            <Self>::u64_unlabeled_opt(self)
        }
        fn u64_optional_opt<'this>(&'this self) -> Option<u64> {
            <Self>::u64_optional_opt(self)
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<
            'this,
            ::puroro::internal::NoAllocBumpVec<u64>,
            u64,
            u64,
        >;

        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.u64_repeated)
        }
        fn s32_unlabeled_opt<'this>(&'this self) -> Option<i32> {
            <Self>::s32_unlabeled_opt(self)
        }
        fn s32_optional_opt<'this>(&'this self) -> Option<i32> {
            <Self>::s32_optional_opt(self)
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<
            'this,
            ::puroro::internal::NoAllocBumpVec<i32>,
            i32,
            i32,
        >;

        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.s32_repeated)
        }
        fn s64_unlabeled_opt<'this>(&'this self) -> Option<i64> {
            <Self>::s64_unlabeled_opt(self)
        }
        fn s64_optional_opt<'this>(&'this self) -> Option<i64> {
            <Self>::s64_optional_opt(self)
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<
            'this,
            ::puroro::internal::NoAllocBumpVec<i64>,
            i64,
            i64,
        >;

        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.s64_repeated)
        }
        fn fixed32_unlabeled_opt<'this>(&'this self) -> Option<u32> {
            <Self>::fixed32_unlabeled_opt(self)
        }
        fn fixed32_optional_opt<'this>(&'this self) -> Option<u32> {
            <Self>::fixed32_optional_opt(self)
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<
            'this,
            ::puroro::internal::NoAllocBumpVec<u32>,
            u32,
            u32,
        >;

        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.fixed32_repeated)
        }
        fn fixed64_unlabeled_opt<'this>(&'this self) -> Option<u64> {
            <Self>::fixed64_unlabeled_opt(self)
        }
        fn fixed64_optional_opt<'this>(&'this self) -> Option<u64> {
            <Self>::fixed64_optional_opt(self)
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<
            'this,
            ::puroro::internal::NoAllocBumpVec<u64>,
            u64,
            u64,
        >;

        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.fixed64_repeated)
        }
        fn sfixed32_unlabeled_opt<'this>(&'this self) -> Option<i32> {
            <Self>::sfixed32_unlabeled_opt(self)
        }
        fn sfixed32_optional_opt<'this>(&'this self) -> Option<i32> {
            <Self>::sfixed32_optional_opt(self)
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<
            'this,
            ::puroro::internal::NoAllocBumpVec<i32>,
            i32,
            i32,
        >;

        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.sfixed32_repeated)
        }
        fn sfixed64_unlabeled_opt<'this>(&'this self) -> Option<i64> {
            <Self>::sfixed64_unlabeled_opt(self)
        }
        fn sfixed64_optional_opt<'this>(&'this self) -> Option<i64> {
            <Self>::sfixed64_optional_opt(self)
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<
            'this,
            ::puroro::internal::NoAllocBumpVec<i64>,
            i64,
            i64,
        >;

        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.sfixed64_repeated)
        }
        fn f64_unlabeled_opt<'this>(&'this self) -> Option<f64> {
            <Self>::f64_unlabeled_opt(self)
        }
        fn f64_optional_opt<'this>(&'this self) -> Option<f64> {
            <Self>::f64_optional_opt(self)
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<
            'this,
            ::puroro::internal::NoAllocBumpVec<f64>,
            f64,
            f64,
        >;

        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.f64_repeated)
        }
    }

    impl<'bump> ::puroro::internal::de::DeserMessageFromBytesIter for MsgBumpalo<'bump> {
        fn deser_field<'this, I>(
            &'this mut self,
            field_number: i32,
            data: ::puroro::internal::types::FieldData<
                &mut ::puroro::internal::de::from_iter::ScopedIter<I>,
            >,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro::internal::impls::bumpalo::de::DeserFieldFromBytesIter;
            match field_number {
            1 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::Int32
                >::deser_field(&mut self.i32_unlabeled, data, self._bump)
            }
            2 => {
                self._bitfield.set(0, true);
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Int32
                >::deser_field(&mut self.i32_optional, data, self._bump)
            }
            3 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Int32
                >::deser_field(&mut self.i32_repeated, data, self._bump)
            }
            11 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::Float
                >::deser_field(&mut self.float_unlabeled, data, self._bump)
            }
            12 => {
                self._bitfield.set(1, true);
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Float
                >::deser_field(&mut self.float_optional, data, self._bump)
            }
            13 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Float
                >::deser_field(&mut self.float_repeated, data, self._bump)
            }
            21 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::Bytes
                >::deser_field(&mut self.bytes_unlabeled, data, self._bump)
            }
            22 => {
                self._bitfield.set(2, true);
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Bytes
                >::deser_field(&mut self.bytes_optional, data, self._bump)
            }
            23 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Bytes
                >::deser_field(&mut self.bytes_repeated, data, self._bump)
            }
            31 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::String
                >::deser_field(&mut self.string_unlabeled, data, self._bump)
            }
            32 => {
                self._bitfield.set(3, true);
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::String
                >::deser_field(&mut self.string_optional, data, self._bump)
            }
            33 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::String
                >::deser_field(&mut self.string_repeated, data, self._bump)
            }
            41 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::Enum3<self::_puroro_root::full_coverage3::Enum>
                >::deser_field(&mut self.enum_unlabeled, data, self._bump)
            }
            42 => {
                self._bitfield.set(4, true);
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Enum3<self::_puroro_root::full_coverage3::Enum>
                >::deser_field(&mut self.enum_optional, data, self._bump)
            }
            43 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Enum3<self::_puroro_root::full_coverage3::Enum>
                >::deser_field(&mut self.enum_repeated, data, self._bump)
            }
            51 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::Message<::puroro::internal::NoAllocBumpBox<self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgBumpalo<'bump>>>
                >::deser_field(&mut self.submsg_unlabeled, data, self._bump)
            }
            52 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Message<::puroro::internal::NoAllocBumpBox<self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgBumpalo<'bump>>>
                >::deser_field(&mut self.submsg_optional, data, self._bump)
            }
            53 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgBumpalo<'bump>>
                >::deser_field(&mut self.submsg_repeated, data, self._bump)
            }
            101 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::Int64
                >::deser_field(&mut self.i64_unlabeled, data, self._bump)
            }
            102 => {
                self._bitfield.set(5, true);
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Int64
                >::deser_field(&mut self.i64_optional, data, self._bump)
            }
            103 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Int64
                >::deser_field(&mut self.i64_repeated, data, self._bump)
            }
            111 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::UInt32
                >::deser_field(&mut self.u32_unlabeled, data, self._bump)
            }
            112 => {
                self._bitfield.set(6, true);
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::UInt32
                >::deser_field(&mut self.u32_optional, data, self._bump)
            }
            113 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::UInt32
                >::deser_field(&mut self.u32_repeated, data, self._bump)
            }
            121 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::UInt64
                >::deser_field(&mut self.u64_unlabeled, data, self._bump)
            }
            122 => {
                self._bitfield.set(7, true);
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::UInt64
                >::deser_field(&mut self.u64_optional, data, self._bump)
            }
            123 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::UInt64
                >::deser_field(&mut self.u64_repeated, data, self._bump)
            }
            131 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::SInt32
                >::deser_field(&mut self.s32_unlabeled, data, self._bump)
            }
            132 => {
                self._bitfield.set(8, true);
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::SInt32
                >::deser_field(&mut self.s32_optional, data, self._bump)
            }
            133 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::SInt32
                >::deser_field(&mut self.s32_repeated, data, self._bump)
            }
            141 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::SInt64
                >::deser_field(&mut self.s64_unlabeled, data, self._bump)
            }
            142 => {
                self._bitfield.set(9, true);
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::SInt64
                >::deser_field(&mut self.s64_optional, data, self._bump)
            }
            143 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::SInt64
                >::deser_field(&mut self.s64_repeated, data, self._bump)
            }
            151 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::Fixed32
                >::deser_field(&mut self.fixed32_unlabeled, data, self._bump)
            }
            152 => {
                self._bitfield.set(10, true);
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Fixed32
                >::deser_field(&mut self.fixed32_optional, data, self._bump)
            }
            153 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Fixed32
                >::deser_field(&mut self.fixed32_repeated, data, self._bump)
            }
            161 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::Fixed64
                >::deser_field(&mut self.fixed64_unlabeled, data, self._bump)
            }
            162 => {
                self._bitfield.set(11, true);
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Fixed64
                >::deser_field(&mut self.fixed64_optional, data, self._bump)
            }
            163 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Fixed64
                >::deser_field(&mut self.fixed64_repeated, data, self._bump)
            }
            171 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::SFixed32
                >::deser_field(&mut self.sfixed32_unlabeled, data, self._bump)
            }
            172 => {
                self._bitfield.set(12, true);
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::SFixed32
                >::deser_field(&mut self.sfixed32_optional, data, self._bump)
            }
            173 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::SFixed32
                >::deser_field(&mut self.sfixed32_repeated, data, self._bump)
            }
            181 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::SFixed64
                >::deser_field(&mut self.sfixed64_unlabeled, data, self._bump)
            }
            182 => {
                self._bitfield.set(13, true);
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::SFixed64
                >::deser_field(&mut self.sfixed64_optional, data, self._bump)
            }
            183 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::SFixed64
                >::deser_field(&mut self.sfixed64_repeated, data, self._bump)
            }
            191 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::Double
                >::deser_field(&mut self.f64_unlabeled, data, self._bump)
            }
            192 => {
                self._bitfield.set(14, true);
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Double
                >::deser_field(&mut self.f64_optional, data, self._bump)
            }
            193 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Double
                >::deser_field(&mut self.f64_repeated, data, self._bump)
            }

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl<'bump> ::puroro::internal::se::SerMessageToIoWrite for MsgBumpalo<'bump>
    where
        Self: super::_puroro_traits::MsgTrait,
        for<'a> <Self as super::_puroro_traits::MsgTrait>::SubmsgUnlabeledMessageType<'a>:
            ::puroro::internal::se::SerMessageToIoWrite,
        for<'a> <Self as super::_puroro_traits::MsgTrait>::SubmsgOptionalMessageType<'a>:
            ::puroro::internal::se::SerMessageToIoWrite,
        for<'a> <Self as super::_puroro_traits::MsgTrait>::SubmsgRepeatedMessageType<'a>:
            ::puroro::internal::se::SerMessageToIoWrite,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::i32_unlabeled_opt(self),
                1,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::i32_optional_opt(self),
                2,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::i32_repeated(self),
                3,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Float,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::float_unlabeled_opt(self),
                11,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Float,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::float_optional_opt(self),
                12,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Float,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::float_repeated(self),
                13,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Bytes,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::bytes_unlabeled_opt(self),
                21,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bytes,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::bytes_optional_opt(self),
                22,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Bytes,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::bytes_repeated(self),
                23,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::string_unlabeled_opt(self),
                31,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::string_optional_opt(self),
                32,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::string_repeated(self),
                33,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Enum3<self::_puroro_root::full_coverage3::Enum>,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::enum_unlabeled_opt(self),
                41,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Enum3<self::_puroro_root::full_coverage3::Enum>,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::enum_optional_opt(self),
                42,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Enum3<self::_puroro_root::full_coverage3::Enum>,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::enum_repeated(self),
                43,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Message<
                    <Self as super::_puroro_traits::MsgTrait>::SubmsgUnlabeledMessageType<'_>,
                >,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::submsg_unlabeled_opt(self),
                51,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    <Self as super::_puroro_traits::MsgTrait>::SubmsgOptionalMessageType<'_>,
                >,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::submsg_optional_opt(self),
                52,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    <Self as super::_puroro_traits::MsgTrait>::SubmsgRepeatedMessageType<'_>,
                >,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::submsg_repeated(self),
                53,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Int64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::i64_unlabeled_opt(self),
                101,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::i64_optional_opt(self),
                102,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Int64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::i64_repeated(self),
                103,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::UInt32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::u32_unlabeled_opt(self),
                111,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::UInt32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::u32_optional_opt(self),
                112,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::UInt32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::u32_repeated(self),
                113,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::UInt64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::u64_unlabeled_opt(self),
                121,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::UInt64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::u64_optional_opt(self),
                122,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::UInt64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::u64_repeated(self),
                123,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::SInt32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::s32_unlabeled_opt(self),
                131,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::SInt32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::s32_optional_opt(self),
                132,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::SInt32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::s32_repeated(self),
                133,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::SInt64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::s64_unlabeled_opt(self),
                141,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::SInt64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::s64_optional_opt(self),
                142,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::SInt64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::s64_repeated(self),
                143,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Fixed32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::fixed32_unlabeled_opt(self),
                151,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Fixed32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::fixed32_optional_opt(self),
                152,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Fixed32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::fixed32_repeated(self),
                153,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Fixed64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::fixed64_unlabeled_opt(self),
                161,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Fixed64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::fixed64_optional_opt(self),
                162,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Fixed64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::fixed64_repeated(self),
                163,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::SFixed32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::sfixed32_unlabeled_opt(self),
                171,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::SFixed32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::sfixed32_optional_opt(self),
                172,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::SFixed32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::sfixed32_repeated(self),
                173,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::SFixed64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::sfixed64_unlabeled_opt(self),
                181,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::SFixed64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::sfixed64_optional_opt(self),
                182,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::SFixed64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::sfixed64_repeated(self),
                183,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Double,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::f64_unlabeled_opt(self),
                191,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Double,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::f64_optional_opt(self),
                192,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Double,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::f64_repeated(self),
                193,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }
    pub struct MsgBuilder<T>(T);

    impl<T> MsgBuilder<T>
    where
        T: MsgTrait,
    {
        pub fn append_i32_unlabeled<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField1<ScalarType>)>
        where
            ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        {
            MsgBuilder((
                self.0,
                MsgSingleField1 {
                    i32_unlabeled: value,
                },
            ))
        }

        pub fn append_i32_optional<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField2<ScalarType>)>
        where
            ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        {
            MsgBuilder((
                self.0,
                MsgSingleField2 {
                    i32_optional: value,
                },
            ))
        }

        pub fn append_i32_repeated<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> MsgBuilder<(T, MsgSingleField3<ScalarType, RepeatedType>)>
        where
            ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
            for<'a> &'a RepeatedType:
                ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            MsgBuilder((
                self.0,
                MsgSingleField3 {
                    i32_repeated: value,
                },
            ))
        }

        pub fn append_float_unlabeled<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField11<ScalarType>)>
        where
            ScalarType: ::std::convert::Into<f32> + ::std::clone::Clone,
        {
            MsgBuilder((
                self.0,
                MsgSingleField11 {
                    float_unlabeled: value,
                },
            ))
        }

        pub fn append_float_optional<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField12<ScalarType>)>
        where
            ScalarType: ::std::convert::Into<f32> + ::std::clone::Clone,
        {
            MsgBuilder((
                self.0,
                MsgSingleField12 {
                    float_optional: value,
                },
            ))
        }

        pub fn append_float_repeated<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> MsgBuilder<(T, MsgSingleField13<ScalarType, RepeatedType>)>
        where
            ScalarType: ::std::convert::Into<f32> + ::std::clone::Clone,
            for<'a> &'a RepeatedType:
                ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            MsgBuilder((
                self.0,
                MsgSingleField13 {
                    float_repeated: value,
                },
            ))
        }

        pub fn append_bytes_unlabeled<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField21<ScalarType>)>
        where
            ScalarType: ::std::convert::AsRef<[u8]>,
        {
            MsgBuilder((
                self.0,
                MsgSingleField21 {
                    bytes_unlabeled: value,
                },
            ))
        }

        pub fn append_bytes_optional<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField22<ScalarType>)>
        where
            ScalarType: ::std::convert::AsRef<[u8]>,
        {
            MsgBuilder((
                self.0,
                MsgSingleField22 {
                    bytes_optional: value,
                },
            ))
        }

        pub fn append_bytes_repeated<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> MsgBuilder<(T, MsgSingleField23<ScalarType, RepeatedType>)>
        where
            ScalarType: ::std::convert::AsRef<[u8]>,
            for<'a> &'a RepeatedType:
                ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            MsgBuilder((
                self.0,
                MsgSingleField23 {
                    bytes_repeated: value,
                },
            ))
        }

        pub fn append_string_unlabeled<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField31<ScalarType>)>
        where
            ScalarType: ::std::convert::AsRef<str>,
        {
            MsgBuilder((
                self.0,
                MsgSingleField31 {
                    string_unlabeled: value,
                },
            ))
        }

        pub fn append_string_optional<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField32<ScalarType>)>
        where
            ScalarType: ::std::convert::AsRef<str>,
        {
            MsgBuilder((
                self.0,
                MsgSingleField32 {
                    string_optional: value,
                },
            ))
        }

        pub fn append_string_repeated<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> MsgBuilder<(T, MsgSingleField33<ScalarType, RepeatedType>)>
        where
            ScalarType: ::std::convert::AsRef<str>,
            for<'a> &'a RepeatedType:
                ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            MsgBuilder((
                self.0,
                MsgSingleField33 {
                    string_repeated: value,
                },
            ))
        }

        pub fn append_enum_unlabeled<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField41<ScalarType>)>
        where
            ScalarType: ::std::convert::Into<self::_puroro_root::full_coverage3::Enum>
                + ::std::clone::Clone,
        {
            MsgBuilder((
                self.0,
                MsgSingleField41 {
                    enum_unlabeled: value,
                },
            ))
        }

        pub fn append_enum_optional<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField42<ScalarType>)>
        where
            ScalarType: ::std::convert::Into<self::_puroro_root::full_coverage3::Enum>
                + ::std::clone::Clone,
        {
            MsgBuilder((
                self.0,
                MsgSingleField42 {
                    enum_optional: value,
                },
            ))
        }

        pub fn append_enum_repeated<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> MsgBuilder<(T, MsgSingleField43<ScalarType, RepeatedType>)>
        where
            ScalarType: ::std::convert::Into<self::_puroro_root::full_coverage3::Enum>
                + ::std::clone::Clone,
            for<'a> &'a RepeatedType:
                ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            MsgBuilder((
                self.0,
                MsgSingleField43 {
                    enum_repeated: value,
                },
            ))
        }

    pub fn append_submsg_unlabeled<ScalarType>(self, value: ScalarType)
        -> MsgBuilder<(T, MsgSingleField51<ScalarType>)>
where

ScalarType:
    self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait,
    {
            MsgBuilder((
                self.0,
                MsgSingleField51 {
                    submsg_unlabeled: value,
                },
            ))
        }

    pub fn append_submsg_optional<ScalarType>(self, value: ScalarType)
        -> MsgBuilder<(T, MsgSingleField52<ScalarType>)>
where

ScalarType:
    self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait,
    {
            MsgBuilder((
                self.0,
                MsgSingleField52 {
                    submsg_optional: value,
                },
            ))
        }

    pub fn append_submsg_repeated<ScalarType, RepeatedType>(self, value: RepeatedType)
        -> MsgBuilder<(T, MsgSingleField53<ScalarType, RepeatedType>)>
where

ScalarType:
    self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait,
for <'a> &'a RepeatedType: ::puroro::RepeatedField<'a> +
    ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
            MsgBuilder((
                self.0,
                MsgSingleField53 {
                    submsg_repeated: value,
                },
            ))
        }

        pub fn append_i64_unlabeled<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField101<ScalarType>)>
        where
            ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
        {
            MsgBuilder((
                self.0,
                MsgSingleField101 {
                    i64_unlabeled: value,
                },
            ))
        }

        pub fn append_i64_optional<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField102<ScalarType>)>
        where
            ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
        {
            MsgBuilder((
                self.0,
                MsgSingleField102 {
                    i64_optional: value,
                },
            ))
        }

        pub fn append_i64_repeated<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> MsgBuilder<(T, MsgSingleField103<ScalarType, RepeatedType>)>
        where
            ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
            for<'a> &'a RepeatedType:
                ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            MsgBuilder((
                self.0,
                MsgSingleField103 {
                    i64_repeated: value,
                },
            ))
        }

        pub fn append_u32_unlabeled<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField111<ScalarType>)>
        where
            ScalarType: ::std::convert::Into<u32> + ::std::clone::Clone,
        {
            MsgBuilder((
                self.0,
                MsgSingleField111 {
                    u32_unlabeled: value,
                },
            ))
        }

        pub fn append_u32_optional<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField112<ScalarType>)>
        where
            ScalarType: ::std::convert::Into<u32> + ::std::clone::Clone,
        {
            MsgBuilder((
                self.0,
                MsgSingleField112 {
                    u32_optional: value,
                },
            ))
        }

        pub fn append_u32_repeated<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> MsgBuilder<(T, MsgSingleField113<ScalarType, RepeatedType>)>
        where
            ScalarType: ::std::convert::Into<u32> + ::std::clone::Clone,
            for<'a> &'a RepeatedType:
                ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            MsgBuilder((
                self.0,
                MsgSingleField113 {
                    u32_repeated: value,
                },
            ))
        }

        pub fn append_u64_unlabeled<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField121<ScalarType>)>
        where
            ScalarType: ::std::convert::Into<u64> + ::std::clone::Clone,
        {
            MsgBuilder((
                self.0,
                MsgSingleField121 {
                    u64_unlabeled: value,
                },
            ))
        }

        pub fn append_u64_optional<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField122<ScalarType>)>
        where
            ScalarType: ::std::convert::Into<u64> + ::std::clone::Clone,
        {
            MsgBuilder((
                self.0,
                MsgSingleField122 {
                    u64_optional: value,
                },
            ))
        }

        pub fn append_u64_repeated<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> MsgBuilder<(T, MsgSingleField123<ScalarType, RepeatedType>)>
        where
            ScalarType: ::std::convert::Into<u64> + ::std::clone::Clone,
            for<'a> &'a RepeatedType:
                ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            MsgBuilder((
                self.0,
                MsgSingleField123 {
                    u64_repeated: value,
                },
            ))
        }

        pub fn append_s32_unlabeled<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField131<ScalarType>)>
        where
            ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        {
            MsgBuilder((
                self.0,
                MsgSingleField131 {
                    s32_unlabeled: value,
                },
            ))
        }

        pub fn append_s32_optional<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField132<ScalarType>)>
        where
            ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        {
            MsgBuilder((
                self.0,
                MsgSingleField132 {
                    s32_optional: value,
                },
            ))
        }

        pub fn append_s32_repeated<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> MsgBuilder<(T, MsgSingleField133<ScalarType, RepeatedType>)>
        where
            ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
            for<'a> &'a RepeatedType:
                ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            MsgBuilder((
                self.0,
                MsgSingleField133 {
                    s32_repeated: value,
                },
            ))
        }

        pub fn append_s64_unlabeled<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField141<ScalarType>)>
        where
            ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
        {
            MsgBuilder((
                self.0,
                MsgSingleField141 {
                    s64_unlabeled: value,
                },
            ))
        }

        pub fn append_s64_optional<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField142<ScalarType>)>
        where
            ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
        {
            MsgBuilder((
                self.0,
                MsgSingleField142 {
                    s64_optional: value,
                },
            ))
        }

        pub fn append_s64_repeated<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> MsgBuilder<(T, MsgSingleField143<ScalarType, RepeatedType>)>
        where
            ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
            for<'a> &'a RepeatedType:
                ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            MsgBuilder((
                self.0,
                MsgSingleField143 {
                    s64_repeated: value,
                },
            ))
        }

        pub fn append_fixed32_unlabeled<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField151<ScalarType>)>
        where
            ScalarType: ::std::convert::Into<u32> + ::std::clone::Clone,
        {
            MsgBuilder((
                self.0,
                MsgSingleField151 {
                    fixed32_unlabeled: value,
                },
            ))
        }

        pub fn append_fixed32_optional<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField152<ScalarType>)>
        where
            ScalarType: ::std::convert::Into<u32> + ::std::clone::Clone,
        {
            MsgBuilder((
                self.0,
                MsgSingleField152 {
                    fixed32_optional: value,
                },
            ))
        }

        pub fn append_fixed32_repeated<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> MsgBuilder<(T, MsgSingleField153<ScalarType, RepeatedType>)>
        where
            ScalarType: ::std::convert::Into<u32> + ::std::clone::Clone,
            for<'a> &'a RepeatedType:
                ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            MsgBuilder((
                self.0,
                MsgSingleField153 {
                    fixed32_repeated: value,
                },
            ))
        }

        pub fn append_fixed64_unlabeled<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField161<ScalarType>)>
        where
            ScalarType: ::std::convert::Into<u64> + ::std::clone::Clone,
        {
            MsgBuilder((
                self.0,
                MsgSingleField161 {
                    fixed64_unlabeled: value,
                },
            ))
        }

        pub fn append_fixed64_optional<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField162<ScalarType>)>
        where
            ScalarType: ::std::convert::Into<u64> + ::std::clone::Clone,
        {
            MsgBuilder((
                self.0,
                MsgSingleField162 {
                    fixed64_optional: value,
                },
            ))
        }

        pub fn append_fixed64_repeated<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> MsgBuilder<(T, MsgSingleField163<ScalarType, RepeatedType>)>
        where
            ScalarType: ::std::convert::Into<u64> + ::std::clone::Clone,
            for<'a> &'a RepeatedType:
                ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            MsgBuilder((
                self.0,
                MsgSingleField163 {
                    fixed64_repeated: value,
                },
            ))
        }

        pub fn append_sfixed32_unlabeled<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField171<ScalarType>)>
        where
            ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        {
            MsgBuilder((
                self.0,
                MsgSingleField171 {
                    sfixed32_unlabeled: value,
                },
            ))
        }

        pub fn append_sfixed32_optional<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField172<ScalarType>)>
        where
            ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        {
            MsgBuilder((
                self.0,
                MsgSingleField172 {
                    sfixed32_optional: value,
                },
            ))
        }

        pub fn append_sfixed32_repeated<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> MsgBuilder<(T, MsgSingleField173<ScalarType, RepeatedType>)>
        where
            ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
            for<'a> &'a RepeatedType:
                ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            MsgBuilder((
                self.0,
                MsgSingleField173 {
                    sfixed32_repeated: value,
                },
            ))
        }

        pub fn append_sfixed64_unlabeled<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField181<ScalarType>)>
        where
            ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
        {
            MsgBuilder((
                self.0,
                MsgSingleField181 {
                    sfixed64_unlabeled: value,
                },
            ))
        }

        pub fn append_sfixed64_optional<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField182<ScalarType>)>
        where
            ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
        {
            MsgBuilder((
                self.0,
                MsgSingleField182 {
                    sfixed64_optional: value,
                },
            ))
        }

        pub fn append_sfixed64_repeated<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> MsgBuilder<(T, MsgSingleField183<ScalarType, RepeatedType>)>
        where
            ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
            for<'a> &'a RepeatedType:
                ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            MsgBuilder((
                self.0,
                MsgSingleField183 {
                    sfixed64_repeated: value,
                },
            ))
        }

        pub fn append_f64_unlabeled<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField191<ScalarType>)>
        where
            ScalarType: ::std::convert::Into<f64> + ::std::clone::Clone,
        {
            MsgBuilder((
                self.0,
                MsgSingleField191 {
                    f64_unlabeled: value,
                },
            ))
        }

        pub fn append_f64_optional<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField192<ScalarType>)>
        where
            ScalarType: ::std::convert::Into<f64> + ::std::clone::Clone,
        {
            MsgBuilder((
                self.0,
                MsgSingleField192 {
                    f64_optional: value,
                },
            ))
        }

        pub fn append_f64_repeated<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> MsgBuilder<(T, MsgSingleField193<ScalarType, RepeatedType>)>
        where
            ScalarType: ::std::convert::Into<f64> + ::std::clone::Clone,
            for<'a> &'a RepeatedType:
                ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            MsgBuilder((
                self.0,
                MsgSingleField193 {
                    f64_repeated: value,
                },
            ))
        }

        pub fn build(self) -> T {
            self.0
        }
    }

    impl MsgBuilder<()> {
        pub fn new() -> Self {
            Self(())
        }
    }
}
pub use _puroro_traits::*;
pub mod _puroro_traits {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }

    pub trait MsgTrait {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            self.i32_unlabeled_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_i32_unlabeled<'this>(&'this self) -> bool {
            self.i32_unlabeled_opt().is_some()
        }
        fn i32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::None
        }
        fn i32_optional<'this>(&'this self) -> i32 {
            self.i32_optional_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_i32_optional<'this>(&'this self) -> bool {
            self.i32_optional_opt().is_some()
        }
        fn i32_optional_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::None
        }

        type I32RepeatedRepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = i32>
        where
            Self: 'this;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this>;
        fn float_unlabeled<'this>(&'this self) -> f32 {
            self.float_unlabeled_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_float_unlabeled<'this>(&'this self) -> bool {
            self.float_unlabeled_opt().is_some()
        }
        fn float_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<f32> {
            ::std::option::Option::None
        }
        fn float_optional<'this>(&'this self) -> f32 {
            self.float_optional_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_float_optional<'this>(&'this self) -> bool {
            self.float_optional_opt().is_some()
        }
        fn float_optional_opt<'this>(&'this self) -> ::std::option::Option<f32> {
            ::std::option::Option::None
        }

        type FloatRepeatedRepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = f32>
        where
            Self: 'this;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this>;
        fn bytes_unlabeled<'this>(&'this self) -> &'this [u8] {
            self.bytes_unlabeled_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_bytes_unlabeled<'this>(&'this self) -> bool {
            self.bytes_unlabeled_opt().is_some()
        }
        fn bytes_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<&'this [u8]> {
            ::std::option::Option::None
        }
        fn bytes_optional<'this>(&'this self) -> &'this [u8] {
            self.bytes_optional_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_bytes_optional<'this>(&'this self) -> bool {
            self.bytes_optional_opt().is_some()
        }
        fn bytes_optional_opt<'this>(&'this self) -> ::std::option::Option<&'this [u8]> {
            ::std::option::Option::None
        }

        type BytesRepeatedRepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = &'this [u8]>
        where
            Self: 'this;
        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this>;
        fn string_unlabeled<'this>(&'this self) -> &'this str {
            self.string_unlabeled_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_string_unlabeled<'this>(&'this self) -> bool {
            self.string_unlabeled_opt().is_some()
        }
        fn string_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::None
        }
        fn string_optional<'this>(&'this self) -> &'this str {
            self.string_optional_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_string_optional<'this>(&'this self) -> bool {
            self.string_optional_opt().is_some()
        }
        fn string_optional_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::None
        }

        type StringRepeatedRepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = &'this str>
        where
            Self: 'this;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this>;
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::full_coverage3::Enum {
            self.enum_unlabeled_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_enum_unlabeled<'this>(&'this self) -> bool {
            self.enum_unlabeled_opt().is_some()
        }
        fn enum_unlabeled_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::full_coverage3::Enum> {
            ::std::option::Option::None
        }
        fn enum_optional<'this>(&'this self) -> self::_puroro_root::full_coverage3::Enum {
            self.enum_optional_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_enum_optional<'this>(&'this self) -> bool {
            self.enum_optional_opt().is_some()
        }
        fn enum_optional_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::full_coverage3::Enum> {
            ::std::option::Option::None
        }

        type EnumRepeatedRepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = self::_puroro_root::full_coverage3::Enum>
        where
            Self: 'this;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this>;
        type SubmsgUnlabeledMessageType<'this>: self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait
            where Self: 'this;
        fn submsg_unlabeled<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::SubmsgUnlabeledMessageType<'this>> {
            self.submsg_unlabeled_opt()
        }
        fn has_submsg_unlabeled<'this>(&'this self) -> bool {
            self.submsg_unlabeled_opt().is_some()
        }
        fn submsg_unlabeled_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::SubmsgUnlabeledMessageType<'this>> {
            ::std::option::Option::None
        }
        type SubmsgOptionalMessageType<'this>: self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait
            where Self: 'this;
        fn submsg_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::SubmsgOptionalMessageType<'this>> {
            self.submsg_optional_opt()
        }
        fn has_submsg_optional<'this>(&'this self) -> bool {
            self.submsg_optional_opt().is_some()
        }
        fn submsg_optional_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::SubmsgOptionalMessageType<'this>> {
            ::std::option::Option::None
        }
        type SubmsgRepeatedMessageType<'this>: self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait
            where Self: 'this;

        type SubmsgRepeatedRepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::SubmsgRepeatedMessageType<'this>>
        where
            Self: 'this;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this>;
        fn i64_unlabeled<'this>(&'this self) -> i64 {
            self.i64_unlabeled_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_i64_unlabeled<'this>(&'this self) -> bool {
            self.i64_unlabeled_opt().is_some()
        }
        fn i64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            ::std::option::Option::None
        }
        fn i64_optional<'this>(&'this self) -> i64 {
            self.i64_optional_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_i64_optional<'this>(&'this self) -> bool {
            self.i64_optional_opt().is_some()
        }
        fn i64_optional_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            ::std::option::Option::None
        }

        type I64RepeatedRepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = i64>
        where
            Self: 'this;
        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this>;
        fn u32_unlabeled<'this>(&'this self) -> u32 {
            self.u32_unlabeled_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_u32_unlabeled<'this>(&'this self) -> bool {
            self.u32_unlabeled_opt().is_some()
        }
        fn u32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<u32> {
            ::std::option::Option::None
        }
        fn u32_optional<'this>(&'this self) -> u32 {
            self.u32_optional_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_u32_optional<'this>(&'this self) -> bool {
            self.u32_optional_opt().is_some()
        }
        fn u32_optional_opt<'this>(&'this self) -> ::std::option::Option<u32> {
            ::std::option::Option::None
        }

        type U32RepeatedRepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = u32>
        where
            Self: 'this;
        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this>;
        fn u64_unlabeled<'this>(&'this self) -> u64 {
            self.u64_unlabeled_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_u64_unlabeled<'this>(&'this self) -> bool {
            self.u64_unlabeled_opt().is_some()
        }
        fn u64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<u64> {
            ::std::option::Option::None
        }
        fn u64_optional<'this>(&'this self) -> u64 {
            self.u64_optional_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_u64_optional<'this>(&'this self) -> bool {
            self.u64_optional_opt().is_some()
        }
        fn u64_optional_opt<'this>(&'this self) -> ::std::option::Option<u64> {
            ::std::option::Option::None
        }

        type U64RepeatedRepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = u64>
        where
            Self: 'this;
        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this>;
        fn s32_unlabeled<'this>(&'this self) -> i32 {
            self.s32_unlabeled_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_s32_unlabeled<'this>(&'this self) -> bool {
            self.s32_unlabeled_opt().is_some()
        }
        fn s32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::None
        }
        fn s32_optional<'this>(&'this self) -> i32 {
            self.s32_optional_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_s32_optional<'this>(&'this self) -> bool {
            self.s32_optional_opt().is_some()
        }
        fn s32_optional_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::None
        }

        type S32RepeatedRepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = i32>
        where
            Self: 'this;
        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this>;
        fn s64_unlabeled<'this>(&'this self) -> i64 {
            self.s64_unlabeled_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_s64_unlabeled<'this>(&'this self) -> bool {
            self.s64_unlabeled_opt().is_some()
        }
        fn s64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            ::std::option::Option::None
        }
        fn s64_optional<'this>(&'this self) -> i64 {
            self.s64_optional_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_s64_optional<'this>(&'this self) -> bool {
            self.s64_optional_opt().is_some()
        }
        fn s64_optional_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            ::std::option::Option::None
        }

        type S64RepeatedRepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = i64>
        where
            Self: 'this;
        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this>;
        fn fixed32_unlabeled<'this>(&'this self) -> u32 {
            self.fixed32_unlabeled_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_fixed32_unlabeled<'this>(&'this self) -> bool {
            self.fixed32_unlabeled_opt().is_some()
        }
        fn fixed32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<u32> {
            ::std::option::Option::None
        }
        fn fixed32_optional<'this>(&'this self) -> u32 {
            self.fixed32_optional_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_fixed32_optional<'this>(&'this self) -> bool {
            self.fixed32_optional_opt().is_some()
        }
        fn fixed32_optional_opt<'this>(&'this self) -> ::std::option::Option<u32> {
            ::std::option::Option::None
        }

        type Fixed32RepeatedRepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = u32>
        where
            Self: 'this;
        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this>;
        fn fixed64_unlabeled<'this>(&'this self) -> u64 {
            self.fixed64_unlabeled_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_fixed64_unlabeled<'this>(&'this self) -> bool {
            self.fixed64_unlabeled_opt().is_some()
        }
        fn fixed64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<u64> {
            ::std::option::Option::None
        }
        fn fixed64_optional<'this>(&'this self) -> u64 {
            self.fixed64_optional_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_fixed64_optional<'this>(&'this self) -> bool {
            self.fixed64_optional_opt().is_some()
        }
        fn fixed64_optional_opt<'this>(&'this self) -> ::std::option::Option<u64> {
            ::std::option::Option::None
        }

        type Fixed64RepeatedRepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = u64>
        where
            Self: 'this;
        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this>;
        fn sfixed32_unlabeled<'this>(&'this self) -> i32 {
            self.sfixed32_unlabeled_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_sfixed32_unlabeled<'this>(&'this self) -> bool {
            self.sfixed32_unlabeled_opt().is_some()
        }
        fn sfixed32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::None
        }
        fn sfixed32_optional<'this>(&'this self) -> i32 {
            self.sfixed32_optional_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_sfixed32_optional<'this>(&'this self) -> bool {
            self.sfixed32_optional_opt().is_some()
        }
        fn sfixed32_optional_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::None
        }

        type Sfixed32RepeatedRepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = i32>
        where
            Self: 'this;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this>;
        fn sfixed64_unlabeled<'this>(&'this self) -> i64 {
            self.sfixed64_unlabeled_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_sfixed64_unlabeled<'this>(&'this self) -> bool {
            self.sfixed64_unlabeled_opt().is_some()
        }
        fn sfixed64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            ::std::option::Option::None
        }
        fn sfixed64_optional<'this>(&'this self) -> i64 {
            self.sfixed64_optional_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_sfixed64_optional<'this>(&'this self) -> bool {
            self.sfixed64_optional_opt().is_some()
        }
        fn sfixed64_optional_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            ::std::option::Option::None
        }

        type Sfixed64RepeatedRepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = i64>
        where
            Self: 'this;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this>;
        fn f64_unlabeled<'this>(&'this self) -> f64 {
            self.f64_unlabeled_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_f64_unlabeled<'this>(&'this self) -> bool {
            self.f64_unlabeled_opt().is_some()
        }
        fn f64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<f64> {
            ::std::option::Option::None
        }
        fn f64_optional<'this>(&'this self) -> f64 {
            self.f64_optional_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_f64_optional<'this>(&'this self) -> bool {
            self.f64_optional_opt().is_some()
        }
        fn f64_optional_opt<'this>(&'this self) -> ::std::option::Option<f64> {
            ::std::option::Option::None
        }

        type F64RepeatedRepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = f64>
        where
            Self: 'this;
        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this>;
    }

    macro_rules! msg_delegate {
        ($ty:ty) => {
            fn i32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).i32_unlabeled_opt()
            }
            fn i32_optional_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).i32_optional_opt()
            }

            type I32RepeatedRepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::I32RepeatedRepeatedType<'this>;
            fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
                (**self).i32_repeated()
            }
            fn float_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<f32> {
                (**self).float_unlabeled_opt()
            }
            fn float_optional_opt<'this>(&'this self) -> ::std::option::Option<f32> {
                (**self).float_optional_opt()
            }

            type FloatRepeatedRepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::FloatRepeatedRepeatedType<'this>;
            fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
                (**self).float_repeated()
            }
            fn bytes_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<&'this [u8]> {
                (**self).bytes_unlabeled_opt()
            }
            fn bytes_optional_opt<'this>(&'this self) -> ::std::option::Option<&'this [u8]> {
                (**self).bytes_optional_opt()
            }

            type BytesRepeatedRepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::BytesRepeatedRepeatedType<'this>;
            fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
                (**self).bytes_repeated()
            }
            fn string_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                (**self).string_unlabeled_opt()
            }
            fn string_optional_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                (**self).string_optional_opt()
            }

            type StringRepeatedRepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::StringRepeatedRepeatedType<'this>;
            fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
                (**self).string_repeated()
            }
            fn enum_unlabeled_opt<'this>(
                &'this self,
            ) -> ::std::option::Option<self::_puroro_root::full_coverage3::Enum> {
                (**self).enum_unlabeled_opt()
            }
            fn enum_optional_opt<'this>(
                &'this self,
            ) -> ::std::option::Option<self::_puroro_root::full_coverage3::Enum> {
                (**self).enum_optional_opt()
            }

            type EnumRepeatedRepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::EnumRepeatedRepeatedType<'this>;
            fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
                (**self).enum_repeated()
            }
            type SubmsgUnlabeledMessageType<'this>
            where
                Self: 'this,
            = <$ty>::SubmsgUnlabeledMessageType<'this>;
            fn submsg_unlabeled_opt<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::SubmsgUnlabeledMessageType<'this>> {
                (**self).submsg_unlabeled_opt()
            }
            type SubmsgOptionalMessageType<'this>
            where
                Self: 'this,
            = <$ty>::SubmsgOptionalMessageType<'this>;
            fn submsg_optional_opt<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::SubmsgOptionalMessageType<'this>> {
                (**self).submsg_optional_opt()
            }
            type SubmsgRepeatedMessageType<'this>
            where
                Self: 'this,
            = <$ty>::SubmsgRepeatedMessageType<'this>;

            type SubmsgRepeatedRepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::SubmsgRepeatedRepeatedType<'this>;
            fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
                (**self).submsg_repeated()
            }
            fn i64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i64> {
                (**self).i64_unlabeled_opt()
            }
            fn i64_optional_opt<'this>(&'this self) -> ::std::option::Option<i64> {
                (**self).i64_optional_opt()
            }

            type I64RepeatedRepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::I64RepeatedRepeatedType<'this>;
            fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
                (**self).i64_repeated()
            }
            fn u32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<u32> {
                (**self).u32_unlabeled_opt()
            }
            fn u32_optional_opt<'this>(&'this self) -> ::std::option::Option<u32> {
                (**self).u32_optional_opt()
            }

            type U32RepeatedRepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::U32RepeatedRepeatedType<'this>;
            fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
                (**self).u32_repeated()
            }
            fn u64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<u64> {
                (**self).u64_unlabeled_opt()
            }
            fn u64_optional_opt<'this>(&'this self) -> ::std::option::Option<u64> {
                (**self).u64_optional_opt()
            }

            type U64RepeatedRepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::U64RepeatedRepeatedType<'this>;
            fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
                (**self).u64_repeated()
            }
            fn s32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).s32_unlabeled_opt()
            }
            fn s32_optional_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).s32_optional_opt()
            }

            type S32RepeatedRepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::S32RepeatedRepeatedType<'this>;
            fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
                (**self).s32_repeated()
            }
            fn s64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i64> {
                (**self).s64_unlabeled_opt()
            }
            fn s64_optional_opt<'this>(&'this self) -> ::std::option::Option<i64> {
                (**self).s64_optional_opt()
            }

            type S64RepeatedRepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::S64RepeatedRepeatedType<'this>;
            fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
                (**self).s64_repeated()
            }
            fn fixed32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<u32> {
                (**self).fixed32_unlabeled_opt()
            }
            fn fixed32_optional_opt<'this>(&'this self) -> ::std::option::Option<u32> {
                (**self).fixed32_optional_opt()
            }

            type Fixed32RepeatedRepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Fixed32RepeatedRepeatedType<'this>;
            fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
                (**self).fixed32_repeated()
            }
            fn fixed64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<u64> {
                (**self).fixed64_unlabeled_opt()
            }
            fn fixed64_optional_opt<'this>(&'this self) -> ::std::option::Option<u64> {
                (**self).fixed64_optional_opt()
            }

            type Fixed64RepeatedRepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Fixed64RepeatedRepeatedType<'this>;
            fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
                (**self).fixed64_repeated()
            }
            fn sfixed32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).sfixed32_unlabeled_opt()
            }
            fn sfixed32_optional_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).sfixed32_optional_opt()
            }

            type Sfixed32RepeatedRepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Sfixed32RepeatedRepeatedType<'this>;
            fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
                (**self).sfixed32_repeated()
            }
            fn sfixed64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i64> {
                (**self).sfixed64_unlabeled_opt()
            }
            fn sfixed64_optional_opt<'this>(&'this self) -> ::std::option::Option<i64> {
                (**self).sfixed64_optional_opt()
            }

            type Sfixed64RepeatedRepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Sfixed64RepeatedRepeatedType<'this>;
            fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
                (**self).sfixed64_repeated()
            }
            fn f64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<f64> {
                (**self).f64_unlabeled_opt()
            }
            fn f64_optional_opt<'this>(&'this self) -> ::std::option::Option<f64> {
                (**self).f64_optional_opt()
            }

            type F64RepeatedRepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::F64RepeatedRepeatedType<'this>;
            fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
                (**self).f64_repeated()
            }
        };
    }

    impl<T> MsgTrait for &'_ T
    where
        T: MsgTrait,
    {
        msg_delegate!(T);
    }

    impl<T> MsgTrait for &'_ mut T
    where
        T: MsgTrait,
    {
        msg_delegate!(T);
    }

    impl<T> MsgTrait for ::std::boxed::Box<T>
    where
        T: MsgTrait,
    {
        msg_delegate!(T);
    }

    impl<'bump, T> MsgTrait for ::puroro::bumpalo::boxed::Box<'bump, T>
    where
        T: MsgTrait,
    {
        msg_delegate!(T);
    }

    impl<T> MsgTrait for ::puroro::BumpaloOwned<T>
    where
        T: MsgTrait,
    {
        msg_delegate!(T);
    }
    impl MsgTrait for () {
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = ();
        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            Self::SubmsgRepeatedMessageType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }
    impl<T, U> MsgTrait for (T, U)
    where
        T: MsgTrait,
        U: MsgTrait,
    {
        fn i32_unlabeled_opt<'this>(&'this self) -> Option<i32> {
            <U as MsgTrait>::i32_unlabeled_opt(&self.1)
                .or_else(|| <T as MsgTrait>::i32_unlabeled_opt(&self.0))
        }
        fn i32_optional_opt<'this>(&'this self) -> Option<i32> {
            <U as MsgTrait>::i32_optional_opt(&self.1)
                .or_else(|| <T as MsgTrait>::i32_optional_opt(&self.0))
        }
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::I32RepeatedRepeatedType<'this>,
            <U as MsgTrait>::I32RepeatedRepeatedType<'this>,
        >;

        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as MsgTrait>::i32_repeated(&self.0),
                <U as MsgTrait>::i32_repeated(&self.1),
            )
        }
        fn float_unlabeled_opt<'this>(&'this self) -> Option<f32> {
            <U as MsgTrait>::float_unlabeled_opt(&self.1)
                .or_else(|| <T as MsgTrait>::float_unlabeled_opt(&self.0))
        }
        fn float_optional_opt<'this>(&'this self) -> Option<f32> {
            <U as MsgTrait>::float_optional_opt(&self.1)
                .or_else(|| <T as MsgTrait>::float_optional_opt(&self.0))
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::FloatRepeatedRepeatedType<'this>,
            <U as MsgTrait>::FloatRepeatedRepeatedType<'this>,
        >;

        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as MsgTrait>::float_repeated(&self.0),
                <U as MsgTrait>::float_repeated(&self.1),
            )
        }
        fn bytes_unlabeled_opt<'this>(&'this self) -> Option<&'this [u8]> {
            <U as MsgTrait>::bytes_unlabeled_opt(&self.1)
                .or_else(|| <T as MsgTrait>::bytes_unlabeled_opt(&self.0))
        }
        fn bytes_optional_opt<'this>(&'this self) -> Option<&'this [u8]> {
            <U as MsgTrait>::bytes_optional_opt(&self.1)
                .or_else(|| <T as MsgTrait>::bytes_optional_opt(&self.0))
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::BytesRepeatedRepeatedType<'this>,
            <U as MsgTrait>::BytesRepeatedRepeatedType<'this>,
        >;

        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as MsgTrait>::bytes_repeated(&self.0),
                <U as MsgTrait>::bytes_repeated(&self.1),
            )
        }
        fn string_unlabeled_opt<'this>(&'this self) -> Option<&'this str> {
            <U as MsgTrait>::string_unlabeled_opt(&self.1)
                .or_else(|| <T as MsgTrait>::string_unlabeled_opt(&self.0))
        }
        fn string_optional_opt<'this>(&'this self) -> Option<&'this str> {
            <U as MsgTrait>::string_optional_opt(&self.1)
                .or_else(|| <T as MsgTrait>::string_optional_opt(&self.0))
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::StringRepeatedRepeatedType<'this>,
            <U as MsgTrait>::StringRepeatedRepeatedType<'this>,
        >;

        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as MsgTrait>::string_repeated(&self.0),
                <U as MsgTrait>::string_repeated(&self.1),
            )
        }
        fn enum_unlabeled_opt<'this>(
            &'this self,
        ) -> Option<self::_puroro_root::full_coverage3::Enum> {
            <U as MsgTrait>::enum_unlabeled_opt(&self.1)
                .or_else(|| <T as MsgTrait>::enum_unlabeled_opt(&self.0))
        }
        fn enum_optional_opt<'this>(
            &'this self,
        ) -> Option<self::_puroro_root::full_coverage3::Enum> {
            <U as MsgTrait>::enum_optional_opt(&self.1)
                .or_else(|| <T as MsgTrait>::enum_optional_opt(&self.0))
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::EnumRepeatedRepeatedType<'this>,
            <U as MsgTrait>::EnumRepeatedRepeatedType<'this>,
        >;

        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as MsgTrait>::enum_repeated(&self.0),
                <U as MsgTrait>::enum_repeated(&self.1),
            )
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = (
            ::std::option::Option<<T as MsgTrait>::SubmsgUnlabeledMessageType<'this>>,
            ::std::option::Option<<U as MsgTrait>::SubmsgUnlabeledMessageType<'this>>,
        );
        fn submsg_unlabeled_opt<'this>(
            &'this self,
        ) -> Option<Self::SubmsgUnlabeledMessageType<'this>> {
            match (
                <T as MsgTrait>::submsg_unlabeled_opt(&self.0),
                <U as MsgTrait>::submsg_unlabeled_opt(&self.1),
            ) {
                (None, None) => None,
                (Some(t), None) => Some((Some(t), None)),
                (None, Some(u)) => Some((None, Some(u))),
                (Some(t), Some(u)) => Some((Some(t), Some(u))),
            }
        }
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = (
            ::std::option::Option<<T as MsgTrait>::SubmsgOptionalMessageType<'this>>,
            ::std::option::Option<<U as MsgTrait>::SubmsgOptionalMessageType<'this>>,
        );
        fn submsg_optional_opt<'this>(
            &'this self,
        ) -> Option<Self::SubmsgOptionalMessageType<'this>> {
            match (
                <T as MsgTrait>::submsg_optional_opt(&self.0),
                <U as MsgTrait>::submsg_optional_opt(&self.1),
            ) {
                (None, None) => None,
                (Some(t), None) => Some((Some(t), None)),
                (None, Some(u)) => Some((None, Some(u))),
                (Some(t), Some(u)) => Some((Some(t), Some(u))),
            }
        }
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as MsgTrait>::SubmsgRepeatedMessageType<'this>,
            <U as MsgTrait>::SubmsgRepeatedMessageType<'this>,
        >;
        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedMessageField<
            <T as MsgTrait>::SubmsgRepeatedRepeatedType<'this>,
            <U as MsgTrait>::SubmsgRepeatedRepeatedType<'this>,
        >;

        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
                <T as MsgTrait>::submsg_repeated(&self.0),
                <U as MsgTrait>::submsg_repeated(&self.1),
            )
        }
        fn i64_unlabeled_opt<'this>(&'this self) -> Option<i64> {
            <U as MsgTrait>::i64_unlabeled_opt(&self.1)
                .or_else(|| <T as MsgTrait>::i64_unlabeled_opt(&self.0))
        }
        fn i64_optional_opt<'this>(&'this self) -> Option<i64> {
            <U as MsgTrait>::i64_optional_opt(&self.1)
                .or_else(|| <T as MsgTrait>::i64_optional_opt(&self.0))
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::I64RepeatedRepeatedType<'this>,
            <U as MsgTrait>::I64RepeatedRepeatedType<'this>,
        >;

        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as MsgTrait>::i64_repeated(&self.0),
                <U as MsgTrait>::i64_repeated(&self.1),
            )
        }
        fn u32_unlabeled_opt<'this>(&'this self) -> Option<u32> {
            <U as MsgTrait>::u32_unlabeled_opt(&self.1)
                .or_else(|| <T as MsgTrait>::u32_unlabeled_opt(&self.0))
        }
        fn u32_optional_opt<'this>(&'this self) -> Option<u32> {
            <U as MsgTrait>::u32_optional_opt(&self.1)
                .or_else(|| <T as MsgTrait>::u32_optional_opt(&self.0))
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::U32RepeatedRepeatedType<'this>,
            <U as MsgTrait>::U32RepeatedRepeatedType<'this>,
        >;

        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as MsgTrait>::u32_repeated(&self.0),
                <U as MsgTrait>::u32_repeated(&self.1),
            )
        }
        fn u64_unlabeled_opt<'this>(&'this self) -> Option<u64> {
            <U as MsgTrait>::u64_unlabeled_opt(&self.1)
                .or_else(|| <T as MsgTrait>::u64_unlabeled_opt(&self.0))
        }
        fn u64_optional_opt<'this>(&'this self) -> Option<u64> {
            <U as MsgTrait>::u64_optional_opt(&self.1)
                .or_else(|| <T as MsgTrait>::u64_optional_opt(&self.0))
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::U64RepeatedRepeatedType<'this>,
            <U as MsgTrait>::U64RepeatedRepeatedType<'this>,
        >;

        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as MsgTrait>::u64_repeated(&self.0),
                <U as MsgTrait>::u64_repeated(&self.1),
            )
        }
        fn s32_unlabeled_opt<'this>(&'this self) -> Option<i32> {
            <U as MsgTrait>::s32_unlabeled_opt(&self.1)
                .or_else(|| <T as MsgTrait>::s32_unlabeled_opt(&self.0))
        }
        fn s32_optional_opt<'this>(&'this self) -> Option<i32> {
            <U as MsgTrait>::s32_optional_opt(&self.1)
                .or_else(|| <T as MsgTrait>::s32_optional_opt(&self.0))
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::S32RepeatedRepeatedType<'this>,
            <U as MsgTrait>::S32RepeatedRepeatedType<'this>,
        >;

        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as MsgTrait>::s32_repeated(&self.0),
                <U as MsgTrait>::s32_repeated(&self.1),
            )
        }
        fn s64_unlabeled_opt<'this>(&'this self) -> Option<i64> {
            <U as MsgTrait>::s64_unlabeled_opt(&self.1)
                .or_else(|| <T as MsgTrait>::s64_unlabeled_opt(&self.0))
        }
        fn s64_optional_opt<'this>(&'this self) -> Option<i64> {
            <U as MsgTrait>::s64_optional_opt(&self.1)
                .or_else(|| <T as MsgTrait>::s64_optional_opt(&self.0))
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::S64RepeatedRepeatedType<'this>,
            <U as MsgTrait>::S64RepeatedRepeatedType<'this>,
        >;

        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as MsgTrait>::s64_repeated(&self.0),
                <U as MsgTrait>::s64_repeated(&self.1),
            )
        }
        fn fixed32_unlabeled_opt<'this>(&'this self) -> Option<u32> {
            <U as MsgTrait>::fixed32_unlabeled_opt(&self.1)
                .or_else(|| <T as MsgTrait>::fixed32_unlabeled_opt(&self.0))
        }
        fn fixed32_optional_opt<'this>(&'this self) -> Option<u32> {
            <U as MsgTrait>::fixed32_optional_opt(&self.1)
                .or_else(|| <T as MsgTrait>::fixed32_optional_opt(&self.0))
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::Fixed32RepeatedRepeatedType<'this>,
            <U as MsgTrait>::Fixed32RepeatedRepeatedType<'this>,
        >;

        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as MsgTrait>::fixed32_repeated(&self.0),
                <U as MsgTrait>::fixed32_repeated(&self.1),
            )
        }
        fn fixed64_unlabeled_opt<'this>(&'this self) -> Option<u64> {
            <U as MsgTrait>::fixed64_unlabeled_opt(&self.1)
                .or_else(|| <T as MsgTrait>::fixed64_unlabeled_opt(&self.0))
        }
        fn fixed64_optional_opt<'this>(&'this self) -> Option<u64> {
            <U as MsgTrait>::fixed64_optional_opt(&self.1)
                .or_else(|| <T as MsgTrait>::fixed64_optional_opt(&self.0))
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::Fixed64RepeatedRepeatedType<'this>,
            <U as MsgTrait>::Fixed64RepeatedRepeatedType<'this>,
        >;

        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as MsgTrait>::fixed64_repeated(&self.0),
                <U as MsgTrait>::fixed64_repeated(&self.1),
            )
        }
        fn sfixed32_unlabeled_opt<'this>(&'this self) -> Option<i32> {
            <U as MsgTrait>::sfixed32_unlabeled_opt(&self.1)
                .or_else(|| <T as MsgTrait>::sfixed32_unlabeled_opt(&self.0))
        }
        fn sfixed32_optional_opt<'this>(&'this self) -> Option<i32> {
            <U as MsgTrait>::sfixed32_optional_opt(&self.1)
                .or_else(|| <T as MsgTrait>::sfixed32_optional_opt(&self.0))
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::Sfixed32RepeatedRepeatedType<'this>,
            <U as MsgTrait>::Sfixed32RepeatedRepeatedType<'this>,
        >;

        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as MsgTrait>::sfixed32_repeated(&self.0),
                <U as MsgTrait>::sfixed32_repeated(&self.1),
            )
        }
        fn sfixed64_unlabeled_opt<'this>(&'this self) -> Option<i64> {
            <U as MsgTrait>::sfixed64_unlabeled_opt(&self.1)
                .or_else(|| <T as MsgTrait>::sfixed64_unlabeled_opt(&self.0))
        }
        fn sfixed64_optional_opt<'this>(&'this self) -> Option<i64> {
            <U as MsgTrait>::sfixed64_optional_opt(&self.1)
                .or_else(|| <T as MsgTrait>::sfixed64_optional_opt(&self.0))
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::Sfixed64RepeatedRepeatedType<'this>,
            <U as MsgTrait>::Sfixed64RepeatedRepeatedType<'this>,
        >;

        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as MsgTrait>::sfixed64_repeated(&self.0),
                <U as MsgTrait>::sfixed64_repeated(&self.1),
            )
        }
        fn f64_unlabeled_opt<'this>(&'this self) -> Option<f64> {
            <U as MsgTrait>::f64_unlabeled_opt(&self.1)
                .or_else(|| <T as MsgTrait>::f64_unlabeled_opt(&self.0))
        }
        fn f64_optional_opt<'this>(&'this self) -> Option<f64> {
            <U as MsgTrait>::f64_optional_opt(&self.1)
                .or_else(|| <T as MsgTrait>::f64_optional_opt(&self.0))
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::F64RepeatedRepeatedType<'this>,
            <U as MsgTrait>::F64RepeatedRepeatedType<'this>,
        >;

        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as MsgTrait>::f64_repeated(&self.0),
                <U as MsgTrait>::f64_repeated(&self.1),
            )
        }
    }
    impl<T, U> MsgTrait for ::puroro::Either<T, U>
    where
        T: MsgTrait,
        U: MsgTrait,
    {
        fn i32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().either(
                |t| <T as MsgTrait>::i32_unlabeled_opt(t),
                |u| <U as MsgTrait>::i32_unlabeled_opt(u),
            )
        }
        fn i32_optional_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().either(
                |t| <T as MsgTrait>::i32_optional_opt(t),
                |u| <U as MsgTrait>::i32_optional_opt(u),
            )
        }
        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as MsgTrait>::I32RepeatedRepeatedType<'this>,
            <U as MsgTrait>::I32RepeatedRepeatedType<'this>,
        >;

        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::i32_repeated(t))
                    .map_right(|u| <U as MsgTrait>::i32_repeated(u)),
            )
        }
        fn float_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<f32> {
            self.as_ref().either(
                |t| <T as MsgTrait>::float_unlabeled_opt(t),
                |u| <U as MsgTrait>::float_unlabeled_opt(u),
            )
        }
        fn float_optional_opt<'this>(&'this self) -> ::std::option::Option<f32> {
            self.as_ref().either(
                |t| <T as MsgTrait>::float_optional_opt(t),
                |u| <U as MsgTrait>::float_optional_opt(u),
            )
        }
        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as MsgTrait>::FloatRepeatedRepeatedType<'this>,
            <U as MsgTrait>::FloatRepeatedRepeatedType<'this>,
        >;

        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::float_repeated(t))
                    .map_right(|u| <U as MsgTrait>::float_repeated(u)),
            )
        }
        fn bytes_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<&'this [u8]> {
            self.as_ref().either(
                |t| <T as MsgTrait>::bytes_unlabeled_opt(t),
                |u| <U as MsgTrait>::bytes_unlabeled_opt(u),
            )
        }
        fn bytes_optional_opt<'this>(&'this self) -> ::std::option::Option<&'this [u8]> {
            self.as_ref().either(
                |t| <T as MsgTrait>::bytes_optional_opt(t),
                |u| <U as MsgTrait>::bytes_optional_opt(u),
            )
        }
        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as MsgTrait>::BytesRepeatedRepeatedType<'this>,
            <U as MsgTrait>::BytesRepeatedRepeatedType<'this>,
        >;

        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::bytes_repeated(t))
                    .map_right(|u| <U as MsgTrait>::bytes_repeated(u)),
            )
        }
        fn string_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().either(
                |t| <T as MsgTrait>::string_unlabeled_opt(t),
                |u| <U as MsgTrait>::string_unlabeled_opt(u),
            )
        }
        fn string_optional_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().either(
                |t| <T as MsgTrait>::string_optional_opt(t),
                |u| <U as MsgTrait>::string_optional_opt(u),
            )
        }
        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as MsgTrait>::StringRepeatedRepeatedType<'this>,
            <U as MsgTrait>::StringRepeatedRepeatedType<'this>,
        >;

        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::string_repeated(t))
                    .map_right(|u| <U as MsgTrait>::string_repeated(u)),
            )
        }
        fn enum_unlabeled_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::full_coverage3::Enum> {
            self.as_ref().either(
                |t| <T as MsgTrait>::enum_unlabeled_opt(t),
                |u| <U as MsgTrait>::enum_unlabeled_opt(u),
            )
        }
        fn enum_optional_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::full_coverage3::Enum> {
            self.as_ref().either(
                |t| <T as MsgTrait>::enum_optional_opt(t),
                |u| <U as MsgTrait>::enum_optional_opt(u),
            )
        }
        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as MsgTrait>::EnumRepeatedRepeatedType<'this>,
            <U as MsgTrait>::EnumRepeatedRepeatedType<'this>,
        >;

        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::enum_repeated(t))
                    .map_right(|u| <U as MsgTrait>::enum_repeated(u)),
            )
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as MsgTrait>::SubmsgUnlabeledMessageType<'this>,
            <U as MsgTrait>::SubmsgUnlabeledMessageType<'this>,
        >;
        fn submsg_unlabeled_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::SubmsgUnlabeledMessageType<'this>> {
            self.as_ref().either(
                |t| <T as MsgTrait>::submsg_unlabeled_opt(t).map(|t| ::puroro::Either::Left(t)),
                |u| <U as MsgTrait>::submsg_unlabeled_opt(u).map(|u| ::puroro::Either::Right(u)),
            )
        }
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as MsgTrait>::SubmsgOptionalMessageType<'this>,
            <U as MsgTrait>::SubmsgOptionalMessageType<'this>,
        >;
        fn submsg_optional_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::SubmsgOptionalMessageType<'this>> {
            self.as_ref().either(
                |t| <T as MsgTrait>::submsg_optional_opt(t).map(|t| ::puroro::Either::Left(t)),
                |u| <U as MsgTrait>::submsg_optional_opt(u).map(|u| ::puroro::Either::Right(u)),
            )
        }
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as MsgTrait>::SubmsgRepeatedMessageType<'this>,
            <U as MsgTrait>::SubmsgRepeatedMessageType<'this>,
        >;
        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedMessageField<
            <T as MsgTrait>::SubmsgRepeatedRepeatedType<'this>,
            <U as MsgTrait>::SubmsgRepeatedRepeatedType<'this>,
        >;

        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::submsg_repeated(t))
                    .map_right(|u| <U as MsgTrait>::submsg_repeated(u)),
            )
        }
        fn i64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            self.as_ref().either(
                |t| <T as MsgTrait>::i64_unlabeled_opt(t),
                |u| <U as MsgTrait>::i64_unlabeled_opt(u),
            )
        }
        fn i64_optional_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            self.as_ref().either(
                |t| <T as MsgTrait>::i64_optional_opt(t),
                |u| <U as MsgTrait>::i64_optional_opt(u),
            )
        }
        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as MsgTrait>::I64RepeatedRepeatedType<'this>,
            <U as MsgTrait>::I64RepeatedRepeatedType<'this>,
        >;

        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::i64_repeated(t))
                    .map_right(|u| <U as MsgTrait>::i64_repeated(u)),
            )
        }
        fn u32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<u32> {
            self.as_ref().either(
                |t| <T as MsgTrait>::u32_unlabeled_opt(t),
                |u| <U as MsgTrait>::u32_unlabeled_opt(u),
            )
        }
        fn u32_optional_opt<'this>(&'this self) -> ::std::option::Option<u32> {
            self.as_ref().either(
                |t| <T as MsgTrait>::u32_optional_opt(t),
                |u| <U as MsgTrait>::u32_optional_opt(u),
            )
        }
        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as MsgTrait>::U32RepeatedRepeatedType<'this>,
            <U as MsgTrait>::U32RepeatedRepeatedType<'this>,
        >;

        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::u32_repeated(t))
                    .map_right(|u| <U as MsgTrait>::u32_repeated(u)),
            )
        }
        fn u64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<u64> {
            self.as_ref().either(
                |t| <T as MsgTrait>::u64_unlabeled_opt(t),
                |u| <U as MsgTrait>::u64_unlabeled_opt(u),
            )
        }
        fn u64_optional_opt<'this>(&'this self) -> ::std::option::Option<u64> {
            self.as_ref().either(
                |t| <T as MsgTrait>::u64_optional_opt(t),
                |u| <U as MsgTrait>::u64_optional_opt(u),
            )
        }
        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as MsgTrait>::U64RepeatedRepeatedType<'this>,
            <U as MsgTrait>::U64RepeatedRepeatedType<'this>,
        >;

        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::u64_repeated(t))
                    .map_right(|u| <U as MsgTrait>::u64_repeated(u)),
            )
        }
        fn s32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().either(
                |t| <T as MsgTrait>::s32_unlabeled_opt(t),
                |u| <U as MsgTrait>::s32_unlabeled_opt(u),
            )
        }
        fn s32_optional_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().either(
                |t| <T as MsgTrait>::s32_optional_opt(t),
                |u| <U as MsgTrait>::s32_optional_opt(u),
            )
        }
        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as MsgTrait>::S32RepeatedRepeatedType<'this>,
            <U as MsgTrait>::S32RepeatedRepeatedType<'this>,
        >;

        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::s32_repeated(t))
                    .map_right(|u| <U as MsgTrait>::s32_repeated(u)),
            )
        }
        fn s64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            self.as_ref().either(
                |t| <T as MsgTrait>::s64_unlabeled_opt(t),
                |u| <U as MsgTrait>::s64_unlabeled_opt(u),
            )
        }
        fn s64_optional_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            self.as_ref().either(
                |t| <T as MsgTrait>::s64_optional_opt(t),
                |u| <U as MsgTrait>::s64_optional_opt(u),
            )
        }
        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as MsgTrait>::S64RepeatedRepeatedType<'this>,
            <U as MsgTrait>::S64RepeatedRepeatedType<'this>,
        >;

        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::s64_repeated(t))
                    .map_right(|u| <U as MsgTrait>::s64_repeated(u)),
            )
        }
        fn fixed32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<u32> {
            self.as_ref().either(
                |t| <T as MsgTrait>::fixed32_unlabeled_opt(t),
                |u| <U as MsgTrait>::fixed32_unlabeled_opt(u),
            )
        }
        fn fixed32_optional_opt<'this>(&'this self) -> ::std::option::Option<u32> {
            self.as_ref().either(
                |t| <T as MsgTrait>::fixed32_optional_opt(t),
                |u| <U as MsgTrait>::fixed32_optional_opt(u),
            )
        }
        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as MsgTrait>::Fixed32RepeatedRepeatedType<'this>,
            <U as MsgTrait>::Fixed32RepeatedRepeatedType<'this>,
        >;

        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::fixed32_repeated(t))
                    .map_right(|u| <U as MsgTrait>::fixed32_repeated(u)),
            )
        }
        fn fixed64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<u64> {
            self.as_ref().either(
                |t| <T as MsgTrait>::fixed64_unlabeled_opt(t),
                |u| <U as MsgTrait>::fixed64_unlabeled_opt(u),
            )
        }
        fn fixed64_optional_opt<'this>(&'this self) -> ::std::option::Option<u64> {
            self.as_ref().either(
                |t| <T as MsgTrait>::fixed64_optional_opt(t),
                |u| <U as MsgTrait>::fixed64_optional_opt(u),
            )
        }
        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as MsgTrait>::Fixed64RepeatedRepeatedType<'this>,
            <U as MsgTrait>::Fixed64RepeatedRepeatedType<'this>,
        >;

        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::fixed64_repeated(t))
                    .map_right(|u| <U as MsgTrait>::fixed64_repeated(u)),
            )
        }
        fn sfixed32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().either(
                |t| <T as MsgTrait>::sfixed32_unlabeled_opt(t),
                |u| <U as MsgTrait>::sfixed32_unlabeled_opt(u),
            )
        }
        fn sfixed32_optional_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().either(
                |t| <T as MsgTrait>::sfixed32_optional_opt(t),
                |u| <U as MsgTrait>::sfixed32_optional_opt(u),
            )
        }
        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as MsgTrait>::Sfixed32RepeatedRepeatedType<'this>,
            <U as MsgTrait>::Sfixed32RepeatedRepeatedType<'this>,
        >;

        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::sfixed32_repeated(t))
                    .map_right(|u| <U as MsgTrait>::sfixed32_repeated(u)),
            )
        }
        fn sfixed64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            self.as_ref().either(
                |t| <T as MsgTrait>::sfixed64_unlabeled_opt(t),
                |u| <U as MsgTrait>::sfixed64_unlabeled_opt(u),
            )
        }
        fn sfixed64_optional_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            self.as_ref().either(
                |t| <T as MsgTrait>::sfixed64_optional_opt(t),
                |u| <U as MsgTrait>::sfixed64_optional_opt(u),
            )
        }
        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as MsgTrait>::Sfixed64RepeatedRepeatedType<'this>,
            <U as MsgTrait>::Sfixed64RepeatedRepeatedType<'this>,
        >;

        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::sfixed64_repeated(t))
                    .map_right(|u| <U as MsgTrait>::sfixed64_repeated(u)),
            )
        }
        fn f64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<f64> {
            self.as_ref().either(
                |t| <T as MsgTrait>::f64_unlabeled_opt(t),
                |u| <U as MsgTrait>::f64_unlabeled_opt(u),
            )
        }
        fn f64_optional_opt<'this>(&'this self) -> ::std::option::Option<f64> {
            self.as_ref().either(
                |t| <T as MsgTrait>::f64_optional_opt(t),
                |u| <U as MsgTrait>::f64_optional_opt(u),
            )
        }
        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as MsgTrait>::F64RepeatedRepeatedType<'this>,
            <U as MsgTrait>::F64RepeatedRepeatedType<'this>,
        >;

        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::f64_repeated(t))
                    .map_right(|u| <U as MsgTrait>::f64_repeated(u)),
            )
        }
    }
    impl<T> MsgTrait for ::std::option::Option<T>
    where
        T: MsgTrait,
    {
        fn i32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().and_then(|msg| msg.i32_unlabeled_opt())
        }
        fn i32_optional_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().and_then(|msg| msg.i32_optional_opt())
        }

        type I32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::option::OptionRepeatedField<T::I32RepeatedRepeatedType<'this>>;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::option::OptionRepeatedField::new(
                self.as_ref().map(|msg| msg.i32_repeated()),
            )
        }
        fn float_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<f32> {
            self.as_ref().and_then(|msg| msg.float_unlabeled_opt())
        }
        fn float_optional_opt<'this>(&'this self) -> ::std::option::Option<f32> {
            self.as_ref().and_then(|msg| msg.float_optional_opt())
        }

        type FloatRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::option::OptionRepeatedField<
            T::FloatRepeatedRepeatedType<'this>,
        >;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::option::OptionRepeatedField::new(
                self.as_ref().map(|msg| msg.float_repeated()),
            )
        }
        fn bytes_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<&'this [u8]> {
            self.as_ref().and_then(|msg| msg.bytes_unlabeled_opt())
        }
        fn bytes_optional_opt<'this>(&'this self) -> ::std::option::Option<&'this [u8]> {
            self.as_ref().and_then(|msg| msg.bytes_optional_opt())
        }

        type BytesRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::option::OptionRepeatedField<
            T::BytesRepeatedRepeatedType<'this>,
        >;
        fn bytes_repeated<'this>(&'this self) -> Self::BytesRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::option::OptionRepeatedField::new(
                self.as_ref().map(|msg| msg.bytes_repeated()),
            )
        }
        fn string_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().and_then(|msg| msg.string_unlabeled_opt())
        }
        fn string_optional_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().and_then(|msg| msg.string_optional_opt())
        }

        type StringRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::option::OptionRepeatedField<
            T::StringRepeatedRepeatedType<'this>,
        >;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::option::OptionRepeatedField::new(
                self.as_ref().map(|msg| msg.string_repeated()),
            )
        }
        fn enum_unlabeled_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::full_coverage3::Enum> {
            self.as_ref().and_then(|msg| msg.enum_unlabeled_opt())
        }
        fn enum_optional_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::full_coverage3::Enum> {
            self.as_ref().and_then(|msg| msg.enum_optional_opt())
        }

        type EnumRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::option::OptionRepeatedField<
            T::EnumRepeatedRepeatedType<'this>,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::option::OptionRepeatedField::new(
                self.as_ref().map(|msg| msg.enum_repeated()),
            )
        }
        type SubmsgUnlabeledMessageType<'this>
        where
            Self: 'this,
        = T::SubmsgUnlabeledMessageType<'this>;
        fn submsg_unlabeled_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::SubmsgUnlabeledMessageType<'this>> {
            self.as_ref().and_then(|msg| msg.submsg_unlabeled_opt())
        }
        type SubmsgOptionalMessageType<'this>
        where
            Self: 'this,
        = T::SubmsgOptionalMessageType<'this>;
        fn submsg_optional_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::SubmsgOptionalMessageType<'this>> {
            self.as_ref().and_then(|msg| msg.submsg_optional_opt())
        }
        type SubmsgRepeatedMessageType<'this>
        where
            Self: 'this,
        = T::SubmsgRepeatedMessageType<'this>;

        type SubmsgRepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::option::OptionRepeatedField<
            T::SubmsgRepeatedRepeatedType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::option::OptionRepeatedField::new(
                self.as_ref().map(|msg| msg.submsg_repeated()),
            )
        }
        fn i64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            self.as_ref().and_then(|msg| msg.i64_unlabeled_opt())
        }
        fn i64_optional_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            self.as_ref().and_then(|msg| msg.i64_optional_opt())
        }

        type I64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::option::OptionRepeatedField<T::I64RepeatedRepeatedType<'this>>;
        fn i64_repeated<'this>(&'this self) -> Self::I64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::option::OptionRepeatedField::new(
                self.as_ref().map(|msg| msg.i64_repeated()),
            )
        }
        fn u32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<u32> {
            self.as_ref().and_then(|msg| msg.u32_unlabeled_opt())
        }
        fn u32_optional_opt<'this>(&'this self) -> ::std::option::Option<u32> {
            self.as_ref().and_then(|msg| msg.u32_optional_opt())
        }

        type U32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::option::OptionRepeatedField<T::U32RepeatedRepeatedType<'this>>;
        fn u32_repeated<'this>(&'this self) -> Self::U32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::option::OptionRepeatedField::new(
                self.as_ref().map(|msg| msg.u32_repeated()),
            )
        }
        fn u64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<u64> {
            self.as_ref().and_then(|msg| msg.u64_unlabeled_opt())
        }
        fn u64_optional_opt<'this>(&'this self) -> ::std::option::Option<u64> {
            self.as_ref().and_then(|msg| msg.u64_optional_opt())
        }

        type U64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::option::OptionRepeatedField<T::U64RepeatedRepeatedType<'this>>;
        fn u64_repeated<'this>(&'this self) -> Self::U64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::option::OptionRepeatedField::new(
                self.as_ref().map(|msg| msg.u64_repeated()),
            )
        }
        fn s32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().and_then(|msg| msg.s32_unlabeled_opt())
        }
        fn s32_optional_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().and_then(|msg| msg.s32_optional_opt())
        }

        type S32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::option::OptionRepeatedField<T::S32RepeatedRepeatedType<'this>>;
        fn s32_repeated<'this>(&'this self) -> Self::S32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::option::OptionRepeatedField::new(
                self.as_ref().map(|msg| msg.s32_repeated()),
            )
        }
        fn s64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            self.as_ref().and_then(|msg| msg.s64_unlabeled_opt())
        }
        fn s64_optional_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            self.as_ref().and_then(|msg| msg.s64_optional_opt())
        }

        type S64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::option::OptionRepeatedField<T::S64RepeatedRepeatedType<'this>>;
        fn s64_repeated<'this>(&'this self) -> Self::S64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::option::OptionRepeatedField::new(
                self.as_ref().map(|msg| msg.s64_repeated()),
            )
        }
        fn fixed32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<u32> {
            self.as_ref().and_then(|msg| msg.fixed32_unlabeled_opt())
        }
        fn fixed32_optional_opt<'this>(&'this self) -> ::std::option::Option<u32> {
            self.as_ref().and_then(|msg| msg.fixed32_optional_opt())
        }

        type Fixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::option::OptionRepeatedField<
            T::Fixed32RepeatedRepeatedType<'this>,
        >;
        fn fixed32_repeated<'this>(&'this self) -> Self::Fixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::option::OptionRepeatedField::new(
                self.as_ref().map(|msg| msg.fixed32_repeated()),
            )
        }
        fn fixed64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<u64> {
            self.as_ref().and_then(|msg| msg.fixed64_unlabeled_opt())
        }
        fn fixed64_optional_opt<'this>(&'this self) -> ::std::option::Option<u64> {
            self.as_ref().and_then(|msg| msg.fixed64_optional_opt())
        }

        type Fixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::option::OptionRepeatedField<
            T::Fixed64RepeatedRepeatedType<'this>,
        >;
        fn fixed64_repeated<'this>(&'this self) -> Self::Fixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::option::OptionRepeatedField::new(
                self.as_ref().map(|msg| msg.fixed64_repeated()),
            )
        }
        fn sfixed32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().and_then(|msg| msg.sfixed32_unlabeled_opt())
        }
        fn sfixed32_optional_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().and_then(|msg| msg.sfixed32_optional_opt())
        }

        type Sfixed32RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::option::OptionRepeatedField<
            T::Sfixed32RepeatedRepeatedType<'this>,
        >;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Sfixed32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::option::OptionRepeatedField::new(
                self.as_ref().map(|msg| msg.sfixed32_repeated()),
            )
        }
        fn sfixed64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            self.as_ref().and_then(|msg| msg.sfixed64_unlabeled_opt())
        }
        fn sfixed64_optional_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            self.as_ref().and_then(|msg| msg.sfixed64_optional_opt())
        }

        type Sfixed64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::option::OptionRepeatedField<
            T::Sfixed64RepeatedRepeatedType<'this>,
        >;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Sfixed64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::option::OptionRepeatedField::new(
                self.as_ref().map(|msg| msg.sfixed64_repeated()),
            )
        }
        fn f64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<f64> {
            self.as_ref().and_then(|msg| msg.f64_unlabeled_opt())
        }
        fn f64_optional_opt<'this>(&'this self) -> ::std::option::Option<f64> {
            self.as_ref().and_then(|msg| msg.f64_optional_opt())
        }

        type F64RepeatedRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::option::OptionRepeatedField<T::F64RepeatedRepeatedType<'this>>;
        fn f64_repeated<'this>(&'this self) -> Self::F64RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::option::OptionRepeatedField::new(
                self.as_ref().map(|msg| msg.f64_repeated()),
            )
        }
    }
}
#[derive(::std::fmt::Debug, ::std::clone::Clone, ::std::marker::Copy, ::std::cmp::PartialEq)]
pub enum Enum {
    Zeroth,
    First,
    Tenth,
    _Unknown(i32),
}

impl ::puroro::Enum3 for Enum {}
impl ::std::convert::From<i32> for Enum {
    fn from(value: i32) -> Self {
        match value {
            0 => Enum::Zeroth,
            1 => Enum::First,
            10 => Enum::Tenth,
            _ => Enum::_Unknown(value),
        }
    }
}

impl ::std::convert::From<Enum> for i32 {
    fn from(value: Enum) -> i32 {
        match value {
            Enum::Zeroth => 0,
            Enum::First => 1,
            Enum::Tenth => 10,
            Enum::_Unknown(ivalue) => ivalue,
        }
    }
}

impl ::std::default::Default for Enum {
    fn default() -> Self {
        Enum::Zeroth
    }
}

impl<'bump> ::puroro::internal::BumpDefault<'bump> for Enum {
    fn default_in(_: &'bump ::puroro::bumpalo::Bump) -> Self {
        ::std::default::Default::default()
    }
}
pub use _puroro_nested::*;
pub mod _puroro_nested {
    pub mod msg {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }

        pub use _puroro_simple_impl::Submsg;
        pub mod _puroro_simple_impl {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }
            pub struct Submsg {
                _bitfield: ::puroro::bitvec::array::BitArray<
                    ::puroro::bitvec::order::Lsb0,
                    [u32; (0 + 31) / 32],
                >,
                i32_unlabeled: ::puroro::internal::Bare<i32>,
                i32_optional: ::puroro::internal::Bare<i32>,
                i64_unlabeled: ::puroro::internal::Bare<i64>,
            }
            impl ::puroro::Message<Submsg> for Submsg {}

            impl Submsg {
                pub fn new() -> Self {
                    Self {
                        _bitfield: ::std::default::Default::default(),
                        i32_unlabeled: ::std::default::Default::default(),
                        i32_optional: ::std::default::Default::default(),
                        i64_unlabeled: ::std::default::Default::default(),
                    }
                }
                pub fn i32_unlabeled_opt(&self) -> ::std::option::Option<i32> {
                    if !::puroro::internal::IsDefault::is_default(&*self.i32_unlabeled) {
                        ::std::option::Option::Some(self.i32_unlabeled.inner())
                    } else {
                        ::std::option::Option::None
                    }
                }

                pub fn has_i32_unlabeled(&self) -> bool {
                    Self::i32_unlabeled_opt(self).is_some()
                }

                pub fn i32_unlabeled(&self) -> i32 {
                    self.i32_unlabeled_opt()
                        .unwrap_or(::std::default::Default::default())
                }
                pub fn i32_optional_opt(&self) -> ::std::option::Option<i32> {
                    if !::puroro::internal::IsDefault::is_default(&*self.i32_optional) {
                        ::std::option::Option::Some(self.i32_optional.inner())
                    } else {
                        ::std::option::Option::None
                    }
                }

                pub fn has_i32_optional(&self) -> bool {
                    Self::i32_optional_opt(self).is_some()
                }

                pub fn i32_optional(&self) -> i32 {
                    self.i32_optional_opt()
                        .unwrap_or(::std::default::Default::default())
                }
                pub fn i64_unlabeled_opt(&self) -> ::std::option::Option<i64> {
                    if !::puroro::internal::IsDefault::is_default(&*self.i64_unlabeled) {
                        ::std::option::Option::Some(self.i64_unlabeled.inner())
                    } else {
                        ::std::option::Option::None
                    }
                }

                pub fn has_i64_unlabeled(&self) -> bool {
                    Self::i64_unlabeled_opt(self).is_some()
                }

                pub fn i64_unlabeled(&self) -> i64 {
                    self.i64_unlabeled_opt()
                        .unwrap_or(::std::default::Default::default())
                }
                pub fn clear_i32_unlabeled(&mut self) {
                    self.i32_unlabeled = ::std::default::Default::default();
                }
                pub fn i32_unlabeled_mut(&mut self) -> &'_ mut i32 {
                    if !self.has_i32_unlabeled() {
                        self.i32_unlabeled = ::std::default::Default::default();
                    }
                    &mut self.i32_unlabeled
                }
                pub fn clear_i32_optional(&mut self) {
                    self.i32_optional = ::std::default::Default::default();
                }
                pub fn i32_optional_mut(&mut self) -> &'_ mut i32 {
                    if !self.has_i32_optional() {
                        self.i32_optional = ::std::default::Default::default();
                    }
                    &mut self.i32_optional
                }
                pub fn clear_i64_unlabeled(&mut self) {
                    self.i64_unlabeled = ::std::default::Default::default();
                }
                pub fn i64_unlabeled_mut(&mut self) -> &'_ mut i64 {
                    if !self.has_i64_unlabeled() {
                        self.i64_unlabeled = ::std::default::Default::default();
                    }
                    &mut self.i64_unlabeled
                }
            }

            impl super::_puroro_traits::SubmsgTrait for Submsg {
                fn i32_unlabeled_opt<'this>(&'this self) -> Option<i32> {
                    <self::Submsg>::i32_unlabeled_opt(self)
                }
                fn i32_optional_opt<'this>(&'this self) -> Option<i32> {
                    <self::Submsg>::i32_optional_opt(self)
                }
                fn i64_unlabeled_opt<'this>(&'this self) -> Option<i64> {
                    <self::Submsg>::i64_unlabeled_opt(self)
                }
            }

            impl ::puroro::MessageRepresentativeImpl for Submsg {}

            impl ::puroro::internal::de::DeserMessageFromBytesIter for Submsg {
                fn deser_field<I>(
                    &mut self,
                    field_number: i32,
                    data: ::puroro::internal::types::FieldData<
                        &mut ::puroro::internal::de::from_iter::ScopedIter<I>,
                    >,
                ) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    use ::puroro::internal::impls::simple::de::DeserFieldFromBytesIter;
                    match field_number {
                        1 => DeserFieldFromBytesIter::<
                            ::puroro::tags::Unlabeled,
                            ::puroro::tags::Int32,
                        >::deser_field(&mut self.i32_unlabeled, data),
                        2 => DeserFieldFromBytesIter::<
                            ::puroro::tags::Unlabeled,
                            ::puroro::tags::Int32,
                        >::deser_field(&mut self.i32_optional, data),
                        101 => DeserFieldFromBytesIter::<
                            ::puroro::tags::Unlabeled,
                            ::puroro::tags::Int64,
                        >::deser_field(&mut self.i64_unlabeled, data),

                        _ => unimplemented!("TODO: This case should be handled properly..."),
                    }
                }
            }

            impl ::puroro::internal::se::SerMessageToIoWrite for Submsg
            where
                Self: super::_puroro_traits::SubmsgTrait,
            {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    ::puroro::internal::se::SerFieldToIoWrite::<
                        ::puroro::tags::Unlabeled,
                        ::puroro::tags::Int32,
                    >::ser_field(
                        <Self as super::_puroro_traits::SubmsgTrait>::i32_unlabeled_opt(self),
                        1,
                        out,
                    )?;
                    ::puroro::internal::se::SerFieldToIoWrite::<
                        ::puroro::tags::Unlabeled,
                        ::puroro::tags::Int32,
                    >::ser_field(
                        <Self as super::_puroro_traits::SubmsgTrait>::i32_optional_opt(self),
                        2,
                        out,
                    )?;
                    ::puroro::internal::se::SerFieldToIoWrite::<
                        ::puroro::tags::Unlabeled,
                        ::puroro::tags::Int64,
                    >::ser_field(
                        <Self as super::_puroro_traits::SubmsgTrait>::i64_unlabeled_opt(self),
                        101,
                        out,
                    )?;
                    ::std::result::Result::Ok(())
                }
            }

            impl ::std::default::Default for Submsg {
                fn default() -> Self {
                    Self::new()
                }
            }

            impl ::std::fmt::Debug for Submsg
            where
                Self: super::_puroro_traits::SubmsgTrait,
            {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.debug_struct("Submsg")
                        .field("i32_unlabeled", &self.i32_unlabeled())
                        .field("i32_optional", &self.i32_optional())
                        .field("i64_unlabeled", &self.i64_unlabeled())
                        .finish()
                }
            }

            impl ::std::clone::Clone for Submsg {
                fn clone(&self) -> Self {
                    Self {
                        _bitfield: ::std::clone::Clone::clone(&self._bitfield),
                        i32_unlabeled: ::std::clone::Clone::clone(&self.i32_unlabeled),
                        i32_optional: ::std::clone::Clone::clone(&self.i32_optional),
                        i64_unlabeled: ::std::clone::Clone::clone(&self.i64_unlabeled),
                    }
                }
            }

            impl ::std::cmp::PartialEq for Submsg {
                fn eq(&self, rhs: &Self) -> bool {
                    self._bitfield == rhs._bitfield
                        && self.i32_unlabeled == rhs.i32_unlabeled
                        && self.i32_optional == rhs.i32_optional
                        && self.i64_unlabeled == rhs.i64_unlabeled
                        && true
                }
            }
        }

        pub use _puroro_impls::*;
        pub mod _puroro_impls {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }
            use super::_puroro_traits::*;

            pub struct SubmsgSingleField1<ScalarType>
            where
                ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
            {
                pub i32_unlabeled: ScalarType,
            }

            impl<ScalarType> ::puroro::Message<super::Submsg> for SubmsgSingleField1<ScalarType> where
                ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone
            {
            }

            impl<ScalarType> super::_puroro_traits::SubmsgTrait for SubmsgSingleField1<ScalarType>
            where
                ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
            {
                fn i32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                    ::std::option::Option::Some(::std::convert::Into::into(
                        ::std::clone::Clone::clone(&self.i32_unlabeled),
                    ))
                }
            }

            impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for SubmsgSingleField1<ScalarType>
            where
                ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
                Self: super::_puroro_traits::SubmsgTrait,
            {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    ::puroro::internal::se::SerFieldToIoWrite::<
                        ::puroro::tags::Unlabeled,
                        ::puroro::tags::Int32,
                    >::ser_field(
                        <Self as super::_puroro_traits::SubmsgTrait>::i32_unlabeled_opt(self),
                        1,
                        out,
                    )?;
                    ::std::result::Result::Ok(())
                }
            }

            impl<ScalarType> ::std::convert::From<ScalarType> for SubmsgSingleField1<ScalarType>
            where
                ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
            {
                fn from(value: ScalarType) -> Self {
                    Self {
                        i32_unlabeled: value,
                    }
                }
            }

            impl<ScalarType> ::std::clone::Clone for SubmsgSingleField1<ScalarType>
            where
                ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
                ScalarType: ::std::clone::Clone,
            {
                fn clone(&self) -> Self {
                    Self {
                        i32_unlabeled: ::std::clone::Clone::clone(&self.i32_unlabeled),
                    }
                }
            }

            pub struct SubmsgSingleField2<ScalarType>
            where
                ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
            {
                pub i32_optional: ScalarType,
            }

            impl<ScalarType> ::puroro::Message<super::Submsg> for SubmsgSingleField2<ScalarType> where
                ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone
            {
            }

            impl<ScalarType> super::_puroro_traits::SubmsgTrait for SubmsgSingleField2<ScalarType>
            where
                ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
            {
                fn i32_optional_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                    ::std::option::Option::Some(::std::convert::Into::into(
                        ::std::clone::Clone::clone(&self.i32_optional),
                    ))
                }
            }

            impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for SubmsgSingleField2<ScalarType>
            where
                ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
                Self: super::_puroro_traits::SubmsgTrait,
            {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    ::puroro::internal::se::SerFieldToIoWrite::<
                        ::puroro::tags::Unlabeled,
                        ::puroro::tags::Int32,
                    >::ser_field(
                        <Self as super::_puroro_traits::SubmsgTrait>::i32_optional_opt(self),
                        2,
                        out,
                    )?;
                    ::std::result::Result::Ok(())
                }
            }

            impl<ScalarType> ::std::convert::From<ScalarType> for SubmsgSingleField2<ScalarType>
            where
                ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
            {
                fn from(value: ScalarType) -> Self {
                    Self {
                        i32_optional: value,
                    }
                }
            }

            impl<ScalarType> ::std::clone::Clone for SubmsgSingleField2<ScalarType>
            where
                ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
                ScalarType: ::std::clone::Clone,
            {
                fn clone(&self) -> Self {
                    Self {
                        i32_optional: ::std::clone::Clone::clone(&self.i32_optional),
                    }
                }
            }

            pub struct SubmsgSingleField101<ScalarType>
            where
                ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
            {
                pub i64_unlabeled: ScalarType,
            }

            impl<ScalarType> ::puroro::Message<super::Submsg> for SubmsgSingleField101<ScalarType> where
                ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone
            {
            }

            impl<ScalarType> super::_puroro_traits::SubmsgTrait for SubmsgSingleField101<ScalarType>
            where
                ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
            {
                fn i64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i64> {
                    ::std::option::Option::Some(::std::convert::Into::into(
                        ::std::clone::Clone::clone(&self.i64_unlabeled),
                    ))
                }
            }

            impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for SubmsgSingleField101<ScalarType>
            where
                ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
                Self: super::_puroro_traits::SubmsgTrait,
            {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    ::puroro::internal::se::SerFieldToIoWrite::<
                        ::puroro::tags::Unlabeled,
                        ::puroro::tags::Int64,
                    >::ser_field(
                        <Self as super::_puroro_traits::SubmsgTrait>::i64_unlabeled_opt(self),
                        101,
                        out,
                    )?;
                    ::std::result::Result::Ok(())
                }
            }

            impl<ScalarType> ::std::convert::From<ScalarType> for SubmsgSingleField101<ScalarType>
            where
                ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
            {
                fn from(value: ScalarType) -> Self {
                    Self {
                        i64_unlabeled: value,
                    }
                }
            }

            impl<ScalarType> ::std::clone::Clone for SubmsgSingleField101<ScalarType>
            where
                ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
                ScalarType: ::std::clone::Clone,
            {
                fn clone(&self) -> Self {
                    Self {
                        i64_unlabeled: ::std::clone::Clone::clone(&self.i64_unlabeled),
                    }
                }
            }
            pub struct SubmsgBumpalo<'bump> {
                _bump: &'bump ::puroro::bumpalo::Bump,
                _bitfield: ::puroro::bitvec::array::BitArray<
                    ::puroro::bitvec::order::Lsb0,
                    [u32; (0 + 31) / 32],
                >,
                i32_unlabeled: ::puroro::internal::Bare<i32>,
                i32_optional: ::puroro::internal::Bare<i32>,
                i64_unlabeled: ::puroro::internal::Bare<i64>,
            }

            pub type SubmsgBumpaloOwned = ::puroro::BumpaloOwned<SubmsgBumpalo<'static>>;
            impl<'bump> SubmsgBumpalo<'bump> {
                pub fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
                    #[allow(unused)]
                    let bump_ref: &::puroro::bumpalo::Bump =
                        unsafe { ::std::mem::transmute(::std::ops::Deref::deref(&bump)) };

                    Self {
                        _bump: bump,
                        _bitfield: ::std::default::Default::default(),
                        i32_unlabeled: ::std::default::Default::default(),
                        i32_optional: ::std::default::Default::default(),
                        i64_unlabeled: ::std::default::Default::default(),
                    }
                }
                pub fn i32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                    if !::puroro::internal::IsDefault::is_default(&*self.i32_unlabeled) {
                        ::std::option::Option::Some(self.i32_unlabeled.inner())
                    } else {
                        ::std::option::Option::None
                    }
                }
                pub fn i32_unlabeled<'this>(&'this self) -> i32 {
                    match self.i32_unlabeled_opt() {
                        ::std::option::Option::Some(x) => x,
                        _ => ::std::default::Default::default(),
                    }
                }

                pub fn has_i32_unlabeled(&self) -> bool {
                    self.i32_unlabeled_opt().is_some()
                }
                pub fn i32_optional_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                    if !::puroro::internal::IsDefault::is_default(&*self.i32_optional) {
                        ::std::option::Option::Some(self.i32_optional.inner())
                    } else {
                        ::std::option::Option::None
                    }
                }
                pub fn i32_optional<'this>(&'this self) -> i32 {
                    match self.i32_optional_opt() {
                        ::std::option::Option::Some(x) => x,
                        _ => ::std::default::Default::default(),
                    }
                }

                pub fn has_i32_optional(&self) -> bool {
                    self.i32_optional_opt().is_some()
                }
                pub fn i64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i64> {
                    if !::puroro::internal::IsDefault::is_default(&*self.i64_unlabeled) {
                        ::std::option::Option::Some(self.i64_unlabeled.inner())
                    } else {
                        ::std::option::Option::None
                    }
                }
                pub fn i64_unlabeled<'this>(&'this self) -> i64 {
                    match self.i64_unlabeled_opt() {
                        ::std::option::Option::Some(x) => x,
                        _ => ::std::default::Default::default(),
                    }
                }

                pub fn has_i64_unlabeled(&self) -> bool {
                    self.i64_unlabeled_opt().is_some()
                }
                pub fn clear_i32_unlabeled(&mut self) {
                    self.i32_unlabeled = ::std::default::Default::default();
                }
                pub fn i32_unlabeled_mut<'this>(&'this mut self) -> &'this mut i32 {
                    if !self.has_i32_unlabeled() {
                        self.i32_unlabeled = ::std::default::Default::default();
                    }
                    &mut self.i32_unlabeled
                }
                pub fn clear_i32_optional(&mut self) {
                    self.i32_optional = ::std::default::Default::default();
                }
                pub fn i32_optional_mut<'this>(&'this mut self) -> &'this mut i32 {
                    if !self.has_i32_optional() {
                        self.i32_optional = ::std::default::Default::default();
                    }
                    &mut self.i32_optional
                }
                pub fn clear_i64_unlabeled(&mut self) {
                    self.i64_unlabeled = ::std::default::Default::default();
                }
                pub fn i64_unlabeled_mut<'this>(&'this mut self) -> &'this mut i64 {
                    if !self.has_i64_unlabeled() {
                        self.i64_unlabeled = ::std::default::Default::default();
                    }
                    &mut self.i64_unlabeled
                }
            }
            impl<'bump> ::puroro::Message<super::_puroro_simple_impl::Submsg> for SubmsgBumpalo<'bump> {}

            impl<'bump> ::puroro::BumpaloMessage<'bump> for SubmsgBumpalo<'bump> {
                fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
                    Self::new_in(bump)
                }
            }

            impl<'bump> ::puroro::internal::BumpDefault<'bump> for SubmsgBumpalo<'bump> {
                fn default_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
                    Self::new_in(bump)
                }
            }

            impl<'bump> super::_puroro_traits::SubmsgTrait for SubmsgBumpalo<'bump> {
                fn i32_unlabeled_opt<'this>(&'this self) -> Option<i32> {
                    <Self>::i32_unlabeled_opt(self)
                }
                fn i32_optional_opt<'this>(&'this self) -> Option<i32> {
                    <Self>::i32_optional_opt(self)
                }
                fn i64_unlabeled_opt<'this>(&'this self) -> Option<i64> {
                    <Self>::i64_unlabeled_opt(self)
                }
            }

            impl<'bump> ::puroro::internal::de::DeserMessageFromBytesIter for SubmsgBumpalo<'bump> {
                fn deser_field<'this, I>(
                    &'this mut self,
                    field_number: i32,
                    data: ::puroro::internal::types::FieldData<
                        &mut ::puroro::internal::de::from_iter::ScopedIter<I>,
                    >,
                ) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    use ::puroro::internal::impls::bumpalo::de::DeserFieldFromBytesIter;
                    match field_number {
                        1 => DeserFieldFromBytesIter::<
                            ::puroro::tags::Unlabeled,
                            ::puroro::tags::Int32,
                        >::deser_field(
                            &mut self.i32_unlabeled, data, self._bump
                        ),
                        2 => DeserFieldFromBytesIter::<
                            ::puroro::tags::Unlabeled,
                            ::puroro::tags::Int32,
                        >::deser_field(
                            &mut self.i32_optional, data, self._bump
                        ),
                        101 => DeserFieldFromBytesIter::<
                            ::puroro::tags::Unlabeled,
                            ::puroro::tags::Int64,
                        >::deser_field(
                            &mut self.i64_unlabeled, data, self._bump
                        ),

                        _ => unimplemented!("TODO: This case should be handled properly..."),
                    }
                }
            }

            impl<'bump> ::puroro::internal::se::SerMessageToIoWrite for SubmsgBumpalo<'bump>
            where
                Self: super::_puroro_traits::SubmsgTrait,
            {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    ::puroro::internal::se::SerFieldToIoWrite::<
                        ::puroro::tags::Unlabeled,
                        ::puroro::tags::Int32,
                    >::ser_field(
                        <Self as super::_puroro_traits::SubmsgTrait>::i32_unlabeled_opt(self),
                        1,
                        out,
                    )?;
                    ::puroro::internal::se::SerFieldToIoWrite::<
                        ::puroro::tags::Unlabeled,
                        ::puroro::tags::Int32,
                    >::ser_field(
                        <Self as super::_puroro_traits::SubmsgTrait>::i32_optional_opt(self),
                        2,
                        out,
                    )?;
                    ::puroro::internal::se::SerFieldToIoWrite::<
                        ::puroro::tags::Unlabeled,
                        ::puroro::tags::Int64,
                    >::ser_field(
                        <Self as super::_puroro_traits::SubmsgTrait>::i64_unlabeled_opt(self),
                        101,
                        out,
                    )?;
                    ::std::result::Result::Ok(())
                }
            }
            pub struct SubmsgBuilder<T>(T);

            impl<T> SubmsgBuilder<T>
            where
                T: SubmsgTrait,
            {
                pub fn append_i32_unlabeled<ScalarType>(
                    self,
                    value: ScalarType,
                ) -> SubmsgBuilder<(T, SubmsgSingleField1<ScalarType>)>
                where
                    ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
                {
                    SubmsgBuilder((
                        self.0,
                        SubmsgSingleField1 {
                            i32_unlabeled: value,
                        },
                    ))
                }

                pub fn append_i32_optional<ScalarType>(
                    self,
                    value: ScalarType,
                ) -> SubmsgBuilder<(T, SubmsgSingleField2<ScalarType>)>
                where
                    ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
                {
                    SubmsgBuilder((
                        self.0,
                        SubmsgSingleField2 {
                            i32_optional: value,
                        },
                    ))
                }

                pub fn append_i64_unlabeled<ScalarType>(
                    self,
                    value: ScalarType,
                ) -> SubmsgBuilder<(T, SubmsgSingleField101<ScalarType>)>
                where
                    ScalarType: ::std::convert::Into<i64> + ::std::clone::Clone,
                {
                    SubmsgBuilder((
                        self.0,
                        SubmsgSingleField101 {
                            i64_unlabeled: value,
                        },
                    ))
                }

                pub fn build(self) -> T {
                    self.0
                }
            }

            impl SubmsgBuilder<()> {
                pub fn new() -> Self {
                    Self(())
                }
            }
        }
        pub use _puroro_traits::*;
        pub mod _puroro_traits {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }

            pub trait SubmsgTrait {
                fn i32_unlabeled<'this>(&'this self) -> i32 {
                    self.i32_unlabeled_opt()
                        .unwrap_or_else(::std::default::Default::default)
                }
                fn has_i32_unlabeled<'this>(&'this self) -> bool {
                    self.i32_unlabeled_opt().is_some()
                }
                fn i32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                    ::std::option::Option::None
                }
                fn i32_optional<'this>(&'this self) -> i32 {
                    self.i32_optional_opt()
                        .unwrap_or_else(::std::default::Default::default)
                }
                fn has_i32_optional<'this>(&'this self) -> bool {
                    self.i32_optional_opt().is_some()
                }
                fn i32_optional_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                    ::std::option::Option::None
                }
                fn i64_unlabeled<'this>(&'this self) -> i64 {
                    self.i64_unlabeled_opt()
                        .unwrap_or_else(::std::default::Default::default)
                }
                fn has_i64_unlabeled<'this>(&'this self) -> bool {
                    self.i64_unlabeled_opt().is_some()
                }
                fn i64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i64> {
                    ::std::option::Option::None
                }
            }

            macro_rules! submsg_delegate {
                ($ty:ty) => {
                    fn i32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                        (**self).i32_unlabeled_opt()
                    }
                    fn i32_optional_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                        (**self).i32_optional_opt()
                    }
                    fn i64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i64> {
                        (**self).i64_unlabeled_opt()
                    }
                };
            }

            impl<T> SubmsgTrait for &'_ T
            where
                T: SubmsgTrait,
            {
                submsg_delegate!(T);
            }

            impl<T> SubmsgTrait for &'_ mut T
            where
                T: SubmsgTrait,
            {
                submsg_delegate!(T);
            }

            impl<T> SubmsgTrait for ::std::boxed::Box<T>
            where
                T: SubmsgTrait,
            {
                submsg_delegate!(T);
            }

            impl<'bump, T> SubmsgTrait for ::puroro::bumpalo::boxed::Box<'bump, T>
            where
                T: SubmsgTrait,
            {
                submsg_delegate!(T);
            }

            impl<T> SubmsgTrait for ::puroro::BumpaloOwned<T>
            where
                T: SubmsgTrait,
            {
                submsg_delegate!(T);
            }
            impl SubmsgTrait for () {}
            impl<T, U> SubmsgTrait for (T, U)
            where
                T: SubmsgTrait,
                U: SubmsgTrait,
            {
                fn i32_unlabeled_opt<'this>(&'this self) -> Option<i32> {
                    <U as SubmsgTrait>::i32_unlabeled_opt(&self.1)
                        .or_else(|| <T as SubmsgTrait>::i32_unlabeled_opt(&self.0))
                }
                fn i32_optional_opt<'this>(&'this self) -> Option<i32> {
                    <U as SubmsgTrait>::i32_optional_opt(&self.1)
                        .or_else(|| <T as SubmsgTrait>::i32_optional_opt(&self.0))
                }
                fn i64_unlabeled_opt<'this>(&'this self) -> Option<i64> {
                    <U as SubmsgTrait>::i64_unlabeled_opt(&self.1)
                        .or_else(|| <T as SubmsgTrait>::i64_unlabeled_opt(&self.0))
                }
            }
            impl<T, U> SubmsgTrait for ::puroro::Either<T, U>
            where
                T: SubmsgTrait,
                U: SubmsgTrait,
            {
                fn i32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                    self.as_ref().either(
                        |t| <T as SubmsgTrait>::i32_unlabeled_opt(t),
                        |u| <U as SubmsgTrait>::i32_unlabeled_opt(u),
                    )
                }
                fn i32_optional_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                    self.as_ref().either(
                        |t| <T as SubmsgTrait>::i32_optional_opt(t),
                        |u| <U as SubmsgTrait>::i32_optional_opt(u),
                    )
                }
                fn i64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i64> {
                    self.as_ref().either(
                        |t| <T as SubmsgTrait>::i64_unlabeled_opt(t),
                        |u| <U as SubmsgTrait>::i64_unlabeled_opt(u),
                    )
                }
            }
            impl<T> SubmsgTrait for ::std::option::Option<T>
            where
                T: SubmsgTrait,
            {
                fn i32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                    self.as_ref().and_then(|msg| msg.i32_unlabeled_opt())
                }
                fn i32_optional_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                    self.as_ref().and_then(|msg| msg.i32_optional_opt())
                }
                fn i64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i64> {
                    self.as_ref().and_then(|msg| msg.i64_unlabeled_opt())
                }
            }
        }
        pub use _puroro_nested::*;
        pub mod _puroro_nested {
            pub mod submsg {
                mod _puroro_root {
                    pub use super::super::super::_puroro_root::*;
                }
            }
        }
    }
}
