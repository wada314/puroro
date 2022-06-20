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
        _bitfield:
            ::puroro::bitvec::array::BitArray<::puroro::bitvec::order::Lsb0, [u32; (5 + 31) / 32]>,
        group_one: ::std::option::Option<
            super::_puroro_nested::msg::_puroro_oneofs::GroupOne<
                ::puroro::internal::Bare<i32>,
                ::puroro::internal::Bare<::std::string::String>,
            >,
        >,
        group_two: ::std::option::Option<
            super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<
                ::puroro::internal::Bare<f32>,
                ::puroro::internal::Bare<::std::string::String>,
                ::puroro::internal::Bare<
                    ::std::boxed::Box<self::_puroro_root::oneofs3::_puroro_simple_impl::Submsg>,
                >,
            >,
        >,
        group_three: ::std::option::Option<
            super::_puroro_nested::msg::_puroro_oneofs::GroupThree<::puroro::internal::Bare<i32>>,
        >,
    }
    impl ::puroro::Message<Msg> for Msg {}

    impl Msg {
        pub fn new() -> Self {
            Self {
                _bitfield: ::std::default::Default::default(),
                group_one: ::std::default::Default::default(),
                group_two: ::std::default::Default::default(),
                group_three: ::std::default::Default::default(),
            }
        }
        pub fn g1_int32_opt(&self) -> ::std::option::Option<i32> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupOne as E;
            #[allow(unused_imports)]
            use ::std::option::Option::{None, Some};
            if let Some(E::G1Int32(ref val)) = &self.group_one {
                Some(val.clone().inner())
            } else {
                None
            }
        }

        pub fn has_g1_int32(&self) -> bool {
            Self::g1_int32_opt(self).is_some()
        }

        pub fn g1_int32(&self) -> i32 {
            self.g1_int32_opt()
                .unwrap_or(::std::default::Default::default())
        }
        pub fn g1_string_opt(&self) -> ::std::option::Option<&'_ str> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupOne as E;
            #[allow(unused_imports)]
            use ::std::option::Option::{None, Some};
            if let Some(E::G1String(ref val)) = &self.group_one {
                Some(val)
            } else {
                None
            }
        }

        pub fn has_g1_string(&self) -> bool {
            Self::g1_string_opt(self).is_some()
        }

        pub fn g1_string(&self) -> &'_ str {
            self.g1_string_opt()
                .unwrap_or(::std::default::Default::default())
        }
        pub fn g2_f32_opt(&self) -> ::std::option::Option<f32> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
            #[allow(unused_imports)]
            use ::std::option::Option::{None, Some};
            if let Some(E::G2F32(ref val)) = &self.group_two {
                Some(val.clone().inner())
            } else {
                None
            }
        }

        pub fn has_g2_f32(&self) -> bool {
            Self::g2_f32_opt(self).is_some()
        }

        pub fn g2_f32(&self) -> f32 {
            self.g2_f32_opt()
                .unwrap_or(::std::default::Default::default())
        }
        pub fn g2_string_opt(&self) -> ::std::option::Option<&'_ str> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
            #[allow(unused_imports)]
            use ::std::option::Option::{None, Some};
            if let Some(E::G2String(ref val)) = &self.group_two {
                Some(val)
            } else {
                None
            }
        }

        pub fn has_g2_string(&self) -> bool {
            Self::g2_string_opt(self).is_some()
        }

        pub fn g2_string(&self) -> &'_ str {
            self.g2_string_opt()
                .unwrap_or(::std::default::Default::default())
        }
        pub fn g2_submsg_opt(
            &self,
        ) -> ::std::option::Option<&'_ self::_puroro_root::oneofs3::_puroro_simple_impl::Submsg>
        {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
            #[allow(unused_imports)]
            use ::std::option::Option::{None, Some};
            if let Some(E::G2Submsg(ref val)) = &self.group_two {
                Some(val)
            } else {
                None
            }
        }

        pub fn has_g2_submsg(&self) -> bool {
            Self::g2_submsg_opt(self).is_some()
        }

        pub fn g2_submsg(
            &self,
        ) -> ::std::option::Option<&'_ self::_puroro_root::oneofs3::_puroro_simple_impl::Submsg>
        {
            self.g2_submsg_opt()
        }
        pub fn g3_int32_opt(&self) -> ::std::option::Option<i32> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupThree as E;
            #[allow(unused_imports)]
            use ::std::option::Option::{None, Some};
            if let Some(E::G3Int32(ref val)) = &self.group_three {
                Some(val.clone().inner())
            } else {
                None
            }
        }

        pub fn has_g3_int32(&self) -> bool {
            Self::g3_int32_opt(self).is_some()
        }

        pub fn g3_int32(&self) -> i32 {
            self.g3_int32_opt()
                .unwrap_or(::std::default::Default::default())
        }
        pub fn group_one(
            &self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<i32, &'_ str>>
        {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupOne as E;
            #[allow(unused_imports)]
            use ::std::option::Option::{None, Some};
            match &self.group_one {
                Some(E::G1Int32(ref val)) => Some(E::G1Int32(val.clone().inner())),
                Some(E::G1String(ref val)) => Some(E::G1String(val)),
                #[allow(unreachable_patterns)]
                _ => None,
            }
        }
        pub fn group_two(
            &self,
        ) -> ::std::option::Option<
            super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<
                f32,
                &'_ str,
                &'_ self::_puroro_root::oneofs3::_puroro_simple_impl::Submsg,
            >,
        > {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
            #[allow(unused_imports)]
            use ::std::option::Option::{None, Some};
            match &self.group_two {
                Some(E::G2F32(ref val)) => Some(E::G2F32(val.clone().inner())),
                Some(E::G2String(ref val)) => Some(E::G2String(val)),
                Some(E::G2Submsg(ref val)) => Some(E::G2Submsg(val)),
                #[allow(unreachable_patterns)]
                _ => None,
            }
        }
        pub fn group_three(
            &self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree<i32>>
        {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupThree as E;
            #[allow(unused_imports)]
            use ::std::option::Option::{None, Some};
            match &self.group_three {
                Some(E::G3Int32(ref val)) => Some(E::G3Int32(val.clone().inner())),
                #[allow(unreachable_patterns)]
                _ => None,
            }
        }
        pub fn clear_group_one(&mut self) {
            self.group_one = ::std::default::Default::default();
        }
        pub fn g1_int32_mut(&mut self) -> &'_ mut i32 {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupOne as E;
            #[allow(unused_imports)]
            use ::std::option::Option::{None, Some};
            if !matches!(&self.group_one, Some(E::G1Int32(_))) {
                self.group_one = Some(E::G1Int32(::std::default::Default::default()));
            }
            match &mut self.group_one {
                Some(E::G1Int32(ref mut v)) => v,
                _ => unreachable!(),
            }
        }
        pub fn g1_string_mut(&mut self) -> &'_ mut ::std::string::String {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupOne as E;
            #[allow(unused_imports)]
            use ::std::option::Option::{None, Some};
            if !matches!(&self.group_one, Some(E::G1String(_))) {
                self.group_one = Some(E::G1String(::std::default::Default::default()));
            }
            match &mut self.group_one {
                Some(E::G1String(ref mut v)) => v,
                _ => unreachable!(),
            }
        }
        pub fn clear_group_two(&mut self) {
            self.group_two = ::std::default::Default::default();
        }
        pub fn g2_f32_mut(&mut self) -> &'_ mut f32 {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
            #[allow(unused_imports)]
            use ::std::option::Option::{None, Some};
            if !matches!(&self.group_two, Some(E::G2F32(_))) {
                self.group_two = Some(E::G2F32(::std::default::Default::default()));
            }
            match &mut self.group_two {
                Some(E::G2F32(ref mut v)) => v,
                _ => unreachable!(),
            }
        }
        pub fn g2_string_mut(&mut self) -> &'_ mut ::std::string::String {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
            #[allow(unused_imports)]
            use ::std::option::Option::{None, Some};
            if !matches!(&self.group_two, Some(E::G2String(_))) {
                self.group_two = Some(E::G2String(::std::default::Default::default()));
            }
            match &mut self.group_two {
                Some(E::G2String(ref mut v)) => v,
                _ => unreachable!(),
            }
        }
        pub fn g2_submsg_mut(
            &mut self,
        ) -> &'_ mut self::_puroro_root::oneofs3::_puroro_simple_impl::Submsg {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
            #[allow(unused_imports)]
            use ::std::option::Option::{None, Some};
            if !matches!(&self.group_two, Some(E::G2Submsg(_))) {
                self.group_two = Some(E::G2Submsg(::std::default::Default::default()));
            }
            match &mut self.group_two {
                Some(E::G2Submsg(ref mut v)) => v,
                _ => unreachable!(),
            }
        }
        pub fn clear_group_three(&mut self) {
            self.group_three = ::std::default::Default::default();
        }
        pub fn g3_int32_mut(&mut self) -> &'_ mut i32 {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupThree as E;
            #[allow(unused_imports)]
            use ::std::option::Option::{None, Some};
            if !matches!(&self.group_three, Some(E::G3Int32(_))) {
                self.group_three = Some(E::G3Int32(::std::default::Default::default()));
            }
            match &mut self.group_three {
                Some(E::G3Int32(ref mut v)) => v,
                _ => unreachable!(),
            }
        }
    }

    impl super::_puroro_traits::MsgTrait for Msg {
        fn g1_int32_opt<'this>(&'this self) -> Option<i32> {
            <self::Msg>::g1_int32_opt(self)
        }
        fn g1_string_opt<'this>(&'this self) -> Option<&'this str> {
            <self::Msg>::g1_string_opt(self)
        }
        fn g2_f32_opt<'this>(&'this self) -> Option<f32> {
            <self::Msg>::g2_f32_opt(self)
        }
        fn g2_string_opt<'this>(&'this self) -> Option<&'this str> {
            <self::Msg>::g2_string_opt(self)
        }
        type G2SubmsgMessageType<'this> = &'this self::_puroro_root::oneofs3::_puroro_simple_impl::Submsg where Self: 'this;
        fn g2_submsg_opt<'this>(&'this self) -> Option<Self::G2SubmsgMessageType<'this>> {
            <self::Msg>::g2_submsg_opt(self)
        }
        fn g3_int32_opt<'this>(&'this self) -> Option<i32> {
            <self::Msg>::g3_int32_opt(self)
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
                    use super::_puroro_nested::msg::_puroro_oneofs::GroupOne as E;
                    if !matches!(&self.group_one, ::std::option::Option::Some(E::G1Int32(_))) {
                        self.group_one = ::std::option::Option::Some(E::G1Int32(
                            ::std::default::Default::default(),
                        ));
                    }
                    let field_value_mut_ref = match &mut self.group_one {
                        ::std::option::Option::Some(E::G1Int32(ref mut v)) => v,
                        _ => unreachable!(),
                    };
                    DeserFieldFromBytesIter::<
                    ::puroro::tags::OneofField, ::puroro::tags::Int32
                >::deser_field(field_value_mut_ref, data)
                }
                2 => {
                    use super::_puroro_nested::msg::_puroro_oneofs::GroupOne as E;
                    if !matches!(&self.group_one, ::std::option::Option::Some(E::G1String(_))) {
                        self.group_one = ::std::option::Option::Some(E::G1String(
                            ::std::default::Default::default(),
                        ));
                    }
                    let field_value_mut_ref = match &mut self.group_one {
                        ::std::option::Option::Some(E::G1String(ref mut v)) => v,
                        _ => unreachable!(),
                    };
                    DeserFieldFromBytesIter::<
                    ::puroro::tags::OneofField, ::puroro::tags::String
                >::deser_field(field_value_mut_ref, data)
                }
                3 => {
                    use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
                    if !matches!(&self.group_two, ::std::option::Option::Some(E::G2F32(_))) {
                        self.group_two = ::std::option::Option::Some(E::G2F32(
                            ::std::default::Default::default(),
                        ));
                    }
                    let field_value_mut_ref = match &mut self.group_two {
                        ::std::option::Option::Some(E::G2F32(ref mut v)) => v,
                        _ => unreachable!(),
                    };
                    DeserFieldFromBytesIter::<
                    ::puroro::tags::OneofField, ::puroro::tags::Float
                >::deser_field(field_value_mut_ref, data)
                }
                4 => {
                    use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
                    if !matches!(&self.group_two, ::std::option::Option::Some(E::G2String(_))) {
                        self.group_two = ::std::option::Option::Some(E::G2String(
                            ::std::default::Default::default(),
                        ));
                    }
                    let field_value_mut_ref = match &mut self.group_two {
                        ::std::option::Option::Some(E::G2String(ref mut v)) => v,
                        _ => unreachable!(),
                    };
                    DeserFieldFromBytesIter::<
                    ::puroro::tags::OneofField, ::puroro::tags::String
                >::deser_field(field_value_mut_ref, data)
                }
                5 => {
                    use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
                    if !matches!(&self.group_two, ::std::option::Option::Some(E::G2Submsg(_))) {
                        self.group_two = ::std::option::Option::Some(E::G2Submsg(
                            ::std::default::Default::default(),
                        ));
                    }
                    let field_value_mut_ref = match &mut self.group_two {
                        ::std::option::Option::Some(E::G2Submsg(ref mut v)) => v,
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
                    use super::_puroro_nested::msg::_puroro_oneofs::GroupThree as E;
                    if !matches!(
                        &self.group_three,
                        ::std::option::Option::Some(E::G3Int32(_))
                    ) {
                        self.group_three = ::std::option::Option::Some(E::G3Int32(
                            ::std::default::Default::default(),
                        ));
                    }
                    let field_value_mut_ref = match &mut self.group_three {
                        ::std::option::Option::Some(E::G3Int32(ref mut v)) => v,
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
                true,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g1_string_opt(self),
                2,
                out,
                true,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::Float,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g2_f32_opt(self),
                3,
                out,
                true,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g2_string_opt(self),
                4,
                out,
                true,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::Message<
                    <Self as super::_puroro_traits::MsgTrait>::G2SubmsgMessageType<'_>,
                >,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g2_submsg_opt(self),
                5,
                out,
                true,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g3_int32_opt(self),
                6,
                out,
                true,
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
                .field("g1_int32", &self.g1_int32_opt())
                .field("g1_string", &self.g1_string_opt())
                .field("g2_f32", &self.g2_f32_opt())
                .field("g2_string", &self.g2_string_opt())
                .field("g2_submsg", &self.g2_submsg_opt())
                .field("g3_int32", &self.g3_int32_opt())
                .finish()
        }
    }

    impl ::std::clone::Clone for Msg {
        fn clone(&self) -> Self {
            Self {
                _bitfield: ::std::clone::Clone::clone(&self._bitfield),
                group_one: ::std::clone::Clone::clone(&self.group_one),
                group_two: ::std::clone::Clone::clone(&self.group_two),
                group_three: ::std::clone::Clone::clone(&self.group_three),
            }
        }
    }

    impl ::std::cmp::PartialEq for Msg {
        fn eq(&self, rhs: &Self) -> bool {
            self._bitfield == rhs._bitfield
                && self.group_one == rhs.group_one
                && self.group_two == rhs.group_two
                && self.group_three == rhs.group_three
                && true
        }
    }
    pub struct Submsg {
        _bitfield:
            ::puroro::bitvec::array::BitArray<::puroro::bitvec::order::Lsb0, [u32; (0 + 31) / 32]>,
        i32_unlabeled: ::puroro::internal::Bare<i32>,
    }
    impl ::puroro::Message<Submsg> for Submsg {}

    impl Submsg {
        pub fn new() -> Self {
            Self {
                _bitfield: ::std::default::Default::default(),
                i32_unlabeled: ::std::default::Default::default(),
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
        pub fn clear_i32_unlabeled(&mut self) {
            self.i32_unlabeled = ::std::default::Default::default();
        }
        pub fn i32_unlabeled_mut(&mut self) -> &'_ mut i32 {
            if !self.has_i32_unlabeled() {
                self.i32_unlabeled = ::std::default::Default::default();
            }
            &mut self.i32_unlabeled
        }
    }

    impl super::_puroro_traits::SubmsgTrait for Submsg {
        fn i32_unlabeled_opt<'this>(&'this self) -> Option<i32> {
            <self::Submsg>::i32_unlabeled_opt(self)
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
            1 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::Int32
                >::deser_field(&mut self.i32_unlabeled, data)
            }

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
                true,
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
                .finish()
        }
    }

    impl ::std::clone::Clone for Submsg {
        fn clone(&self) -> Self {
            Self {
                _bitfield: ::std::clone::Clone::clone(&self._bitfield),
                i32_unlabeled: ::std::clone::Clone::clone(&self.i32_unlabeled),
            }
        }
    }

    impl ::std::cmp::PartialEq for Submsg {
        fn eq(&self, rhs: &Self) -> bool {
            self._bitfield == rhs._bitfield && self.i32_unlabeled == rhs.i32_unlabeled && true
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
        pub g1_int32: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField1<ScalarType> where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
    {
        fn g1_int32_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.g1_int32,
            )))
        }
        type G2SubmsgMessageType<'this> = () where Self: 'this;
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
                ::puroro::tags::OneofField,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g1_int32_opt(self),
                1,
                out,
                true,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
    {
        fn from(value: ScalarType) -> Self {
            Self { g1_int32: value }
        }
    }

    impl<ScalarType> ::std::clone::Clone for MsgSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        ScalarType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                g1_int32: ::std::clone::Clone::clone(&self.g1_int32),
            }
        }
    }

    pub struct MsgSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>,
    {
        pub g1_string: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField2<ScalarType> where
        ScalarType: ::std::convert::AsRef<str>
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>,
    {
        fn g1_string_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::Some(self.g1_string.as_ref())
        }
        type G2SubmsgMessageType<'this> = () where Self: 'this;
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g1_string_opt(self),
                2,
                out,
                true,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>,
    {
        fn from(value: ScalarType) -> Self {
            Self { g1_string: value }
        }
    }

    impl<ScalarType> ::std::clone::Clone for MsgSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>,
        ScalarType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                g1_string: ::std::clone::Clone::clone(&self.g1_string),
            }
        }
    }

    pub struct MsgSingleField3<ScalarType>
    where
        ScalarType: ::std::convert::Into<f32> + ::std::clone::Clone,
    {
        pub g2_f32: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField3<ScalarType> where
        ScalarType: ::std::convert::Into<f32> + ::std::clone::Clone
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField3<ScalarType>
    where
        ScalarType: ::std::convert::Into<f32> + ::std::clone::Clone,
    {
        fn g2_f32_opt<'this>(&'this self) -> ::std::option::Option<f32> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.g2_f32,
            )))
        }
        type G2SubmsgMessageType<'this> = () where Self: 'this;
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField3<ScalarType>
    where
        ScalarType: ::std::convert::Into<f32> + ::std::clone::Clone,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::Float,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g2_f32_opt(self),
                3,
                out,
                true,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField3<ScalarType>
    where
        ScalarType: ::std::convert::Into<f32> + ::std::clone::Clone,
    {
        fn from(value: ScalarType) -> Self {
            Self { g2_f32: value }
        }
    }

    impl<ScalarType> ::std::clone::Clone for MsgSingleField3<ScalarType>
    where
        ScalarType: ::std::convert::Into<f32> + ::std::clone::Clone,
        ScalarType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                g2_f32: ::std::clone::Clone::clone(&self.g2_f32),
            }
        }
    }

    pub struct MsgSingleField4<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>,
    {
        pub g2_string: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField4<ScalarType> where
        ScalarType: ::std::convert::AsRef<str>
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField4<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>,
    {
        fn g2_string_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::Some(self.g2_string.as_ref())
        }
        type G2SubmsgMessageType<'this> = () where Self: 'this;
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField4<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g2_string_opt(self),
                4,
                out,
                true,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField4<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>,
    {
        fn from(value: ScalarType) -> Self {
            Self { g2_string: value }
        }
    }

    impl<ScalarType> ::std::clone::Clone for MsgSingleField4<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>,
        ScalarType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                g2_string: ::std::clone::Clone::clone(&self.g2_string),
            }
        }
    }

    pub struct MsgSingleField5<ScalarType>
    where
        ScalarType: self::_puroro_root::oneofs3::_puroro_traits::SubmsgTrait,
    {
        pub g2_submsg: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField5<ScalarType> where
        ScalarType: self::_puroro_root::oneofs3::_puroro_traits::SubmsgTrait
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField5<ScalarType>
    where
        ScalarType: self::_puroro_root::oneofs3::_puroro_traits::SubmsgTrait,
    {
        type G2SubmsgMessageType<'this> = &'this ScalarType where Self: 'this;

        fn g2_submsg_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::G2SubmsgMessageType<'this>> {
            ::std::option::Option::Some(&self.g2_submsg)
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField5<ScalarType>
    where
        ScalarType: self::_puroro_root::oneofs3::_puroro_traits::SubmsgTrait,
        Self: super::_puroro_traits::MsgTrait,
        for<'a> <Self as super::_puroro_traits::MsgTrait>::G2SubmsgMessageType<'a>:
            ::puroro::internal::se::SerMessageToIoWrite,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::Message<
                    <Self as super::_puroro_traits::MsgTrait>::G2SubmsgMessageType<'_>,
                >,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g2_submsg_opt(self),
                5,
                out,
                true,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField5<ScalarType>
    where
        ScalarType: self::_puroro_root::oneofs3::_puroro_traits::SubmsgTrait,
    {
        fn from(value: ScalarType) -> Self {
            Self { g2_submsg: value }
        }
    }

    impl<ScalarType> ::std::clone::Clone for MsgSingleField5<ScalarType>
    where
        ScalarType: self::_puroro_root::oneofs3::_puroro_traits::SubmsgTrait,
        ScalarType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                g2_submsg: ::std::clone::Clone::clone(&self.g2_submsg),
            }
        }
    }

    pub struct MsgSingleField6<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
    {
        pub g3_int32: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField6<ScalarType> where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField6<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
    {
        type G2SubmsgMessageType<'this> = () where Self: 'this;

        fn g3_int32_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.g3_int32,
            )))
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField6<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
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
                <Self as super::_puroro_traits::MsgTrait>::g3_int32_opt(self),
                6,
                out,
                true,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField6<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
    {
        fn from(value: ScalarType) -> Self {
            Self { g3_int32: value }
        }
    }

    impl<ScalarType> ::std::clone::Clone for MsgSingleField6<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        ScalarType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                g3_int32: ::std::clone::Clone::clone(&self.g3_int32),
            }
        }
    }
    pub struct MsgBumpalo<'bump> {
        _bump: &'bump ::puroro::bumpalo::Bump,
        _bitfield:
            ::puroro::bitvec::array::BitArray<::puroro::bitvec::order::Lsb0, [u32; (5 + 31) / 32]>,
        group_one: ::std::option::Option<
            super::_puroro_nested::msg::_puroro_oneofs::GroupOne<
                ::puroro::internal::Bare<i32>,
                ::puroro::internal::Bare<::puroro::internal::NoAllocBumpString>,
            >,
        >,
        group_two: ::std::option::Option<
            super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<
                ::puroro::internal::Bare<f32>,
                ::puroro::internal::Bare<::puroro::internal::NoAllocBumpString>,
                ::puroro::internal::Bare<
                    ::puroro::internal::NoAllocBumpBox<
                        self::_puroro_root::oneofs3::_puroro_impls::SubmsgBumpalo<'bump>,
                    >,
                >,
            >,
        >,
        group_three: ::std::option::Option<
            super::_puroro_nested::msg::_puroro_oneofs::GroupThree<::puroro::internal::Bare<i32>>,
        >,
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
                group_one: ::std::default::Default::default(),
                group_two: ::std::default::Default::default(),
                group_three: ::std::default::Default::default(),
            }
        }
        pub fn g1_int32_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupOne as E;
            #[allow(unused_imports)]
            use ::std::option::Option::{None, Some};
            if let Some(E::G1Int32(ref val)) = &self.group_one {
                Some(val.clone().inner())
            } else {
                None
            }
        }
        pub fn g1_int32<'this>(&'this self) -> i32 {
            match self.g1_int32_opt() {
                ::std::option::Option::Some(x) => x,
                _ => ::std::default::Default::default(),
            }
        }

        pub fn has_g1_int32(&self) -> bool {
            self.g1_int32_opt().is_some()
        }
        pub fn g1_string_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupOne as E;
            #[allow(unused_imports)]
            use ::std::option::Option::{None, Some};
            if let Some(E::G1String(ref val)) = &self.group_one {
                Some(val)
            } else {
                None
            }
        }
        pub fn g1_string<'this>(&'this self) -> &'this str {
            match self.g1_string_opt() {
                ::std::option::Option::Some(x) => x,
                _ => ::std::default::Default::default(),
            }
        }

        pub fn has_g1_string(&self) -> bool {
            self.g1_string_opt().is_some()
        }
        pub fn g2_f32_opt<'this>(&'this self) -> ::std::option::Option<f32> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
            #[allow(unused_imports)]
            use ::std::option::Option::{None, Some};
            if let Some(E::G2F32(ref val)) = &self.group_two {
                Some(val.clone().inner())
            } else {
                None
            }
        }
        pub fn g2_f32<'this>(&'this self) -> f32 {
            match self.g2_f32_opt() {
                ::std::option::Option::Some(x) => x,
                _ => ::std::default::Default::default(),
            }
        }

        pub fn has_g2_f32(&self) -> bool {
            self.g2_f32_opt().is_some()
        }
        pub fn g2_string_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
            #[allow(unused_imports)]
            use ::std::option::Option::{None, Some};
            if let Some(E::G2String(ref val)) = &self.group_two {
                Some(val)
            } else {
                None
            }
        }
        pub fn g2_string<'this>(&'this self) -> &'this str {
            match self.g2_string_opt() {
                ::std::option::Option::Some(x) => x,
                _ => ::std::default::Default::default(),
            }
        }

        pub fn has_g2_string(&self) -> bool {
            self.g2_string_opt().is_some()
        }
        pub fn g2_submsg_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<
            &'this self::_puroro_root::oneofs3::_puroro_impls::SubmsgBumpalo<'this>,
        > {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
            #[allow(unused_imports)]
            use ::std::option::Option::{None, Some};
            if let Some(E::G2Submsg(ref val)) = &self.group_two {
                Some(val)
            } else {
                None
            }
        }
        pub fn g2_submsg<'this>(
            &'this self,
        ) -> ::std::option::Option<
            &'this self::_puroro_root::oneofs3::_puroro_impls::SubmsgBumpalo<'this>,
        > {
            self.g2_submsg_opt()
        }

        pub fn has_g2_submsg(&self) -> bool {
            self.g2_submsg_opt().is_some()
        }
        pub fn g3_int32_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupThree as E;
            #[allow(unused_imports)]
            use ::std::option::Option::{None, Some};
            if let Some(E::G3Int32(ref val)) = &self.group_three {
                Some(val.clone().inner())
            } else {
                None
            }
        }
        pub fn g3_int32<'this>(&'this self) -> i32 {
            match self.g3_int32_opt() {
                ::std::option::Option::Some(x) => x,
                _ => ::std::default::Default::default(),
            }
        }

        pub fn has_g3_int32(&self) -> bool {
            self.g3_int32_opt().is_some()
        }
        pub fn group_one(
            &self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<i32, &'_ str>> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupOne as E;
            #[allow(unused_imports)]
            use ::std::option::Option::{None, Some};
            match &self.group_one {
                Some(E::G1Int32(ref val)) => Some(E::G1Int32(val.clone().inner())),
                Some(E::G1String(ref val)) => Some(E::G1String(val)),
                #[allow(unreachable_patterns)]
                _ => None,
            }
        }
        pub fn group_two(
            &self,
        ) -> Option<
            super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<
                f32,
                &'_ str,
                &'_ self::_puroro_root::oneofs3::_puroro_impls::SubmsgBumpalo<'_>,
            >,
        > {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
            #[allow(unused_imports)]
            use ::std::option::Option::{None, Some};
            match &self.group_two {
                Some(E::G2F32(ref val)) => Some(E::G2F32(val.clone().inner())),
                Some(E::G2String(ref val)) => Some(E::G2String(val)),
                Some(E::G2Submsg(ref val)) => Some(E::G2Submsg(val)),
                #[allow(unreachable_patterns)]
                _ => None,
            }
        }
        pub fn group_three(
            &self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree<i32>> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupThree as E;
            #[allow(unused_imports)]
            use ::std::option::Option::{None, Some};
            match &self.group_three {
                Some(E::G3Int32(ref val)) => Some(E::G3Int32(val.clone().inner())),
                #[allow(unreachable_patterns)]
                _ => None,
            }
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
        fn g1_int32_opt<'this>(&'this self) -> Option<i32> {
            <Self>::g1_int32_opt(self)
        }
        fn g1_string_opt<'this>(&'this self) -> Option<&'this str> {
            <Self>::g1_string_opt(self)
        }
        fn g2_f32_opt<'this>(&'this self) -> Option<f32> {
            <Self>::g2_f32_opt(self)
        }
        fn g2_string_opt<'this>(&'this self) -> Option<&'this str> {
            <Self>::g2_string_opt(self)
        }
        type G2SubmsgMessageType<'this> = &'this self::_puroro_root::oneofs3::_puroro_impls::SubmsgBumpalo<'this> where Self: 'this;
        fn g2_submsg_opt<'this>(&'this self) -> Option<Self::G2SubmsgMessageType<'this>> {
            <Self>::g2_submsg_opt(self)
        }
        fn g3_int32_opt<'this>(&'this self) -> Option<i32> {
            <Self>::g3_int32_opt(self)
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
                    use super::_puroro_nested::msg::_puroro_oneofs::GroupOne as E;
                    if !matches!(&self.group_one, ::std::option::Option::Some(E::G1Int32(_))) {
                        self.group_one = ::std::option::Option::Some(E::G1Int32(
                            ::puroro::internal::BumpDefault::default_in(self._bump),
                        ));
                    }
                    let field_value_mut_ref = match &mut self.group_one {
                        ::std::option::Option::Some(E::G1Int32(v)) => v,
                        _ => unreachable!(),
                    };
                    DeserFieldFromBytesIter::<
                    ::puroro::tags::OneofField, ::puroro::tags::Int32,
                >::deser_field(field_value_mut_ref, data, self._bump)
                }
                2 => {
                    use super::_puroro_nested::msg::_puroro_oneofs::GroupOne as E;
                    if !matches!(&self.group_one, ::std::option::Option::Some(E::G1String(_))) {
                        self.group_one = ::std::option::Option::Some(E::G1String(
                            ::puroro::internal::BumpDefault::default_in(self._bump),
                        ));
                    }
                    let field_value_mut_ref = match &mut self.group_one {
                        ::std::option::Option::Some(E::G1String(v)) => v,
                        _ => unreachable!(),
                    };
                    DeserFieldFromBytesIter::<
                    ::puroro::tags::OneofField, ::puroro::tags::String,
                >::deser_field(field_value_mut_ref, data, self._bump)
                }
                3 => {
                    use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
                    if !matches!(&self.group_two, ::std::option::Option::Some(E::G2F32(_))) {
                        self.group_two = ::std::option::Option::Some(E::G2F32(
                            ::puroro::internal::BumpDefault::default_in(self._bump),
                        ));
                    }
                    let field_value_mut_ref = match &mut self.group_two {
                        ::std::option::Option::Some(E::G2F32(v)) => v,
                        _ => unreachable!(),
                    };
                    DeserFieldFromBytesIter::<
                    ::puroro::tags::OneofField, ::puroro::tags::Float,
                >::deser_field(field_value_mut_ref, data, self._bump)
                }
                4 => {
                    use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
                    if !matches!(&self.group_two, ::std::option::Option::Some(E::G2String(_))) {
                        self.group_two = ::std::option::Option::Some(E::G2String(
                            ::puroro::internal::BumpDefault::default_in(self._bump),
                        ));
                    }
                    let field_value_mut_ref = match &mut self.group_two {
                        ::std::option::Option::Some(E::G2String(v)) => v,
                        _ => unreachable!(),
                    };
                    DeserFieldFromBytesIter::<
                    ::puroro::tags::OneofField, ::puroro::tags::String,
                >::deser_field(field_value_mut_ref, data, self._bump)
                }
                5 => {
                    use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
                    if !matches!(&self.group_two, ::std::option::Option::Some(E::G2Submsg(_))) {
                        self.group_two = ::std::option::Option::Some(E::G2Submsg(
                            ::puroro::internal::Bare::new(::puroro::BumpaloMessage::new_in(
                                self._bump,
                            )),
                        ));
                    }
                    let field_value_mut_ref = match &mut self.group_two {
                        ::std::option::Option::Some(E::G2Submsg(v)) => v,
                        _ => unreachable!(),
                    };
                    DeserFieldFromBytesIter::<
                        ::puroro::tags::OneofField,
                        ::puroro::tags::Message<
                            ::puroro::internal::NoAllocBumpBox<
                                self::_puroro_root::oneofs3::_puroro_impls::SubmsgBumpalo<'bump>,
                            >,
                        >,
                    >::deser_field(field_value_mut_ref, data, self._bump)
                }
                6 => {
                    use super::_puroro_nested::msg::_puroro_oneofs::GroupThree as E;
                    if !matches!(
                        &self.group_three,
                        ::std::option::Option::Some(E::G3Int32(_))
                    ) {
                        self.group_three = ::std::option::Option::Some(E::G3Int32(
                            ::puroro::internal::BumpDefault::default_in(self._bump),
                        ));
                    }
                    let field_value_mut_ref = match &mut self.group_three {
                        ::std::option::Option::Some(E::G3Int32(v)) => v,
                        _ => unreachable!(),
                    };
                    DeserFieldFromBytesIter::<
                    ::puroro::tags::OneofField, ::puroro::tags::Int32,
                >::deser_field(field_value_mut_ref, data, self._bump)
                }

                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }

    impl<'bump> ::puroro::internal::se::SerMessageToIoWrite for MsgBumpalo<'bump>
    where
        Self: super::_puroro_traits::MsgTrait,
        for<'a> <Self as super::_puroro_traits::MsgTrait>::G2SubmsgMessageType<'a>:
            ::puroro::internal::se::SerMessageToIoWrite,
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
                true,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g1_string_opt(self),
                2,
                out,
                true,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::Float,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g2_f32_opt(self),
                3,
                out,
                true,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g2_string_opt(self),
                4,
                out,
                true,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::Message<
                    <Self as super::_puroro_traits::MsgTrait>::G2SubmsgMessageType<'_>,
                >,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g2_submsg_opt(self),
                5,
                out,
                true,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g3_int32_opt(self),
                6,
                out,
                true,
            )?;
            ::std::result::Result::Ok(())
        }
    }
    pub struct MsgBuilder<T>(T);

    impl<T> MsgBuilder<T>
    where
        T: MsgTrait,
    {
        pub fn append_g1_int32<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField1<ScalarType>)>
        where
            ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        {
            MsgBuilder((self.0, MsgSingleField1 { g1_int32: value }))
        }

        pub fn append_g1_string<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField2<ScalarType>)>
        where
            ScalarType: ::std::convert::AsRef<str>,
        {
            MsgBuilder((self.0, MsgSingleField2 { g1_string: value }))
        }

        pub fn append_g2_f32<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField3<ScalarType>)>
        where
            ScalarType: ::std::convert::Into<f32> + ::std::clone::Clone,
        {
            MsgBuilder((self.0, MsgSingleField3 { g2_f32: value }))
        }

        pub fn append_g2_string<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField4<ScalarType>)>
        where
            ScalarType: ::std::convert::AsRef<str>,
        {
            MsgBuilder((self.0, MsgSingleField4 { g2_string: value }))
        }

        pub fn append_g2_submsg<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField5<ScalarType>)>
        where
            ScalarType: self::_puroro_root::oneofs3::_puroro_traits::SubmsgTrait,
        {
            MsgBuilder((self.0, MsgSingleField5 { g2_submsg: value }))
        }

        pub fn append_g3_int32<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField6<ScalarType>)>
        where
            ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        {
            MsgBuilder((self.0, MsgSingleField6 { g3_int32: value }))
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
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.i32_unlabeled,
            )))
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
                true,
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
    pub struct SubmsgBumpalo<'bump> {
        _bump: &'bump ::puroro::bumpalo::Bump,
        _bitfield:
            ::puroro::bitvec::array::BitArray<::puroro::bitvec::order::Lsb0, [u32; (0 + 31) / 32]>,
        i32_unlabeled: ::puroro::internal::Bare<i32>,
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
        pub fn clear_i32_unlabeled(&mut self) {
            self.i32_unlabeled = ::std::default::Default::default();
        }
        pub fn i32_unlabeled_mut<'this>(&'this mut self) -> &'this mut i32 {
            if !self.has_i32_unlabeled() {
                self.i32_unlabeled = ::std::default::Default::default();
            }
            &mut self.i32_unlabeled
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
            1 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::Int32
                >::deser_field(&mut self.i32_unlabeled, data, self._bump)
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
                true,
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

    pub trait MsgTrait {
        fn g1_int32<'this>(&'this self) -> i32 {
            self.g1_int32_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_g1_int32<'this>(&'this self) -> bool {
            self.g1_int32_opt().is_some()
        }

        fn g1_int32_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::None
        }
        fn g1_string<'this>(&'this self) -> &'this str {
            self.g1_string_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_g1_string<'this>(&'this self) -> bool {
            self.g1_string_opt().is_some()
        }

        fn g1_string_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::None
        }
        fn g2_f32<'this>(&'this self) -> f32 {
            self.g2_f32_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_g2_f32<'this>(&'this self) -> bool {
            self.g2_f32_opt().is_some()
        }

        fn g2_f32_opt<'this>(&'this self) -> ::std::option::Option<f32> {
            ::std::option::Option::None
        }
        fn g2_string<'this>(&'this self) -> &'this str {
            self.g2_string_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_g2_string<'this>(&'this self) -> bool {
            self.g2_string_opt().is_some()
        }

        fn g2_string_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::None
        }
        type G2SubmsgMessageType<'this>: self::_puroro_root::oneofs3::_puroro_traits::SubmsgTrait
        where
            Self: 'this;
        fn g2_submsg<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::G2SubmsgMessageType<'this>> {
            self.g2_submsg_opt()
        }
        fn has_g2_submsg<'this>(&'this self) -> bool {
            self.g2_submsg_opt().is_some()
        }

        fn g2_submsg_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::G2SubmsgMessageType<'this>> {
            ::std::option::Option::None
        }
        fn g3_int32<'this>(&'this self) -> i32 {
            self.g3_int32_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_g3_int32<'this>(&'this self) -> bool {
            self.g3_int32_opt().is_some()
        }

        fn g3_int32_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::None
        }
        fn group_one<'this>(
            &'this self,
        ) -> ::std::option::Option<
            super::_puroro_nested::msg::_puroro_oneofs::GroupOne<i32, &'this str>,
        > {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupOne as E;
            if let ::std::option::Option::Some(val) = self.g1_int32_opt() {
                return ::std::option::Option::Some(E::G1Int32(val));
            }
            if let ::std::option::Option::Some(val) = self.g1_string_opt() {
                return ::std::option::Option::Some(E::G1String(val));
            }

            ::std::option::Option::None
        }
        fn group_two<'this>(
            &'this self,
        ) -> ::std::option::Option<
            super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<
                f32,
                &'this str,
                Self::G2SubmsgMessageType<'this>,
            >,
        > {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
            if let ::std::option::Option::Some(val) = self.g2_f32_opt() {
                return ::std::option::Option::Some(E::G2F32(val));
            }
            if let ::std::option::Option::Some(val) = self.g2_string_opt() {
                return ::std::option::Option::Some(E::G2String(val));
            }
            if let ::std::option::Option::Some(val) = self.g2_submsg_opt() {
                return ::std::option::Option::Some(E::G2Submsg(val));
            }

            ::std::option::Option::None
        }
        fn group_three<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree<i32>>
        {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupThree as E;
            if let ::std::option::Option::Some(val) = self.g3_int32_opt() {
                return ::std::option::Option::Some(E::G3Int32(val));
            }

            ::std::option::Option::None
        }
    }

    macro_rules! msg_delegate {
        ($ty:ty) => {
            fn g1_int32_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).g1_int32_opt()
            }

            fn g1_string_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                (**self).g1_string_opt()
            }

            fn g2_f32_opt<'this>(&'this self) -> ::std::option::Option<f32> {
                (**self).g2_f32_opt()
            }

            fn g2_string_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                (**self).g2_string_opt()
            }
            type G2SubmsgMessageType<'this> = <$ty>::G2SubmsgMessageType<'this> where Self: 'this;

            fn g2_submsg_opt<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::G2SubmsgMessageType<'this>> {
                (**self).g2_submsg_opt()
            }

            fn g3_int32_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).g3_int32_opt()
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
        type G2SubmsgMessageType<'this> = () where Self: 'this;
    }
    impl<T, U> MsgTrait for (T, U)
    where
        T: MsgTrait,
        U: MsgTrait,
    {
        fn g1_int32_opt<'this>(&'this self) -> Option<i32> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupOne as E;
            if let ::std::option::Option::Some(E::G1Int32(val)) = self.group_one() {
                ::std::option::Option::Some(val)
            } else {
                ::std::option::Option::None
            }
        }

        fn g1_string_opt<'this>(&'this self) -> Option<&'this str> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupOne as E;
            if let ::std::option::Option::Some(E::G1String(val)) = self.group_one() {
                ::std::option::Option::Some(val)
            } else {
                ::std::option::Option::None
            }
        }

        fn g2_f32_opt<'this>(&'this self) -> Option<f32> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
            if let ::std::option::Option::Some(E::G2F32(val)) = self.group_two() {
                ::std::option::Option::Some(val)
            } else {
                ::std::option::Option::None
            }
        }

        fn g2_string_opt<'this>(&'this self) -> Option<&'this str> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
            if let ::std::option::Option::Some(E::G2String(val)) = self.group_two() {
                ::std::option::Option::Some(val)
            } else {
                ::std::option::Option::None
            }
        }
        type G2SubmsgMessageType<'this> = (
            ::std::option::Option<<T as MsgTrait>::G2SubmsgMessageType<'this>>,
            ::std::option::Option<<U as MsgTrait>::G2SubmsgMessageType<'this>>,
        ) where Self: 'this;

        fn g2_submsg_opt<'this>(&'this self) -> Option<Self::G2SubmsgMessageType<'this>> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
            if let ::std::option::Option::Some(E::G2Submsg(val)) = self.group_two() {
                ::std::option::Option::Some(val)
            } else {
                ::std::option::Option::None
            }
        }

        fn g3_int32_opt<'this>(&'this self) -> Option<i32> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupThree as E;
            if let ::std::option::Option::Some(E::G3Int32(val)) = self.group_three() {
                ::std::option::Option::Some(val)
            } else {
                ::std::option::Option::None
            }
        }
        fn group_one<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<i32, &'this str>> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupOne as E;
            Some(match (self.0.group_one(), self.1.group_one()) {
                (None, None) => {
                    return None;
                }
                (_, Some(E::G1Int32(right))) => E::G1Int32(right),
                (_, Some(E::G1String(right))) => E::G1String(right),
                (Some(E::G1Int32(left)), None) => E::G1Int32(left),
                (Some(E::G1String(left)), None) => E::G1String(left),
            })
        }
        fn group_two<'this>(
            &'this self,
        ) -> Option<
            super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<
                f32,
                &'this str,
                Self::G2SubmsgMessageType<'this>,
            >,
        > {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
            Some(match (self.0.group_two(), self.1.group_two()) {
                (None, None) => {
                    return None;
                }
                (Some(E::G2Submsg(left)), Some(E::G2Submsg(right))) => {
                    E::G2Submsg((Some(left), Some(right)))
                }
                (_, Some(E::G2F32(right))) => E::G2F32(right),
                (_, Some(E::G2String(right))) => E::G2String(right),
                (_, Some(E::G2Submsg(right))) => E::G2Submsg((None, Some(right))),
                (Some(E::G2F32(left)), None) => E::G2F32(left),
                (Some(E::G2String(left)), None) => E::G2String(left),
                (Some(E::G2Submsg(left)), None) => E::G2Submsg((Some(left), None)),
            })
        }
        fn group_three<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree<i32>> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupThree as E;
            Some(match (self.0.group_three(), self.1.group_three()) {
                (None, None) => {
                    return None;
                }
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
        type G2SubmsgMessageType<'this> = ::puroro::Either<
            <T as MsgTrait>::G2SubmsgMessageType<'this>,
            <U as MsgTrait>::G2SubmsgMessageType<'this>,
        > where Self: 'this;
    }
    impl<T> MsgTrait for ::std::option::Option<T>
    where
        T: MsgTrait,
    {
        fn g1_int32_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().and_then(|msg| msg.g1_int32_opt())
        }

        fn g1_string_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().and_then(|msg| msg.g1_string_opt())
        }

        fn g2_f32_opt<'this>(&'this self) -> ::std::option::Option<f32> {
            self.as_ref().and_then(|msg| msg.g2_f32_opt())
        }

        fn g2_string_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().and_then(|msg| msg.g2_string_opt())
        }
        type G2SubmsgMessageType<'this> = T::G2SubmsgMessageType<'this> where Self: 'this;

        fn g2_submsg_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::G2SubmsgMessageType<'this>> {
            self.as_ref().and_then(|msg| msg.g2_submsg_opt())
        }

        fn g3_int32_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().and_then(|msg| msg.g3_int32_opt())
        }
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

        pub use _puroro_oneofs::*;
        pub mod _puroro_oneofs {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }

            pub enum GroupOne<G1Int32, G1String> {
                G1Int32(G1Int32),
                G1String(G1String),
            }
            impl<G1Int32, G1String> GroupOne<G1Int32, G1String> {
                pub fn g1_int32(self) -> ::std::option::Option<G1Int32> {
                    match self {
                        Self::G1Int32(v) => ::std::option::Option::Some(v),
                        #[allow(unreachable_patterns)]
                        _ => None,
                    }
                }
                pub fn g1_string(self) -> ::std::option::Option<G1String> {
                    match self {
                        Self::G1String(v) => ::std::option::Option::Some(v),
                        #[allow(unreachable_patterns)]
                        _ => None,
                    }
                }
            }

            impl<G1Int32, G1String> ::std::fmt::Debug for GroupOne<G1Int32, G1String>
            where
                G1Int32: ::std::fmt::Debug,
                G1String: ::std::fmt::Debug,
            {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    match self {
                        Self::G1Int32(v) => f.debug_tuple("GroupOne::G1Int32").field(&v).finish(),
                        Self::G1String(v) => f.debug_tuple("GroupOne::G1String").field(&v).finish(),
                    }
                }
            }

            impl<G1Int32, G1String> ::std::clone::Clone for GroupOne<G1Int32, G1String>
            where
                G1Int32: ::std::clone::Clone,
                G1String: ::std::clone::Clone,
            {
                fn clone(&self) -> Self {
                    match self {
                        Self::G1Int32(ref v) => {
                            Self::G1Int32(<G1Int32 as ::std::clone::Clone>::clone(v))
                        }
                        Self::G1String(ref v) => {
                            Self::G1String(<G1String as ::std::clone::Clone>::clone(v))
                        }
                    }
                }
            }

            impl<G1Int32, G1String> ::std::cmp::PartialEq for GroupOne<G1Int32, G1String>
            where
                G1Int32: ::std::cmp::PartialEq,
                G1String: ::std::cmp::PartialEq,
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

            pub enum GroupTwo<G2F32, G2String, G2Submsg> {
                G2F32(G2F32),
                G2String(G2String),
                G2Submsg(G2Submsg),
            }
            impl<G2F32, G2String, G2Submsg> GroupTwo<G2F32, G2String, G2Submsg> {
                pub fn g2_f32(self) -> ::std::option::Option<G2F32> {
                    match self {
                        Self::G2F32(v) => ::std::option::Option::Some(v),
                        #[allow(unreachable_patterns)]
                        _ => None,
                    }
                }
                pub fn g2_string(self) -> ::std::option::Option<G2String> {
                    match self {
                        Self::G2String(v) => ::std::option::Option::Some(v),
                        #[allow(unreachable_patterns)]
                        _ => None,
                    }
                }
                pub fn g2_submsg(self) -> ::std::option::Option<G2Submsg> {
                    match self {
                        Self::G2Submsg(v) => ::std::option::Option::Some(v),
                        #[allow(unreachable_patterns)]
                        _ => None,
                    }
                }
            }

            impl<G2F32, G2String, G2Submsg> ::std::fmt::Debug for GroupTwo<G2F32, G2String, G2Submsg>
            where
                G2F32: ::std::fmt::Debug,
                G2String: ::std::fmt::Debug,
                G2Submsg: ::std::fmt::Debug,
            {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    match self {
                        Self::G2F32(v) => f.debug_tuple("GroupTwo::G2F32").field(&v).finish(),
                        Self::G2String(v) => f.debug_tuple("GroupTwo::G2String").field(&v).finish(),
                        Self::G2Submsg(v) => f.debug_tuple("GroupTwo::G2Submsg").field(&v).finish(),
                    }
                }
            }

            impl<G2F32, G2String, G2Submsg> ::std::clone::Clone for GroupTwo<G2F32, G2String, G2Submsg>
            where
                G2F32: ::std::clone::Clone,
                G2String: ::std::clone::Clone,
                G2Submsg: ::std::clone::Clone,
            {
                fn clone(&self) -> Self {
                    match self {
                        Self::G2F32(ref v) => Self::G2F32(<G2F32 as ::std::clone::Clone>::clone(v)),
                        Self::G2String(ref v) => {
                            Self::G2String(<G2String as ::std::clone::Clone>::clone(v))
                        }
                        Self::G2Submsg(ref v) => {
                            Self::G2Submsg(<G2Submsg as ::std::clone::Clone>::clone(v))
                        }
                    }
                }
            }

            impl<G2F32, G2String, G2Submsg> ::std::cmp::PartialEq for GroupTwo<G2F32, G2String, G2Submsg>
            where
                G2F32: ::std::cmp::PartialEq,
                G2String: ::std::cmp::PartialEq,
                G2Submsg: ::std::cmp::PartialEq,
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

            pub enum GroupThree<G3Int32> {
                G3Int32(G3Int32),
            }
            impl<G3Int32> GroupThree<G3Int32> {
                pub fn g3_int32(self) -> ::std::option::Option<G3Int32> {
                    match self {
                        Self::G3Int32(v) => ::std::option::Option::Some(v),
                        #[allow(unreachable_patterns)]
                        _ => None,
                    }
                }
            }

            impl<G3Int32> ::std::fmt::Debug for GroupThree<G3Int32>
            where
                G3Int32: ::std::fmt::Debug,
            {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    match self {
                        Self::G3Int32(v) => f.debug_tuple("GroupThree::G3Int32").field(&v).finish(),
                    }
                }
            }

            impl<G3Int32> ::std::clone::Clone for GroupThree<G3Int32>
            where
                G3Int32: ::std::clone::Clone,
            {
                fn clone(&self) -> Self {
                    match self {
                        Self::G3Int32(ref v) => {
                            Self::G3Int32(<G3Int32 as ::std::clone::Clone>::clone(v))
                        }
                    }
                }
            }

            impl<G3Int32> ::std::cmp::PartialEq for GroupThree<G3Int32>
            where
                G3Int32: ::std::cmp::PartialEq,
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
    }
    pub mod submsg {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }
    }
}
