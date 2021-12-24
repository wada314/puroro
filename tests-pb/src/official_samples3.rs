// A generated source code by puroro library
// package official_samples3

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub use _puroro_simple_impl::Test1;
pub use _puroro_simple_impl::Test2;
pub use _puroro_simple_impl::Test3;
pub use _puroro_simple_impl::Test4;
pub mod _puroro_simple_impl {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
    pub struct Test1 {
        a: i32,
    }
    impl ::puroro::Message<Test1> for Test1 {}

    impl Test1 {
        pub fn new() -> Self {
            Self {
                a: ::std::default::Default::default(),
            }
        }
        pub fn a_mut(&mut self) -> &mut i32 {
            &mut self.a
        }
    }

    impl super::_puroro_traits::Test1Trait for Test1 {
        fn a_opt<'this>(&'this self) -> Option<i32> {
            if self.a == ::std::default::Default::default() {
                ::std::option::Option::None
            } else {
                ::std::option::Option::Some(self.a.clone())
            }
        }
    }

    impl ::puroro::MessageRepresentativeImpl for Test1 {}

    impl ::puroro::internal::de::DeserMessageFromBytesIter for Test1 {
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
            >::deser_field(&mut self.a, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::internal::se::SerMessageToIoWrite for Test1
    where
        Self: super::_puroro_traits::Test1Trait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::Test1Trait>::a_opt(self),
                1,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::default::Default for Test1 {
        fn default() -> Self {
            Self::new()
        }
    }

    impl ::std::fmt::Debug for Test1
    where
        Self: super::_puroro_traits::Test1Trait,
    {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.debug_struct("Test1")
                .field("a", &<Self as super::_puroro_traits::Test1Trait>::a(self))
                .finish()
        }
    }

    impl ::std::clone::Clone for Test1 {
        fn clone(&self) -> Self {
            Self {
                a: ::std::clone::Clone::clone(&self.a),
            }
        }
    }

    impl ::std::cmp::PartialEq for Test1 {
        fn eq(&self, rhs: &Self) -> bool {
            self.a == rhs.a && true
        }
    }
    pub struct Test2 {
        b: ::std::string::String,
    }
    impl ::puroro::Message<Test2> for Test2 {}

    impl Test2 {
        pub fn new() -> Self {
            Self {
                b: ::std::default::Default::default(),
            }
        }
        pub fn b_mut(&mut self) -> &mut ::std::string::String {
            &mut self.b
        }
    }

    impl super::_puroro_traits::Test2Trait for Test2 {
        type Field2ScalarGetterType<'this>
        where
            Self: 'this,
        = &'this ::std::string::String;
        fn b_opt<'this>(&'this self) -> Option<Self::Field2ScalarGetterType<'this>> {
            if self.b.is_empty() {
                ::std::option::Option::None
            } else {
                ::std::option::Option::Some(&self.b)
            }
        }
        fn b_default_value(&self) -> Self::Field2ScalarGetterType<'_> {
            static DEFAULT_VALUE: ::puroro::once_cell::sync::Lazy<::std::string::String> =
                ::puroro::once_cell::sync::Lazy::new(|| {
                    ::std::convert::From::<&str>::from(::std::default::Default::default())
                });

            &DEFAULT_VALUE
        }
    }

    impl ::puroro::MessageRepresentativeImpl for Test2 {}

    impl ::puroro::internal::de::DeserMessageFromBytesIter for Test2 {
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
            2 => DeserFieldFromBytesIter::<
                ::puroro::tags::Unlabeled, ::puroro::tags::String
            >::deser_field(&mut self.b, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::internal::se::SerMessageToIoWrite for Test2
    where
        Self: super::_puroro_traits::Test2Trait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::Test2Trait>::b_opt(self),
                2,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::default::Default for Test2 {
        fn default() -> Self {
            Self::new()
        }
    }

    impl ::std::fmt::Debug for Test2
    where
        Self: super::_puroro_traits::Test2Trait,
    {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.debug_struct("Test2")
                .field("b", &<Self as super::_puroro_traits::Test2Trait>::b(self))
                .finish()
        }
    }

    impl ::std::clone::Clone for Test2 {
        fn clone(&self) -> Self {
            Self {
                b: ::std::clone::Clone::clone(&self.b),
            }
        }
    }

    impl ::std::cmp::PartialEq for Test2 {
        fn eq(&self, rhs: &Self) -> bool {
            self.b == rhs.b && true
        }
    }
    pub struct Test3 {
        c: ::std::option::Option<
            ::std::boxed::Box<self::_puroro_root::official_samples3::_puroro_simple_impl::Test1>,
        >,
    }
    impl ::puroro::Message<Test3> for Test3 {}

    impl Test3 {
        pub fn new() -> Self {
            Self {
                c: ::std::default::Default::default(),
            }
        }
        pub fn c_mut(
            &mut self,
        ) -> &mut ::std::option::Option<
            ::std::boxed::Box<self::_puroro_root::official_samples3::_puroro_simple_impl::Test1>,
        > {
            &mut self.c
        }
    }

    impl super::_puroro_traits::Test3Trait for Test3 {
        type Field3ScalarGetterType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::official_samples3::_puroro_simple_impl::Test1;
        fn c_opt<'this>(&'this self) -> Option<Self::Field3ScalarGetterType<'this>> {
            self.c.as_ref().map(|v| v.as_ref())
        }
        fn c_default_value(&self) -> Self::Field3ScalarGetterType<'_> {
            static DEFAULT_VALUE: ::puroro::once_cell::sync::Lazy<
                self::_puroro_root::official_samples3::_puroro_simple_impl::Test1,
            > = ::puroro::once_cell::sync::Lazy::new(|| ::std::default::Default::default());

            &DEFAULT_VALUE
        }
    }

    impl ::puroro::MessageRepresentativeImpl for Test3 {}

    impl ::puroro::internal::de::DeserMessageFromBytesIter for Test3 {
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
                3 => DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled,
                    ::puroro::tags::Message<
                        ::std::boxed::Box<
                            self::_puroro_root::official_samples3::_puroro_simple_impl::Test1,
                        >,
                    >,
                >::deser_field(&mut self.c, data),

                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }

    impl ::puroro::internal::se::SerMessageToIoWrite for Test3
    where
        Self: super::_puroro_traits::Test3Trait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Message<
                    <Self as super::_puroro_traits::Test3Trait>::Field3ScalarGetterType<'_>,
                >,
            >::ser_field(
                <Self as super::_puroro_traits::Test3Trait>::c_opt(self),
                3,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::default::Default for Test3 {
        fn default() -> Self {
            Self::new()
        }
    }

    impl ::std::fmt::Debug for Test3
    where
        Self: super::_puroro_traits::Test3Trait,
    {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.debug_struct("Test3")
                .field("c", &<Self as super::_puroro_traits::Test3Trait>::c(self))
                .finish()
        }
    }

    impl ::std::clone::Clone for Test3 {
        fn clone(&self) -> Self {
            Self {
                c: ::std::clone::Clone::clone(&self.c),
            }
        }
    }

    impl ::std::cmp::PartialEq for Test3 {
        fn eq(&self, rhs: &Self) -> bool {
            self.c == rhs.c && true
        }
    }
    pub struct Test4 {
        d: ::std::vec::Vec<i32>,
    }
    impl ::puroro::Message<Test4> for Test4 {}

    impl Test4 {
        pub fn new() -> Self {
            Self {
                d: ::std::default::Default::default(),
            }
        }
        pub fn d_mut(&mut self) -> &mut ::std::vec::Vec<i32> {
            &mut self.d
        }
    }

    impl super::_puroro_traits::Test4Trait for Test4 {
        type Field4RepeatedType<'this> =
            ::puroro::CloneThenIntoRepeatedField<'this, ::std::vec::Vec<i32>, i32, i32>;

        fn d<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.d)
        }
    }

    impl ::puroro::MessageRepresentativeImpl for Test4 {}

    impl ::puroro::internal::de::DeserMessageFromBytesIter for Test4 {
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
            4 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Int32
            >::deser_field(&mut self.d, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::internal::se::SerMessageToIoWrite for Test4
    where
        Self: super::_puroro_traits::Test4Trait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Int32,
            >::ser_field(<Self as super::_puroro_traits::Test4Trait>::d(self), 4, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::default::Default for Test4 {
        fn default() -> Self {
            Self::new()
        }
    }

    impl ::std::fmt::Debug for Test4
    where
        Self: super::_puroro_traits::Test4Trait,
    {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.debug_struct("Test4")
                .field(
                    "d",
                    &<Self as super::_puroro_traits::Test4Trait>::d(self)
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>(),
                )
                .finish()
        }
    }

    impl ::std::clone::Clone for Test4 {
        fn clone(&self) -> Self {
            Self {
                d: ::std::clone::Clone::clone(&self.d),
            }
        }
    }

    impl ::std::cmp::PartialEq for Test4 {
        fn eq(&self, rhs: &Self) -> bool {
            self.d == rhs.d && true
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

    pub struct Test1SingleField1<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub a: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Test1> for Test1SingleField1<ScalarType> where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::Test1Trait for Test1SingleField1<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn a_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.a,
            )))
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for Test1SingleField1<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        Self: super::_puroro_traits::Test1Trait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::Test1Trait>::a_opt(self),
                1,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for Test1SingleField1<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self { a: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct Test2SingleField2<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub b: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Test2> for Test2SingleField2<ScalarType> where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::Test2Trait for Test2SingleField2<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn b_opt<'this>(&'this self) -> ::std::option::Option<Self::Field2ScalarGetterType<'this>> {
            ::std::option::Option::Some(self.b.as_ref())
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for Test2SingleField2<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        Self: super::_puroro_traits::Test2Trait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::Test2Trait>::b_opt(self),
                2,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for Test2SingleField2<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self { b: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct Test3SingleField3<ScalarType>
    where
        ScalarType: self::_puroro_root::official_samples3::_puroro_traits::Test1Trait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub c: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Test3> for Test3SingleField3<ScalarType> where
        ScalarType: self::_puroro_root::official_samples3::_puroro_traits::Test1Trait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::Test3Trait for Test3SingleField3<ScalarType>
    where
        ScalarType: self::_puroro_root::official_samples3::_puroro_traits::Test1Trait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field3MessageType<'this>
        where
            Self: 'this,
        = &'this ScalarType;

        fn c_opt<'this>(&'this self) -> ::std::option::Option<Self::Field3ScalarGetterType<'this>> {
            ::std::option::Option::Some(&self.c)
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for Test3SingleField3<ScalarType>
    where
        ScalarType: self::_puroro_root::official_samples3::_puroro_traits::Test1Trait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        Self: super::_puroro_traits::Test3Trait,
        for<'a> <Self as super::_puroro_traits::Test3Trait>::Field3MessageType<'a>:
            ::puroro::internal::se::SerMessageToIoWrite,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Message<
                    <Self as super::_puroro_traits::Test3Trait>::Field3ScalarGetterType<'_>,
                >,
            >::ser_field(
                <Self as super::_puroro_traits::Test3Trait>::c_opt(self),
                3,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for Test3SingleField3<ScalarType>
    where
        ScalarType: self::_puroro_root::official_samples3::_puroro_traits::Test1Trait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self { c: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct Test4SingleField4<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub d: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::Test4>
        for Test4SingleField4<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::Test4Trait
        for Test4SingleField4<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::CloneThenIntoRepeatedField<'this, RepeatedType, ScalarType, i32>;

        fn d<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.d)
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::se::SerMessageToIoWrite
        for Test4SingleField4<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        Self: super::_puroro_traits::Test4Trait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Int32,
            >::ser_field(<Self as super::_puroro_traits::Test4Trait>::d(self), 4, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for Test4SingleField4<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self { d: value }
        }
    }
}
pub use _puroro_traits::*;
pub mod _puroro_traits {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }

    pub trait Test1Trait {
        fn a<'this>(&'this self) -> i32 {
            self.a_opt().unwrap_or(::std::default::Default::default())
        }

        fn has_a<'this>(&'this self) -> bool {
            self.a_opt().is_some()
        }
        fn a_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::None
        }
    }

    macro_rules! test1_delegate {
        ($ty:ty) => {
            fn a_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).a_opt()
            }
        };
    }

    impl<T> Test1Trait for &'_ T
    where
        T: Test1Trait,
    {
        test1_delegate!(T);
    }

    impl<T> Test1Trait for &'_ mut T
    where
        T: Test1Trait,
    {
        test1_delegate!(T);
    }

    impl<T> Test1Trait for ::std::boxed::Box<T>
    where
        T: Test1Trait,
    {
        test1_delegate!(T);
    }

    impl<'bump, T> Test1Trait for ::puroro::bumpalo::boxed::Box<'bump, T>
    where
        T: Test1Trait,
    {
        test1_delegate!(T);
    }

    impl<T> Test1Trait for ::puroro::BumpaloOwned<T>
    where
        T: Test1Trait,
    {
        test1_delegate!(T);
    }
    impl Test1Trait for () {}
    impl<T> Test1Trait for ::std::option::Option<T>
    where
        T: Test1Trait,
    {
        fn a_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().and_then(|msg| msg.a_opt())
        }
    }
    impl<T, U> Test1Trait for (T, U)
    where
        T: Test1Trait,
        U: Test1Trait,
    {
        fn a_opt<'this>(&'this self) -> Option<i32> {
            <U as Test1Trait>::a_opt(&self.1).or_else(|| <T as Test1Trait>::a_opt(&self.0))
        }
    }
    impl<T, U> Test1Trait for ::puroro::Either<T, U>
    where
        T: Test1Trait,
        U: Test1Trait,
    {
        fn a_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().either(
                |t| <T as Test1Trait>::a_opt(t),
                |u| <U as Test1Trait>::a_opt(u),
            )
        }
    }

    pub trait Test2Trait {
        type Field2ScalarGetterType<'this>: ::std::convert::AsRef<str>
        where
            Self: 'this;

        fn b<'this>(&'this self) -> Self::Field2ScalarGetterType<'this> {
            self.b_opt().unwrap_or(self.b_default_value())
        }
        fn b_default_value(&self) -> Self::Field2ScalarGetterType<'_>;

        fn has_b<'this>(&'this self) -> bool {
            self.b_opt().is_some()
        }
        fn b_opt<'this>(&'this self) -> ::std::option::Option<Self::Field2ScalarGetterType<'this>> {
            ::std::option::Option::None
        }
    }

    macro_rules! test2_delegate {
        ($ty:ty) => {
            type Field2ScalarGetterType<'this>
            where
                Self: 'this,
            = <$ty as Test2Trait>::Field2ScalarGetterType<'this>;
            fn b_opt<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field2ScalarGetterType<'this>> {
                (**self).b_opt()
            }
            fn b_default_value(&self) -> <$ty as Test2Trait>::Field2ScalarGetterType<'_> {
                <$ty as Test2Trait>::b_default_value(self)
            }
        };
    }

    impl<T> Test2Trait for &'_ T
    where
        T: Test2Trait,
    {
        test2_delegate!(T);
    }

    impl<T> Test2Trait for &'_ mut T
    where
        T: Test2Trait,
    {
        test2_delegate!(T);
    }

    impl<T> Test2Trait for ::std::boxed::Box<T>
    where
        T: Test2Trait,
    {
        test2_delegate!(T);
    }

    impl<'bump, T> Test2Trait for ::puroro::bumpalo::boxed::Box<'bump, T>
    where
        T: Test2Trait,
    {
        test2_delegate!(T);
    }

    impl<T> Test2Trait for ::puroro::BumpaloOwned<T>
    where
        T: Test2Trait,
    {
        test2_delegate!(T);
    }
    impl Test2Trait for () {
        type Field2ScalarGetterType<'this> = &'this str;
        fn b_default_value(&self) -> Self::Field2ScalarGetterType<'_> {
            ::std::default::Default::default()
        }
    }
    impl<T> Test2Trait for ::std::option::Option<T>
    where
        T: Test2Trait,
    {
        type Field2ScalarGetterType<'this>
        where
            Self: 'this,
        = ::puroro::Either<T::Field2ScalarGetterType<'this>, &'this str>;
        fn b_opt<'this>(&'this self) -> ::std::option::Option<Self::Field2ScalarGetterType<'this>> {
            self.as_ref()
                .and_then(|msg| msg.b_opt().map(|val| ::puroro::Either::Left(val)))
        }
        fn b_default_value(&self) -> Self::Field2ScalarGetterType<'_> {
            ::puroro::Either::Right(::std::default::Default::default())
        }
    }
    impl<T, U> Test2Trait for (T, U)
    where
        T: Test2Trait,
        U: Test2Trait,
    {
        type Field2ScalarGetterType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as Test2Trait>::Field2ScalarGetterType<'this>,
            <U as Test2Trait>::Field2ScalarGetterType<'this>,
        >;
        fn b_opt<'this>(&'this self) -> Option<Self::Field2ScalarGetterType<'this>> {
            <U as Test2Trait>::b_opt(&self.1)
                .map(|u| ::puroro::Either::Right(u))
                .or_else(|| <T as Test2Trait>::b_opt(&self.0).map(|t| ::puroro::Either::Left(t)))
        }
        fn b_default_value(&self) -> Self::Field2ScalarGetterType<'_> {
            ::puroro::Either::Right(self.1.b_default_value())
        }
    }
    impl<T, U> Test2Trait for ::puroro::Either<T, U>
    where
        T: Test2Trait,
        U: Test2Trait,
    {
        type Field2ScalarGetterType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as Test2Trait>::Field2ScalarGetterType<'this>,
            <U as Test2Trait>::Field2ScalarGetterType<'this>,
        >;
        fn b_opt<'this>(&'this self) -> ::std::option::Option<Self::Field2ScalarGetterType<'this>> {
            self.as_ref().either(
                |t| <T as Test2Trait>::b_opt(t).map(|t| ::puroro::Either::Left(t)),
                |u| <U as Test2Trait>::b_opt(u).map(|u| ::puroro::Either::Right(u)),
            )
        }
        fn b_default_value(&self) -> Self::Field2ScalarGetterType<'_> {
            self.as_ref()
                .map_left(|t| <T as Test2Trait>::b_default_value(t))
                .map_right(|u| <U as Test2Trait>::b_default_value(u))
        }
    }

    pub trait Test3Trait {
        type Field3ScalarGetterType<'this>: self::_puroro_root::official_samples3::_puroro_traits::Test1Trait
            where Self: 'this;

        fn c<'this>(&'this self) -> Self::Field3ScalarGetterType<'this> {
            self.c_opt().unwrap_or(self.c_default_value())
        }
        fn c_default_value(&self) -> Self::Field3ScalarGetterType<'_>;

        fn has_c<'this>(&'this self) -> bool {
            self.c_opt().is_some()
        }
        fn c_opt<'this>(&'this self) -> ::std::option::Option<Self::Field3ScalarGetterType<'this>> {
            ::std::option::Option::None
        }
    }

    macro_rules! test3_delegate {
        ($ty:ty) => {
            type Field3ScalarGetterType<'this>
            where
                Self: 'this,
            = <$ty as Test3Trait>::Field3ScalarGetterType<'this>;
            fn c_opt<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field3ScalarGetterType<'this>> {
                (**self).c_opt()
            }
            fn c_default_value(&self) -> <$ty as Test3Trait>::Field3ScalarGetterType<'_> {
                <$ty as Test3Trait>::c_default_value(self)
            }
        };
    }

    impl<T> Test3Trait for &'_ T
    where
        T: Test3Trait,
    {
        test3_delegate!(T);
    }

    impl<T> Test3Trait for &'_ mut T
    where
        T: Test3Trait,
    {
        test3_delegate!(T);
    }

    impl<T> Test3Trait for ::std::boxed::Box<T>
    where
        T: Test3Trait,
    {
        test3_delegate!(T);
    }

    impl<'bump, T> Test3Trait for ::puroro::bumpalo::boxed::Box<'bump, T>
    where
        T: Test3Trait,
    {
        test3_delegate!(T);
    }

    impl<T> Test3Trait for ::puroro::BumpaloOwned<T>
    where
        T: Test3Trait,
    {
        test3_delegate!(T);
    }
    impl Test3Trait for () {
        type Field3ScalarGetterType<'this> = ();
        fn c_default_value(&self) -> Self::Field3ScalarGetterType<'_> {
            ::std::default::Default::default()
        }
    }
    impl<T> Test3Trait for ::std::option::Option<T>
    where
        T: Test3Trait,
    {
        type Field3ScalarGetterType<'this>
        where
            Self: 'this,
        = ::puroro::Either<T::Field3ScalarGetterType<'this>, ()>;
        fn c_opt<'this>(&'this self) -> ::std::option::Option<Self::Field3ScalarGetterType<'this>> {
            self.as_ref()
                .and_then(|msg| msg.c_opt().map(|val| ::puroro::Either::Left(val)))
        }
        fn c_default_value(&self) -> Self::Field3ScalarGetterType<'_> {
            ::puroro::Either::Right(::std::default::Default::default())
        }
    }
    impl<T, U> Test3Trait for (T, U)
    where
        T: Test3Trait,
        U: Test3Trait,
    {
        type Field3ScalarGetterType<'this>
        where
            Self: 'this,
        = (
            ::std::option::Option<<T as Test3Trait>::Field3ScalarGetterType<'this>>,
            ::std::option::Option<<U as Test3Trait>::Field3ScalarGetterType<'this>>,
        );
        fn c_opt<'this>(&'this self) -> Option<Self::Field3ScalarGetterType<'this>> {
            match (
                <T as Test3Trait>::c_opt(&self.0),
                <U as Test3Trait>::c_opt(&self.1),
            ) {
                (None, None) => None,
                (Some(t), None) => Some((Some(t), None)),
                (None, Some(u)) => Some((None, Some(u))),
                (Some(t), Some(u)) => Some((Some(t), Some(u))),
            }
        }
        fn c_default_value(&self) -> Self::Field3ScalarGetterType<'_> {
            (::std::option::Option::None, ::std::option::Option::None)
        }
    }
    impl<T, U> Test3Trait for ::puroro::Either<T, U>
    where
        T: Test3Trait,
        U: Test3Trait,
    {
        type Field3ScalarGetterType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as Test3Trait>::Field3ScalarGetterType<'this>,
            <U as Test3Trait>::Field3ScalarGetterType<'this>,
        >;
        fn c_opt<'this>(&'this self) -> ::std::option::Option<Self::Field3ScalarGetterType<'this>> {
            self.as_ref().either(
                |t| <T as Test3Trait>::c_opt(t).map(|t| ::puroro::Either::Left(t)),
                |u| <U as Test3Trait>::c_opt(u).map(|u| ::puroro::Either::Right(u)),
            )
        }
        fn c_default_value(&self) -> Self::Field3ScalarGetterType<'_> {
            self.as_ref()
                .map_left(|t| <T as Test3Trait>::c_default_value(t))
                .map_right(|u| <U as Test3Trait>::c_default_value(u))
        }
    }

    pub trait Test4Trait {
        type Field4RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = i32>
        where
            Self: 'this;
        fn d<'this>(&'this self) -> Self::Field4RepeatedType<'this>;
    }

    macro_rules! test4_delegate {
        ($ty:ty) => {
            type Field4RepeatedType<'this>
            where
                Self: 'this,
            = <$ty as Test4Trait>::Field4RepeatedType<'this>;
            fn d<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
                (**self).d()
            }
        };
    }

    impl<T> Test4Trait for &'_ T
    where
        T: Test4Trait,
    {
        test4_delegate!(T);
    }

    impl<T> Test4Trait for &'_ mut T
    where
        T: Test4Trait,
    {
        test4_delegate!(T);
    }

    impl<T> Test4Trait for ::std::boxed::Box<T>
    where
        T: Test4Trait,
    {
        test4_delegate!(T);
    }

    impl<'bump, T> Test4Trait for ::puroro::bumpalo::boxed::Box<'bump, T>
    where
        T: Test4Trait,
    {
        test4_delegate!(T);
    }

    impl<T> Test4Trait for ::puroro::BumpaloOwned<T>
    where
        T: Test4Trait,
    {
        test4_delegate!(T);
    }
    impl Test4Trait for () {
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn d<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }
    impl<T> Test4Trait for ::std::option::Option<T>
    where
        T: Test4Trait,
    {
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<::std::option::IntoIter<T::Field4RepeatedType<'this>>>;
        fn d<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            self.as_ref().map(|msg| msg.d()).into_iter().flatten()
        }
    }
    impl<T, U> Test4Trait for (T, U)
    where
        T: Test4Trait,
        U: Test4Trait,
    {
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as Test4Trait>::Field4RepeatedType<'this>,
            <U as Test4Trait>::Field4RepeatedType<'this>,
        >;

        fn d<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as Test4Trait>::d(&self.0),
                <U as Test4Trait>::d(&self.1),
            )
        }
    }
    impl<T, U> Test4Trait for ::puroro::Either<T, U>
    where
        T: Test4Trait,
        U: Test4Trait,
    {
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as Test4Trait>::Field4RepeatedType<'this>,
            <U as Test4Trait>::Field4RepeatedType<'this>,
        >;

        fn d<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T as Test4Trait>::d(t))
                    .map_right(|u| <U as Test4Trait>::d(u)),
            )
        }
    }
}
pub use _puroro_nested::*;
pub mod _puroro_nested {
    pub mod test1 {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }
    }
    pub mod test2 {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }
    }
    pub mod test3 {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }
    }
    pub mod test4 {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }
    }
}
