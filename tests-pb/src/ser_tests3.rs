// A generated source code by puroro library
// package ser_tests3

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub use _puroro_simple_impl::Msg;
pub mod _puroro_simple_impl {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
    pub struct Msg {
        _bitfield:
            ::puroro::bitvec::array::BitArray<::puroro::bitvec::order::Lsb0, [u32; (0 + 31) / 32]>,
        i32_unlabeled: ::puroro::internal::Bare<i32>,
        i32_repeated: ::std::vec::Vec<i32>,
        float_unlabeled: ::puroro::internal::Bare<f32>,
        float_repeated: ::std::vec::Vec<f32>,
        string_unlabeled: ::puroro::internal::Bare<::std::string::String>,
        string_repeated: ::std::vec::Vec<::std::string::String>,
        submsg_unlabeled: ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_simple_impl::Submsg,
            >,
        >,
        submsg_repeated: ::std::vec::Vec<
            self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_simple_impl::Submsg,
        >,
        enum_unlabeled: ::puroro::internal::Bare<self::_puroro_root::ser_tests3::Enum>,
        enum_repeated: ::std::vec::Vec<self::_puroro_root::ser_tests3::Enum>,
        very_large_field_number: ::puroro::internal::Bare<i32>,
    }
    impl ::puroro::Message<Msg> for Msg {}

    impl Msg {
        pub fn new() -> Self {
            Self {
                _bitfield: ::std::default::Default::default(),
                i32_unlabeled: ::std::default::Default::default(),
                i32_repeated: ::std::default::Default::default(),
                float_unlabeled: ::std::default::Default::default(),
                float_repeated: ::std::default::Default::default(),
                string_unlabeled: ::std::default::Default::default(),
                string_repeated: ::std::default::Default::default(),
                submsg_unlabeled: ::std::default::Default::default(),
                submsg_repeated: ::std::default::Default::default(),
                enum_unlabeled: ::std::default::Default::default(),
                enum_repeated: ::std::default::Default::default(),
                very_large_field_number: ::std::default::Default::default(),
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
        pub fn float_repeated(&self) -> &'_ [f32] {
            &self.float_repeated
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
        pub fn string_repeated(
            &self,
        ) -> &'_ [impl ::std::ops::Deref<Target = str> + ::std::fmt::Debug] {
            &self.string_repeated
        }
        pub fn submsg_unlabeled_opt(
            &self,
        ) -> ::std::option::Option<
            &'_ self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_simple_impl::Submsg,
        > {
            self.submsg_unlabeled.as_deref()
        }

        pub fn has_submsg_unlabeled(&self) -> bool {
            Self::submsg_unlabeled_opt(self).is_some()
        }

        pub fn submsg_unlabeled(
            &self,
        ) -> ::std::option::Option<
            &'_ self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_simple_impl::Submsg,
        > {
            self.submsg_unlabeled_opt()
        }
        pub fn submsg_repeated(
            &self,
        ) -> &'_ [self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_simple_impl::Submsg]
        {
            &self.submsg_repeated
        }
        pub fn enum_unlabeled_opt(
            &self,
        ) -> ::std::option::Option<self::_puroro_root::ser_tests3::Enum> {
            if !::puroro::internal::IsDefault::is_default(&*self.enum_unlabeled) {
                ::std::option::Option::Some(self.enum_unlabeled.inner())
            } else {
                ::std::option::Option::None
            }
        }

        pub fn has_enum_unlabeled(&self) -> bool {
            Self::enum_unlabeled_opt(self).is_some()
        }

        pub fn enum_unlabeled(&self) -> self::_puroro_root::ser_tests3::Enum {
            self.enum_unlabeled_opt()
                .unwrap_or(::std::default::Default::default())
        }
        pub fn enum_repeated(&self) -> &'_ [self::_puroro_root::ser_tests3::Enum] {
            &self.enum_repeated
        }
        pub fn very_large_field_number_opt(&self) -> ::std::option::Option<i32> {
            if !::puroro::internal::IsDefault::is_default(&*self.very_large_field_number) {
                ::std::option::Option::Some(self.very_large_field_number.inner())
            } else {
                ::std::option::Option::None
            }
        }

        pub fn has_very_large_field_number(&self) -> bool {
            Self::very_large_field_number_opt(self).is_some()
        }

        pub fn very_large_field_number(&self) -> i32 {
            self.very_large_field_number_opt()
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
        pub fn float_repeated_mut(&mut self) -> &'_ mut ::std::vec::Vec<f32> {
            &mut self.float_repeated
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
        pub fn string_repeated_mut(&mut self) -> &'_ mut ::std::vec::Vec<::std::string::String> {
            &mut self.string_repeated
        }
        pub fn clear_submsg_unlabeled(&mut self) {
            self.submsg_unlabeled = ::std::default::Default::default();
        }
        pub fn submsg_unlabeled_mut(
            &mut self,
        ) -> &'_ mut self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_simple_impl::Submsg
        {
            if !self.has_submsg_unlabeled() {
                self.submsg_unlabeled = ::std::default::Default::default();
            }
            self.submsg_unlabeled
                .get_or_insert_with(::std::default::Default::default)
        }
        pub fn submsg_repeated_mut(
            &mut self,
        ) -> &'_ mut ::std::vec::Vec<
            self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_simple_impl::Submsg,
        > {
            &mut self.submsg_repeated
        }
        pub fn clear_enum_unlabeled(&mut self) {
            self.enum_unlabeled = ::std::default::Default::default();
        }
        pub fn enum_unlabeled_mut(&mut self) -> &'_ mut self::_puroro_root::ser_tests3::Enum {
            if !self.has_enum_unlabeled() {
                self.enum_unlabeled = ::std::default::Default::default();
            }
            &mut self.enum_unlabeled
        }
        pub fn enum_repeated_mut(
            &mut self,
        ) -> &'_ mut ::std::vec::Vec<self::_puroro_root::ser_tests3::Enum> {
            &mut self.enum_repeated
        }
        pub fn clear_very_large_field_number(&mut self) {
            self.very_large_field_number = ::std::default::Default::default();
        }
        pub fn very_large_field_number_mut(&mut self) -> &'_ mut i32 {
            if !self.has_very_large_field_number() {
                self.very_large_field_number = ::std::default::Default::default();
            }
            &mut self.very_large_field_number
        }
    }

    impl super::_puroro_traits::MsgTrait for Msg {
        fn i32_unlabeled_opt<'this>(&'this self) -> Option<i32> {
            <self::Msg>::i32_unlabeled_opt(self)
        }
        type I32RepeatedRepeatedType<'this> =
            ::puroro::CloneThenIntoRepeatedField<'this, ::std::vec::Vec<i32>, i32, i32>;

        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.i32_repeated)
        }
        fn float_unlabeled_opt<'this>(&'this self) -> Option<f32> {
            <self::Msg>::float_unlabeled_opt(self)
        }
        type FloatRepeatedRepeatedType<'this> =
            ::puroro::CloneThenIntoRepeatedField<'this, ::std::vec::Vec<f32>, f32, f32>;

        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.float_repeated)
        }
        fn string_unlabeled_opt<'this>(&'this self) -> Option<&'this str> {
            <self::Msg>::string_unlabeled_opt(self)
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
        type SubmsgUnlabeledMessageType<'this> = &'this self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_simple_impl::Submsg where Self: 'this;
        fn submsg_unlabeled_opt<'this>(
            &'this self,
        ) -> Option<Self::SubmsgUnlabeledMessageType<'this>> {
            <self::Msg>::submsg_unlabeled_opt(self)
        }
        type SubmsgRepeatedMessageType<'this> = &'this self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_simple_impl::Submsg where Self: 'this;
        type SubmsgRepeatedRepeatedType<'this> =
    &'this [self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_simple_impl::Submsg];

        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            &self.submsg_repeated
        }
        fn enum_unlabeled_opt<'this>(&'this self) -> Option<self::_puroro_root::ser_tests3::Enum> {
            <self::Msg>::enum_unlabeled_opt(self)
        }
        type EnumRepeatedRepeatedType<'this> = ::puroro::CloneThenIntoRepeatedField<
            'this,
            ::std::vec::Vec<self::_puroro_root::ser_tests3::Enum>,
            self::_puroro_root::ser_tests3::Enum,
            self::_puroro_root::ser_tests3::Enum,
        >;

        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.enum_repeated)
        }
        fn very_large_field_number_opt<'this>(&'this self) -> Option<i32> {
            <self::Msg>::very_large_field_number_opt(self)
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
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Int32
                >::deser_field(&mut self.i32_repeated, data)
            }
            3 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::Float
                >::deser_field(&mut self.float_unlabeled, data)
            }
            4 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Float
                >::deser_field(&mut self.float_repeated, data)
            }
            5 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::String
                >::deser_field(&mut self.string_unlabeled, data)
            }
            6 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::String
                >::deser_field(&mut self.string_repeated, data)
            }
            7 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::Message<::std::boxed::Box<self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_simple_impl::Submsg>>
                >::deser_field(&mut self.submsg_unlabeled, data)
            }
            8 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_simple_impl::Submsg>
                >::deser_field(&mut self.submsg_repeated, data)
            }
            9 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::Enum3<self::_puroro_root::ser_tests3::Enum>
                >::deser_field(&mut self.enum_unlabeled, data)
            }
            10 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Enum3<self::_puroro_root::ser_tests3::Enum>
                >::deser_field(&mut self.enum_repeated, data)
            }
            536870911 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::Int32
                >::deser_field(&mut self.very_large_field_number, data)
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
                true,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::i32_repeated(self),
                2,
                out,
                true,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Float,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::float_unlabeled_opt(self),
                3,
                out,
                true,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Float,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::float_repeated(self),
                4,
                out,
                true,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::string_unlabeled_opt(self),
                5,
                out,
                true,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::string_repeated(self),
                6,
                out,
                true,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Message<
                    <Self as super::_puroro_traits::MsgTrait>::SubmsgUnlabeledMessageType<'_>,
                >,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::submsg_unlabeled_opt(self),
                7,
                out,
                true,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    <Self as super::_puroro_traits::MsgTrait>::SubmsgRepeatedMessageType<'_>,
                >,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::submsg_repeated(self),
                8,
                out,
                true,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Enum3<self::_puroro_root::ser_tests3::Enum>,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::enum_unlabeled_opt(self),
                9,
                out,
                true,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Enum3<self::_puroro_root::ser_tests3::Enum>,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::enum_repeated(self),
                10,
                out,
                true,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::very_large_field_number_opt(self),
                536870911,
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
                .field("i32_unlabeled", &self.i32_unlabeled())
                .field("i32_repeated", &self.i32_repeated())
                .field("float_unlabeled", &self.float_unlabeled())
                .field("float_repeated", &self.float_repeated())
                .field("string_unlabeled", &self.string_unlabeled())
                .field("string_repeated", &self.string_repeated())
                .field("submsg_unlabeled", &self.submsg_unlabeled())
                .field("submsg_repeated", &self.submsg_repeated())
                .field("enum_unlabeled", &self.enum_unlabeled())
                .field("enum_repeated", &self.enum_repeated())
                .field("very_large_field_number", &self.very_large_field_number())
                .finish()
        }
    }

    impl ::std::clone::Clone for Msg {
        fn clone(&self) -> Self {
            Self {
                _bitfield: ::std::clone::Clone::clone(&self._bitfield),
                i32_unlabeled: ::std::clone::Clone::clone(&self.i32_unlabeled),
                i32_repeated: ::std::clone::Clone::clone(&self.i32_repeated),
                float_unlabeled: ::std::clone::Clone::clone(&self.float_unlabeled),
                float_repeated: ::std::clone::Clone::clone(&self.float_repeated),
                string_unlabeled: ::std::clone::Clone::clone(&self.string_unlabeled),
                string_repeated: ::std::clone::Clone::clone(&self.string_repeated),
                submsg_unlabeled: ::std::clone::Clone::clone(&self.submsg_unlabeled),
                submsg_repeated: ::std::clone::Clone::clone(&self.submsg_repeated),
                enum_unlabeled: ::std::clone::Clone::clone(&self.enum_unlabeled),
                enum_repeated: ::std::clone::Clone::clone(&self.enum_repeated),
                very_large_field_number: ::std::clone::Clone::clone(&self.very_large_field_number),
            }
        }
    }

    impl ::std::cmp::PartialEq for Msg {
        fn eq(&self, rhs: &Self) -> bool {
            self._bitfield == rhs._bitfield
                && self.i32_unlabeled == rhs.i32_unlabeled
                && self.i32_repeated == rhs.i32_repeated
                && self.float_unlabeled == rhs.float_unlabeled
                && self.float_repeated == rhs.float_repeated
                && self.string_unlabeled == rhs.string_unlabeled
                && self.string_repeated == rhs.string_repeated
                && self.submsg_unlabeled == rhs.submsg_unlabeled
                && self.submsg_repeated == rhs.submsg_repeated
                && self.enum_unlabeled == rhs.enum_unlabeled
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
        type I32RepeatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32> where Self: 'this;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<f32> where Self: 'this;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str> where Self: 'this;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this> = () where Self: 'this;
        type SubmsgRepeatedMessageType<'this> = () where Self: 'this;
        type SubmsgRepeatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::SubmsgRepeatedMessageType<'this>> where Self: 'this;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<self::_puroro_root::ser_tests3::Enum> where Self: 'this;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
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

    pub struct MsgSingleField2<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub i32_repeated: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::Msg>
        for MsgSingleField2<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::MsgTrait
        for MsgSingleField2<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type I32RepeatedRepeatedType<'this> =
    ::puroro::CloneThenIntoRepeatedField<
        'this,
        RepeatedType,
        ScalarType,
        i32
    > where Self: 'this;

        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.i32_repeated)
        }
        type FloatRepeatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<f32> where Self: 'this;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str> where Self: 'this;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this> = () where Self: 'this;
        type SubmsgRepeatedMessageType<'this> = () where Self: 'this;
        type SubmsgRepeatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::SubmsgRepeatedMessageType<'this>> where Self: 'this;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<self::_puroro_root::ser_tests3::Enum> where Self: 'this;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::se::SerMessageToIoWrite
        for MsgSingleField2<ScalarType, RepeatedType>
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
                2,
                out,
                true,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for MsgSingleField2<ScalarType, RepeatedType>
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

    impl<ScalarType, RepeatedType> ::std::clone::Clone for MsgSingleField2<ScalarType, RepeatedType>
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

    pub struct MsgSingleField3<ScalarType>
    where
        ScalarType: ::std::convert::Into<f32> + ::std::clone::Clone,
    {
        pub float_unlabeled: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField3<ScalarType> where
        ScalarType: ::std::convert::Into<f32> + ::std::clone::Clone
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField3<ScalarType>
    where
        ScalarType: ::std::convert::Into<f32> + ::std::clone::Clone,
    {
        type I32RepeatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32> where Self: 'this;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn float_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<f32> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.float_unlabeled,
            )))
        }
        type FloatRepeatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<f32> where Self: 'this;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str> where Self: 'this;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this> = () where Self: 'this;
        type SubmsgRepeatedMessageType<'this> = () where Self: 'this;
        type SubmsgRepeatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::SubmsgRepeatedMessageType<'this>> where Self: 'this;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<self::_puroro_root::ser_tests3::Enum> where Self: 'this;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
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
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Float,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::float_unlabeled_opt(self),
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
            Self {
                float_unlabeled: value,
            }
        }
    }

    impl<ScalarType> ::std::clone::Clone for MsgSingleField3<ScalarType>
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

    pub struct MsgSingleField4<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<f32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub float_repeated: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::Msg>
        for MsgSingleField4<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<f32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::MsgTrait
        for MsgSingleField4<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<f32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type I32RepeatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32> where Self: 'this;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        type FloatRepeatedRepeatedType<'this> =
    ::puroro::CloneThenIntoRepeatedField<
        'this,
        RepeatedType,
        ScalarType,
        f32
    > where Self: 'this;

        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.float_repeated)
        }
        type StringRepeatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str> where Self: 'this;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this> = () where Self: 'this;
        type SubmsgRepeatedMessageType<'this> = () where Self: 'this;
        type SubmsgRepeatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::SubmsgRepeatedMessageType<'this>> where Self: 'this;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<self::_puroro_root::ser_tests3::Enum> where Self: 'this;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::se::SerMessageToIoWrite
        for MsgSingleField4<ScalarType, RepeatedType>
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
                4,
                out,
                true,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for MsgSingleField4<ScalarType, RepeatedType>
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

    impl<ScalarType, RepeatedType> ::std::clone::Clone for MsgSingleField4<ScalarType, RepeatedType>
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

    pub struct MsgSingleField5<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>,
    {
        pub string_unlabeled: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField5<ScalarType> where
        ScalarType: ::std::convert::AsRef<str>
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField5<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>,
    {
        type I32RepeatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32> where Self: 'this;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<f32> where Self: 'this;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn string_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::Some(self.string_unlabeled.as_ref())
        }
        type StringRepeatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str> where Self: 'this;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this> = () where Self: 'this;
        type SubmsgRepeatedMessageType<'this> = () where Self: 'this;
        type SubmsgRepeatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::SubmsgRepeatedMessageType<'this>> where Self: 'this;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<self::_puroro_root::ser_tests3::Enum> where Self: 'this;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField5<ScalarType>
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
                5,
                out,
                true,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField5<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                string_unlabeled: value,
            }
        }
    }

    impl<ScalarType> ::std::clone::Clone for MsgSingleField5<ScalarType>
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

    pub struct MsgSingleField6<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::AsRef<str>,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub string_repeated: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::Msg>
        for MsgSingleField6<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::AsRef<str>,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::MsgTrait
        for MsgSingleField6<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::AsRef<str>,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type I32RepeatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32> where Self: 'this;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<f32> where Self: 'this;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        type StringRepeatedRepeatedType<'this> =
    ::puroro::AsRefRepeatedField<
        'this,
        RepeatedType,
        ScalarType,str,
    > where Self: 'this;

        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::AsRefRepeatedField::new(&self.string_repeated)
        }
        type SubmsgUnlabeledMessageType<'this> = () where Self: 'this;
        type SubmsgRepeatedMessageType<'this> = () where Self: 'this;
        type SubmsgRepeatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::SubmsgRepeatedMessageType<'this>> where Self: 'this;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<self::_puroro_root::ser_tests3::Enum> where Self: 'this;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::se::SerMessageToIoWrite
        for MsgSingleField6<ScalarType, RepeatedType>
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
                6,
                out,
                true,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for MsgSingleField6<ScalarType, RepeatedType>
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

    impl<ScalarType, RepeatedType> ::std::clone::Clone for MsgSingleField6<ScalarType, RepeatedType>
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

    pub struct MsgSingleField7<ScalarType>
    where
        ScalarType:
            self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_traits::SubmsgTrait,
    {
        pub submsg_unlabeled: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField7<ScalarType> where
        ScalarType:
            self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_traits::SubmsgTrait
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField7<ScalarType>
    where
        ScalarType:
            self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_traits::SubmsgTrait,
    {
        type I32RepeatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32> where Self: 'this;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<f32> where Self: 'this;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str> where Self: 'this;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this> = &'this ScalarType where Self: 'this;

        fn submsg_unlabeled_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::SubmsgUnlabeledMessageType<'this>> {
            ::std::option::Option::Some(&self.submsg_unlabeled)
        }
        type SubmsgRepeatedMessageType<'this> = () where Self: 'this;
        type SubmsgRepeatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::SubmsgRepeatedMessageType<'this>> where Self: 'this;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<self::_puroro_root::ser_tests3::Enum> where Self: 'this;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField7<ScalarType>
    where
        ScalarType:
            self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_traits::SubmsgTrait,
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
                7,
                out,
                true,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField7<ScalarType>
    where
        ScalarType:
            self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_traits::SubmsgTrait,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                submsg_unlabeled: value,
            }
        }
    }

    impl<ScalarType> ::std::clone::Clone for MsgSingleField7<ScalarType>
    where
        ScalarType:
            self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_traits::SubmsgTrait,
        ScalarType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                submsg_unlabeled: ::std::clone::Clone::clone(&self.submsg_unlabeled),
            }
        }
    }

    pub struct MsgSingleField8<ScalarType, RepeatedType>
    where
        ScalarType:
            self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_traits::SubmsgTrait,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub submsg_repeated: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::Msg>
        for MsgSingleField8<ScalarType, RepeatedType>
    where
        ScalarType:
            self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_traits::SubmsgTrait,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::MsgTrait
        for MsgSingleField8<ScalarType, RepeatedType>
    where
        ScalarType:
            self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_traits::SubmsgTrait,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type I32RepeatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32> where Self: 'this;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<f32> where Self: 'this;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str> where Self: 'this;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this> = () where Self: 'this;
        type SubmsgRepeatedMessageType<'this> = &'this ScalarType where Self: 'this;

        type SubmsgRepeatedRepeatedType<'this> =
    &'this RepeatedType where Self: 'this;

        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            &self.submsg_repeated
        }
        type EnumRepeatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<self::_puroro_root::ser_tests3::Enum> where Self: 'this;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::se::SerMessageToIoWrite
        for MsgSingleField8<ScalarType, RepeatedType>
    where
        ScalarType:
            self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_traits::SubmsgTrait,
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
                8,
                out,
                true,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for MsgSingleField8<ScalarType, RepeatedType>
    where
        ScalarType:
            self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_traits::SubmsgTrait,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                submsg_repeated: value,
            }
        }
    }

    impl<ScalarType, RepeatedType> ::std::clone::Clone for MsgSingleField8<ScalarType, RepeatedType>
    where
        ScalarType:
            self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_traits::SubmsgTrait,
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

    pub struct MsgSingleField9<ScalarType>
    where
        ScalarType:
            ::std::convert::Into<self::_puroro_root::ser_tests3::Enum> + ::std::clone::Clone,
    {
        pub enum_unlabeled: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField9<ScalarType> where
        ScalarType:
            ::std::convert::Into<self::_puroro_root::ser_tests3::Enum> + ::std::clone::Clone
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField9<ScalarType>
    where
        ScalarType:
            ::std::convert::Into<self::_puroro_root::ser_tests3::Enum> + ::std::clone::Clone,
    {
        type I32RepeatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32> where Self: 'this;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<f32> where Self: 'this;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str> where Self: 'this;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this> = () where Self: 'this;
        type SubmsgRepeatedMessageType<'this> = () where Self: 'this;
        type SubmsgRepeatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::SubmsgRepeatedMessageType<'this>> where Self: 'this;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn enum_unlabeled_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::ser_tests3::Enum> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.enum_unlabeled,
            )))
        }
        type EnumRepeatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<self::_puroro_root::ser_tests3::Enum> where Self: 'this;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField9<ScalarType>
    where
        ScalarType:
            ::std::convert::Into<self::_puroro_root::ser_tests3::Enum> + ::std::clone::Clone,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Enum3<self::_puroro_root::ser_tests3::Enum>,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::enum_unlabeled_opt(self),
                9,
                out,
                true,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField9<ScalarType>
    where
        ScalarType:
            ::std::convert::Into<self::_puroro_root::ser_tests3::Enum> + ::std::clone::Clone,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                enum_unlabeled: value,
            }
        }
    }

    impl<ScalarType> ::std::clone::Clone for MsgSingleField9<ScalarType>
    where
        ScalarType:
            ::std::convert::Into<self::_puroro_root::ser_tests3::Enum> + ::std::clone::Clone,
        ScalarType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                enum_unlabeled: ::std::clone::Clone::clone(&self.enum_unlabeled),
            }
        }
    }

    pub struct MsgSingleField10<ScalarType, RepeatedType>
    where
        ScalarType:
            ::std::convert::Into<self::_puroro_root::ser_tests3::Enum> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub enum_repeated: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::Msg>
        for MsgSingleField10<ScalarType, RepeatedType>
    where
        ScalarType:
            ::std::convert::Into<self::_puroro_root::ser_tests3::Enum> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::MsgTrait
        for MsgSingleField10<ScalarType, RepeatedType>
    where
        ScalarType:
            ::std::convert::Into<self::_puroro_root::ser_tests3::Enum> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type I32RepeatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32> where Self: 'this;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<f32> where Self: 'this;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str> where Self: 'this;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this> = () where Self: 'this;
        type SubmsgRepeatedMessageType<'this> = () where Self: 'this;
        type SubmsgRepeatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::SubmsgRepeatedMessageType<'this>> where Self: 'this;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        type EnumRepeatedRepeatedType<'this> =
    ::puroro::CloneThenIntoRepeatedField<
        'this,
        RepeatedType,
        ScalarType,
        self::_puroro_root::ser_tests3::Enum
    > where Self: 'this;

        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.enum_repeated)
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::se::SerMessageToIoWrite
        for MsgSingleField10<ScalarType, RepeatedType>
    where
        ScalarType:
            ::std::convert::Into<self::_puroro_root::ser_tests3::Enum> + ::std::clone::Clone,
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
                ::puroro::tags::Enum3<self::_puroro_root::ser_tests3::Enum>,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::enum_repeated(self),
                10,
                out,
                true,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for MsgSingleField10<ScalarType, RepeatedType>
    where
        ScalarType:
            ::std::convert::Into<self::_puroro_root::ser_tests3::Enum> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                enum_repeated: value,
            }
        }
    }

    impl<ScalarType, RepeatedType> ::std::clone::Clone for MsgSingleField10<ScalarType, RepeatedType>
    where
        ScalarType:
            ::std::convert::Into<self::_puroro_root::ser_tests3::Enum> + ::std::clone::Clone,
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

    pub struct MsgSingleField536870911<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
    {
        pub very_large_field_number: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField536870911<ScalarType> where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField536870911<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
    {
        type I32RepeatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32> where Self: 'this;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<f32> where Self: 'this;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str> where Self: 'this;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this> = () where Self: 'this;
        type SubmsgRepeatedMessageType<'this> = () where Self: 'this;
        type SubmsgRepeatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::SubmsgRepeatedMessageType<'this>> where Self: 'this;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<self::_puroro_root::ser_tests3::Enum> where Self: 'this;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn very_large_field_number_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.very_large_field_number,
            )))
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField536870911<ScalarType>
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
                <Self as super::_puroro_traits::MsgTrait>::very_large_field_number_opt(self),
                536870911,
                out,
                true,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField536870911<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                very_large_field_number: value,
            }
        }
    }

    impl<ScalarType> ::std::clone::Clone for MsgSingleField536870911<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        ScalarType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                very_large_field_number: ::std::clone::Clone::clone(&self.very_large_field_number),
            }
        }
    }
    pub struct MsgBumpalo<'bump> {
        _bump: &'bump ::puroro::bumpalo::Bump,
        _bitfield:
            ::puroro::bitvec::array::BitArray<::puroro::bitvec::order::Lsb0, [u32; (0 + 31) / 32]>,
        i32_unlabeled: ::puroro::internal::Bare<i32>,
        i32_repeated: ::puroro::internal::NoAllocBumpVec<i32>,
        float_unlabeled: ::puroro::internal::Bare<f32>,
        float_repeated: ::puroro::internal::NoAllocBumpVec<f32>,
        string_unlabeled: ::puroro::internal::Bare<::puroro::internal::NoAllocBumpString>,
        string_repeated: ::puroro::internal::NoAllocBumpVec<::puroro::internal::NoAllocBumpString>,
        submsg_unlabeled: ::std::option::Option<
            ::puroro::internal::NoAllocBumpBox<
                self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_impls::SubmsgBumpalo<
                    'bump,
                >,
            >,
        >,
        submsg_repeated: ::puroro::internal::NoAllocBumpVec<
            self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_impls::SubmsgBumpalo<
                'bump,
            >,
        >,
        enum_unlabeled: ::puroro::internal::Bare<self::_puroro_root::ser_tests3::Enum>,
        enum_repeated: ::puroro::internal::NoAllocBumpVec<self::_puroro_root::ser_tests3::Enum>,
        very_large_field_number: ::puroro::internal::Bare<i32>,
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
                i32_repeated: ::std::default::Default::default(),
                float_unlabeled: ::std::default::Default::default(),
                float_repeated: ::std::default::Default::default(),
                string_unlabeled: ::std::default::Default::default(),
                string_repeated: ::std::default::Default::default(),
                submsg_unlabeled: ::std::default::Default::default(),
                submsg_repeated: ::std::default::Default::default(),
                enum_unlabeled: ::std::default::Default::default(),
                enum_repeated: ::std::default::Default::default(),
                very_large_field_number: ::std::default::Default::default(),
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
        pub fn float_repeated<'this>(&'this self) -> &'this [f32] {
            &self.float_repeated
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
        pub fn string_repeated<'this>(
            &'this self,
        ) -> &'this [impl ::std::ops::Deref<Target = str>] {
            &self.string_repeated
        }
        pub fn submsg_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<&'this self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_impls::SubmsgBumpalo<'this>>{
            self.submsg_unlabeled
                .as_ref()
                .map(|x| unsafe { ::std::mem::transmute(::std::ops::Deref::deref(x)) })
        }
        pub fn submsg_unlabeled<'this>(&'this self) -> ::std::option::Option<&'this self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_impls::SubmsgBumpalo<'this>>{
            self.submsg_unlabeled_opt()
        }

        pub fn has_submsg_unlabeled(&self) -> bool {
            self.submsg_unlabeled_opt().is_some()
        }
        pub fn submsg_repeated<'this>(&'this self) -> &'this[self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_impls::SubmsgBumpalo<'this>]{
            unsafe { self.submsg_repeated.cast_item_unchecked() }
        }
        pub fn enum_unlabeled_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::ser_tests3::Enum> {
            if !::puroro::internal::IsDefault::is_default(&*self.enum_unlabeled) {
                ::std::option::Option::Some(self.enum_unlabeled.inner())
            } else {
                ::std::option::Option::None
            }
        }
        pub fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::ser_tests3::Enum {
            match self.enum_unlabeled_opt() {
                ::std::option::Option::Some(x) => x,
                _ => ::std::default::Default::default(),
            }
        }

        pub fn has_enum_unlabeled(&self) -> bool {
            self.enum_unlabeled_opt().is_some()
        }
        pub fn enum_repeated<'this>(&'this self) -> &'this [self::_puroro_root::ser_tests3::Enum] {
            &self.enum_repeated
        }
        pub fn very_large_field_number_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            if !::puroro::internal::IsDefault::is_default(&*self.very_large_field_number) {
                ::std::option::Option::Some(self.very_large_field_number.inner())
            } else {
                ::std::option::Option::None
            }
        }
        pub fn very_large_field_number<'this>(&'this self) -> i32 {
            match self.very_large_field_number_opt() {
                ::std::option::Option::Some(x) => x,
                _ => ::std::default::Default::default(),
            }
        }

        pub fn has_very_large_field_number(&self) -> bool {
            self.very_large_field_number_opt().is_some()
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
        pub fn float_repeated_mut<'this>(
            &'this mut self,
        ) -> ::puroro::internal::RefMutBumpVec<'bump, 'this, f32> {
            unsafe { self.float_repeated.as_mut_vec_in(self._bump) }
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
        pub fn string_repeated_mut<'this>(
            &'this mut self,
        ) -> ::puroro::internal::AddBumpVecView<'bump, 'this, ::puroro::internal::NoAllocBumpString>
        {
            unsafe { self.string_repeated.as_add_bump_vec_view_in(self._bump) }
        }
        pub fn clear_submsg_unlabeled(&mut self) {
            self.submsg_unlabeled = ::std::default::Default::default();
        }
        pub fn submsg_unlabeled_mut<'this>(&'this mut self) -> &'this mut self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_impls::SubmsgBumpalo<'bump>{
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
        pub fn submsg_repeated_mut<'this>(
            &'this mut self,
        ) -> ::puroro::internal::RefMutBumpVec<
            'bump,
            'this,
            self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_impls::SubmsgBumpalo<
                'bump,
            >,
        > {
            unsafe { self.submsg_repeated.as_mut_vec_in(self._bump) }
        }
        pub fn clear_enum_unlabeled(&mut self) {
            self.enum_unlabeled = ::std::default::Default::default();
        }
        pub fn enum_unlabeled_mut<'this>(
            &'this mut self,
        ) -> &'this mut self::_puroro_root::ser_tests3::Enum {
            if !self.has_enum_unlabeled() {
                self.enum_unlabeled = ::std::default::Default::default();
            }
            &mut self.enum_unlabeled
        }
        pub fn enum_repeated_mut<'this>(
            &'this mut self,
        ) -> ::puroro::internal::RefMutBumpVec<'bump, 'this, self::_puroro_root::ser_tests3::Enum>
        {
            unsafe { self.enum_repeated.as_mut_vec_in(self._bump) }
        }
        pub fn clear_very_large_field_number(&mut self) {
            self.very_large_field_number = ::std::default::Default::default();
        }
        pub fn very_large_field_number_mut<'this>(&'this mut self) -> &'this mut i32 {
            if !self.has_very_large_field_number() {
                self.very_large_field_number = ::std::default::Default::default();
            }
            &mut self.very_large_field_number
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
        type I32RepeatedRepeatedType<'this> =
    ::puroro::CloneThenIntoRepeatedField<
        'this,
        ::puroro::internal::NoAllocBumpVec<i32>,
        i32,
        i32
    > where Self: 'this;

        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.i32_repeated)
        }
        fn float_unlabeled_opt<'this>(&'this self) -> Option<f32> {
            <Self>::float_unlabeled_opt(self)
        }
        type FloatRepeatedRepeatedType<'this> =
    ::puroro::CloneThenIntoRepeatedField<
        'this,
        ::puroro::internal::NoAllocBumpVec<f32>,
        f32,
        f32
    > where Self: 'this;

        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.float_repeated)
        }
        fn string_unlabeled_opt<'this>(&'this self) -> Option<&'this str> {
            <Self>::string_unlabeled_opt(self)
        }
        type StringRepeatedRepeatedType<'this> =
    ::puroro::AsRefRepeatedField<
        'this,
        ::puroro::internal::NoAllocBumpVec<::puroro::internal::NoAllocBumpString>,
        ::puroro::internal::NoAllocBumpString,str,
    > where Self: 'this;

        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::AsRefRepeatedField::new(&self.string_repeated)
        }
        type SubmsgUnlabeledMessageType<'this> = &'this self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_impls::SubmsgBumpalo<'this> where Self: 'this;
        fn submsg_unlabeled_opt<'this>(
            &'this self,
        ) -> Option<Self::SubmsgUnlabeledMessageType<'this>> {
            <Self>::submsg_unlabeled_opt(self)
        }
        type SubmsgRepeatedMessageType<'this> = &'this self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_impls::SubmsgBumpalo<'this> where Self: 'this;
        type SubmsgRepeatedRepeatedType<'this> =
    &'this [self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_impls::SubmsgBumpalo<'this>] where Self: 'this;

        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            unsafe { self.submsg_repeated.cast_item_unchecked() }
        }
        fn enum_unlabeled_opt<'this>(&'this self) -> Option<self::_puroro_root::ser_tests3::Enum> {
            <Self>::enum_unlabeled_opt(self)
        }
        type EnumRepeatedRepeatedType<'this> =
    ::puroro::CloneThenIntoRepeatedField<
        'this,
        ::puroro::internal::NoAllocBumpVec<self::_puroro_root::ser_tests3::Enum>,
        self::_puroro_root::ser_tests3::Enum,
        self::_puroro_root::ser_tests3::Enum
    > where Self: 'this;

        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.enum_repeated)
        }
        fn very_large_field_number_opt<'this>(&'this self) -> Option<i32> {
            <Self>::very_large_field_number_opt(self)
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
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Int32
                >::deser_field(&mut self.i32_repeated, data, self._bump)
            }
            3 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::Float
                >::deser_field(&mut self.float_unlabeled, data, self._bump)
            }
            4 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Float
                >::deser_field(&mut self.float_repeated, data, self._bump)
            }
            5 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::String
                >::deser_field(&mut self.string_unlabeled, data, self._bump)
            }
            6 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::String
                >::deser_field(&mut self.string_repeated, data, self._bump)
            }
            7 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::Message<::puroro::internal::NoAllocBumpBox<self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_impls::SubmsgBumpalo<'bump>>>
                >::deser_field(&mut self.submsg_unlabeled, data, self._bump)
            }
            8 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_impls::SubmsgBumpalo<'bump>>
                >::deser_field(&mut self.submsg_repeated, data, self._bump)
            }
            9 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::Enum3<self::_puroro_root::ser_tests3::Enum>
                >::deser_field(&mut self.enum_unlabeled, data, self._bump)
            }
            10 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Enum3<self::_puroro_root::ser_tests3::Enum>
                >::deser_field(&mut self.enum_repeated, data, self._bump)
            }
            536870911 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::Int32
                >::deser_field(&mut self.very_large_field_number, data, self._bump)
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
                true,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::i32_repeated(self),
                2,
                out,
                true,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Float,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::float_unlabeled_opt(self),
                3,
                out,
                true,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Float,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::float_repeated(self),
                4,
                out,
                true,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::string_unlabeled_opt(self),
                5,
                out,
                true,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::string_repeated(self),
                6,
                out,
                true,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Message<
                    <Self as super::_puroro_traits::MsgTrait>::SubmsgUnlabeledMessageType<'_>,
                >,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::submsg_unlabeled_opt(self),
                7,
                out,
                true,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    <Self as super::_puroro_traits::MsgTrait>::SubmsgRepeatedMessageType<'_>,
                >,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::submsg_repeated(self),
                8,
                out,
                true,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Enum3<self::_puroro_root::ser_tests3::Enum>,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::enum_unlabeled_opt(self),
                9,
                out,
                true,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Enum3<self::_puroro_root::ser_tests3::Enum>,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::enum_repeated(self),
                10,
                out,
                true,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::very_large_field_number_opt(self),
                536870911,
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

        pub fn append_i32_repeated<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> MsgBuilder<(T, MsgSingleField2<ScalarType, RepeatedType>)>
        where
            ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
            for<'a> &'a RepeatedType:
                ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            MsgBuilder((
                self.0,
                MsgSingleField2 {
                    i32_repeated: value,
                },
            ))
        }

        pub fn append_float_unlabeled<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField3<ScalarType>)>
        where
            ScalarType: ::std::convert::Into<f32> + ::std::clone::Clone,
        {
            MsgBuilder((
                self.0,
                MsgSingleField3 {
                    float_unlabeled: value,
                },
            ))
        }

        pub fn append_float_repeated<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> MsgBuilder<(T, MsgSingleField4<ScalarType, RepeatedType>)>
        where
            ScalarType: ::std::convert::Into<f32> + ::std::clone::Clone,
            for<'a> &'a RepeatedType:
                ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            MsgBuilder((
                self.0,
                MsgSingleField4 {
                    float_repeated: value,
                },
            ))
        }

        pub fn append_string_unlabeled<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField5<ScalarType>)>
        where
            ScalarType: ::std::convert::AsRef<str>,
        {
            MsgBuilder((
                self.0,
                MsgSingleField5 {
                    string_unlabeled: value,
                },
            ))
        }

        pub fn append_string_repeated<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> MsgBuilder<(T, MsgSingleField6<ScalarType, RepeatedType>)>
        where
            ScalarType: ::std::convert::AsRef<str>,
            for<'a> &'a RepeatedType:
                ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            MsgBuilder((
                self.0,
                MsgSingleField6 {
                    string_repeated: value,
                },
            ))
        }

        pub fn append_submsg_unlabeled<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField7<ScalarType>)>
        where
            ScalarType:
                self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_traits::SubmsgTrait,
        {
            MsgBuilder((
                self.0,
                MsgSingleField7 {
                    submsg_unlabeled: value,
                },
            ))
        }

        pub fn append_submsg_repeated<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> MsgBuilder<(T, MsgSingleField8<ScalarType, RepeatedType>)>
        where
            ScalarType:
                self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_traits::SubmsgTrait,
            for<'a> &'a RepeatedType:
                ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            MsgBuilder((
                self.0,
                MsgSingleField8 {
                    submsg_repeated: value,
                },
            ))
        }

        pub fn append_enum_unlabeled<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField9<ScalarType>)>
        where
            ScalarType:
                ::std::convert::Into<self::_puroro_root::ser_tests3::Enum> + ::std::clone::Clone,
        {
            MsgBuilder((
                self.0,
                MsgSingleField9 {
                    enum_unlabeled: value,
                },
            ))
        }

        pub fn append_enum_repeated<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> MsgBuilder<(T, MsgSingleField10<ScalarType, RepeatedType>)>
        where
            ScalarType:
                ::std::convert::Into<self::_puroro_root::ser_tests3::Enum> + ::std::clone::Clone,
            for<'a> &'a RepeatedType:
                ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            MsgBuilder((
                self.0,
                MsgSingleField10 {
                    enum_repeated: value,
                },
            ))
        }

        pub fn append_very_large_field_number<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField536870911<ScalarType>)>
        where
            ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        {
            MsgBuilder((
                self.0,
                MsgSingleField536870911 {
                    very_large_field_number: value,
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

        type FloatRepeatedRepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = f32>
        where
            Self: 'this;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this>;
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

        type StringRepeatedRepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = &'this str>
        where
            Self: 'this;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this>;
        type SubmsgUnlabeledMessageType<'this>: self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_traits::SubmsgTrait
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
        type SubmsgRepeatedMessageType<'this>: self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_traits::SubmsgTrait
            where Self: 'this;

        type SubmsgRepeatedRepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::SubmsgRepeatedMessageType<'this>>
        where
            Self: 'this;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this>;
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::ser_tests3::Enum {
            self.enum_unlabeled_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_enum_unlabeled<'this>(&'this self) -> bool {
            self.enum_unlabeled_opt().is_some()
        }

        fn enum_unlabeled_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::ser_tests3::Enum> {
            ::std::option::Option::None
        }

        type EnumRepeatedRepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = self::_puroro_root::ser_tests3::Enum>
        where
            Self: 'this;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this>;
        fn very_large_field_number<'this>(&'this self) -> i32 {
            self.very_large_field_number_opt()
                .unwrap_or_else(::std::default::Default::default)
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

            fn i32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).i32_unlabeled_opt()
            }

            type I32RepeatedRepeatedType<'this> = <$ty>::I32RepeatedRepeatedType<'this> where Self: 'this;
            fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
                (**self).i32_repeated()
            }

            fn float_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<f32> {
                (**self).float_unlabeled_opt()
            }

            type FloatRepeatedRepeatedType<'this> = <$ty>::FloatRepeatedRepeatedType<'this> where Self: 'this;
            fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
                (**self).float_repeated()
            }

            fn string_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                (**self).string_unlabeled_opt()
            }

            type StringRepeatedRepeatedType<'this> = <$ty>::StringRepeatedRepeatedType<'this> where Self: 'this;
            fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
                (**self).string_repeated()
            }
            type SubmsgUnlabeledMessageType<'this> = <$ty>::SubmsgUnlabeledMessageType<'this> where Self: 'this;

            fn submsg_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<Self::SubmsgUnlabeledMessageType<'this>> {
                (**self).submsg_unlabeled_opt()
            }
            type SubmsgRepeatedMessageType<'this> = <$ty>::SubmsgRepeatedMessageType<'this> where Self: 'this;

            type SubmsgRepeatedRepeatedType<'this> = <$ty>::SubmsgRepeatedRepeatedType<'this> where Self: 'this;
            fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
                (**self).submsg_repeated()
            }

            fn enum_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<self::_puroro_root::ser_tests3::Enum> {
                (**self).enum_unlabeled_opt()
            }

            type EnumRepeatedRepeatedType<'this> = <$ty>::EnumRepeatedRepeatedType<'this> where Self: 'this;
            fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
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
        type I32RepeatedRepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<i32>
            where Self: 'this;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type FloatRepeatedRepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<f32>
            where Self: 'this;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type StringRepeatedRepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>
            where Self: 'this;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type SubmsgUnlabeledMessageType<'this> = () where Self: 'this;
        type SubmsgRepeatedMessageType<'this> = () where Self: 'this;
        type SubmsgRepeatedRepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::SubmsgRepeatedMessageType<'this>>
            where Self: 'this;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type EnumRepeatedRepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<self::_puroro_root::ser_tests3::Enum>
            where Self: 'this;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
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
        type I32RepeatedRepeatedType<'this>
            = ::puroro::internal::impls::merged::MergedRepeatedField<
                <T as MsgTrait>::I32RepeatedRepeatedType<'this>,
                <U as MsgTrait>::I32RepeatedRepeatedType<'this>,
            > where Self: 'this;

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
        type FloatRepeatedRepeatedType<'this>
            = ::puroro::internal::impls::merged::MergedRepeatedField<
                <T as MsgTrait>::FloatRepeatedRepeatedType<'this>,
                <U as MsgTrait>::FloatRepeatedRepeatedType<'this>,
            > where Self: 'this;

        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as MsgTrait>::float_repeated(&self.0),
                <U as MsgTrait>::float_repeated(&self.1),
            )
        }

        fn string_unlabeled_opt<'this>(&'this self) -> Option<&'this str> {
            <U as MsgTrait>::string_unlabeled_opt(&self.1)
                .or_else(|| <T as MsgTrait>::string_unlabeled_opt(&self.0))
        }
        type StringRepeatedRepeatedType<'this>
            = ::puroro::internal::impls::merged::MergedRepeatedField<
                <T as MsgTrait>::StringRepeatedRepeatedType<'this>,
                <U as MsgTrait>::StringRepeatedRepeatedType<'this>,
            > where Self: 'this;

        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as MsgTrait>::string_repeated(&self.0),
                <U as MsgTrait>::string_repeated(&self.1),
            )
        }
        type SubmsgUnlabeledMessageType<'this> = (
            ::std::option::Option<<T as MsgTrait>::SubmsgUnlabeledMessageType<'this>>,
            ::std::option::Option<<U as MsgTrait>::SubmsgUnlabeledMessageType<'this>>,
        ) where Self: 'this;

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
        type SubmsgRepeatedMessageType<'this> = ::puroro::Either<
            <T as MsgTrait>::SubmsgRepeatedMessageType<'this>,
            <U as MsgTrait>::SubmsgRepeatedMessageType<'this>,
        > where Self: 'this;
        type SubmsgRepeatedRepeatedType<'this>
            = ::puroro::internal::impls::merged::MergedRepeatedMessageField<
                <T as MsgTrait>::SubmsgRepeatedRepeatedType<'this>,
                <U as MsgTrait>::SubmsgRepeatedRepeatedType<'this>,
            > where Self: 'this;

        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
                <T as MsgTrait>::submsg_repeated(&self.0),
                <U as MsgTrait>::submsg_repeated(&self.1),
            )
        }

        fn enum_unlabeled_opt<'this>(&'this self) -> Option<self::_puroro_root::ser_tests3::Enum> {
            <U as MsgTrait>::enum_unlabeled_opt(&self.1)
                .or_else(|| <T as MsgTrait>::enum_unlabeled_opt(&self.0))
        }
        type EnumRepeatedRepeatedType<'this>
            = ::puroro::internal::impls::merged::MergedRepeatedField<
                <T as MsgTrait>::EnumRepeatedRepeatedType<'this>,
                <U as MsgTrait>::EnumRepeatedRepeatedType<'this>,
            > where Self: 'this;

        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as MsgTrait>::enum_repeated(&self.0),
                <U as MsgTrait>::enum_repeated(&self.1),
            )
        }

        fn very_large_field_number_opt<'this>(&'this self) -> Option<i32> {
            <U as MsgTrait>::very_large_field_number_opt(&self.1)
                .or_else(|| <T as MsgTrait>::very_large_field_number_opt(&self.0))
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
        type I32RepeatedRepeatedType<'this>
            = ::puroro::internal::impls::either::EitherRepeatedField<
                <T as MsgTrait>::I32RepeatedRepeatedType<'this>,
                <U as MsgTrait>::I32RepeatedRepeatedType<'this>,
            > where Self: 'this;

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
        type FloatRepeatedRepeatedType<'this>
            = ::puroro::internal::impls::either::EitherRepeatedField<
                <T as MsgTrait>::FloatRepeatedRepeatedType<'this>,
                <U as MsgTrait>::FloatRepeatedRepeatedType<'this>,
            > where Self: 'this;

        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::float_repeated(t))
                    .map_right(|u| <U as MsgTrait>::float_repeated(u)),
            )
        }
        fn string_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().either(
                |t| <T as MsgTrait>::string_unlabeled_opt(t),
                |u| <U as MsgTrait>::string_unlabeled_opt(u),
            )
        }
        type StringRepeatedRepeatedType<'this>
            = ::puroro::internal::impls::either::EitherRepeatedField<
                <T as MsgTrait>::StringRepeatedRepeatedType<'this>,
                <U as MsgTrait>::StringRepeatedRepeatedType<'this>,
            > where Self: 'this;

        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::string_repeated(t))
                    .map_right(|u| <U as MsgTrait>::string_repeated(u)),
            )
        }
        type SubmsgUnlabeledMessageType<'this> = ::puroro::Either<
            <T as MsgTrait>::SubmsgUnlabeledMessageType<'this>,
            <U as MsgTrait>::SubmsgUnlabeledMessageType<'this>,
        > where Self: 'this;
        fn submsg_unlabeled_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::SubmsgUnlabeledMessageType<'this>> {
            self.as_ref().either(
                |t| <T as MsgTrait>::submsg_unlabeled_opt(t).map(|t| ::puroro::Either::Left(t)),
                |u| <U as MsgTrait>::submsg_unlabeled_opt(u).map(|u| ::puroro::Either::Right(u)),
            )
        }
        type SubmsgRepeatedMessageType<'this> = ::puroro::Either<
            <T as MsgTrait>::SubmsgRepeatedMessageType<'this>,
            <U as MsgTrait>::SubmsgRepeatedMessageType<'this>,
        > where Self: 'this;
        type SubmsgRepeatedRepeatedType<'this>
            = ::puroro::internal::impls::either::EitherRepeatedMessageField<
                <T as MsgTrait>::SubmsgRepeatedRepeatedType<'this>,
                <U as MsgTrait>::SubmsgRepeatedRepeatedType<'this>,
            > where Self: 'this;

        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::submsg_repeated(t))
                    .map_right(|u| <U as MsgTrait>::submsg_repeated(u)),
            )
        }
        fn enum_unlabeled_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::ser_tests3::Enum> {
            self.as_ref().either(
                |t| <T as MsgTrait>::enum_unlabeled_opt(t),
                |u| <U as MsgTrait>::enum_unlabeled_opt(u),
            )
        }
        type EnumRepeatedRepeatedType<'this>
            = ::puroro::internal::impls::either::EitherRepeatedField<
                <T as MsgTrait>::EnumRepeatedRepeatedType<'this>,
                <U as MsgTrait>::EnumRepeatedRepeatedType<'this>,
            > where Self: 'this;

        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::enum_repeated(t))
                    .map_right(|u| <U as MsgTrait>::enum_repeated(u)),
            )
        }
        fn very_large_field_number_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().either(
                |t| <T as MsgTrait>::very_large_field_number_opt(t),
                |u| <U as MsgTrait>::very_large_field_number_opt(u),
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

        type I32RepeatedRepeatedType<'this> =
            ::puroro::internal::impls::option::OptionRepeatedField<
                T::I32RepeatedRepeatedType<'this>
            > where Self: 'this;
        fn i32_repeated<'this>(&'this self) -> Self::I32RepeatedRepeatedType<'this> {
            ::puroro::internal::impls::option::OptionRepeatedField::new(
                self.as_ref().map(|msg| msg.i32_repeated()),
            )
        }
        fn float_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<f32> {
            self.as_ref().and_then(|msg| msg.float_unlabeled_opt())
        }

        type FloatRepeatedRepeatedType<'this> =
            ::puroro::internal::impls::option::OptionRepeatedField<
                T::FloatRepeatedRepeatedType<'this>
            > where Self: 'this;
        fn float_repeated<'this>(&'this self) -> Self::FloatRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::option::OptionRepeatedField::new(
                self.as_ref().map(|msg| msg.float_repeated()),
            )
        }
        fn string_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().and_then(|msg| msg.string_unlabeled_opt())
        }

        type StringRepeatedRepeatedType<'this> =
            ::puroro::internal::impls::option::OptionRepeatedField<
                T::StringRepeatedRepeatedType<'this>
            > where Self: 'this;
        fn string_repeated<'this>(&'this self) -> Self::StringRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::option::OptionRepeatedField::new(
                self.as_ref().map(|msg| msg.string_repeated()),
            )
        }
        type SubmsgUnlabeledMessageType<'this> = T::SubmsgUnlabeledMessageType<'this> where Self: 'this;
        fn submsg_unlabeled_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::SubmsgUnlabeledMessageType<'this>> {
            self.as_ref().and_then(|msg| msg.submsg_unlabeled_opt())
        }
        type SubmsgRepeatedMessageType<'this> = T::SubmsgRepeatedMessageType<'this> where Self: 'this;

        type SubmsgRepeatedRepeatedType<'this> =
            ::puroro::internal::impls::option::OptionRepeatedField<
                T::SubmsgRepeatedRepeatedType<'this>
            > where Self: 'this;
        fn submsg_repeated<'this>(&'this self) -> Self::SubmsgRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::option::OptionRepeatedField::new(
                self.as_ref().map(|msg| msg.submsg_repeated()),
            )
        }
        fn enum_unlabeled_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::ser_tests3::Enum> {
            self.as_ref().and_then(|msg| msg.enum_unlabeled_opt())
        }

        type EnumRepeatedRepeatedType<'this> =
            ::puroro::internal::impls::option::OptionRepeatedField<
                T::EnumRepeatedRepeatedType<'this>
            > where Self: 'this;
        fn enum_repeated<'this>(&'this self) -> Self::EnumRepeatedRepeatedType<'this> {
            ::puroro::internal::impls::option::OptionRepeatedField::new(
                self.as_ref().map(|msg| msg.enum_repeated()),
            )
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
                        1 => DeserFieldFromBytesIter::<
                            ::puroro::tags::Unlabeled,
                            ::puroro::tags::Int32,
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
                    self._bitfield == rhs._bitfield
                        && self.i32_unlabeled == rhs.i32_unlabeled
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
                _bitfield: ::puroro::bitvec::array::BitArray<
                    ::puroro::bitvec::order::Lsb0,
                    [u32; (0 + 31) / 32],
                >,
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
                        1 => DeserFieldFromBytesIter::<
                            ::puroro::tags::Unlabeled,
                            ::puroro::tags::Int32,
                        >::deser_field(
                            &mut self.i32_unlabeled, data, self._bump
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
            pub mod submsg {
                mod _puroro_root {
                    pub use super::super::super::_puroro_root::*;
                }
            }
        }
    }
}
