// A generated source code by puroro library
// package full_coverage3

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub use _puroro_impls::MsgSimple as Msg;
pub mod _puroro_impls {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct MsgSimple {
    pub i32_unlabeled: i32,
    pub i32_optional: ::std::option::Option<i32>,
    pub i32_repeated: ::std::vec::Vec<i32>,
    pub float_unlabeled: f32,
    pub float_optional: ::std::option::Option<f32>,
    pub float_repeated: ::std::vec::Vec<f32>,
    pub string_unlabeled: ::std::string::String,
    pub string_optional: ::std::option::Option<::std::string::String>,
    pub string_repeated: ::std::vec::Vec<::std::string::String>,
    pub enum_unlabeled: self::_puroro_root::full_coverage3::Enum,
    pub enum_optional: ::std::option::Option<self::_puroro_root::full_coverage3::Enum>,
    pub enum_repeated: ::std::vec::Vec<self::_puroro_root::full_coverage3::Enum>,
    pub submsg_unlabeled: ::std::option::Option<::std::boxed::Box<self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgSimple>>,
    pub submsg_optional: ::std::option::Option<::std::boxed::Box<self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgSimple>>,
    pub submsg_repeated: ::std::vec::Vec<self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgSimple>,
}
    impl ::puroro::Message for MsgSimple {}

    impl super::_puroro_traits::MsgTrait for MsgSimple {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            ::std::clone::Clone::clone(&self.i32_unlabeled)
        }
        fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::clone::Clone::clone(&self.i32_optional)
        }
        type Field3RepeatedType<'this> = ::std::iter::Cloned<::std::slice::Iter<'this, i32>>;

        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            self.i32_repeated.iter().cloned()
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            ::std::clone::Clone::clone(&self.float_unlabeled)
        }
        fn float_optional<'this>(&'this self) -> ::std::option::Option<f32> {
            ::std::clone::Clone::clone(&self.float_optional)
        }
        type Field13RepeatedType<'this> = ::std::iter::Cloned<::std::slice::Iter<'this, f32>>;

        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            self.float_repeated.iter().cloned()
        }
        type Field21ScalarGetterType<'this> = &'this str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field21ScalarGetterType<'this> {
            &self.string_unlabeled
        }
        type Field22ScalarGetterType<'this> = &'this str;
        fn string_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field22ScalarGetterType<'this>> {
            self.string_optional.as_ref().map(|v| v.as_ref())
        }
        type Field23ScalarGetterType<'this> = &'this str;
        type Field23RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            str,
            ::std::slice::Iter<'this, ::std::string::String>,
        >;

        fn string_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.string_repeated.iter())
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::full_coverage3::Enum {
            ::std::clone::Clone::clone(&self.enum_unlabeled)
        }
        fn enum_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::full_coverage3::Enum> {
            ::std::clone::Clone::clone(&self.enum_optional)
        }
        type Field33RepeatedType<'this> = ::std::iter::Cloned<
            ::std::slice::Iter<'this, self::_puroro_root::full_coverage3::Enum>,
        >;

        fn enum_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            self.enum_repeated.iter().cloned()
        }
        type Field41MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgSimple;
        type Field41ScalarGetterType<'this> = &'this Self::Field41MessageType<'this>;
        fn submsg_unlabeled<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field41ScalarGetterType<'this>> {
            self.submsg_unlabeled.as_ref().map(|v| v.as_ref())
        }
        type Field42MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgSimple;
        type Field42ScalarGetterType<'this> = &'this Self::Field42MessageType<'this>;
        fn submsg_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field42ScalarGetterType<'this>> {
            self.submsg_optional.as_ref().map(|v| v.as_ref())
        }
        type Field43MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgSimple;
        type Field43ScalarGetterType<'this> = &'this Self::Field43MessageType<'this>;
        type Field43RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
    self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgSimple,
    ::std::slice::Iter<'this, self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgSimple>>;

        fn submsg_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.submsg_repeated.iter())
        }
    }

    impl ::puroro::DeserFromBytesIter for MsgSimple {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for MsgSimple {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            match field_number {
            1 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Unlabeled, ::puroro::tags::Int32
            >::deser_field(&mut self.i32_unlabeled, data),
            2 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Int32
            >::deser_field(&mut self.i32_optional, data),
            3 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Int32
            >::deser_field(&mut self.i32_repeated, data),
            11 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Unlabeled, ::puroro::tags::Float
            >::deser_field(&mut self.float_unlabeled, data),
            12 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Float
            >::deser_field(&mut self.float_optional, data),
            13 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Float
            >::deser_field(&mut self.float_repeated, data),
            21 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Unlabeled, ::puroro::tags::String
            >::deser_field(&mut self.string_unlabeled, data),
            22 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.string_optional, data),
            23 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::String
            >::deser_field(&mut self.string_repeated, data),
            31 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Unlabeled, ::puroro::tags::Enum3<self::_puroro_root::full_coverage3::Enum>
            >::deser_field(&mut self.enum_unlabeled, data),
            32 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Enum3<self::_puroro_root::full_coverage3::Enum>
            >::deser_field(&mut self.enum_optional, data),
            33 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Enum3<self::_puroro_root::full_coverage3::Enum>
            >::deser_field(&mut self.enum_repeated, data),
            41 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Unlabeled, ::puroro::tags::Message<self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgSimple>
            >::deser_field(&mut self.submsg_unlabeled, data),
            42 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Message<self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgSimple>
            >::deser_field(&mut self.submsg_optional, data),
            43 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgSimple>
            >::deser_field(&mut self.submsg_repeated, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::SerToIoWrite for MsgSimple {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Int32,
            >::ser_field(&self.i32_unlabeled, 1, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >::ser_field(&self.i32_optional, 2, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Int32,
            >::ser_field(&self.i32_repeated, 3, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Float,
            >::ser_field(&self.float_unlabeled, 11, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Float,
            >::ser_field(&self.float_optional, 12, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Float,
            >::ser_field(&self.float_repeated, 13, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::String,
            >::ser_field(&self.string_unlabeled, 21, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.string_optional, 22, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::String,
            >::ser_field(&self.string_repeated, 23, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Enum3<self::_puroro_root::full_coverage3::Enum>,
            >::ser_field(&self.enum_unlabeled, 31, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Enum3<self::_puroro_root::full_coverage3::Enum>,
            >::ser_field(&self.enum_optional, 32, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Enum3<self::_puroro_root::full_coverage3::Enum>,
            >::ser_field(&self.enum_repeated, 33, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
            ::puroro::tags::Unlabeled, ::puroro::tags::Message<self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgSimple>
        >::ser_field(&self.submsg_unlabeled, 41, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
            ::puroro::tags::Optional, ::puroro::tags::Message<self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgSimple>
        >::ser_field(&self.submsg_optional, 42, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgSimple>
        >::ser_field(&self.submsg_repeated, 43, out)?;
            ::std::result::Result::Ok(())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct MsgEmpty;

    impl ::puroro::Message for MsgEmpty {}

    impl super::_puroro_traits::MsgTrait for MsgEmpty {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            ::std::default::Default::default()
        }
        type Field3RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            ::std::default::Default::default()
        }
        type Field13RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21ScalarGetterType<'this> = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field21ScalarGetterType<'this> {
            ""
        }
        type Field22ScalarGetterType<'this> = &'static str;
        type Field23ScalarGetterType<'this> = &'static str;
        type Field23RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            Self::Field23ScalarGetterType<'this>,
        >;
        fn string_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::full_coverage3::Enum {
            ::std::default::Default::default()
        }
        type Field33RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field41MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field41ScalarGetterType<'this> = &'static Self::Field41MessageType<'this>;
        type Field42MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field42ScalarGetterType<'this> = &'static Self::Field42MessageType<'this>;
        type Field43MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field43ScalarGetterType<'this> = &'static Self::Field43MessageType<'this>;
        type Field43RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            Self::Field43ScalarGetterType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerToIoWrite for MsgEmpty {
        fn ser<W>(&self, _out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::std::result::Result::Ok(())
        }
    }
    impl<T, U> super::_puroro_traits::MsgTrait for ::puroro::Either<T, U>
    where
        T: ::std::ops::Deref,
        U: ::std::ops::Deref,
        T::Target: super::_puroro_traits::MsgTrait,
        U::Target: super::_puroro_traits::MsgTrait,
    {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            self.as_ref().either(
                |t| <T::Target as super::_puroro_traits::MsgTrait>::i32_unlabeled(t),
                |u| <U::Target as super::_puroro_traits::MsgTrait>::i32_unlabeled(u),
            )
        }
        fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().either(
                |t| <T::Target as super::_puroro_traits::MsgTrait>::i32_optional(t),
                |u| <U::Target as super::_puroro_traits::MsgTrait>::i32_optional(u),
            )
        }
        type Field3RepeatedType<'this> = ::puroro_internal::impls::either::EitherRepeatedField<
            <T::Target as super::_puroro_traits::MsgTrait>::Field3RepeatedType<'this>,
            <U::Target as super::_puroro_traits::MsgTrait>::Field3RepeatedType<'this>,
        >;

        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T::Target as super::_puroro_traits::MsgTrait>::i32_repeated(t))
                    .map_right(|u| <U::Target as super::_puroro_traits::MsgTrait>::i32_repeated(u)),
            )
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            self.as_ref().either(
                |t| <T::Target as super::_puroro_traits::MsgTrait>::float_unlabeled(t),
                |u| <U::Target as super::_puroro_traits::MsgTrait>::float_unlabeled(u),
            )
        }
        fn float_optional<'this>(&'this self) -> ::std::option::Option<f32> {
            self.as_ref().either(
                |t| <T::Target as super::_puroro_traits::MsgTrait>::float_optional(t),
                |u| <U::Target as super::_puroro_traits::MsgTrait>::float_optional(u),
            )
        }
        type Field13RepeatedType<'this> = ::puroro_internal::impls::either::EitherRepeatedField<
            <T::Target as super::_puroro_traits::MsgTrait>::Field13RepeatedType<'this>,
            <U::Target as super::_puroro_traits::MsgTrait>::Field13RepeatedType<'this>,
        >;

        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T::Target as super::_puroro_traits::MsgTrait>::float_repeated(t))
                    .map_right(|u| {
                        <U::Target as super::_puroro_traits::MsgTrait>::float_repeated(u)
                    }),
            )
        }
        type Field21ScalarGetterType<'this> = ::puroro::Either<
            <T::Target as super::_puroro_traits::MsgTrait>::Field21ScalarGetterType<'this>,
            <U::Target as super::_puroro_traits::MsgTrait>::Field21ScalarGetterType<'this>,
        >;
        fn string_unlabeled<'this>(&'this self) -> Self::Field21ScalarGetterType<'this> {
            todo!()
        }
        type Field22ScalarGetterType<'this> = ::puroro::Either<
            <T::Target as super::_puroro_traits::MsgTrait>::Field22ScalarGetterType<'this>,
            <U::Target as super::_puroro_traits::MsgTrait>::Field22ScalarGetterType<'this>,
        >;
        fn string_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field22ScalarGetterType<'this>> {
            todo!()
        }
        type Field23ScalarGetterType<'this> = ::puroro::Either<
            <T::Target as super::_puroro_traits::MsgTrait>::Field23ScalarGetterType<'this>,
            <U::Target as super::_puroro_traits::MsgTrait>::Field23ScalarGetterType<'this>,
        >;
        type Field23RepeatedType<'this> = ::puroro_internal::impls::either::EitherRepeatedLDField<
            <T::Target as super::_puroro_traits::MsgTrait>::Field23RepeatedType<'this>,
            <U::Target as super::_puroro_traits::MsgTrait>::Field23RepeatedType<'this>,
        >;

        fn string_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedLDField::new(
                self.as_ref()
                    .map_left(|t| {
                        <T::Target as super::_puroro_traits::MsgTrait>::string_repeated(t)
                    })
                    .map_right(|u| {
                        <U::Target as super::_puroro_traits::MsgTrait>::string_repeated(u)
                    }),
            )
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::full_coverage3::Enum {
            self.as_ref().either(
                |t| <T::Target as super::_puroro_traits::MsgTrait>::enum_unlabeled(t),
                |u| <U::Target as super::_puroro_traits::MsgTrait>::enum_unlabeled(u),
            )
        }
        fn enum_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::full_coverage3::Enum> {
            self.as_ref().either(
                |t| <T::Target as super::_puroro_traits::MsgTrait>::enum_optional(t),
                |u| <U::Target as super::_puroro_traits::MsgTrait>::enum_optional(u),
            )
        }
        type Field33RepeatedType<'this> = ::puroro_internal::impls::either::EitherRepeatedField<
            <T::Target as super::_puroro_traits::MsgTrait>::Field33RepeatedType<'this>,
            <U::Target as super::_puroro_traits::MsgTrait>::Field33RepeatedType<'this>,
        >;

        fn enum_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T::Target as super::_puroro_traits::MsgTrait>::enum_repeated(t))
                    .map_right(|u| {
                        <U::Target as super::_puroro_traits::MsgTrait>::enum_repeated(u)
                    }),
            )
        }
        type Field41MessageType<'this> = ::puroro::Either<
            <T::Target as super::_puroro_traits::MsgTrait>::Field41ScalarGetterType<'this>,
            <U::Target as super::_puroro_traits::MsgTrait>::Field41ScalarGetterType<'this>,
        >;
        type Field41ScalarGetterType<'this> =
            ::puroro_internal::Derefable<Self::Field41MessageType<'this>>;
        fn submsg_unlabeled<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field41ScalarGetterType<'this>> {
            todo!()
        }
        type Field42MessageType<'this> = ::puroro::Either<
            <T::Target as super::_puroro_traits::MsgTrait>::Field42ScalarGetterType<'this>,
            <U::Target as super::_puroro_traits::MsgTrait>::Field42ScalarGetterType<'this>,
        >;
        type Field42ScalarGetterType<'this> =
            ::puroro_internal::Derefable<Self::Field42MessageType<'this>>;
        fn submsg_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field42ScalarGetterType<'this>> {
            todo!()
        }
        type Field43MessageType<'this> = ::puroro::Either<
            <T::Target as super::_puroro_traits::MsgTrait>::Field43ScalarGetterType<'this>,
            <U::Target as super::_puroro_traits::MsgTrait>::Field43ScalarGetterType<'this>,
        >;
        type Field43ScalarGetterType<'this> =
            ::puroro_internal::Derefable<Self::Field43MessageType<'this>>;
        type Field43RepeatedType<'this> =
            ::puroro_internal::impls::either::EitherRepeatedMessageField<
                <T::Target as super::_puroro_traits::MsgTrait>::Field43RepeatedType<'this>,
                <U::Target as super::_puroro_traits::MsgTrait>::Field43RepeatedType<'this>,
            >;

        fn submsg_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| {
                        <T::Target as super::_puroro_traits::MsgTrait>::submsg_repeated(t)
                    })
                    .map_right(|u| {
                        <U::Target as super::_puroro_traits::MsgTrait>::submsg_repeated(u)
                    }),
            )
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField1 {
        i32_unlabeled: i32,
    }

    impl ::puroro::Message for MsgSimpleField1 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField1 {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            ::std::clone::Clone::clone(&self.i32_unlabeled)
        }
        type Field3RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            ::std::default::Default::default()
        }
        type Field13RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21ScalarGetterType<'this> = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field21ScalarGetterType<'this> {
            ""
        }
        type Field22ScalarGetterType<'this> = &'static str;
        type Field23ScalarGetterType<'this> = &'static str;
        type Field23RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            Self::Field23ScalarGetterType<'this>,
        >;
        fn string_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::full_coverage3::Enum {
            ::std::default::Default::default()
        }
        type Field33RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field41MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field41ScalarGetterType<'this> = &'static Self::Field41MessageType<'this>;
        type Field42MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field42ScalarGetterType<'this> = &'static Self::Field42MessageType<'this>;
        type Field43MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field43ScalarGetterType<'this> = &'static Self::Field43MessageType<'this>;
        type Field43RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            Self::Field43ScalarGetterType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField2 {
        i32_optional: ::std::option::Option<i32>,
    }

    impl ::puroro::Message for MsgSimpleField2 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField2 {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            ::std::default::Default::default()
        }
        fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::clone::Clone::clone(&self.i32_optional)
        }
        type Field3RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            ::std::default::Default::default()
        }
        type Field13RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21ScalarGetterType<'this> = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field21ScalarGetterType<'this> {
            ""
        }
        type Field22ScalarGetterType<'this> = &'static str;
        type Field23ScalarGetterType<'this> = &'static str;
        type Field23RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            Self::Field23ScalarGetterType<'this>,
        >;
        fn string_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::full_coverage3::Enum {
            ::std::default::Default::default()
        }
        type Field33RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field41MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field41ScalarGetterType<'this> = &'static Self::Field41MessageType<'this>;
        type Field42MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field42ScalarGetterType<'this> = &'static Self::Field42MessageType<'this>;
        type Field43MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field43ScalarGetterType<'this> = &'static Self::Field43MessageType<'this>;
        type Field43RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            Self::Field43ScalarGetterType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField3 {
        i32_repeated: ::std::vec::Vec<i32>,
    }

    impl ::puroro::Message for MsgSimpleField3 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField3 {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            ::std::default::Default::default()
        }
        type Field3RepeatedType<'this> = ::std::iter::Cloned<::std::slice::Iter<'this, i32>>;

        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            self.i32_repeated.iter().cloned()
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            ::std::default::Default::default()
        }
        type Field13RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21ScalarGetterType<'this> = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field21ScalarGetterType<'this> {
            ""
        }
        type Field22ScalarGetterType<'this> = &'static str;
        type Field23ScalarGetterType<'this> = &'static str;
        type Field23RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            Self::Field23ScalarGetterType<'this>,
        >;
        fn string_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::full_coverage3::Enum {
            ::std::default::Default::default()
        }
        type Field33RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field41MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field41ScalarGetterType<'this> = &'static Self::Field41MessageType<'this>;
        type Field42MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field42ScalarGetterType<'this> = &'static Self::Field42MessageType<'this>;
        type Field43MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field43ScalarGetterType<'this> = &'static Self::Field43MessageType<'this>;
        type Field43RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            Self::Field43ScalarGetterType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField11 {
        float_unlabeled: f32,
    }

    impl ::puroro::Message for MsgSimpleField11 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField11 {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            ::std::default::Default::default()
        }
        type Field3RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            ::std::clone::Clone::clone(&self.float_unlabeled)
        }
        type Field13RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21ScalarGetterType<'this> = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field21ScalarGetterType<'this> {
            ""
        }
        type Field22ScalarGetterType<'this> = &'static str;
        type Field23ScalarGetterType<'this> = &'static str;
        type Field23RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            Self::Field23ScalarGetterType<'this>,
        >;
        fn string_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::full_coverage3::Enum {
            ::std::default::Default::default()
        }
        type Field33RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field41MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field41ScalarGetterType<'this> = &'static Self::Field41MessageType<'this>;
        type Field42MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field42ScalarGetterType<'this> = &'static Self::Field42MessageType<'this>;
        type Field43MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field43ScalarGetterType<'this> = &'static Self::Field43MessageType<'this>;
        type Field43RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            Self::Field43ScalarGetterType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField12 {
        float_optional: ::std::option::Option<f32>,
    }

    impl ::puroro::Message for MsgSimpleField12 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField12 {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            ::std::default::Default::default()
        }
        type Field3RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            ::std::default::Default::default()
        }
        fn float_optional<'this>(&'this self) -> ::std::option::Option<f32> {
            ::std::clone::Clone::clone(&self.float_optional)
        }
        type Field13RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21ScalarGetterType<'this> = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field21ScalarGetterType<'this> {
            ""
        }
        type Field22ScalarGetterType<'this> = &'static str;
        type Field23ScalarGetterType<'this> = &'static str;
        type Field23RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            Self::Field23ScalarGetterType<'this>,
        >;
        fn string_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::full_coverage3::Enum {
            ::std::default::Default::default()
        }
        type Field33RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field41MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field41ScalarGetterType<'this> = &'static Self::Field41MessageType<'this>;
        type Field42MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field42ScalarGetterType<'this> = &'static Self::Field42MessageType<'this>;
        type Field43MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field43ScalarGetterType<'this> = &'static Self::Field43MessageType<'this>;
        type Field43RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            Self::Field43ScalarGetterType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField13 {
        float_repeated: ::std::vec::Vec<f32>,
    }

    impl ::puroro::Message for MsgSimpleField13 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField13 {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            ::std::default::Default::default()
        }
        type Field3RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            ::std::default::Default::default()
        }
        type Field13RepeatedType<'this> = ::std::iter::Cloned<::std::slice::Iter<'this, f32>>;

        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            self.float_repeated.iter().cloned()
        }
        type Field21ScalarGetterType<'this> = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field21ScalarGetterType<'this> {
            ""
        }
        type Field22ScalarGetterType<'this> = &'static str;
        type Field23ScalarGetterType<'this> = &'static str;
        type Field23RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            Self::Field23ScalarGetterType<'this>,
        >;
        fn string_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::full_coverage3::Enum {
            ::std::default::Default::default()
        }
        type Field33RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field41MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field41ScalarGetterType<'this> = &'static Self::Field41MessageType<'this>;
        type Field42MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field42ScalarGetterType<'this> = &'static Self::Field42MessageType<'this>;
        type Field43MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field43ScalarGetterType<'this> = &'static Self::Field43MessageType<'this>;
        type Field43RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            Self::Field43ScalarGetterType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField21 {
        string_unlabeled: ::std::string::String,
    }

    impl ::puroro::Message for MsgSimpleField21 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField21 {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            ::std::default::Default::default()
        }
        type Field3RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            ::std::default::Default::default()
        }
        type Field13RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21ScalarGetterType<'this> = &'this str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field21ScalarGetterType<'this> {
            &self.string_unlabeled
        }
        type Field22ScalarGetterType<'this> = &'static str;
        type Field23ScalarGetterType<'this> = &'static str;
        type Field23RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            Self::Field23ScalarGetterType<'this>,
        >;
        fn string_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::full_coverage3::Enum {
            ::std::default::Default::default()
        }
        type Field33RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field41MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field41ScalarGetterType<'this> = &'static Self::Field41MessageType<'this>;
        type Field42MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field42ScalarGetterType<'this> = &'static Self::Field42MessageType<'this>;
        type Field43MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field43ScalarGetterType<'this> = &'static Self::Field43MessageType<'this>;
        type Field43RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            Self::Field43ScalarGetterType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField22 {
        string_optional: ::std::option::Option<::std::string::String>,
    }

    impl ::puroro::Message for MsgSimpleField22 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField22 {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            ::std::default::Default::default()
        }
        type Field3RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            ::std::default::Default::default()
        }
        type Field13RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21ScalarGetterType<'this> = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field21ScalarGetterType<'this> {
            ""
        }
        type Field22ScalarGetterType<'this> = &'this str;
        fn string_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field22ScalarGetterType<'this>> {
            self.string_optional.as_ref().map(|v| v.as_ref())
        }
        type Field23ScalarGetterType<'this> = &'static str;
        type Field23RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            Self::Field23ScalarGetterType<'this>,
        >;
        fn string_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::full_coverage3::Enum {
            ::std::default::Default::default()
        }
        type Field33RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field41MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field41ScalarGetterType<'this> = &'static Self::Field41MessageType<'this>;
        type Field42MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field42ScalarGetterType<'this> = &'static Self::Field42MessageType<'this>;
        type Field43MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field43ScalarGetterType<'this> = &'static Self::Field43MessageType<'this>;
        type Field43RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            Self::Field43ScalarGetterType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField23 {
        string_repeated: ::std::vec::Vec<::std::string::String>,
    }

    impl ::puroro::Message for MsgSimpleField23 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField23 {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            ::std::default::Default::default()
        }
        type Field3RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            ::std::default::Default::default()
        }
        type Field13RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21ScalarGetterType<'this> = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field21ScalarGetterType<'this> {
            ""
        }
        type Field22ScalarGetterType<'this> = &'static str;
        type Field23ScalarGetterType<'this> = &'this str;
        type Field23RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            str,
            ::std::slice::Iter<'this, ::std::string::String>,
        >;

        fn string_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.string_repeated.iter())
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::full_coverage3::Enum {
            ::std::default::Default::default()
        }
        type Field33RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field41MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field41ScalarGetterType<'this> = &'static Self::Field41MessageType<'this>;
        type Field42MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field42ScalarGetterType<'this> = &'static Self::Field42MessageType<'this>;
        type Field43MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field43ScalarGetterType<'this> = &'static Self::Field43MessageType<'this>;
        type Field43RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            Self::Field43ScalarGetterType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField31 {
        enum_unlabeled: self::_puroro_root::full_coverage3::Enum,
    }

    impl ::puroro::Message for MsgSimpleField31 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField31 {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            ::std::default::Default::default()
        }
        type Field3RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            ::std::default::Default::default()
        }
        type Field13RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21ScalarGetterType<'this> = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field21ScalarGetterType<'this> {
            ""
        }
        type Field22ScalarGetterType<'this> = &'static str;
        type Field23ScalarGetterType<'this> = &'static str;
        type Field23RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            Self::Field23ScalarGetterType<'this>,
        >;
        fn string_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::full_coverage3::Enum {
            ::std::clone::Clone::clone(&self.enum_unlabeled)
        }
        type Field33RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field41MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field41ScalarGetterType<'this> = &'static Self::Field41MessageType<'this>;
        type Field42MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field42ScalarGetterType<'this> = &'static Self::Field42MessageType<'this>;
        type Field43MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field43ScalarGetterType<'this> = &'static Self::Field43MessageType<'this>;
        type Field43RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            Self::Field43ScalarGetterType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField32 {
        enum_optional: ::std::option::Option<self::_puroro_root::full_coverage3::Enum>,
    }

    impl ::puroro::Message for MsgSimpleField32 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField32 {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            ::std::default::Default::default()
        }
        type Field3RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            ::std::default::Default::default()
        }
        type Field13RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21ScalarGetterType<'this> = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field21ScalarGetterType<'this> {
            ""
        }
        type Field22ScalarGetterType<'this> = &'static str;
        type Field23ScalarGetterType<'this> = &'static str;
        type Field23RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            Self::Field23ScalarGetterType<'this>,
        >;
        fn string_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::full_coverage3::Enum {
            ::std::default::Default::default()
        }
        fn enum_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::full_coverage3::Enum> {
            ::std::clone::Clone::clone(&self.enum_optional)
        }
        type Field33RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field41MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field41ScalarGetterType<'this> = &'static Self::Field41MessageType<'this>;
        type Field42MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field42ScalarGetterType<'this> = &'static Self::Field42MessageType<'this>;
        type Field43MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field43ScalarGetterType<'this> = &'static Self::Field43MessageType<'this>;
        type Field43RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            Self::Field43ScalarGetterType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField33 {
        enum_repeated: ::std::vec::Vec<self::_puroro_root::full_coverage3::Enum>,
    }

    impl ::puroro::Message for MsgSimpleField33 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField33 {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            ::std::default::Default::default()
        }
        type Field3RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            ::std::default::Default::default()
        }
        type Field13RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21ScalarGetterType<'this> = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field21ScalarGetterType<'this> {
            ""
        }
        type Field22ScalarGetterType<'this> = &'static str;
        type Field23ScalarGetterType<'this> = &'static str;
        type Field23RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            Self::Field23ScalarGetterType<'this>,
        >;
        fn string_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::full_coverage3::Enum {
            ::std::default::Default::default()
        }
        type Field33RepeatedType<'this> = ::std::iter::Cloned<
            ::std::slice::Iter<'this, self::_puroro_root::full_coverage3::Enum>,
        >;

        fn enum_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            self.enum_repeated.iter().cloned()
        }
        type Field41MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field41ScalarGetterType<'this> = &'static Self::Field41MessageType<'this>;
        type Field42MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field42ScalarGetterType<'this> = &'static Self::Field42MessageType<'this>;
        type Field43MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field43ScalarGetterType<'this> = &'static Self::Field43MessageType<'this>;
        type Field43RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            Self::Field43ScalarGetterType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField41 {
    submsg_unlabeled: ::std::option::Option<::std::boxed::Box<self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgSimple>>,
}

    impl ::puroro::Message for MsgSimpleField41 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField41 {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            ::std::default::Default::default()
        }
        type Field3RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            ::std::default::Default::default()
        }
        type Field13RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21ScalarGetterType<'this> = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field21ScalarGetterType<'this> {
            ""
        }
        type Field22ScalarGetterType<'this> = &'static str;
        type Field23ScalarGetterType<'this> = &'static str;
        type Field23RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            Self::Field23ScalarGetterType<'this>,
        >;
        fn string_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::full_coverage3::Enum {
            ::std::default::Default::default()
        }
        type Field33RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field41MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgSimple;
        type Field41ScalarGetterType<'this> = &'this Self::Field41MessageType<'this>;
        fn submsg_unlabeled<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field41ScalarGetterType<'this>> {
            self.submsg_unlabeled.as_ref().map(|v| v.as_ref())
        }
        type Field42MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field42ScalarGetterType<'this> = &'static Self::Field42MessageType<'this>;
        type Field43MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field43ScalarGetterType<'this> = &'static Self::Field43MessageType<'this>;
        type Field43RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            Self::Field43ScalarGetterType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField42 {
    submsg_optional: ::std::option::Option<::std::boxed::Box<self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgSimple>>,
}

    impl ::puroro::Message for MsgSimpleField42 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField42 {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            ::std::default::Default::default()
        }
        type Field3RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            ::std::default::Default::default()
        }
        type Field13RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21ScalarGetterType<'this> = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field21ScalarGetterType<'this> {
            ""
        }
        type Field22ScalarGetterType<'this> = &'static str;
        type Field23ScalarGetterType<'this> = &'static str;
        type Field23RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            Self::Field23ScalarGetterType<'this>,
        >;
        fn string_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::full_coverage3::Enum {
            ::std::default::Default::default()
        }
        type Field33RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field41MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field41ScalarGetterType<'this> = &'static Self::Field41MessageType<'this>;
        type Field42MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgSimple;
        type Field42ScalarGetterType<'this> = &'this Self::Field42MessageType<'this>;
        fn submsg_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field42ScalarGetterType<'this>> {
            self.submsg_optional.as_ref().map(|v| v.as_ref())
        }
        type Field43MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field43ScalarGetterType<'this> = &'static Self::Field43MessageType<'this>;
        type Field43RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            Self::Field43ScalarGetterType<'this>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField43 {
        submsg_repeated: ::std::vec::Vec<
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgSimple,
        >,
    }

    impl ::puroro::Message for MsgSimpleField43 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField43 {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            ::std::default::Default::default()
        }
        type Field3RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            ::std::default::Default::default()
        }
        type Field13RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21ScalarGetterType<'this> = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field21ScalarGetterType<'this> {
            ""
        }
        type Field22ScalarGetterType<'this> = &'static str;
        type Field23ScalarGetterType<'this> = &'static str;
        type Field23RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            Self::Field23ScalarGetterType<'this>,
        >;
        fn string_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::full_coverage3::Enum {
            ::std::default::Default::default()
        }
        type Field33RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field41MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field41ScalarGetterType<'this> = &'static Self::Field41MessageType<'this>;
        type Field42MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field42ScalarGetterType<'this> = &'static Self::Field42MessageType<'this>;
        type Field43MessageType<'this> =
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgSimple;
        type Field43ScalarGetterType<'this> = &'this Self::Field43MessageType<'this>;
        type Field43RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
    self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgSimple,
    ::std::slice::Iter<'this, self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgSimple>>;

        fn submsg_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.submsg_repeated.iter())
        }
    }
}
pub use _puroro_traits::*;
pub mod _puroro_traits {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }

    pub trait MsgTrait {
        fn i32_unlabeled<'this>(&'this self) -> i32;
        fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::default::Default::default()
        }
        type Field3RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this>;
        fn float_unlabeled<'this>(&'this self) -> f32;
        fn float_optional<'this>(&'this self) -> ::std::option::Option<f32> {
            ::std::default::Default::default()
        }
        type Field13RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this>;
        type Field21ScalarGetterType<'this>: ::std::ops::Deref<Target = str>;
        fn string_unlabeled<'this>(&'this self) -> Self::Field21ScalarGetterType<'this>;
        type Field22ScalarGetterType<'this>: ::std::ops::Deref<Target = str>;
        fn string_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field22ScalarGetterType<'this>> {
            ::std::default::Default::default()
        }
        type Field23ScalarGetterType<'this>: ::std::ops::Deref<Target = str>;
        type Field23RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field23ScalarGetterType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this>;
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::full_coverage3::Enum;
        fn enum_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::full_coverage3::Enum> {
            ::std::default::Default::default()
        }
        type Field33RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = self::_puroro_root::full_coverage3::Enum>;
        fn enum_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this>;
        type Field41MessageType<'this>: self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait;
        type Field41ScalarGetterType<'this>: ::std::ops::Deref<
            Target = Self::Field41MessageType<'this>,
        >;
        fn submsg_unlabeled<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field41ScalarGetterType<'this>> {
            ::std::default::Default::default()
        }
        type Field42MessageType<'this>: self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait;
        type Field42ScalarGetterType<'this>: ::std::ops::Deref<
            Target = Self::Field42MessageType<'this>,
        >;
        fn submsg_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field42ScalarGetterType<'this>> {
            ::std::default::Default::default()
        }
        type Field43MessageType<'this>: self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait;
        type Field43ScalarGetterType<'this>: ::std::ops::Deref<
            Target = Self::Field43MessageType<'this>,
        >;
        type Field43RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field43ScalarGetterType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this>;
    }
}
#[derive(::std::fmt::Debug, ::std::clone::Clone, ::std::cmp::PartialEq)]
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
pub use _puroro_nested::*;
pub mod _puroro_nested {
    pub mod msg {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }

        pub use _puroro_impls::SubmsgSimple as Submsg;
        pub mod _puroro_impls {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }
            #[derive(
                ::std::clone::Clone,
                ::std::default::Default,
                ::std::cmp::PartialEq,
                ::std::fmt::Debug,
            )]
            pub struct SubmsgSimple {
                pub i32_unlabeled: i32,
            }
            impl ::puroro::Message for SubmsgSimple {}

            impl super::_puroro_traits::SubmsgTrait for SubmsgSimple {
                fn i32_unlabeled<'this>(&'this self) -> i32 {
                    ::std::clone::Clone::clone(&self.i32_unlabeled)
                }
            }

            impl ::puroro::DeserFromBytesIter for SubmsgSimple {
                fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
                }
            }

            impl ::puroro_internal::de::DeserFieldsFromBytesIter for SubmsgSimple {
                fn deser_field<I>(
                    &mut self,
                    field_number: i32,
                    data: ::puroro::types::FieldData<
                        &mut ::puroro_internal::de::from_iter::ScopedIter<I>,
                    >,
                ) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    match field_number {
                        1 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                            ::puroro::tags::Unlabeled,
                            ::puroro::tags::Int32,
                        >::deser_field(&mut self.i32_unlabeled, data),

                        _ => unimplemented!("TODO: This case should be handled properly..."),
                    }
                }
            }

            impl ::puroro::SerToIoWrite for SubmsgSimple {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                        ::puroro::tags::Unlabeled,
                        ::puroro::tags::Int32,
                    >::ser_field(&self.i32_unlabeled, 1, out)?;
                    ::std::result::Result::Ok(())
                }
            }
            #[derive(
                ::std::clone::Clone,
                ::std::default::Default,
                ::std::cmp::PartialEq,
                ::std::fmt::Debug,
            )]
            pub struct SubmsgEmpty;

            impl ::puroro::Message for SubmsgEmpty {}

            impl super::_puroro_traits::SubmsgTrait for SubmsgEmpty {
                fn i32_unlabeled<'this>(&'this self) -> i32 {
                    ::std::default::Default::default()
                }
            }

            impl ::puroro::SerToIoWrite for SubmsgEmpty {
                fn ser<W>(&self, _out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    ::std::result::Result::Ok(())
                }
            }
            impl<T, U> super::_puroro_traits::SubmsgTrait for ::puroro::Either<T, U>
            where
                T: ::std::ops::Deref,
                U: ::std::ops::Deref,
                T::Target: super::_puroro_traits::SubmsgTrait,
                U::Target: super::_puroro_traits::SubmsgTrait,
            {
                fn i32_unlabeled<'this>(&'this self) -> i32 {
                    self.as_ref().either(
                        |t| <T::Target as super::_puroro_traits::SubmsgTrait>::i32_unlabeled(t),
                        |u| <U::Target as super::_puroro_traits::SubmsgTrait>::i32_unlabeled(u),
                    )
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
            struct SubmsgSimpleField1 {
                i32_unlabeled: i32,
            }

            impl ::puroro::Message for SubmsgSimpleField1 {}

            impl super::_puroro_traits::SubmsgTrait for SubmsgSimpleField1 {
                fn i32_unlabeled<'this>(&'this self) -> i32 {
                    ::std::clone::Clone::clone(&self.i32_unlabeled)
                }
            }
        }
        pub use _puroro_traits::*;
        pub mod _puroro_traits {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }

            pub trait SubmsgTrait {
                fn i32_unlabeled<'this>(&'this self) -> i32;
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
