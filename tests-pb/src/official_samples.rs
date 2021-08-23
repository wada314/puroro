// A generated source code by puroro library
// package official_samples

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub use _puroro_impls::Test1_Simple as Test1;
pub use _puroro_impls::Test2_Simple as Test2;
pub use _puroro_impls::Test3_Simple as Test3;
pub use _puroro_impls::Test4_Simple as Test4;
pub mod _puroro_impls {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }

    #[allow(non_camel_case_types)]
    #[derive(Clone, Default, PartialEq, Debug)]
    pub struct Test1_Simple {
        pub a: ::std::option::Option<i32>,
    }
    impl ::puroro::Message for Test1_Simple {}

    impl super::_puroro_traits::Test1Trait for Test1_Simple {
        fn a<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::clone::Clone::clone(&self.a)
        }
    }

    impl ::puroro::DeserFromBytesIter for Test1_Simple {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for Test1_Simple {
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
                    ::puroro::tags::Optional,
                    ::puroro::tags::Int32,
                >::deser_field(&mut self.a, data),

                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }

    impl ::puroro::SerToIoWrite for Test1_Simple {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >::ser_field(&self.a, 1, out)?;
            ::std::result::Result::Ok(())
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(Clone, Default, PartialEq, Debug)]
    pub struct Test1_Empty;

    impl ::puroro::Message for Test1_Empty {}

    impl super::_puroro_traits::Test1Trait for Test1_Empty {
        fn a<'this>(&'this self) -> ::std::option::Option<i32> {
            None
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(Clone, Default, PartialEq, Debug)]
    pub struct Test2_Simple {
        pub b: ::std::option::Option<::std::string::String>,
    }
    impl ::puroro::Message for Test2_Simple {}

    impl super::_puroro_traits::Test2Trait for Test2_Simple {
        fn b<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            self.b
                .as_ref()
                .map(|v| ::std::borrow::Cow::Borrowed(v.as_ref()))
        }
    }

    impl ::puroro::DeserFromBytesIter for Test2_Simple {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for Test2_Simple {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            match field_number {
                2 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::String,
                >::deser_field(&mut self.b, data),

                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }

    impl ::puroro::SerToIoWrite for Test2_Simple {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.b, 2, out)?;
            ::std::result::Result::Ok(())
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(Clone, Default, PartialEq, Debug)]
    pub struct Test2_Empty;

    impl ::puroro::Message for Test2_Empty {}

    impl super::_puroro_traits::Test2Trait for Test2_Empty {
        fn b<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            None
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(Clone, Default, PartialEq, Debug)]
    pub struct Test3_Simple {
        pub c: ::std::option::Option<
            ::std::boxed::Box<self::_puroro_root::official_samples::_puroro_impls::Test1_Simple>,
        >,
    }
    impl ::puroro::Message for Test3_Simple {}

    impl super::_puroro_traits::Test3Trait for Test3_Simple {
        type Field3MessageType<'this> =
            self::_puroro_root::official_samples::_puroro_impls::Test1_Simple;
        fn c<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field3MessageType<'this>>>
        {
            self.c
                .as_ref()
                .map(|boxed| ::std::borrow::Cow::Borrowed(boxed.as_ref()))
        }
    }

    impl ::puroro::DeserFromBytesIter for Test3_Simple {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for Test3_Simple {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            match field_number {
                3 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::Message<
                        self::_puroro_root::official_samples::_puroro_impls::Test1_Simple,
                    >,
                >::deser_field(&mut self.c, data),

                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }

    impl ::puroro::SerToIoWrite for Test3_Simple {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    self::_puroro_root::official_samples::_puroro_impls::Test1_Simple,
                >,
            >::ser_field(&self.c, 3, out)?;
            ::std::result::Result::Ok(())
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(Clone, Default, PartialEq, Debug)]
    pub struct Test3_Empty;

    impl ::puroro::Message for Test3_Empty {}

    impl super::_puroro_traits::Test3Trait for Test3_Empty {
        type Field3MessageType<'this> =
            self::_puroro_root::official_samples::_puroro_impls::Test1_Simple;
        fn c<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field3MessageType<'this>>>
        {
            None
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(Clone, Default, PartialEq, Debug)]
    pub struct Test4_Simple {
        pub d: ::std::vec::Vec<i32>,
    }
    impl ::puroro::Message for Test4_Simple {}

    impl super::_puroro_traits::Test4Trait for Test4_Simple {
        type Field4RepeatedType<'this> = ::puroro_internal::impls::simple::VecWrapper<'this, i32>;

        fn d<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecWrapper::new(&self.d)
        }
    }

    impl ::puroro::DeserFromBytesIter for Test4_Simple {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for Test4_Simple {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            match field_number {
                4 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated,
                    ::puroro::tags::Int32,
                >::deser_field(&mut self.d, data),

                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }

    impl ::puroro::SerToIoWrite for Test4_Simple {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Int32,
            >::ser_field(&self.d, 4, out)?;
            ::std::result::Result::Ok(())
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(Clone, Default, PartialEq, Debug)]
    pub struct Test4_Empty;

    impl ::puroro::Message for Test4_Empty {}

    impl super::_puroro_traits::Test4Trait for Test4_Empty {
        type Field4RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn d<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }
}
pub use _puroro_traits::*;
pub mod _puroro_traits {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }

    pub trait Test1Trait: ::std::clone::Clone {
        fn a<'this>(&'this self) -> ::std::option::Option<i32>;
    }
    pub trait Test2Trait: ::std::clone::Clone {
        fn b<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>>;
    }
    pub trait Test3Trait: ::std::clone::Clone {
        type Field3MessageType<'this>: 'this + self::_puroro_root::official_samples::_puroro_traits::Test1Trait;
        fn c<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field3MessageType<'this>>>;
    }
    pub trait Test4Trait: ::std::clone::Clone {
        type Field4RepeatedType<'this>: ::puroro::RepeatedField<'this, i32>;

        fn d<'this>(&'this self) -> Self::Field4RepeatedType<'this>;
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
