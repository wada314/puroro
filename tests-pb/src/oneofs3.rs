// A generated source code by puroro library
// package oneofs3

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
        group_one: ::std::option::Option<
            super::_puroro_nested::msg::_puroro_private_oneofs::GroupOneSimple,
        >,
        group_two: ::std::option::Option<
            super::_puroro_nested::msg::_puroro_private_oneofs::GroupTwoSimple,
        >,
        group_three: ::std::option::Option<
            super::_puroro_nested::msg::_puroro_private_oneofs::GroupThreeSimple,
        >,
    }
    impl ::puroro::Message<Msg> for Msg {}

    impl Msg {
        pub fn new() -> Self {
            Self {
                group_one: ::std::default::Default::default(),
                group_two: ::std::default::Default::default(),
                group_three: ::std::default::Default::default(),
            }
        }
        pub fn clear_group_one(&mut self) {
            self.group_one = ::std::option::Option::None;
        }
        pub fn g1_int32_mut(&mut self) -> &mut i32 {
            use super::_puroro_nested::msg::_puroro_private_oneofs::GroupOneSimple as E;
            if !matches!(&self.group_one, Some(E::G1Int32(_))) {
                self.group_one = Some(E::G1Int32(::std::default::Default::default()));
            }
            match &mut self.group_one {
                Some(E::G1Int32(v)) => v,
                _ => unreachable!(),
            }
        }
        pub fn g1_string_mut(&mut self) -> &mut ::std::string::String {
            use super::_puroro_nested::msg::_puroro_private_oneofs::GroupOneSimple as E;
            if !matches!(&self.group_one, Some(E::G1String(_))) {
                self.group_one = Some(E::G1String(::std::default::Default::default()));
            }
            match &mut self.group_one {
                Some(E::G1String(v)) => v,
                _ => unreachable!(),
            }
        }
        pub fn clear_group_two(&mut self) {
            self.group_two = ::std::option::Option::None;
        }
        pub fn g2_f32_mut(&mut self) -> &mut f32 {
            use super::_puroro_nested::msg::_puroro_private_oneofs::GroupTwoSimple as E;
            if !matches!(&self.group_two, Some(E::G2F32(_))) {
                self.group_two = Some(E::G2F32(::std::default::Default::default()));
            }
            match &mut self.group_two {
                Some(E::G2F32(v)) => v,
                _ => unreachable!(),
            }
        }
        pub fn g2_string_mut(&mut self) -> &mut ::std::string::String {
            use super::_puroro_nested::msg::_puroro_private_oneofs::GroupTwoSimple as E;
            if !matches!(&self.group_two, Some(E::G2String(_))) {
                self.group_two = Some(E::G2String(::std::default::Default::default()));
            }
            match &mut self.group_two {
                Some(E::G2String(v)) => v,
                _ => unreachable!(),
            }
        }
        pub fn g2_submsg_mut(
            &mut self,
        ) -> &mut ::std::boxed::Box<self::_puroro_root::oneofs3::_puroro_simple_impl::Submsg>
        {
            use super::_puroro_nested::msg::_puroro_private_oneofs::GroupTwoSimple as E;
            if !matches!(&self.group_two, Some(E::G2Submsg(_))) {
                self.group_two = Some(E::G2Submsg(::std::default::Default::default()));
            }
            match &mut self.group_two {
                Some(E::G2Submsg(v)) => v,
                _ => unreachable!(),
            }
        }
        pub fn clear_group_three(&mut self) {
            self.group_three = ::std::option::Option::None;
        }
        pub fn g3_int32_mut(&mut self) -> &mut i32 {
            use super::_puroro_nested::msg::_puroro_private_oneofs::GroupThreeSimple as E;
            if !matches!(&self.group_three, Some(E::G3Int32(_))) {
                self.group_three = Some(E::G3Int32(::std::default::Default::default()));
            }
            match &mut self.group_three {
                Some(E::G3Int32(v)) => v,
                _ => unreachable!(),
            }
        }
    }

    impl super::_puroro_traits::MsgTrait for Msg {
        type Field2ScalarGetterType<'this>
        where
            Self: 'this,
        = &'this ::std::string::String;
        fn g1_string_default_value(&self) -> Self::Field2ScalarGetterType<'_> {
            static DEFAULT_VALUE: ::puroro::once_cell::sync::Lazy<::std::string::String> =
                ::puroro::once_cell::sync::Lazy::new(|| {
                    ::std::convert::From::<&str>::from(::std::default::Default::default())
                });

            &DEFAULT_VALUE
        }
        type Field4ScalarGetterType<'this>
        where
            Self: 'this,
        = &'this ::std::string::String;
        fn g2_string_default_value(&self) -> Self::Field4ScalarGetterType<'_> {
            static DEFAULT_VALUE: ::puroro::once_cell::sync::Lazy<::std::string::String> =
                ::puroro::once_cell::sync::Lazy::new(|| {
                    ::std::convert::From::<&str>::from(::std::default::Default::default())
                });

            &DEFAULT_VALUE
        }
        type Field5ScalarGetterType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::oneofs3::_puroro_simple_impl::Submsg;
        fn g2_submsg_default_value(&self) -> Self::Field5ScalarGetterType<'_> {
            static DEFAULT_VALUE: ::puroro::once_cell::sync::Lazy<
                self::_puroro_root::oneofs3::_puroro_simple_impl::Submsg,
            > = ::puroro::once_cell::sync::Lazy::new(|| ::std::default::Default::default());

            &DEFAULT_VALUE
        }
        fn group_one<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this, Self>> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupOne as F;
            use super::_puroro_nested::msg::_puroro_private_oneofs::GroupOneSimple as E;
            self.group_one.as_ref().map(|oneof| match oneof {
                E::G1Int32(v) => F::G1Int32(v.clone()),
                E::G1String(v) => F::G1String(v),
            })
        }
        fn group_two<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as F;
            use super::_puroro_nested::msg::_puroro_private_oneofs::GroupTwoSimple as E;
            self.group_two.as_ref().map(|oneof| match oneof {
                E::G2F32(v) => F::G2F32(v.clone()),
                E::G2String(v) => F::G2String(v),
                E::G2Submsg(v) => F::G2Submsg(v.as_ref()),
            })
        }
        fn group_three<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupThree as F;
            use super::_puroro_nested::msg::_puroro_private_oneofs::GroupThreeSimple as E;
            self.group_three.as_ref().map(|oneof| match oneof {
                E::G3Int32(v) => F::G3Int32(v.clone()),
            })
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
                    use super::_puroro_nested::msg::_puroro_private_oneofs::GroupOneSimple as E;
                    if !matches!(&self.group_one, Some(E::G1Int32(_))) {
                        self.group_one = Some(E::G1Int32(::std::default::Default::default()));
                    }
                    let field_value_mut_ref = match &mut self.group_one {
                        Some(E::G1Int32(v)) => v,
                        _ => unreachable!(),
                    };
                    DeserFieldFromBytesIter::<
                    ::puroro::tags::OneofField, ::puroro::tags::Int32
                >::deser_field(field_value_mut_ref, data)
                }
                2 => {
                    use super::_puroro_nested::msg::_puroro_private_oneofs::GroupOneSimple as E;
                    if !matches!(&self.group_one, Some(E::G1String(_))) {
                        self.group_one = Some(E::G1String(::std::default::Default::default()));
                    }
                    let field_value_mut_ref = match &mut self.group_one {
                        Some(E::G1String(v)) => v,
                        _ => unreachable!(),
                    };
                    DeserFieldFromBytesIter::<
                    ::puroro::tags::OneofField, ::puroro::tags::String
                >::deser_field(field_value_mut_ref, data)
                }
                3 => {
                    use super::_puroro_nested::msg::_puroro_private_oneofs::GroupTwoSimple as E;
                    if !matches!(&self.group_two, Some(E::G2F32(_))) {
                        self.group_two = Some(E::G2F32(::std::default::Default::default()));
                    }
                    let field_value_mut_ref = match &mut self.group_two {
                        Some(E::G2F32(v)) => v,
                        _ => unreachable!(),
                    };
                    DeserFieldFromBytesIter::<
                    ::puroro::tags::OneofField, ::puroro::tags::Float
                >::deser_field(field_value_mut_ref, data)
                }
                4 => {
                    use super::_puroro_nested::msg::_puroro_private_oneofs::GroupTwoSimple as E;
                    if !matches!(&self.group_two, Some(E::G2String(_))) {
                        self.group_two = Some(E::G2String(::std::default::Default::default()));
                    }
                    let field_value_mut_ref = match &mut self.group_two {
                        Some(E::G2String(v)) => v,
                        _ => unreachable!(),
                    };
                    DeserFieldFromBytesIter::<
                    ::puroro::tags::OneofField, ::puroro::tags::String
                >::deser_field(field_value_mut_ref, data)
                }
                5 => {
                    use super::_puroro_nested::msg::_puroro_private_oneofs::GroupTwoSimple as E;
                    if !matches!(&self.group_two, Some(E::G2Submsg(_))) {
                        self.group_two = Some(E::G2Submsg(::std::default::Default::default()));
                    }
                    let field_value_mut_ref = match &mut self.group_two {
                        Some(E::G2Submsg(v)) => v,
                        _ => unreachable!(),
                    };
                    DeserFieldFromBytesIter::<
                        ::puroro::tags::OneofField,
                        ::puroro::tags::Message<
                            ::std::boxed::Box<
                                self::_puroro_root::oneofs3::_puroro_simple_impl::Submsg,
                            >,
                        >,
                    >::deser_field(field_value_mut_ref, data)
                }
                6 => {
                    use super::_puroro_nested::msg::_puroro_private_oneofs::GroupThreeSimple as E;
                    if !matches!(&self.group_three, Some(E::G3Int32(_))) {
                        self.group_three = Some(E::G3Int32(::std::default::Default::default()));
                    }
                    let field_value_mut_ref = match &mut self.group_three {
                        Some(E::G3Int32(v)) => v,
                        _ => unreachable!(),
                    };
                    DeserFieldFromBytesIter::<
                    ::puroro::tags::OneofField, ::puroro::tags::Int32
                >::deser_field(field_value_mut_ref, data)
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
                ::puroro::tags::OneofField,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g1_int32_opt(self),
                1,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g1_string_opt(self),
                2,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::Float,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g2_f32_opt(self),
                3,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g2_string_opt(self),
                4,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::Message<
                    <Self as super::_puroro_traits::MsgTrait>::Field5ScalarGetterType<'_>,
                >,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g2_submsg_opt(self),
                5,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g3_int32_opt(self),
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
                    "g1_int32",
                    &<Self as super::_puroro_traits::MsgTrait>::g1_int32_opt(self),
                )
                .field(
                    "g1_string",
                    &<Self as super::_puroro_traits::MsgTrait>::g1_string_opt(self),
                )
                .field(
                    "g2_f32",
                    &<Self as super::_puroro_traits::MsgTrait>::g2_f32_opt(self),
                )
                .field(
                    "g2_string",
                    &<Self as super::_puroro_traits::MsgTrait>::g2_string_opt(self),
                )
                .field(
                    "g2_submsg",
                    &<Self as super::_puroro_traits::MsgTrait>::g2_submsg_opt(self),
                )
                .field(
                    "g3_int32",
                    &<Self as super::_puroro_traits::MsgTrait>::g3_int32_opt(self),
                )
                .finish()
        }
    }

    impl ::std::clone::Clone for Msg {
        fn clone(&self) -> Self {
            Self {
                group_one: ::std::clone::Clone::clone(&self.group_one),
                group_two: ::std::clone::Clone::clone(&self.group_two),
                group_three: ::std::clone::Clone::clone(&self.group_three),
            }
        }
    }

    impl ::std::cmp::PartialEq for Msg {
        fn eq(&self, rhs: &Self) -> bool {
            self.group_one == rhs.group_one
                && self.group_two == rhs.group_two
                && self.group_three == rhs.group_three
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
        fn g1_int32<'this>(&'this self) -> i32 {
            self.g1_int32_opt()
                .unwrap_or(::std::default::Default::default())
        }

        fn has_g1_int32<'this>(&'this self) -> bool {
            self.g1_int32_opt().is_some()
        }

        type Field2ScalarGetterType<'this>: ::std::convert::AsRef<str>
        where
            Self: 'this;

        fn g1_string<'this>(&'this self) -> Self::Field2ScalarGetterType<'this> {
            self.g1_string_opt()
                .unwrap_or(self.g1_string_default_value())
        }
        fn g1_string_default_value(&self) -> Self::Field2ScalarGetterType<'_>;

        fn has_g1_string<'this>(&'this self) -> bool {
            self.g1_string_opt().is_some()
        }

        fn g2_f32<'this>(&'this self) -> f32 {
            self.g2_f32_opt()
                .unwrap_or(::std::default::Default::default())
        }

        fn has_g2_f32<'this>(&'this self) -> bool {
            self.g2_f32_opt().is_some()
        }

        type Field4ScalarGetterType<'this>: ::std::convert::AsRef<str>
        where
            Self: 'this;

        fn g2_string<'this>(&'this self) -> Self::Field4ScalarGetterType<'this> {
            self.g2_string_opt()
                .unwrap_or(self.g2_string_default_value())
        }
        fn g2_string_default_value(&self) -> Self::Field4ScalarGetterType<'_>;

        fn has_g2_string<'this>(&'this self) -> bool {
            self.g2_string_opt().is_some()
        }

        type Field5ScalarGetterType<'this>: self::_puroro_root::oneofs3::_puroro_traits::SubmsgTrait
        where
            Self: 'this;

        fn g2_submsg<'this>(&'this self) -> Self::Field5ScalarGetterType<'this> {
            self.g2_submsg_opt()
                .unwrap_or(self.g2_submsg_default_value())
        }
        fn g2_submsg_default_value(&self) -> Self::Field5ScalarGetterType<'_>;

        fn has_g2_submsg<'this>(&'this self) -> bool {
            self.g2_submsg_opt().is_some()
        }

        fn g3_int32<'this>(&'this self) -> i32 {
            self.g3_int32_opt()
                .unwrap_or(::std::default::Default::default())
        }

        fn has_g3_int32<'this>(&'this self) -> bool {
            self.g3_int32_opt().is_some()
        }
        fn group_one<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this, Self>>
        {
            ::std::option::Option::None
        }
        fn g1_int32_opt<'this>(&'this self) -> Option<i32> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupOne as E;
            self.group_one().and_then(|oneof| {
                #[allow(irrefutable_let_patterns)]
                if let E::G1Int32(v) = oneof {
                    ::std::option::Option::Some(v)
                } else {
                    ::std::option::Option::None
                }
            })
        }
        fn g1_string_opt<'this>(
            &'this self,
        ) -> Option<
            <Self as self::_puroro_root::oneofs3::_puroro_traits::MsgTrait>::Field2ScalarGetterType<
                'this,
            >,
        > {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupOne as E;
            self.group_one().and_then(|oneof| {
                #[allow(irrefutable_let_patterns)]
                if let E::G1String(v) = oneof {
                    ::std::option::Option::Some(v)
                } else {
                    ::std::option::Option::None
                }
            })
        }
        fn group_two<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>>
        {
            ::std::option::Option::None
        }
        fn g2_f32_opt<'this>(&'this self) -> Option<f32> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
            self.group_two().and_then(|oneof| {
                #[allow(irrefutable_let_patterns)]
                if let E::G2F32(v) = oneof {
                    ::std::option::Option::Some(v)
                } else {
                    ::std::option::Option::None
                }
            })
        }
        fn g2_string_opt<'this>(
            &'this self,
        ) -> Option<
            <Self as self::_puroro_root::oneofs3::_puroro_traits::MsgTrait>::Field4ScalarGetterType<
                'this,
            >,
        > {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
            self.group_two().and_then(|oneof| {
                #[allow(irrefutable_let_patterns)]
                if let E::G2String(v) = oneof {
                    ::std::option::Option::Some(v)
                } else {
                    ::std::option::Option::None
                }
            })
        }
        fn g2_submsg_opt<'this>(
            &'this self,
        ) -> Option<
            <Self as self::_puroro_root::oneofs3::_puroro_traits::MsgTrait>::Field5ScalarGetterType<
                'this,
            >,
        > {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
            self.group_two().and_then(|oneof| {
                #[allow(irrefutable_let_patterns)]
                if let E::G2Submsg(v) = oneof {
                    ::std::option::Option::Some(v)
                } else {
                    ::std::option::Option::None
                }
            })
        }
        fn group_three<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            ::std::option::Option::None
        }
        fn g3_int32_opt<'this>(&'this self) -> Option<i32> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupThree as E;
            self.group_three().and_then(|oneof| {
                #[allow(irrefutable_let_patterns)]
                if let E::G3Int32(v) = oneof {
                    ::std::option::Option::Some(v)
                } else {
                    ::std::option::Option::None
                }
            })
        }
    }

    macro_rules! msg_delegate {
        ($ty:ty) => {
            type Field2ScalarGetterType<'this>
            where
                Self: 'this,
            = <$ty as MsgTrait>::Field2ScalarGetterType<'this>;
            fn g1_string_default_value(&self) -> <$ty as MsgTrait>::Field2ScalarGetterType<'_> {
                <$ty as MsgTrait>::g1_string_default_value(self)
            }
            type Field4ScalarGetterType<'this>
            where
                Self: 'this,
            = <$ty as MsgTrait>::Field4ScalarGetterType<'this>;
            fn g2_string_default_value(&self) -> <$ty as MsgTrait>::Field4ScalarGetterType<'_> {
                <$ty as MsgTrait>::g2_string_default_value(self)
            }
            type Field5ScalarGetterType<'this>
            where
                Self: 'this,
            = <$ty as MsgTrait>::Field5ScalarGetterType<'this>;
            fn g2_submsg_default_value(&self) -> <$ty as MsgTrait>::Field5ScalarGetterType<'_> {
                <$ty as MsgTrait>::g2_submsg_default_value(self)
            }
            fn group_one<'this>(
                &'this self,
            ) -> ::std::option::Option<
                super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this, Self>,
            > {
                (**self).group_one().map(|v| v.into())
            }
            fn group_two<'this>(
                &'this self,
            ) -> ::std::option::Option<
                super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>,
            > {
                (**self).group_two().map(|v| v.into())
            }
            fn group_three<'this>(
                &'this self,
            ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
                (**self).group_three().map(|v| v.into())
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
        type Field2ScalarGetterType<'this> = &'this str;
        fn g1_string_default_value(&self) -> Self::Field2ScalarGetterType<'_> {
            ::std::default::Default::default()
        }
        type Field4ScalarGetterType<'this> = &'this str;
        fn g2_string_default_value(&self) -> Self::Field4ScalarGetterType<'_> {
            ::std::default::Default::default()
        }
        type Field5ScalarGetterType<'this> = ();
        fn g2_submsg_default_value(&self) -> Self::Field5ScalarGetterType<'_> {
            ::std::default::Default::default()
        }
        fn group_one<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this, Self>> {
            None
        }
        fn group_two<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>> {
            None
        }
        fn group_three<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            None
        }
    }
    impl<T> MsgTrait for ::std::option::Option<T>
    where
        T: MsgTrait,
    {
        type Field2ScalarGetterType<'this>
        where
            Self: 'this,
        = ::puroro::Either<T::Field2ScalarGetterType<'this>, &'this str>;
        fn g1_string_default_value(&self) -> Self::Field2ScalarGetterType<'_> {
            ::puroro::Either::Right(::std::default::Default::default())
        }
        type Field4ScalarGetterType<'this>
        where
            Self: 'this,
        = ::puroro::Either<T::Field4ScalarGetterType<'this>, &'this str>;
        fn g2_string_default_value(&self) -> Self::Field4ScalarGetterType<'_> {
            ::puroro::Either::Right(::std::default::Default::default())
        }
        type Field5ScalarGetterType<'this>
        where
            Self: 'this,
        = ::puroro::Either<T::Field5ScalarGetterType<'this>, ()>;
        fn g2_submsg_default_value(&self) -> Self::Field5ScalarGetterType<'_> {
            ::puroro::Either::Right(::std::default::Default::default())
        }
        fn group_one<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this, Self>> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupOne as E;
            self.as_ref().and_then(|msg| {
                msg.group_one().map(|oneof| match oneof {
                    E::G1Int32(v) => E::G1Int32(v),
                    E::G1String(v) => E::G1String(::puroro::Either::Left(v)),
                })
            })
        }
        fn group_two<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
            self.as_ref().and_then(|msg| {
                msg.group_two().map(|oneof| match oneof {
                    E::G2F32(v) => E::G2F32(v),
                    E::G2String(v) => E::G2String(::puroro::Either::Left(v)),
                    E::G2Submsg(v) => E::G2Submsg(::puroro::Either::Left(v)),
                })
            })
        }
        fn group_three<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupThree as E;
            self.as_ref().and_then(|msg| {
                msg.group_three().map(|oneof| match oneof {
                    E::G3Int32(v) => E::G3Int32(v),
                })
            })
        }
    }
    impl<T, U> MsgTrait for (T, U)
    where
        T: MsgTrait,
        U: MsgTrait,
    {
        type Field2ScalarGetterType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as MsgTrait>::Field2ScalarGetterType<'this>,
            <U as MsgTrait>::Field2ScalarGetterType<'this>,
        >;
        type Field4ScalarGetterType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as MsgTrait>::Field4ScalarGetterType<'this>,
            <U as MsgTrait>::Field4ScalarGetterType<'this>,
        >;
        type Field5ScalarGetterType<'this>
        where
            Self: 'this,
        = (
            ::std::option::Option<<T as MsgTrait>::Field5ScalarGetterType<'this>>,
            ::std::option::Option<<U as MsgTrait>::Field5ScalarGetterType<'this>>,
        );
        fn group_one<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this, Self>> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupOne as E;
            Some(match (self.0.group_one(), self.1.group_one()) {
                (None, None) => {
                    return None;
                }
                (Some(E::G1Int32(_)), Some(E::G1Int32(right))) => E::G1Int32(right),
                (Some(E::G1String(_)), Some(E::G1String(right))) => {
                    E::G1String(::puroro::Either::Right(right))
                }
                (_, Some(E::G1Int32(right))) => E::G1Int32(right),
                (_, Some(E::G1String(right))) => E::G1String(::puroro::Either::Right(right)),
                (Some(E::G1Int32(left)), None) => E::G1Int32(left),
                (Some(E::G1String(left)), None) => E::G1String(::puroro::Either::Left(left)),
            })
        }
        fn group_two<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
            Some(match (self.0.group_two(), self.1.group_two()) {
                (None, None) => {
                    return None;
                }
                (Some(E::G2F32(_)), Some(E::G2F32(right))) => E::G2F32(right),
                (Some(E::G2String(_)), Some(E::G2String(right))) => {
                    E::G2String(::puroro::Either::Right(right))
                }
                (Some(E::G2Submsg(left)), Some(E::G2Submsg(right))) => {
                    E::G2Submsg((Some(left), Some(right)))
                }
                (_, Some(E::G2F32(right))) => E::G2F32(right),
                (_, Some(E::G2String(right))) => E::G2String(::puroro::Either::Right(right)),
                (_, Some(E::G2Submsg(right))) => E::G2Submsg((None, Some(right))),
                (Some(E::G2F32(left)), None) => E::G2F32(left),
                (Some(E::G2String(left)), None) => E::G2String(::puroro::Either::Left(left)),
                (Some(E::G2Submsg(left)), None) => E::G2Submsg((Some(left), None)),
            })
        }
        fn group_three<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupThree as E;
            Some(match (self.0.group_three(), self.1.group_three()) {
                (None, None) => {
                    return None;
                }
                (Some(E::G3Int32(_)), Some(E::G3Int32(right))) => E::G3Int32(right),
                (_, Some(E::G3Int32(right))) => E::G3Int32(right),
                (Some(E::G3Int32(left)), None) => E::G3Int32(left),
            })
        }
    }
    impl<T, U> MsgTrait for ::puroro::Either<T, U>
    where
        T: MsgTrait,
        U: MsgTrait,
    {
        type Field2ScalarGetterType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as MsgTrait>::Field2ScalarGetterType<'this>,
            <U as MsgTrait>::Field2ScalarGetterType<'this>,
        >;
        type Field4ScalarGetterType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as MsgTrait>::Field4ScalarGetterType<'this>,
            <U as MsgTrait>::Field4ScalarGetterType<'this>,
        >;
        type Field5ScalarGetterType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as MsgTrait>::Field5ScalarGetterType<'this>,
            <U as MsgTrait>::Field5ScalarGetterType<'this>,
        >;
        fn group_one<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this, Self>> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupOne as E;
            match self.as_ref().either(
                |t| t.group_one().map(|t| ::puroro::Either::Left(t)),
                |u| u.group_one().map(|u| ::puroro::Either::Right(u)),
            ) {
                Some(::puroro::Either::Left(E::G1Int32(v))) => Some(E::G1Int32(v)),
                Some(::puroro::Either::Right(E::G1Int32(v))) => Some(E::G1Int32(v)),
                Some(::puroro::Either::Left(E::G1String(v))) => {
                    Some(E::G1String(::puroro::Either::Left(v)))
                }
                Some(::puroro::Either::Right(E::G1String(v))) => {
                    Some(E::G1String(::puroro::Either::Right(v)))
                }
                None => None,
            }
        }
        fn group_two<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
            match self.as_ref().either(
                |t| t.group_two().map(|t| ::puroro::Either::Left(t)),
                |u| u.group_two().map(|u| ::puroro::Either::Right(u)),
            ) {
                Some(::puroro::Either::Left(E::G2F32(v))) => Some(E::G2F32(v)),
                Some(::puroro::Either::Right(E::G2F32(v))) => Some(E::G2F32(v)),
                Some(::puroro::Either::Left(E::G2String(v))) => {
                    Some(E::G2String(::puroro::Either::Left(v)))
                }
                Some(::puroro::Either::Right(E::G2String(v))) => {
                    Some(E::G2String(::puroro::Either::Right(v)))
                }
                Some(::puroro::Either::Left(E::G2Submsg(v))) => {
                    Some(E::G2Submsg(::puroro::Either::Left(v)))
                }
                Some(::puroro::Either::Right(E::G2Submsg(v))) => {
                    Some(E::G2Submsg(::puroro::Either::Right(v)))
                }
                None => None,
            }
        }
        fn group_three<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupThree as E;
            match self.as_ref().either(
                |t| t.group_three().map(|t| ::puroro::Either::Left(t)),
                |u| u.group_three().map(|u| ::puroro::Either::Right(u)),
            ) {
                Some(::puroro::Either::Left(E::G3Int32(v))) => Some(E::G3Int32(v)),
                Some(::puroro::Either::Right(E::G3Int32(v))) => Some(E::G3Int32(v)),
                None => None,
            }
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
    impl<T, U> SubmsgTrait for (T, U)
    where
        T: SubmsgTrait,
        U: SubmsgTrait,
    {
        fn i32_unlabeled_opt<'this>(&'this self) -> Option<i32> {
            <U as SubmsgTrait>::i32_unlabeled_opt(&self.1)
                .or_else(|| <T as SubmsgTrait>::i32_unlabeled_opt(&self.0))
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
    }
}
pub use _puroro_nested::*;
pub mod _puroro_nested {
    pub mod msg {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }

        pub use _puroro_oneofs::*;
        pub mod _puroro_oneofs {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }

            pub enum GroupOne<'msg, T>
            where
                T: 'msg + ?Sized + self::_puroro_root::oneofs3::_puroro_traits::MsgTrait,
            {
                G1Int32(i32),
                G1String(<T as self::_puroro_root::oneofs3::_puroro_traits::MsgTrait>::Field2ScalarGetterType<'msg>),
            }

            impl<'msg, T> GroupOne<'msg, T>
            where
                T: 'msg + ?Sized + self::_puroro_root::oneofs3::_puroro_traits::MsgTrait,
            {
                pub fn g1_int32(self) -> ::std::option::Option<i32> {
                    match self {
                        Self::G1Int32(v) => ::std::option::Option::Some(v),
                        #[allow(unreachable_patterns)]
                        _ => None,
                    }
                }
                pub fn g1_string(self) -> ::std::option::Option<<T as self::_puroro_root::oneofs3::_puroro_traits::MsgTrait>::Field2ScalarGetterType<'msg>>{
                    match self {
                        Self::G1String(v) => ::std::option::Option::Some(v),
                        #[allow(unreachable_patterns)]
                        _ => None,
                    }
                }
            }

            impl<'msg, T> ::std::fmt::Debug for GroupOne<'msg, T>
            where
                i32: ::std::fmt::Debug,
                <T as self::_puroro_root::oneofs3::_puroro_traits::MsgTrait>::Field2ScalarGetterType<'msg>: ::std::fmt::Debug,
                T: 'msg + ?Sized + self::_puroro_root::oneofs3::_puroro_traits::MsgTrait,
            {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    match self {
                        Self::G1Int32(v) => f
                            .debug_tuple("GroupOne::G1Int32")
                            .field(&v)
                            .finish(),
                        Self::G1String(v) => f
                            .debug_tuple("GroupOne::G1String")
                            .field(&v)
                            .finish(),
                    }
                }
            }

            impl<'msg, T> ::std::clone::Clone for GroupOne<'msg, T>
            where
                i32: ::std::clone::Clone,
                <T as self::_puroro_root::oneofs3::_puroro_traits::MsgTrait>::Field2ScalarGetterType<'msg>: ::std::clone::Clone,
                T: 'msg + ?Sized + self::_puroro_root::oneofs3::_puroro_traits::MsgTrait,
            {
                fn clone(&self) -> Self {
                    match self {
                        Self::G1Int32(v) => Self::G1Int32(
                            ::std::clone::Clone::clone(&v)
                        ),
                        Self::G1String(v) => Self::G1String(
                            ::std::clone::Clone::clone(&v)
                        ),
                    }
                }
            }

            impl<'msg, T> ::std::cmp::PartialEq for GroupOne<'msg, T>
            where
                i32: ::std::cmp::PartialEq,
                <T as self::_puroro_root::oneofs3::_puroro_traits::MsgTrait>::Field2ScalarGetterType<'msg>: ::std::cmp::PartialEq,
                T: 'msg + ?Sized + self::_puroro_root::oneofs3::_puroro_traits::MsgTrait,
            {
                fn eq(&self, rhs: &Self) -> bool {
                    match (self, rhs) {
                        (Self::G1Int32(left), Self::G1Int32(right)) => left == right,
                        (Self::G1String(left), Self::G1String(right)) => left == right,
                        #[allow(unreachable_patterns)]
                        _ => false,
                    }
                }
            }
            impl<'msg, T> ::std::convert::From<GroupOne<'msg, T>> for GroupOne<'msg, &'_ T>
            where
                T: 'msg + self::_puroro_root::oneofs3::_puroro_traits::MsgTrait,
            {
                fn from(value: GroupOne<'msg, T>) -> Self {
                    match value {
                        GroupOne::G1Int32(v) => GroupOne::G1Int32(v),
                        GroupOne::G1String(v) => GroupOne::G1String(v),
                    }
                }
            }
            impl<'msg, T> ::std::convert::From<GroupOne<'msg, T>> for GroupOne<'msg, &'_ mut T>
            where
                T: 'msg + self::_puroro_root::oneofs3::_puroro_traits::MsgTrait,
            {
                fn from(value: GroupOne<'msg, T>) -> Self {
                    match value {
                        GroupOne::G1Int32(v) => GroupOne::G1Int32(v),
                        GroupOne::G1String(v) => GroupOne::G1String(v),
                    }
                }
            }
            impl<'msg, T> ::std::convert::From<GroupOne<'msg, T>> for GroupOne<'msg, ::std::boxed::Box<T>>
            where
                T: 'msg + self::_puroro_root::oneofs3::_puroro_traits::MsgTrait,
            {
                fn from(value: GroupOne<'msg, T>) -> Self {
                    match value {
                        GroupOne::G1Int32(v) => GroupOne::G1Int32(v),
                        GroupOne::G1String(v) => GroupOne::G1String(v),
                    }
                }
            }
            impl<'msg, 'bump, T> ::std::convert::From<GroupOne<'msg, T>>
                for GroupOne<'msg, ::puroro::bumpalo::boxed::Box<'bump, T>>
            where
                T: 'msg + self::_puroro_root::oneofs3::_puroro_traits::MsgTrait,
            {
                fn from(value: GroupOne<'msg, T>) -> Self {
                    match value {
                        GroupOne::G1Int32(v) => GroupOne::G1Int32(v),
                        GroupOne::G1String(v) => GroupOne::G1String(v),
                    }
                }
            }
            impl<'msg, 'bump, T> ::std::convert::From<GroupOne<'msg, T>>
                for GroupOne<'msg, ::puroro::BumpaloOwned<T>>
            where
                T: 'msg + self::_puroro_root::oneofs3::_puroro_traits::MsgTrait,
            {
                fn from(value: GroupOne<'msg, T>) -> Self {
                    match value {
                        GroupOne::G1Int32(v) => GroupOne::G1Int32(v),
                        GroupOne::G1String(v) => GroupOne::G1String(v),
                    }
                }
            }

            pub enum GroupTwo<'msg, T>
            where
                T: 'msg + ?Sized + self::_puroro_root::oneofs3::_puroro_traits::MsgTrait,
            {
                G2F32(f32),
                G2String(<T as self::_puroro_root::oneofs3::_puroro_traits::MsgTrait>::Field4ScalarGetterType<'msg>),
                G2Submsg(<T as self::_puroro_root::oneofs3::_puroro_traits::MsgTrait>::Field5ScalarGetterType<'msg>),
            }

            impl<'msg, T> GroupTwo<'msg, T>
            where
                T: 'msg + ?Sized + self::_puroro_root::oneofs3::_puroro_traits::MsgTrait,
            {
                pub fn g2_f32(self) -> ::std::option::Option<f32> {
                    match self {
                        Self::G2F32(v) => ::std::option::Option::Some(v),
                        #[allow(unreachable_patterns)]
                        _ => None,
                    }
                }
                pub fn g2_string(self) -> ::std::option::Option<<T as self::_puroro_root::oneofs3::_puroro_traits::MsgTrait>::Field4ScalarGetterType<'msg>>{
                    match self {
                        Self::G2String(v) => ::std::option::Option::Some(v),
                        #[allow(unreachable_patterns)]
                        _ => None,
                    }
                }
                pub fn g2_submsg(self) -> ::std::option::Option<<T as self::_puroro_root::oneofs3::_puroro_traits::MsgTrait>::Field5ScalarGetterType<'msg>>{
                    match self {
                        Self::G2Submsg(v) => ::std::option::Option::Some(v),
                        #[allow(unreachable_patterns)]
                        _ => None,
                    }
                }
            }

            impl<'msg, T> ::std::fmt::Debug for GroupTwo<'msg, T>
            where
                f32: ::std::fmt::Debug,
                <T as self::_puroro_root::oneofs3::_puroro_traits::MsgTrait>::Field4ScalarGetterType<'msg>: ::std::fmt::Debug,
                <T as self::_puroro_root::oneofs3::_puroro_traits::MsgTrait>::Field5ScalarGetterType<'msg>: ::std::fmt::Debug,
                T: 'msg + ?Sized + self::_puroro_root::oneofs3::_puroro_traits::MsgTrait,
            {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    match self {
                        Self::G2F32(v) => f
                            .debug_tuple("GroupTwo::G2F32")
                            .field(&v)
                            .finish(),
                        Self::G2String(v) => f
                            .debug_tuple("GroupTwo::G2String")
                            .field(&v)
                            .finish(),
                        Self::G2Submsg(v) => f
                            .debug_tuple("GroupTwo::G2Submsg")
                            .field(&v)
                            .finish(),
                    }
                }
            }

            impl<'msg, T> ::std::clone::Clone for GroupTwo<'msg, T>
            where
                f32: ::std::clone::Clone,
                <T as self::_puroro_root::oneofs3::_puroro_traits::MsgTrait>::Field4ScalarGetterType<'msg>: ::std::clone::Clone,
                <T as self::_puroro_root::oneofs3::_puroro_traits::MsgTrait>::Field5ScalarGetterType<'msg>: ::std::clone::Clone,
                T: 'msg + ?Sized + self::_puroro_root::oneofs3::_puroro_traits::MsgTrait,
            {
                fn clone(&self) -> Self {
                    match self {
                        Self::G2F32(v) => Self::G2F32(
                            ::std::clone::Clone::clone(&v)
                        ),
                        Self::G2String(v) => Self::G2String(
                            ::std::clone::Clone::clone(&v)
                        ),
                        Self::G2Submsg(v) => Self::G2Submsg(
                            ::std::clone::Clone::clone(&v)
                        ),
                    }
                }
            }

            impl<'msg, T> ::std::cmp::PartialEq for GroupTwo<'msg, T>
            where
                f32: ::std::cmp::PartialEq,
                <T as self::_puroro_root::oneofs3::_puroro_traits::MsgTrait>::Field4ScalarGetterType<'msg>: ::std::cmp::PartialEq,
                <T as self::_puroro_root::oneofs3::_puroro_traits::MsgTrait>::Field5ScalarGetterType<'msg>: ::std::cmp::PartialEq,
                T: 'msg + ?Sized + self::_puroro_root::oneofs3::_puroro_traits::MsgTrait,
            {
                fn eq(&self, rhs: &Self) -> bool {
                    match (self, rhs) {
                        (Self::G2F32(left), Self::G2F32(right)) => left == right,
                        (Self::G2String(left), Self::G2String(right)) => left == right,
                        (Self::G2Submsg(left), Self::G2Submsg(right)) => left == right,
                        #[allow(unreachable_patterns)]
                        _ => false,
                    }
                }
            }
            impl<'msg, T> ::std::convert::From<GroupTwo<'msg, T>> for GroupTwo<'msg, &'_ T>
            where
                T: 'msg + self::_puroro_root::oneofs3::_puroro_traits::MsgTrait,
            {
                fn from(value: GroupTwo<'msg, T>) -> Self {
                    match value {
                        GroupTwo::G2F32(v) => GroupTwo::G2F32(v),
                        GroupTwo::G2String(v) => GroupTwo::G2String(v),
                        GroupTwo::G2Submsg(v) => GroupTwo::G2Submsg(v),
                    }
                }
            }
            impl<'msg, T> ::std::convert::From<GroupTwo<'msg, T>> for GroupTwo<'msg, &'_ mut T>
            where
                T: 'msg + self::_puroro_root::oneofs3::_puroro_traits::MsgTrait,
            {
                fn from(value: GroupTwo<'msg, T>) -> Self {
                    match value {
                        GroupTwo::G2F32(v) => GroupTwo::G2F32(v),
                        GroupTwo::G2String(v) => GroupTwo::G2String(v),
                        GroupTwo::G2Submsg(v) => GroupTwo::G2Submsg(v),
                    }
                }
            }
            impl<'msg, T> ::std::convert::From<GroupTwo<'msg, T>> for GroupTwo<'msg, ::std::boxed::Box<T>>
            where
                T: 'msg + self::_puroro_root::oneofs3::_puroro_traits::MsgTrait,
            {
                fn from(value: GroupTwo<'msg, T>) -> Self {
                    match value {
                        GroupTwo::G2F32(v) => GroupTwo::G2F32(v),
                        GroupTwo::G2String(v) => GroupTwo::G2String(v),
                        GroupTwo::G2Submsg(v) => GroupTwo::G2Submsg(v),
                    }
                }
            }
            impl<'msg, 'bump, T> ::std::convert::From<GroupTwo<'msg, T>>
                for GroupTwo<'msg, ::puroro::bumpalo::boxed::Box<'bump, T>>
            where
                T: 'msg + self::_puroro_root::oneofs3::_puroro_traits::MsgTrait,
            {
                fn from(value: GroupTwo<'msg, T>) -> Self {
                    match value {
                        GroupTwo::G2F32(v) => GroupTwo::G2F32(v),
                        GroupTwo::G2String(v) => GroupTwo::G2String(v),
                        GroupTwo::G2Submsg(v) => GroupTwo::G2Submsg(v),
                    }
                }
            }
            impl<'msg, 'bump, T> ::std::convert::From<GroupTwo<'msg, T>>
                for GroupTwo<'msg, ::puroro::BumpaloOwned<T>>
            where
                T: 'msg + self::_puroro_root::oneofs3::_puroro_traits::MsgTrait,
            {
                fn from(value: GroupTwo<'msg, T>) -> Self {
                    match value {
                        GroupTwo::G2F32(v) => GroupTwo::G2F32(v),
                        GroupTwo::G2String(v) => GroupTwo::G2String(v),
                        GroupTwo::G2Submsg(v) => GroupTwo::G2Submsg(v),
                    }
                }
            }

            pub enum GroupThree {
                G3Int32(i32),
            }

            impl GroupThree {
                pub fn g3_int32(self) -> ::std::option::Option<i32> {
                    match self {
                        Self::G3Int32(v) => ::std::option::Option::Some(v),
                        #[allow(unreachable_patterns)]
                        _ => None,
                    }
                }
            }

            impl ::std::fmt::Debug for GroupThree
            where
                i32: ::std::fmt::Debug,
            {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    match self {
                        Self::G3Int32(v) => f.debug_tuple("GroupThree::G3Int32").field(&v).finish(),
                    }
                }
            }

            impl ::std::clone::Clone for GroupThree
            where
                i32: ::std::clone::Clone,
            {
                fn clone(&self) -> Self {
                    match self {
                        Self::G3Int32(v) => Self::G3Int32(::std::clone::Clone::clone(&v)),
                    }
                }
            }

            impl ::std::cmp::PartialEq for GroupThree
            where
                i32: ::std::cmp::PartialEq,
            {
                fn eq(&self, rhs: &Self) -> bool {
                    match (self, rhs) {
                        (Self::G3Int32(left), Self::G3Int32(right)) => left == right,
                        #[allow(unreachable_patterns)]
                        _ => false,
                    }
                }
            }
        }
        pub mod _puroro_private_oneofs {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }

            pub(crate) enum GroupOneSimple {
                G1Int32(i32),
                G1String(::std::string::String),
            }

            impl ::std::fmt::Debug for GroupOneSimple {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    match self {
                        Self::G1Int32(v) => {
                            f.debug_tuple("GroupOneSimple::G1Int32").field(&v).finish()
                        }
                        Self::G1String(v) => {
                            f.debug_tuple("GroupOneSimple::G1String").field(&v).finish()
                        }
                    }
                }
            }

            impl ::std::clone::Clone for GroupOneSimple {
                fn clone(&self) -> Self {
                    match self {
                        Self::G1Int32(v) => Self::G1Int32(::std::clone::Clone::clone(&v)),
                        Self::G1String(v) => Self::G1String(::std::clone::Clone::clone(&v)),
                    }
                }
            }

            impl ::std::cmp::PartialEq for GroupOneSimple {
                fn eq(&self, rhs: &Self) -> bool {
                    match (self, rhs) {
                        (Self::G1Int32(left), Self::G1Int32(right)) => left == right,
                        (Self::G1String(left), Self::G1String(right)) => left == right,
                        #[allow(unreachable_patterns)]
                        _ => false,
                    }
                }
            }

            pub(crate) enum GroupTwoSimple {
                G2F32(f32),
                G2String(::std::string::String),
                G2Submsg(
                    ::std::boxed::Box<self::_puroro_root::oneofs3::_puroro_simple_impl::Submsg>,
                ),
            }

            impl ::std::fmt::Debug for GroupTwoSimple {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    match self {
                        Self::G2F32(v) => f.debug_tuple("GroupTwoSimple::G2F32").field(&v).finish(),
                        Self::G2String(v) => {
                            f.debug_tuple("GroupTwoSimple::G2String").field(&v).finish()
                        }
                        Self::G2Submsg(v) => {
                            f.debug_tuple("GroupTwoSimple::G2Submsg").field(&v).finish()
                        }
                    }
                }
            }

            impl ::std::clone::Clone for GroupTwoSimple {
                fn clone(&self) -> Self {
                    match self {
                        Self::G2F32(v) => Self::G2F32(::std::clone::Clone::clone(&v)),
                        Self::G2String(v) => Self::G2String(::std::clone::Clone::clone(&v)),
                        Self::G2Submsg(v) => Self::G2Submsg(::std::clone::Clone::clone(&v)),
                    }
                }
            }

            impl ::std::cmp::PartialEq for GroupTwoSimple {
                fn eq(&self, rhs: &Self) -> bool {
                    match (self, rhs) {
                        (Self::G2F32(left), Self::G2F32(right)) => left == right,
                        (Self::G2String(left), Self::G2String(right)) => left == right,
                        (Self::G2Submsg(left), Self::G2Submsg(right)) => left == right,
                        #[allow(unreachable_patterns)]
                        _ => false,
                    }
                }
            }

            pub(crate) enum GroupThreeSimple {
                G3Int32(i32),
            }

            impl ::std::fmt::Debug for GroupThreeSimple {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    match self {
                        Self::G3Int32(v) => f
                            .debug_tuple("GroupThreeSimple::G3Int32")
                            .field(&v)
                            .finish(),
                    }
                }
            }

            impl ::std::clone::Clone for GroupThreeSimple {
                fn clone(&self) -> Self {
                    match self {
                        Self::G3Int32(v) => Self::G3Int32(::std::clone::Clone::clone(&v)),
                    }
                }
            }

            impl ::std::cmp::PartialEq for GroupThreeSimple {
                fn eq(&self, rhs: &Self) -> bool {
                    match (self, rhs) {
                        (Self::G3Int32(left), Self::G3Int32(right)) => left == right,
                        #[allow(unreachable_patterns)]
                        _ => false,
                    }
                }
            }
        }
    }
    pub mod submsg {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }
    }
}
