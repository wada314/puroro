// A generated source code by puroro library
// package official_samples3

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

//pub use _puroro_simple_impl::Test1;
//pub use _puroro_simple_impl::Test2;
//pub use _puroro_simple_impl::Test3;
//pub use _puroro_simple_impl::Test4;
pub mod _puroro_simple_impl {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
}

pub use _puroro_impls::*;
pub mod _puroro_impls {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
    use super::_puroro_internal::*;
    use super::_puroro_traits::*;
    pub struct Test1Template<Fields, Shared>
    where
        Fields: Test1TemplateFieldTypes,
    {
        _shared: Shared,
        _phantom: ::std::marker::PhantomData<Fields>,
        a: <Fields as Test1TemplateFieldTypes>::AType,
    }

    impl<Fields, Shared> Test1Template<Fields, Shared>
    where
        Fields: Test1TemplateFieldTypes,
        Self: ::std::default::Default,
    {
        pub fn new() -> Self {
            ::std::default::Default::default()
        }
    }
    impl<Fields, Shared, AllocatorType> ::puroro::NewIn<AllocatorType> for Test1Template<Fields, Shared>
    where
        Fields: Test1TemplateFieldTypes,
        AllocatorType: ::std::convert::Into<Shared>,
        Fields::AType: ::std::default::Default,
    {
        fn new_in(alloc: AllocatorType) -> Self {
            Self {
                _shared: ::std::convert::Into::into(alloc),
                _phantom: ::std::default::Default::default(),
                a: ::std::default::Default::default(),
            }
        }
    }

    impl<Fields, Shared> Test1Template<Fields, Shared>
    where
        Fields: Test1TemplateFieldTypes,
    {
        pub fn new_in<AllocatorType>(alloc: AllocatorType) -> Self
        where
            Self: ::puroro::NewIn<AllocatorType>,
        {
            <Self as ::puroro::NewIn<AllocatorType>>::new_in(alloc)
        }
    }

    pub struct Test1MessageProperties;
    impl ::puroro::internal::MessageProperties for Test1MessageProperties {
        const BITFIELD_OPTIONAL_FIELD_COUNT: usize = 0;
    }

    pub struct Test1FieldProperties<const FIELD_NUMBER: i32>;
    impl ::puroro::internal::FieldProperties for Test1FieldProperties<1> {
        type MessageProperties = self::Test1MessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Unlabeled;
        type TypeTag = ::puroro::tags::Int32;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }

    pub type Test1Simple2 = Test1Template<
        ::puroro::internal::SimpleFields,
        ::puroro::internal::SimpleShared<{ (0 + 31) / 32 }>,
    >;

    impl<Fields, Shared> Test1Template<Fields, Shared>
    where
        Fields: Test1TemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::AType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::Test1FieldProperties<1>,
                Fields::ImplTag,
            >,
    {
        pub fn a(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::AType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::Test1FieldProperties<1>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.a, &self._shared).get()
        }
    }

    impl<Fields, Shared> Test1Template<Fields, Shared>
    where
        Fields: Test1TemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::AType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::Test1FieldProperties<1>,
                Fields::ImplTag,
            >,
    {
        pub fn a_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::AType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::Test1FieldProperties<1>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.a, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> Test1Template<Fields, Shared>
    where
        Fields: Test1TemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::AType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::Test1FieldProperties<1>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_a(&self) -> bool {
            self.a_opt().is_some()
        }
    }

    impl<Fields, Shared> Test1Template<Fields, Shared>
    where
        Fields: Test1TemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::AType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::Test1FieldProperties<1>,
                Fields::ImplTag,
            >,
    {
        pub fn a_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::AType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::Test1FieldProperties<1>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.a, &mut self._shared).get_mut()
        }
    }

    impl<Fields, Shared> ::std::default::Default for Test1Template<Fields, Shared>
    where
        Fields: Test1TemplateFieldTypes,
        Shared: ::std::default::Default,
        Fields::AType: ::std::default::Default,
    {
        fn default() -> Self {
            Self {
                _shared: ::std::default::Default::default(),
                _phantom: ::std::default::Default::default(),
                a: ::std::default::Default::default(),
            }
        }
    }
    pub struct Test2Template<Fields, Shared>
    where
        Fields: Test2TemplateFieldTypes,
    {
        _shared: Shared,
        _phantom: ::std::marker::PhantomData<Fields>,
        b: <Fields as Test2TemplateFieldTypes>::BType,
    }

    impl<Fields, Shared> Test2Template<Fields, Shared>
    where
        Fields: Test2TemplateFieldTypes,
        Self: ::std::default::Default,
    {
        pub fn new() -> Self {
            ::std::default::Default::default()
        }
    }
    impl<Fields, Shared, AllocatorType> ::puroro::NewIn<AllocatorType> for Test2Template<Fields, Shared>
    where
        Fields: Test2TemplateFieldTypes,
        AllocatorType: ::std::convert::Into<Shared>,
        Fields::BType: ::std::default::Default,
    {
        fn new_in(alloc: AllocatorType) -> Self {
            Self {
                _shared: ::std::convert::Into::into(alloc),
                _phantom: ::std::default::Default::default(),
                b: ::std::default::Default::default(),
            }
        }
    }

    impl<Fields, Shared> Test2Template<Fields, Shared>
    where
        Fields: Test2TemplateFieldTypes,
    {
        pub fn new_in<AllocatorType>(alloc: AllocatorType) -> Self
        where
            Self: ::puroro::NewIn<AllocatorType>,
        {
            <Self as ::puroro::NewIn<AllocatorType>>::new_in(alloc)
        }
    }

    pub struct Test2MessageProperties;
    impl ::puroro::internal::MessageProperties for Test2MessageProperties {
        const BITFIELD_OPTIONAL_FIELD_COUNT: usize = 0;
    }

    pub struct Test2FieldProperties<const FIELD_NUMBER: i32>;
    impl ::puroro::internal::FieldProperties for Test2FieldProperties<2> {
        type MessageProperties = self::Test2MessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Unlabeled;
        type TypeTag = ::puroro::tags::String;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = "";
    }

    pub type Test2Simple2 = Test2Template<
        ::puroro::internal::SimpleFields,
        ::puroro::internal::SimpleShared<{ (0 + 31) / 32 }>,
    >;

    impl<Fields, Shared> Test2Template<Fields, Shared>
    where
        Fields: Test2TemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::BType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::Test2FieldProperties<2>,
                Fields::ImplTag,
            >,
    {
        pub fn b(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::BType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::Test2FieldProperties<2>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.b, &self._shared).get()
        }
    }

    impl<Fields, Shared> Test2Template<Fields, Shared>
    where
        Fields: Test2TemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::BType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::Test2FieldProperties<2>,
                Fields::ImplTag,
            >,
    {
        pub fn b_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::BType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::Test2FieldProperties<2>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.b, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> Test2Template<Fields, Shared>
    where
        Fields: Test2TemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::BType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::Test2FieldProperties<2>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_b(&self) -> bool {
            self.b_opt().is_some()
        }
    }

    impl<Fields, Shared> Test2Template<Fields, Shared>
    where
        Fields: Test2TemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::BType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::Test2FieldProperties<2>,
                Fields::ImplTag,
            >,
    {
        pub fn b_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::BType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::Test2FieldProperties<2>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.b, &mut self._shared).get_mut()
        }
    }

    impl<Fields, Shared> ::std::default::Default for Test2Template<Fields, Shared>
    where
        Fields: Test2TemplateFieldTypes,
        Shared: ::std::default::Default,
        Fields::BType: ::std::default::Default,
    {
        fn default() -> Self {
            Self {
                _shared: ::std::default::Default::default(),
                _phantom: ::std::default::Default::default(),
                b: ::std::default::Default::default(),
            }
        }
    }
    pub struct Test3Template<Fields, Shared>
    where
        Fields: Test3TemplateFieldTypes,
    {
        _shared: Shared,
        _phantom: ::std::marker::PhantomData<Fields>,
        c: <Fields as Test3TemplateFieldTypes>::CType,
    }

    impl<Fields, Shared> Test3Template<Fields, Shared>
    where
        Fields: Test3TemplateFieldTypes,
        Self: ::std::default::Default,
    {
        pub fn new() -> Self {
            ::std::default::Default::default()
        }
    }
    impl<Fields, Shared, AllocatorType> ::puroro::NewIn<AllocatorType> for Test3Template<Fields, Shared>
    where
        Fields: Test3TemplateFieldTypes,
        AllocatorType: ::std::convert::Into<Shared>,
        Fields::CType: ::std::default::Default,
    {
        fn new_in(alloc: AllocatorType) -> Self {
            Self {
                _shared: ::std::convert::Into::into(alloc),
                _phantom: ::std::default::Default::default(),
                c: ::std::default::Default::default(),
            }
        }
    }

    impl<Fields, Shared> Test3Template<Fields, Shared>
    where
        Fields: Test3TemplateFieldTypes,
    {
        pub fn new_in<AllocatorType>(alloc: AllocatorType) -> Self
        where
            Self: ::puroro::NewIn<AllocatorType>,
        {
            <Self as ::puroro::NewIn<AllocatorType>>::new_in(alloc)
        }
    }

    pub struct Test3MessageProperties;
    impl ::puroro::internal::MessageProperties for Test3MessageProperties {
        const BITFIELD_OPTIONAL_FIELD_COUNT: usize = 0;
    }

    pub struct Test3FieldProperties<const FIELD_NUMBER: i32>;
    impl ::puroro::internal::FieldProperties for Test3FieldProperties<3> {
        type MessageProperties = self::Test3MessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Unlabeled;
        type TypeTag =
            ::puroro::tags::Message<self::_puroro_root::official_samples3::Test1MessageProperties>;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = ();
    }

    pub type Test3Simple2 = Test3Template<
        ::puroro::internal::SimpleFields,
        ::puroro::internal::SimpleShared<{ (0 + 31) / 32 }>,
    >;

    impl<Fields, Shared> Test3Template<Fields, Shared>
    where
        Fields: Test3TemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::CType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::Test3FieldProperties<3>,
                Fields::ImplTag,
            >,
    {
        pub fn c(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::CType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::Test3FieldProperties<3>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.c, &self._shared).get()
        }
    }

    impl<Fields, Shared> Test3Template<Fields, Shared>
    where
        Fields: Test3TemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::CType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::Test3FieldProperties<3>,
                Fields::ImplTag,
            >,
    {
        pub fn c_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::CType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::Test3FieldProperties<3>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.c, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> Test3Template<Fields, Shared>
    where
        Fields: Test3TemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::CType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::Test3FieldProperties<3>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_c(&self) -> bool {
            self.c_opt().is_some()
        }
    }

    impl<Fields, Shared> Test3Template<Fields, Shared>
    where
        Fields: Test3TemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::CType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::Test3FieldProperties<3>,
                Fields::ImplTag,
            >,
    {
        pub fn c_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::CType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::Test3FieldProperties<3>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.c, &mut self._shared).get_mut()
        }
    }

    impl<Fields, Shared> ::std::default::Default for Test3Template<Fields, Shared>
    where
        Fields: Test3TemplateFieldTypes,
        Shared: ::std::default::Default,
        Fields::CType: ::std::default::Default,
    {
        fn default() -> Self {
            Self {
                _shared: ::std::default::Default::default(),
                _phantom: ::std::default::Default::default(),
                c: ::std::default::Default::default(),
            }
        }
    }
    pub struct Test4Template<Fields, Shared>
    where
        Fields: Test4TemplateFieldTypes,
    {
        _shared: Shared,
        _phantom: ::std::marker::PhantomData<Fields>,
        d: <Fields as Test4TemplateFieldTypes>::DType,
    }

    impl<Fields, Shared> Test4Template<Fields, Shared>
    where
        Fields: Test4TemplateFieldTypes,
        Self: ::std::default::Default,
    {
        pub fn new() -> Self {
            ::std::default::Default::default()
        }
    }
    impl<Fields, Shared, AllocatorType> ::puroro::NewIn<AllocatorType> for Test4Template<Fields, Shared>
    where
        Fields: Test4TemplateFieldTypes,
        AllocatorType: ::std::convert::Into<Shared>,
        Fields::DType: ::std::default::Default,
    {
        fn new_in(alloc: AllocatorType) -> Self {
            Self {
                _shared: ::std::convert::Into::into(alloc),
                _phantom: ::std::default::Default::default(),
                d: ::std::default::Default::default(),
            }
        }
    }

    impl<Fields, Shared> Test4Template<Fields, Shared>
    where
        Fields: Test4TemplateFieldTypes,
    {
        pub fn new_in<AllocatorType>(alloc: AllocatorType) -> Self
        where
            Self: ::puroro::NewIn<AllocatorType>,
        {
            <Self as ::puroro::NewIn<AllocatorType>>::new_in(alloc)
        }
    }

    pub struct Test4MessageProperties;
    impl ::puroro::internal::MessageProperties for Test4MessageProperties {
        const BITFIELD_OPTIONAL_FIELD_COUNT: usize = 0;
    }

    pub struct Test4FieldProperties<const FIELD_NUMBER: i32>;
    impl ::puroro::internal::FieldProperties for Test4FieldProperties<4> {
        type MessageProperties = self::Test4MessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Repeated;
        type TypeTag = ::puroro::tags::Int32;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }

    pub type Test4Simple2 = Test4Template<
        ::puroro::internal::SimpleFields,
        ::puroro::internal::SimpleShared<{ (0 + 31) / 32 }>,
    >;

    impl<Fields, Shared> Test4Template<Fields, Shared>
    where
        Fields: Test4TemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::DType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::Test4FieldProperties<4>,
                Fields::ImplTag,
            >,
    {
        pub fn d(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::DType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::Test4FieldProperties<4>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.d, &self._shared).get()
        }
    }

    impl<Fields, Shared> Test4Template<Fields, Shared>
    where
        Fields: Test4TemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::DType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::Test4FieldProperties<4>,
                Fields::ImplTag,
            >,
    {
        pub fn d_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::DType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::Test4FieldProperties<4>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.d, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> Test4Template<Fields, Shared>
    where
        Fields: Test4TemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::DType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::Test4FieldProperties<4>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_d(&self) -> bool {
            self.d_opt().is_some()
        }
    }

    impl<Fields, Shared> Test4Template<Fields, Shared>
    where
        Fields: Test4TemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::DType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::Test4FieldProperties<4>,
                Fields::ImplTag,
            >,
    {
        pub fn d_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::DType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::Test4FieldProperties<4>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.d, &mut self._shared).get_mut()
        }
    }

    impl<Fields, Shared> ::std::default::Default for Test4Template<Fields, Shared>
    where
        Fields: Test4TemplateFieldTypes,
        Shared: ::std::default::Default,
        Fields::DType: ::std::default::Default,
    {
        fn default() -> Self {
            Self {
                _shared: ::std::default::Default::default(),
                _phantom: ::std::default::Default::default(),
                d: ::std::default::Default::default(),
            }
        }
    }
}
pub mod _puroro_internal {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
    pub trait Test1TemplateFieldTypes {
        type ImplTag;
        type AType;
    }

    impl Test1TemplateFieldTypes for ::puroro::internal::SimpleFields {
        type ImplTag = ::puroro::tags::SimpleImpl;
        type AType = i32;
    }
    pub trait Test2TemplateFieldTypes {
        type ImplTag;
        type BType;
    }

    impl Test2TemplateFieldTypes for ::puroro::internal::SimpleFields {
        type ImplTag = ::puroro::tags::SimpleImpl;
        type BType = ::std::string::String;
    }
    pub trait Test3TemplateFieldTypes {
        type ImplTag;
        type CType;
    }

    impl Test3TemplateFieldTypes for ::puroro::internal::SimpleFields {
        type ImplTag = ::puroro::tags::SimpleImpl;
        type CType = ::std::option::Option<
            ::std::boxed::Box<self::_puroro_root::official_samples3::_puroro_simple_impl::Test1>,
        >;
    }
    pub trait Test4TemplateFieldTypes {
        type ImplTag;
        type DType;
    }

    impl Test4TemplateFieldTypes for ::puroro::internal::SimpleFields {
        type ImplTag = ::puroro::tags::SimpleImpl;
        type DType = ::std::vec::Vec<i32>;
    }
}
pub use _puroro_traits::*;
pub mod _puroro_traits {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
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
