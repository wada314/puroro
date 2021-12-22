// A generated source code by puroro library
// package ser_tests2

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub use _puroro_simple_impl::Msg;
pub mod _puroro_simple_impl {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
    pub struct Msg {
        i32_optional: ::std::option::Option<i32>,
        i32_repeated: ::std::vec::Vec<i32>,
        float_optional: ::std::option::Option<f32>,
        float_repeated: ::std::vec::Vec<f32>,
        string_optional: ::std::option::Option<::std::string::String>,
        string_repeated: ::std::vec::Vec<::std::string::String>,
        submsg_optional: ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_simple_impl::Submsg,
            >,
        >,
        submsg_repeated: ::std::vec::Vec<
            self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_simple_impl::Submsg,
        >,
        enum_optional: ::std::option::Option<self::_puroro_root::ser_tests2::Enum>,
        enum_repeated: ::std::vec::Vec<self::_puroro_root::ser_tests2::Enum>,
        very_large_field_number: ::std::option::Option<i32>,
    }
    impl ::puroro::Message<Msg> for Msg {}

    impl Msg {
        pub fn new() -> Self {
            Self {
                i32_optional: ::std::default::Default::default(),
                i32_repeated: ::std::default::Default::default(),
                float_optional: ::std::default::Default::default(),
                float_repeated: ::std::default::Default::default(),
                string_optional: ::std::default::Default::default(),
                string_repeated: ::std::default::Default::default(),
                submsg_optional: ::std::default::Default::default(),
                submsg_repeated: ::std::default::Default::default(),
                enum_optional: ::std::default::Default::default(),
                enum_repeated: ::std::default::Default::default(),
                very_large_field_number: ::std::default::Default::default(),
            }
        }
        pub fn i32_optional_mut(&mut self) -> &mut ::std::option::Option<i32> {
            &mut self.i32_optional
        }
        pub fn i32_repeated_mut(&mut self) -> &mut ::std::vec::Vec<i32> {
            &mut self.i32_repeated
        }
        pub fn float_optional_mut(&mut self) -> &mut ::std::option::Option<f32> {
            &mut self.float_optional
        }
        pub fn float_repeated_mut(&mut self) -> &mut ::std::vec::Vec<f32> {
            &mut self.float_repeated
        }
        pub fn string_optional_mut(&mut self) -> &mut ::std::option::Option<::std::string::String> {
            &mut self.string_optional
        }
        pub fn string_repeated_mut(&mut self) -> &mut ::std::vec::Vec<::std::string::String> {
            &mut self.string_repeated
        }
        pub fn submsg_optional_mut(
            &mut self,
        ) -> &mut ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_simple_impl::Submsg,
            >,
        > {
            &mut self.submsg_optional
        }
        pub fn submsg_repeated_mut(
            &mut self,
        ) -> &mut ::std::vec::Vec<
            self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_simple_impl::Submsg,
        > {
            &mut self.submsg_repeated
        }
        pub fn enum_optional_mut(
            &mut self,
        ) -> &mut ::std::option::Option<self::_puroro_root::ser_tests2::Enum> {
            &mut self.enum_optional
        }
        pub fn enum_repeated_mut(
            &mut self,
        ) -> &mut ::std::vec::Vec<self::_puroro_root::ser_tests2::Enum> {
            &mut self.enum_repeated
        }
        pub fn very_large_field_number_mut(&mut self) -> &mut ::std::option::Option<i32> {
            &mut self.very_large_field_number
        }
    }

    impl super::_puroro_traits::MsgTrait for Msg {
        fn i32_optional_opt<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.i32_optional)
        }
        type Field2RepeatedType<'this> =
            ::puroro::CloneThenIntoRepeatedField<'this, ::std::vec::Vec<i32>, i32, i32>;

        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.i32_repeated)
        }
        fn float_optional_opt<'this>(&'this self) -> Option<f32> {
            Clone::clone(&self.float_optional)
        }
        type Field4RepeatedType<'this> =
            ::puroro::CloneThenIntoRepeatedField<'this, ::std::vec::Vec<f32>, f32, f32>;

        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.float_repeated)
        }
        type Field5ScalarGetterType<'this>
        where
            Self: 'this,
        = &'this ::std::string::String;
        fn string_optional_opt<'this>(&'this self) -> Option<Self::Field5ScalarGetterType<'this>> {
            self.string_optional.as_ref()
        }
        fn string_optional_default_value(&self) -> Self::Field5ScalarGetterType<'_> {
            static DEFAULT_VALUE: ::puroro::once_cell::sync::Lazy<::std::string::String> =
                ::puroro::once_cell::sync::Lazy::new(|| {
                    ::std::convert::From::<&str>::from(::std::default::Default::default())
                });

            &DEFAULT_VALUE
        }
        type Field6ScalarGetterType<'this>
        where
            Self: 'this,
        = &'this ::std::string::String;
        type Field6RepeatedType<'this> = &'this [::std::string::String];

        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            &self.string_repeated
        }
        type Field7ScalarGetterType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_simple_impl::Submsg;
        fn submsg_optional_opt<'this>(&'this self) -> Option<Self::Field7ScalarGetterType<'this>> {
            self.submsg_optional.as_ref().map(|x| x.as_ref())
        }
        fn submsg_optional_default_value(&self) -> Self::Field7ScalarGetterType<'_> {
            static DEFAULT_VALUE: ::puroro::once_cell::sync::Lazy<
                self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_simple_impl::Submsg,
            > = ::puroro::once_cell::sync::Lazy::new(|| ::std::default::Default::default());

            &DEFAULT_VALUE
        }
        type Field8ScalarGetterType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_simple_impl::Submsg;
        type Field8RepeatedType<'this> = &'this [self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_simple_impl::Submsg];

        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            &self.submsg_repeated
        }
        fn enum_optional_opt<'this>(&'this self) -> Option<self::_puroro_root::ser_tests2::Enum> {
            Clone::clone(&self.enum_optional)
        }
        type Field10RepeatedType<'this> = ::puroro::CloneThenIntoRepeatedField<
            'this,
            ::std::vec::Vec<self::_puroro_root::ser_tests2::Enum>,
            self::_puroro_root::ser_tests2::Enum,
            self::_puroro_root::ser_tests2::Enum,
        >;

        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.enum_repeated)
        }
        fn very_large_field_number_opt<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.very_large_field_number)
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
            >::deser_field(&mut self.i32_optional, data),
            2 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Int32
            >::deser_field(&mut self.i32_repeated, data),
            3 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Float
            >::deser_field(&mut self.float_optional, data),
            4 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Float
            >::deser_field(&mut self.float_repeated, data),
            5 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.string_optional, data),
            6 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::String
            >::deser_field(&mut self.string_repeated, data),
            7 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Message<::std::boxed::Box<self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_simple_impl::Submsg>>
            >::deser_field(&mut self.submsg_optional, data),
            8 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_simple_impl::Submsg>
            >::deser_field(&mut self.submsg_repeated, data),
            9 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Enum2<self::_puroro_root::ser_tests2::Enum>
            >::deser_field(&mut self.enum_optional, data),
            10 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Enum2<self::_puroro_root::ser_tests2::Enum>
            >::deser_field(&mut self.enum_repeated, data),
            536870911 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Int32
            >::deser_field(&mut self.very_large_field_number, data),

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
                <Self as super::_puroro_traits::MsgTrait>::i32_optional_opt(self),
                1,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::i32_repeated(self),
                2,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Float,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::float_optional_opt(self),
                3,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Float,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::float_repeated(self),
                4,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::string_optional_opt(self),
                5,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::string_repeated(self),
                6,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    <Self as super::_puroro_traits::MsgTrait>::Field7ScalarGetterType<'_>,
                >,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::submsg_optional_opt(self),
                7,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    <Self as super::_puroro_traits::MsgTrait>::Field8ScalarGetterType<'_>,
                >,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::submsg_repeated(self),
                8,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Enum2<self::_puroro_root::ser_tests2::Enum>,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::enum_optional_opt(self),
                9,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Enum2<self::_puroro_root::ser_tests2::Enum>,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::enum_repeated(self),
                10,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::very_large_field_number_opt(self),
                536870911,
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
                    "very_large_field_number",
                    &<Self as super::_puroro_traits::MsgTrait>::very_large_field_number_opt(self),
                )
                .finish()
        }
    }

    impl ::std::clone::Clone for Msg {
        fn clone(&self) -> Self {
            Self {
                i32_optional: ::std::clone::Clone::clone(&self.i32_optional),
                i32_repeated: ::std::clone::Clone::clone(&self.i32_repeated),
                float_optional: ::std::clone::Clone::clone(&self.float_optional),
                float_repeated: ::std::clone::Clone::clone(&self.float_repeated),
                string_optional: ::std::clone::Clone::clone(&self.string_optional),
                string_repeated: ::std::clone::Clone::clone(&self.string_repeated),
                submsg_optional: ::std::clone::Clone::clone(&self.submsg_optional),
                submsg_repeated: ::std::clone::Clone::clone(&self.submsg_repeated),
                enum_optional: ::std::clone::Clone::clone(&self.enum_optional),
                enum_repeated: ::std::clone::Clone::clone(&self.enum_repeated),
                very_large_field_number: ::std::clone::Clone::clone(&self.very_large_field_number),
            }
        }
    }

    impl ::std::cmp::PartialEq for Msg {
        fn eq(&self, rhs: &Self) -> bool {
            self.i32_optional == rhs.i32_optional
                && self.i32_repeated == rhs.i32_repeated
                && self.float_optional == rhs.float_optional
                && self.float_repeated == rhs.float_repeated
                && self.string_optional == rhs.string_optional
                && self.string_repeated == rhs.string_repeated
                && self.submsg_optional == rhs.submsg_optional
                && self.submsg_repeated == rhs.submsg_repeated
                && self.enum_optional == rhs.enum_optional
                && self.enum_repeated == rhs.enum_repeated
                && self.very_large_field_number == rhs.very_large_field_number
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

        type Field2RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = i32>
        where
            Self: 'this;
        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this>;

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

        type Field4RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = f32>
        where
            Self: 'this;
        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this>;

        type Field5ScalarGetterType<'this>: ::std::convert::AsRef<str>
        where
            Self: 'this;

        fn string_optional<'this>(&'this self) -> Self::Field5ScalarGetterType<'this> {
            self.string_optional_opt()
                .unwrap_or(self.string_optional_default_value())
        }
        fn string_optional_default_value(&self) -> Self::Field5ScalarGetterType<'_>;

        fn has_string_optional<'this>(&'this self) -> bool {
            self.string_optional_opt().is_some()
        }
        fn string_optional_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field5ScalarGetterType<'this>> {
            ::std::option::Option::None
        }

        type Field6ScalarGetterType<'this>: ::std::convert::AsRef<str>
        where
            Self: 'this;

        type Field6RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field6ScalarGetterType<'this>>
        where
            Self: 'this;
        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this>;

        type Field7ScalarGetterType<'this>: self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_traits::SubmsgTrait
            where Self: 'this;

        fn submsg_optional<'this>(&'this self) -> Self::Field7ScalarGetterType<'this> {
            self.submsg_optional_opt()
                .unwrap_or(self.submsg_optional_default_value())
        }
        fn submsg_optional_default_value(&self) -> Self::Field7ScalarGetterType<'_>;

        fn has_submsg_optional<'this>(&'this self) -> bool {
            self.submsg_optional_opt().is_some()
        }
        fn submsg_optional_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field7ScalarGetterType<'this>> {
            ::std::option::Option::None
        }

        type Field8ScalarGetterType<'this>: self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_traits::SubmsgTrait
            where Self: 'this;

        type Field8RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field8ScalarGetterType<'this>>
        where
            Self: 'this;
        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this>;

        fn enum_optional<'this>(&'this self) -> self::_puroro_root::ser_tests2::Enum {
            self.enum_optional_opt()
                .unwrap_or(::std::default::Default::default())
        }

        fn has_enum_optional<'this>(&'this self) -> bool {
            self.enum_optional_opt().is_some()
        }
        fn enum_optional_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::ser_tests2::Enum> {
            ::std::option::Option::None
        }

        type Field10RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = self::_puroro_root::ser_tests2::Enum>
        where
            Self: 'this;
        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this>;

        fn very_large_field_number<'this>(&'this self) -> i32 {
            self.very_large_field_number_opt()
                .unwrap_or(::std::default::Default::default())
        }

        fn has_very_large_field_number<'this>(&'this self) -> bool {
            self.very_large_field_number_opt().is_some()
        }
        fn very_large_field_number_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::None
        }
    }

    macro_rules! msg_delegate {
        ($ty:ty) => {
            fn i32_optional_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).i32_optional_opt()
            }

            type Field2RepeatedType<'this>
            where
                Self: 'this,
            = <$ty as MsgTrait>::Field2RepeatedType<'this>;
            fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
                (**self).i32_repeated()
            }
            fn float_optional_opt<'this>(&'this self) -> ::std::option::Option<f32> {
                (**self).float_optional_opt()
            }

            type Field4RepeatedType<'this>
            where
                Self: 'this,
            = <$ty as MsgTrait>::Field4RepeatedType<'this>;
            fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
                (**self).float_repeated()
            }
            type Field5ScalarGetterType<'this>
            where
                Self: 'this,
            = <$ty as MsgTrait>::Field5ScalarGetterType<'this>;
            fn string_optional_opt<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field5ScalarGetterType<'this>> {
                (**self).string_optional_opt()
            }
            fn string_optional_default_value(
                &self,
            ) -> <$ty as MsgTrait>::Field5ScalarGetterType<'_> {
                <$ty as MsgTrait>::string_optional_default_value(self)
            }
            type Field6ScalarGetterType<'this>
            where
                Self: 'this,
            = <$ty as MsgTrait>::Field6ScalarGetterType<'this>;

            type Field6RepeatedType<'this>
            where
                Self: 'this,
            = <$ty as MsgTrait>::Field6RepeatedType<'this>;
            fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
                (**self).string_repeated()
            }
            type Field7ScalarGetterType<'this>
            where
                Self: 'this,
            = <$ty as MsgTrait>::Field7ScalarGetterType<'this>;
            fn submsg_optional_opt<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field7ScalarGetterType<'this>> {
                (**self).submsg_optional_opt()
            }
            fn submsg_optional_default_value(
                &self,
            ) -> <$ty as MsgTrait>::Field7ScalarGetterType<'_> {
                <$ty as MsgTrait>::submsg_optional_default_value(self)
            }
            type Field8ScalarGetterType<'this>
            where
                Self: 'this,
            = <$ty as MsgTrait>::Field8ScalarGetterType<'this>;

            type Field8RepeatedType<'this>
            where
                Self: 'this,
            = <$ty as MsgTrait>::Field8RepeatedType<'this>;
            fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
                (**self).submsg_repeated()
            }
            fn enum_optional_opt<'this>(
                &'this self,
            ) -> ::std::option::Option<self::_puroro_root::ser_tests2::Enum> {
                (**self).enum_optional_opt()
            }

            type Field10RepeatedType<'this>
            where
                Self: 'this,
            = <$ty as MsgTrait>::Field10RepeatedType<'this>;
            fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
                (**self).enum_repeated()
            }
            fn very_large_field_number_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).very_large_field_number_opt()
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
        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5ScalarGetterType<'this> = &'this str;
        fn string_optional_default_value(&self) -> Self::Field5ScalarGetterType<'_> {
            ::std::default::Default::default()
        }
        type Field6ScalarGetterType<'this> = &'this str;
        type Field6RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6ScalarGetterType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7ScalarGetterType<'this> = ();
        fn submsg_optional_default_value(&self) -> Self::Field7ScalarGetterType<'_> {
            ::std::default::Default::default()
        }
        type Field8ScalarGetterType<'this> = ();
        type Field8RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field8ScalarGetterType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::ser_tests2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }
    impl<T> MsgTrait for ::std::option::Option<T>
    where
        T: MsgTrait,
    {
        fn i32_optional_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().and_then(|msg| msg.i32_optional_opt())
        }
        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<::std::option::IntoIter<T::Field2RepeatedType<'this>>>;
        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.i32_repeated())
                .into_iter()
                .flatten()
        }
        fn float_optional_opt<'this>(&'this self) -> ::std::option::Option<f32> {
            self.as_ref().and_then(|msg| msg.float_optional_opt())
        }
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<::std::option::IntoIter<T::Field4RepeatedType<'this>>>;
        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.float_repeated())
                .into_iter()
                .flatten()
        }
        type Field5ScalarGetterType<'this>
        where
            Self: 'this,
        = ::puroro::Either<T::Field5ScalarGetterType<'this>, &'this str>;
        fn string_optional_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field5ScalarGetterType<'this>> {
            self.as_ref().and_then(|msg| {
                msg.string_optional_opt()
                    .map(|val| ::puroro::Either::Left(val))
            })
        }
        fn string_optional_default_value(&self) -> Self::Field5ScalarGetterType<'_> {
            todo!()
        }
        type Field6ScalarGetterType<'this>
        where
            Self: 'this,
        = ::puroro::Either<T::Field6ScalarGetterType<'this>, &'this str>;
        type Field6RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::option::EitherLeftRepeatedField<
            ::std::iter::Flatten<::std::option::IntoIter<T::Field6RepeatedType<'this>>>,
            &'this str,
        >;
        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::option::EitherLeftRepeatedField::new(
                self.as_ref()
                    .map(|msg| msg.string_repeated())
                    .into_iter()
                    .flatten(),
            )
        }
        type Field7ScalarGetterType<'this>
        where
            Self: 'this,
        = T::Field7ScalarGetterType<'this>;
        fn submsg_optional_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field7ScalarGetterType<'this>> {
            self.as_ref().and_then(|msg| msg.submsg_optional_opt())
        }
        fn submsg_optional_default_value(&self) -> Self::Field7ScalarGetterType<'_> {
            todo!()
        }
        type Field8ScalarGetterType<'this>
        where
            Self: 'this,
        = T::Field8ScalarGetterType<'this>;
        type Field8RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<::std::option::IntoIter<T::Field8RepeatedType<'this>>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.submsg_repeated())
                .into_iter()
                .flatten()
        }
        fn enum_optional_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::ser_tests2::Enum> {
            self.as_ref().and_then(|msg| msg.enum_optional_opt())
        }
        type Field10RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<::std::option::IntoIter<T::Field10RepeatedType<'this>>>;
        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.enum_repeated())
                .into_iter()
                .flatten()
        }
        fn very_large_field_number_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref()
                .and_then(|msg| msg.very_large_field_number_opt())
        }
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
                i32_optional: ::std::option::Option<i32>,
            }
            impl ::puroro::Message<Submsg> for Submsg {}

            impl Submsg {
                pub fn new() -> Self {
                    Self {
                        i32_optional: ::std::default::Default::default(),
                    }
                }
                pub fn i32_optional_mut(&mut self) -> &mut ::std::option::Option<i32> {
                    &mut self.i32_optional
                }
            }

            impl super::_puroro_traits::SubmsgTrait for Submsg {
                fn i32_optional_opt<'this>(&'this self) -> Option<i32> {
                    Clone::clone(&self.i32_optional)
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
                            ::puroro::tags::Optional,
                            ::puroro::tags::Int32,
                        >::deser_field(&mut self.i32_optional, data),

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
                        ::puroro::tags::Optional,
                        ::puroro::tags::Int32,
                    >::ser_field(
                        <Self as super::_puroro_traits::SubmsgTrait>::i32_optional_opt(self),
                        1,
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
                            "i32_optional",
                            &<Self as super::_puroro_traits::SubmsgTrait>::i32_optional_opt(self),
                        )
                        .finish()
                }
            }

            impl ::std::clone::Clone for Submsg {
                fn clone(&self) -> Self {
                    Self {
                        i32_optional: ::std::clone::Clone::clone(&self.i32_optional),
                    }
                }
            }

            impl ::std::cmp::PartialEq for Submsg {
                fn eq(&self, rhs: &Self) -> bool {
                    self.i32_optional == rhs.i32_optional && true
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
            }

            macro_rules! submsg_delegate {
                ($ty:ty) => {
                    fn i32_optional_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                        (**self).i32_optional_opt()
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
            impl<T> SubmsgTrait for ::std::option::Option<T>
            where
                T: SubmsgTrait,
            {
                fn i32_optional_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                    self.as_ref().and_then(|msg| msg.i32_optional_opt())
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
