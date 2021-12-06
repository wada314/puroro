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
    i32_unlabeled: i32,
    i32_optional: ::std::option::Option<i32>,
    i32_repeated: ::std::vec::Vec<i32>,
    float_unlabeled: f32,
    float_optional: ::std::option::Option<f32>,
    float_repeated: ::std::vec::Vec<f32>,
    bytes_unlabeled: ::std::vec::Vec<u8>,
    bytes_optional: ::std::option::Option<::std::vec::Vec<u8>>,
    bytes_repeated: ::std::vec::Vec<::std::vec::Vec<u8>>,
    string_unlabeled: ::std::string::String,
    string_optional: ::std::option::Option<::std::string::String>,
    string_repeated: ::std::vec::Vec<::std::string::String>,
    enum_unlabeled: self::_puroro_root::full_coverage3::Enum,
    enum_optional: ::std::option::Option<self::_puroro_root::full_coverage3::Enum>,
    enum_repeated: ::std::vec::Vec<self::_puroro_root::full_coverage3::Enum>,
    submsg_unlabeled: ::std::option::Option<::std::boxed::Box<self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_simple_impl::Submsg>>,
    submsg_optional: ::std::option::Option<::std::boxed::Box<self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_simple_impl::Submsg>>,
    submsg_repeated: ::std::vec::Vec<self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_simple_impl::Submsg>,
    i64_unlabeled: i64,
    i64_optional: ::std::option::Option<i64>,
    i64_repeated: ::std::vec::Vec<i64>,
    u32_unlabeled: u32,
    u32_optional: ::std::option::Option<u32>,
    u32_repeated: ::std::vec::Vec<u32>,
    u64_unlabeled: u64,
    u64_optional: ::std::option::Option<u64>,
    u64_repeated: ::std::vec::Vec<u64>,
    s32_unlabeled: i32,
    s32_optional: ::std::option::Option<i32>,
    s32_repeated: ::std::vec::Vec<i32>,
    s64_unlabeled: i64,
    s64_optional: ::std::option::Option<i64>,
    s64_repeated: ::std::vec::Vec<i64>,
    fixed32_unlabeled: u32,
    fixed32_optional: ::std::option::Option<u32>,
    fixed32_repeated: ::std::vec::Vec<u32>,
    fixed64_unlabeled: u64,
    fixed64_optional: ::std::option::Option<u64>,
    fixed64_repeated: ::std::vec::Vec<u64>,
    sfixed32_unlabeled: i32,
    sfixed32_optional: ::std::option::Option<i32>,
    sfixed32_repeated: ::std::vec::Vec<i32>,
    sfixed64_unlabeled: i64,
    sfixed64_optional: ::std::option::Option<i64>,
    sfixed64_repeated: ::std::vec::Vec<i64>,
    f64_unlabeled: f64,
    f64_optional: ::std::option::Option<f64>,
    f64_repeated: ::std::vec::Vec<f64>,
}
    impl ::puroro::Message<Msg> for Msg {}

    impl Msg {
        pub fn new() -> Self {
            Self {
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
        pub fn i32_unlabeled_mut(&mut self) -> &mut i32 {
            &mut self.i32_unlabeled
        }
        pub fn i32_optional_mut(&mut self) -> &mut ::std::option::Option<i32> {
            &mut self.i32_optional
        }
        pub fn i32_repeated_mut(&mut self) -> &mut ::std::vec::Vec<i32> {
            &mut self.i32_repeated
        }
        pub fn float_unlabeled_mut(&mut self) -> &mut f32 {
            &mut self.float_unlabeled
        }
        pub fn float_optional_mut(&mut self) -> &mut ::std::option::Option<f32> {
            &mut self.float_optional
        }
        pub fn float_repeated_mut(&mut self) -> &mut ::std::vec::Vec<f32> {
            &mut self.float_repeated
        }
        pub fn bytes_unlabeled_mut(&mut self) -> &mut ::std::vec::Vec<u8> {
            &mut self.bytes_unlabeled
        }
        pub fn bytes_optional_mut(&mut self) -> &mut ::std::option::Option<::std::vec::Vec<u8>> {
            &mut self.bytes_optional
        }
        pub fn bytes_repeated_mut(&mut self) -> &mut ::std::vec::Vec<::std::vec::Vec<u8>> {
            &mut self.bytes_repeated
        }
        pub fn string_unlabeled_mut(&mut self) -> &mut ::std::string::String {
            &mut self.string_unlabeled
        }
        pub fn string_optional_mut(&mut self) -> &mut ::std::option::Option<::std::string::String> {
            &mut self.string_optional
        }
        pub fn string_repeated_mut(&mut self) -> &mut ::std::vec::Vec<::std::string::String> {
            &mut self.string_repeated
        }
        pub fn enum_unlabeled_mut(&mut self) -> &mut self::_puroro_root::full_coverage3::Enum {
            &mut self.enum_unlabeled
        }
        pub fn enum_optional_mut(
            &mut self,
        ) -> &mut ::std::option::Option<self::_puroro_root::full_coverage3::Enum> {
            &mut self.enum_optional
        }
        pub fn enum_repeated_mut(
            &mut self,
        ) -> &mut ::std::vec::Vec<self::_puroro_root::full_coverage3::Enum> {
            &mut self.enum_repeated
        }
        pub fn submsg_unlabeled_mut(&mut self) -> &mut ::std::option::Option<::std::boxed::Box<self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_simple_impl::Submsg>>{
            &mut self.submsg_unlabeled
        }
        pub fn submsg_optional_mut(&mut self) -> &mut ::std::option::Option<::std::boxed::Box<self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_simple_impl::Submsg>>{
            &mut self.submsg_optional
        }
        pub fn submsg_repeated_mut(
            &mut self,
        ) -> &mut ::std::vec::Vec<
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_simple_impl::Submsg,
        > {
            &mut self.submsg_repeated
        }
        pub fn i64_unlabeled_mut(&mut self) -> &mut i64 {
            &mut self.i64_unlabeled
        }
        pub fn i64_optional_mut(&mut self) -> &mut ::std::option::Option<i64> {
            &mut self.i64_optional
        }
        pub fn i64_repeated_mut(&mut self) -> &mut ::std::vec::Vec<i64> {
            &mut self.i64_repeated
        }
        pub fn u32_unlabeled_mut(&mut self) -> &mut u32 {
            &mut self.u32_unlabeled
        }
        pub fn u32_optional_mut(&mut self) -> &mut ::std::option::Option<u32> {
            &mut self.u32_optional
        }
        pub fn u32_repeated_mut(&mut self) -> &mut ::std::vec::Vec<u32> {
            &mut self.u32_repeated
        }
        pub fn u64_unlabeled_mut(&mut self) -> &mut u64 {
            &mut self.u64_unlabeled
        }
        pub fn u64_optional_mut(&mut self) -> &mut ::std::option::Option<u64> {
            &mut self.u64_optional
        }
        pub fn u64_repeated_mut(&mut self) -> &mut ::std::vec::Vec<u64> {
            &mut self.u64_repeated
        }
        pub fn s32_unlabeled_mut(&mut self) -> &mut i32 {
            &mut self.s32_unlabeled
        }
        pub fn s32_optional_mut(&mut self) -> &mut ::std::option::Option<i32> {
            &mut self.s32_optional
        }
        pub fn s32_repeated_mut(&mut self) -> &mut ::std::vec::Vec<i32> {
            &mut self.s32_repeated
        }
        pub fn s64_unlabeled_mut(&mut self) -> &mut i64 {
            &mut self.s64_unlabeled
        }
        pub fn s64_optional_mut(&mut self) -> &mut ::std::option::Option<i64> {
            &mut self.s64_optional
        }
        pub fn s64_repeated_mut(&mut self) -> &mut ::std::vec::Vec<i64> {
            &mut self.s64_repeated
        }
        pub fn fixed32_unlabeled_mut(&mut self) -> &mut u32 {
            &mut self.fixed32_unlabeled
        }
        pub fn fixed32_optional_mut(&mut self) -> &mut ::std::option::Option<u32> {
            &mut self.fixed32_optional
        }
        pub fn fixed32_repeated_mut(&mut self) -> &mut ::std::vec::Vec<u32> {
            &mut self.fixed32_repeated
        }
        pub fn fixed64_unlabeled_mut(&mut self) -> &mut u64 {
            &mut self.fixed64_unlabeled
        }
        pub fn fixed64_optional_mut(&mut self) -> &mut ::std::option::Option<u64> {
            &mut self.fixed64_optional
        }
        pub fn fixed64_repeated_mut(&mut self) -> &mut ::std::vec::Vec<u64> {
            &mut self.fixed64_repeated
        }
        pub fn sfixed32_unlabeled_mut(&mut self) -> &mut i32 {
            &mut self.sfixed32_unlabeled
        }
        pub fn sfixed32_optional_mut(&mut self) -> &mut ::std::option::Option<i32> {
            &mut self.sfixed32_optional
        }
        pub fn sfixed32_repeated_mut(&mut self) -> &mut ::std::vec::Vec<i32> {
            &mut self.sfixed32_repeated
        }
        pub fn sfixed64_unlabeled_mut(&mut self) -> &mut i64 {
            &mut self.sfixed64_unlabeled
        }
        pub fn sfixed64_optional_mut(&mut self) -> &mut ::std::option::Option<i64> {
            &mut self.sfixed64_optional
        }
        pub fn sfixed64_repeated_mut(&mut self) -> &mut ::std::vec::Vec<i64> {
            &mut self.sfixed64_repeated
        }
        pub fn f64_unlabeled_mut(&mut self) -> &mut f64 {
            &mut self.f64_unlabeled
        }
        pub fn f64_optional_mut(&mut self) -> &mut ::std::option::Option<f64> {
            &mut self.f64_optional
        }
        pub fn f64_repeated_mut(&mut self) -> &mut ::std::vec::Vec<f64> {
            &mut self.f64_repeated
        }
    }

    impl super::_puroro_traits::MsgTrait for Msg {
        fn i32_unlabeled_opt<'this>(&'this self) -> Option<i32> {
            if self.i32_unlabeled == ::std::default::Default::default() {
                ::std::option::Option::None
            } else {
                ::std::option::Option::Some(self.i32_unlabeled.clone())
            }
        }
        fn i32_optional_opt<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.i32_optional)
        }
        type Field3RepeatedType<'this> =
            ::puroro::CloneThenIntoRepeatedField<'this, ::std::vec::Vec<i32>, i32, i32>;

        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.i32_repeated)
        }
        fn float_unlabeled_opt<'this>(&'this self) -> Option<f32> {
            if self.float_unlabeled == ::std::default::Default::default() {
                ::std::option::Option::None
            } else {
                ::std::option::Option::Some(self.float_unlabeled.clone())
            }
        }
        fn float_optional_opt<'this>(&'this self) -> Option<f32> {
            Clone::clone(&self.float_optional)
        }
        type Field13RepeatedType<'this> =
            ::puroro::CloneThenIntoRepeatedField<'this, ::std::vec::Vec<f32>, f32, f32>;

        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.float_repeated)
        }
        fn bytes_unlabeled_opt<'this>(&'this self) -> Option<&'this [u8]> {
            if self.bytes_unlabeled.is_empty() {
                ::std::option::Option::None
            } else {
                ::std::option::Option::Some(self.bytes_unlabeled.as_ref())
            }
        }
        fn bytes_optional_opt<'this>(&'this self) -> Option<&'this [u8]> {
            self.bytes_optional.as_ref().map(|v| v.as_ref())
        }
        type Field23RepeatedType<'this> = ::puroro::AsRefRepeatedField<
            'this,
            ::std::vec::Vec<::std::vec::Vec<u8>>,
            ::std::vec::Vec<u8>,
            [u8],
        >;

        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::AsRefRepeatedField::new(&self.bytes_repeated)
        }
        fn string_unlabeled_opt<'this>(&'this self) -> Option<&'this str> {
            if self.string_unlabeled.is_empty() {
                ::std::option::Option::None
            } else {
                ::std::option::Option::Some(self.string_unlabeled.as_ref())
            }
        }
        fn string_optional_opt<'this>(&'this self) -> Option<&'this str> {
            self.string_optional.as_ref().map(|v| v.as_ref())
        }
        type Field33RepeatedType<'this> = ::puroro::AsRefRepeatedField<
            'this,
            ::std::vec::Vec<::std::string::String>,
            ::std::string::String,
            str,
        >;

        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::AsRefRepeatedField::new(&self.string_repeated)
        }
        fn enum_unlabeled_opt<'this>(
            &'this self,
        ) -> Option<self::_puroro_root::full_coverage3::Enum> {
            if self.enum_unlabeled == ::std::default::Default::default() {
                ::std::option::Option::None
            } else {
                ::std::option::Option::Some(self.enum_unlabeled.clone())
            }
        }
        fn enum_optional_opt<'this>(
            &'this self,
        ) -> Option<self::_puroro_root::full_coverage3::Enum> {
            Clone::clone(&self.enum_optional)
        }
        type Field43RepeatedType<'this> = ::puroro::CloneThenIntoRepeatedField<
            'this,
            ::std::vec::Vec<self::_puroro_root::full_coverage3::Enum>,
            self::_puroro_root::full_coverage3::Enum,
            self::_puroro_root::full_coverage3::Enum,
        >;

        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.enum_repeated)
        }
        type Field51MessageType<'this> where Self: 'this = &'this self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_simple_impl::Submsg;
        fn submsg_unlabeled_opt<'this>(&'this self) -> Option<Self::Field51MessageType<'this>> {
            self.submsg_unlabeled.as_ref().map(|v| v.as_ref())
        }
        type Field52MessageType<'this> where Self: 'this = &'this self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_simple_impl::Submsg;
        fn submsg_optional_opt<'this>(&'this self) -> Option<Self::Field52MessageType<'this>> {
            self.submsg_optional.as_ref().map(|v| v.as_ref())
        }
        type Field53MessageType<'this> where Self: 'this = &'this self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_simple_impl::Submsg;
        type Field53RepeatedType<'this> = &'this [self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_simple_impl::Submsg];

        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            &self.submsg_repeated
        }
        fn i64_unlabeled_opt<'this>(&'this self) -> Option<i64> {
            if self.i64_unlabeled == ::std::default::Default::default() {
                ::std::option::Option::None
            } else {
                ::std::option::Option::Some(self.i64_unlabeled.clone())
            }
        }
        fn i64_optional_opt<'this>(&'this self) -> Option<i64> {
            Clone::clone(&self.i64_optional)
        }
        type Field103RepeatedType<'this> =
            ::puroro::CloneThenIntoRepeatedField<'this, ::std::vec::Vec<i64>, i64, i64>;

        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.i64_repeated)
        }
        fn u32_unlabeled_opt<'this>(&'this self) -> Option<u32> {
            if self.u32_unlabeled == ::std::default::Default::default() {
                ::std::option::Option::None
            } else {
                ::std::option::Option::Some(self.u32_unlabeled.clone())
            }
        }
        fn u32_optional_opt<'this>(&'this self) -> Option<u32> {
            Clone::clone(&self.u32_optional)
        }
        type Field113RepeatedType<'this> =
            ::puroro::CloneThenIntoRepeatedField<'this, ::std::vec::Vec<u32>, u32, u32>;

        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.u32_repeated)
        }
        fn u64_unlabeled_opt<'this>(&'this self) -> Option<u64> {
            if self.u64_unlabeled == ::std::default::Default::default() {
                ::std::option::Option::None
            } else {
                ::std::option::Option::Some(self.u64_unlabeled.clone())
            }
        }
        fn u64_optional_opt<'this>(&'this self) -> Option<u64> {
            Clone::clone(&self.u64_optional)
        }
        type Field123RepeatedType<'this> =
            ::puroro::CloneThenIntoRepeatedField<'this, ::std::vec::Vec<u64>, u64, u64>;

        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.u64_repeated)
        }
        fn s32_unlabeled_opt<'this>(&'this self) -> Option<i32> {
            if self.s32_unlabeled == ::std::default::Default::default() {
                ::std::option::Option::None
            } else {
                ::std::option::Option::Some(self.s32_unlabeled.clone())
            }
        }
        fn s32_optional_opt<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.s32_optional)
        }
        type Field133RepeatedType<'this> =
            ::puroro::CloneThenIntoRepeatedField<'this, ::std::vec::Vec<i32>, i32, i32>;

        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.s32_repeated)
        }
        fn s64_unlabeled_opt<'this>(&'this self) -> Option<i64> {
            if self.s64_unlabeled == ::std::default::Default::default() {
                ::std::option::Option::None
            } else {
                ::std::option::Option::Some(self.s64_unlabeled.clone())
            }
        }
        fn s64_optional_opt<'this>(&'this self) -> Option<i64> {
            Clone::clone(&self.s64_optional)
        }
        type Field143RepeatedType<'this> =
            ::puroro::CloneThenIntoRepeatedField<'this, ::std::vec::Vec<i64>, i64, i64>;

        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.s64_repeated)
        }
        fn fixed32_unlabeled_opt<'this>(&'this self) -> Option<u32> {
            if self.fixed32_unlabeled == ::std::default::Default::default() {
                ::std::option::Option::None
            } else {
                ::std::option::Option::Some(self.fixed32_unlabeled.clone())
            }
        }
        fn fixed32_optional_opt<'this>(&'this self) -> Option<u32> {
            Clone::clone(&self.fixed32_optional)
        }
        type Field153RepeatedType<'this> =
            ::puroro::CloneThenIntoRepeatedField<'this, ::std::vec::Vec<u32>, u32, u32>;

        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.fixed32_repeated)
        }
        fn fixed64_unlabeled_opt<'this>(&'this self) -> Option<u64> {
            if self.fixed64_unlabeled == ::std::default::Default::default() {
                ::std::option::Option::None
            } else {
                ::std::option::Option::Some(self.fixed64_unlabeled.clone())
            }
        }
        fn fixed64_optional_opt<'this>(&'this self) -> Option<u64> {
            Clone::clone(&self.fixed64_optional)
        }
        type Field163RepeatedType<'this> =
            ::puroro::CloneThenIntoRepeatedField<'this, ::std::vec::Vec<u64>, u64, u64>;

        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.fixed64_repeated)
        }
        fn sfixed32_unlabeled_opt<'this>(&'this self) -> Option<i32> {
            if self.sfixed32_unlabeled == ::std::default::Default::default() {
                ::std::option::Option::None
            } else {
                ::std::option::Option::Some(self.sfixed32_unlabeled.clone())
            }
        }
        fn sfixed32_optional_opt<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.sfixed32_optional)
        }
        type Field173RepeatedType<'this> =
            ::puroro::CloneThenIntoRepeatedField<'this, ::std::vec::Vec<i32>, i32, i32>;

        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.sfixed32_repeated)
        }
        fn sfixed64_unlabeled_opt<'this>(&'this self) -> Option<i64> {
            if self.sfixed64_unlabeled == ::std::default::Default::default() {
                ::std::option::Option::None
            } else {
                ::std::option::Option::Some(self.sfixed64_unlabeled.clone())
            }
        }
        fn sfixed64_optional_opt<'this>(&'this self) -> Option<i64> {
            Clone::clone(&self.sfixed64_optional)
        }
        type Field183RepeatedType<'this> =
            ::puroro::CloneThenIntoRepeatedField<'this, ::std::vec::Vec<i64>, i64, i64>;

        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.sfixed64_repeated)
        }
        fn f64_unlabeled_opt<'this>(&'this self) -> Option<f64> {
            if self.f64_unlabeled == ::std::default::Default::default() {
                ::std::option::Option::None
            } else {
                ::std::option::Option::Some(self.f64_unlabeled.clone())
            }
        }
        fn f64_optional_opt<'this>(&'this self) -> Option<f64> {
            Clone::clone(&self.f64_optional)
        }
        type Field193RepeatedType<'this> =
            ::puroro::CloneThenIntoRepeatedField<'this, ::std::vec::Vec<f64>, f64, f64>;

        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
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
            1 => DeserFieldFromBytesIter::<
                ::puroro::tags::Unlabeled, ::puroro::tags::Int32
            >::deser_field(&mut self.i32_unlabeled, data),
            2 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Int32
            >::deser_field(&mut self.i32_optional, data),
            3 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Int32
            >::deser_field(&mut self.i32_repeated, data),
            11 => DeserFieldFromBytesIter::<
                ::puroro::tags::Unlabeled, ::puroro::tags::Float
            >::deser_field(&mut self.float_unlabeled, data),
            12 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Float
            >::deser_field(&mut self.float_optional, data),
            13 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Float
            >::deser_field(&mut self.float_repeated, data),
            21 => DeserFieldFromBytesIter::<
                ::puroro::tags::Unlabeled, ::puroro::tags::Bytes
            >::deser_field(&mut self.bytes_unlabeled, data),
            22 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bytes
            >::deser_field(&mut self.bytes_optional, data),
            23 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Bytes
            >::deser_field(&mut self.bytes_repeated, data),
            31 => DeserFieldFromBytesIter::<
                ::puroro::tags::Unlabeled, ::puroro::tags::String
            >::deser_field(&mut self.string_unlabeled, data),
            32 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.string_optional, data),
            33 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::String
            >::deser_field(&mut self.string_repeated, data),
            41 => DeserFieldFromBytesIter::<
                ::puroro::tags::Unlabeled, ::puroro::tags::Enum3<self::_puroro_root::full_coverage3::Enum>
            >::deser_field(&mut self.enum_unlabeled, data),
            42 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Enum3<self::_puroro_root::full_coverage3::Enum>
            >::deser_field(&mut self.enum_optional, data),
            43 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Enum3<self::_puroro_root::full_coverage3::Enum>
            >::deser_field(&mut self.enum_repeated, data),
            51 => DeserFieldFromBytesIter::<
                ::puroro::tags::Unlabeled, ::puroro::tags::Message<::std::boxed::Box<self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_simple_impl::Submsg>>
            >::deser_field(&mut self.submsg_unlabeled, data),
            52 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Message<::std::boxed::Box<self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_simple_impl::Submsg>>
            >::deser_field(&mut self.submsg_optional, data),
            53 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_simple_impl::Submsg>
            >::deser_field(&mut self.submsg_repeated, data),
            101 => DeserFieldFromBytesIter::<
                ::puroro::tags::Unlabeled, ::puroro::tags::Int64
            >::deser_field(&mut self.i64_unlabeled, data),
            102 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Int64
            >::deser_field(&mut self.i64_optional, data),
            103 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Int64
            >::deser_field(&mut self.i64_repeated, data),
            111 => DeserFieldFromBytesIter::<
                ::puroro::tags::Unlabeled, ::puroro::tags::UInt32
            >::deser_field(&mut self.u32_unlabeled, data),
            112 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::UInt32
            >::deser_field(&mut self.u32_optional, data),
            113 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::UInt32
            >::deser_field(&mut self.u32_repeated, data),
            121 => DeserFieldFromBytesIter::<
                ::puroro::tags::Unlabeled, ::puroro::tags::UInt64
            >::deser_field(&mut self.u64_unlabeled, data),
            122 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::UInt64
            >::deser_field(&mut self.u64_optional, data),
            123 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::UInt64
            >::deser_field(&mut self.u64_repeated, data),
            131 => DeserFieldFromBytesIter::<
                ::puroro::tags::Unlabeled, ::puroro::tags::SInt32
            >::deser_field(&mut self.s32_unlabeled, data),
            132 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::SInt32
            >::deser_field(&mut self.s32_optional, data),
            133 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::SInt32
            >::deser_field(&mut self.s32_repeated, data),
            141 => DeserFieldFromBytesIter::<
                ::puroro::tags::Unlabeled, ::puroro::tags::SInt64
            >::deser_field(&mut self.s64_unlabeled, data),
            142 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::SInt64
            >::deser_field(&mut self.s64_optional, data),
            143 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::SInt64
            >::deser_field(&mut self.s64_repeated, data),
            151 => DeserFieldFromBytesIter::<
                ::puroro::tags::Unlabeled, ::puroro::tags::Fixed32
            >::deser_field(&mut self.fixed32_unlabeled, data),
            152 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Fixed32
            >::deser_field(&mut self.fixed32_optional, data),
            153 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Fixed32
            >::deser_field(&mut self.fixed32_repeated, data),
            161 => DeserFieldFromBytesIter::<
                ::puroro::tags::Unlabeled, ::puroro::tags::Fixed64
            >::deser_field(&mut self.fixed64_unlabeled, data),
            162 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Fixed64
            >::deser_field(&mut self.fixed64_optional, data),
            163 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Fixed64
            >::deser_field(&mut self.fixed64_repeated, data),
            171 => DeserFieldFromBytesIter::<
                ::puroro::tags::Unlabeled, ::puroro::tags::SFixed32
            >::deser_field(&mut self.sfixed32_unlabeled, data),
            172 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::SFixed32
            >::deser_field(&mut self.sfixed32_optional, data),
            173 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::SFixed32
            >::deser_field(&mut self.sfixed32_repeated, data),
            181 => DeserFieldFromBytesIter::<
                ::puroro::tags::Unlabeled, ::puroro::tags::SFixed64
            >::deser_field(&mut self.sfixed64_unlabeled, data),
            182 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::SFixed64
            >::deser_field(&mut self.sfixed64_optional, data),
            183 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::SFixed64
            >::deser_field(&mut self.sfixed64_repeated, data),
            191 => DeserFieldFromBytesIter::<
                ::puroro::tags::Unlabeled, ::puroro::tags::Double
            >::deser_field(&mut self.f64_unlabeled, data),
            192 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Double
            >::deser_field(&mut self.f64_optional, data),
            193 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Double
            >::deser_field(&mut self.f64_repeated, data),

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
                    <Self as super::_puroro_traits::MsgTrait>::Field51MessageType<'_>,
                >,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::submsg_unlabeled_opt(self),
                51,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    <Self as super::_puroro_traits::MsgTrait>::Field52MessageType<'_>,
                >,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::submsg_optional_opt(self),
                52,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    <Self as super::_puroro_traits::MsgTrait>::Field53MessageType<'_>,
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
                .field(
                    "i32_unlabeled",
                    &<Self as super::_puroro_traits::MsgTrait>::i32_unlabeled(self),
                )
                .field(
                    "i32_optional",
                    &<Self as super::_puroro_traits::MsgTrait>::i32_optional_opt(self),
                )
                .field(
                    "i32_repeated",
                    &<Self as super::_puroro_traits::MsgTrait>::i32_repeated(self)
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>(),
                )
                .field(
                    "float_unlabeled",
                    &<Self as super::_puroro_traits::MsgTrait>::float_unlabeled(self),
                )
                .field(
                    "float_optional",
                    &<Self as super::_puroro_traits::MsgTrait>::float_optional_opt(self),
                )
                .field(
                    "float_repeated",
                    &<Self as super::_puroro_traits::MsgTrait>::float_repeated(self)
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>(),
                )
                .field(
                    "bytes_unlabeled",
                    &<Self as super::_puroro_traits::MsgTrait>::bytes_unlabeled(self),
                )
                .field(
                    "bytes_optional",
                    &<Self as super::_puroro_traits::MsgTrait>::bytes_optional_opt(self),
                )
                .field(
                    "bytes_repeated",
                    &<Self as super::_puroro_traits::MsgTrait>::bytes_repeated(self)
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>(),
                )
                .field(
                    "string_unlabeled",
                    &<Self as super::_puroro_traits::MsgTrait>::string_unlabeled(self),
                )
                .field(
                    "string_optional",
                    &<Self as super::_puroro_traits::MsgTrait>::string_optional_opt(self),
                )
                .field(
                    "string_repeated",
                    &<Self as super::_puroro_traits::MsgTrait>::string_repeated(self)
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>(),
                )
                .field(
                    "enum_unlabeled",
                    &<Self as super::_puroro_traits::MsgTrait>::enum_unlabeled(self),
                )
                .field(
                    "enum_optional",
                    &<Self as super::_puroro_traits::MsgTrait>::enum_optional_opt(self),
                )
                .field(
                    "enum_repeated",
                    &<Self as super::_puroro_traits::MsgTrait>::enum_repeated(self)
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>(),
                )
                .field(
                    "submsg_unlabeled",
                    &<Self as super::_puroro_traits::MsgTrait>::submsg_unlabeled(self),
                )
                .field(
                    "submsg_optional",
                    &<Self as super::_puroro_traits::MsgTrait>::submsg_optional(self),
                )
                .field(
                    "submsg_repeated",
                    &<Self as super::_puroro_traits::MsgTrait>::submsg_repeated(self)
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>(),
                )
                .field(
                    "i64_unlabeled",
                    &<Self as super::_puroro_traits::MsgTrait>::i64_unlabeled(self),
                )
                .field(
                    "i64_optional",
                    &<Self as super::_puroro_traits::MsgTrait>::i64_optional_opt(self),
                )
                .field(
                    "i64_repeated",
                    &<Self as super::_puroro_traits::MsgTrait>::i64_repeated(self)
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>(),
                )
                .field(
                    "u32_unlabeled",
                    &<Self as super::_puroro_traits::MsgTrait>::u32_unlabeled(self),
                )
                .field(
                    "u32_optional",
                    &<Self as super::_puroro_traits::MsgTrait>::u32_optional_opt(self),
                )
                .field(
                    "u32_repeated",
                    &<Self as super::_puroro_traits::MsgTrait>::u32_repeated(self)
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>(),
                )
                .field(
                    "u64_unlabeled",
                    &<Self as super::_puroro_traits::MsgTrait>::u64_unlabeled(self),
                )
                .field(
                    "u64_optional",
                    &<Self as super::_puroro_traits::MsgTrait>::u64_optional_opt(self),
                )
                .field(
                    "u64_repeated",
                    &<Self as super::_puroro_traits::MsgTrait>::u64_repeated(self)
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>(),
                )
                .field(
                    "s32_unlabeled",
                    &<Self as super::_puroro_traits::MsgTrait>::s32_unlabeled(self),
                )
                .field(
                    "s32_optional",
                    &<Self as super::_puroro_traits::MsgTrait>::s32_optional_opt(self),
                )
                .field(
                    "s32_repeated",
                    &<Self as super::_puroro_traits::MsgTrait>::s32_repeated(self)
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>(),
                )
                .field(
                    "s64_unlabeled",
                    &<Self as super::_puroro_traits::MsgTrait>::s64_unlabeled(self),
                )
                .field(
                    "s64_optional",
                    &<Self as super::_puroro_traits::MsgTrait>::s64_optional_opt(self),
                )
                .field(
                    "s64_repeated",
                    &<Self as super::_puroro_traits::MsgTrait>::s64_repeated(self)
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>(),
                )
                .field(
                    "fixed32_unlabeled",
                    &<Self as super::_puroro_traits::MsgTrait>::fixed32_unlabeled(self),
                )
                .field(
                    "fixed32_optional",
                    &<Self as super::_puroro_traits::MsgTrait>::fixed32_optional_opt(self),
                )
                .field(
                    "fixed32_repeated",
                    &<Self as super::_puroro_traits::MsgTrait>::fixed32_repeated(self)
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>(),
                )
                .field(
                    "fixed64_unlabeled",
                    &<Self as super::_puroro_traits::MsgTrait>::fixed64_unlabeled(self),
                )
                .field(
                    "fixed64_optional",
                    &<Self as super::_puroro_traits::MsgTrait>::fixed64_optional_opt(self),
                )
                .field(
                    "fixed64_repeated",
                    &<Self as super::_puroro_traits::MsgTrait>::fixed64_repeated(self)
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>(),
                )
                .field(
                    "sfixed32_unlabeled",
                    &<Self as super::_puroro_traits::MsgTrait>::sfixed32_unlabeled(self),
                )
                .field(
                    "sfixed32_optional",
                    &<Self as super::_puroro_traits::MsgTrait>::sfixed32_optional_opt(self),
                )
                .field(
                    "sfixed32_repeated",
                    &<Self as super::_puroro_traits::MsgTrait>::sfixed32_repeated(self)
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>(),
                )
                .field(
                    "sfixed64_unlabeled",
                    &<Self as super::_puroro_traits::MsgTrait>::sfixed64_unlabeled(self),
                )
                .field(
                    "sfixed64_optional",
                    &<Self as super::_puroro_traits::MsgTrait>::sfixed64_optional_opt(self),
                )
                .field(
                    "sfixed64_repeated",
                    &<Self as super::_puroro_traits::MsgTrait>::sfixed64_repeated(self)
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>(),
                )
                .field(
                    "f64_unlabeled",
                    &<Self as super::_puroro_traits::MsgTrait>::f64_unlabeled(self),
                )
                .field(
                    "f64_optional",
                    &<Self as super::_puroro_traits::MsgTrait>::f64_optional_opt(self),
                )
                .field(
                    "f64_repeated",
                    &<Self as super::_puroro_traits::MsgTrait>::f64_repeated(self)
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>(),
                )
                .finish()
        }
    }

    impl ::std::clone::Clone for Msg {
        fn clone(&self) -> Self {
            Self {
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
            self.i32_unlabeled == rhs.i32_unlabeled
                && self.i32_optional == rhs.i32_optional
                && self.i32_repeated == rhs.i32_repeated
                && self.float_unlabeled == rhs.float_unlabeled
                && self.float_optional == rhs.float_optional
                && self.float_repeated == rhs.float_repeated
                && self.bytes_unlabeled == rhs.bytes_unlabeled
                && self.bytes_optional == rhs.bytes_optional
                && self.bytes_repeated == rhs.bytes_repeated
                && self.string_unlabeled == rhs.string_unlabeled
                && self.string_optional == rhs.string_optional
                && self.string_repeated == rhs.string_repeated
                && self.enum_unlabeled == rhs.enum_unlabeled
                && self.enum_optional == rhs.enum_optional
                && self.enum_repeated == rhs.enum_repeated
                && self.submsg_unlabeled == rhs.submsg_unlabeled
                && self.submsg_optional == rhs.submsg_optional
                && self.submsg_repeated == rhs.submsg_repeated
                && self.i64_unlabeled == rhs.i64_unlabeled
                && self.i64_optional == rhs.i64_optional
                && self.i64_repeated == rhs.i64_repeated
                && self.u32_unlabeled == rhs.u32_unlabeled
                && self.u32_optional == rhs.u32_optional
                && self.u32_repeated == rhs.u32_repeated
                && self.u64_unlabeled == rhs.u64_unlabeled
                && self.u64_optional == rhs.u64_optional
                && self.u64_repeated == rhs.u64_repeated
                && self.s32_unlabeled == rhs.s32_unlabeled
                && self.s32_optional == rhs.s32_optional
                && self.s32_repeated == rhs.s32_repeated
                && self.s64_unlabeled == rhs.s64_unlabeled
                && self.s64_optional == rhs.s64_optional
                && self.s64_repeated == rhs.s64_repeated
                && self.fixed32_unlabeled == rhs.fixed32_unlabeled
                && self.fixed32_optional == rhs.fixed32_optional
                && self.fixed32_repeated == rhs.fixed32_repeated
                && self.fixed64_unlabeled == rhs.fixed64_unlabeled
                && self.fixed64_optional == rhs.fixed64_optional
                && self.fixed64_repeated == rhs.fixed64_repeated
                && self.sfixed32_unlabeled == rhs.sfixed32_unlabeled
                && self.sfixed32_optional == rhs.sfixed32_optional
                && self.sfixed32_repeated == rhs.sfixed32_repeated
                && self.sfixed64_unlabeled == rhs.sfixed64_unlabeled
                && self.sfixed64_optional == rhs.sfixed64_optional
                && self.sfixed64_repeated == rhs.sfixed64_repeated
                && self.f64_unlabeled == rhs.f64_unlabeled
                && self.f64_optional == rhs.f64_optional
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

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub i32_unlabeled: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField1<ScalarType> where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn i32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.i32_unlabeled,
            )))
        }
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
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
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                i32_unlabeled: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub i32_optional: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField2<ScalarType> where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn i32_optional_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.i32_optional,
            )))
        }
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
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
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                i32_optional: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField3<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub i32_repeated: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::Msg>
        for MsgSingleField3<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::MsgTrait
        for MsgSingleField3<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<'this, RepeatedType, ScalarType, i32>;

        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.i32_repeated)
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::se::SerMessageToIoWrite
        for MsgSingleField3<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
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
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                i32_repeated: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField11<ScalarType>
    where
        ScalarType: ::std::convert::Into<f32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub float_unlabeled: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField11<ScalarType> where
        ScalarType: ::std::convert::Into<f32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField11<ScalarType>
    where
        ScalarType: ::std::convert::Into<f32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn float_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<f32> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.float_unlabeled,
            )))
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField11<ScalarType>
    where
        ScalarType: ::std::convert::Into<f32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
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
        ScalarType: ::std::convert::Into<f32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                float_unlabeled: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField12<ScalarType>
    where
        ScalarType: ::std::convert::Into<f32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub float_optional: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField12<ScalarType> where
        ScalarType: ::std::convert::Into<f32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField12<ScalarType>
    where
        ScalarType: ::std::convert::Into<f32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn float_optional_opt<'this>(&'this self) -> ::std::option::Option<f32> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.float_optional,
            )))
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField12<ScalarType>
    where
        ScalarType: ::std::convert::Into<f32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
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
        ScalarType: ::std::convert::Into<f32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                float_optional: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField13<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<f32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub float_repeated: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::Msg>
        for MsgSingleField13<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<f32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::MsgTrait
        for MsgSingleField13<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<f32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<'this, RepeatedType, ScalarType, f32>;

        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.float_repeated)
        }
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::se::SerMessageToIoWrite
        for MsgSingleField13<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<f32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
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
        ScalarType: ::std::convert::Into<f32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                float_repeated: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField21<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<[u8]>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub bytes_unlabeled: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField21<ScalarType> where
        ScalarType: ::std::convert::AsRef<[u8]>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField21<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<[u8]>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn bytes_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<&'this [u8]> {
            ::std::option::Option::Some(self.bytes_unlabeled.as_ref())
        }
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField21<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<[u8]>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
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
        ScalarType: ::std::convert::AsRef<[u8]>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                bytes_unlabeled: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField22<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<[u8]>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub bytes_optional: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField22<ScalarType> where
        ScalarType: ::std::convert::AsRef<[u8]>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField22<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<[u8]>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn bytes_optional_opt<'this>(&'this self) -> ::std::option::Option<&'this [u8]> {
            ::std::option::Option::Some(self.bytes_optional.as_ref())
        }
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField22<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<[u8]>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
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
        ScalarType: ::std::convert::AsRef<[u8]>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                bytes_optional: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField23<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::AsRef<[u8]>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub bytes_repeated: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::Msg>
        for MsgSingleField23<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::AsRef<[u8]>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::MsgTrait
        for MsgSingleField23<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::AsRef<[u8]>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::AsRefRepeatedField<'this, RepeatedType, ScalarType, [u8]>;

        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::AsRefRepeatedField::new(&self.bytes_repeated)
        }
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::se::SerMessageToIoWrite
        for MsgSingleField23<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::AsRef<[u8]>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
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
        ScalarType: ::std::convert::AsRef<[u8]>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                bytes_repeated: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField31<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub string_unlabeled: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField31<ScalarType> where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField31<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn string_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::Some(self.string_unlabeled.as_ref())
        }
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField31<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
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
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                string_unlabeled: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField32<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub string_optional: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField32<ScalarType> where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField32<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn string_optional_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::Some(self.string_optional.as_ref())
        }
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField32<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
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
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                string_optional: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField33<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub string_repeated: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::Msg>
        for MsgSingleField33<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::MsgTrait
        for MsgSingleField33<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::AsRefRepeatedField<'this, RepeatedType, ScalarType, str>;

        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::AsRefRepeatedField::new(&self.string_repeated)
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::se::SerMessageToIoWrite
        for MsgSingleField33<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
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
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                string_repeated: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField41<ScalarType>
    where
        ScalarType: ::std::convert::Into<self::_puroro_root::full_coverage3::Enum>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub enum_unlabeled: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField41<ScalarType> where
        ScalarType: ::std::convert::Into<self::_puroro_root::full_coverage3::Enum>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField41<ScalarType>
    where
        ScalarType: ::std::convert::Into<self::_puroro_root::full_coverage3::Enum>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn enum_unlabeled_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::full_coverage3::Enum> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.enum_unlabeled,
            )))
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField41<ScalarType>
    where
        ScalarType: ::std::convert::Into<self::_puroro_root::full_coverage3::Enum>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
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
        ScalarType: ::std::convert::Into<self::_puroro_root::full_coverage3::Enum>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                enum_unlabeled: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField42<ScalarType>
    where
        ScalarType: ::std::convert::Into<self::_puroro_root::full_coverage3::Enum>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub enum_optional: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField42<ScalarType> where
        ScalarType: ::std::convert::Into<self::_puroro_root::full_coverage3::Enum>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField42<ScalarType>
    where
        ScalarType: ::std::convert::Into<self::_puroro_root::full_coverage3::Enum>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn enum_optional_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::full_coverage3::Enum> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.enum_optional,
            )))
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField42<ScalarType>
    where
        ScalarType: ::std::convert::Into<self::_puroro_root::full_coverage3::Enum>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
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
        ScalarType: ::std::convert::Into<self::_puroro_root::full_coverage3::Enum>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                enum_optional: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField43<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<self::_puroro_root::full_coverage3::Enum>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub enum_repeated: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::Msg>
        for MsgSingleField43<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<self::_puroro_root::full_coverage3::Enum>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::MsgTrait
        for MsgSingleField43<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<self::_puroro_root::full_coverage3::Enum>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<
            'this,
            RepeatedType,
            ScalarType,
            self::_puroro_root::full_coverage3::Enum,
        >;

        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.enum_repeated)
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::se::SerMessageToIoWrite
        for MsgSingleField43<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<self::_puroro_root::full_coverage3::Enum>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
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
        ScalarType: ::std::convert::Into<self::_puroro_root::full_coverage3::Enum>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                enum_repeated: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField51<ScalarType>
    where
        ScalarType:
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
    {
        pub submsg_unlabeled: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField51<ScalarType> where
        ScalarType:
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField51<ScalarType>
    where
        ScalarType:
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = &'this ScalarType;

        fn submsg_unlabeled_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field51MessageType<'this>> {
            ::std::option::Option::Some(&self.submsg_unlabeled)
        }
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField51<ScalarType>
    where
        ScalarType:
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        Self: super::_puroro_traits::MsgTrait,
        for<'a> <Self as super::_puroro_traits::MsgTrait>::Field51MessageType<'a>:
            ::puroro::internal::se::SerMessageToIoWrite,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Message<
                    <Self as super::_puroro_traits::MsgTrait>::Field51MessageType<'_>,
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
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                submsg_unlabeled: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField52<ScalarType>
    where
        ScalarType:
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
    {
        pub submsg_optional: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField52<ScalarType> where
        ScalarType:
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField52<ScalarType>
    where
        ScalarType:
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = &'this ScalarType;

        fn submsg_optional_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field52MessageType<'this>> {
            ::std::option::Option::Some(&self.submsg_optional)
        }
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField52<ScalarType>
    where
        ScalarType:
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        Self: super::_puroro_traits::MsgTrait,
        for<'a> <Self as super::_puroro_traits::MsgTrait>::Field52MessageType<'a>:
            ::puroro::internal::se::SerMessageToIoWrite,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    <Self as super::_puroro_traits::MsgTrait>::Field52MessageType<'_>,
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
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                submsg_optional: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField53<ScalarType, RepeatedType>
    where
        ScalarType:
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub submsg_repeated: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::Msg>
        for MsgSingleField53<ScalarType, RepeatedType>
    where
        ScalarType:
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::MsgTrait
        for MsgSingleField53<ScalarType, RepeatedType>
    where
        ScalarType:
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = &'this ScalarType;
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = &'this RepeatedType;

        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            &self.submsg_repeated
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::se::SerMessageToIoWrite
        for MsgSingleField53<ScalarType, RepeatedType>
    where
        ScalarType:
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        Self: super::_puroro_traits::MsgTrait,
        for<'a> <Self as super::_puroro_traits::MsgTrait>::Field53MessageType<'a>:
            ::puroro::internal::se::SerMessageToIoWrite,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    <Self as super::_puroro_traits::MsgTrait>::Field53MessageType<'_>,
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
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                submsg_repeated: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField101<ScalarType>
    where
        ScalarType: ::std::convert::Into<i64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub i64_unlabeled: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField101<ScalarType> where
        ScalarType: ::std::convert::Into<i64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField101<ScalarType>
    where
        ScalarType: ::std::convert::Into<i64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn i64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.i64_unlabeled,
            )))
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField101<ScalarType>
    where
        ScalarType: ::std::convert::Into<i64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
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
        ScalarType: ::std::convert::Into<i64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                i64_unlabeled: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField102<ScalarType>
    where
        ScalarType: ::std::convert::Into<i64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub i64_optional: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField102<ScalarType> where
        ScalarType: ::std::convert::Into<i64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField102<ScalarType>
    where
        ScalarType: ::std::convert::Into<i64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn i64_optional_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.i64_optional,
            )))
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField102<ScalarType>
    where
        ScalarType: ::std::convert::Into<i64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
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
        ScalarType: ::std::convert::Into<i64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                i64_optional: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField103<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub i64_repeated: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::Msg>
        for MsgSingleField103<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::MsgTrait
        for MsgSingleField103<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<'this, RepeatedType, ScalarType, i64>;

        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.i64_repeated)
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::se::SerMessageToIoWrite
        for MsgSingleField103<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
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
        ScalarType: ::std::convert::Into<i64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                i64_repeated: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField111<ScalarType>
    where
        ScalarType: ::std::convert::Into<u32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub u32_unlabeled: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField111<ScalarType> where
        ScalarType: ::std::convert::Into<u32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField111<ScalarType>
    where
        ScalarType: ::std::convert::Into<u32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn u32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<u32> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.u32_unlabeled,
            )))
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField111<ScalarType>
    where
        ScalarType: ::std::convert::Into<u32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
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
        ScalarType: ::std::convert::Into<u32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                u32_unlabeled: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField112<ScalarType>
    where
        ScalarType: ::std::convert::Into<u32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub u32_optional: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField112<ScalarType> where
        ScalarType: ::std::convert::Into<u32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField112<ScalarType>
    where
        ScalarType: ::std::convert::Into<u32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn u32_optional_opt<'this>(&'this self) -> ::std::option::Option<u32> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.u32_optional,
            )))
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField112<ScalarType>
    where
        ScalarType: ::std::convert::Into<u32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
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
        ScalarType: ::std::convert::Into<u32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                u32_optional: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField113<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<u32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub u32_repeated: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::Msg>
        for MsgSingleField113<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<u32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::MsgTrait
        for MsgSingleField113<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<u32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<'this, RepeatedType, ScalarType, u32>;

        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.u32_repeated)
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::se::SerMessageToIoWrite
        for MsgSingleField113<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<u32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
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
        ScalarType: ::std::convert::Into<u32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                u32_repeated: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField121<ScalarType>
    where
        ScalarType: ::std::convert::Into<u64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub u64_unlabeled: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField121<ScalarType> where
        ScalarType: ::std::convert::Into<u64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField121<ScalarType>
    where
        ScalarType: ::std::convert::Into<u64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn u64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<u64> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.u64_unlabeled,
            )))
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField121<ScalarType>
    where
        ScalarType: ::std::convert::Into<u64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
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
        ScalarType: ::std::convert::Into<u64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                u64_unlabeled: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField122<ScalarType>
    where
        ScalarType: ::std::convert::Into<u64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub u64_optional: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField122<ScalarType> where
        ScalarType: ::std::convert::Into<u64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField122<ScalarType>
    where
        ScalarType: ::std::convert::Into<u64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn u64_optional_opt<'this>(&'this self) -> ::std::option::Option<u64> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.u64_optional,
            )))
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField122<ScalarType>
    where
        ScalarType: ::std::convert::Into<u64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
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
        ScalarType: ::std::convert::Into<u64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                u64_optional: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField123<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<u64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub u64_repeated: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::Msg>
        for MsgSingleField123<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<u64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::MsgTrait
        for MsgSingleField123<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<u64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<'this, RepeatedType, ScalarType, u64>;

        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.u64_repeated)
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::se::SerMessageToIoWrite
        for MsgSingleField123<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<u64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
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
        ScalarType: ::std::convert::Into<u64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                u64_repeated: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField131<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub s32_unlabeled: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField131<ScalarType> where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField131<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn s32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.s32_unlabeled,
            )))
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField131<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
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
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                s32_unlabeled: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField132<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub s32_optional: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField132<ScalarType> where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField132<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn s32_optional_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.s32_optional,
            )))
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField132<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
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
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                s32_optional: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField133<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub s32_repeated: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::Msg>
        for MsgSingleField133<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::MsgTrait
        for MsgSingleField133<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<'this, RepeatedType, ScalarType, i32>;

        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.s32_repeated)
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::se::SerMessageToIoWrite
        for MsgSingleField133<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
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
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                s32_repeated: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField141<ScalarType>
    where
        ScalarType: ::std::convert::Into<i64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub s64_unlabeled: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField141<ScalarType> where
        ScalarType: ::std::convert::Into<i64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField141<ScalarType>
    where
        ScalarType: ::std::convert::Into<i64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn s64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.s64_unlabeled,
            )))
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField141<ScalarType>
    where
        ScalarType: ::std::convert::Into<i64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
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
        ScalarType: ::std::convert::Into<i64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                s64_unlabeled: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField142<ScalarType>
    where
        ScalarType: ::std::convert::Into<i64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub s64_optional: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField142<ScalarType> where
        ScalarType: ::std::convert::Into<i64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField142<ScalarType>
    where
        ScalarType: ::std::convert::Into<i64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn s64_optional_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.s64_optional,
            )))
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField142<ScalarType>
    where
        ScalarType: ::std::convert::Into<i64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
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
        ScalarType: ::std::convert::Into<i64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                s64_optional: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField143<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub s64_repeated: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::Msg>
        for MsgSingleField143<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::MsgTrait
        for MsgSingleField143<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<'this, RepeatedType, ScalarType, i64>;

        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.s64_repeated)
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::se::SerMessageToIoWrite
        for MsgSingleField143<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
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
        ScalarType: ::std::convert::Into<i64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                s64_repeated: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField151<ScalarType>
    where
        ScalarType: ::std::convert::Into<u32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub fixed32_unlabeled: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField151<ScalarType> where
        ScalarType: ::std::convert::Into<u32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField151<ScalarType>
    where
        ScalarType: ::std::convert::Into<u32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn fixed32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<u32> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.fixed32_unlabeled,
            )))
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField151<ScalarType>
    where
        ScalarType: ::std::convert::Into<u32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
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
        ScalarType: ::std::convert::Into<u32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                fixed32_unlabeled: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField152<ScalarType>
    where
        ScalarType: ::std::convert::Into<u32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub fixed32_optional: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField152<ScalarType> where
        ScalarType: ::std::convert::Into<u32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField152<ScalarType>
    where
        ScalarType: ::std::convert::Into<u32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn fixed32_optional_opt<'this>(&'this self) -> ::std::option::Option<u32> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.fixed32_optional,
            )))
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField152<ScalarType>
    where
        ScalarType: ::std::convert::Into<u32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
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
        ScalarType: ::std::convert::Into<u32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                fixed32_optional: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField153<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<u32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub fixed32_repeated: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::Msg>
        for MsgSingleField153<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<u32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::MsgTrait
        for MsgSingleField153<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<u32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<'this, RepeatedType, ScalarType, u32>;

        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.fixed32_repeated)
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::se::SerMessageToIoWrite
        for MsgSingleField153<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<u32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
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
        ScalarType: ::std::convert::Into<u32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                fixed32_repeated: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField161<ScalarType>
    where
        ScalarType: ::std::convert::Into<u64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub fixed64_unlabeled: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField161<ScalarType> where
        ScalarType: ::std::convert::Into<u64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField161<ScalarType>
    where
        ScalarType: ::std::convert::Into<u64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn fixed64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<u64> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.fixed64_unlabeled,
            )))
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField161<ScalarType>
    where
        ScalarType: ::std::convert::Into<u64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
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
        ScalarType: ::std::convert::Into<u64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                fixed64_unlabeled: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField162<ScalarType>
    where
        ScalarType: ::std::convert::Into<u64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub fixed64_optional: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField162<ScalarType> where
        ScalarType: ::std::convert::Into<u64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField162<ScalarType>
    where
        ScalarType: ::std::convert::Into<u64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn fixed64_optional_opt<'this>(&'this self) -> ::std::option::Option<u64> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.fixed64_optional,
            )))
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField162<ScalarType>
    where
        ScalarType: ::std::convert::Into<u64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
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
        ScalarType: ::std::convert::Into<u64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                fixed64_optional: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField163<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<u64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub fixed64_repeated: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::Msg>
        for MsgSingleField163<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<u64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::MsgTrait
        for MsgSingleField163<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<u64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<'this, RepeatedType, ScalarType, u64>;

        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.fixed64_repeated)
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::se::SerMessageToIoWrite
        for MsgSingleField163<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<u64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
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
        ScalarType: ::std::convert::Into<u64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                fixed64_repeated: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField171<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub sfixed32_unlabeled: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField171<ScalarType> where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField171<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn sfixed32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.sfixed32_unlabeled,
            )))
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField171<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
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
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                sfixed32_unlabeled: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField172<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub sfixed32_optional: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField172<ScalarType> where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField172<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn sfixed32_optional_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.sfixed32_optional,
            )))
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField172<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
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
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                sfixed32_optional: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField173<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub sfixed32_repeated: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::Msg>
        for MsgSingleField173<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::MsgTrait
        for MsgSingleField173<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<'this, RepeatedType, ScalarType, i32>;

        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.sfixed32_repeated)
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::se::SerMessageToIoWrite
        for MsgSingleField173<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
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
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                sfixed32_repeated: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField181<ScalarType>
    where
        ScalarType: ::std::convert::Into<i64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub sfixed64_unlabeled: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField181<ScalarType> where
        ScalarType: ::std::convert::Into<i64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField181<ScalarType>
    where
        ScalarType: ::std::convert::Into<i64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn sfixed64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.sfixed64_unlabeled,
            )))
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField181<ScalarType>
    where
        ScalarType: ::std::convert::Into<i64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
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
        ScalarType: ::std::convert::Into<i64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                sfixed64_unlabeled: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField182<ScalarType>
    where
        ScalarType: ::std::convert::Into<i64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub sfixed64_optional: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField182<ScalarType> where
        ScalarType: ::std::convert::Into<i64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField182<ScalarType>
    where
        ScalarType: ::std::convert::Into<i64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn sfixed64_optional_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.sfixed64_optional,
            )))
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField182<ScalarType>
    where
        ScalarType: ::std::convert::Into<i64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
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
        ScalarType: ::std::convert::Into<i64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                sfixed64_optional: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField183<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub sfixed64_repeated: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::Msg>
        for MsgSingleField183<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::MsgTrait
        for MsgSingleField183<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<'this, RepeatedType, ScalarType, i64>;

        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.sfixed64_repeated)
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::se::SerMessageToIoWrite
        for MsgSingleField183<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
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
        ScalarType: ::std::convert::Into<i64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                sfixed64_repeated: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField191<ScalarType>
    where
        ScalarType: ::std::convert::Into<f64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub f64_unlabeled: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField191<ScalarType> where
        ScalarType: ::std::convert::Into<f64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField191<ScalarType>
    where
        ScalarType: ::std::convert::Into<f64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn f64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<f64> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.f64_unlabeled,
            )))
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField191<ScalarType>
    where
        ScalarType: ::std::convert::Into<f64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
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
        ScalarType: ::std::convert::Into<f64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                f64_unlabeled: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField192<ScalarType>
    where
        ScalarType: ::std::convert::Into<f64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub f64_optional: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField192<ScalarType> where
        ScalarType: ::std::convert::Into<f64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField192<ScalarType>
    where
        ScalarType: ::std::convert::Into<f64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn f64_optional_opt<'this>(&'this self) -> ::std::option::Option<f64> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.f64_optional,
            )))
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField192<ScalarType>
    where
        ScalarType: ::std::convert::Into<f64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
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
        ScalarType: ::std::convert::Into<f64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                f64_optional: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField193<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<f64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub f64_repeated: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::Msg>
        for MsgSingleField193<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<f64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::MsgTrait
        for MsgSingleField193<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<f64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<'this, RepeatedType, ScalarType, f64>;

        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.f64_repeated)
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::se::SerMessageToIoWrite
        for MsgSingleField193<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<f64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
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
        ScalarType: ::std::convert::Into<f64>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                f64_repeated: value,
            }
        }
    }
    pub struct MsgBumpalo<'bump> {
    _bump: &'bump ::puroro::bumpalo::Bump,
    _bitfield: ::puroro::bitvec::array::BitArray<
        ::puroro::bitvec::order::Lsb0,
        [u32; (15 + 31) / 32],
    >,
    i32_unlabeled: i32,
    i32_optional: i32,
    i32_repeated: ::puroro::internal::NoAllocBumpVec<i32>,
    float_unlabeled: f32,
    float_optional: f32,
    float_repeated: ::puroro::internal::NoAllocBumpVec<f32>,
    bytes_unlabeled: ::puroro::internal::NoAllocBumpVec<u8>,
    bytes_optional: ::puroro::internal::NoAllocBumpVec<u8>,
    bytes_repeated: ::puroro::internal::NoAllocBumpVec<::puroro::internal::NoAllocBumpVec<u8>>,
    string_unlabeled: ::puroro::internal::NoAllocBumpString,
    string_optional: ::puroro::internal::NoAllocBumpString,
    string_repeated: ::puroro::internal::NoAllocBumpVec<::puroro::internal::NoAllocBumpString>,
    enum_unlabeled: self::_puroro_root::full_coverage3::Enum,
    enum_optional: self::_puroro_root::full_coverage3::Enum,
    enum_repeated: ::puroro::internal::NoAllocBumpVec<self::_puroro_root::full_coverage3::Enum>,
    submsg_unlabeled: ::std::option::Option<::puroro::internal::NoAllocBumpBox<self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgBumpalo<'bump>>>,
    submsg_optional: ::std::option::Option<::puroro::internal::NoAllocBumpBox<self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgBumpalo<'bump>>>,
    submsg_repeated: ::puroro::internal::NoAllocBumpVec<self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgBumpalo<'bump>>,
    i64_unlabeled: i64,
    i64_optional: i64,
    i64_repeated: ::puroro::internal::NoAllocBumpVec<i64>,
    u32_unlabeled: u32,
    u32_optional: u32,
    u32_repeated: ::puroro::internal::NoAllocBumpVec<u32>,
    u64_unlabeled: u64,
    u64_optional: u64,
    u64_repeated: ::puroro::internal::NoAllocBumpVec<u64>,
    s32_unlabeled: i32,
    s32_optional: i32,
    s32_repeated: ::puroro::internal::NoAllocBumpVec<i32>,
    s64_unlabeled: i64,
    s64_optional: i64,
    s64_repeated: ::puroro::internal::NoAllocBumpVec<i64>,
    fixed32_unlabeled: u32,
    fixed32_optional: u32,
    fixed32_repeated: ::puroro::internal::NoAllocBumpVec<u32>,
    fixed64_unlabeled: u64,
    fixed64_optional: u64,
    fixed64_repeated: ::puroro::internal::NoAllocBumpVec<u64>,
    sfixed32_unlabeled: i32,
    sfixed32_optional: i32,
    sfixed32_repeated: ::puroro::internal::NoAllocBumpVec<i32>,
    sfixed64_unlabeled: i64,
    sfixed64_optional: i64,
    sfixed64_repeated: ::puroro::internal::NoAllocBumpVec<i64>,
    f64_unlabeled: f64,
    f64_optional: f64,
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
                i32_repeated: ::puroro::internal::NoAllocBumpVec::new_in(bump),
                float_unlabeled: ::std::default::Default::default(),
                float_optional: ::std::default::Default::default(),
                float_repeated: ::puroro::internal::NoAllocBumpVec::new_in(bump),
                bytes_unlabeled: ::puroro::internal::NoAllocBumpVec::new_in(bump),
                bytes_optional: ::puroro::internal::NoAllocBumpVec::new_in(bump),
                bytes_repeated: ::puroro::internal::NoAllocBumpVec::new_in(bump),
                string_unlabeled: ::puroro::internal::NoAllocBumpString::new_in(bump),
                string_optional: ::puroro::internal::NoAllocBumpString::new_in(bump),
                string_repeated: ::puroro::internal::NoAllocBumpVec::new_in(bump),
                enum_unlabeled: ::std::default::Default::default(),
                enum_optional: ::std::default::Default::default(),
                enum_repeated: ::puroro::internal::NoAllocBumpVec::new_in(bump),
                submsg_unlabeled: ::std::option::Option::None,
                submsg_optional: ::std::option::Option::None,
                submsg_repeated: ::puroro::internal::NoAllocBumpVec::new_in(bump),
                i64_unlabeled: ::std::default::Default::default(),
                i64_optional: ::std::default::Default::default(),
                i64_repeated: ::puroro::internal::NoAllocBumpVec::new_in(bump),
                u32_unlabeled: ::std::default::Default::default(),
                u32_optional: ::std::default::Default::default(),
                u32_repeated: ::puroro::internal::NoAllocBumpVec::new_in(bump),
                u64_unlabeled: ::std::default::Default::default(),
                u64_optional: ::std::default::Default::default(),
                u64_repeated: ::puroro::internal::NoAllocBumpVec::new_in(bump),
                s32_unlabeled: ::std::default::Default::default(),
                s32_optional: ::std::default::Default::default(),
                s32_repeated: ::puroro::internal::NoAllocBumpVec::new_in(bump),
                s64_unlabeled: ::std::default::Default::default(),
                s64_optional: ::std::default::Default::default(),
                s64_repeated: ::puroro::internal::NoAllocBumpVec::new_in(bump),
                fixed32_unlabeled: ::std::default::Default::default(),
                fixed32_optional: ::std::default::Default::default(),
                fixed32_repeated: ::puroro::internal::NoAllocBumpVec::new_in(bump),
                fixed64_unlabeled: ::std::default::Default::default(),
                fixed64_optional: ::std::default::Default::default(),
                fixed64_repeated: ::puroro::internal::NoAllocBumpVec::new_in(bump),
                sfixed32_unlabeled: ::std::default::Default::default(),
                sfixed32_optional: ::std::default::Default::default(),
                sfixed32_repeated: ::puroro::internal::NoAllocBumpVec::new_in(bump),
                sfixed64_unlabeled: ::std::default::Default::default(),
                sfixed64_optional: ::std::default::Default::default(),
                sfixed64_repeated: ::puroro::internal::NoAllocBumpVec::new_in(bump),
                f64_unlabeled: ::std::default::Default::default(),
                f64_optional: ::std::default::Default::default(),
                f64_repeated: ::puroro::internal::NoAllocBumpVec::new_in(bump),
            }
        }
        pub fn i32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            if !::puroro::internal::IsDefault::is_default(&self.i32_unlabeled) {
                ::std::option::Option::Some(self.i32_unlabeled)
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
                ::std::option::Option::Some(self.i32_optional)
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
        pub fn i32_repeated<'this>(&'this self) -> ::puroro::ClonedSlice<'this, i32> {
            ::puroro::ClonedSlice::new(&self.i32_repeated)
        }
        pub fn float_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<f32> {
            if !::puroro::internal::IsDefault::is_default(&self.float_unlabeled) {
                ::std::option::Option::Some(self.float_unlabeled)
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
                ::std::option::Option::Some(self.float_optional)
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
        pub fn float_repeated<'this>(&'this self) -> ::puroro::ClonedSlice<'this, f32> {
            ::puroro::ClonedSlice::new(&self.float_repeated)
        }
        pub fn bytes_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<&'this [u8]> {
            if !::puroro::internal::IsDefault::is_default(&self.bytes_unlabeled) {
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
        ) -> ::puroro::AsRefSlice<'this, ::puroro::internal::NoAllocBumpVec<u8>, [u8]> {
            ::puroro::AsRefSlice::new(&self.bytes_repeated)
        }
        pub fn string_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            if !::puroro::internal::IsDefault::is_default(&self.string_unlabeled) {
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
        ) -> ::puroro::AsRefSlice<'this, ::puroro::internal::NoAllocBumpString, str> {
            ::puroro::AsRefSlice::new(&self.string_repeated)
        }
        pub fn enum_unlabeled_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::full_coverage3::Enum> {
            if !::puroro::internal::IsDefault::is_default(&self.enum_unlabeled) {
                ::std::option::Option::Some(self.enum_unlabeled)
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
                ::std::option::Option::Some(self.enum_optional)
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
        ) -> ::puroro::ClonedSlice<'this, self::_puroro_root::full_coverage3::Enum> {
            ::puroro::ClonedSlice::new(&self.enum_repeated)
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
            if !::puroro::internal::IsDefault::is_default(&self.i64_unlabeled) {
                ::std::option::Option::Some(self.i64_unlabeled)
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
                ::std::option::Option::Some(self.i64_optional)
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
        pub fn i64_repeated<'this>(&'this self) -> ::puroro::ClonedSlice<'this, i64> {
            ::puroro::ClonedSlice::new(&self.i64_repeated)
        }
        pub fn u32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<u32> {
            if !::puroro::internal::IsDefault::is_default(&self.u32_unlabeled) {
                ::std::option::Option::Some(self.u32_unlabeled)
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
                ::std::option::Option::Some(self.u32_optional)
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
        pub fn u32_repeated<'this>(&'this self) -> ::puroro::ClonedSlice<'this, u32> {
            ::puroro::ClonedSlice::new(&self.u32_repeated)
        }
        pub fn u64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<u64> {
            if !::puroro::internal::IsDefault::is_default(&self.u64_unlabeled) {
                ::std::option::Option::Some(self.u64_unlabeled)
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
                ::std::option::Option::Some(self.u64_optional)
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
        pub fn u64_repeated<'this>(&'this self) -> ::puroro::ClonedSlice<'this, u64> {
            ::puroro::ClonedSlice::new(&self.u64_repeated)
        }
        pub fn s32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            if !::puroro::internal::IsDefault::is_default(&self.s32_unlabeled) {
                ::std::option::Option::Some(self.s32_unlabeled)
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
                ::std::option::Option::Some(self.s32_optional)
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
        pub fn s32_repeated<'this>(&'this self) -> ::puroro::ClonedSlice<'this, i32> {
            ::puroro::ClonedSlice::new(&self.s32_repeated)
        }
        pub fn s64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            if !::puroro::internal::IsDefault::is_default(&self.s64_unlabeled) {
                ::std::option::Option::Some(self.s64_unlabeled)
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
                ::std::option::Option::Some(self.s64_optional)
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
        pub fn s64_repeated<'this>(&'this self) -> ::puroro::ClonedSlice<'this, i64> {
            ::puroro::ClonedSlice::new(&self.s64_repeated)
        }
        pub fn fixed32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<u32> {
            if !::puroro::internal::IsDefault::is_default(&self.fixed32_unlabeled) {
                ::std::option::Option::Some(self.fixed32_unlabeled)
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
                ::std::option::Option::Some(self.fixed32_optional)
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
        pub fn fixed32_repeated<'this>(&'this self) -> ::puroro::ClonedSlice<'this, u32> {
            ::puroro::ClonedSlice::new(&self.fixed32_repeated)
        }
        pub fn fixed64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<u64> {
            if !::puroro::internal::IsDefault::is_default(&self.fixed64_unlabeled) {
                ::std::option::Option::Some(self.fixed64_unlabeled)
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
                ::std::option::Option::Some(self.fixed64_optional)
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
        pub fn fixed64_repeated<'this>(&'this self) -> ::puroro::ClonedSlice<'this, u64> {
            ::puroro::ClonedSlice::new(&self.fixed64_repeated)
        }
        pub fn sfixed32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            if !::puroro::internal::IsDefault::is_default(&self.sfixed32_unlabeled) {
                ::std::option::Option::Some(self.sfixed32_unlabeled)
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
                ::std::option::Option::Some(self.sfixed32_optional)
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
        pub fn sfixed32_repeated<'this>(&'this self) -> ::puroro::ClonedSlice<'this, i32> {
            ::puroro::ClonedSlice::new(&self.sfixed32_repeated)
        }
        pub fn sfixed64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            if !::puroro::internal::IsDefault::is_default(&self.sfixed64_unlabeled) {
                ::std::option::Option::Some(self.sfixed64_unlabeled)
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
                ::std::option::Option::Some(self.sfixed64_optional)
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
        pub fn sfixed64_repeated<'this>(&'this self) -> ::puroro::ClonedSlice<'this, i64> {
            ::puroro::ClonedSlice::new(&self.sfixed64_repeated)
        }
        pub fn f64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<f64> {
            if !::puroro::internal::IsDefault::is_default(&self.f64_unlabeled) {
                ::std::option::Option::Some(self.f64_unlabeled)
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
                ::std::option::Option::Some(self.f64_optional)
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
        pub fn f64_repeated<'this>(&'this self) -> ::puroro::ClonedSlice<'this, f64> {
            ::puroro::ClonedSlice::new(&self.f64_repeated)
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
        pub fn clear_bytes_unlabeled(&mut self) {
            self.bytes_unlabeled = ::std::default::Default::default();
        }
        pub fn bytes_unlabeled_mut<'this>(
            &'this mut self,
        ) -> ::puroro::internal::RefMutBumpVec<'bump, 'this, u8> {
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
        ) -> ::puroro::internal::RefMutBumpVec<'bump, 'this, u8> {
            if !self.has_bytes_optional() {
                self.bytes_optional = ::std::default::Default::default();
                self._bitfield.set(2, true);
            }
            unsafe { self.bytes_optional.as_mut_vec_in(self._bump) }
        }
        pub fn clear_string_unlabeled(&mut self) {
            self.string_unlabeled = ::std::default::Default::default();
        }
        pub fn string_unlabeled_mut<'this>(
            &'this mut self,
        ) -> ::puroro::internal::RefMutBumpString<'bump, 'this> {
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
        ) -> ::puroro::internal::RefMutBumpString<'bump, 'this> {
            if !self.has_string_optional() {
                self.string_optional = ::std::default::Default::default();
                self._bitfield.set(3, true);
            }
            unsafe { self.string_optional.as_mut_string_in(self._bump) }
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
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.i32_unlabeled))
        }
        fn i32_optional_opt<'this>(&'this self) -> Option<i32> {
            if ::puroro::internal::get_bitvec_bit(&self._bitfield, 0) {
                ::std::option::Option::Some(::std::clone::Clone::clone(&self.i32_optional))
            } else {
                ::std::option::Option::None
            }
        }

        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<
            'this,
            ::puroro::internal::NoAllocBumpVec<i32>,
            i32,
            i32,
        >;

        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.i32_repeated)
        }
        fn float_unlabeled_opt<'this>(&'this self) -> Option<f32> {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.float_unlabeled))
        }
        fn float_optional_opt<'this>(&'this self) -> Option<f32> {
            if ::puroro::internal::get_bitvec_bit(&self._bitfield, 1) {
                ::std::option::Option::Some(::std::clone::Clone::clone(&self.float_optional))
            } else {
                ::std::option::Option::None
            }
        }

        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<
            'this,
            ::puroro::internal::NoAllocBumpVec<f32>,
            f32,
            f32,
        >;

        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.float_repeated)
        }
        fn bytes_unlabeled_opt<'this>(&'this self) -> Option<&'this [u8]> {
            ::std::option::Option::Some(self.bytes_unlabeled.as_ref())
        }
        fn bytes_optional_opt<'this>(&'this self) -> Option<&'this [u8]> {
            if ::puroro::internal::get_bitvec_bit(&self._bitfield, 2) {
                ::std::option::Option::Some(self.bytes_optional.as_ref())
            } else {
                ::std::option::Option::None
            }
        }

        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::AsRefRepeatedField<
            'this,
            ::puroro::internal::NoAllocBumpVec<::puroro::internal::NoAllocBumpVec<u8>>,
            ::puroro::internal::NoAllocBumpVec<u8>,
            [u8],
        >;

        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::AsRefRepeatedField::new(&self.bytes_repeated)
        }
        fn string_unlabeled_opt<'this>(&'this self) -> Option<&'this str> {
            ::std::option::Option::Some(self.string_unlabeled.as_ref())
        }
        fn string_optional_opt<'this>(&'this self) -> Option<&'this str> {
            if ::puroro::internal::get_bitvec_bit(&self._bitfield, 3) {
                ::std::option::Option::Some(self.string_optional.as_ref())
            } else {
                ::std::option::Option::None
            }
        }

        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::AsRefRepeatedField<
            'this,
            ::puroro::internal::NoAllocBumpVec<::puroro::internal::NoAllocBumpString>,
            ::puroro::internal::NoAllocBumpString,
            str,
        >;

        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::AsRefRepeatedField::new(&self.string_repeated)
        }
        fn enum_unlabeled_opt<'this>(
            &'this self,
        ) -> Option<self::_puroro_root::full_coverage3::Enum> {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.enum_unlabeled))
        }
        fn enum_optional_opt<'this>(
            &'this self,
        ) -> Option<self::_puroro_root::full_coverage3::Enum> {
            if ::puroro::internal::get_bitvec_bit(&self._bitfield, 4) {
                ::std::option::Option::Some(::std::clone::Clone::clone(&self.enum_optional))
            } else {
                ::std::option::Option::None
            }
        }

        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<
            'this,
            ::puroro::internal::NoAllocBumpVec<self::_puroro_root::full_coverage3::Enum>,
            self::_puroro_root::full_coverage3::Enum,
            self::_puroro_root::full_coverage3::Enum,
        >;

        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.enum_repeated)
        }
        type Field51MessageType<'this> where Self: 'this = &'this self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgBumpalo<'this>;
        fn submsg_unlabeled_opt<'this>(&'this self) -> Option<Self::Field51MessageType<'this>> {
            self.submsg_unlabeled.as_ref().map(|b| b.as_ref())
        }
        type Field52MessageType<'this> where Self: 'this = &'this self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgBumpalo<'this>;
        fn submsg_optional_opt<'this>(&'this self) -> Option<Self::Field52MessageType<'this>> {
            self.submsg_optional.as_ref().map(|b| b.as_ref())
        }
        type Field53MessageType<'this> where Self: 'this = &'this self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgBumpalo<'this>;

        type Field53RepeatedType<'this> where Self: 'this =
    &'this [self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgBumpalo<'this>];

        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            unsafe { self.submsg_repeated.cast_item_unchecked() }
        }
        fn i64_unlabeled_opt<'this>(&'this self) -> Option<i64> {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.i64_unlabeled))
        }
        fn i64_optional_opt<'this>(&'this self) -> Option<i64> {
            if ::puroro::internal::get_bitvec_bit(&self._bitfield, 5) {
                ::std::option::Option::Some(::std::clone::Clone::clone(&self.i64_optional))
            } else {
                ::std::option::Option::None
            }
        }

        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<
            'this,
            ::puroro::internal::NoAllocBumpVec<i64>,
            i64,
            i64,
        >;

        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.i64_repeated)
        }
        fn u32_unlabeled_opt<'this>(&'this self) -> Option<u32> {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.u32_unlabeled))
        }
        fn u32_optional_opt<'this>(&'this self) -> Option<u32> {
            if ::puroro::internal::get_bitvec_bit(&self._bitfield, 6) {
                ::std::option::Option::Some(::std::clone::Clone::clone(&self.u32_optional))
            } else {
                ::std::option::Option::None
            }
        }

        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<
            'this,
            ::puroro::internal::NoAllocBumpVec<u32>,
            u32,
            u32,
        >;

        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.u32_repeated)
        }
        fn u64_unlabeled_opt<'this>(&'this self) -> Option<u64> {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.u64_unlabeled))
        }
        fn u64_optional_opt<'this>(&'this self) -> Option<u64> {
            if ::puroro::internal::get_bitvec_bit(&self._bitfield, 7) {
                ::std::option::Option::Some(::std::clone::Clone::clone(&self.u64_optional))
            } else {
                ::std::option::Option::None
            }
        }

        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<
            'this,
            ::puroro::internal::NoAllocBumpVec<u64>,
            u64,
            u64,
        >;

        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.u64_repeated)
        }
        fn s32_unlabeled_opt<'this>(&'this self) -> Option<i32> {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.s32_unlabeled))
        }
        fn s32_optional_opt<'this>(&'this self) -> Option<i32> {
            if ::puroro::internal::get_bitvec_bit(&self._bitfield, 8) {
                ::std::option::Option::Some(::std::clone::Clone::clone(&self.s32_optional))
            } else {
                ::std::option::Option::None
            }
        }

        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<
            'this,
            ::puroro::internal::NoAllocBumpVec<i32>,
            i32,
            i32,
        >;

        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.s32_repeated)
        }
        fn s64_unlabeled_opt<'this>(&'this self) -> Option<i64> {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.s64_unlabeled))
        }
        fn s64_optional_opt<'this>(&'this self) -> Option<i64> {
            if ::puroro::internal::get_bitvec_bit(&self._bitfield, 9) {
                ::std::option::Option::Some(::std::clone::Clone::clone(&self.s64_optional))
            } else {
                ::std::option::Option::None
            }
        }

        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<
            'this,
            ::puroro::internal::NoAllocBumpVec<i64>,
            i64,
            i64,
        >;

        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.s64_repeated)
        }
        fn fixed32_unlabeled_opt<'this>(&'this self) -> Option<u32> {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.fixed32_unlabeled))
        }
        fn fixed32_optional_opt<'this>(&'this self) -> Option<u32> {
            if ::puroro::internal::get_bitvec_bit(&self._bitfield, 10) {
                ::std::option::Option::Some(::std::clone::Clone::clone(&self.fixed32_optional))
            } else {
                ::std::option::Option::None
            }
        }

        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<
            'this,
            ::puroro::internal::NoAllocBumpVec<u32>,
            u32,
            u32,
        >;

        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.fixed32_repeated)
        }
        fn fixed64_unlabeled_opt<'this>(&'this self) -> Option<u64> {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.fixed64_unlabeled))
        }
        fn fixed64_optional_opt<'this>(&'this self) -> Option<u64> {
            if ::puroro::internal::get_bitvec_bit(&self._bitfield, 11) {
                ::std::option::Option::Some(::std::clone::Clone::clone(&self.fixed64_optional))
            } else {
                ::std::option::Option::None
            }
        }

        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<
            'this,
            ::puroro::internal::NoAllocBumpVec<u64>,
            u64,
            u64,
        >;

        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.fixed64_repeated)
        }
        fn sfixed32_unlabeled_opt<'this>(&'this self) -> Option<i32> {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.sfixed32_unlabeled))
        }
        fn sfixed32_optional_opt<'this>(&'this self) -> Option<i32> {
            if ::puroro::internal::get_bitvec_bit(&self._bitfield, 12) {
                ::std::option::Option::Some(::std::clone::Clone::clone(&self.sfixed32_optional))
            } else {
                ::std::option::Option::None
            }
        }

        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<
            'this,
            ::puroro::internal::NoAllocBumpVec<i32>,
            i32,
            i32,
        >;

        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.sfixed32_repeated)
        }
        fn sfixed64_unlabeled_opt<'this>(&'this self) -> Option<i64> {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.sfixed64_unlabeled))
        }
        fn sfixed64_optional_opt<'this>(&'this self) -> Option<i64> {
            if ::puroro::internal::get_bitvec_bit(&self._bitfield, 13) {
                ::std::option::Option::Some(::std::clone::Clone::clone(&self.sfixed64_optional))
            } else {
                ::std::option::Option::None
            }
        }

        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<
            'this,
            ::puroro::internal::NoAllocBumpVec<i64>,
            i64,
            i64,
        >;

        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.sfixed64_repeated)
        }
        fn f64_unlabeled_opt<'this>(&'this self) -> Option<f64> {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.f64_unlabeled))
        }
        fn f64_optional_opt<'this>(&'this self) -> Option<f64> {
            if ::puroro::internal::get_bitvec_bit(&self._bitfield, 14) {
                ::std::option::Option::Some(::std::clone::Clone::clone(&self.f64_optional))
            } else {
                ::std::option::Option::None
            }
        }

        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<
            'this,
            ::puroro::internal::NoAllocBumpVec<f64>,
            f64,
            f64,
        >;

        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.f64_repeated)
        }
    }

    impl<'bump> ::puroro::internal::de::DeserMessageFromBytesIter for MsgBumpalo<'bump> {
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
            use ::puroro::internal::impls::bumpalo::de::DeserFieldFromBytesIter;
            match field_number {
                1 => {
                    todo!()
                }
                2 => {
                    self._bitfield.set(0, true);
                    todo!()
                }
                3 => {
                    todo!()
                }
                11 => {
                    todo!()
                }
                12 => {
                    self._bitfield.set(1, true);
                    todo!()
                }
                13 => {
                    todo!()
                }
                21 => {
                    todo!()
                }
                22 => {
                    self._bitfield.set(2, true);
                    todo!()
                }
                23 => {
                    todo!()
                }
                31 => {
                    todo!()
                }
                32 => {
                    self._bitfield.set(3, true);
                    todo!()
                }
                33 => {
                    todo!()
                }
                41 => {
                    todo!()
                }
                42 => {
                    self._bitfield.set(4, true);
                    todo!()
                }
                43 => {
                    todo!()
                }
                51 => {
                    todo!()
                }
                52 => {
                    todo!()
                }
                53 => {
                    todo!()
                }
                101 => {
                    todo!()
                }
                102 => {
                    self._bitfield.set(5, true);
                    todo!()
                }
                103 => {
                    todo!()
                }
                111 => {
                    todo!()
                }
                112 => {
                    self._bitfield.set(6, true);
                    todo!()
                }
                113 => {
                    todo!()
                }
                121 => {
                    todo!()
                }
                122 => {
                    self._bitfield.set(7, true);
                    todo!()
                }
                123 => {
                    todo!()
                }
                131 => {
                    todo!()
                }
                132 => {
                    self._bitfield.set(8, true);
                    todo!()
                }
                133 => {
                    todo!()
                }
                141 => {
                    todo!()
                }
                142 => {
                    self._bitfield.set(9, true);
                    todo!()
                }
                143 => {
                    todo!()
                }
                151 => {
                    todo!()
                }
                152 => {
                    self._bitfield.set(10, true);
                    todo!()
                }
                153 => {
                    todo!()
                }
                161 => {
                    todo!()
                }
                162 => {
                    self._bitfield.set(11, true);
                    todo!()
                }
                163 => {
                    todo!()
                }
                171 => {
                    todo!()
                }
                172 => {
                    self._bitfield.set(12, true);
                    todo!()
                }
                173 => {
                    todo!()
                }
                181 => {
                    todo!()
                }
                182 => {
                    self._bitfield.set(13, true);
                    todo!()
                }
                183 => {
                    todo!()
                }
                191 => {
                    todo!()
                }
                192 => {
                    self._bitfield.set(14, true);
                    todo!()
                }
                193 => {
                    todo!()
                }

                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }

    impl<'bump> ::puroro::internal::se::SerMessageToIoWrite for MsgBumpalo<'bump>
    where
        Self: super::_puroro_traits::MsgTrait,
        for<'a> <Self as super::_puroro_traits::MsgTrait>::Field51MessageType<'a>:
            ::puroro::internal::se::SerMessageToIoWrite,
        for<'a> <Self as super::_puroro_traits::MsgTrait>::Field52MessageType<'a>:
            ::puroro::internal::se::SerMessageToIoWrite,
        for<'a> <Self as super::_puroro_traits::MsgTrait>::Field53MessageType<'a>:
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
                    <Self as super::_puroro_traits::MsgTrait>::Field51MessageType<'_>,
                >,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::submsg_unlabeled_opt(self),
                51,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    <Self as super::_puroro_traits::MsgTrait>::Field52MessageType<'_>,
                >,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::submsg_optional_opt(self),
                52,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    <Self as super::_puroro_traits::MsgTrait>::Field53MessageType<'_>,
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
            ScalarType: ::std::convert::Into<i32>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
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
            ScalarType: ::std::convert::Into<i32>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
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
            ScalarType: ::std::convert::Into<i32>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
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
            ScalarType: ::std::convert::Into<f32>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
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
            ScalarType: ::std::convert::Into<f32>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
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
            ScalarType: ::std::convert::Into<f32>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
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
            ScalarType: ::std::convert::AsRef<[u8]>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
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
            ScalarType: ::std::convert::AsRef<[u8]>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
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
            ScalarType: ::std::convert::AsRef<[u8]>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
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
            ScalarType: ::std::convert::AsRef<str>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
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
            ScalarType: ::std::convert::AsRef<str>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
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
            ScalarType: ::std::convert::AsRef<str>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
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
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
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
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
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
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
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
ScalarType: self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
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
ScalarType: self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
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
ScalarType: self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
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
            ScalarType: ::std::convert::Into<i64>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
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
            ScalarType: ::std::convert::Into<i64>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
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
            ScalarType: ::std::convert::Into<i64>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
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
            ScalarType: ::std::convert::Into<u32>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
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
            ScalarType: ::std::convert::Into<u32>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
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
            ScalarType: ::std::convert::Into<u32>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
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
            ScalarType: ::std::convert::Into<u64>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
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
            ScalarType: ::std::convert::Into<u64>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
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
            ScalarType: ::std::convert::Into<u64>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
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
            ScalarType: ::std::convert::Into<i32>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
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
            ScalarType: ::std::convert::Into<i32>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
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
            ScalarType: ::std::convert::Into<i32>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
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
            ScalarType: ::std::convert::Into<i64>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
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
            ScalarType: ::std::convert::Into<i64>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
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
            ScalarType: ::std::convert::Into<i64>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
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
            ScalarType: ::std::convert::Into<u32>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
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
            ScalarType: ::std::convert::Into<u32>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
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
            ScalarType: ::std::convert::Into<u32>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
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
            ScalarType: ::std::convert::Into<u64>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
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
            ScalarType: ::std::convert::Into<u64>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
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
            ScalarType: ::std::convert::Into<u64>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
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
            ScalarType: ::std::convert::Into<i32>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
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
            ScalarType: ::std::convert::Into<i32>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
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
            ScalarType: ::std::convert::Into<i32>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
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
            ScalarType: ::std::convert::Into<i64>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
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
            ScalarType: ::std::convert::Into<i64>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
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
            ScalarType: ::std::convert::Into<i64>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
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
            ScalarType: ::std::convert::Into<f64>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
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
            ScalarType: ::std::convert::Into<f64>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
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
            ScalarType: ::std::convert::Into<f64>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
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

        type Field3RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = i32>
        where
            Self: 'this;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this>;
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

        type Field13RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = f32>
        where
            Self: 'this;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this>;
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

        type Field23RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = &'this [u8]>
        where
            Self: 'this;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this>;
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

        type Field33RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = &'this str>
        where
            Self: 'this;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this>;
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

        type Field43RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = self::_puroro_root::full_coverage3::Enum>
        where
            Self: 'this;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this>;
        type Field51MessageType<'this>: self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait
            where Self: 'this;
        fn submsg_unlabeled<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field51MessageType<'this>> {
            self.submsg_unlabeled_opt()
        }
        fn has_submsg_unlabeled<'this>(&'this self) -> bool {
            self.submsg_unlabeled_opt().is_some()
        }
        fn submsg_unlabeled_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field51MessageType<'this>> {
            ::std::option::Option::None
        }
        type Field52MessageType<'this>: self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait
            where Self: 'this;
        fn submsg_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field52MessageType<'this>> {
            self.submsg_optional_opt()
        }
        fn has_submsg_optional<'this>(&'this self) -> bool {
            self.submsg_optional_opt().is_some()
        }
        fn submsg_optional_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field52MessageType<'this>> {
            ::std::option::Option::None
        }
        type Field53MessageType<'this>: self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait
            where Self: 'this;

        type Field53RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field53MessageType<'this>>
        where
            Self: 'this;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this>;
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

        type Field103RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = i64>
        where
            Self: 'this;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this>;
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

        type Field113RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = u32>
        where
            Self: 'this;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this>;
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

        type Field123RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = u64>
        where
            Self: 'this;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this>;
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

        type Field133RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = i32>
        where
            Self: 'this;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this>;
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

        type Field143RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = i64>
        where
            Self: 'this;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this>;
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

        type Field153RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = u32>
        where
            Self: 'this;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this>;
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

        type Field163RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = u64>
        where
            Self: 'this;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this>;
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

        type Field173RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = i32>
        where
            Self: 'this;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this>;
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

        type Field183RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = i64>
        where
            Self: 'this;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this>;
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

        type Field193RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = f64>
        where
            Self: 'this;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this>;
    }

    macro_rules! msg_delegate {
        ($ty:ty) => {
            fn i32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).i32_unlabeled_opt()
            }
            fn i32_optional_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).i32_optional_opt()
            }

            type Field3RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field3RepeatedType<'this>;
            fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
                (**self).i32_repeated()
            }
            fn float_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<f32> {
                (**self).float_unlabeled_opt()
            }
            fn float_optional_opt<'this>(&'this self) -> ::std::option::Option<f32> {
                (**self).float_optional_opt()
            }

            type Field13RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field13RepeatedType<'this>;
            fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
                (**self).float_repeated()
            }
            fn bytes_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<&'this [u8]> {
                (**self).bytes_unlabeled_opt()
            }
            fn bytes_optional_opt<'this>(&'this self) -> ::std::option::Option<&'this [u8]> {
                (**self).bytes_optional_opt()
            }

            type Field23RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field23RepeatedType<'this>;
            fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
                (**self).bytes_repeated()
            }
            fn string_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                (**self).string_unlabeled_opt()
            }
            fn string_optional_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                (**self).string_optional_opt()
            }

            type Field33RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field33RepeatedType<'this>;
            fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
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

            type Field43RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field43RepeatedType<'this>;
            fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
                (**self).enum_repeated()
            }
            type Field51MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field51MessageType<'this>;
            fn submsg_unlabeled_opt<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field51MessageType<'this>> {
                (**self).submsg_unlabeled_opt()
            }
            type Field52MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field52MessageType<'this>;
            fn submsg_optional_opt<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field52MessageType<'this>> {
                (**self).submsg_optional_opt()
            }
            type Field53MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field53MessageType<'this>;

            type Field53RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field53RepeatedType<'this>;
            fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
                (**self).submsg_repeated()
            }
            fn i64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i64> {
                (**self).i64_unlabeled_opt()
            }
            fn i64_optional_opt<'this>(&'this self) -> ::std::option::Option<i64> {
                (**self).i64_optional_opt()
            }

            type Field103RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field103RepeatedType<'this>;
            fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
                (**self).i64_repeated()
            }
            fn u32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<u32> {
                (**self).u32_unlabeled_opt()
            }
            fn u32_optional_opt<'this>(&'this self) -> ::std::option::Option<u32> {
                (**self).u32_optional_opt()
            }

            type Field113RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field113RepeatedType<'this>;
            fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
                (**self).u32_repeated()
            }
            fn u64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<u64> {
                (**self).u64_unlabeled_opt()
            }
            fn u64_optional_opt<'this>(&'this self) -> ::std::option::Option<u64> {
                (**self).u64_optional_opt()
            }

            type Field123RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field123RepeatedType<'this>;
            fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
                (**self).u64_repeated()
            }
            fn s32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).s32_unlabeled_opt()
            }
            fn s32_optional_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).s32_optional_opt()
            }

            type Field133RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field133RepeatedType<'this>;
            fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
                (**self).s32_repeated()
            }
            fn s64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i64> {
                (**self).s64_unlabeled_opt()
            }
            fn s64_optional_opt<'this>(&'this self) -> ::std::option::Option<i64> {
                (**self).s64_optional_opt()
            }

            type Field143RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field143RepeatedType<'this>;
            fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
                (**self).s64_repeated()
            }
            fn fixed32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<u32> {
                (**self).fixed32_unlabeled_opt()
            }
            fn fixed32_optional_opt<'this>(&'this self) -> ::std::option::Option<u32> {
                (**self).fixed32_optional_opt()
            }

            type Field153RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field153RepeatedType<'this>;
            fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
                (**self).fixed32_repeated()
            }
            fn fixed64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<u64> {
                (**self).fixed64_unlabeled_opt()
            }
            fn fixed64_optional_opt<'this>(&'this self) -> ::std::option::Option<u64> {
                (**self).fixed64_optional_opt()
            }

            type Field163RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field163RepeatedType<'this>;
            fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
                (**self).fixed64_repeated()
            }
            fn sfixed32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).sfixed32_unlabeled_opt()
            }
            fn sfixed32_optional_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).sfixed32_optional_opt()
            }

            type Field173RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field173RepeatedType<'this>;
            fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
                (**self).sfixed32_repeated()
            }
            fn sfixed64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i64> {
                (**self).sfixed64_unlabeled_opt()
            }
            fn sfixed64_optional_opt<'this>(&'this self) -> ::std::option::Option<i64> {
                (**self).sfixed64_optional_opt()
            }

            type Field183RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field183RepeatedType<'this>;
            fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
                (**self).sfixed64_repeated()
            }
            fn f64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<f64> {
                (**self).f64_unlabeled_opt()
            }
            fn f64_optional_opt<'this>(&'this self) -> ::std::option::Option<f64> {
                (**self).f64_optional_opt()
            }

            type Field193RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field193RepeatedType<'this>;
            fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
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
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this [u8]>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
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
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::Field3RepeatedType<'this>,
            <U as MsgTrait>::Field3RepeatedType<'this>,
        >;

        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
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
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::Field13RepeatedType<'this>,
            <U as MsgTrait>::Field13RepeatedType<'this>,
        >;

        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
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
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::Field23RepeatedType<'this>,
            <U as MsgTrait>::Field23RepeatedType<'this>,
        >;

        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
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
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::Field33RepeatedType<'this>,
            <U as MsgTrait>::Field33RepeatedType<'this>,
        >;

        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
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
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::Field43RepeatedType<'this>,
            <U as MsgTrait>::Field43RepeatedType<'this>,
        >;

        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as MsgTrait>::enum_repeated(&self.0),
                <U as MsgTrait>::enum_repeated(&self.1),
            )
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = (
            ::std::option::Option<<T as MsgTrait>::Field51MessageType<'this>>,
            ::std::option::Option<<U as MsgTrait>::Field51MessageType<'this>>,
        );
        fn submsg_unlabeled_opt<'this>(&'this self) -> Option<Self::Field51MessageType<'this>> {
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
        type Field52MessageType<'this>
        where
            Self: 'this,
        = (
            ::std::option::Option<<T as MsgTrait>::Field52MessageType<'this>>,
            ::std::option::Option<<U as MsgTrait>::Field52MessageType<'this>>,
        );
        fn submsg_optional_opt<'this>(&'this self) -> Option<Self::Field52MessageType<'this>> {
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
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as MsgTrait>::Field53MessageType<'this>,
            <U as MsgTrait>::Field53MessageType<'this>,
        >;
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedMessageField<
            <T as MsgTrait>::Field53RepeatedType<'this>,
            <U as MsgTrait>::Field53RepeatedType<'this>,
        >;

        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
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
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::Field103RepeatedType<'this>,
            <U as MsgTrait>::Field103RepeatedType<'this>,
        >;

        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
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
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::Field113RepeatedType<'this>,
            <U as MsgTrait>::Field113RepeatedType<'this>,
        >;

        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
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
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::Field123RepeatedType<'this>,
            <U as MsgTrait>::Field123RepeatedType<'this>,
        >;

        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
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
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::Field133RepeatedType<'this>,
            <U as MsgTrait>::Field133RepeatedType<'this>,
        >;

        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
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
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::Field143RepeatedType<'this>,
            <U as MsgTrait>::Field143RepeatedType<'this>,
        >;

        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
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
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::Field153RepeatedType<'this>,
            <U as MsgTrait>::Field153RepeatedType<'this>,
        >;

        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
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
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::Field163RepeatedType<'this>,
            <U as MsgTrait>::Field163RepeatedType<'this>,
        >;

        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
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
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::Field173RepeatedType<'this>,
            <U as MsgTrait>::Field173RepeatedType<'this>,
        >;

        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
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
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::Field183RepeatedType<'this>,
            <U as MsgTrait>::Field183RepeatedType<'this>,
        >;

        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
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
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::Field193RepeatedType<'this>,
            <U as MsgTrait>::Field193RepeatedType<'this>,
        >;

        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
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
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as MsgTrait>::Field3RepeatedType<'this>,
            <U as MsgTrait>::Field3RepeatedType<'this>,
        >;

        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
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
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as MsgTrait>::Field13RepeatedType<'this>,
            <U as MsgTrait>::Field13RepeatedType<'this>,
        >;

        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
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
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as MsgTrait>::Field23RepeatedType<'this>,
            <U as MsgTrait>::Field23RepeatedType<'this>,
        >;

        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
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
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as MsgTrait>::Field33RepeatedType<'this>,
            <U as MsgTrait>::Field33RepeatedType<'this>,
        >;

        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
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
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as MsgTrait>::Field43RepeatedType<'this>,
            <U as MsgTrait>::Field43RepeatedType<'this>,
        >;

        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::enum_repeated(t))
                    .map_right(|u| <U as MsgTrait>::enum_repeated(u)),
            )
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as MsgTrait>::Field51MessageType<'this>,
            <U as MsgTrait>::Field51MessageType<'this>,
        >;
        fn submsg_unlabeled_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field51MessageType<'this>> {
            self.as_ref().either(
                |t| <T as MsgTrait>::submsg_unlabeled_opt(t).map(|t| ::puroro::Either::Left(t)),
                |u| <U as MsgTrait>::submsg_unlabeled_opt(u).map(|u| ::puroro::Either::Right(u)),
            )
        }
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as MsgTrait>::Field52MessageType<'this>,
            <U as MsgTrait>::Field52MessageType<'this>,
        >;
        fn submsg_optional_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field52MessageType<'this>> {
            self.as_ref().either(
                |t| <T as MsgTrait>::submsg_optional_opt(t).map(|t| ::puroro::Either::Left(t)),
                |u| <U as MsgTrait>::submsg_optional_opt(u).map(|u| ::puroro::Either::Right(u)),
            )
        }
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as MsgTrait>::Field53MessageType<'this>,
            <U as MsgTrait>::Field53MessageType<'this>,
        >;
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedMessageField<
            <T as MsgTrait>::Field53RepeatedType<'this>,
            <U as MsgTrait>::Field53RepeatedType<'this>,
        >;

        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
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
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as MsgTrait>::Field103RepeatedType<'this>,
            <U as MsgTrait>::Field103RepeatedType<'this>,
        >;

        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
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
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as MsgTrait>::Field113RepeatedType<'this>,
            <U as MsgTrait>::Field113RepeatedType<'this>,
        >;

        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
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
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as MsgTrait>::Field123RepeatedType<'this>,
            <U as MsgTrait>::Field123RepeatedType<'this>,
        >;

        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
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
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as MsgTrait>::Field133RepeatedType<'this>,
            <U as MsgTrait>::Field133RepeatedType<'this>,
        >;

        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
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
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as MsgTrait>::Field143RepeatedType<'this>,
            <U as MsgTrait>::Field143RepeatedType<'this>,
        >;

        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
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
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as MsgTrait>::Field153RepeatedType<'this>,
            <U as MsgTrait>::Field153RepeatedType<'this>,
        >;

        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
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
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as MsgTrait>::Field163RepeatedType<'this>,
            <U as MsgTrait>::Field163RepeatedType<'this>,
        >;

        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
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
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as MsgTrait>::Field173RepeatedType<'this>,
            <U as MsgTrait>::Field173RepeatedType<'this>,
        >;

        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
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
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as MsgTrait>::Field183RepeatedType<'this>,
            <U as MsgTrait>::Field183RepeatedType<'this>,
        >;

        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
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
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as MsgTrait>::Field193RepeatedType<'this>,
            <U as MsgTrait>::Field193RepeatedType<'this>,
        >;

        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
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

        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::option::OptionRepeatedField<T::Field3RepeatedType<'this>>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
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

        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::option::OptionRepeatedField<T::Field13RepeatedType<'this>>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
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

        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::option::OptionRepeatedField<T::Field23RepeatedType<'this>>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
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

        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::option::OptionRepeatedField<T::Field33RepeatedType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
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

        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::option::OptionRepeatedField<T::Field43RepeatedType<'this>>;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::option::OptionRepeatedField::new(
                self.as_ref().map(|msg| msg.enum_repeated()),
            )
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = T::Field51MessageType<'this>;
        fn submsg_unlabeled_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field51MessageType<'this>> {
            self.as_ref().and_then(|msg| msg.submsg_unlabeled_opt())
        }
        type Field52MessageType<'this>
        where
            Self: 'this,
        = T::Field52MessageType<'this>;
        fn submsg_optional_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field52MessageType<'this>> {
            self.as_ref().and_then(|msg| msg.submsg_optional_opt())
        }
        type Field53MessageType<'this>
        where
            Self: 'this,
        = T::Field53MessageType<'this>;

        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::option::OptionRepeatedField<T::Field53RepeatedType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
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

        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::option::OptionRepeatedField<T::Field103RepeatedType<'this>>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
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

        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::option::OptionRepeatedField<T::Field113RepeatedType<'this>>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
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

        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::option::OptionRepeatedField<T::Field123RepeatedType<'this>>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
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

        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::option::OptionRepeatedField<T::Field133RepeatedType<'this>>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
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

        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::option::OptionRepeatedField<T::Field143RepeatedType<'this>>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
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

        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::option::OptionRepeatedField<T::Field153RepeatedType<'this>>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
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

        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::option::OptionRepeatedField<T::Field163RepeatedType<'this>>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
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

        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::option::OptionRepeatedField<T::Field173RepeatedType<'this>>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
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

        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::option::OptionRepeatedField<T::Field183RepeatedType<'this>>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
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

        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::option::OptionRepeatedField<T::Field193RepeatedType<'this>>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
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
                i32_unlabeled: i32,
                i64_unlabeled: i64,
            }
            impl ::puroro::Message<Submsg> for Submsg {}

            impl Submsg {
                pub fn new() -> Self {
                    Self {
                        i32_unlabeled: ::std::default::Default::default(),
                        i64_unlabeled: ::std::default::Default::default(),
                    }
                }
                pub fn i32_unlabeled_mut(&mut self) -> &mut i32 {
                    &mut self.i32_unlabeled
                }
                pub fn i64_unlabeled_mut(&mut self) -> &mut i64 {
                    &mut self.i64_unlabeled
                }
            }

            impl super::_puroro_traits::SubmsgTrait for Submsg {
                fn i32_unlabeled_opt<'this>(&'this self) -> Option<i32> {
                    if self.i32_unlabeled == ::std::default::Default::default() {
                        ::std::option::Option::None
                    } else {
                        ::std::option::Option::Some(self.i32_unlabeled.clone())
                    }
                }
                fn i64_unlabeled_opt<'this>(&'this self) -> Option<i64> {
                    if self.i64_unlabeled == ::std::default::Default::default() {
                        ::std::option::Option::None
                    } else {
                        ::std::option::Option::Some(self.i64_unlabeled.clone())
                    }
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
                        .field(
                            "i32_unlabeled",
                            &<Self as super::_puroro_traits::SubmsgTrait>::i32_unlabeled(self),
                        )
                        .field(
                            "i64_unlabeled",
                            &<Self as super::_puroro_traits::SubmsgTrait>::i64_unlabeled(self),
                        )
                        .finish()
                }
            }

            impl ::std::clone::Clone for Submsg {
                fn clone(&self) -> Self {
                    Self {
                        i32_unlabeled: ::std::clone::Clone::clone(&self.i32_unlabeled),
                        i64_unlabeled: ::std::clone::Clone::clone(&self.i64_unlabeled),
                    }
                }
            }

            impl ::std::cmp::PartialEq for Submsg {
                fn eq(&self, rhs: &Self) -> bool {
                    self.i32_unlabeled == rhs.i32_unlabeled
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

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

            pub struct SubmsgSingleField1<ScalarType>
            where
                ScalarType: ::std::convert::Into<i32>
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
            {
                pub i32_unlabeled: ScalarType,
            }

            impl<ScalarType> ::puroro::Message<super::Submsg> for SubmsgSingleField1<ScalarType> where
                ScalarType: ::std::convert::Into<i32>
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug
            {
            }

            impl<ScalarType> super::_puroro_traits::SubmsgTrait for SubmsgSingleField1<ScalarType>
            where
                ScalarType: ::std::convert::Into<i32>
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
            {
                fn i32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                    ::std::option::Option::Some(::std::convert::Into::into(
                        ::std::clone::Clone::clone(&self.i32_unlabeled),
                    ))
                }
            }

            impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for SubmsgSingleField1<ScalarType>
            where
                ScalarType: ::std::convert::Into<i32>
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
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
                ScalarType: ::std::convert::Into<i32>
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
            {
                fn from(value: ScalarType) -> Self {
                    Self {
                        i32_unlabeled: value,
                    }
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

            pub struct SubmsgSingleField101<ScalarType>
            where
                ScalarType: ::std::convert::Into<i64>
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
            {
                pub i64_unlabeled: ScalarType,
            }

            impl<ScalarType> ::puroro::Message<super::Submsg> for SubmsgSingleField101<ScalarType> where
                ScalarType: ::std::convert::Into<i64>
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug
            {
            }

            impl<ScalarType> super::_puroro_traits::SubmsgTrait for SubmsgSingleField101<ScalarType>
            where
                ScalarType: ::std::convert::Into<i64>
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
            {
                fn i64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i64> {
                    ::std::option::Option::Some(::std::convert::Into::into(
                        ::std::clone::Clone::clone(&self.i64_unlabeled),
                    ))
                }
            }

            impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for SubmsgSingleField101<ScalarType>
            where
                ScalarType: ::std::convert::Into<i64>
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
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
                ScalarType: ::std::convert::Into<i64>
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
            {
                fn from(value: ScalarType) -> Self {
                    Self {
                        i64_unlabeled: value,
                    }
                }
            }
            pub struct SubmsgBumpalo<'bump> {
                _bump: &'bump ::puroro::bumpalo::Bump,
                _bitfield: ::puroro::bitvec::array::BitArray<
                    ::puroro::bitvec::order::Lsb0,
                    [u32; (0 + 31) / 32],
                >,
                i32_unlabeled: i32,
                i64_unlabeled: i64,
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
                        i64_unlabeled: ::std::default::Default::default(),
                    }
                }
                pub fn i32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                    if !::puroro::internal::IsDefault::is_default(&self.i32_unlabeled) {
                        ::std::option::Option::Some(self.i32_unlabeled)
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
                pub fn i64_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i64> {
                    if !::puroro::internal::IsDefault::is_default(&self.i64_unlabeled) {
                        ::std::option::Option::Some(self.i64_unlabeled)
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
                    ::std::option::Option::Some(::std::clone::Clone::clone(&self.i32_unlabeled))
                }
                fn i64_unlabeled_opt<'this>(&'this self) -> Option<i64> {
                    ::std::option::Option::Some(::std::clone::Clone::clone(&self.i64_unlabeled))
                }
            }

            impl<'bump> ::puroro::internal::de::DeserMessageFromBytesIter for SubmsgBumpalo<'bump> {
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
                    use ::puroro::internal::impls::bumpalo::de::DeserFieldFromBytesIter;
                    match field_number {
                        1 => {
                            todo!()
                        }
                        101 => {
                            todo!()
                        }

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
                    ScalarType: ::std::convert::Into<i32>
                        + ::std::clone::Clone
                        + ::std::cmp::PartialEq
                        + ::std::fmt::Debug,
                {
                    SubmsgBuilder((
                        self.0,
                        SubmsgSingleField1 {
                            i32_unlabeled: value,
                        },
                    ))
                }

                pub fn append_i64_unlabeled<ScalarType>(
                    self,
                    value: ScalarType,
                ) -> SubmsgBuilder<(T, SubmsgSingleField101<ScalarType>)>
                where
                    ScalarType: ::std::convert::Into<i64>
                        + ::std::clone::Clone
                        + ::std::cmp::PartialEq
                        + ::std::fmt::Debug,
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
