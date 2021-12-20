// A generated source code by puroro library
// package proto2_defaults

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub use _puroro_simple_impl::Msg;
pub mod _puroro_simple_impl {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
    pub struct Msg {
        i32_default: ::std::option::Option<i32>,
        i32_0: ::std::option::Option<i32>,
        i32_42: ::std::option::Option<i32>,
        i32_m42: ::std::option::Option<i32>,
        i32_2147483647: ::std::option::Option<i32>,
        i32_m2147483648: ::std::option::Option<i32>,
        i32_0123: ::std::option::Option<i32>,
        i32_0x123: ::std::option::Option<i32>,
        u32_default: ::std::option::Option<u32>,
        u32_0: ::std::option::Option<u32>,
        u32_42: ::std::option::Option<u32>,
        u32_4294967295: ::std::option::Option<u32>,
        u32_0123: ::std::option::Option<u32>,
        u32_0x123: ::std::option::Option<u32>,
        i64_default: ::std::option::Option<i64>,
        i64_0: ::std::option::Option<i64>,
        i64_42: ::std::option::Option<i64>,
        i64_m42: ::std::option::Option<i64>,
        i64_9223372036854775807: ::std::option::Option<i64>,
        i64_m9223372036854775808: ::std::option::Option<i64>,
        i64_0123: ::std::option::Option<i64>,
        i64_0x123: ::std::option::Option<i64>,
        u64_default: ::std::option::Option<u64>,
        u64_0: ::std::option::Option<u64>,
        u64_42: ::std::option::Option<u64>,
        u64_18446744073709551615: ::std::option::Option<u64>,
        u64_0123: ::std::option::Option<u64>,
        u64_0x123: ::std::option::Option<u64>,
        f32_default: ::std::option::Option<f32>,
        f32_0: ::std::option::Option<f32>,
        f32_m0: ::std::option::Option<f32>,
        f32_0p: ::std::option::Option<f32>,
        f32_p0: ::std::option::Option<f32>,
        f32_0p0: ::std::option::Option<f32>,
        f32_42: ::std::option::Option<f32>,
        f32_m42: ::std::option::Option<f32>,
        f32_0p25: ::std::option::Option<f32>,
        f32_1p5e2: ::std::option::Option<f32>,
        f32_inf: ::std::option::Option<f32>,
        f32_minf: ::std::option::Option<f32>,
        f32_nan: ::std::option::Option<f32>,
        f32_mnan: ::std::option::Option<f32>,
        bool_default: ::std::option::Option<bool>,
        bool_true: ::std::option::Option<bool>,
        bool_false: ::std::option::Option<bool>,
        string_default: ::std::option::Option<::std::string::String>,
        string_empty: ::std::option::Option<::std::string::String>,
        string_abc: ::std::option::Option<::std::string::String>,
        string_aiu: ::std::option::Option<::std::string::String>,
        string_backslash: ::std::option::Option<::std::string::String>,
        string_tab: ::std::option::Option<::std::string::String>,
        string_crlf: ::std::option::Option<::std::string::String>,
        bytes_default: ::std::option::Option<::std::vec::Vec<u8>>,
        bytes_empty: ::std::option::Option<::std::vec::Vec<u8>>,
        bytes_abc: ::std::option::Option<::std::vec::Vec<u8>>,
        bytes_aiu: ::std::option::Option<::std::vec::Vec<u8>>,
        bytes_backslash: ::std::option::Option<::std::vec::Vec<u8>>,
        bytes_tab: ::std::option::Option<::std::vec::Vec<u8>>,
        bytes_crlf: ::std::option::Option<::std::vec::Vec<u8>>,
        enum_default: ::std::option::Option<self::_puroro_root::proto2_defaults::MyEnum>,
        enum_one: ::std::option::Option<self::_puroro_root::proto2_defaults::MyEnum>,
        enum_fourty_two: ::std::option::Option<self::_puroro_root::proto2_defaults::MyEnum>,
    }
    impl ::puroro::Message<Msg> for Msg {}

    impl Msg {
        pub fn new() -> Self {
            Self {
                i32_default: ::std::default::Default::default(),
                i32_0: ::std::default::Default::default(),
                i32_42: ::std::default::Default::default(),
                i32_m42: ::std::default::Default::default(),
                i32_2147483647: ::std::default::Default::default(),
                i32_m2147483648: ::std::default::Default::default(),
                i32_0123: ::std::default::Default::default(),
                i32_0x123: ::std::default::Default::default(),
                u32_default: ::std::default::Default::default(),
                u32_0: ::std::default::Default::default(),
                u32_42: ::std::default::Default::default(),
                u32_4294967295: ::std::default::Default::default(),
                u32_0123: ::std::default::Default::default(),
                u32_0x123: ::std::default::Default::default(),
                i64_default: ::std::default::Default::default(),
                i64_0: ::std::default::Default::default(),
                i64_42: ::std::default::Default::default(),
                i64_m42: ::std::default::Default::default(),
                i64_9223372036854775807: ::std::default::Default::default(),
                i64_m9223372036854775808: ::std::default::Default::default(),
                i64_0123: ::std::default::Default::default(),
                i64_0x123: ::std::default::Default::default(),
                u64_default: ::std::default::Default::default(),
                u64_0: ::std::default::Default::default(),
                u64_42: ::std::default::Default::default(),
                u64_18446744073709551615: ::std::default::Default::default(),
                u64_0123: ::std::default::Default::default(),
                u64_0x123: ::std::default::Default::default(),
                f32_default: ::std::default::Default::default(),
                f32_0: ::std::default::Default::default(),
                f32_m0: ::std::default::Default::default(),
                f32_0p: ::std::default::Default::default(),
                f32_p0: ::std::default::Default::default(),
                f32_0p0: ::std::default::Default::default(),
                f32_42: ::std::default::Default::default(),
                f32_m42: ::std::default::Default::default(),
                f32_0p25: ::std::default::Default::default(),
                f32_1p5e2: ::std::default::Default::default(),
                f32_inf: ::std::default::Default::default(),
                f32_minf: ::std::default::Default::default(),
                f32_nan: ::std::default::Default::default(),
                f32_mnan: ::std::default::Default::default(),
                bool_default: ::std::default::Default::default(),
                bool_true: ::std::default::Default::default(),
                bool_false: ::std::default::Default::default(),
                string_default: ::std::default::Default::default(),
                string_empty: ::std::default::Default::default(),
                string_abc: ::std::default::Default::default(),
                string_aiu: ::std::default::Default::default(),
                string_backslash: ::std::default::Default::default(),
                string_tab: ::std::default::Default::default(),
                string_crlf: ::std::default::Default::default(),
                bytes_default: ::std::default::Default::default(),
                bytes_empty: ::std::default::Default::default(),
                bytes_abc: ::std::default::Default::default(),
                bytes_aiu: ::std::default::Default::default(),
                bytes_backslash: ::std::default::Default::default(),
                bytes_tab: ::std::default::Default::default(),
                bytes_crlf: ::std::default::Default::default(),
                enum_default: ::std::default::Default::default(),
                enum_one: ::std::default::Default::default(),
                enum_fourty_two: ::std::default::Default::default(),
            }
        }
        pub fn i32_default_mut(&mut self) -> &mut ::std::option::Option<i32> {
            &mut self.i32_default
        }
        pub fn i32_0_mut(&mut self) -> &mut ::std::option::Option<i32> {
            &mut self.i32_0
        }
        pub fn i32_42_mut(&mut self) -> &mut ::std::option::Option<i32> {
            &mut self.i32_42
        }
        pub fn i32_m42_mut(&mut self) -> &mut ::std::option::Option<i32> {
            &mut self.i32_m42
        }
        pub fn i32_2147483647_mut(&mut self) -> &mut ::std::option::Option<i32> {
            &mut self.i32_2147483647
        }
        pub fn i32_m2147483648_mut(&mut self) -> &mut ::std::option::Option<i32> {
            &mut self.i32_m2147483648
        }
        pub fn i32_0123_mut(&mut self) -> &mut ::std::option::Option<i32> {
            &mut self.i32_0123
        }
        pub fn i32_0x123_mut(&mut self) -> &mut ::std::option::Option<i32> {
            &mut self.i32_0x123
        }
        pub fn u32_default_mut(&mut self) -> &mut ::std::option::Option<u32> {
            &mut self.u32_default
        }
        pub fn u32_0_mut(&mut self) -> &mut ::std::option::Option<u32> {
            &mut self.u32_0
        }
        pub fn u32_42_mut(&mut self) -> &mut ::std::option::Option<u32> {
            &mut self.u32_42
        }
        pub fn u32_4294967295_mut(&mut self) -> &mut ::std::option::Option<u32> {
            &mut self.u32_4294967295
        }
        pub fn u32_0123_mut(&mut self) -> &mut ::std::option::Option<u32> {
            &mut self.u32_0123
        }
        pub fn u32_0x123_mut(&mut self) -> &mut ::std::option::Option<u32> {
            &mut self.u32_0x123
        }
        pub fn i64_default_mut(&mut self) -> &mut ::std::option::Option<i64> {
            &mut self.i64_default
        }
        pub fn i64_0_mut(&mut self) -> &mut ::std::option::Option<i64> {
            &mut self.i64_0
        }
        pub fn i64_42_mut(&mut self) -> &mut ::std::option::Option<i64> {
            &mut self.i64_42
        }
        pub fn i64_m42_mut(&mut self) -> &mut ::std::option::Option<i64> {
            &mut self.i64_m42
        }
        pub fn i64_9223372036854775807_mut(&mut self) -> &mut ::std::option::Option<i64> {
            &mut self.i64_9223372036854775807
        }
        pub fn i64_m9223372036854775808_mut(&mut self) -> &mut ::std::option::Option<i64> {
            &mut self.i64_m9223372036854775808
        }
        pub fn i64_0123_mut(&mut self) -> &mut ::std::option::Option<i64> {
            &mut self.i64_0123
        }
        pub fn i64_0x123_mut(&mut self) -> &mut ::std::option::Option<i64> {
            &mut self.i64_0x123
        }
        pub fn u64_default_mut(&mut self) -> &mut ::std::option::Option<u64> {
            &mut self.u64_default
        }
        pub fn u64_0_mut(&mut self) -> &mut ::std::option::Option<u64> {
            &mut self.u64_0
        }
        pub fn u64_42_mut(&mut self) -> &mut ::std::option::Option<u64> {
            &mut self.u64_42
        }
        pub fn u64_18446744073709551615_mut(&mut self) -> &mut ::std::option::Option<u64> {
            &mut self.u64_18446744073709551615
        }
        pub fn u64_0123_mut(&mut self) -> &mut ::std::option::Option<u64> {
            &mut self.u64_0123
        }
        pub fn u64_0x123_mut(&mut self) -> &mut ::std::option::Option<u64> {
            &mut self.u64_0x123
        }
        pub fn f32_default_mut(&mut self) -> &mut ::std::option::Option<f32> {
            &mut self.f32_default
        }
        pub fn f32_0_mut(&mut self) -> &mut ::std::option::Option<f32> {
            &mut self.f32_0
        }
        pub fn f32_m0_mut(&mut self) -> &mut ::std::option::Option<f32> {
            &mut self.f32_m0
        }
        pub fn f32_0p_mut(&mut self) -> &mut ::std::option::Option<f32> {
            &mut self.f32_0p
        }
        pub fn f32_p0_mut(&mut self) -> &mut ::std::option::Option<f32> {
            &mut self.f32_p0
        }
        pub fn f32_0p0_mut(&mut self) -> &mut ::std::option::Option<f32> {
            &mut self.f32_0p0
        }
        pub fn f32_42_mut(&mut self) -> &mut ::std::option::Option<f32> {
            &mut self.f32_42
        }
        pub fn f32_m42_mut(&mut self) -> &mut ::std::option::Option<f32> {
            &mut self.f32_m42
        }
        pub fn f32_0p25_mut(&mut self) -> &mut ::std::option::Option<f32> {
            &mut self.f32_0p25
        }
        pub fn f32_1p5e2_mut(&mut self) -> &mut ::std::option::Option<f32> {
            &mut self.f32_1p5e2
        }
        pub fn f32_inf_mut(&mut self) -> &mut ::std::option::Option<f32> {
            &mut self.f32_inf
        }
        pub fn f32_minf_mut(&mut self) -> &mut ::std::option::Option<f32> {
            &mut self.f32_minf
        }
        pub fn f32_nan_mut(&mut self) -> &mut ::std::option::Option<f32> {
            &mut self.f32_nan
        }
        pub fn f32_mnan_mut(&mut self) -> &mut ::std::option::Option<f32> {
            &mut self.f32_mnan
        }
        pub fn bool_default_mut(&mut self) -> &mut ::std::option::Option<bool> {
            &mut self.bool_default
        }
        pub fn bool_true_mut(&mut self) -> &mut ::std::option::Option<bool> {
            &mut self.bool_true
        }
        pub fn bool_false_mut(&mut self) -> &mut ::std::option::Option<bool> {
            &mut self.bool_false
        }
        pub fn string_default_mut(&mut self) -> &mut ::std::option::Option<::std::string::String> {
            &mut self.string_default
        }
        pub fn string_empty_mut(&mut self) -> &mut ::std::option::Option<::std::string::String> {
            &mut self.string_empty
        }
        pub fn string_abc_mut(&mut self) -> &mut ::std::option::Option<::std::string::String> {
            &mut self.string_abc
        }
        pub fn string_aiu_mut(&mut self) -> &mut ::std::option::Option<::std::string::String> {
            &mut self.string_aiu
        }
        pub fn string_backslash_mut(
            &mut self,
        ) -> &mut ::std::option::Option<::std::string::String> {
            &mut self.string_backslash
        }
        pub fn string_tab_mut(&mut self) -> &mut ::std::option::Option<::std::string::String> {
            &mut self.string_tab
        }
        pub fn string_crlf_mut(&mut self) -> &mut ::std::option::Option<::std::string::String> {
            &mut self.string_crlf
        }
        pub fn bytes_default_mut(&mut self) -> &mut ::std::option::Option<::std::vec::Vec<u8>> {
            &mut self.bytes_default
        }
        pub fn bytes_empty_mut(&mut self) -> &mut ::std::option::Option<::std::vec::Vec<u8>> {
            &mut self.bytes_empty
        }
        pub fn bytes_abc_mut(&mut self) -> &mut ::std::option::Option<::std::vec::Vec<u8>> {
            &mut self.bytes_abc
        }
        pub fn bytes_aiu_mut(&mut self) -> &mut ::std::option::Option<::std::vec::Vec<u8>> {
            &mut self.bytes_aiu
        }
        pub fn bytes_backslash_mut(&mut self) -> &mut ::std::option::Option<::std::vec::Vec<u8>> {
            &mut self.bytes_backslash
        }
        pub fn bytes_tab_mut(&mut self) -> &mut ::std::option::Option<::std::vec::Vec<u8>> {
            &mut self.bytes_tab
        }
        pub fn bytes_crlf_mut(&mut self) -> &mut ::std::option::Option<::std::vec::Vec<u8>> {
            &mut self.bytes_crlf
        }
        pub fn enum_default_mut(
            &mut self,
        ) -> &mut ::std::option::Option<self::_puroro_root::proto2_defaults::MyEnum> {
            &mut self.enum_default
        }
        pub fn enum_one_mut(
            &mut self,
        ) -> &mut ::std::option::Option<self::_puroro_root::proto2_defaults::MyEnum> {
            &mut self.enum_one
        }
        pub fn enum_fourty_two_mut(
            &mut self,
        ) -> &mut ::std::option::Option<self::_puroro_root::proto2_defaults::MyEnum> {
            &mut self.enum_fourty_two
        }
    }

    impl super::_puroro_traits::MsgTrait for Msg {
        fn i32_default_opt<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.i32_default)
        }
        fn i32_0_opt<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.i32_0)
        }
        fn i32_42_opt<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.i32_42)
        }
        fn i32_m42_opt<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.i32_m42)
        }
        fn i32_2147483647_opt<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.i32_2147483647)
        }
        fn i32_m2147483648_opt<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.i32_m2147483648)
        }
        fn i32_0123_opt<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.i32_0123)
        }
        fn i32_0x123_opt<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.i32_0x123)
        }
        fn u32_default_opt<'this>(&'this self) -> Option<u32> {
            Clone::clone(&self.u32_default)
        }
        fn u32_0_opt<'this>(&'this self) -> Option<u32> {
            Clone::clone(&self.u32_0)
        }
        fn u32_42_opt<'this>(&'this self) -> Option<u32> {
            Clone::clone(&self.u32_42)
        }
        fn u32_4294967295_opt<'this>(&'this self) -> Option<u32> {
            Clone::clone(&self.u32_4294967295)
        }
        fn u32_0123_opt<'this>(&'this self) -> Option<u32> {
            Clone::clone(&self.u32_0123)
        }
        fn u32_0x123_opt<'this>(&'this self) -> Option<u32> {
            Clone::clone(&self.u32_0x123)
        }
        fn i64_default_opt<'this>(&'this self) -> Option<i64> {
            Clone::clone(&self.i64_default)
        }
        fn i64_0_opt<'this>(&'this self) -> Option<i64> {
            Clone::clone(&self.i64_0)
        }
        fn i64_42_opt<'this>(&'this self) -> Option<i64> {
            Clone::clone(&self.i64_42)
        }
        fn i64_m42_opt<'this>(&'this self) -> Option<i64> {
            Clone::clone(&self.i64_m42)
        }
        fn i64_9223372036854775807_opt<'this>(&'this self) -> Option<i64> {
            Clone::clone(&self.i64_9223372036854775807)
        }
        fn i64_m9223372036854775808_opt<'this>(&'this self) -> Option<i64> {
            Clone::clone(&self.i64_m9223372036854775808)
        }
        fn i64_0123_opt<'this>(&'this self) -> Option<i64> {
            Clone::clone(&self.i64_0123)
        }
        fn i64_0x123_opt<'this>(&'this self) -> Option<i64> {
            Clone::clone(&self.i64_0x123)
        }
        fn u64_default_opt<'this>(&'this self) -> Option<u64> {
            Clone::clone(&self.u64_default)
        }
        fn u64_0_opt<'this>(&'this self) -> Option<u64> {
            Clone::clone(&self.u64_0)
        }
        fn u64_42_opt<'this>(&'this self) -> Option<u64> {
            Clone::clone(&self.u64_42)
        }
        fn u64_18446744073709551615_opt<'this>(&'this self) -> Option<u64> {
            Clone::clone(&self.u64_18446744073709551615)
        }
        fn u64_0123_opt<'this>(&'this self) -> Option<u64> {
            Clone::clone(&self.u64_0123)
        }
        fn u64_0x123_opt<'this>(&'this self) -> Option<u64> {
            Clone::clone(&self.u64_0x123)
        }
        fn f32_default_opt<'this>(&'this self) -> Option<f32> {
            Clone::clone(&self.f32_default)
        }
        fn f32_0_opt<'this>(&'this self) -> Option<f32> {
            Clone::clone(&self.f32_0)
        }
        fn f32_m0_opt<'this>(&'this self) -> Option<f32> {
            Clone::clone(&self.f32_m0)
        }
        fn f32_0p_opt<'this>(&'this self) -> Option<f32> {
            Clone::clone(&self.f32_0p)
        }
        fn f32_p0_opt<'this>(&'this self) -> Option<f32> {
            Clone::clone(&self.f32_p0)
        }
        fn f32_0p0_opt<'this>(&'this self) -> Option<f32> {
            Clone::clone(&self.f32_0p0)
        }
        fn f32_42_opt<'this>(&'this self) -> Option<f32> {
            Clone::clone(&self.f32_42)
        }
        fn f32_m42_opt<'this>(&'this self) -> Option<f32> {
            Clone::clone(&self.f32_m42)
        }
        fn f32_0p25_opt<'this>(&'this self) -> Option<f32> {
            Clone::clone(&self.f32_0p25)
        }
        fn f32_1p5e2_opt<'this>(&'this self) -> Option<f32> {
            Clone::clone(&self.f32_1p5e2)
        }
        fn f32_inf_opt<'this>(&'this self) -> Option<f32> {
            Clone::clone(&self.f32_inf)
        }
        fn f32_minf_opt<'this>(&'this self) -> Option<f32> {
            Clone::clone(&self.f32_minf)
        }
        fn f32_nan_opt<'this>(&'this self) -> Option<f32> {
            Clone::clone(&self.f32_nan)
        }
        fn f32_mnan_opt<'this>(&'this self) -> Option<f32> {
            Clone::clone(&self.f32_mnan)
        }
        fn bool_default_opt<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.bool_default)
        }
        fn bool_true_opt<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.bool_true)
        }
        fn bool_false_opt<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.bool_false)
        }
        type Field71ScalarGetterType<'this>
        where
            Self: 'this,
        = &'this ::std::string::String;
        fn string_default_opt<'this>(&'this self) -> Option<Self::Field71ScalarGetterType<'this>> {
            self.string_default.as_ref().map(|v| v.as_ref())
        }
        type Field72ScalarGetterType<'this>
        where
            Self: 'this,
        = &'this ::std::string::String;
        fn string_empty_opt<'this>(&'this self) -> Option<Self::Field72ScalarGetterType<'this>> {
            self.string_empty.as_ref().map(|v| v.as_ref())
        }
        type Field73ScalarGetterType<'this>
        where
            Self: 'this,
        = &'this ::std::string::String;
        fn string_abc_opt<'this>(&'this self) -> Option<Self::Field73ScalarGetterType<'this>> {
            self.string_abc.as_ref().map(|v| v.as_ref())
        }
        type Field74ScalarGetterType<'this>
        where
            Self: 'this,
        = &'this ::std::string::String;
        fn string_aiu_opt<'this>(&'this self) -> Option<Self::Field74ScalarGetterType<'this>> {
            self.string_aiu.as_ref().map(|v| v.as_ref())
        }
        type Field75ScalarGetterType<'this>
        where
            Self: 'this,
        = &'this ::std::string::String;
        fn string_backslash_opt<'this>(
            &'this self,
        ) -> Option<Self::Field75ScalarGetterType<'this>> {
            self.string_backslash.as_ref().map(|v| v.as_ref())
        }
        type Field76ScalarGetterType<'this>
        where
            Self: 'this,
        = &'this ::std::string::String;
        fn string_tab_opt<'this>(&'this self) -> Option<Self::Field76ScalarGetterType<'this>> {
            self.string_tab.as_ref().map(|v| v.as_ref())
        }
        type Field77ScalarGetterType<'this>
        where
            Self: 'this,
        = &'this ::std::string::String;
        fn string_crlf_opt<'this>(&'this self) -> Option<Self::Field77ScalarGetterType<'this>> {
            self.string_crlf.as_ref().map(|v| v.as_ref())
        }
        type Field81ScalarGetterType<'this>
        where
            Self: 'this,
        = &'this ::std::vec::Vec<u8>;
        fn bytes_default_opt<'this>(&'this self) -> Option<Self::Field81ScalarGetterType<'this>> {
            self.bytes_default.as_ref().map(|v| v.as_ref())
        }
        type Field82ScalarGetterType<'this>
        where
            Self: 'this,
        = &'this ::std::vec::Vec<u8>;
        fn bytes_empty_opt<'this>(&'this self) -> Option<Self::Field82ScalarGetterType<'this>> {
            self.bytes_empty.as_ref().map(|v| v.as_ref())
        }
        type Field83ScalarGetterType<'this>
        where
            Self: 'this,
        = &'this ::std::vec::Vec<u8>;
        fn bytes_abc_opt<'this>(&'this self) -> Option<Self::Field83ScalarGetterType<'this>> {
            self.bytes_abc.as_ref().map(|v| v.as_ref())
        }
        type Field84ScalarGetterType<'this>
        where
            Self: 'this,
        = &'this ::std::vec::Vec<u8>;
        fn bytes_aiu_opt<'this>(&'this self) -> Option<Self::Field84ScalarGetterType<'this>> {
            self.bytes_aiu.as_ref().map(|v| v.as_ref())
        }
        type Field85ScalarGetterType<'this>
        where
            Self: 'this,
        = &'this ::std::vec::Vec<u8>;
        fn bytes_backslash_opt<'this>(&'this self) -> Option<Self::Field85ScalarGetterType<'this>> {
            self.bytes_backslash.as_ref().map(|v| v.as_ref())
        }
        type Field86ScalarGetterType<'this>
        where
            Self: 'this,
        = &'this ::std::vec::Vec<u8>;
        fn bytes_tab_opt<'this>(&'this self) -> Option<Self::Field86ScalarGetterType<'this>> {
            self.bytes_tab.as_ref().map(|v| v.as_ref())
        }
        type Field87ScalarGetterType<'this>
        where
            Self: 'this,
        = &'this ::std::vec::Vec<u8>;
        fn bytes_crlf_opt<'this>(&'this self) -> Option<Self::Field87ScalarGetterType<'this>> {
            self.bytes_crlf.as_ref().map(|v| v.as_ref())
        }
        fn enum_default_opt<'this>(
            &'this self,
        ) -> Option<self::_puroro_root::proto2_defaults::MyEnum> {
            Clone::clone(&self.enum_default)
        }
        fn enum_one_opt<'this>(&'this self) -> Option<self::_puroro_root::proto2_defaults::MyEnum> {
            Clone::clone(&self.enum_one)
        }
        fn enum_fourty_two_opt<'this>(
            &'this self,
        ) -> Option<self::_puroro_root::proto2_defaults::MyEnum> {
            Clone::clone(&self.enum_fourty_two)
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
            1 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Int32
            >::deser_field(&mut self.i32_default, data),
            2 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Int32
            >::deser_field(&mut self.i32_0, data),
            3 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Int32
            >::deser_field(&mut self.i32_42, data),
            4 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Int32
            >::deser_field(&mut self.i32_m42, data),
            5 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Int32
            >::deser_field(&mut self.i32_2147483647, data),
            6 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Int32
            >::deser_field(&mut self.i32_m2147483648, data),
            7 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Int32
            >::deser_field(&mut self.i32_0123, data),
            8 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Int32
            >::deser_field(&mut self.i32_0x123, data),
            11 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::UInt32
            >::deser_field(&mut self.u32_default, data),
            12 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::UInt32
            >::deser_field(&mut self.u32_0, data),
            13 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::UInt32
            >::deser_field(&mut self.u32_42, data),
            15 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::UInt32
            >::deser_field(&mut self.u32_4294967295, data),
            17 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::UInt32
            >::deser_field(&mut self.u32_0123, data),
            18 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::UInt32
            >::deser_field(&mut self.u32_0x123, data),
            21 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Int64
            >::deser_field(&mut self.i64_default, data),
            22 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Int64
            >::deser_field(&mut self.i64_0, data),
            23 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Int64
            >::deser_field(&mut self.i64_42, data),
            24 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Int64
            >::deser_field(&mut self.i64_m42, data),
            25 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Int64
            >::deser_field(&mut self.i64_9223372036854775807, data),
            26 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Int64
            >::deser_field(&mut self.i64_m9223372036854775808, data),
            27 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Int64
            >::deser_field(&mut self.i64_0123, data),
            28 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Int64
            >::deser_field(&mut self.i64_0x123, data),
            31 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::UInt64
            >::deser_field(&mut self.u64_default, data),
            32 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::UInt64
            >::deser_field(&mut self.u64_0, data),
            33 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::UInt64
            >::deser_field(&mut self.u64_42, data),
            35 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::UInt64
            >::deser_field(&mut self.u64_18446744073709551615, data),
            37 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::UInt64
            >::deser_field(&mut self.u64_0123, data),
            38 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::UInt64
            >::deser_field(&mut self.u64_0x123, data),
            41 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Float
            >::deser_field(&mut self.f32_default, data),
            42 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Float
            >::deser_field(&mut self.f32_0, data),
            43 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Float
            >::deser_field(&mut self.f32_m0, data),
            44 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Float
            >::deser_field(&mut self.f32_0p, data),
            45 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Float
            >::deser_field(&mut self.f32_p0, data),
            46 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Float
            >::deser_field(&mut self.f32_0p0, data),
            47 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Float
            >::deser_field(&mut self.f32_42, data),
            48 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Float
            >::deser_field(&mut self.f32_m42, data),
            49 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Float
            >::deser_field(&mut self.f32_0p25, data),
            50 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Float
            >::deser_field(&mut self.f32_1p5e2, data),
            51 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Float
            >::deser_field(&mut self.f32_inf, data),
            52 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Float
            >::deser_field(&mut self.f32_minf, data),
            53 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Float
            >::deser_field(&mut self.f32_nan, data),
            54 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Float
            >::deser_field(&mut self.f32_mnan, data),
            61 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.bool_default, data),
            62 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.bool_true, data),
            63 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.bool_false, data),
            71 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.string_default, data),
            72 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.string_empty, data),
            73 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.string_abc, data),
            74 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.string_aiu, data),
            75 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.string_backslash, data),
            76 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.string_tab, data),
            77 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.string_crlf, data),
            81 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bytes
            >::deser_field(&mut self.bytes_default, data),
            82 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bytes
            >::deser_field(&mut self.bytes_empty, data),
            83 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bytes
            >::deser_field(&mut self.bytes_abc, data),
            84 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bytes
            >::deser_field(&mut self.bytes_aiu, data),
            85 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bytes
            >::deser_field(&mut self.bytes_backslash, data),
            86 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bytes
            >::deser_field(&mut self.bytes_tab, data),
            87 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bytes
            >::deser_field(&mut self.bytes_crlf, data),
            91 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Enum2<self::_puroro_root::proto2_defaults::MyEnum>
            >::deser_field(&mut self.enum_default, data),
            92 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Enum2<self::_puroro_root::proto2_defaults::MyEnum>
            >::deser_field(&mut self.enum_one, data),
            93 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Enum2<self::_puroro_root::proto2_defaults::MyEnum>
            >::deser_field(&mut self.enum_fourty_two, data),

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
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::i32_default_opt(self),
                1,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::i32_0_opt(self),
                2,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::i32_42_opt(self),
                3,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::i32_m42_opt(self),
                4,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::i32_2147483647_opt(self),
                5,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::i32_m2147483648_opt(self),
                6,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::i32_0123_opt(self),
                7,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::i32_0x123_opt(self),
                8,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::UInt32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::u32_default_opt(self),
                11,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::UInt32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::u32_0_opt(self),
                12,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::UInt32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::u32_42_opt(self),
                13,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::UInt32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::u32_4294967295_opt(self),
                15,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::UInt32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::u32_0123_opt(self),
                17,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::UInt32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::u32_0x123_opt(self),
                18,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::i64_default_opt(self),
                21,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::i64_0_opt(self),
                22,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::i64_42_opt(self),
                23,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::i64_m42_opt(self),
                24,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::i64_9223372036854775807_opt(self),
                25,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::i64_m9223372036854775808_opt(self),
                26,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::i64_0123_opt(self),
                27,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::i64_0x123_opt(self),
                28,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::UInt64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::u64_default_opt(self),
                31,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::UInt64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::u64_0_opt(self),
                32,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::UInt64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::u64_42_opt(self),
                33,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::UInt64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::u64_18446744073709551615_opt(self),
                35,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::UInt64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::u64_0123_opt(self),
                37,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::UInt64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::u64_0x123_opt(self),
                38,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Float,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::f32_default_opt(self),
                41,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Float,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::f32_0_opt(self),
                42,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Float,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::f32_m0_opt(self),
                43,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Float,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::f32_0p_opt(self),
                44,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Float,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::f32_p0_opt(self),
                45,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Float,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::f32_0p0_opt(self),
                46,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Float,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::f32_42_opt(self),
                47,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Float,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::f32_m42_opt(self),
                48,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Float,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::f32_0p25_opt(self),
                49,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Float,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::f32_1p5e2_opt(self),
                50,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Float,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::f32_inf_opt(self),
                51,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Float,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::f32_minf_opt(self),
                52,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Float,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::f32_nan_opt(self),
                53,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Float,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::f32_mnan_opt(self),
                54,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::bool_default_opt(self),
                61,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::bool_true_opt(self),
                62,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::bool_false_opt(self),
                63,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::string_default_opt(self),
                71,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::string_empty_opt(self),
                72,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::string_abc_opt(self),
                73,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::string_aiu_opt(self),
                74,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::string_backslash_opt(self),
                75,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::string_tab_opt(self),
                76,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::string_crlf_opt(self),
                77,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bytes,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::bytes_default_opt(self),
                81,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bytes,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::bytes_empty_opt(self),
                82,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bytes,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::bytes_abc_opt(self),
                83,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bytes,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::bytes_aiu_opt(self),
                84,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bytes,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::bytes_backslash_opt(self),
                85,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bytes,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::bytes_tab_opt(self),
                86,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bytes,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::bytes_crlf_opt(self),
                87,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Enum2<self::_puroro_root::proto2_defaults::MyEnum>,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::enum_default_opt(self),
                91,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Enum2<self::_puroro_root::proto2_defaults::MyEnum>,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::enum_one_opt(self),
                92,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Enum2<self::_puroro_root::proto2_defaults::MyEnum>,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::enum_fourty_two_opt(self),
                93,
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
                .field(
                    "i32_default",
                    &<Self as super::_puroro_traits::MsgTrait>::i32_default_opt(self),
                )
                .field(
                    "i32_0",
                    &<Self as super::_puroro_traits::MsgTrait>::i32_0_opt(self),
                )
                .field(
                    "i32_42",
                    &<Self as super::_puroro_traits::MsgTrait>::i32_42_opt(self),
                )
                .field(
                    "i32_m42",
                    &<Self as super::_puroro_traits::MsgTrait>::i32_m42_opt(self),
                )
                .field(
                    "i32_2147483647",
                    &<Self as super::_puroro_traits::MsgTrait>::i32_2147483647_opt(self),
                )
                .field(
                    "i32_m2147483648",
                    &<Self as super::_puroro_traits::MsgTrait>::i32_m2147483648_opt(self),
                )
                .field(
                    "i32_0123",
                    &<Self as super::_puroro_traits::MsgTrait>::i32_0123_opt(self),
                )
                .field(
                    "i32_0x123",
                    &<Self as super::_puroro_traits::MsgTrait>::i32_0x123_opt(self),
                )
                .field(
                    "u32_default",
                    &<Self as super::_puroro_traits::MsgTrait>::u32_default_opt(self),
                )
                .field(
                    "u32_0",
                    &<Self as super::_puroro_traits::MsgTrait>::u32_0_opt(self),
                )
                .field(
                    "u32_42",
                    &<Self as super::_puroro_traits::MsgTrait>::u32_42_opt(self),
                )
                .field(
                    "u32_4294967295",
                    &<Self as super::_puroro_traits::MsgTrait>::u32_4294967295_opt(self),
                )
                .field(
                    "u32_0123",
                    &<Self as super::_puroro_traits::MsgTrait>::u32_0123_opt(self),
                )
                .field(
                    "u32_0x123",
                    &<Self as super::_puroro_traits::MsgTrait>::u32_0x123_opt(self),
                )
                .field(
                    "i64_default",
                    &<Self as super::_puroro_traits::MsgTrait>::i64_default_opt(self),
                )
                .field(
                    "i64_0",
                    &<Self as super::_puroro_traits::MsgTrait>::i64_0_opt(self),
                )
                .field(
                    "i64_42",
                    &<Self as super::_puroro_traits::MsgTrait>::i64_42_opt(self),
                )
                .field(
                    "i64_m42",
                    &<Self as super::_puroro_traits::MsgTrait>::i64_m42_opt(self),
                )
                .field(
                    "i64_9223372036854775807",
                    &<Self as super::_puroro_traits::MsgTrait>::i64_9223372036854775807_opt(self),
                )
                .field(
                    "i64_m9223372036854775808",
                    &<Self as super::_puroro_traits::MsgTrait>::i64_m9223372036854775808_opt(self),
                )
                .field(
                    "i64_0123",
                    &<Self as super::_puroro_traits::MsgTrait>::i64_0123_opt(self),
                )
                .field(
                    "i64_0x123",
                    &<Self as super::_puroro_traits::MsgTrait>::i64_0x123_opt(self),
                )
                .field(
                    "u64_default",
                    &<Self as super::_puroro_traits::MsgTrait>::u64_default_opt(self),
                )
                .field(
                    "u64_0",
                    &<Self as super::_puroro_traits::MsgTrait>::u64_0_opt(self),
                )
                .field(
                    "u64_42",
                    &<Self as super::_puroro_traits::MsgTrait>::u64_42_opt(self),
                )
                .field(
                    "u64_18446744073709551615",
                    &<Self as super::_puroro_traits::MsgTrait>::u64_18446744073709551615_opt(self),
                )
                .field(
                    "u64_0123",
                    &<Self as super::_puroro_traits::MsgTrait>::u64_0123_opt(self),
                )
                .field(
                    "u64_0x123",
                    &<Self as super::_puroro_traits::MsgTrait>::u64_0x123_opt(self),
                )
                .field(
                    "f32_default",
                    &<Self as super::_puroro_traits::MsgTrait>::f32_default_opt(self),
                )
                .field(
                    "f32_0",
                    &<Self as super::_puroro_traits::MsgTrait>::f32_0_opt(self),
                )
                .field(
                    "f32_m0",
                    &<Self as super::_puroro_traits::MsgTrait>::f32_m0_opt(self),
                )
                .field(
                    "f32_0p",
                    &<Self as super::_puroro_traits::MsgTrait>::f32_0p_opt(self),
                )
                .field(
                    "f32_p0",
                    &<Self as super::_puroro_traits::MsgTrait>::f32_p0_opt(self),
                )
                .field(
                    "f32_0p0",
                    &<Self as super::_puroro_traits::MsgTrait>::f32_0p0_opt(self),
                )
                .field(
                    "f32_42",
                    &<Self as super::_puroro_traits::MsgTrait>::f32_42_opt(self),
                )
                .field(
                    "f32_m42",
                    &<Self as super::_puroro_traits::MsgTrait>::f32_m42_opt(self),
                )
                .field(
                    "f32_0p25",
                    &<Self as super::_puroro_traits::MsgTrait>::f32_0p25_opt(self),
                )
                .field(
                    "f32_1p5e2",
                    &<Self as super::_puroro_traits::MsgTrait>::f32_1p5e2_opt(self),
                )
                .field(
                    "f32_inf",
                    &<Self as super::_puroro_traits::MsgTrait>::f32_inf_opt(self),
                )
                .field(
                    "f32_minf",
                    &<Self as super::_puroro_traits::MsgTrait>::f32_minf_opt(self),
                )
                .field(
                    "f32_nan",
                    &<Self as super::_puroro_traits::MsgTrait>::f32_nan_opt(self),
                )
                .field(
                    "f32_mnan",
                    &<Self as super::_puroro_traits::MsgTrait>::f32_mnan_opt(self),
                )
                .field(
                    "bool_default",
                    &<Self as super::_puroro_traits::MsgTrait>::bool_default_opt(self),
                )
                .field(
                    "bool_true",
                    &<Self as super::_puroro_traits::MsgTrait>::bool_true_opt(self),
                )
                .field(
                    "bool_false",
                    &<Self as super::_puroro_traits::MsgTrait>::bool_false_opt(self),
                )
                .field(
                    "string_default",
                    &<Self as super::_puroro_traits::MsgTrait>::string_default_opt(self),
                )
                .field(
                    "string_empty",
                    &<Self as super::_puroro_traits::MsgTrait>::string_empty_opt(self),
                )
                .field(
                    "string_abc",
                    &<Self as super::_puroro_traits::MsgTrait>::string_abc_opt(self),
                )
                .field(
                    "string_aiu",
                    &<Self as super::_puroro_traits::MsgTrait>::string_aiu_opt(self),
                )
                .field(
                    "string_backslash",
                    &<Self as super::_puroro_traits::MsgTrait>::string_backslash_opt(self),
                )
                .field(
                    "string_tab",
                    &<Self as super::_puroro_traits::MsgTrait>::string_tab_opt(self),
                )
                .field(
                    "string_crlf",
                    &<Self as super::_puroro_traits::MsgTrait>::string_crlf_opt(self),
                )
                .field(
                    "bytes_default",
                    &<Self as super::_puroro_traits::MsgTrait>::bytes_default_opt(self),
                )
                .field(
                    "bytes_empty",
                    &<Self as super::_puroro_traits::MsgTrait>::bytes_empty_opt(self),
                )
                .field(
                    "bytes_abc",
                    &<Self as super::_puroro_traits::MsgTrait>::bytes_abc_opt(self),
                )
                .field(
                    "bytes_aiu",
                    &<Self as super::_puroro_traits::MsgTrait>::bytes_aiu_opt(self),
                )
                .field(
                    "bytes_backslash",
                    &<Self as super::_puroro_traits::MsgTrait>::bytes_backslash_opt(self),
                )
                .field(
                    "bytes_tab",
                    &<Self as super::_puroro_traits::MsgTrait>::bytes_tab_opt(self),
                )
                .field(
                    "bytes_crlf",
                    &<Self as super::_puroro_traits::MsgTrait>::bytes_crlf_opt(self),
                )
                .field(
                    "enum_default",
                    &<Self as super::_puroro_traits::MsgTrait>::enum_default_opt(self),
                )
                .field(
                    "enum_one",
                    &<Self as super::_puroro_traits::MsgTrait>::enum_one_opt(self),
                )
                .field(
                    "enum_fourty_two",
                    &<Self as super::_puroro_traits::MsgTrait>::enum_fourty_two_opt(self),
                )
                .finish()
        }
    }

    impl ::std::clone::Clone for Msg {
        fn clone(&self) -> Self {
            Self {
                i32_default: ::std::clone::Clone::clone(&self.i32_default),
                i32_0: ::std::clone::Clone::clone(&self.i32_0),
                i32_42: ::std::clone::Clone::clone(&self.i32_42),
                i32_m42: ::std::clone::Clone::clone(&self.i32_m42),
                i32_2147483647: ::std::clone::Clone::clone(&self.i32_2147483647),
                i32_m2147483648: ::std::clone::Clone::clone(&self.i32_m2147483648),
                i32_0123: ::std::clone::Clone::clone(&self.i32_0123),
                i32_0x123: ::std::clone::Clone::clone(&self.i32_0x123),
                u32_default: ::std::clone::Clone::clone(&self.u32_default),
                u32_0: ::std::clone::Clone::clone(&self.u32_0),
                u32_42: ::std::clone::Clone::clone(&self.u32_42),
                u32_4294967295: ::std::clone::Clone::clone(&self.u32_4294967295),
                u32_0123: ::std::clone::Clone::clone(&self.u32_0123),
                u32_0x123: ::std::clone::Clone::clone(&self.u32_0x123),
                i64_default: ::std::clone::Clone::clone(&self.i64_default),
                i64_0: ::std::clone::Clone::clone(&self.i64_0),
                i64_42: ::std::clone::Clone::clone(&self.i64_42),
                i64_m42: ::std::clone::Clone::clone(&self.i64_m42),
                i64_9223372036854775807: ::std::clone::Clone::clone(&self.i64_9223372036854775807),
                i64_m9223372036854775808: ::std::clone::Clone::clone(
                    &self.i64_m9223372036854775808,
                ),
                i64_0123: ::std::clone::Clone::clone(&self.i64_0123),
                i64_0x123: ::std::clone::Clone::clone(&self.i64_0x123),
                u64_default: ::std::clone::Clone::clone(&self.u64_default),
                u64_0: ::std::clone::Clone::clone(&self.u64_0),
                u64_42: ::std::clone::Clone::clone(&self.u64_42),
                u64_18446744073709551615: ::std::clone::Clone::clone(
                    &self.u64_18446744073709551615,
                ),
                u64_0123: ::std::clone::Clone::clone(&self.u64_0123),
                u64_0x123: ::std::clone::Clone::clone(&self.u64_0x123),
                f32_default: ::std::clone::Clone::clone(&self.f32_default),
                f32_0: ::std::clone::Clone::clone(&self.f32_0),
                f32_m0: ::std::clone::Clone::clone(&self.f32_m0),
                f32_0p: ::std::clone::Clone::clone(&self.f32_0p),
                f32_p0: ::std::clone::Clone::clone(&self.f32_p0),
                f32_0p0: ::std::clone::Clone::clone(&self.f32_0p0),
                f32_42: ::std::clone::Clone::clone(&self.f32_42),
                f32_m42: ::std::clone::Clone::clone(&self.f32_m42),
                f32_0p25: ::std::clone::Clone::clone(&self.f32_0p25),
                f32_1p5e2: ::std::clone::Clone::clone(&self.f32_1p5e2),
                f32_inf: ::std::clone::Clone::clone(&self.f32_inf),
                f32_minf: ::std::clone::Clone::clone(&self.f32_minf),
                f32_nan: ::std::clone::Clone::clone(&self.f32_nan),
                f32_mnan: ::std::clone::Clone::clone(&self.f32_mnan),
                bool_default: ::std::clone::Clone::clone(&self.bool_default),
                bool_true: ::std::clone::Clone::clone(&self.bool_true),
                bool_false: ::std::clone::Clone::clone(&self.bool_false),
                string_default: ::std::clone::Clone::clone(&self.string_default),
                string_empty: ::std::clone::Clone::clone(&self.string_empty),
                string_abc: ::std::clone::Clone::clone(&self.string_abc),
                string_aiu: ::std::clone::Clone::clone(&self.string_aiu),
                string_backslash: ::std::clone::Clone::clone(&self.string_backslash),
                string_tab: ::std::clone::Clone::clone(&self.string_tab),
                string_crlf: ::std::clone::Clone::clone(&self.string_crlf),
                bytes_default: ::std::clone::Clone::clone(&self.bytes_default),
                bytes_empty: ::std::clone::Clone::clone(&self.bytes_empty),
                bytes_abc: ::std::clone::Clone::clone(&self.bytes_abc),
                bytes_aiu: ::std::clone::Clone::clone(&self.bytes_aiu),
                bytes_backslash: ::std::clone::Clone::clone(&self.bytes_backslash),
                bytes_tab: ::std::clone::Clone::clone(&self.bytes_tab),
                bytes_crlf: ::std::clone::Clone::clone(&self.bytes_crlf),
                enum_default: ::std::clone::Clone::clone(&self.enum_default),
                enum_one: ::std::clone::Clone::clone(&self.enum_one),
                enum_fourty_two: ::std::clone::Clone::clone(&self.enum_fourty_two),
            }
        }
    }

    impl ::std::cmp::PartialEq for Msg {
        fn eq(&self, rhs: &Self) -> bool {
            self.i32_default == rhs.i32_default
                && self.i32_0 == rhs.i32_0
                && self.i32_42 == rhs.i32_42
                && self.i32_m42 == rhs.i32_m42
                && self.i32_2147483647 == rhs.i32_2147483647
                && self.i32_m2147483648 == rhs.i32_m2147483648
                && self.i32_0123 == rhs.i32_0123
                && self.i32_0x123 == rhs.i32_0x123
                && self.u32_default == rhs.u32_default
                && self.u32_0 == rhs.u32_0
                && self.u32_42 == rhs.u32_42
                && self.u32_4294967295 == rhs.u32_4294967295
                && self.u32_0123 == rhs.u32_0123
                && self.u32_0x123 == rhs.u32_0x123
                && self.i64_default == rhs.i64_default
                && self.i64_0 == rhs.i64_0
                && self.i64_42 == rhs.i64_42
                && self.i64_m42 == rhs.i64_m42
                && self.i64_9223372036854775807 == rhs.i64_9223372036854775807
                && self.i64_m9223372036854775808 == rhs.i64_m9223372036854775808
                && self.i64_0123 == rhs.i64_0123
                && self.i64_0x123 == rhs.i64_0x123
                && self.u64_default == rhs.u64_default
                && self.u64_0 == rhs.u64_0
                && self.u64_42 == rhs.u64_42
                && self.u64_18446744073709551615 == rhs.u64_18446744073709551615
                && self.u64_0123 == rhs.u64_0123
                && self.u64_0x123 == rhs.u64_0x123
                && self.f32_default == rhs.f32_default
                && self.f32_0 == rhs.f32_0
                && self.f32_m0 == rhs.f32_m0
                && self.f32_0p == rhs.f32_0p
                && self.f32_p0 == rhs.f32_p0
                && self.f32_0p0 == rhs.f32_0p0
                && self.f32_42 == rhs.f32_42
                && self.f32_m42 == rhs.f32_m42
                && self.f32_0p25 == rhs.f32_0p25
                && self.f32_1p5e2 == rhs.f32_1p5e2
                && self.f32_inf == rhs.f32_inf
                && self.f32_minf == rhs.f32_minf
                && self.f32_nan == rhs.f32_nan
                && self.f32_mnan == rhs.f32_mnan
                && self.bool_default == rhs.bool_default
                && self.bool_true == rhs.bool_true
                && self.bool_false == rhs.bool_false
                && self.string_default == rhs.string_default
                && self.string_empty == rhs.string_empty
                && self.string_abc == rhs.string_abc
                && self.string_aiu == rhs.string_aiu
                && self.string_backslash == rhs.string_backslash
                && self.string_tab == rhs.string_tab
                && self.string_crlf == rhs.string_crlf
                && self.bytes_default == rhs.bytes_default
                && self.bytes_empty == rhs.bytes_empty
                && self.bytes_abc == rhs.bytes_abc
                && self.bytes_aiu == rhs.bytes_aiu
                && self.bytes_backslash == rhs.bytes_backslash
                && self.bytes_tab == rhs.bytes_tab
                && self.bytes_crlf == rhs.bytes_crlf
                && self.enum_default == rhs.enum_default
                && self.enum_one == rhs.enum_one
                && self.enum_fourty_two == rhs.enum_fourty_two
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
}
pub use _puroro_traits::*;
pub mod _puroro_traits {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }

    pub trait MsgTrait {
        fn i32_default<'this>(&'this self) -> i32 {
            self.i32_default_opt()
                .unwrap_or(::std::default::Default::default())
        }

        fn has_i32_default<'this>(&'this self) -> bool {
            self.i32_default_opt().is_some()
        }
        fn i32_default_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::None
        }

        fn i32_0<'this>(&'this self) -> i32 {
            self.i32_0_opt().unwrap_or(0)
        }

        fn has_i32_0<'this>(&'this self) -> bool {
            self.i32_0_opt().is_some()
        }
        fn i32_0_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::None
        }

        fn i32_42<'this>(&'this self) -> i32 {
            self.i32_42_opt().unwrap_or(42)
        }

        fn has_i32_42<'this>(&'this self) -> bool {
            self.i32_42_opt().is_some()
        }
        fn i32_42_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::None
        }

        fn i32_m42<'this>(&'this self) -> i32 {
            self.i32_m42_opt().unwrap_or(-42)
        }

        fn has_i32_m42<'this>(&'this self) -> bool {
            self.i32_m42_opt().is_some()
        }
        fn i32_m42_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::None
        }

        fn i32_2147483647<'this>(&'this self) -> i32 {
            self.i32_2147483647_opt().unwrap_or(2147483647)
        }

        fn has_i32_2147483647<'this>(&'this self) -> bool {
            self.i32_2147483647_opt().is_some()
        }
        fn i32_2147483647_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::None
        }

        fn i32_m2147483648<'this>(&'this self) -> i32 {
            self.i32_m2147483648_opt().unwrap_or(-2147483648)
        }

        fn has_i32_m2147483648<'this>(&'this self) -> bool {
            self.i32_m2147483648_opt().is_some()
        }
        fn i32_m2147483648_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::None
        }

        fn i32_0123<'this>(&'this self) -> i32 {
            self.i32_0123_opt().unwrap_or(83)
        }

        fn has_i32_0123<'this>(&'this self) -> bool {
            self.i32_0123_opt().is_some()
        }
        fn i32_0123_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::None
        }

        fn i32_0x123<'this>(&'this self) -> i32 {
            self.i32_0x123_opt().unwrap_or(291)
        }

        fn has_i32_0x123<'this>(&'this self) -> bool {
            self.i32_0x123_opt().is_some()
        }
        fn i32_0x123_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::None
        }

        fn u32_default<'this>(&'this self) -> u32 {
            self.u32_default_opt()
                .unwrap_or(::std::default::Default::default())
        }

        fn has_u32_default<'this>(&'this self) -> bool {
            self.u32_default_opt().is_some()
        }
        fn u32_default_opt<'this>(&'this self) -> ::std::option::Option<u32> {
            ::std::option::Option::None
        }

        fn u32_0<'this>(&'this self) -> u32 {
            self.u32_0_opt().unwrap_or(0)
        }

        fn has_u32_0<'this>(&'this self) -> bool {
            self.u32_0_opt().is_some()
        }
        fn u32_0_opt<'this>(&'this self) -> ::std::option::Option<u32> {
            ::std::option::Option::None
        }

        fn u32_42<'this>(&'this self) -> u32 {
            self.u32_42_opt().unwrap_or(42)
        }

        fn has_u32_42<'this>(&'this self) -> bool {
            self.u32_42_opt().is_some()
        }
        fn u32_42_opt<'this>(&'this self) -> ::std::option::Option<u32> {
            ::std::option::Option::None
        }

        fn u32_4294967295<'this>(&'this self) -> u32 {
            self.u32_4294967295_opt().unwrap_or(4294967295)
        }

        fn has_u32_4294967295<'this>(&'this self) -> bool {
            self.u32_4294967295_opt().is_some()
        }
        fn u32_4294967295_opt<'this>(&'this self) -> ::std::option::Option<u32> {
            ::std::option::Option::None
        }

        fn u32_0123<'this>(&'this self) -> u32 {
            self.u32_0123_opt().unwrap_or(83)
        }

        fn has_u32_0123<'this>(&'this self) -> bool {
            self.u32_0123_opt().is_some()
        }
        fn u32_0123_opt<'this>(&'this self) -> ::std::option::Option<u32> {
            ::std::option::Option::None
        }

        fn u32_0x123<'this>(&'this self) -> u32 {
            self.u32_0x123_opt().unwrap_or(291)
        }

        fn has_u32_0x123<'this>(&'this self) -> bool {
            self.u32_0x123_opt().is_some()
        }
        fn u32_0x123_opt<'this>(&'this self) -> ::std::option::Option<u32> {
            ::std::option::Option::None
        }

        fn i64_default<'this>(&'this self) -> i64 {
            self.i64_default_opt()
                .unwrap_or(::std::default::Default::default())
        }

        fn has_i64_default<'this>(&'this self) -> bool {
            self.i64_default_opt().is_some()
        }
        fn i64_default_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            ::std::option::Option::None
        }

        fn i64_0<'this>(&'this self) -> i64 {
            self.i64_0_opt().unwrap_or(0)
        }

        fn has_i64_0<'this>(&'this self) -> bool {
            self.i64_0_opt().is_some()
        }
        fn i64_0_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            ::std::option::Option::None
        }

        fn i64_42<'this>(&'this self) -> i64 {
            self.i64_42_opt().unwrap_or(42)
        }

        fn has_i64_42<'this>(&'this self) -> bool {
            self.i64_42_opt().is_some()
        }
        fn i64_42_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            ::std::option::Option::None
        }

        fn i64_m42<'this>(&'this self) -> i64 {
            self.i64_m42_opt().unwrap_or(-42)
        }

        fn has_i64_m42<'this>(&'this self) -> bool {
            self.i64_m42_opt().is_some()
        }
        fn i64_m42_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            ::std::option::Option::None
        }

        fn i64_9223372036854775807<'this>(&'this self) -> i64 {
            self.i64_9223372036854775807_opt()
                .unwrap_or(9223372036854775807)
        }

        fn has_i64_9223372036854775807<'this>(&'this self) -> bool {
            self.i64_9223372036854775807_opt().is_some()
        }
        fn i64_9223372036854775807_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            ::std::option::Option::None
        }

        fn i64_m9223372036854775808<'this>(&'this self) -> i64 {
            self.i64_m9223372036854775808_opt()
                .unwrap_or(-9223372036854775808)
        }

        fn has_i64_m9223372036854775808<'this>(&'this self) -> bool {
            self.i64_m9223372036854775808_opt().is_some()
        }
        fn i64_m9223372036854775808_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            ::std::option::Option::None
        }

        fn i64_0123<'this>(&'this self) -> i64 {
            self.i64_0123_opt().unwrap_or(83)
        }

        fn has_i64_0123<'this>(&'this self) -> bool {
            self.i64_0123_opt().is_some()
        }
        fn i64_0123_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            ::std::option::Option::None
        }

        fn i64_0x123<'this>(&'this self) -> i64 {
            self.i64_0x123_opt().unwrap_or(291)
        }

        fn has_i64_0x123<'this>(&'this self) -> bool {
            self.i64_0x123_opt().is_some()
        }
        fn i64_0x123_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            ::std::option::Option::None
        }

        fn u64_default<'this>(&'this self) -> u64 {
            self.u64_default_opt()
                .unwrap_or(::std::default::Default::default())
        }

        fn has_u64_default<'this>(&'this self) -> bool {
            self.u64_default_opt().is_some()
        }
        fn u64_default_opt<'this>(&'this self) -> ::std::option::Option<u64> {
            ::std::option::Option::None
        }

        fn u64_0<'this>(&'this self) -> u64 {
            self.u64_0_opt().unwrap_or(0)
        }

        fn has_u64_0<'this>(&'this self) -> bool {
            self.u64_0_opt().is_some()
        }
        fn u64_0_opt<'this>(&'this self) -> ::std::option::Option<u64> {
            ::std::option::Option::None
        }

        fn u64_42<'this>(&'this self) -> u64 {
            self.u64_42_opt().unwrap_or(42)
        }

        fn has_u64_42<'this>(&'this self) -> bool {
            self.u64_42_opt().is_some()
        }
        fn u64_42_opt<'this>(&'this self) -> ::std::option::Option<u64> {
            ::std::option::Option::None
        }

        fn u64_18446744073709551615<'this>(&'this self) -> u64 {
            self.u64_18446744073709551615_opt()
                .unwrap_or(18446744073709551615)
        }

        fn has_u64_18446744073709551615<'this>(&'this self) -> bool {
            self.u64_18446744073709551615_opt().is_some()
        }
        fn u64_18446744073709551615_opt<'this>(&'this self) -> ::std::option::Option<u64> {
            ::std::option::Option::None
        }

        fn u64_0123<'this>(&'this self) -> u64 {
            self.u64_0123_opt().unwrap_or(83)
        }

        fn has_u64_0123<'this>(&'this self) -> bool {
            self.u64_0123_opt().is_some()
        }
        fn u64_0123_opt<'this>(&'this self) -> ::std::option::Option<u64> {
            ::std::option::Option::None
        }

        fn u64_0x123<'this>(&'this self) -> u64 {
            self.u64_0x123_opt().unwrap_or(291)
        }

        fn has_u64_0x123<'this>(&'this self) -> bool {
            self.u64_0x123_opt().is_some()
        }
        fn u64_0x123_opt<'this>(&'this self) -> ::std::option::Option<u64> {
            ::std::option::Option::None
        }

        fn f32_default<'this>(&'this self) -> f32 {
            self.f32_default_opt()
                .unwrap_or(::std::default::Default::default())
        }

        fn has_f32_default<'this>(&'this self) -> bool {
            self.f32_default_opt().is_some()
        }
        fn f32_default_opt<'this>(&'this self) -> ::std::option::Option<f32> {
            ::std::option::Option::None
        }

        fn f32_0<'this>(&'this self) -> f32 {
            self.f32_0_opt().unwrap_or(0f32)
        }

        fn has_f32_0<'this>(&'this self) -> bool {
            self.f32_0_opt().is_some()
        }
        fn f32_0_opt<'this>(&'this self) -> ::std::option::Option<f32> {
            ::std::option::Option::None
        }

        fn f32_m0<'this>(&'this self) -> f32 {
            self.f32_m0_opt().unwrap_or(-0f32)
        }

        fn has_f32_m0<'this>(&'this self) -> bool {
            self.f32_m0_opt().is_some()
        }
        fn f32_m0_opt<'this>(&'this self) -> ::std::option::Option<f32> {
            ::std::option::Option::None
        }

        fn f32_0p<'this>(&'this self) -> f32 {
            self.f32_0p_opt().unwrap_or(0f32)
        }

        fn has_f32_0p<'this>(&'this self) -> bool {
            self.f32_0p_opt().is_some()
        }
        fn f32_0p_opt<'this>(&'this self) -> ::std::option::Option<f32> {
            ::std::option::Option::None
        }

        fn f32_p0<'this>(&'this self) -> f32 {
            self.f32_p0_opt().unwrap_or(0f32)
        }

        fn has_f32_p0<'this>(&'this self) -> bool {
            self.f32_p0_opt().is_some()
        }
        fn f32_p0_opt<'this>(&'this self) -> ::std::option::Option<f32> {
            ::std::option::Option::None
        }

        fn f32_0p0<'this>(&'this self) -> f32 {
            self.f32_0p0_opt().unwrap_or(0f32)
        }

        fn has_f32_0p0<'this>(&'this self) -> bool {
            self.f32_0p0_opt().is_some()
        }
        fn f32_0p0_opt<'this>(&'this self) -> ::std::option::Option<f32> {
            ::std::option::Option::None
        }

        fn f32_42<'this>(&'this self) -> f32 {
            self.f32_42_opt().unwrap_or(42f32)
        }

        fn has_f32_42<'this>(&'this self) -> bool {
            self.f32_42_opt().is_some()
        }
        fn f32_42_opt<'this>(&'this self) -> ::std::option::Option<f32> {
            ::std::option::Option::None
        }

        fn f32_m42<'this>(&'this self) -> f32 {
            self.f32_m42_opt().unwrap_or(-42f32)
        }

        fn has_f32_m42<'this>(&'this self) -> bool {
            self.f32_m42_opt().is_some()
        }
        fn f32_m42_opt<'this>(&'this self) -> ::std::option::Option<f32> {
            ::std::option::Option::None
        }

        fn f32_0p25<'this>(&'this self) -> f32 {
            self.f32_0p25_opt().unwrap_or(0.25f32)
        }

        fn has_f32_0p25<'this>(&'this self) -> bool {
            self.f32_0p25_opt().is_some()
        }
        fn f32_0p25_opt<'this>(&'this self) -> ::std::option::Option<f32> {
            ::std::option::Option::None
        }

        fn f32_1p5e2<'this>(&'this self) -> f32 {
            self.f32_1p5e2_opt().unwrap_or(150f32)
        }

        fn has_f32_1p5e2<'this>(&'this self) -> bool {
            self.f32_1p5e2_opt().is_some()
        }
        fn f32_1p5e2_opt<'this>(&'this self) -> ::std::option::Option<f32> {
            ::std::option::Option::None
        }

        fn f32_inf<'this>(&'this self) -> f32 {
            self.f32_inf_opt().unwrap_or(f32::INFINITY)
        }

        fn has_f32_inf<'this>(&'this self) -> bool {
            self.f32_inf_opt().is_some()
        }
        fn f32_inf_opt<'this>(&'this self) -> ::std::option::Option<f32> {
            ::std::option::Option::None
        }

        fn f32_minf<'this>(&'this self) -> f32 {
            self.f32_minf_opt().unwrap_or(f32::NEG_INFINITY)
        }

        fn has_f32_minf<'this>(&'this self) -> bool {
            self.f32_minf_opt().is_some()
        }
        fn f32_minf_opt<'this>(&'this self) -> ::std::option::Option<f32> {
            ::std::option::Option::None
        }

        fn f32_nan<'this>(&'this self) -> f32 {
            self.f32_nan_opt().unwrap_or(f32::NAN)
        }

        fn has_f32_nan<'this>(&'this self) -> bool {
            self.f32_nan_opt().is_some()
        }
        fn f32_nan_opt<'this>(&'this self) -> ::std::option::Option<f32> {
            ::std::option::Option::None
        }

        fn f32_mnan<'this>(&'this self) -> f32 {
            self.f32_mnan_opt().unwrap_or(f32::NAN)
        }

        fn has_f32_mnan<'this>(&'this self) -> bool {
            self.f32_mnan_opt().is_some()
        }
        fn f32_mnan_opt<'this>(&'this self) -> ::std::option::Option<f32> {
            ::std::option::Option::None
        }

        fn bool_default<'this>(&'this self) -> bool {
            self.bool_default_opt()
                .unwrap_or(::std::default::Default::default())
        }

        fn has_bool_default<'this>(&'this self) -> bool {
            self.bool_default_opt().is_some()
        }
        fn bool_default_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::None
        }

        fn bool_true<'this>(&'this self) -> bool {
            self.bool_true_opt().unwrap_or(true)
        }

        fn has_bool_true<'this>(&'this self) -> bool {
            self.bool_true_opt().is_some()
        }
        fn bool_true_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::None
        }

        fn bool_false<'this>(&'this self) -> bool {
            self.bool_false_opt().unwrap_or(false)
        }

        fn has_bool_false<'this>(&'this self) -> bool {
            self.bool_false_opt().is_some()
        }
        fn bool_false_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::None
        }

        type Field71ScalarGetterType<'this>: ::std::convert::AsRef<str>
            + ::std::convert::From<&'static str>
        where
            Self: 'this;

        fn string_default<'this>(&'this self) -> Self::Field71ScalarGetterType<'this> {
            self.string_default_opt()
                .unwrap_or(Self::string_default_default_value())
        }
        fn string_default_default_value() -> Self::Field71ScalarGetterType<'static>;

        fn has_string_default<'this>(&'this self) -> bool {
            self.string_default_opt().is_some()
        }
        fn string_default_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field71ScalarGetterType<'this>> {
            ::std::option::Option::None
        }

        type Field72ScalarGetterType<'this>: ::std::convert::AsRef<str>
            + ::std::convert::From<&'static str>
        where
            Self: 'this;

        fn string_empty<'this>(&'this self) -> Self::Field72ScalarGetterType<'this> {
            self.string_empty_opt()
                .unwrap_or(Self::string_empty_default_value())
        }
        fn string_empty_default_value() -> Self::Field72ScalarGetterType<'static>;

        fn has_string_empty<'this>(&'this self) -> bool {
            self.string_empty_opt().is_some()
        }
        fn string_empty_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field72ScalarGetterType<'this>> {
            ::std::option::Option::None
        }

        type Field73ScalarGetterType<'this>: ::std::convert::AsRef<str>
            + ::std::convert::From<&'static str>
        where
            Self: 'this;

        fn string_abc<'this>(&'this self) -> Self::Field73ScalarGetterType<'this> {
            self.string_abc_opt()
                .unwrap_or(Self::string_abc_default_value())
        }
        fn string_abc_default_value() -> Self::Field73ScalarGetterType<'static>;

        fn has_string_abc<'this>(&'this self) -> bool {
            self.string_abc_opt().is_some()
        }
        fn string_abc_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field73ScalarGetterType<'this>> {
            ::std::option::Option::None
        }

        type Field74ScalarGetterType<'this>: ::std::convert::AsRef<str>
            + ::std::convert::From<&'static str>
        where
            Self: 'this;

        fn string_aiu<'this>(&'this self) -> Self::Field74ScalarGetterType<'this> {
            self.string_aiu_opt()
                .unwrap_or(Self::string_aiu_default_value())
        }
        fn string_aiu_default_value() -> Self::Field74ScalarGetterType<'static>;

        fn has_string_aiu<'this>(&'this self) -> bool {
            self.string_aiu_opt().is_some()
        }
        fn string_aiu_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field74ScalarGetterType<'this>> {
            ::std::option::Option::None
        }

        type Field75ScalarGetterType<'this>: ::std::convert::AsRef<str>
            + ::std::convert::From<&'static str>
        where
            Self: 'this;

        fn string_backslash<'this>(&'this self) -> Self::Field75ScalarGetterType<'this> {
            self.string_backslash_opt()
                .unwrap_or(Self::string_backslash_default_value())
        }
        fn string_backslash_default_value() -> Self::Field75ScalarGetterType<'static>;

        fn has_string_backslash<'this>(&'this self) -> bool {
            self.string_backslash_opt().is_some()
        }
        fn string_backslash_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field75ScalarGetterType<'this>> {
            ::std::option::Option::None
        }

        type Field76ScalarGetterType<'this>: ::std::convert::AsRef<str>
            + ::std::convert::From<&'static str>
        where
            Self: 'this;

        fn string_tab<'this>(&'this self) -> Self::Field76ScalarGetterType<'this> {
            self.string_tab_opt()
                .unwrap_or(Self::string_tab_default_value())
        }
        fn string_tab_default_value() -> Self::Field76ScalarGetterType<'static>;

        fn has_string_tab<'this>(&'this self) -> bool {
            self.string_tab_opt().is_some()
        }
        fn string_tab_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field76ScalarGetterType<'this>> {
            ::std::option::Option::None
        }

        type Field77ScalarGetterType<'this>: ::std::convert::AsRef<str>
            + ::std::convert::From<&'static str>
        where
            Self: 'this;

        fn string_crlf<'this>(&'this self) -> Self::Field77ScalarGetterType<'this> {
            self.string_crlf_opt()
                .unwrap_or(Self::string_crlf_default_value())
        }
        fn string_crlf_default_value() -> Self::Field77ScalarGetterType<'static>;

        fn has_string_crlf<'this>(&'this self) -> bool {
            self.string_crlf_opt().is_some()
        }
        fn string_crlf_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field77ScalarGetterType<'this>> {
            ::std::option::Option::None
        }

        type Field81ScalarGetterType<'this>: ::std::convert::AsRef<[u8]>
            + ::std::convert::From<&'static [u8]>
        where
            Self: 'this;

        fn bytes_default<'this>(&'this self) -> Self::Field81ScalarGetterType<'this> {
            self.bytes_default_opt()
                .unwrap_or(Self::bytes_default_default_value())
        }
        fn bytes_default_default_value() -> Self::Field81ScalarGetterType<'static>;

        fn has_bytes_default<'this>(&'this self) -> bool {
            self.bytes_default_opt().is_some()
        }
        fn bytes_default_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field81ScalarGetterType<'this>> {
            ::std::option::Option::None
        }

        type Field82ScalarGetterType<'this>: ::std::convert::AsRef<[u8]>
            + ::std::convert::From<&'static [u8]>
        where
            Self: 'this;

        fn bytes_empty<'this>(&'this self) -> Self::Field82ScalarGetterType<'this> {
            self.bytes_empty_opt()
                .unwrap_or(Self::bytes_empty_default_value())
        }
        fn bytes_empty_default_value() -> Self::Field82ScalarGetterType<'static>;

        fn has_bytes_empty<'this>(&'this self) -> bool {
            self.bytes_empty_opt().is_some()
        }
        fn bytes_empty_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field82ScalarGetterType<'this>> {
            ::std::option::Option::None
        }

        type Field83ScalarGetterType<'this>: ::std::convert::AsRef<[u8]>
            + ::std::convert::From<&'static [u8]>
        where
            Self: 'this;

        fn bytes_abc<'this>(&'this self) -> Self::Field83ScalarGetterType<'this> {
            self.bytes_abc_opt()
                .unwrap_or(Self::bytes_abc_default_value())
        }
        fn bytes_abc_default_value() -> Self::Field83ScalarGetterType<'static>;

        fn has_bytes_abc<'this>(&'this self) -> bool {
            self.bytes_abc_opt().is_some()
        }
        fn bytes_abc_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field83ScalarGetterType<'this>> {
            ::std::option::Option::None
        }

        type Field84ScalarGetterType<'this>: ::std::convert::AsRef<[u8]>
            + ::std::convert::From<&'static [u8]>
        where
            Self: 'this;

        fn bytes_aiu<'this>(&'this self) -> Self::Field84ScalarGetterType<'this> {
            self.bytes_aiu_opt()
                .unwrap_or(Self::bytes_aiu_default_value())
        }
        fn bytes_aiu_default_value() -> Self::Field84ScalarGetterType<'static>;

        fn has_bytes_aiu<'this>(&'this self) -> bool {
            self.bytes_aiu_opt().is_some()
        }
        fn bytes_aiu_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field84ScalarGetterType<'this>> {
            ::std::option::Option::None
        }

        type Field85ScalarGetterType<'this>: ::std::convert::AsRef<[u8]>
            + ::std::convert::From<&'static [u8]>
        where
            Self: 'this;

        fn bytes_backslash<'this>(&'this self) -> Self::Field85ScalarGetterType<'this> {
            self.bytes_backslash_opt()
                .unwrap_or(Self::bytes_backslash_default_value())
        }
        fn bytes_backslash_default_value() -> Self::Field85ScalarGetterType<'static>;

        fn has_bytes_backslash<'this>(&'this self) -> bool {
            self.bytes_backslash_opt().is_some()
        }
        fn bytes_backslash_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field85ScalarGetterType<'this>> {
            ::std::option::Option::None
        }

        type Field86ScalarGetterType<'this>: ::std::convert::AsRef<[u8]>
            + ::std::convert::From<&'static [u8]>
        where
            Self: 'this;

        fn bytes_tab<'this>(&'this self) -> Self::Field86ScalarGetterType<'this> {
            self.bytes_tab_opt()
                .unwrap_or(Self::bytes_tab_default_value())
        }
        fn bytes_tab_default_value() -> Self::Field86ScalarGetterType<'static>;

        fn has_bytes_tab<'this>(&'this self) -> bool {
            self.bytes_tab_opt().is_some()
        }
        fn bytes_tab_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field86ScalarGetterType<'this>> {
            ::std::option::Option::None
        }

        type Field87ScalarGetterType<'this>: ::std::convert::AsRef<[u8]>
            + ::std::convert::From<&'static [u8]>
        where
            Self: 'this;

        fn bytes_crlf<'this>(&'this self) -> Self::Field87ScalarGetterType<'this> {
            self.bytes_crlf_opt()
                .unwrap_or(Self::bytes_crlf_default_value())
        }
        fn bytes_crlf_default_value() -> Self::Field87ScalarGetterType<'static>;

        fn has_bytes_crlf<'this>(&'this self) -> bool {
            self.bytes_crlf_opt().is_some()
        }
        fn bytes_crlf_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field87ScalarGetterType<'this>> {
            ::std::option::Option::None
        }

        fn enum_default<'this>(&'this self) -> self::_puroro_root::proto2_defaults::MyEnum {
            self.enum_default_opt()
                .unwrap_or(::std::default::Default::default())
        }

        fn has_enum_default<'this>(&'this self) -> bool {
            self.enum_default_opt().is_some()
        }
        fn enum_default_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::proto2_defaults::MyEnum> {
            ::std::option::Option::None
        }

        fn enum_one<'this>(&'this self) -> self::_puroro_root::proto2_defaults::MyEnum {
            self.enum_one_opt()
                .unwrap_or(self::_puroro_root::proto2_defaults::MyEnum::One)
        }

        fn has_enum_one<'this>(&'this self) -> bool {
            self.enum_one_opt().is_some()
        }
        fn enum_one_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::proto2_defaults::MyEnum> {
            ::std::option::Option::None
        }

        fn enum_fourty_two<'this>(&'this self) -> self::_puroro_root::proto2_defaults::MyEnum {
            self.enum_fourty_two_opt()
                .unwrap_or(self::_puroro_root::proto2_defaults::MyEnum::FourtyTwo)
        }

        fn has_enum_fourty_two<'this>(&'this self) -> bool {
            self.enum_fourty_two_opt().is_some()
        }
        fn enum_fourty_two_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::proto2_defaults::MyEnum> {
            ::std::option::Option::None
        }
    }

    macro_rules! msg_delegate {
        ($ty:ty) => {
            fn i32_default_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).i32_default_opt()
            }
            fn i32_0_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).i32_0_opt()
            }
            fn i32_42_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).i32_42_opt()
            }
            fn i32_m42_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).i32_m42_opt()
            }
            fn i32_2147483647_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).i32_2147483647_opt()
            }
            fn i32_m2147483648_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).i32_m2147483648_opt()
            }
            fn i32_0123_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).i32_0123_opt()
            }
            fn i32_0x123_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).i32_0x123_opt()
            }
            fn u32_default_opt<'this>(&'this self) -> ::std::option::Option<u32> {
                (**self).u32_default_opt()
            }
            fn u32_0_opt<'this>(&'this self) -> ::std::option::Option<u32> {
                (**self).u32_0_opt()
            }
            fn u32_42_opt<'this>(&'this self) -> ::std::option::Option<u32> {
                (**self).u32_42_opt()
            }
            fn u32_4294967295_opt<'this>(&'this self) -> ::std::option::Option<u32> {
                (**self).u32_4294967295_opt()
            }
            fn u32_0123_opt<'this>(&'this self) -> ::std::option::Option<u32> {
                (**self).u32_0123_opt()
            }
            fn u32_0x123_opt<'this>(&'this self) -> ::std::option::Option<u32> {
                (**self).u32_0x123_opt()
            }
            fn i64_default_opt<'this>(&'this self) -> ::std::option::Option<i64> {
                (**self).i64_default_opt()
            }
            fn i64_0_opt<'this>(&'this self) -> ::std::option::Option<i64> {
                (**self).i64_0_opt()
            }
            fn i64_42_opt<'this>(&'this self) -> ::std::option::Option<i64> {
                (**self).i64_42_opt()
            }
            fn i64_m42_opt<'this>(&'this self) -> ::std::option::Option<i64> {
                (**self).i64_m42_opt()
            }
            fn i64_9223372036854775807_opt<'this>(&'this self) -> ::std::option::Option<i64> {
                (**self).i64_9223372036854775807_opt()
            }
            fn i64_m9223372036854775808_opt<'this>(&'this self) -> ::std::option::Option<i64> {
                (**self).i64_m9223372036854775808_opt()
            }
            fn i64_0123_opt<'this>(&'this self) -> ::std::option::Option<i64> {
                (**self).i64_0123_opt()
            }
            fn i64_0x123_opt<'this>(&'this self) -> ::std::option::Option<i64> {
                (**self).i64_0x123_opt()
            }
            fn u64_default_opt<'this>(&'this self) -> ::std::option::Option<u64> {
                (**self).u64_default_opt()
            }
            fn u64_0_opt<'this>(&'this self) -> ::std::option::Option<u64> {
                (**self).u64_0_opt()
            }
            fn u64_42_opt<'this>(&'this self) -> ::std::option::Option<u64> {
                (**self).u64_42_opt()
            }
            fn u64_18446744073709551615_opt<'this>(&'this self) -> ::std::option::Option<u64> {
                (**self).u64_18446744073709551615_opt()
            }
            fn u64_0123_opt<'this>(&'this self) -> ::std::option::Option<u64> {
                (**self).u64_0123_opt()
            }
            fn u64_0x123_opt<'this>(&'this self) -> ::std::option::Option<u64> {
                (**self).u64_0x123_opt()
            }
            fn f32_default_opt<'this>(&'this self) -> ::std::option::Option<f32> {
                (**self).f32_default_opt()
            }
            fn f32_0_opt<'this>(&'this self) -> ::std::option::Option<f32> {
                (**self).f32_0_opt()
            }
            fn f32_m0_opt<'this>(&'this self) -> ::std::option::Option<f32> {
                (**self).f32_m0_opt()
            }
            fn f32_0p_opt<'this>(&'this self) -> ::std::option::Option<f32> {
                (**self).f32_0p_opt()
            }
            fn f32_p0_opt<'this>(&'this self) -> ::std::option::Option<f32> {
                (**self).f32_p0_opt()
            }
            fn f32_0p0_opt<'this>(&'this self) -> ::std::option::Option<f32> {
                (**self).f32_0p0_opt()
            }
            fn f32_42_opt<'this>(&'this self) -> ::std::option::Option<f32> {
                (**self).f32_42_opt()
            }
            fn f32_m42_opt<'this>(&'this self) -> ::std::option::Option<f32> {
                (**self).f32_m42_opt()
            }
            fn f32_0p25_opt<'this>(&'this self) -> ::std::option::Option<f32> {
                (**self).f32_0p25_opt()
            }
            fn f32_1p5e2_opt<'this>(&'this self) -> ::std::option::Option<f32> {
                (**self).f32_1p5e2_opt()
            }
            fn f32_inf_opt<'this>(&'this self) -> ::std::option::Option<f32> {
                (**self).f32_inf_opt()
            }
            fn f32_minf_opt<'this>(&'this self) -> ::std::option::Option<f32> {
                (**self).f32_minf_opt()
            }
            fn f32_nan_opt<'this>(&'this self) -> ::std::option::Option<f32> {
                (**self).f32_nan_opt()
            }
            fn f32_mnan_opt<'this>(&'this self) -> ::std::option::Option<f32> {
                (**self).f32_mnan_opt()
            }
            fn bool_default_opt<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).bool_default_opt()
            }
            fn bool_true_opt<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).bool_true_opt()
            }
            fn bool_false_opt<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).bool_false_opt()
            }
            type Field71ScalarGetterType<'this> where Self: 'this =
                <$ty as MsgTrait>::Field71ScalarGetterType<'this>;
            fn string_default_opt<'this>(&'this self) -> ::std::option::Option<Self::Field71ScalarGetterType<'this>> {
                (**self).string_default_opt()
            }
            fn string_default_default_value() -> <$ty as MsgTrait>::Field71ScalarGetterType<'static> {
                <$ty as MsgTrait>::string_default_default_value()
            }
            type Field72ScalarGetterType<'this> where Self: 'this =
                <$ty as MsgTrait>::Field72ScalarGetterType<'this>;
            fn string_empty_opt<'this>(&'this self) -> ::std::option::Option<Self::Field72ScalarGetterType<'this>> {
                (**self).string_empty_opt()
            }
            fn string_empty_default_value() -> <$ty as MsgTrait>::Field72ScalarGetterType<'static> {
                <$ty as MsgTrait>::string_empty_default_value()
            }
            type Field73ScalarGetterType<'this> where Self: 'this =
                <$ty as MsgTrait>::Field73ScalarGetterType<'this>;
            fn string_abc_opt<'this>(&'this self) -> ::std::option::Option<Self::Field73ScalarGetterType<'this>> {
                (**self).string_abc_opt()
            }
            fn string_abc_default_value() -> <$ty as MsgTrait>::Field73ScalarGetterType<'static> {
                <$ty as MsgTrait>::string_abc_default_value()
            }
            type Field74ScalarGetterType<'this> where Self: 'this =
                <$ty as MsgTrait>::Field74ScalarGetterType<'this>;
            fn string_aiu_opt<'this>(&'this self) -> ::std::option::Option<Self::Field74ScalarGetterType<'this>> {
                (**self).string_aiu_opt()
            }
            fn string_aiu_default_value() -> <$ty as MsgTrait>::Field74ScalarGetterType<'static> {
                <$ty as MsgTrait>::string_aiu_default_value()
            }
            type Field75ScalarGetterType<'this> where Self: 'this =
                <$ty as MsgTrait>::Field75ScalarGetterType<'this>;
            fn string_backslash_opt<'this>(&'this self) -> ::std::option::Option<Self::Field75ScalarGetterType<'this>> {
                (**self).string_backslash_opt()
            }
            fn string_backslash_default_value() -> <$ty as MsgTrait>::Field75ScalarGetterType<'static> {
                <$ty as MsgTrait>::string_backslash_default_value()
            }
            type Field76ScalarGetterType<'this> where Self: 'this =
                <$ty as MsgTrait>::Field76ScalarGetterType<'this>;
            fn string_tab_opt<'this>(&'this self) -> ::std::option::Option<Self::Field76ScalarGetterType<'this>> {
                (**self).string_tab_opt()
            }
            fn string_tab_default_value() -> <$ty as MsgTrait>::Field76ScalarGetterType<'static> {
                <$ty as MsgTrait>::string_tab_default_value()
            }
            type Field77ScalarGetterType<'this> where Self: 'this =
                <$ty as MsgTrait>::Field77ScalarGetterType<'this>;
            fn string_crlf_opt<'this>(&'this self) -> ::std::option::Option<Self::Field77ScalarGetterType<'this>> {
                (**self).string_crlf_opt()
            }
            fn string_crlf_default_value() -> <$ty as MsgTrait>::Field77ScalarGetterType<'static> {
                <$ty as MsgTrait>::string_crlf_default_value()
            }
            type Field81ScalarGetterType<'this> where Self: 'this =
                <$ty as MsgTrait>::Field81ScalarGetterType<'this>;
            fn bytes_default_opt<'this>(&'this self) -> ::std::option::Option<Self::Field81ScalarGetterType<'this>> {
                (**self).bytes_default_opt()
            }
            fn bytes_default_default_value() -> <$ty as MsgTrait>::Field81ScalarGetterType<'static> {
                <$ty as MsgTrait>::bytes_default_default_value()
            }
            type Field82ScalarGetterType<'this> where Self: 'this =
                <$ty as MsgTrait>::Field82ScalarGetterType<'this>;
            fn bytes_empty_opt<'this>(&'this self) -> ::std::option::Option<Self::Field82ScalarGetterType<'this>> {
                (**self).bytes_empty_opt()
            }
            fn bytes_empty_default_value() -> <$ty as MsgTrait>::Field82ScalarGetterType<'static> {
                <$ty as MsgTrait>::bytes_empty_default_value()
            }
            type Field83ScalarGetterType<'this> where Self: 'this =
                <$ty as MsgTrait>::Field83ScalarGetterType<'this>;
            fn bytes_abc_opt<'this>(&'this self) -> ::std::option::Option<Self::Field83ScalarGetterType<'this>> {
                (**self).bytes_abc_opt()
            }
            fn bytes_abc_default_value() -> <$ty as MsgTrait>::Field83ScalarGetterType<'static> {
                <$ty as MsgTrait>::bytes_abc_default_value()
            }
            type Field84ScalarGetterType<'this> where Self: 'this =
                <$ty as MsgTrait>::Field84ScalarGetterType<'this>;
            fn bytes_aiu_opt<'this>(&'this self) -> ::std::option::Option<Self::Field84ScalarGetterType<'this>> {
                (**self).bytes_aiu_opt()
            }
            fn bytes_aiu_default_value() -> <$ty as MsgTrait>::Field84ScalarGetterType<'static> {
                <$ty as MsgTrait>::bytes_aiu_default_value()
            }
            type Field85ScalarGetterType<'this> where Self: 'this =
                <$ty as MsgTrait>::Field85ScalarGetterType<'this>;
            fn bytes_backslash_opt<'this>(&'this self) -> ::std::option::Option<Self::Field85ScalarGetterType<'this>> {
                (**self).bytes_backslash_opt()
            }
            fn bytes_backslash_default_value() -> <$ty as MsgTrait>::Field85ScalarGetterType<'static> {
                <$ty as MsgTrait>::bytes_backslash_default_value()
            }
            type Field86ScalarGetterType<'this> where Self: 'this =
                <$ty as MsgTrait>::Field86ScalarGetterType<'this>;
            fn bytes_tab_opt<'this>(&'this self) -> ::std::option::Option<Self::Field86ScalarGetterType<'this>> {
                (**self).bytes_tab_opt()
            }
            fn bytes_tab_default_value() -> <$ty as MsgTrait>::Field86ScalarGetterType<'static> {
                <$ty as MsgTrait>::bytes_tab_default_value()
            }
            type Field87ScalarGetterType<'this> where Self: 'this =
                <$ty as MsgTrait>::Field87ScalarGetterType<'this>;
            fn bytes_crlf_opt<'this>(&'this self) -> ::std::option::Option<Self::Field87ScalarGetterType<'this>> {
                (**self).bytes_crlf_opt()
            }
            fn bytes_crlf_default_value() -> <$ty as MsgTrait>::Field87ScalarGetterType<'static> {
                <$ty as MsgTrait>::bytes_crlf_default_value()
            }
            fn enum_default_opt<'this>(&'this self) -> ::std::option::Option<self::_puroro_root::proto2_defaults::MyEnum> {
                (**self).enum_default_opt()
            }
            fn enum_one_opt<'this>(&'this self) -> ::std::option::Option<self::_puroro_root::proto2_defaults::MyEnum> {
                (**self).enum_one_opt()
            }
            fn enum_fourty_two_opt<'this>(&'this self) -> ::std::option::Option<self::_puroro_root::proto2_defaults::MyEnum> {
                (**self).enum_fourty_two_opt()
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
}
#[derive(::std::fmt::Debug, ::std::clone::Clone, ::std::marker::Copy, ::std::cmp::PartialEq)]
pub enum MyEnum {
    One,
    FourtyTwo,
}

impl ::puroro::Enum2 for MyEnum {}
impl ::std::convert::TryFrom<i32> for MyEnum {
    type Error = i32;
    fn try_from(value: i32) -> ::std::result::Result<Self, i32> {
        ::std::result::Result::Ok(match value {
            1 => MyEnum::One,
            42 => MyEnum::FourtyTwo,
            _ => Err(value)?,
        })
    }
}

impl ::std::convert::From<MyEnum> for i32 {
    fn from(value: MyEnum) -> i32 {
        match value {
            MyEnum::One => 1,
            MyEnum::FourtyTwo => 42,
        }
    }
}

impl ::std::default::Default for MyEnum {
    fn default() -> Self {
        MyEnum::One
    }
}

impl<'bump> ::puroro::internal::BumpDefault<'bump> for MyEnum {
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
    }
}
