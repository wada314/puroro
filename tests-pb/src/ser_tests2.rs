// A generated source code by puroro library
// package ser_tests2

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

//pub use _puroro_simple_impl::Msg;
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
        i32_optional: <Fields as MsgTemplateFieldTypes>::I32OptionalType,
        i32_repeated: <Fields as MsgTemplateFieldTypes>::I32RepeatedType,
        float_optional: <Fields as MsgTemplateFieldTypes>::FloatOptionalType,
        float_repeated: <Fields as MsgTemplateFieldTypes>::FloatRepeatedType,
        string_optional: <Fields as MsgTemplateFieldTypes>::StringOptionalType,
        string_repeated: <Fields as MsgTemplateFieldTypes>::StringRepeatedType,
        submsg_optional: <Fields as MsgTemplateFieldTypes>::SubmsgOptionalType,
        submsg_repeated: <Fields as MsgTemplateFieldTypes>::SubmsgRepeatedType,
        enum_optional: <Fields as MsgTemplateFieldTypes>::EnumOptionalType,
        enum_repeated: <Fields as MsgTemplateFieldTypes>::EnumRepeatedType,
        very_large_field_number: <Fields as MsgTemplateFieldTypes>::VeryLargeFieldNumberType,
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
        Fields::I32OptionalType: ::std::default::Default,
        Fields::I32RepeatedType: ::std::default::Default,
        Fields::FloatOptionalType: ::std::default::Default,
        Fields::FloatRepeatedType: ::std::default::Default,
        Fields::StringOptionalType: ::std::default::Default,
        Fields::StringRepeatedType: ::std::default::Default,
        Fields::SubmsgOptionalType: ::std::default::Default,
        Fields::SubmsgRepeatedType: ::std::default::Default,
        Fields::EnumOptionalType: ::std::default::Default,
        Fields::EnumRepeatedType: ::std::default::Default,
        Fields::VeryLargeFieldNumberType: ::std::default::Default,
    {
        fn new_in(alloc: AllocatorType) -> Self {
            Self {
                _shared: ::std::convert::Into::into(alloc),
                _phantom: ::std::default::Default::default(),
                i32_optional: ::std::default::Default::default(),
                i32_repeated: ::std::default::Default::default(),
                float_optional: ::std::default::Default::default(),
                float_repeated: ::std::default::Default::default(),
                string_optional: ::std::default::Default::default(),
                string_repeated: ::std::default::Default::default(),
                submsg_optional: ::std::default::Default::default(),
                submsg_repeated: ::std::default::Default::default(),
                enum_optional: ::std::default::Default::default(),
                enum_repeated: ::std::default::Default::default(),
                very_large_field_number: ::std::default::Default::default(),
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
        const BITFIELD_OPTIONAL_FIELD_COUNT: usize = 5;
    }

    pub struct MsgFieldProperties<const FIELD_NUMBER: i32>;
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<1> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Int32;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<2> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Repeated;
        type TypeTag = ::puroro::tags::Int32;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<3> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 1;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Float;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            0.0f32;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<4> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Repeated;
        type TypeTag = ::puroro::tags::Float;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            0.0f32;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<5> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 2;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::String;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = "";
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<6> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Repeated;
        type TypeTag = ::puroro::tags::String;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = "";
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<7> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Message<
            self::_puroro_root::ser_tests2::_puroro_nested::msg::SubmsgMessageProperties,
        >;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = ();
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<8> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Repeated;
        type TypeTag = ::puroro::tags::Message<
            self::_puroro_root::ser_tests2::_puroro_nested::msg::SubmsgMessageProperties,
        >;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = ();
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<9> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 3;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Enum2<self::_puroro_root::ser_tests2::Enum>;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            self::_puroro_root::ser_tests2::Enum::Zeroth;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<10> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Repeated;
        type TypeTag = ::puroro::tags::Enum2<self::_puroro_root::ser_tests2::Enum>;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            self::_puroro_root::ser_tests2::Enum::Zeroth;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<536870911> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 4;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Int32;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }

    pub type MsgSimple2 = MsgTemplate<
        ::puroro::internal::SimpleFields,
        ::puroro::internal::SimpleShared<{ (5 + 31) / 32 }>,
    >;

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I32OptionalType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<1>,
                Fields::ImplTag,
            >,
    {
        pub fn i32_optional(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I32OptionalType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<1>,
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
                self::MsgFieldProperties<1>,
                Fields::ImplTag,
            >,
    {
        pub fn i32_optional_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I32OptionalType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<1>,
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
                self::MsgFieldProperties<1>,
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
                self::MsgFieldProperties<1>,
                Fields::ImplTag,
            >,
    {
        pub fn i32_optional_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::I32OptionalType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<1>,
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
                self::MsgFieldProperties<2>,
                Fields::ImplTag,
            >,
    {
        pub fn i32_repeated(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I32RepeatedType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<2>,
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
                self::MsgFieldProperties<2>,
                Fields::ImplTag,
            >,
    {
        pub fn i32_repeated_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I32RepeatedType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<2>,
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
                self::MsgFieldProperties<2>,
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
                self::MsgFieldProperties<2>,
                Fields::ImplTag,
            >,
    {
        pub fn i32_repeated_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::I32RepeatedType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<2>,
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
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::FloatOptionalType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<3>,
                Fields::ImplTag,
            >,
    {
        pub fn float_optional(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::FloatOptionalType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<3>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.float_optional, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::FloatOptionalType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<3>,
                Fields::ImplTag,
            >,
    {
        pub fn float_optional_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::FloatOptionalType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<3>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.float_optional, &self._shared)
                .get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::FloatOptionalType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<3>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_float_optional(&self) -> bool {
            self.float_optional_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::FloatOptionalType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<3>,
                Fields::ImplTag,
            >,
    {
        pub fn float_optional_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::FloatOptionalType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<3>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.float_optional, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::FloatRepeatedType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<4>,
                Fields::ImplTag,
            >,
    {
        pub fn float_repeated(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::FloatRepeatedType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<4>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.float_repeated, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::FloatRepeatedType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<4>,
                Fields::ImplTag,
            >,
    {
        pub fn float_repeated_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::FloatRepeatedType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<4>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.float_repeated, &self._shared)
                .get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::FloatRepeatedType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<4>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_float_repeated(&self) -> bool {
            self.float_repeated_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::FloatRepeatedType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<4>,
                Fields::ImplTag,
            >,
    {
        pub fn float_repeated_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::FloatRepeatedType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<4>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.float_repeated, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::StringOptionalType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<5>,
                Fields::ImplTag,
            >,
    {
        pub fn string_optional(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::StringOptionalType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<5>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.string_optional, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::StringOptionalType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<5>,
                Fields::ImplTag,
            >,
    {
        pub fn string_optional_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::StringOptionalType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<5>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.string_optional, &self._shared)
                .get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::StringOptionalType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<5>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_string_optional(&self) -> bool {
            self.string_optional_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::StringOptionalType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<5>,
                Fields::ImplTag,
            >,
    {
        pub fn string_optional_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::StringOptionalType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<5>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.string_optional, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::StringRepeatedType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<6>,
                Fields::ImplTag,
            >,
    {
        pub fn string_repeated(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::StringRepeatedType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<6>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.string_repeated, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::StringRepeatedType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<6>,
                Fields::ImplTag,
            >,
    {
        pub fn string_repeated_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::StringRepeatedType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<6>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.string_repeated, &self._shared)
                .get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::StringRepeatedType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<6>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_string_repeated(&self) -> bool {
            self.string_repeated_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::StringRepeatedType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<6>,
                Fields::ImplTag,
            >,
    {
        pub fn string_repeated_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::StringRepeatedType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<6>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.string_repeated, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::SubmsgOptionalType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<7>,
                Fields::ImplTag,
            >,
    {
        pub fn submsg_optional(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::SubmsgOptionalType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<7>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.submsg_optional, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::SubmsgOptionalType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<7>,
                Fields::ImplTag,
            >,
    {
        pub fn submsg_optional_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::SubmsgOptionalType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<7>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.submsg_optional, &self._shared)
                .get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::SubmsgOptionalType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<7>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_submsg_optional(&self) -> bool {
            self.submsg_optional_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::SubmsgOptionalType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<7>,
                Fields::ImplTag,
            >,
    {
        pub fn submsg_optional_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::SubmsgOptionalType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<7>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.submsg_optional, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::SubmsgRepeatedType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<8>,
                Fields::ImplTag,
            >,
    {
        pub fn submsg_repeated(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::SubmsgRepeatedType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<8>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.submsg_repeated, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::SubmsgRepeatedType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<8>,
                Fields::ImplTag,
            >,
    {
        pub fn submsg_repeated_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::SubmsgRepeatedType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<8>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.submsg_repeated, &self._shared)
                .get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::SubmsgRepeatedType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<8>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_submsg_repeated(&self) -> bool {
            self.submsg_repeated_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::SubmsgRepeatedType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<8>,
                Fields::ImplTag,
            >,
    {
        pub fn submsg_repeated_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::SubmsgRepeatedType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<8>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.submsg_repeated, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::EnumOptionalType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<9>,
                Fields::ImplTag,
            >,
    {
        pub fn enum_optional(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::EnumOptionalType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<9>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.enum_optional, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::EnumOptionalType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<9>,
                Fields::ImplTag,
            >,
    {
        pub fn enum_optional_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::EnumOptionalType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<9>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.enum_optional, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::EnumOptionalType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<9>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_enum_optional(&self) -> bool {
            self.enum_optional_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::EnumOptionalType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<9>,
                Fields::ImplTag,
            >,
    {
        pub fn enum_optional_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::EnumOptionalType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<9>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.enum_optional, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::EnumRepeatedType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<10>,
                Fields::ImplTag,
            >,
    {
        pub fn enum_repeated(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::EnumRepeatedType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<10>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.enum_repeated, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::EnumRepeatedType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<10>,
                Fields::ImplTag,
            >,
    {
        pub fn enum_repeated_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::EnumRepeatedType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<10>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.enum_repeated, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::EnumRepeatedType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<10>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_enum_repeated(&self) -> bool {
            self.enum_repeated_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::EnumRepeatedType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<10>,
                Fields::ImplTag,
            >,
    {
        pub fn enum_repeated_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::EnumRepeatedType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<10>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.enum_repeated, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::VeryLargeFieldNumberType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<536870911>,
                Fields::ImplTag,
            >,
    {
        pub fn very_large_field_number(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::VeryLargeFieldNumberType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<536870911>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.very_large_field_number, &self._shared)
                .get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::VeryLargeFieldNumberType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<536870911>,
                Fields::ImplTag,
            >,
    {
        pub fn very_large_field_number_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::VeryLargeFieldNumberType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<536870911>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.very_large_field_number, &self._shared)
                .get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::VeryLargeFieldNumberType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<536870911>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_very_large_field_number(&self) -> bool {
            self.very_large_field_number_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::VeryLargeFieldNumberType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<536870911>,
                Fields::ImplTag,
            >,
    {
        pub fn very_large_field_number_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::VeryLargeFieldNumberType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<536870911>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(
                &mut self.very_large_field_number,
                &mut self._shared,
            )
            .get_mut()
        }
    }

    impl<Fields, Shared> ::std::default::Default for MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        Shared: ::std::default::Default,
        Fields::I32OptionalType: ::std::default::Default,
        Fields::I32RepeatedType: ::std::default::Default,
        Fields::FloatOptionalType: ::std::default::Default,
        Fields::FloatRepeatedType: ::std::default::Default,
        Fields::StringOptionalType: ::std::default::Default,
        Fields::StringRepeatedType: ::std::default::Default,
        Fields::SubmsgOptionalType: ::std::default::Default,
        Fields::SubmsgRepeatedType: ::std::default::Default,
        Fields::EnumOptionalType: ::std::default::Default,
        Fields::EnumRepeatedType: ::std::default::Default,
        Fields::VeryLargeFieldNumberType: ::std::default::Default,
    {
        fn default() -> Self {
            Self {
                _shared: ::std::default::Default::default(),
                _phantom: ::std::default::Default::default(),
                i32_optional: ::std::default::Default::default(),
                i32_repeated: ::std::default::Default::default(),
                float_optional: ::std::default::Default::default(),
                float_repeated: ::std::default::Default::default(),
                string_optional: ::std::default::Default::default(),
                string_repeated: ::std::default::Default::default(),
                submsg_optional: ::std::default::Default::default(),
                submsg_repeated: ::std::default::Default::default(),
                enum_optional: ::std::default::Default::default(),
                enum_repeated: ::std::default::Default::default(),
                very_large_field_number: ::std::default::Default::default(),
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
        type I32OptionalType;
        type I32RepeatedType;
        type FloatOptionalType;
        type FloatRepeatedType;
        type StringOptionalType;
        type StringRepeatedType;
        type SubmsgOptionalType;
        type SubmsgRepeatedType;
        type EnumOptionalType;
        type EnumRepeatedType;
        type VeryLargeFieldNumberType;
    }

    impl MsgTemplateFieldTypes for ::puroro::internal::SimpleFields {
        type ImplTag = ::puroro::tags::SimpleImpl;
        type I32OptionalType = i32;
        type I32RepeatedType = ::std::vec::Vec<i32>;
        type FloatOptionalType = f32;
        type FloatRepeatedType = ::std::vec::Vec<f32>;
        type StringOptionalType = ::std::string::String;
        type StringRepeatedType = ::std::vec::Vec<::std::string::String>;
        type SubmsgOptionalType = ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_simple_impl::Submsg,
            >,
        >;
        type SubmsgRepeatedType = ::std::vec::Vec<
            self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_simple_impl::Submsg,
        >;
        type EnumOptionalType = self::_puroro_root::ser_tests2::Enum;
        type EnumRepeatedType = ::std::vec::Vec<self::_puroro_root::ser_tests2::Enum>;
        type VeryLargeFieldNumberType = i32;
    }
}
pub use _puroro_traits::*;
pub mod _puroro_traits {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
}
#[derive(::std::fmt::Debug, ::std::clone::Clone, ::std::marker::Copy, ::std::cmp::PartialEq)]
pub enum Enum {
    Zeroth,
    First,
    Tenth,
}

impl ::puroro::Enum2 for Enum {}
impl ::std::convert::TryFrom<i32> for Enum {
    type Error = i32;
    fn try_from(value: i32) -> ::std::result::Result<Self, i32> {
        ::std::result::Result::Ok(match value {
            0 => Enum::Zeroth,
            1 => Enum::First,
            10 => Enum::Tenth,
            _ => Err(value)?,
        })
    }
}

impl ::std::convert::From<Enum> for i32 {
    fn from(value: Enum) -> i32 {
        match value {
            Enum::Zeroth => 0,
            Enum::First => 1,
            Enum::Tenth => 10,
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
            pub struct SubmsgTemplate<Fields, Shared>
            where
                Fields: SubmsgTemplateFieldTypes,
            {
                _shared: Shared,
                _phantom: ::std::marker::PhantomData<Fields>,
                i32_optional: <Fields as SubmsgTemplateFieldTypes>::I32OptionalType,
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
                Fields::I32OptionalType: ::std::default::Default,
            {
                fn new_in(alloc: AllocatorType) -> Self {
                    Self {
                        _shared: ::std::convert::Into::into(alloc),
                        _phantom: ::std::default::Default::default(),
                        i32_optional: ::std::default::Default::default(),
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
                const BITFIELD_OPTIONAL_FIELD_COUNT: usize = 1;
            }

            pub struct SubmsgFieldProperties<const FIELD_NUMBER: i32>;
            impl ::puroro::internal::FieldProperties for SubmsgFieldProperties<1> {
                type MessageProperties = self::SubmsgMessageProperties;
                const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
                type LabelTag = ::puroro::tags::Optional;
                type TypeTag = ::puroro::tags::Int32;
                const DEFAULT_VALUE:
                    <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
            }

            pub type SubmsgSimple2 = SubmsgTemplate<
                ::puroro::internal::SimpleFields,
                ::puroro::internal::SimpleShared<{ (1 + 31) / 32 }>,
            >;

            impl<Fields, Shared> SubmsgTemplate<Fields, Shared>
            where
                Fields: SubmsgTemplateFieldTypes,
                for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I32OptionalType, Shared>:
                    ::puroro::internal::methods::GetFieldMethod<
                        'a,
                        self::SubmsgFieldProperties<1>,
                        Fields::ImplTag,
                    >,
            {
                pub fn i32_optional(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I32OptionalType, Shared> as
                ::puroro::internal::methods::GetFieldMethod<
                    self::SubmsgFieldProperties<1>,
                    Fields::ImplTag,
                >
                >::GetterType{
                    use ::puroro::internal::methods::GetFieldMethod as _;
                    ::puroro::internal::FieldAndSharedRef::new(&self.i32_optional, &self._shared)
                        .get()
                }
            }

            impl<Fields, Shared> SubmsgTemplate<Fields, Shared>
            where
                Fields: SubmsgTemplateFieldTypes,
                for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I32OptionalType, Shared>:
                    ::puroro::internal::methods::GetOptFieldMethod<
                        'a,
                        self::SubmsgFieldProperties<1>,
                        Fields::ImplTag,
                    >,
            {
                pub fn i32_optional_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I32OptionalType, Shared> as
                ::puroro::internal::methods::GetOptFieldMethod<
                    self::SubmsgFieldProperties<1>,
                    Fields::ImplTag,
                >
                >::GetterType{
                    use ::puroro::internal::methods::GetOptFieldMethod as _;
                    ::puroro::internal::FieldAndSharedRef::new(&self.i32_optional, &self._shared)
                        .get_opt()
                }
            }

            impl<Fields, Shared, OptionInnerType> SubmsgTemplate<Fields, Shared>
            where
                Fields: SubmsgTemplateFieldTypes,
                for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I32OptionalType, Shared>:
                    ::puroro::internal::methods::GetOptFieldMethod<
                        'a,
                        self::SubmsgFieldProperties<1>,
                        Fields::ImplTag,
                        GetterType = Option<OptionInnerType>,
                    >,
            {
                pub fn has_i32_optional(&self) -> bool {
                    self.i32_optional_opt().is_some()
                }
            }

            impl<Fields, Shared> SubmsgTemplate<Fields, Shared>
            where
                Fields: SubmsgTemplateFieldTypes,
                for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::I32OptionalType, Shared>:
                    ::puroro::internal::methods::GetMutFieldMethod<
                        'a,
                        self::SubmsgFieldProperties<1>,
                        Fields::ImplTag,
                    >,
            {
                pub fn i32_optional_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::I32OptionalType, Shared> as
                ::puroro::internal::methods::GetMutFieldMethod<
                    self::SubmsgFieldProperties<1>,
                    Fields::ImplTag,
                >
                >::GetterType{
                    use ::puroro::internal::methods::GetMutFieldMethod as _;
                    ::puroro::internal::FieldAndSharedMut::new(
                        &mut self.i32_optional,
                        &mut self._shared,
                    )
                    .get_mut()
                }
            }

            impl<Fields, Shared> ::std::default::Default for SubmsgTemplate<Fields, Shared>
            where
                Fields: SubmsgTemplateFieldTypes,
                Shared: ::std::default::Default,
                Fields::I32OptionalType: ::std::default::Default,
            {
                fn default() -> Self {
                    Self {
                        _shared: ::std::default::Default::default(),
                        _phantom: ::std::default::Default::default(),
                        i32_optional: ::std::default::Default::default(),
                    }
                }
            }
        }
        pub mod _puroro_internal {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }
            pub trait SubmsgTemplateFieldTypes {
                type ImplTag;
                type I32OptionalType;
            }

            impl SubmsgTemplateFieldTypes for ::puroro::internal::SimpleFields {
                type ImplTag = ::puroro::tags::SimpleImpl;
                type I32OptionalType = i32;
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
            pub mod submsg {
                mod _puroro_root {
                    pub use super::super::super::_puroro_root::*;
                }
            }
        }
    }
}
