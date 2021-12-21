// A generated source code by puroro library
// package proto3_defaults

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub use _puroro_simple_impl::Msg;
pub use _puroro_simple_impl::Submsg;
pub mod _puroro_simple_impl {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
    pub struct Msg {
        i32_unlabeled: i32,
        i32_optional: ::std::option::Option<i32>,
        i32_repeated: ::std::vec::Vec<i32>,
        f32_unlabeled: f32,
        string_unlabeled: ::std::string::String,
        submsg_unlabeled: ::std::option::Option<
            ::std::boxed::Box<self::_puroro_root::proto3_defaults::_puroro_simple_impl::Submsg>,
        >,
    }
    impl ::puroro::Message<Msg> for Msg {}

    impl Msg {
        pub fn new() -> Self {
            Self {
                i32_unlabeled: ::std::default::Default::default(),
                i32_optional: ::std::default::Default::default(),
                i32_repeated: ::std::default::Default::default(),
                f32_unlabeled: ::std::default::Default::default(),
                string_unlabeled: ::std::default::Default::default(),
                submsg_unlabeled: ::std::default::Default::default(),
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
        pub fn f32_unlabeled_mut(&mut self) -> &mut f32 {
            &mut self.f32_unlabeled
        }
        pub fn string_unlabeled_mut(&mut self) -> &mut ::std::string::String {
            &mut self.string_unlabeled
        }
        pub fn submsg_unlabeled_mut(
            &mut self,
        ) -> &mut ::std::option::Option<
            ::std::boxed::Box<self::_puroro_root::proto3_defaults::_puroro_simple_impl::Submsg>,
        > {
            &mut self.submsg_unlabeled
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
        fn f32_unlabeled_opt<'this>(&'this self) -> Option<f32> {
            if self.f32_unlabeled == ::std::default::Default::default() {
                ::std::option::Option::None
            } else {
                ::std::option::Option::Some(self.f32_unlabeled.clone())
            }
        }
        type Field5ScalarGetterType<'this>
        where
            Self: 'this,
        = &'this ::std::string::String;
        fn string_unlabeled_opt<'this>(&'this self) -> Option<Self::Field5ScalarGetterType<'this>> {
            if self.string_unlabeled.is_empty() {
                ::std::option::Option::None
            } else {
                ::std::option::Option::Some(&self.string_unlabeled)
            }
        }
        fn string_unlabeled_default_value(&self) -> Self::Field5ScalarGetterType<'_> {
            static DEFAULT_VALUE: ::puroro::once_cell::sync::Lazy<::std::string::String> =
                ::puroro::once_cell::sync::Lazy::new(|| {
                    ::std::convert::From::<&str>::from(::std::default::Default::default())
                });

            &DEFAULT_VALUE
        }
        type Field6ScalarGetterType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::proto3_defaults::_puroro_simple_impl::Submsg;
        fn submsg_unlabeled_opt<'this>(&'this self) -> Option<Self::Field6ScalarGetterType<'this>> {
            self.submsg_unlabeled.as_ref().map(|v| v.as_ref())
        }
        fn submsg_unlabeled_default_value(&self) -> Self::Field6ScalarGetterType<'_> {
            static DEFAULT_VALUE: ::puroro::once_cell::sync::Lazy<
                self::_puroro_root::proto3_defaults::_puroro_simple_impl::Submsg,
            > = ::puroro::once_cell::sync::Lazy::new(|| ::std::default::Default::default());

            &DEFAULT_VALUE
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
            4 => DeserFieldFromBytesIter::<
                ::puroro::tags::Unlabeled, ::puroro::tags::Float
            >::deser_field(&mut self.f32_unlabeled, data),
            5 => DeserFieldFromBytesIter::<
                ::puroro::tags::Unlabeled, ::puroro::tags::String
            >::deser_field(&mut self.string_unlabeled, data),
            6 => DeserFieldFromBytesIter::<
                ::puroro::tags::Unlabeled, ::puroro::tags::Message<::std::boxed::Box<self::_puroro_root::proto3_defaults::_puroro_simple_impl::Submsg>>
            >::deser_field(&mut self.submsg_unlabeled, data),

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
                <Self as super::_puroro_traits::MsgTrait>::f32_unlabeled_opt(self),
                4,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::string_unlabeled_opt(self),
                5,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Message<
                    <Self as super::_puroro_traits::MsgTrait>::Field6ScalarGetterType<'_>,
                >,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::submsg_unlabeled_opt(self),
                6,
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
                    "f32_unlabeled",
                    &<Self as super::_puroro_traits::MsgTrait>::f32_unlabeled(self),
                )
                .field(
                    "string_unlabeled",
                    &<Self as super::_puroro_traits::MsgTrait>::string_unlabeled(self),
                )
                .field(
                    "submsg_unlabeled",
                    &<Self as super::_puroro_traits::MsgTrait>::submsg_unlabeled(self),
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
                f32_unlabeled: ::std::clone::Clone::clone(&self.f32_unlabeled),
                string_unlabeled: ::std::clone::Clone::clone(&self.string_unlabeled),
                submsg_unlabeled: ::std::clone::Clone::clone(&self.submsg_unlabeled),
            }
        }
    }

    impl ::std::cmp::PartialEq for Msg {
        fn eq(&self, rhs: &Self) -> bool {
            self.i32_unlabeled == rhs.i32_unlabeled
                && self.i32_optional == rhs.i32_optional
                && self.i32_repeated == rhs.i32_repeated
                && self.f32_unlabeled == rhs.f32_unlabeled
                && self.string_unlabeled == rhs.string_unlabeled
                && self.submsg_unlabeled == rhs.submsg_unlabeled
                && true
        }
    }
    pub struct Submsg {
        i32_unlabeled: i32,
    }
    impl ::puroro::Message<Submsg> for Submsg {}

    impl Submsg {
        pub fn new() -> Self {
            Self {
                i32_unlabeled: ::std::default::Default::default(),
            }
        }
        pub fn i32_unlabeled_mut(&mut self) -> &mut i32 {
            &mut self.i32_unlabeled
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
                ::puroro::tags::Unlabeled, ::puroro::tags::Int32
            >::deser_field(&mut self.i32_unlabeled, data),

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
                .finish()
        }
    }

    impl ::std::clone::Clone for Submsg {
        fn clone(&self) -> Self {
            Self {
                i32_unlabeled: ::std::clone::Clone::clone(&self.i32_unlabeled),
            }
        }
    }

    impl ::std::cmp::PartialEq for Submsg {
        fn eq(&self, rhs: &Self) -> bool {
            self.i32_unlabeled == rhs.i32_unlabeled && true
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
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            self.i32_unlabeled_opt()
                .unwrap_or(::std::default::Default::default())
        }

        fn has_i32_unlabeled<'this>(&'this self) -> bool {
            self.i32_unlabeled_opt().is_some()
        }
        fn i32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i32> {
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

        fn f32_unlabeled<'this>(&'this self) -> f32 {
            self.f32_unlabeled_opt()
                .unwrap_or(::std::default::Default::default())
        }

        fn has_f32_unlabeled<'this>(&'this self) -> bool {
            self.f32_unlabeled_opt().is_some()
        }
        fn f32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<f32> {
            ::std::option::Option::None
        }

        type Field5ScalarGetterType<'this>: ::std::convert::AsRef<str>
        where
            Self: 'this;

        fn string_unlabeled<'this>(&'this self) -> Self::Field5ScalarGetterType<'this> {
            self.string_unlabeled_opt()
                .unwrap_or(self.string_unlabeled_default_value())
        }
        fn string_unlabeled_default_value(&self) -> Self::Field5ScalarGetterType<'_>;

        fn has_string_unlabeled<'this>(&'this self) -> bool {
            self.string_unlabeled_opt().is_some()
        }
        fn string_unlabeled_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field5ScalarGetterType<'this>> {
            ::std::option::Option::None
        }

        type Field6ScalarGetterType<'this>: self::_puroro_root::proto3_defaults::_puroro_traits::SubmsgTrait
            where Self: 'this;

        fn submsg_unlabeled<'this>(&'this self) -> Self::Field6ScalarGetterType<'this> {
            self.submsg_unlabeled_opt()
                .unwrap_or(self.submsg_unlabeled_default_value())
        }
        fn submsg_unlabeled_default_value(&self) -> Self::Field6ScalarGetterType<'_>;

        fn has_submsg_unlabeled<'this>(&'this self) -> bool {
            self.submsg_unlabeled_opt().is_some()
        }
        fn submsg_unlabeled_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field6ScalarGetterType<'this>> {
            ::std::option::Option::None
        }
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
            = <$ty as MsgTrait>::Field3RepeatedType<'this>;
            fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
                (**self).i32_repeated()
            }
            fn f32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<f32> {
                (**self).f32_unlabeled_opt()
            }
            type Field5ScalarGetterType<'this>
            where
                Self: 'this,
            = <$ty as MsgTrait>::Field5ScalarGetterType<'this>;
            fn string_unlabeled_opt<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field5ScalarGetterType<'this>> {
                (**self).string_unlabeled_opt()
            }
            fn string_unlabeled_default_value(
                &self,
            ) -> <$ty as MsgTrait>::Field5ScalarGetterType<'_> {
                <$ty as MsgTrait>::string_unlabeled_default_value(self)
            }
            type Field6ScalarGetterType<'this>
            where
                Self: 'this,
            = <$ty as MsgTrait>::Field6ScalarGetterType<'this>;
            fn submsg_unlabeled_opt<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field6ScalarGetterType<'this>> {
                (**self).submsg_unlabeled_opt()
            }
            fn submsg_unlabeled_default_value(
                &self,
            ) -> <$ty as MsgTrait>::Field6ScalarGetterType<'_> {
                <$ty as MsgTrait>::submsg_unlabeled_default_value(self)
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
        type Field5ScalarGetterType<'this> = &'this str;
        fn string_unlabeled_default_value(&self) -> Self::Field5ScalarGetterType<'_> {
            ::std::default::Default::default()
        }
        type Field6ScalarGetterType<'this> = ();
        fn submsg_unlabeled_default_value(&self) -> Self::Field6ScalarGetterType<'_> {
            ::std::default::Default::default()
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
        fn f32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<f32> {
            self.as_ref().and_then(|msg| msg.f32_unlabeled_opt())
        }
        type Field5ScalarGetterType<'this>
        where
            Self: 'this,
        = ::puroro::Either<T::Field5ScalarGetterType<'this>, &'this str>;
        fn string_unlabeled_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field5ScalarGetterType<'this>> {
            self.as_ref().and_then(|msg| {
                msg.string_unlabeled_opt()
                    .map(|val| ::puroro::Either::Left(val))
            })
        }
        fn string_unlabeled_default_value(&self) -> Self::Field5ScalarGetterType<'_> {
            todo!()
        }
        type Field6ScalarGetterType<'this>
        where
            Self: 'this,
        = T::Field6ScalarGetterType<'this>;
        fn submsg_unlabeled_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field6ScalarGetterType<'this>> {
            self.as_ref().and_then(|msg| msg.submsg_unlabeled_opt())
        }
        fn submsg_unlabeled_default_value(&self) -> Self::Field6ScalarGetterType<'_> {
            todo!()
        }
    }

    pub trait SubmsgTrait {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            self.i32_unlabeled_opt()
                .unwrap_or(::std::default::Default::default())
        }

        fn has_i32_unlabeled<'this>(&'this self) -> bool {
            self.i32_unlabeled_opt().is_some()
        }
        fn i32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::None
        }
    }

    macro_rules! submsg_delegate {
        ($ty:ty) => {
            fn i32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).i32_unlabeled_opt()
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
        fn i32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().and_then(|msg| msg.i32_unlabeled_opt())
        }
    }
}
pub use _puroro_nested::*;
pub mod _puroro_nested {
    pub mod msg {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }
    }
    pub mod submsg {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }
    }
}
