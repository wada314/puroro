// A generated source code by puroro library
// package full_coverage2

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub use _puroro_simple_impl::Msg;
pub mod _puroro_simple_impl {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
    pub struct Msg {
    i32_required: ::std::option::Option<i32>,
    i32_optional: ::std::option::Option<i32>,
    i32_repeated: ::std::vec::Vec<i32>,
    float_required: ::std::option::Option<f32>,
    float_optional: ::std::option::Option<f32>,
    float_repeated: ::std::vec::Vec<f32>,
    bytes_required: ::std::option::Option<::std::vec::Vec<u8>>,
    bytes_optional: ::std::option::Option<::std::vec::Vec<u8>>,
    bytes_repeated: ::std::vec::Vec<::std::vec::Vec<u8>>,
    string_required: ::std::option::Option<::std::string::String>,
    string_optional: ::std::option::Option<::std::string::String>,
    string_repeated: ::std::vec::Vec<::std::string::String>,
    enum_required: ::std::option::Option<self::_puroro_root::full_coverage2::Enum>,
    enum_optional: ::std::option::Option<self::_puroro_root::full_coverage2::Enum>,
    enum_repeated: ::std::vec::Vec<self::_puroro_root::full_coverage2::Enum>,
    submsg_required: ::std::option::Option<::std::boxed::Box<self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_simple_impl::Submsg>>,
    submsg_optional: ::std::option::Option<::std::boxed::Box<self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_simple_impl::Submsg>>,
    submsg_repeated: ::std::vec::Vec<self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_simple_impl::Submsg>,
    i64_required: ::std::option::Option<i64>,
    i64_optional: ::std::option::Option<i64>,
    i64_repeated: ::std::vec::Vec<i64>,
    u32_required: ::std::option::Option<u32>,
    u32_optional: ::std::option::Option<u32>,
    u32_repeated: ::std::vec::Vec<u32>,
    u64_required: ::std::option::Option<u64>,
    u64_optional: ::std::option::Option<u64>,
    u64_repeated: ::std::vec::Vec<u64>,
    s32_required: ::std::option::Option<i32>,
    s32_optional: ::std::option::Option<i32>,
    s32_repeated: ::std::vec::Vec<i32>,
    s64_required: ::std::option::Option<i64>,
    s64_optional: ::std::option::Option<i64>,
    s64_repeated: ::std::vec::Vec<i64>,
    fixed32_required: ::std::option::Option<u32>,
    fixed32_optional: ::std::option::Option<u32>,
    fixed32_repeated: ::std::vec::Vec<u32>,
    fixed64_required: ::std::option::Option<u64>,
    fixed64_optional: ::std::option::Option<u64>,
    fixed64_repeated: ::std::vec::Vec<u64>,
    sfixed32_required: ::std::option::Option<i32>,
    sfixed32_optional: ::std::option::Option<i32>,
    sfixed32_repeated: ::std::vec::Vec<i32>,
    sfixed64_required: ::std::option::Option<i64>,
    sfixed64_optional: ::std::option::Option<i64>,
    sfixed64_repeated: ::std::vec::Vec<i64>,
    f64_required: ::std::option::Option<f64>,
    f64_optional: ::std::option::Option<f64>,
    f64_repeated: ::std::vec::Vec<f64>,
}
    impl ::puroro::Message<Msg> for Msg {}

    impl Msg {
        pub fn new() -> Self {
            Self {
                i32_required: ::std::default::Default::default(),
                i32_optional: ::std::default::Default::default(),
                i32_repeated: ::std::default::Default::default(),
                float_required: ::std::default::Default::default(),
                float_optional: ::std::default::Default::default(),
                float_repeated: ::std::default::Default::default(),
                bytes_required: ::std::default::Default::default(),
                bytes_optional: ::std::default::Default::default(),
                bytes_repeated: ::std::default::Default::default(),
                string_required: ::std::default::Default::default(),
                string_optional: ::std::default::Default::default(),
                string_repeated: ::std::default::Default::default(),
                enum_required: ::std::default::Default::default(),
                enum_optional: ::std::default::Default::default(),
                enum_repeated: ::std::default::Default::default(),
                submsg_required: ::std::default::Default::default(),
                submsg_optional: ::std::default::Default::default(),
                submsg_repeated: ::std::default::Default::default(),
                i64_required: ::std::default::Default::default(),
                i64_optional: ::std::default::Default::default(),
                i64_repeated: ::std::default::Default::default(),
                u32_required: ::std::default::Default::default(),
                u32_optional: ::std::default::Default::default(),
                u32_repeated: ::std::default::Default::default(),
                u64_required: ::std::default::Default::default(),
                u64_optional: ::std::default::Default::default(),
                u64_repeated: ::std::default::Default::default(),
                s32_required: ::std::default::Default::default(),
                s32_optional: ::std::default::Default::default(),
                s32_repeated: ::std::default::Default::default(),
                s64_required: ::std::default::Default::default(),
                s64_optional: ::std::default::Default::default(),
                s64_repeated: ::std::default::Default::default(),
                fixed32_required: ::std::default::Default::default(),
                fixed32_optional: ::std::default::Default::default(),
                fixed32_repeated: ::std::default::Default::default(),
                fixed64_required: ::std::default::Default::default(),
                fixed64_optional: ::std::default::Default::default(),
                fixed64_repeated: ::std::default::Default::default(),
                sfixed32_required: ::std::default::Default::default(),
                sfixed32_optional: ::std::default::Default::default(),
                sfixed32_repeated: ::std::default::Default::default(),
                sfixed64_required: ::std::default::Default::default(),
                sfixed64_optional: ::std::default::Default::default(),
                sfixed64_repeated: ::std::default::Default::default(),
                f64_required: ::std::default::Default::default(),
                f64_optional: ::std::default::Default::default(),
                f64_repeated: ::std::default::Default::default(),
            }
        }
        pub fn i32_required_mut(&mut self) -> &mut ::std::option::Option<i32> {
            &mut self.i32_required
        }
        pub fn i32_optional_mut(&mut self) -> &mut ::std::option::Option<i32> {
            &mut self.i32_optional
        }
        pub fn i32_repeated_mut(&mut self) -> &mut ::std::vec::Vec<i32> {
            &mut self.i32_repeated
        }
        pub fn float_required_mut(&mut self) -> &mut ::std::option::Option<f32> {
            &mut self.float_required
        }
        pub fn float_optional_mut(&mut self) -> &mut ::std::option::Option<f32> {
            &mut self.float_optional
        }
        pub fn float_repeated_mut(&mut self) -> &mut ::std::vec::Vec<f32> {
            &mut self.float_repeated
        }
        pub fn bytes_required_mut(&mut self) -> &mut ::std::option::Option<::std::vec::Vec<u8>> {
            &mut self.bytes_required
        }
        pub fn bytes_optional_mut(&mut self) -> &mut ::std::option::Option<::std::vec::Vec<u8>> {
            &mut self.bytes_optional
        }
        pub fn bytes_repeated_mut(&mut self) -> &mut ::std::vec::Vec<::std::vec::Vec<u8>> {
            &mut self.bytes_repeated
        }
        pub fn string_required_mut(&mut self) -> &mut ::std::option::Option<::std::string::String> {
            &mut self.string_required
        }
        pub fn string_optional_mut(&mut self) -> &mut ::std::option::Option<::std::string::String> {
            &mut self.string_optional
        }
        pub fn string_repeated_mut(&mut self) -> &mut ::std::vec::Vec<::std::string::String> {
            &mut self.string_repeated
        }
        pub fn enum_required_mut(
            &mut self,
        ) -> &mut ::std::option::Option<self::_puroro_root::full_coverage2::Enum> {
            &mut self.enum_required
        }
        pub fn enum_optional_mut(
            &mut self,
        ) -> &mut ::std::option::Option<self::_puroro_root::full_coverage2::Enum> {
            &mut self.enum_optional
        }
        pub fn enum_repeated_mut(
            &mut self,
        ) -> &mut ::std::vec::Vec<self::_puroro_root::full_coverage2::Enum> {
            &mut self.enum_repeated
        }
        pub fn submsg_required_mut(&mut self) -> &mut ::std::option::Option<::std::boxed::Box<self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_simple_impl::Submsg>>{
            &mut self.submsg_required
        }
        pub fn submsg_optional_mut(&mut self) -> &mut ::std::option::Option<::std::boxed::Box<self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_simple_impl::Submsg>>{
            &mut self.submsg_optional
        }
        pub fn submsg_repeated_mut(
            &mut self,
        ) -> &mut ::std::vec::Vec<
            self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_simple_impl::Submsg,
        > {
            &mut self.submsg_repeated
        }
        pub fn i64_required_mut(&mut self) -> &mut ::std::option::Option<i64> {
            &mut self.i64_required
        }
        pub fn i64_optional_mut(&mut self) -> &mut ::std::option::Option<i64> {
            &mut self.i64_optional
        }
        pub fn i64_repeated_mut(&mut self) -> &mut ::std::vec::Vec<i64> {
            &mut self.i64_repeated
        }
        pub fn u32_required_mut(&mut self) -> &mut ::std::option::Option<u32> {
            &mut self.u32_required
        }
        pub fn u32_optional_mut(&mut self) -> &mut ::std::option::Option<u32> {
            &mut self.u32_optional
        }
        pub fn u32_repeated_mut(&mut self) -> &mut ::std::vec::Vec<u32> {
            &mut self.u32_repeated
        }
        pub fn u64_required_mut(&mut self) -> &mut ::std::option::Option<u64> {
            &mut self.u64_required
        }
        pub fn u64_optional_mut(&mut self) -> &mut ::std::option::Option<u64> {
            &mut self.u64_optional
        }
        pub fn u64_repeated_mut(&mut self) -> &mut ::std::vec::Vec<u64> {
            &mut self.u64_repeated
        }
        pub fn s32_required_mut(&mut self) -> &mut ::std::option::Option<i32> {
            &mut self.s32_required
        }
        pub fn s32_optional_mut(&mut self) -> &mut ::std::option::Option<i32> {
            &mut self.s32_optional
        }
        pub fn s32_repeated_mut(&mut self) -> &mut ::std::vec::Vec<i32> {
            &mut self.s32_repeated
        }
        pub fn s64_required_mut(&mut self) -> &mut ::std::option::Option<i64> {
            &mut self.s64_required
        }
        pub fn s64_optional_mut(&mut self) -> &mut ::std::option::Option<i64> {
            &mut self.s64_optional
        }
        pub fn s64_repeated_mut(&mut self) -> &mut ::std::vec::Vec<i64> {
            &mut self.s64_repeated
        }
        pub fn fixed32_required_mut(&mut self) -> &mut ::std::option::Option<u32> {
            &mut self.fixed32_required
        }
        pub fn fixed32_optional_mut(&mut self) -> &mut ::std::option::Option<u32> {
            &mut self.fixed32_optional
        }
        pub fn fixed32_repeated_mut(&mut self) -> &mut ::std::vec::Vec<u32> {
            &mut self.fixed32_repeated
        }
        pub fn fixed64_required_mut(&mut self) -> &mut ::std::option::Option<u64> {
            &mut self.fixed64_required
        }
        pub fn fixed64_optional_mut(&mut self) -> &mut ::std::option::Option<u64> {
            &mut self.fixed64_optional
        }
        pub fn fixed64_repeated_mut(&mut self) -> &mut ::std::vec::Vec<u64> {
            &mut self.fixed64_repeated
        }
        pub fn sfixed32_required_mut(&mut self) -> &mut ::std::option::Option<i32> {
            &mut self.sfixed32_required
        }
        pub fn sfixed32_optional_mut(&mut self) -> &mut ::std::option::Option<i32> {
            &mut self.sfixed32_optional
        }
        pub fn sfixed32_repeated_mut(&mut self) -> &mut ::std::vec::Vec<i32> {
            &mut self.sfixed32_repeated
        }
        pub fn sfixed64_required_mut(&mut self) -> &mut ::std::option::Option<i64> {
            &mut self.sfixed64_required
        }
        pub fn sfixed64_optional_mut(&mut self) -> &mut ::std::option::Option<i64> {
            &mut self.sfixed64_optional
        }
        pub fn sfixed64_repeated_mut(&mut self) -> &mut ::std::vec::Vec<i64> {
            &mut self.sfixed64_repeated
        }
        pub fn f64_required_mut(&mut self) -> &mut ::std::option::Option<f64> {
            &mut self.f64_required
        }
        pub fn f64_optional_mut(&mut self) -> &mut ::std::option::Option<f64> {
            &mut self.f64_optional
        }
        pub fn f64_repeated_mut(&mut self) -> &mut ::std::vec::Vec<f64> {
            &mut self.f64_repeated
        }
    }

    impl super::_puroro_traits::MsgTrait for Msg {
        fn i32_required_opt<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.i32_required)
        }
        fn i32_optional_opt<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.i32_optional)
        }
        type Field3RepeatedType<'this> =
            ::puroro::CloneThenIntoRepeatedField<'this, ::std::vec::Vec<i32>, i32, i32>;

        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.i32_repeated)
        }
        fn float_required_opt<'this>(&'this self) -> Option<f32> {
            Clone::clone(&self.float_required)
        }
        fn float_optional_opt<'this>(&'this self) -> Option<f32> {
            Clone::clone(&self.float_optional)
        }
        type Field13RepeatedType<'this> =
            ::puroro::CloneThenIntoRepeatedField<'this, ::std::vec::Vec<f32>, f32, f32>;

        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.float_repeated)
        }
        type Field21ScalarGetterType<'this>
        where
            Self: 'this,
        = &'this ::std::vec::Vec<u8>;
        fn bytes_required_opt<'this>(&'this self) -> Option<Self::Field21ScalarGetterType<'this>> {
            self.bytes_required.as_ref().map(|v| v.as_ref())
        }
        type Field22ScalarGetterType<'this>
        where
            Self: 'this,
        = &'this ::std::vec::Vec<u8>;
        fn bytes_optional_opt<'this>(&'this self) -> Option<Self::Field22ScalarGetterType<'this>> {
            self.bytes_optional.as_ref().map(|v| v.as_ref())
        }
        type Field23ScalarGetterType<'this>
        where
            Self: 'this,
        = &'this ::std::vec::Vec<u8>;
        type Field23RepeatedType<'this> = &'this [::std::vec::Vec<u8>];

        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            &self.bytes_repeated
        }
        type Field31ScalarGetterType<'this>
        where
            Self: 'this,
        = &'this ::std::string::String;
        fn string_required_opt<'this>(&'this self) -> Option<Self::Field31ScalarGetterType<'this>> {
            self.string_required.as_ref().map(|v| v.as_ref())
        }
        type Field32ScalarGetterType<'this>
        where
            Self: 'this,
        = &'this ::std::string::String;
        fn string_optional_opt<'this>(&'this self) -> Option<Self::Field32ScalarGetterType<'this>> {
            self.string_optional.as_ref().map(|v| v.as_ref())
        }
        type Field33ScalarGetterType<'this>
        where
            Self: 'this,
        = &'this ::std::string::String;
        type Field33RepeatedType<'this> = &'this [::std::string::String];

        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            &self.string_repeated
        }
        fn enum_required_opt<'this>(
            &'this self,
        ) -> Option<self::_puroro_root::full_coverage2::Enum> {
            Clone::clone(&self.enum_required)
        }
        fn enum_optional_opt<'this>(
            &'this self,
        ) -> Option<self::_puroro_root::full_coverage2::Enum> {
            Clone::clone(&self.enum_optional)
        }
        type Field43RepeatedType<'this> = ::puroro::CloneThenIntoRepeatedField<
            'this,
            ::std::vec::Vec<self::_puroro_root::full_coverage2::Enum>,
            self::_puroro_root::full_coverage2::Enum,
            self::_puroro_root::full_coverage2::Enum,
        >;

        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.enum_repeated)
        }
        type Field51ScalarGetterType<'this> where Self: 'this = &'this self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_simple_impl::Submsg;
        fn submsg_required_opt<'this>(&'this self) -> Option<Self::Field51ScalarGetterType<'this>> {
            self.submsg_required.as_ref().map(|v| v.as_ref())
        }
        type Field52ScalarGetterType<'this> where Self: 'this = &'this self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_simple_impl::Submsg;
        fn submsg_optional_opt<'this>(&'this self) -> Option<Self::Field52ScalarGetterType<'this>> {
            self.submsg_optional.as_ref().map(|v| v.as_ref())
        }
        type Field53ScalarGetterType<'this> where Self: 'this = &'this self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_simple_impl::Submsg;
        type Field53RepeatedType<'this> = &'this [self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_simple_impl::Submsg];

        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            &self.submsg_repeated
        }
        fn i64_required_opt<'this>(&'this self) -> Option<i64> {
            Clone::clone(&self.i64_required)
        }
        fn i64_optional_opt<'this>(&'this self) -> Option<i64> {
            Clone::clone(&self.i64_optional)
        }
        type Field103RepeatedType<'this> =
            ::puroro::CloneThenIntoRepeatedField<'this, ::std::vec::Vec<i64>, i64, i64>;

        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.i64_repeated)
        }
        fn u32_required_opt<'this>(&'this self) -> Option<u32> {
            Clone::clone(&self.u32_required)
        }
        fn u32_optional_opt<'this>(&'this self) -> Option<u32> {
            Clone::clone(&self.u32_optional)
        }
        type Field113RepeatedType<'this> =
            ::puroro::CloneThenIntoRepeatedField<'this, ::std::vec::Vec<u32>, u32, u32>;

        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.u32_repeated)
        }
        fn u64_required_opt<'this>(&'this self) -> Option<u64> {
            Clone::clone(&self.u64_required)
        }
        fn u64_optional_opt<'this>(&'this self) -> Option<u64> {
            Clone::clone(&self.u64_optional)
        }
        type Field123RepeatedType<'this> =
            ::puroro::CloneThenIntoRepeatedField<'this, ::std::vec::Vec<u64>, u64, u64>;

        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.u64_repeated)
        }
        fn s32_required_opt<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.s32_required)
        }
        fn s32_optional_opt<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.s32_optional)
        }
        type Field133RepeatedType<'this> =
            ::puroro::CloneThenIntoRepeatedField<'this, ::std::vec::Vec<i32>, i32, i32>;

        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.s32_repeated)
        }
        fn s64_required_opt<'this>(&'this self) -> Option<i64> {
            Clone::clone(&self.s64_required)
        }
        fn s64_optional_opt<'this>(&'this self) -> Option<i64> {
            Clone::clone(&self.s64_optional)
        }
        type Field143RepeatedType<'this> =
            ::puroro::CloneThenIntoRepeatedField<'this, ::std::vec::Vec<i64>, i64, i64>;

        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.s64_repeated)
        }
        fn fixed32_required_opt<'this>(&'this self) -> Option<u32> {
            Clone::clone(&self.fixed32_required)
        }
        fn fixed32_optional_opt<'this>(&'this self) -> Option<u32> {
            Clone::clone(&self.fixed32_optional)
        }
        type Field153RepeatedType<'this> =
            ::puroro::CloneThenIntoRepeatedField<'this, ::std::vec::Vec<u32>, u32, u32>;

        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.fixed32_repeated)
        }
        fn fixed64_required_opt<'this>(&'this self) -> Option<u64> {
            Clone::clone(&self.fixed64_required)
        }
        fn fixed64_optional_opt<'this>(&'this self) -> Option<u64> {
            Clone::clone(&self.fixed64_optional)
        }
        type Field163RepeatedType<'this> =
            ::puroro::CloneThenIntoRepeatedField<'this, ::std::vec::Vec<u64>, u64, u64>;

        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.fixed64_repeated)
        }
        fn sfixed32_required_opt<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.sfixed32_required)
        }
        fn sfixed32_optional_opt<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.sfixed32_optional)
        }
        type Field173RepeatedType<'this> =
            ::puroro::CloneThenIntoRepeatedField<'this, ::std::vec::Vec<i32>, i32, i32>;

        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.sfixed32_repeated)
        }
        fn sfixed64_required_opt<'this>(&'this self) -> Option<i64> {
            Clone::clone(&self.sfixed64_required)
        }
        fn sfixed64_optional_opt<'this>(&'this self) -> Option<i64> {
            Clone::clone(&self.sfixed64_optional)
        }
        type Field183RepeatedType<'this> =
            ::puroro::CloneThenIntoRepeatedField<'this, ::std::vec::Vec<i64>, i64, i64>;

        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.sfixed64_repeated)
        }
        fn f64_required_opt<'this>(&'this self) -> Option<f64> {
            Clone::clone(&self.f64_required)
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
                ::puroro::tags::Required, ::puroro::tags::Int32
            >::deser_field(&mut self.i32_required, data),
            2 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Int32
            >::deser_field(&mut self.i32_optional, data),
            3 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Int32
            >::deser_field(&mut self.i32_repeated, data),
            11 => DeserFieldFromBytesIter::<
                ::puroro::tags::Required, ::puroro::tags::Float
            >::deser_field(&mut self.float_required, data),
            12 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Float
            >::deser_field(&mut self.float_optional, data),
            13 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Float
            >::deser_field(&mut self.float_repeated, data),
            21 => DeserFieldFromBytesIter::<
                ::puroro::tags::Required, ::puroro::tags::Bytes
            >::deser_field(&mut self.bytes_required, data),
            22 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bytes
            >::deser_field(&mut self.bytes_optional, data),
            23 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Bytes
            >::deser_field(&mut self.bytes_repeated, data),
            31 => DeserFieldFromBytesIter::<
                ::puroro::tags::Required, ::puroro::tags::String
            >::deser_field(&mut self.string_required, data),
            32 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.string_optional, data),
            33 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::String
            >::deser_field(&mut self.string_repeated, data),
            41 => DeserFieldFromBytesIter::<
                ::puroro::tags::Required, ::puroro::tags::Enum2<self::_puroro_root::full_coverage2::Enum>
            >::deser_field(&mut self.enum_required, data),
            42 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Enum2<self::_puroro_root::full_coverage2::Enum>
            >::deser_field(&mut self.enum_optional, data),
            43 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Enum2<self::_puroro_root::full_coverage2::Enum>
            >::deser_field(&mut self.enum_repeated, data),
            51 => DeserFieldFromBytesIter::<
                ::puroro::tags::Required, ::puroro::tags::Message<::std::boxed::Box<self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_simple_impl::Submsg>>
            >::deser_field(&mut self.submsg_required, data),
            52 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Message<::std::boxed::Box<self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_simple_impl::Submsg>>
            >::deser_field(&mut self.submsg_optional, data),
            53 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_simple_impl::Submsg>
            >::deser_field(&mut self.submsg_repeated, data),
            101 => DeserFieldFromBytesIter::<
                ::puroro::tags::Required, ::puroro::tags::Int64
            >::deser_field(&mut self.i64_required, data),
            102 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Int64
            >::deser_field(&mut self.i64_optional, data),
            103 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Int64
            >::deser_field(&mut self.i64_repeated, data),
            111 => DeserFieldFromBytesIter::<
                ::puroro::tags::Required, ::puroro::tags::UInt32
            >::deser_field(&mut self.u32_required, data),
            112 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::UInt32
            >::deser_field(&mut self.u32_optional, data),
            113 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::UInt32
            >::deser_field(&mut self.u32_repeated, data),
            121 => DeserFieldFromBytesIter::<
                ::puroro::tags::Required, ::puroro::tags::UInt64
            >::deser_field(&mut self.u64_required, data),
            122 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::UInt64
            >::deser_field(&mut self.u64_optional, data),
            123 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::UInt64
            >::deser_field(&mut self.u64_repeated, data),
            131 => DeserFieldFromBytesIter::<
                ::puroro::tags::Required, ::puroro::tags::SInt32
            >::deser_field(&mut self.s32_required, data),
            132 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::SInt32
            >::deser_field(&mut self.s32_optional, data),
            133 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::SInt32
            >::deser_field(&mut self.s32_repeated, data),
            141 => DeserFieldFromBytesIter::<
                ::puroro::tags::Required, ::puroro::tags::SInt64
            >::deser_field(&mut self.s64_required, data),
            142 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::SInt64
            >::deser_field(&mut self.s64_optional, data),
            143 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::SInt64
            >::deser_field(&mut self.s64_repeated, data),
            151 => DeserFieldFromBytesIter::<
                ::puroro::tags::Required, ::puroro::tags::Fixed32
            >::deser_field(&mut self.fixed32_required, data),
            152 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Fixed32
            >::deser_field(&mut self.fixed32_optional, data),
            153 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Fixed32
            >::deser_field(&mut self.fixed32_repeated, data),
            161 => DeserFieldFromBytesIter::<
                ::puroro::tags::Required, ::puroro::tags::Fixed64
            >::deser_field(&mut self.fixed64_required, data),
            162 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Fixed64
            >::deser_field(&mut self.fixed64_optional, data),
            163 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Fixed64
            >::deser_field(&mut self.fixed64_repeated, data),
            171 => DeserFieldFromBytesIter::<
                ::puroro::tags::Required, ::puroro::tags::SFixed32
            >::deser_field(&mut self.sfixed32_required, data),
            172 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::SFixed32
            >::deser_field(&mut self.sfixed32_optional, data),
            173 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::SFixed32
            >::deser_field(&mut self.sfixed32_repeated, data),
            181 => DeserFieldFromBytesIter::<
                ::puroro::tags::Required, ::puroro::tags::SFixed64
            >::deser_field(&mut self.sfixed64_required, data),
            182 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::SFixed64
            >::deser_field(&mut self.sfixed64_optional, data),
            183 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::SFixed64
            >::deser_field(&mut self.sfixed64_repeated, data),
            191 => DeserFieldFromBytesIter::<
                ::puroro::tags::Required, ::puroro::tags::Double
            >::deser_field(&mut self.f64_required, data),
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
                ::puroro::tags::Required,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::i32_required_opt(self),
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
                ::puroro::tags::Required,
                ::puroro::tags::Float,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::float_required_opt(self),
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
                ::puroro::tags::Required,
                ::puroro::tags::Bytes,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::bytes_required_opt(self),
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
                ::puroro::tags::Required,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::string_required_opt(self),
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
                ::puroro::tags::Required,
                ::puroro::tags::Enum2<self::_puroro_root::full_coverage2::Enum>,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::enum_required_opt(self),
                41,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Enum2<self::_puroro_root::full_coverage2::Enum>,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::enum_optional_opt(self),
                42,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Enum2<self::_puroro_root::full_coverage2::Enum>,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::enum_repeated(self),
                43,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Required,
                ::puroro::tags::Message<
                    <Self as super::_puroro_traits::MsgTrait>::Field51ScalarGetterType<'_>,
                >,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::submsg_required_opt(self),
                51,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    <Self as super::_puroro_traits::MsgTrait>::Field52ScalarGetterType<'_>,
                >,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::submsg_optional_opt(self),
                52,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    <Self as super::_puroro_traits::MsgTrait>::Field53ScalarGetterType<'_>,
                >,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::submsg_repeated(self),
                53,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Required,
                ::puroro::tags::Int64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::i64_required_opt(self),
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
                ::puroro::tags::Required,
                ::puroro::tags::UInt32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::u32_required_opt(self),
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
                ::puroro::tags::Required,
                ::puroro::tags::UInt64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::u64_required_opt(self),
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
                ::puroro::tags::Required,
                ::puroro::tags::SInt32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::s32_required_opt(self),
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
                ::puroro::tags::Required,
                ::puroro::tags::SInt64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::s64_required_opt(self),
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
                ::puroro::tags::Required,
                ::puroro::tags::Fixed32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::fixed32_required_opt(self),
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
                ::puroro::tags::Required,
                ::puroro::tags::Fixed64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::fixed64_required_opt(self),
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
                ::puroro::tags::Required,
                ::puroro::tags::SFixed32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::sfixed32_required_opt(self),
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
                ::puroro::tags::Required,
                ::puroro::tags::SFixed64,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::sfixed64_required_opt(self),
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
                ::puroro::tags::Required,
                ::puroro::tags::Double,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::f64_required_opt(self),
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
                    "i32_required",
                    &<Self as super::_puroro_traits::MsgTrait>::i32_required_opt(self),
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
                    "float_required",
                    &<Self as super::_puroro_traits::MsgTrait>::float_required_opt(self),
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
                    "bytes_required",
                    &<Self as super::_puroro_traits::MsgTrait>::bytes_required_opt(self),
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
                    "string_required",
                    &<Self as super::_puroro_traits::MsgTrait>::string_required_opt(self),
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
                    "enum_required",
                    &<Self as super::_puroro_traits::MsgTrait>::enum_required_opt(self),
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
                    "submsg_required",
                    &<Self as super::_puroro_traits::MsgTrait>::submsg_required(self),
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
                    "i64_required",
                    &<Self as super::_puroro_traits::MsgTrait>::i64_required_opt(self),
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
                    "u32_required",
                    &<Self as super::_puroro_traits::MsgTrait>::u32_required_opt(self),
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
                    "u64_required",
                    &<Self as super::_puroro_traits::MsgTrait>::u64_required_opt(self),
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
                    "s32_required",
                    &<Self as super::_puroro_traits::MsgTrait>::s32_required_opt(self),
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
                    "s64_required",
                    &<Self as super::_puroro_traits::MsgTrait>::s64_required_opt(self),
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
                    "fixed32_required",
                    &<Self as super::_puroro_traits::MsgTrait>::fixed32_required_opt(self),
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
                    "fixed64_required",
                    &<Self as super::_puroro_traits::MsgTrait>::fixed64_required_opt(self),
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
                    "sfixed32_required",
                    &<Self as super::_puroro_traits::MsgTrait>::sfixed32_required_opt(self),
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
                    "sfixed64_required",
                    &<Self as super::_puroro_traits::MsgTrait>::sfixed64_required_opt(self),
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
                    "f64_required",
                    &<Self as super::_puroro_traits::MsgTrait>::f64_required_opt(self),
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
                i32_required: ::std::clone::Clone::clone(&self.i32_required),
                i32_optional: ::std::clone::Clone::clone(&self.i32_optional),
                i32_repeated: ::std::clone::Clone::clone(&self.i32_repeated),
                float_required: ::std::clone::Clone::clone(&self.float_required),
                float_optional: ::std::clone::Clone::clone(&self.float_optional),
                float_repeated: ::std::clone::Clone::clone(&self.float_repeated),
                bytes_required: ::std::clone::Clone::clone(&self.bytes_required),
                bytes_optional: ::std::clone::Clone::clone(&self.bytes_optional),
                bytes_repeated: ::std::clone::Clone::clone(&self.bytes_repeated),
                string_required: ::std::clone::Clone::clone(&self.string_required),
                string_optional: ::std::clone::Clone::clone(&self.string_optional),
                string_repeated: ::std::clone::Clone::clone(&self.string_repeated),
                enum_required: ::std::clone::Clone::clone(&self.enum_required),
                enum_optional: ::std::clone::Clone::clone(&self.enum_optional),
                enum_repeated: ::std::clone::Clone::clone(&self.enum_repeated),
                submsg_required: ::std::clone::Clone::clone(&self.submsg_required),
                submsg_optional: ::std::clone::Clone::clone(&self.submsg_optional),
                submsg_repeated: ::std::clone::Clone::clone(&self.submsg_repeated),
                i64_required: ::std::clone::Clone::clone(&self.i64_required),
                i64_optional: ::std::clone::Clone::clone(&self.i64_optional),
                i64_repeated: ::std::clone::Clone::clone(&self.i64_repeated),
                u32_required: ::std::clone::Clone::clone(&self.u32_required),
                u32_optional: ::std::clone::Clone::clone(&self.u32_optional),
                u32_repeated: ::std::clone::Clone::clone(&self.u32_repeated),
                u64_required: ::std::clone::Clone::clone(&self.u64_required),
                u64_optional: ::std::clone::Clone::clone(&self.u64_optional),
                u64_repeated: ::std::clone::Clone::clone(&self.u64_repeated),
                s32_required: ::std::clone::Clone::clone(&self.s32_required),
                s32_optional: ::std::clone::Clone::clone(&self.s32_optional),
                s32_repeated: ::std::clone::Clone::clone(&self.s32_repeated),
                s64_required: ::std::clone::Clone::clone(&self.s64_required),
                s64_optional: ::std::clone::Clone::clone(&self.s64_optional),
                s64_repeated: ::std::clone::Clone::clone(&self.s64_repeated),
                fixed32_required: ::std::clone::Clone::clone(&self.fixed32_required),
                fixed32_optional: ::std::clone::Clone::clone(&self.fixed32_optional),
                fixed32_repeated: ::std::clone::Clone::clone(&self.fixed32_repeated),
                fixed64_required: ::std::clone::Clone::clone(&self.fixed64_required),
                fixed64_optional: ::std::clone::Clone::clone(&self.fixed64_optional),
                fixed64_repeated: ::std::clone::Clone::clone(&self.fixed64_repeated),
                sfixed32_required: ::std::clone::Clone::clone(&self.sfixed32_required),
                sfixed32_optional: ::std::clone::Clone::clone(&self.sfixed32_optional),
                sfixed32_repeated: ::std::clone::Clone::clone(&self.sfixed32_repeated),
                sfixed64_required: ::std::clone::Clone::clone(&self.sfixed64_required),
                sfixed64_optional: ::std::clone::Clone::clone(&self.sfixed64_optional),
                sfixed64_repeated: ::std::clone::Clone::clone(&self.sfixed64_repeated),
                f64_required: ::std::clone::Clone::clone(&self.f64_required),
                f64_optional: ::std::clone::Clone::clone(&self.f64_optional),
                f64_repeated: ::std::clone::Clone::clone(&self.f64_repeated),
            }
        }
    }

    impl ::std::cmp::PartialEq for Msg {
        fn eq(&self, rhs: &Self) -> bool {
            self.i32_required == rhs.i32_required
                && self.i32_optional == rhs.i32_optional
                && self.i32_repeated == rhs.i32_repeated
                && self.float_required == rhs.float_required
                && self.float_optional == rhs.float_optional
                && self.float_repeated == rhs.float_repeated
                && self.bytes_required == rhs.bytes_required
                && self.bytes_optional == rhs.bytes_optional
                && self.bytes_repeated == rhs.bytes_repeated
                && self.string_required == rhs.string_required
                && self.string_optional == rhs.string_optional
                && self.string_repeated == rhs.string_repeated
                && self.enum_required == rhs.enum_required
                && self.enum_optional == rhs.enum_optional
                && self.enum_repeated == rhs.enum_repeated
                && self.submsg_required == rhs.submsg_required
                && self.submsg_optional == rhs.submsg_optional
                && self.submsg_repeated == rhs.submsg_repeated
                && self.i64_required == rhs.i64_required
                && self.i64_optional == rhs.i64_optional
                && self.i64_repeated == rhs.i64_repeated
                && self.u32_required == rhs.u32_required
                && self.u32_optional == rhs.u32_optional
                && self.u32_repeated == rhs.u32_repeated
                && self.u64_required == rhs.u64_required
                && self.u64_optional == rhs.u64_optional
                && self.u64_repeated == rhs.u64_repeated
                && self.s32_required == rhs.s32_required
                && self.s32_optional == rhs.s32_optional
                && self.s32_repeated == rhs.s32_repeated
                && self.s64_required == rhs.s64_required
                && self.s64_optional == rhs.s64_optional
                && self.s64_repeated == rhs.s64_repeated
                && self.fixed32_required == rhs.fixed32_required
                && self.fixed32_optional == rhs.fixed32_optional
                && self.fixed32_repeated == rhs.fixed32_repeated
                && self.fixed64_required == rhs.fixed64_required
                && self.fixed64_optional == rhs.fixed64_optional
                && self.fixed64_repeated == rhs.fixed64_repeated
                && self.sfixed32_required == rhs.sfixed32_required
                && self.sfixed32_optional == rhs.sfixed32_optional
                && self.sfixed32_repeated == rhs.sfixed32_repeated
                && self.sfixed64_required == rhs.sfixed64_required
                && self.sfixed64_optional == rhs.sfixed64_optional
                && self.sfixed64_repeated == rhs.sfixed64_repeated
                && self.f64_required == rhs.f64_required
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
}
pub use _puroro_traits::*;
pub mod _puroro_traits {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }

    pub trait MsgTrait {
        fn i32_required<'this>(&'this self) -> i32 {
            self.i32_required_opt()
                .unwrap_or(::std::default::Default::default())
        }

        fn has_i32_required<'this>(&'this self) -> bool {
            self.i32_required_opt().is_some()
        }
        fn i32_required_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::None
        }

        fn i32_optional<'this>(&'this self) -> i32 {
            self.i32_optional_opt()
                .unwrap_or(::std::default::Default::default())
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

        fn float_required<'this>(&'this self) -> f32 {
            self.float_required_opt()
                .unwrap_or(::std::default::Default::default())
        }

        fn has_float_required<'this>(&'this self) -> bool {
            self.float_required_opt().is_some()
        }
        fn float_required_opt<'this>(&'this self) -> ::std::option::Option<f32> {
            ::std::option::Option::None
        }

        fn float_optional<'this>(&'this self) -> f32 {
            self.float_optional_opt()
                .unwrap_or(::std::default::Default::default())
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

        type Field21ScalarGetterType<'this>: ::std::convert::AsRef<[u8]>
            + ::std::convert::From<&'static [u8]>
        where
            Self: 'this;

        fn bytes_required<'this>(&'this self) -> Self::Field21ScalarGetterType<'this> {
            self.bytes_required_opt()
                .unwrap_or(Self::bytes_required_default_value())
        }
        fn bytes_required_default_value() -> Self::Field21ScalarGetterType<'static>;

        fn has_bytes_required<'this>(&'this self) -> bool {
            self.bytes_required_opt().is_some()
        }
        fn bytes_required_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field21ScalarGetterType<'this>> {
            ::std::option::Option::None
        }

        type Field22ScalarGetterType<'this>: ::std::convert::AsRef<[u8]>
            + ::std::convert::From<&'static [u8]>
        where
            Self: 'this;

        fn bytes_optional<'this>(&'this self) -> Self::Field22ScalarGetterType<'this> {
            self.bytes_optional_opt()
                .unwrap_or(Self::bytes_optional_default_value())
        }
        fn bytes_optional_default_value() -> Self::Field22ScalarGetterType<'static>;

        fn has_bytes_optional<'this>(&'this self) -> bool {
            self.bytes_optional_opt().is_some()
        }
        fn bytes_optional_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field22ScalarGetterType<'this>> {
            ::std::option::Option::None
        }

        type Field23ScalarGetterType<'this>: ::std::convert::AsRef<[u8]>
            + ::std::convert::From<&'static [u8]>
        where
            Self: 'this;

        type Field23RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field23ScalarGetterType<'this>>
        where
            Self: 'this;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this>;

        type Field31ScalarGetterType<'this>: ::std::convert::AsRef<str>
            + ::std::convert::From<&'static str>
        where
            Self: 'this;

        fn string_required<'this>(&'this self) -> Self::Field31ScalarGetterType<'this> {
            self.string_required_opt()
                .unwrap_or(Self::string_required_default_value())
        }
        fn string_required_default_value() -> Self::Field31ScalarGetterType<'static>;

        fn has_string_required<'this>(&'this self) -> bool {
            self.string_required_opt().is_some()
        }
        fn string_required_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field31ScalarGetterType<'this>> {
            ::std::option::Option::None
        }

        type Field32ScalarGetterType<'this>: ::std::convert::AsRef<str>
            + ::std::convert::From<&'static str>
        where
            Self: 'this;

        fn string_optional<'this>(&'this self) -> Self::Field32ScalarGetterType<'this> {
            self.string_optional_opt()
                .unwrap_or(Self::string_optional_default_value())
        }
        fn string_optional_default_value() -> Self::Field32ScalarGetterType<'static>;

        fn has_string_optional<'this>(&'this self) -> bool {
            self.string_optional_opt().is_some()
        }
        fn string_optional_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field32ScalarGetterType<'this>> {
            ::std::option::Option::None
        }

        type Field33ScalarGetterType<'this>: ::std::convert::AsRef<str>
            + ::std::convert::From<&'static str>
        where
            Self: 'this;

        type Field33RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field33ScalarGetterType<'this>>
        where
            Self: 'this;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this>;

        fn enum_required<'this>(&'this self) -> self::_puroro_root::full_coverage2::Enum {
            self.enum_required_opt()
                .unwrap_or(::std::default::Default::default())
        }

        fn has_enum_required<'this>(&'this self) -> bool {
            self.enum_required_opt().is_some()
        }
        fn enum_required_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::full_coverage2::Enum> {
            ::std::option::Option::None
        }

        fn enum_optional<'this>(&'this self) -> self::_puroro_root::full_coverage2::Enum {
            self.enum_optional_opt()
                .unwrap_or(::std::default::Default::default())
        }

        fn has_enum_optional<'this>(&'this self) -> bool {
            self.enum_optional_opt().is_some()
        }
        fn enum_optional_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::full_coverage2::Enum> {
            ::std::option::Option::None
        }

        type Field43RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = self::_puroro_root::full_coverage2::Enum>
        where
            Self: 'this;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this>;

        type Field51ScalarGetterType<'this>: self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_traits::SubmsgTrait
            where Self: 'this;

        fn submsg_required<'this>(&'this self) -> Self::Field51ScalarGetterType<'this> {
            self.submsg_required_opt()
                .unwrap_or(Self::submsg_required_default_value())
        }
        fn submsg_required_default_value() -> Self::Field51ScalarGetterType<'static>;

        fn has_submsg_required<'this>(&'this self) -> bool {
            self.submsg_required_opt().is_some()
        }
        fn submsg_required_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field51ScalarGetterType<'this>> {
            ::std::option::Option::None
        }

        type Field52ScalarGetterType<'this>: self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_traits::SubmsgTrait
            where Self: 'this;

        fn submsg_optional<'this>(&'this self) -> Self::Field52ScalarGetterType<'this> {
            self.submsg_optional_opt()
                .unwrap_or(Self::submsg_optional_default_value())
        }
        fn submsg_optional_default_value() -> Self::Field52ScalarGetterType<'static>;

        fn has_submsg_optional<'this>(&'this self) -> bool {
            self.submsg_optional_opt().is_some()
        }
        fn submsg_optional_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field52ScalarGetterType<'this>> {
            ::std::option::Option::None
        }

        type Field53ScalarGetterType<'this>: self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_traits::SubmsgTrait
            where Self: 'this;

        type Field53RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field53ScalarGetterType<'this>>
        where
            Self: 'this;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this>;

        fn i64_required<'this>(&'this self) -> i64 {
            self.i64_required_opt()
                .unwrap_or(::std::default::Default::default())
        }

        fn has_i64_required<'this>(&'this self) -> bool {
            self.i64_required_opt().is_some()
        }
        fn i64_required_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            ::std::option::Option::None
        }

        fn i64_optional<'this>(&'this self) -> i64 {
            self.i64_optional_opt()
                .unwrap_or(::std::default::Default::default())
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

        fn u32_required<'this>(&'this self) -> u32 {
            self.u32_required_opt()
                .unwrap_or(::std::default::Default::default())
        }

        fn has_u32_required<'this>(&'this self) -> bool {
            self.u32_required_opt().is_some()
        }
        fn u32_required_opt<'this>(&'this self) -> ::std::option::Option<u32> {
            ::std::option::Option::None
        }

        fn u32_optional<'this>(&'this self) -> u32 {
            self.u32_optional_opt()
                .unwrap_or(::std::default::Default::default())
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

        fn u64_required<'this>(&'this self) -> u64 {
            self.u64_required_opt()
                .unwrap_or(::std::default::Default::default())
        }

        fn has_u64_required<'this>(&'this self) -> bool {
            self.u64_required_opt().is_some()
        }
        fn u64_required_opt<'this>(&'this self) -> ::std::option::Option<u64> {
            ::std::option::Option::None
        }

        fn u64_optional<'this>(&'this self) -> u64 {
            self.u64_optional_opt()
                .unwrap_or(::std::default::Default::default())
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

        fn s32_required<'this>(&'this self) -> i32 {
            self.s32_required_opt()
                .unwrap_or(::std::default::Default::default())
        }

        fn has_s32_required<'this>(&'this self) -> bool {
            self.s32_required_opt().is_some()
        }
        fn s32_required_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::None
        }

        fn s32_optional<'this>(&'this self) -> i32 {
            self.s32_optional_opt()
                .unwrap_or(::std::default::Default::default())
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

        fn s64_required<'this>(&'this self) -> i64 {
            self.s64_required_opt()
                .unwrap_or(::std::default::Default::default())
        }

        fn has_s64_required<'this>(&'this self) -> bool {
            self.s64_required_opt().is_some()
        }
        fn s64_required_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            ::std::option::Option::None
        }

        fn s64_optional<'this>(&'this self) -> i64 {
            self.s64_optional_opt()
                .unwrap_or(::std::default::Default::default())
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

        fn fixed32_required<'this>(&'this self) -> u32 {
            self.fixed32_required_opt()
                .unwrap_or(::std::default::Default::default())
        }

        fn has_fixed32_required<'this>(&'this self) -> bool {
            self.fixed32_required_opt().is_some()
        }
        fn fixed32_required_opt<'this>(&'this self) -> ::std::option::Option<u32> {
            ::std::option::Option::None
        }

        fn fixed32_optional<'this>(&'this self) -> u32 {
            self.fixed32_optional_opt()
                .unwrap_or(::std::default::Default::default())
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

        fn fixed64_required<'this>(&'this self) -> u64 {
            self.fixed64_required_opt()
                .unwrap_or(::std::default::Default::default())
        }

        fn has_fixed64_required<'this>(&'this self) -> bool {
            self.fixed64_required_opt().is_some()
        }
        fn fixed64_required_opt<'this>(&'this self) -> ::std::option::Option<u64> {
            ::std::option::Option::None
        }

        fn fixed64_optional<'this>(&'this self) -> u64 {
            self.fixed64_optional_opt()
                .unwrap_or(::std::default::Default::default())
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

        fn sfixed32_required<'this>(&'this self) -> i32 {
            self.sfixed32_required_opt()
                .unwrap_or(::std::default::Default::default())
        }

        fn has_sfixed32_required<'this>(&'this self) -> bool {
            self.sfixed32_required_opt().is_some()
        }
        fn sfixed32_required_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::None
        }

        fn sfixed32_optional<'this>(&'this self) -> i32 {
            self.sfixed32_optional_opt()
                .unwrap_or(::std::default::Default::default())
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

        fn sfixed64_required<'this>(&'this self) -> i64 {
            self.sfixed64_required_opt()
                .unwrap_or(::std::default::Default::default())
        }

        fn has_sfixed64_required<'this>(&'this self) -> bool {
            self.sfixed64_required_opt().is_some()
        }
        fn sfixed64_required_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            ::std::option::Option::None
        }

        fn sfixed64_optional<'this>(&'this self) -> i64 {
            self.sfixed64_optional_opt()
                .unwrap_or(::std::default::Default::default())
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

        fn f64_required<'this>(&'this self) -> f64 {
            self.f64_required_opt()
                .unwrap_or(::std::default::Default::default())
        }

        fn has_f64_required<'this>(&'this self) -> bool {
            self.f64_required_opt().is_some()
        }
        fn f64_required_opt<'this>(&'this self) -> ::std::option::Option<f64> {
            ::std::option::Option::None
        }

        fn f64_optional<'this>(&'this self) -> f64 {
            self.f64_optional_opt()
                .unwrap_or(::std::default::Default::default())
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
            fn i32_required_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).i32_required_opt()
            }
            fn i32_optional_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).i32_optional_opt()
            }

            type Field3RepeatedType<'this> where Self: 'this =
                <$ty as MsgTrait>::Field3RepeatedType<'this>;
            fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
                (**self).i32_repeated()
            }
            fn float_required_opt<'this>(&'this self) -> ::std::option::Option<f32> {
                (**self).float_required_opt()
            }
            fn float_optional_opt<'this>(&'this self) -> ::std::option::Option<f32> {
                (**self).float_optional_opt()
            }

            type Field13RepeatedType<'this> where Self: 'this =
                <$ty as MsgTrait>::Field13RepeatedType<'this>;
            fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
                (**self).float_repeated()
            }
            type Field21ScalarGetterType<'this> where Self: 'this =
                <$ty as MsgTrait>::Field21ScalarGetterType<'this>;
            fn bytes_required_opt<'this>(&'this self) -> ::std::option::Option<Self::Field21ScalarGetterType<'this>> {
                (**self).bytes_required_opt()
            }
            fn bytes_required_default_value() -> <$ty as MsgTrait>::Field21ScalarGetterType<'static> {
                <$ty as MsgTrait>::bytes_required_default_value()
            }
            type Field22ScalarGetterType<'this> where Self: 'this =
                <$ty as MsgTrait>::Field22ScalarGetterType<'this>;
            fn bytes_optional_opt<'this>(&'this self) -> ::std::option::Option<Self::Field22ScalarGetterType<'this>> {
                (**self).bytes_optional_opt()
            }
            fn bytes_optional_default_value() -> <$ty as MsgTrait>::Field22ScalarGetterType<'static> {
                <$ty as MsgTrait>::bytes_optional_default_value()
            }
            type Field23ScalarGetterType<'this> where Self: 'this =
                <$ty as MsgTrait>::Field23ScalarGetterType<'this>;

            type Field23RepeatedType<'this> where Self: 'this =
                <$ty as MsgTrait>::Field23RepeatedType<'this>;
            fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
                (**self).bytes_repeated()
            }
            type Field31ScalarGetterType<'this> where Self: 'this =
                <$ty as MsgTrait>::Field31ScalarGetterType<'this>;
            fn string_required_opt<'this>(&'this self) -> ::std::option::Option<Self::Field31ScalarGetterType<'this>> {
                (**self).string_required_opt()
            }
            fn string_required_default_value() -> <$ty as MsgTrait>::Field31ScalarGetterType<'static> {
                <$ty as MsgTrait>::string_required_default_value()
            }
            type Field32ScalarGetterType<'this> where Self: 'this =
                <$ty as MsgTrait>::Field32ScalarGetterType<'this>;
            fn string_optional_opt<'this>(&'this self) -> ::std::option::Option<Self::Field32ScalarGetterType<'this>> {
                (**self).string_optional_opt()
            }
            fn string_optional_default_value() -> <$ty as MsgTrait>::Field32ScalarGetterType<'static> {
                <$ty as MsgTrait>::string_optional_default_value()
            }
            type Field33ScalarGetterType<'this> where Self: 'this =
                <$ty as MsgTrait>::Field33ScalarGetterType<'this>;

            type Field33RepeatedType<'this> where Self: 'this =
                <$ty as MsgTrait>::Field33RepeatedType<'this>;
            fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
                (**self).string_repeated()
            }
            fn enum_required_opt<'this>(&'this self) -> ::std::option::Option<self::_puroro_root::full_coverage2::Enum> {
                (**self).enum_required_opt()
            }
            fn enum_optional_opt<'this>(&'this self) -> ::std::option::Option<self::_puroro_root::full_coverage2::Enum> {
                (**self).enum_optional_opt()
            }

            type Field43RepeatedType<'this> where Self: 'this =
                <$ty as MsgTrait>::Field43RepeatedType<'this>;
            fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
                (**self).enum_repeated()
            }
            type Field51ScalarGetterType<'this> where Self: 'this =
                <$ty as MsgTrait>::Field51ScalarGetterType<'this>;
            fn submsg_required_opt<'this>(&'this self) -> ::std::option::Option<Self::Field51ScalarGetterType<'this>> {
                (**self).submsg_required_opt()
            }
            fn submsg_required_default_value() -> <$ty as MsgTrait>::Field51ScalarGetterType<'static> {
                <$ty as MsgTrait>::submsg_required_default_value()
            }
            type Field52ScalarGetterType<'this> where Self: 'this =
                <$ty as MsgTrait>::Field52ScalarGetterType<'this>;
            fn submsg_optional_opt<'this>(&'this self) -> ::std::option::Option<Self::Field52ScalarGetterType<'this>> {
                (**self).submsg_optional_opt()
            }
            fn submsg_optional_default_value() -> <$ty as MsgTrait>::Field52ScalarGetterType<'static> {
                <$ty as MsgTrait>::submsg_optional_default_value()
            }
            type Field53ScalarGetterType<'this> where Self: 'this =
                <$ty as MsgTrait>::Field53ScalarGetterType<'this>;

            type Field53RepeatedType<'this> where Self: 'this =
                <$ty as MsgTrait>::Field53RepeatedType<'this>;
            fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
                (**self).submsg_repeated()
            }
            fn i64_required_opt<'this>(&'this self) -> ::std::option::Option<i64> {
                (**self).i64_required_opt()
            }
            fn i64_optional_opt<'this>(&'this self) -> ::std::option::Option<i64> {
                (**self).i64_optional_opt()
            }

            type Field103RepeatedType<'this> where Self: 'this =
                <$ty as MsgTrait>::Field103RepeatedType<'this>;
            fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
                (**self).i64_repeated()
            }
            fn u32_required_opt<'this>(&'this self) -> ::std::option::Option<u32> {
                (**self).u32_required_opt()
            }
            fn u32_optional_opt<'this>(&'this self) -> ::std::option::Option<u32> {
                (**self).u32_optional_opt()
            }

            type Field113RepeatedType<'this> where Self: 'this =
                <$ty as MsgTrait>::Field113RepeatedType<'this>;
            fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
                (**self).u32_repeated()
            }
            fn u64_required_opt<'this>(&'this self) -> ::std::option::Option<u64> {
                (**self).u64_required_opt()
            }
            fn u64_optional_opt<'this>(&'this self) -> ::std::option::Option<u64> {
                (**self).u64_optional_opt()
            }

            type Field123RepeatedType<'this> where Self: 'this =
                <$ty as MsgTrait>::Field123RepeatedType<'this>;
            fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
                (**self).u64_repeated()
            }
            fn s32_required_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).s32_required_opt()
            }
            fn s32_optional_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).s32_optional_opt()
            }

            type Field133RepeatedType<'this> where Self: 'this =
                <$ty as MsgTrait>::Field133RepeatedType<'this>;
            fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
                (**self).s32_repeated()
            }
            fn s64_required_opt<'this>(&'this self) -> ::std::option::Option<i64> {
                (**self).s64_required_opt()
            }
            fn s64_optional_opt<'this>(&'this self) -> ::std::option::Option<i64> {
                (**self).s64_optional_opt()
            }

            type Field143RepeatedType<'this> where Self: 'this =
                <$ty as MsgTrait>::Field143RepeatedType<'this>;
            fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
                (**self).s64_repeated()
            }
            fn fixed32_required_opt<'this>(&'this self) -> ::std::option::Option<u32> {
                (**self).fixed32_required_opt()
            }
            fn fixed32_optional_opt<'this>(&'this self) -> ::std::option::Option<u32> {
                (**self).fixed32_optional_opt()
            }

            type Field153RepeatedType<'this> where Self: 'this =
                <$ty as MsgTrait>::Field153RepeatedType<'this>;
            fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
                (**self).fixed32_repeated()
            }
            fn fixed64_required_opt<'this>(&'this self) -> ::std::option::Option<u64> {
                (**self).fixed64_required_opt()
            }
            fn fixed64_optional_opt<'this>(&'this self) -> ::std::option::Option<u64> {
                (**self).fixed64_optional_opt()
            }

            type Field163RepeatedType<'this> where Self: 'this =
                <$ty as MsgTrait>::Field163RepeatedType<'this>;
            fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
                (**self).fixed64_repeated()
            }
            fn sfixed32_required_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).sfixed32_required_opt()
            }
            fn sfixed32_optional_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).sfixed32_optional_opt()
            }

            type Field173RepeatedType<'this> where Self: 'this =
                <$ty as MsgTrait>::Field173RepeatedType<'this>;
            fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
                (**self).sfixed32_repeated()
            }
            fn sfixed64_required_opt<'this>(&'this self) -> ::std::option::Option<i64> {
                (**self).sfixed64_required_opt()
            }
            fn sfixed64_optional_opt<'this>(&'this self) -> ::std::option::Option<i64> {
                (**self).sfixed64_optional_opt()
            }

            type Field183RepeatedType<'this> where Self: 'this =
                <$ty as MsgTrait>::Field183RepeatedType<'this>;
            fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
                (**self).sfixed64_repeated()
            }
            fn f64_required_opt<'this>(&'this self) -> ::std::option::Option<f64> {
                (**self).f64_required_opt()
            }
            fn f64_optional_opt<'this>(&'this self) -> ::std::option::Option<f64> {
                (**self).f64_optional_opt()
            }

            type Field193RepeatedType<'this> where Self: 'this =
                <$ty as MsgTrait>::Field193RepeatedType<'this>;
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
}
#[derive(::std::fmt::Debug, ::std::clone::Clone, ::std::marker::Copy, ::std::cmp::PartialEq)]
pub enum Enum {
    Zeroth,
    First,
    Tenth,
}

impl ::puroro::Enum2 for Enum {}
impl ::std::convert::TryFrom<i32> for Enum {
    type Error = i32;
    fn try_from(value: i32) -> ::std::result::Result<Self, i32> {
        ::std::result::Result::Ok(match value {
            0 => Enum::Zeroth,
            1 => Enum::First,
            10 => Enum::Tenth,
            _ => Err(value)?,
        })
    }
}

impl ::std::convert::From<Enum> for i32 {
    fn from(value: Enum) -> i32 {
        match value {
            Enum::Zeroth => 0,
            Enum::First => 1,
            Enum::Tenth => 10,
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
                i32_required: ::std::option::Option<i32>,
                i64_required: ::std::option::Option<i64>,
            }
            impl ::puroro::Message<Submsg> for Submsg {}

            impl Submsg {
                pub fn new() -> Self {
                    Self {
                        i32_required: ::std::default::Default::default(),
                        i64_required: ::std::default::Default::default(),
                    }
                }
                pub fn i32_required_mut(&mut self) -> &mut ::std::option::Option<i32> {
                    &mut self.i32_required
                }
                pub fn i64_required_mut(&mut self) -> &mut ::std::option::Option<i64> {
                    &mut self.i64_required
                }
            }

            impl super::_puroro_traits::SubmsgTrait for Submsg {
                fn i32_required_opt<'this>(&'this self) -> Option<i32> {
                    Clone::clone(&self.i32_required)
                }
                fn i64_required_opt<'this>(&'this self) -> Option<i64> {
                    Clone::clone(&self.i64_required)
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
                            ::puroro::tags::Required,
                            ::puroro::tags::Int32,
                        >::deser_field(&mut self.i32_required, data),
                        101 => DeserFieldFromBytesIter::<
                            ::puroro::tags::Required,
                            ::puroro::tags::Int64,
                        >::deser_field(&mut self.i64_required, data),

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
                        ::puroro::tags::Required,
                        ::puroro::tags::Int32,
                    >::ser_field(
                        <Self as super::_puroro_traits::SubmsgTrait>::i32_required_opt(self),
                        1,
                        out,
                    )?;
                    ::puroro::internal::se::SerFieldToIoWrite::<
                        ::puroro::tags::Required,
                        ::puroro::tags::Int64,
                    >::ser_field(
                        <Self as super::_puroro_traits::SubmsgTrait>::i64_required_opt(self),
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
                            "i32_required",
                            &<Self as super::_puroro_traits::SubmsgTrait>::i32_required_opt(self),
                        )
                        .field(
                            "i64_required",
                            &<Self as super::_puroro_traits::SubmsgTrait>::i64_required_opt(self),
                        )
                        .finish()
                }
            }

            impl ::std::clone::Clone for Submsg {
                fn clone(&self) -> Self {
                    Self {
                        i32_required: ::std::clone::Clone::clone(&self.i32_required),
                        i64_required: ::std::clone::Clone::clone(&self.i64_required),
                    }
                }
            }

            impl ::std::cmp::PartialEq for Submsg {
                fn eq(&self, rhs: &Self) -> bool {
                    self.i32_required == rhs.i32_required
                        && self.i64_required == rhs.i64_required
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

            pub trait SubmsgTrait {
                fn i32_required<'this>(&'this self) -> i32 {
                    self.i32_required_opt()
                        .unwrap_or(::std::default::Default::default())
                }

                fn has_i32_required<'this>(&'this self) -> bool {
                    self.i32_required_opt().is_some()
                }
                fn i32_required_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                    ::std::option::Option::None
                }

                fn i64_required<'this>(&'this self) -> i64 {
                    self.i64_required_opt()
                        .unwrap_or(::std::default::Default::default())
                }

                fn has_i64_required<'this>(&'this self) -> bool {
                    self.i64_required_opt().is_some()
                }
                fn i64_required_opt<'this>(&'this self) -> ::std::option::Option<i64> {
                    ::std::option::Option::None
                }
            }

            macro_rules! submsg_delegate {
                ($ty:ty) => {
                    fn i32_required_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                        (**self).i32_required_opt()
                    }
                    fn i64_required_opt<'this>(&'this self) -> ::std::option::Option<i64> {
                        (**self).i64_required_opt()
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
