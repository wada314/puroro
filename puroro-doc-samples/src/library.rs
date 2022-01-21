// A generated source code by puroro library
// package library

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

//pub use _puroro_simple_impl::Book;
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
    pub struct BookTemplate<Fields, Shared>
    where
        Fields: BookTemplateFieldTypes,
    {
        _shared: Shared,
        _phantom: ::std::marker::PhantomData<Fields>,
        title: <Fields as BookTemplateFieldTypes>::TitleType,
        num_pages: <Fields as BookTemplateFieldTypes>::NumPagesType,
    }

    impl<Fields, Shared> BookTemplate<Fields, Shared>
    where
        Fields: BookTemplateFieldTypes,
        Self: ::std::default::Default,
    {
        pub fn new() -> Self {
            ::std::default::Default::default()
        }
    }
    impl<Fields, Shared, AllocatorType> ::puroro::NewIn<AllocatorType> for BookTemplate<Fields, Shared>
    where
        Fields: BookTemplateFieldTypes,
        AllocatorType: ::std::convert::Into<Shared>,
        Fields::TitleType: ::std::default::Default,
        Fields::NumPagesType: ::std::default::Default,
    {
        fn new_in(alloc: AllocatorType) -> Self {
            Self {
                _shared: ::std::convert::Into::into(alloc),
                _phantom: ::std::default::Default::default(),
                title: ::std::default::Default::default(),
                num_pages: ::std::default::Default::default(),
            }
        }
    }

    impl<Fields, Shared> BookTemplate<Fields, Shared>
    where
        Fields: BookTemplateFieldTypes,
    {
        pub fn new_in<AllocatorType>(alloc: AllocatorType) -> Self
        where
            Self: ::puroro::NewIn<AllocatorType>,
        {
            <Self as ::puroro::NewIn<AllocatorType>>::new_in(alloc)
        }
    }

    pub struct BookMessageProperties;
    impl ::puroro::internal::MessageProperties for BookMessageProperties {
        const BITFIELD_OPTIONAL_FIELD_COUNT: usize = 0;
    }

    pub struct BookFieldProperties<const FIELD_NUMBER: i32>;
    impl ::puroro::internal::FieldProperties for BookFieldProperties<1> {
        type MessageProperties = self::BookMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Unlabeled;
        type TypeTag = ::puroro::tags::String;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = "";
    }
    impl ::puroro::internal::FieldProperties for BookFieldProperties<2> {
        type MessageProperties = self::BookMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Unlabeled;
        type TypeTag = ::puroro::tags::UInt32;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }

    pub type BookSimple2 = BookTemplate<
        ::puroro::internal::SimpleFields,
        ::puroro::internal::SimpleShared<{ (0 + 31) / 32 }>,
    >;

    impl<Fields, Shared> BookTemplate<Fields, Shared>
    where
        Fields: BookTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::TitleType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::BookFieldProperties<1>,
                Fields::ImplTag,
            >,
    {
        pub fn title(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::TitleType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::BookFieldProperties<1>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.title, &self._shared).get()
        }
    }

    impl<Fields, Shared> BookTemplate<Fields, Shared>
    where
        Fields: BookTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::TitleType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::BookFieldProperties<1>,
                Fields::ImplTag,
            >,
    {
        pub fn title_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::TitleType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::BookFieldProperties<1>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.title, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> BookTemplate<Fields, Shared>
    where
        Fields: BookTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::TitleType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::BookFieldProperties<1>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_title(&self) -> bool {
            self.title_opt().is_some()
        }
    }

    impl<Fields, Shared> BookTemplate<Fields, Shared>
    where
        Fields: BookTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::TitleType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::BookFieldProperties<1>,
                Fields::ImplTag,
            >,
    {
        pub fn title_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::TitleType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::BookFieldProperties<1>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.title, &mut self._shared).get_mut()
        }
    }

    impl<Fields, Shared> BookTemplate<Fields, Shared>
    where
        Fields: BookTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::NumPagesType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::BookFieldProperties<2>,
                Fields::ImplTag,
            >,
    {
        pub fn num_pages(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::NumPagesType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::BookFieldProperties<2>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.num_pages, &self._shared).get()
        }
    }

    impl<Fields, Shared> BookTemplate<Fields, Shared>
    where
        Fields: BookTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::NumPagesType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::BookFieldProperties<2>,
                Fields::ImplTag,
            >,
    {
        pub fn num_pages_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::NumPagesType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::BookFieldProperties<2>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.num_pages, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> BookTemplate<Fields, Shared>
    where
        Fields: BookTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::NumPagesType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::BookFieldProperties<2>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_num_pages(&self) -> bool {
            self.num_pages_opt().is_some()
        }
    }

    impl<Fields, Shared> BookTemplate<Fields, Shared>
    where
        Fields: BookTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::NumPagesType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::BookFieldProperties<2>,
                Fields::ImplTag,
            >,
    {
        pub fn num_pages_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::NumPagesType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::BookFieldProperties<2>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.num_pages, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> ::std::default::Default for BookTemplate<Fields, Shared>
    where
        Fields: BookTemplateFieldTypes,
        Shared: ::std::default::Default,
        Fields::TitleType: ::std::default::Default,
        Fields::NumPagesType: ::std::default::Default,
    {
        fn default() -> Self {
            Self {
                _shared: ::std::default::Default::default(),
                _phantom: ::std::default::Default::default(),
                title: ::std::default::Default::default(),
                num_pages: ::std::default::Default::default(),
            }
        }
    }
}
pub mod _puroro_internal {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
    pub trait BookTemplateFieldTypes {
        type ImplTag;
        type TitleType;
        type NumPagesType;
    }

    impl BookTemplateFieldTypes for ::puroro::internal::SimpleFields {
        type ImplTag = ::puroro::tags::SimpleImpl;
        type TitleType = ::std::string::String;
        type NumPagesType = u32;
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
    pub mod book {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }
    }
}
