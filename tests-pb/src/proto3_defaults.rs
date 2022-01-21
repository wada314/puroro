// A generated source code by puroro library
// package proto3_defaults

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

//pub use _puroro_simple_impl::Msg;
//pub use _puroro_simple_impl::Submsg;
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
    pub struct MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
    {
        _shared: Shared,
        _phantom: ::std::marker::PhantomData<Fields>,
        i32_unlabeled: <Fields as MsgTemplateFieldTypes>::I32UnlabeledType,
        i32_optional: <Fields as MsgTemplateFieldTypes>::I32OptionalType,
        i32_repeated: <Fields as MsgTemplateFieldTypes>::I32RepeatedType,
        f32_unlabeled: <Fields as MsgTemplateFieldTypes>::F32UnlabeledType,
        string_unlabeled: <Fields as MsgTemplateFieldTypes>::StringUnlabeledType,
        submsg_unlabeled: <Fields as MsgTemplateFieldTypes>::SubmsgUnlabeledType,
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        Self: ::std::default::Default,
    {
        pub fn new() -> Self {
            ::std::default::Default::default()
        }
    }
    impl<Fields, Shared, AllocatorType> ::puroro::NewIn<AllocatorType> for MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        AllocatorType: ::std::convert::Into<Shared>,
        Fields::I32UnlabeledType: ::std::default::Default,
        Fields::I32OptionalType: ::std::default::Default,
        Fields::I32RepeatedType: ::std::default::Default,
        Fields::F32UnlabeledType: ::std::default::Default,
        Fields::StringUnlabeledType: ::std::default::Default,
        Fields::SubmsgUnlabeledType: ::std::default::Default,
    {
        fn new_in(alloc: AllocatorType) -> Self {
            Self {
                _shared: ::std::convert::Into::into(alloc),
                _phantom: ::std::default::Default::default(),
                i32_unlabeled: ::std::default::Default::default(),
                i32_optional: ::std::default::Default::default(),
                i32_repeated: ::std::default::Default::default(),
                f32_unlabeled: ::std::default::Default::default(),
                string_unlabeled: ::std::default::Default::default(),
                submsg_unlabeled: ::std::default::Default::default(),
            }
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
    {
        pub fn new_in<AllocatorType>(alloc: AllocatorType) -> Self
        where
            Self: ::puroro::NewIn<AllocatorType>,
        {
            <Self as ::puroro::NewIn<AllocatorType>>::new_in(alloc)
        }
    }

    pub struct MsgMessageProperties;
    impl ::puroro::internal::MessageProperties for MsgMessageProperties {
        const BITFIELD_OPTIONAL_FIELD_COUNT: usize = 1;
    }

    pub struct MsgFieldProperties<const FIELD_NUMBER: i32>;
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<1> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Unlabeled;
        type TypeTag = ::puroro::tags::Int32;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<2> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Int32;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<3> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Repeated;
        type TypeTag = ::puroro::tags::Int32;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<4> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Unlabeled;
        type TypeTag = ::puroro::tags::Float;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            0.0f32;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<5> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Unlabeled;
        type TypeTag = ::puroro::tags::String;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = "";
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<6> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Unlabeled;
        type TypeTag =
            ::puroro::tags::Message<self::_puroro_root::proto3_defaults::SubmsgMessageProperties>;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = ();
    }

    pub type MsgSimple2 = MsgTemplate<
        ::puroro::internal::SimpleFields,
        ::puroro::internal::SimpleShared<{ (1 + 31) / 32 }>,
    >;

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I32UnlabeledType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<1>,
                Fields::ImplTag,
            >,
    {
        pub fn i32_unlabeled(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I32UnlabeledType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<1>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.i32_unlabeled, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I32UnlabeledType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<1>,
                Fields::ImplTag,
            >,
    {
        pub fn i32_unlabeled_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I32UnlabeledType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<1>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.i32_unlabeled, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I32UnlabeledType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<1>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_i32_unlabeled(&self) -> bool {
            self.i32_unlabeled_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::I32UnlabeledType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<1>,
                Fields::ImplTag,
            >,
    {
        pub fn i32_unlabeled_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::I32UnlabeledType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<1>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.i32_unlabeled, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I32OptionalType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<2>,
                Fields::ImplTag,
            >,
    {
        pub fn i32_optional(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I32OptionalType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<2>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.i32_optional, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I32OptionalType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<2>,
                Fields::ImplTag,
            >,
    {
        pub fn i32_optional_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I32OptionalType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<2>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.i32_optional, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I32OptionalType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<2>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_i32_optional(&self) -> bool {
            self.i32_optional_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::I32OptionalType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<2>,
                Fields::ImplTag,
            >,
    {
        pub fn i32_optional_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::I32OptionalType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<2>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.i32_optional, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I32RepeatedType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<3>,
                Fields::ImplTag,
            >,
    {
        pub fn i32_repeated(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I32RepeatedType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<3>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.i32_repeated, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I32RepeatedType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<3>,
                Fields::ImplTag,
            >,
    {
        pub fn i32_repeated_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I32RepeatedType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<3>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.i32_repeated, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I32RepeatedType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<3>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_i32_repeated(&self) -> bool {
            self.i32_repeated_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::I32RepeatedType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<3>,
                Fields::ImplTag,
            >,
    {
        pub fn i32_repeated_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::I32RepeatedType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<3>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.i32_repeated, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F32UnlabeledType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<4>,
                Fields::ImplTag,
            >,
    {
        pub fn f32_unlabeled(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::F32UnlabeledType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<4>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.f32_unlabeled, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F32UnlabeledType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<4>,
                Fields::ImplTag,
            >,
    {
        pub fn f32_unlabeled_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::F32UnlabeledType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<4>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.f32_unlabeled, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F32UnlabeledType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<4>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_f32_unlabeled(&self) -> bool {
            self.f32_unlabeled_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::F32UnlabeledType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<4>,
                Fields::ImplTag,
            >,
    {
        pub fn f32_unlabeled_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::F32UnlabeledType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<4>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.f32_unlabeled, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::StringUnlabeledType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<5>,
                Fields::ImplTag,
            >,
    {
        pub fn string_unlabeled(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::StringUnlabeledType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<5>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.string_unlabeled, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::StringUnlabeledType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<5>,
                Fields::ImplTag,
            >,
    {
        pub fn string_unlabeled_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::StringUnlabeledType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<5>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.string_unlabeled, &self._shared)
                .get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::StringUnlabeledType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<5>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_string_unlabeled(&self) -> bool {
            self.string_unlabeled_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::StringUnlabeledType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<5>,
                Fields::ImplTag,
            >,
    {
        pub fn string_unlabeled_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::StringUnlabeledType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<5>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(
                &mut self.string_unlabeled,
                &mut self._shared,
            )
            .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::SubmsgUnlabeledType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<6>,
                Fields::ImplTag,
            >,
    {
        pub fn submsg_unlabeled(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::SubmsgUnlabeledType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<6>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.submsg_unlabeled, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::SubmsgUnlabeledType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<6>,
                Fields::ImplTag,
            >,
    {
        pub fn submsg_unlabeled_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::SubmsgUnlabeledType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<6>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.submsg_unlabeled, &self._shared)
                .get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::SubmsgUnlabeledType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<6>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_submsg_unlabeled(&self) -> bool {
            self.submsg_unlabeled_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::SubmsgUnlabeledType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<6>,
                Fields::ImplTag,
            >,
    {
        pub fn submsg_unlabeled_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::SubmsgUnlabeledType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<6>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(
                &mut self.submsg_unlabeled,
                &mut self._shared,
            )
            .get_mut()
        }
    }

    impl<Fields, Shared> ::std::default::Default for MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        Shared: ::std::default::Default,
        Fields::I32UnlabeledType: ::std::default::Default,
        Fields::I32OptionalType: ::std::default::Default,
        Fields::I32RepeatedType: ::std::default::Default,
        Fields::F32UnlabeledType: ::std::default::Default,
        Fields::StringUnlabeledType: ::std::default::Default,
        Fields::SubmsgUnlabeledType: ::std::default::Default,
    {
        fn default() -> Self {
            Self {
                _shared: ::std::default::Default::default(),
                _phantom: ::std::default::Default::default(),
                i32_unlabeled: ::std::default::Default::default(),
                i32_optional: ::std::default::Default::default(),
                i32_repeated: ::std::default::Default::default(),
                f32_unlabeled: ::std::default::Default::default(),
                string_unlabeled: ::std::default::Default::default(),
                submsg_unlabeled: ::std::default::Default::default(),
            }
        }
    }
    pub struct SubmsgTemplate<Fields, Shared>
    where
        Fields: SubmsgTemplateFieldTypes,
    {
        _shared: Shared,
        _phantom: ::std::marker::PhantomData<Fields>,
        i32_unlabeled: <Fields as SubmsgTemplateFieldTypes>::I32UnlabeledType,
    }

    impl<Fields, Shared> SubmsgTemplate<Fields, Shared>
    where
        Fields: SubmsgTemplateFieldTypes,
        Self: ::std::default::Default,
    {
        pub fn new() -> Self {
            ::std::default::Default::default()
        }
    }
    impl<Fields, Shared, AllocatorType> ::puroro::NewIn<AllocatorType>
        for SubmsgTemplate<Fields, Shared>
    where
        Fields: SubmsgTemplateFieldTypes,
        AllocatorType: ::std::convert::Into<Shared>,
        Fields::I32UnlabeledType: ::std::default::Default,
    {
        fn new_in(alloc: AllocatorType) -> Self {
            Self {
                _shared: ::std::convert::Into::into(alloc),
                _phantom: ::std::default::Default::default(),
                i32_unlabeled: ::std::default::Default::default(),
            }
        }
    }

    impl<Fields, Shared> SubmsgTemplate<Fields, Shared>
    where
        Fields: SubmsgTemplateFieldTypes,
    {
        pub fn new_in<AllocatorType>(alloc: AllocatorType) -> Self
        where
            Self: ::puroro::NewIn<AllocatorType>,
        {
            <Self as ::puroro::NewIn<AllocatorType>>::new_in(alloc)
        }
    }

    pub struct SubmsgMessageProperties;
    impl ::puroro::internal::MessageProperties for SubmsgMessageProperties {
        const BITFIELD_OPTIONAL_FIELD_COUNT: usize = 0;
    }

    pub struct SubmsgFieldProperties<const FIELD_NUMBER: i32>;
    impl ::puroro::internal::FieldProperties for SubmsgFieldProperties<1> {
        type MessageProperties = self::SubmsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Unlabeled;
        type TypeTag = ::puroro::tags::Int32;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }

    pub type SubmsgSimple2 = SubmsgTemplate<
        ::puroro::internal::SimpleFields,
        ::puroro::internal::SimpleShared<{ (0 + 31) / 32 }>,
    >;

    impl<Fields, Shared> SubmsgTemplate<Fields, Shared>
    where
        Fields: SubmsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I32UnlabeledType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::SubmsgFieldProperties<1>,
                Fields::ImplTag,
            >,
    {
        pub fn i32_unlabeled(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I32UnlabeledType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::SubmsgFieldProperties<1>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.i32_unlabeled, &self._shared).get()
        }
    }

    impl<Fields, Shared> SubmsgTemplate<Fields, Shared>
    where
        Fields: SubmsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I32UnlabeledType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::SubmsgFieldProperties<1>,
                Fields::ImplTag,
            >,
    {
        pub fn i32_unlabeled_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I32UnlabeledType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::SubmsgFieldProperties<1>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.i32_unlabeled, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> SubmsgTemplate<Fields, Shared>
    where
        Fields: SubmsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I32UnlabeledType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::SubmsgFieldProperties<1>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_i32_unlabeled(&self) -> bool {
            self.i32_unlabeled_opt().is_some()
        }
    }

    impl<Fields, Shared> SubmsgTemplate<Fields, Shared>
    where
        Fields: SubmsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::I32UnlabeledType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::SubmsgFieldProperties<1>,
                Fields::ImplTag,
            >,
    {
        pub fn i32_unlabeled_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::I32UnlabeledType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::SubmsgFieldProperties<1>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.i32_unlabeled, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> ::std::default::Default for SubmsgTemplate<Fields, Shared>
    where
        Fields: SubmsgTemplateFieldTypes,
        Shared: ::std::default::Default,
        Fields::I32UnlabeledType: ::std::default::Default,
    {
        fn default() -> Self {
            Self {
                _shared: ::std::default::Default::default(),
                _phantom: ::std::default::Default::default(),
                i32_unlabeled: ::std::default::Default::default(),
            }
        }
    }
}
pub mod _puroro_internal {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
    pub trait MsgTemplateFieldTypes {
        type ImplTag;
        type I32UnlabeledType;
        type I32OptionalType;
        type I32RepeatedType;
        type F32UnlabeledType;
        type StringUnlabeledType;
        type SubmsgUnlabeledType;
    }

    impl MsgTemplateFieldTypes for ::puroro::internal::SimpleFields {
        type ImplTag = ::puroro::tags::SimpleImpl;
        type I32UnlabeledType = i32;
        type I32OptionalType = i32;
        type I32RepeatedType = ::std::vec::Vec<i32>;
        type F32UnlabeledType = f32;
        type StringUnlabeledType = ::std::string::String;
        type SubmsgUnlabeledType = ::std::option::Option<
            ::std::boxed::Box<self::_puroro_root::proto3_defaults::_puroro_simple_impl::Submsg>,
        >;
    }
    pub trait SubmsgTemplateFieldTypes {
        type ImplTag;
        type I32UnlabeledType;
    }

    impl SubmsgTemplateFieldTypes for ::puroro::internal::SimpleFields {
        type ImplTag = ::puroro::tags::SimpleImpl;
        type I32UnlabeledType = i32;
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
