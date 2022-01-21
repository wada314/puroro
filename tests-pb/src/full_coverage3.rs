// A generated source code by puroro library
// package full_coverage3

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
        i32_unlabeled: <Fields as MsgTemplateFieldTypes>::I32UnlabeledType,
        i32_optional: <Fields as MsgTemplateFieldTypes>::I32OptionalType,
        i32_repeated: <Fields as MsgTemplateFieldTypes>::I32RepeatedType,
        float_unlabeled: <Fields as MsgTemplateFieldTypes>::FloatUnlabeledType,
        float_optional: <Fields as MsgTemplateFieldTypes>::FloatOptionalType,
        float_repeated: <Fields as MsgTemplateFieldTypes>::FloatRepeatedType,
        bytes_unlabeled: <Fields as MsgTemplateFieldTypes>::BytesUnlabeledType,
        bytes_optional: <Fields as MsgTemplateFieldTypes>::BytesOptionalType,
        bytes_repeated: <Fields as MsgTemplateFieldTypes>::BytesRepeatedType,
        string_unlabeled: <Fields as MsgTemplateFieldTypes>::StringUnlabeledType,
        string_optional: <Fields as MsgTemplateFieldTypes>::StringOptionalType,
        string_repeated: <Fields as MsgTemplateFieldTypes>::StringRepeatedType,
        enum_unlabeled: <Fields as MsgTemplateFieldTypes>::EnumUnlabeledType,
        enum_optional: <Fields as MsgTemplateFieldTypes>::EnumOptionalType,
        enum_repeated: <Fields as MsgTemplateFieldTypes>::EnumRepeatedType,
        submsg_unlabeled: <Fields as MsgTemplateFieldTypes>::SubmsgUnlabeledType,
        submsg_optional: <Fields as MsgTemplateFieldTypes>::SubmsgOptionalType,
        submsg_repeated: <Fields as MsgTemplateFieldTypes>::SubmsgRepeatedType,
        i64_unlabeled: <Fields as MsgTemplateFieldTypes>::I64UnlabeledType,
        i64_optional: <Fields as MsgTemplateFieldTypes>::I64OptionalType,
        i64_repeated: <Fields as MsgTemplateFieldTypes>::I64RepeatedType,
        u32_unlabeled: <Fields as MsgTemplateFieldTypes>::U32UnlabeledType,
        u32_optional: <Fields as MsgTemplateFieldTypes>::U32OptionalType,
        u32_repeated: <Fields as MsgTemplateFieldTypes>::U32RepeatedType,
        u64_unlabeled: <Fields as MsgTemplateFieldTypes>::U64UnlabeledType,
        u64_optional: <Fields as MsgTemplateFieldTypes>::U64OptionalType,
        u64_repeated: <Fields as MsgTemplateFieldTypes>::U64RepeatedType,
        s32_unlabeled: <Fields as MsgTemplateFieldTypes>::S32UnlabeledType,
        s32_optional: <Fields as MsgTemplateFieldTypes>::S32OptionalType,
        s32_repeated: <Fields as MsgTemplateFieldTypes>::S32RepeatedType,
        s64_unlabeled: <Fields as MsgTemplateFieldTypes>::S64UnlabeledType,
        s64_optional: <Fields as MsgTemplateFieldTypes>::S64OptionalType,
        s64_repeated: <Fields as MsgTemplateFieldTypes>::S64RepeatedType,
        fixed32_unlabeled: <Fields as MsgTemplateFieldTypes>::Fixed32UnlabeledType,
        fixed32_optional: <Fields as MsgTemplateFieldTypes>::Fixed32OptionalType,
        fixed32_repeated: <Fields as MsgTemplateFieldTypes>::Fixed32RepeatedType,
        fixed64_unlabeled: <Fields as MsgTemplateFieldTypes>::Fixed64UnlabeledType,
        fixed64_optional: <Fields as MsgTemplateFieldTypes>::Fixed64OptionalType,
        fixed64_repeated: <Fields as MsgTemplateFieldTypes>::Fixed64RepeatedType,
        sfixed32_unlabeled: <Fields as MsgTemplateFieldTypes>::Sfixed32UnlabeledType,
        sfixed32_optional: <Fields as MsgTemplateFieldTypes>::Sfixed32OptionalType,
        sfixed32_repeated: <Fields as MsgTemplateFieldTypes>::Sfixed32RepeatedType,
        sfixed64_unlabeled: <Fields as MsgTemplateFieldTypes>::Sfixed64UnlabeledType,
        sfixed64_optional: <Fields as MsgTemplateFieldTypes>::Sfixed64OptionalType,
        sfixed64_repeated: <Fields as MsgTemplateFieldTypes>::Sfixed64RepeatedType,
        f64_unlabeled: <Fields as MsgTemplateFieldTypes>::F64UnlabeledType,
        f64_optional: <Fields as MsgTemplateFieldTypes>::F64OptionalType,
        f64_repeated: <Fields as MsgTemplateFieldTypes>::F64RepeatedType,
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
        Fields::FloatUnlabeledType: ::std::default::Default,
        Fields::FloatOptionalType: ::std::default::Default,
        Fields::FloatRepeatedType: ::std::default::Default,
        Fields::BytesUnlabeledType: ::std::default::Default,
        Fields::BytesOptionalType: ::std::default::Default,
        Fields::BytesRepeatedType: ::std::default::Default,
        Fields::StringUnlabeledType: ::std::default::Default,
        Fields::StringOptionalType: ::std::default::Default,
        Fields::StringRepeatedType: ::std::default::Default,
        Fields::EnumUnlabeledType: ::std::default::Default,
        Fields::EnumOptionalType: ::std::default::Default,
        Fields::EnumRepeatedType: ::std::default::Default,
        Fields::SubmsgUnlabeledType: ::std::default::Default,
        Fields::SubmsgOptionalType: ::std::default::Default,
        Fields::SubmsgRepeatedType: ::std::default::Default,
        Fields::I64UnlabeledType: ::std::default::Default,
        Fields::I64OptionalType: ::std::default::Default,
        Fields::I64RepeatedType: ::std::default::Default,
        Fields::U32UnlabeledType: ::std::default::Default,
        Fields::U32OptionalType: ::std::default::Default,
        Fields::U32RepeatedType: ::std::default::Default,
        Fields::U64UnlabeledType: ::std::default::Default,
        Fields::U64OptionalType: ::std::default::Default,
        Fields::U64RepeatedType: ::std::default::Default,
        Fields::S32UnlabeledType: ::std::default::Default,
        Fields::S32OptionalType: ::std::default::Default,
        Fields::S32RepeatedType: ::std::default::Default,
        Fields::S64UnlabeledType: ::std::default::Default,
        Fields::S64OptionalType: ::std::default::Default,
        Fields::S64RepeatedType: ::std::default::Default,
        Fields::Fixed32UnlabeledType: ::std::default::Default,
        Fields::Fixed32OptionalType: ::std::default::Default,
        Fields::Fixed32RepeatedType: ::std::default::Default,
        Fields::Fixed64UnlabeledType: ::std::default::Default,
        Fields::Fixed64OptionalType: ::std::default::Default,
        Fields::Fixed64RepeatedType: ::std::default::Default,
        Fields::Sfixed32UnlabeledType: ::std::default::Default,
        Fields::Sfixed32OptionalType: ::std::default::Default,
        Fields::Sfixed32RepeatedType: ::std::default::Default,
        Fields::Sfixed64UnlabeledType: ::std::default::Default,
        Fields::Sfixed64OptionalType: ::std::default::Default,
        Fields::Sfixed64RepeatedType: ::std::default::Default,
        Fields::F64UnlabeledType: ::std::default::Default,
        Fields::F64OptionalType: ::std::default::Default,
        Fields::F64RepeatedType: ::std::default::Default,
    {
        fn new_in(alloc: AllocatorType) -> Self {
            Self {
                _shared: ::std::convert::Into::into(alloc),
                _phantom: ::std::default::Default::default(),
                i32_unlabeled: ::std::default::Default::default(),
                i32_optional: ::std::default::Default::default(),
                i32_repeated: ::std::default::Default::default(),
                float_unlabeled: ::std::default::Default::default(),
                float_optional: ::std::default::Default::default(),
                float_repeated: ::std::default::Default::default(),
                bytes_unlabeled: ::std::default::Default::default(),
                bytes_optional: ::std::default::Default::default(),
                bytes_repeated: ::std::default::Default::default(),
                string_unlabeled: ::std::default::Default::default(),
                string_optional: ::std::default::Default::default(),
                string_repeated: ::std::default::Default::default(),
                enum_unlabeled: ::std::default::Default::default(),
                enum_optional: ::std::default::Default::default(),
                enum_repeated: ::std::default::Default::default(),
                submsg_unlabeled: ::std::default::Default::default(),
                submsg_optional: ::std::default::Default::default(),
                submsg_repeated: ::std::default::Default::default(),
                i64_unlabeled: ::std::default::Default::default(),
                i64_optional: ::std::default::Default::default(),
                i64_repeated: ::std::default::Default::default(),
                u32_unlabeled: ::std::default::Default::default(),
                u32_optional: ::std::default::Default::default(),
                u32_repeated: ::std::default::Default::default(),
                u64_unlabeled: ::std::default::Default::default(),
                u64_optional: ::std::default::Default::default(),
                u64_repeated: ::std::default::Default::default(),
                s32_unlabeled: ::std::default::Default::default(),
                s32_optional: ::std::default::Default::default(),
                s32_repeated: ::std::default::Default::default(),
                s64_unlabeled: ::std::default::Default::default(),
                s64_optional: ::std::default::Default::default(),
                s64_repeated: ::std::default::Default::default(),
                fixed32_unlabeled: ::std::default::Default::default(),
                fixed32_optional: ::std::default::Default::default(),
                fixed32_repeated: ::std::default::Default::default(),
                fixed64_unlabeled: ::std::default::Default::default(),
                fixed64_optional: ::std::default::Default::default(),
                fixed64_repeated: ::std::default::Default::default(),
                sfixed32_unlabeled: ::std::default::Default::default(),
                sfixed32_optional: ::std::default::Default::default(),
                sfixed32_repeated: ::std::default::Default::default(),
                sfixed64_unlabeled: ::std::default::Default::default(),
                sfixed64_optional: ::std::default::Default::default(),
                sfixed64_repeated: ::std::default::Default::default(),
                f64_unlabeled: ::std::default::Default::default(),
                f64_optional: ::std::default::Default::default(),
                f64_repeated: ::std::default::Default::default(),
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
        const BITFIELD_OPTIONAL_FIELD_COUNT: usize = 15;
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
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<11> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Unlabeled;
        type TypeTag = ::puroro::tags::Float;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            0.0f32;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<12> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 1;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Float;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            0.0f32;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<13> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Repeated;
        type TypeTag = ::puroro::tags::Float;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            0.0f32;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<21> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Unlabeled;
        type TypeTag = ::puroro::tags::Bytes;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            b"";
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<22> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 2;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Bytes;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            b"";
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<23> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Repeated;
        type TypeTag = ::puroro::tags::Bytes;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            b"";
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<31> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Unlabeled;
        type TypeTag = ::puroro::tags::String;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = "";
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<32> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 3;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::String;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = "";
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<33> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Repeated;
        type TypeTag = ::puroro::tags::String;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = "";
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<41> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Unlabeled;
        type TypeTag = ::puroro::tags::Enum3<self::_puroro_root::full_coverage3::Enum>;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            self::_puroro_root::full_coverage3::Enum::Zeroth;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<42> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 4;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Enum3<self::_puroro_root::full_coverage3::Enum>;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            self::_puroro_root::full_coverage3::Enum::Zeroth;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<43> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Repeated;
        type TypeTag = ::puroro::tags::Enum3<self::_puroro_root::full_coverage3::Enum>;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            self::_puroro_root::full_coverage3::Enum::Zeroth;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<51> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Unlabeled;
        type TypeTag = ::puroro::tags::Message<
            self::_puroro_root::full_coverage3::_puroro_nested::msg::SubmsgMessageProperties,
        >;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = ();
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<52> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Message<
            self::_puroro_root::full_coverage3::_puroro_nested::msg::SubmsgMessageProperties,
        >;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = ();
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<53> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Repeated;
        type TypeTag = ::puroro::tags::Message<
            self::_puroro_root::full_coverage3::_puroro_nested::msg::SubmsgMessageProperties,
        >;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = ();
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<101> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Unlabeled;
        type TypeTag = ::puroro::tags::Int64;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<102> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 5;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Int64;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<103> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Repeated;
        type TypeTag = ::puroro::tags::Int64;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<111> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Unlabeled;
        type TypeTag = ::puroro::tags::UInt32;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<112> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 6;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::UInt32;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<113> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Repeated;
        type TypeTag = ::puroro::tags::UInt32;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<121> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Unlabeled;
        type TypeTag = ::puroro::tags::UInt64;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<122> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 7;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::UInt64;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<123> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Repeated;
        type TypeTag = ::puroro::tags::UInt64;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<131> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Unlabeled;
        type TypeTag = ::puroro::tags::SInt32;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<132> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 8;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::SInt32;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<133> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Repeated;
        type TypeTag = ::puroro::tags::SInt32;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<141> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Unlabeled;
        type TypeTag = ::puroro::tags::SInt64;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<142> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 9;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::SInt64;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<143> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Repeated;
        type TypeTag = ::puroro::tags::SInt64;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<151> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Unlabeled;
        type TypeTag = ::puroro::tags::Fixed32;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<152> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 10;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Fixed32;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<153> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Repeated;
        type TypeTag = ::puroro::tags::Fixed32;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<161> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Unlabeled;
        type TypeTag = ::puroro::tags::Fixed64;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<162> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 11;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Fixed64;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<163> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Repeated;
        type TypeTag = ::puroro::tags::Fixed64;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<171> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Unlabeled;
        type TypeTag = ::puroro::tags::SFixed32;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<172> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 12;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::SFixed32;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<173> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Repeated;
        type TypeTag = ::puroro::tags::SFixed32;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<181> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Unlabeled;
        type TypeTag = ::puroro::tags::SFixed64;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<182> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 13;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::SFixed64;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<183> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Repeated;
        type TypeTag = ::puroro::tags::SFixed64;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<191> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Unlabeled;
        type TypeTag = ::puroro::tags::Double;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            0.0f64;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<192> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 14;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Double;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            0.0f64;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<193> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Repeated;
        type TypeTag = ::puroro::tags::Double;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            0.0f64;
    }

    pub type MsgSimple2 = MsgTemplate<
        ::puroro::internal::SimpleFields,
        ::puroro::internal::SimpleShared<{ (15 + 31) / 32 }>,
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
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::FloatUnlabeledType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<11>,
                Fields::ImplTag,
            >,
    {
        pub fn float_unlabeled(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::FloatUnlabeledType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<11>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.float_unlabeled, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::FloatUnlabeledType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<11>,
                Fields::ImplTag,
            >,
    {
        pub fn float_unlabeled_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::FloatUnlabeledType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<11>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.float_unlabeled, &self._shared)
                .get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::FloatUnlabeledType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<11>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_float_unlabeled(&self) -> bool {
            self.float_unlabeled_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::FloatUnlabeledType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<11>,
                Fields::ImplTag,
            >,
    {
        pub fn float_unlabeled_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::FloatUnlabeledType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<11>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.float_unlabeled, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::FloatOptionalType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<12>,
                Fields::ImplTag,
            >,
    {
        pub fn float_optional(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::FloatOptionalType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<12>,
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
                self::MsgFieldProperties<12>,
                Fields::ImplTag,
            >,
    {
        pub fn float_optional_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::FloatOptionalType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<12>,
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
                self::MsgFieldProperties<12>,
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
                self::MsgFieldProperties<12>,
                Fields::ImplTag,
            >,
    {
        pub fn float_optional_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::FloatOptionalType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<12>,
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
                self::MsgFieldProperties<13>,
                Fields::ImplTag,
            >,
    {
        pub fn float_repeated(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::FloatRepeatedType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<13>,
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
                self::MsgFieldProperties<13>,
                Fields::ImplTag,
            >,
    {
        pub fn float_repeated_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::FloatRepeatedType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<13>,
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
                self::MsgFieldProperties<13>,
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
                self::MsgFieldProperties<13>,
                Fields::ImplTag,
            >,
    {
        pub fn float_repeated_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::FloatRepeatedType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<13>,
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
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::BytesUnlabeledType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<21>,
                Fields::ImplTag,
            >,
    {
        pub fn bytes_unlabeled(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::BytesUnlabeledType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<21>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.bytes_unlabeled, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::BytesUnlabeledType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<21>,
                Fields::ImplTag,
            >,
    {
        pub fn bytes_unlabeled_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::BytesUnlabeledType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<21>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.bytes_unlabeled, &self._shared)
                .get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::BytesUnlabeledType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<21>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_bytes_unlabeled(&self) -> bool {
            self.bytes_unlabeled_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::BytesUnlabeledType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<21>,
                Fields::ImplTag,
            >,
    {
        pub fn bytes_unlabeled_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::BytesUnlabeledType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<21>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.bytes_unlabeled, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::BytesOptionalType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<22>,
                Fields::ImplTag,
            >,
    {
        pub fn bytes_optional(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::BytesOptionalType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<22>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.bytes_optional, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::BytesOptionalType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<22>,
                Fields::ImplTag,
            >,
    {
        pub fn bytes_optional_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::BytesOptionalType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<22>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.bytes_optional, &self._shared)
                .get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::BytesOptionalType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<22>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_bytes_optional(&self) -> bool {
            self.bytes_optional_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::BytesOptionalType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<22>,
                Fields::ImplTag,
            >,
    {
        pub fn bytes_optional_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::BytesOptionalType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<22>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.bytes_optional, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::BytesRepeatedType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<23>,
                Fields::ImplTag,
            >,
    {
        pub fn bytes_repeated(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::BytesRepeatedType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<23>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.bytes_repeated, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::BytesRepeatedType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<23>,
                Fields::ImplTag,
            >,
    {
        pub fn bytes_repeated_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::BytesRepeatedType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<23>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.bytes_repeated, &self._shared)
                .get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::BytesRepeatedType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<23>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_bytes_repeated(&self) -> bool {
            self.bytes_repeated_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::BytesRepeatedType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<23>,
                Fields::ImplTag,
            >,
    {
        pub fn bytes_repeated_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::BytesRepeatedType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<23>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.bytes_repeated, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::StringUnlabeledType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<31>,
                Fields::ImplTag,
            >,
    {
        pub fn string_unlabeled(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::StringUnlabeledType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<31>,
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
                self::MsgFieldProperties<31>,
                Fields::ImplTag,
            >,
    {
        pub fn string_unlabeled_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::StringUnlabeledType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<31>,
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
                self::MsgFieldProperties<31>,
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
                self::MsgFieldProperties<31>,
                Fields::ImplTag,
            >,
    {
        pub fn string_unlabeled_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::StringUnlabeledType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<31>,
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
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::StringOptionalType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<32>,
                Fields::ImplTag,
            >,
    {
        pub fn string_optional(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::StringOptionalType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<32>,
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
                self::MsgFieldProperties<32>,
                Fields::ImplTag,
            >,
    {
        pub fn string_optional_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::StringOptionalType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<32>,
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
                self::MsgFieldProperties<32>,
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
                self::MsgFieldProperties<32>,
                Fields::ImplTag,
            >,
    {
        pub fn string_optional_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::StringOptionalType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<32>,
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
                self::MsgFieldProperties<33>,
                Fields::ImplTag,
            >,
    {
        pub fn string_repeated(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::StringRepeatedType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<33>,
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
                self::MsgFieldProperties<33>,
                Fields::ImplTag,
            >,
    {
        pub fn string_repeated_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::StringRepeatedType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<33>,
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
                self::MsgFieldProperties<33>,
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
                self::MsgFieldProperties<33>,
                Fields::ImplTag,
            >,
    {
        pub fn string_repeated_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::StringRepeatedType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<33>,
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
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::EnumUnlabeledType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<41>,
                Fields::ImplTag,
            >,
    {
        pub fn enum_unlabeled(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::EnumUnlabeledType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<41>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.enum_unlabeled, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::EnumUnlabeledType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<41>,
                Fields::ImplTag,
            >,
    {
        pub fn enum_unlabeled_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::EnumUnlabeledType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<41>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.enum_unlabeled, &self._shared)
                .get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::EnumUnlabeledType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<41>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_enum_unlabeled(&self) -> bool {
            self.enum_unlabeled_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::EnumUnlabeledType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<41>,
                Fields::ImplTag,
            >,
    {
        pub fn enum_unlabeled_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::EnumUnlabeledType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<41>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.enum_unlabeled, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::EnumOptionalType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<42>,
                Fields::ImplTag,
            >,
    {
        pub fn enum_optional(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::EnumOptionalType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<42>,
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
                self::MsgFieldProperties<42>,
                Fields::ImplTag,
            >,
    {
        pub fn enum_optional_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::EnumOptionalType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<42>,
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
                self::MsgFieldProperties<42>,
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
                self::MsgFieldProperties<42>,
                Fields::ImplTag,
            >,
    {
        pub fn enum_optional_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::EnumOptionalType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<42>,
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
                self::MsgFieldProperties<43>,
                Fields::ImplTag,
            >,
    {
        pub fn enum_repeated(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::EnumRepeatedType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<43>,
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
                self::MsgFieldProperties<43>,
                Fields::ImplTag,
            >,
    {
        pub fn enum_repeated_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::EnumRepeatedType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<43>,
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
                self::MsgFieldProperties<43>,
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
                self::MsgFieldProperties<43>,
                Fields::ImplTag,
            >,
    {
        pub fn enum_repeated_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::EnumRepeatedType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<43>,
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
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::SubmsgUnlabeledType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<51>,
                Fields::ImplTag,
            >,
    {
        pub fn submsg_unlabeled(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::SubmsgUnlabeledType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<51>,
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
                self::MsgFieldProperties<51>,
                Fields::ImplTag,
            >,
    {
        pub fn submsg_unlabeled_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::SubmsgUnlabeledType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<51>,
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
                self::MsgFieldProperties<51>,
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
                self::MsgFieldProperties<51>,
                Fields::ImplTag,
            >,
    {
        pub fn submsg_unlabeled_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::SubmsgUnlabeledType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<51>,
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

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::SubmsgOptionalType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<52>,
                Fields::ImplTag,
            >,
    {
        pub fn submsg_optional(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::SubmsgOptionalType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<52>,
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
                self::MsgFieldProperties<52>,
                Fields::ImplTag,
            >,
    {
        pub fn submsg_optional_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::SubmsgOptionalType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<52>,
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
                self::MsgFieldProperties<52>,
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
                self::MsgFieldProperties<52>,
                Fields::ImplTag,
            >,
    {
        pub fn submsg_optional_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::SubmsgOptionalType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<52>,
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
                self::MsgFieldProperties<53>,
                Fields::ImplTag,
            >,
    {
        pub fn submsg_repeated(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::SubmsgRepeatedType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<53>,
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
                self::MsgFieldProperties<53>,
                Fields::ImplTag,
            >,
    {
        pub fn submsg_repeated_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::SubmsgRepeatedType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<53>,
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
                self::MsgFieldProperties<53>,
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
                self::MsgFieldProperties<53>,
                Fields::ImplTag,
            >,
    {
        pub fn submsg_repeated_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::SubmsgRepeatedType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<53>,
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
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I64UnlabeledType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<101>,
                Fields::ImplTag,
            >,
    {
        pub fn i64_unlabeled(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I64UnlabeledType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<101>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.i64_unlabeled, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I64UnlabeledType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<101>,
                Fields::ImplTag,
            >,
    {
        pub fn i64_unlabeled_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I64UnlabeledType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<101>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.i64_unlabeled, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I64UnlabeledType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<101>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_i64_unlabeled(&self) -> bool {
            self.i64_unlabeled_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::I64UnlabeledType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<101>,
                Fields::ImplTag,
            >,
    {
        pub fn i64_unlabeled_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::I64UnlabeledType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<101>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.i64_unlabeled, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I64OptionalType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<102>,
                Fields::ImplTag,
            >,
    {
        pub fn i64_optional(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I64OptionalType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<102>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.i64_optional, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I64OptionalType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<102>,
                Fields::ImplTag,
            >,
    {
        pub fn i64_optional_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I64OptionalType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<102>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.i64_optional, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I64OptionalType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<102>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_i64_optional(&self) -> bool {
            self.i64_optional_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::I64OptionalType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<102>,
                Fields::ImplTag,
            >,
    {
        pub fn i64_optional_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::I64OptionalType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<102>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.i64_optional, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I64RepeatedType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<103>,
                Fields::ImplTag,
            >,
    {
        pub fn i64_repeated(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I64RepeatedType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<103>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.i64_repeated, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I64RepeatedType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<103>,
                Fields::ImplTag,
            >,
    {
        pub fn i64_repeated_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I64RepeatedType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<103>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.i64_repeated, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I64RepeatedType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<103>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_i64_repeated(&self) -> bool {
            self.i64_repeated_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::I64RepeatedType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<103>,
                Fields::ImplTag,
            >,
    {
        pub fn i64_repeated_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::I64RepeatedType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<103>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.i64_repeated, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U32UnlabeledType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<111>,
                Fields::ImplTag,
            >,
    {
        pub fn u32_unlabeled(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::U32UnlabeledType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<111>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.u32_unlabeled, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U32UnlabeledType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<111>,
                Fields::ImplTag,
            >,
    {
        pub fn u32_unlabeled_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::U32UnlabeledType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<111>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.u32_unlabeled, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U32UnlabeledType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<111>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_u32_unlabeled(&self) -> bool {
            self.u32_unlabeled_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::U32UnlabeledType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<111>,
                Fields::ImplTag,
            >,
    {
        pub fn u32_unlabeled_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::U32UnlabeledType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<111>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.u32_unlabeled, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U32OptionalType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<112>,
                Fields::ImplTag,
            >,
    {
        pub fn u32_optional(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::U32OptionalType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<112>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.u32_optional, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U32OptionalType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<112>,
                Fields::ImplTag,
            >,
    {
        pub fn u32_optional_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::U32OptionalType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<112>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.u32_optional, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U32OptionalType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<112>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_u32_optional(&self) -> bool {
            self.u32_optional_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::U32OptionalType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<112>,
                Fields::ImplTag,
            >,
    {
        pub fn u32_optional_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::U32OptionalType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<112>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.u32_optional, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U32RepeatedType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<113>,
                Fields::ImplTag,
            >,
    {
        pub fn u32_repeated(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::U32RepeatedType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<113>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.u32_repeated, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U32RepeatedType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<113>,
                Fields::ImplTag,
            >,
    {
        pub fn u32_repeated_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::U32RepeatedType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<113>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.u32_repeated, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U32RepeatedType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<113>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_u32_repeated(&self) -> bool {
            self.u32_repeated_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::U32RepeatedType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<113>,
                Fields::ImplTag,
            >,
    {
        pub fn u32_repeated_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::U32RepeatedType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<113>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.u32_repeated, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U64UnlabeledType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<121>,
                Fields::ImplTag,
            >,
    {
        pub fn u64_unlabeled(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::U64UnlabeledType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<121>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.u64_unlabeled, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U64UnlabeledType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<121>,
                Fields::ImplTag,
            >,
    {
        pub fn u64_unlabeled_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::U64UnlabeledType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<121>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.u64_unlabeled, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U64UnlabeledType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<121>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_u64_unlabeled(&self) -> bool {
            self.u64_unlabeled_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::U64UnlabeledType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<121>,
                Fields::ImplTag,
            >,
    {
        pub fn u64_unlabeled_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::U64UnlabeledType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<121>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.u64_unlabeled, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U64OptionalType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<122>,
                Fields::ImplTag,
            >,
    {
        pub fn u64_optional(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::U64OptionalType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<122>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.u64_optional, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U64OptionalType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<122>,
                Fields::ImplTag,
            >,
    {
        pub fn u64_optional_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::U64OptionalType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<122>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.u64_optional, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U64OptionalType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<122>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_u64_optional(&self) -> bool {
            self.u64_optional_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::U64OptionalType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<122>,
                Fields::ImplTag,
            >,
    {
        pub fn u64_optional_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::U64OptionalType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<122>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.u64_optional, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U64RepeatedType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<123>,
                Fields::ImplTag,
            >,
    {
        pub fn u64_repeated(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::U64RepeatedType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<123>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.u64_repeated, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U64RepeatedType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<123>,
                Fields::ImplTag,
            >,
    {
        pub fn u64_repeated_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::U64RepeatedType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<123>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.u64_repeated, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U64RepeatedType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<123>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_u64_repeated(&self) -> bool {
            self.u64_repeated_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::U64RepeatedType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<123>,
                Fields::ImplTag,
            >,
    {
        pub fn u64_repeated_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::U64RepeatedType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<123>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.u64_repeated, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::S32UnlabeledType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<131>,
                Fields::ImplTag,
            >,
    {
        pub fn s32_unlabeled(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::S32UnlabeledType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<131>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.s32_unlabeled, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::S32UnlabeledType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<131>,
                Fields::ImplTag,
            >,
    {
        pub fn s32_unlabeled_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::S32UnlabeledType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<131>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.s32_unlabeled, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::S32UnlabeledType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<131>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_s32_unlabeled(&self) -> bool {
            self.s32_unlabeled_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::S32UnlabeledType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<131>,
                Fields::ImplTag,
            >,
    {
        pub fn s32_unlabeled_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::S32UnlabeledType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<131>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.s32_unlabeled, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::S32OptionalType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<132>,
                Fields::ImplTag,
            >,
    {
        pub fn s32_optional(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::S32OptionalType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<132>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.s32_optional, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::S32OptionalType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<132>,
                Fields::ImplTag,
            >,
    {
        pub fn s32_optional_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::S32OptionalType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<132>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.s32_optional, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::S32OptionalType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<132>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_s32_optional(&self) -> bool {
            self.s32_optional_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::S32OptionalType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<132>,
                Fields::ImplTag,
            >,
    {
        pub fn s32_optional_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::S32OptionalType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<132>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.s32_optional, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::S32RepeatedType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<133>,
                Fields::ImplTag,
            >,
    {
        pub fn s32_repeated(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::S32RepeatedType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<133>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.s32_repeated, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::S32RepeatedType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<133>,
                Fields::ImplTag,
            >,
    {
        pub fn s32_repeated_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::S32RepeatedType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<133>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.s32_repeated, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::S32RepeatedType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<133>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_s32_repeated(&self) -> bool {
            self.s32_repeated_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::S32RepeatedType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<133>,
                Fields::ImplTag,
            >,
    {
        pub fn s32_repeated_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::S32RepeatedType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<133>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.s32_repeated, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::S64UnlabeledType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<141>,
                Fields::ImplTag,
            >,
    {
        pub fn s64_unlabeled(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::S64UnlabeledType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<141>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.s64_unlabeled, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::S64UnlabeledType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<141>,
                Fields::ImplTag,
            >,
    {
        pub fn s64_unlabeled_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::S64UnlabeledType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<141>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.s64_unlabeled, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::S64UnlabeledType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<141>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_s64_unlabeled(&self) -> bool {
            self.s64_unlabeled_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::S64UnlabeledType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<141>,
                Fields::ImplTag,
            >,
    {
        pub fn s64_unlabeled_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::S64UnlabeledType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<141>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.s64_unlabeled, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::S64OptionalType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<142>,
                Fields::ImplTag,
            >,
    {
        pub fn s64_optional(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::S64OptionalType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<142>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.s64_optional, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::S64OptionalType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<142>,
                Fields::ImplTag,
            >,
    {
        pub fn s64_optional_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::S64OptionalType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<142>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.s64_optional, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::S64OptionalType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<142>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_s64_optional(&self) -> bool {
            self.s64_optional_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::S64OptionalType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<142>,
                Fields::ImplTag,
            >,
    {
        pub fn s64_optional_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::S64OptionalType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<142>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.s64_optional, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::S64RepeatedType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<143>,
                Fields::ImplTag,
            >,
    {
        pub fn s64_repeated(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::S64RepeatedType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<143>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.s64_repeated, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::S64RepeatedType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<143>,
                Fields::ImplTag,
            >,
    {
        pub fn s64_repeated_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::S64RepeatedType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<143>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.s64_repeated, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::S64RepeatedType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<143>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_s64_repeated(&self) -> bool {
            self.s64_repeated_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::S64RepeatedType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<143>,
                Fields::ImplTag,
            >,
    {
        pub fn s64_repeated_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::S64RepeatedType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<143>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.s64_repeated, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::Fixed32UnlabeledType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<151>,
                Fields::ImplTag,
            >,
    {
        pub fn fixed32_unlabeled(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::Fixed32UnlabeledType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<151>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.fixed32_unlabeled, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::Fixed32UnlabeledType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<151>,
                Fields::ImplTag,
            >,
    {
        pub fn fixed32_unlabeled_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::Fixed32UnlabeledType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<151>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.fixed32_unlabeled, &self._shared)
                .get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::Fixed32UnlabeledType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<151>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_fixed32_unlabeled(&self) -> bool {
            self.fixed32_unlabeled_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::Fixed32UnlabeledType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<151>,
                Fields::ImplTag,
            >,
    {
        pub fn fixed32_unlabeled_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::Fixed32UnlabeledType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<151>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(
                &mut self.fixed32_unlabeled,
                &mut self._shared,
            )
            .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::Fixed32OptionalType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<152>,
                Fields::ImplTag,
            >,
    {
        pub fn fixed32_optional(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::Fixed32OptionalType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<152>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.fixed32_optional, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::Fixed32OptionalType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<152>,
                Fields::ImplTag,
            >,
    {
        pub fn fixed32_optional_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::Fixed32OptionalType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<152>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.fixed32_optional, &self._shared)
                .get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::Fixed32OptionalType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<152>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_fixed32_optional(&self) -> bool {
            self.fixed32_optional_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::Fixed32OptionalType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<152>,
                Fields::ImplTag,
            >,
    {
        pub fn fixed32_optional_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::Fixed32OptionalType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<152>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(
                &mut self.fixed32_optional,
                &mut self._shared,
            )
            .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::Fixed32RepeatedType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<153>,
                Fields::ImplTag,
            >,
    {
        pub fn fixed32_repeated(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::Fixed32RepeatedType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<153>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.fixed32_repeated, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::Fixed32RepeatedType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<153>,
                Fields::ImplTag,
            >,
    {
        pub fn fixed32_repeated_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::Fixed32RepeatedType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<153>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.fixed32_repeated, &self._shared)
                .get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::Fixed32RepeatedType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<153>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_fixed32_repeated(&self) -> bool {
            self.fixed32_repeated_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::Fixed32RepeatedType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<153>,
                Fields::ImplTag,
            >,
    {
        pub fn fixed32_repeated_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::Fixed32RepeatedType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<153>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(
                &mut self.fixed32_repeated,
                &mut self._shared,
            )
            .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::Fixed64UnlabeledType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<161>,
                Fields::ImplTag,
            >,
    {
        pub fn fixed64_unlabeled(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::Fixed64UnlabeledType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<161>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.fixed64_unlabeled, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::Fixed64UnlabeledType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<161>,
                Fields::ImplTag,
            >,
    {
        pub fn fixed64_unlabeled_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::Fixed64UnlabeledType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<161>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.fixed64_unlabeled, &self._shared)
                .get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::Fixed64UnlabeledType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<161>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_fixed64_unlabeled(&self) -> bool {
            self.fixed64_unlabeled_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::Fixed64UnlabeledType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<161>,
                Fields::ImplTag,
            >,
    {
        pub fn fixed64_unlabeled_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::Fixed64UnlabeledType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<161>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(
                &mut self.fixed64_unlabeled,
                &mut self._shared,
            )
            .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::Fixed64OptionalType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<162>,
                Fields::ImplTag,
            >,
    {
        pub fn fixed64_optional(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::Fixed64OptionalType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<162>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.fixed64_optional, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::Fixed64OptionalType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<162>,
                Fields::ImplTag,
            >,
    {
        pub fn fixed64_optional_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::Fixed64OptionalType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<162>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.fixed64_optional, &self._shared)
                .get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::Fixed64OptionalType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<162>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_fixed64_optional(&self) -> bool {
            self.fixed64_optional_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::Fixed64OptionalType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<162>,
                Fields::ImplTag,
            >,
    {
        pub fn fixed64_optional_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::Fixed64OptionalType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<162>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(
                &mut self.fixed64_optional,
                &mut self._shared,
            )
            .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::Fixed64RepeatedType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<163>,
                Fields::ImplTag,
            >,
    {
        pub fn fixed64_repeated(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::Fixed64RepeatedType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<163>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.fixed64_repeated, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::Fixed64RepeatedType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<163>,
                Fields::ImplTag,
            >,
    {
        pub fn fixed64_repeated_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::Fixed64RepeatedType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<163>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.fixed64_repeated, &self._shared)
                .get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::Fixed64RepeatedType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<163>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_fixed64_repeated(&self) -> bool {
            self.fixed64_repeated_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::Fixed64RepeatedType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<163>,
                Fields::ImplTag,
            >,
    {
        pub fn fixed64_repeated_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::Fixed64RepeatedType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<163>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(
                &mut self.fixed64_repeated,
                &mut self._shared,
            )
            .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::Sfixed32UnlabeledType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<171>,
                Fields::ImplTag,
            >,
    {
        pub fn sfixed32_unlabeled(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::Sfixed32UnlabeledType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<171>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.sfixed32_unlabeled, &self._shared)
                .get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::Sfixed32UnlabeledType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<171>,
                Fields::ImplTag,
            >,
    {
        pub fn sfixed32_unlabeled_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::Sfixed32UnlabeledType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<171>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.sfixed32_unlabeled, &self._shared)
                .get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::Sfixed32UnlabeledType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<171>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_sfixed32_unlabeled(&self) -> bool {
            self.sfixed32_unlabeled_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::Sfixed32UnlabeledType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<171>,
                Fields::ImplTag,
            >,
    {
        pub fn sfixed32_unlabeled_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::Sfixed32UnlabeledType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<171>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(
                &mut self.sfixed32_unlabeled,
                &mut self._shared,
            )
            .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::Sfixed32OptionalType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<172>,
                Fields::ImplTag,
            >,
    {
        pub fn sfixed32_optional(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::Sfixed32OptionalType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<172>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.sfixed32_optional, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::Sfixed32OptionalType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<172>,
                Fields::ImplTag,
            >,
    {
        pub fn sfixed32_optional_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::Sfixed32OptionalType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<172>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.sfixed32_optional, &self._shared)
                .get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::Sfixed32OptionalType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<172>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_sfixed32_optional(&self) -> bool {
            self.sfixed32_optional_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::Sfixed32OptionalType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<172>,
                Fields::ImplTag,
            >,
    {
        pub fn sfixed32_optional_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::Sfixed32OptionalType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<172>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(
                &mut self.sfixed32_optional,
                &mut self._shared,
            )
            .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::Sfixed32RepeatedType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<173>,
                Fields::ImplTag,
            >,
    {
        pub fn sfixed32_repeated(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::Sfixed32RepeatedType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<173>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.sfixed32_repeated, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::Sfixed32RepeatedType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<173>,
                Fields::ImplTag,
            >,
    {
        pub fn sfixed32_repeated_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::Sfixed32RepeatedType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<173>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.sfixed32_repeated, &self._shared)
                .get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::Sfixed32RepeatedType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<173>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_sfixed32_repeated(&self) -> bool {
            self.sfixed32_repeated_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::Sfixed32RepeatedType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<173>,
                Fields::ImplTag,
            >,
    {
        pub fn sfixed32_repeated_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::Sfixed32RepeatedType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<173>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(
                &mut self.sfixed32_repeated,
                &mut self._shared,
            )
            .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::Sfixed64UnlabeledType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<181>,
                Fields::ImplTag,
            >,
    {
        pub fn sfixed64_unlabeled(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::Sfixed64UnlabeledType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<181>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.sfixed64_unlabeled, &self._shared)
                .get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::Sfixed64UnlabeledType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<181>,
                Fields::ImplTag,
            >,
    {
        pub fn sfixed64_unlabeled_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::Sfixed64UnlabeledType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<181>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.sfixed64_unlabeled, &self._shared)
                .get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::Sfixed64UnlabeledType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<181>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_sfixed64_unlabeled(&self) -> bool {
            self.sfixed64_unlabeled_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::Sfixed64UnlabeledType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<181>,
                Fields::ImplTag,
            >,
    {
        pub fn sfixed64_unlabeled_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::Sfixed64UnlabeledType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<181>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(
                &mut self.sfixed64_unlabeled,
                &mut self._shared,
            )
            .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::Sfixed64OptionalType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<182>,
                Fields::ImplTag,
            >,
    {
        pub fn sfixed64_optional(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::Sfixed64OptionalType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<182>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.sfixed64_optional, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::Sfixed64OptionalType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<182>,
                Fields::ImplTag,
            >,
    {
        pub fn sfixed64_optional_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::Sfixed64OptionalType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<182>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.sfixed64_optional, &self._shared)
                .get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::Sfixed64OptionalType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<182>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_sfixed64_optional(&self) -> bool {
            self.sfixed64_optional_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::Sfixed64OptionalType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<182>,
                Fields::ImplTag,
            >,
    {
        pub fn sfixed64_optional_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::Sfixed64OptionalType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<182>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(
                &mut self.sfixed64_optional,
                &mut self._shared,
            )
            .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::Sfixed64RepeatedType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<183>,
                Fields::ImplTag,
            >,
    {
        pub fn sfixed64_repeated(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::Sfixed64RepeatedType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<183>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.sfixed64_repeated, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::Sfixed64RepeatedType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<183>,
                Fields::ImplTag,
            >,
    {
        pub fn sfixed64_repeated_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::Sfixed64RepeatedType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<183>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.sfixed64_repeated, &self._shared)
                .get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::Sfixed64RepeatedType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<183>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_sfixed64_repeated(&self) -> bool {
            self.sfixed64_repeated_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::Sfixed64RepeatedType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<183>,
                Fields::ImplTag,
            >,
    {
        pub fn sfixed64_repeated_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::Sfixed64RepeatedType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<183>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(
                &mut self.sfixed64_repeated,
                &mut self._shared,
            )
            .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F64UnlabeledType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<191>,
                Fields::ImplTag,
            >,
    {
        pub fn f64_unlabeled(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::F64UnlabeledType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<191>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.f64_unlabeled, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F64UnlabeledType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<191>,
                Fields::ImplTag,
            >,
    {
        pub fn f64_unlabeled_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::F64UnlabeledType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<191>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.f64_unlabeled, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F64UnlabeledType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<191>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_f64_unlabeled(&self) -> bool {
            self.f64_unlabeled_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::F64UnlabeledType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<191>,
                Fields::ImplTag,
            >,
    {
        pub fn f64_unlabeled_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::F64UnlabeledType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<191>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.f64_unlabeled, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F64OptionalType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<192>,
                Fields::ImplTag,
            >,
    {
        pub fn f64_optional(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::F64OptionalType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<192>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.f64_optional, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F64OptionalType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<192>,
                Fields::ImplTag,
            >,
    {
        pub fn f64_optional_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::F64OptionalType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<192>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.f64_optional, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F64OptionalType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<192>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_f64_optional(&self) -> bool {
            self.f64_optional_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::F64OptionalType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<192>,
                Fields::ImplTag,
            >,
    {
        pub fn f64_optional_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::F64OptionalType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<192>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.f64_optional, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F64RepeatedType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<193>,
                Fields::ImplTag,
            >,
    {
        pub fn f64_repeated(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::F64RepeatedType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<193>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.f64_repeated, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F64RepeatedType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<193>,
                Fields::ImplTag,
            >,
    {
        pub fn f64_repeated_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::F64RepeatedType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<193>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.f64_repeated, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F64RepeatedType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<193>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_f64_repeated(&self) -> bool {
            self.f64_repeated_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::F64RepeatedType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<193>,
                Fields::ImplTag,
            >,
    {
        pub fn f64_repeated_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::F64RepeatedType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<193>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.f64_repeated, &mut self._shared)
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
        Fields::FloatUnlabeledType: ::std::default::Default,
        Fields::FloatOptionalType: ::std::default::Default,
        Fields::FloatRepeatedType: ::std::default::Default,
        Fields::BytesUnlabeledType: ::std::default::Default,
        Fields::BytesOptionalType: ::std::default::Default,
        Fields::BytesRepeatedType: ::std::default::Default,
        Fields::StringUnlabeledType: ::std::default::Default,
        Fields::StringOptionalType: ::std::default::Default,
        Fields::StringRepeatedType: ::std::default::Default,
        Fields::EnumUnlabeledType: ::std::default::Default,
        Fields::EnumOptionalType: ::std::default::Default,
        Fields::EnumRepeatedType: ::std::default::Default,
        Fields::SubmsgUnlabeledType: ::std::default::Default,
        Fields::SubmsgOptionalType: ::std::default::Default,
        Fields::SubmsgRepeatedType: ::std::default::Default,
        Fields::I64UnlabeledType: ::std::default::Default,
        Fields::I64OptionalType: ::std::default::Default,
        Fields::I64RepeatedType: ::std::default::Default,
        Fields::U32UnlabeledType: ::std::default::Default,
        Fields::U32OptionalType: ::std::default::Default,
        Fields::U32RepeatedType: ::std::default::Default,
        Fields::U64UnlabeledType: ::std::default::Default,
        Fields::U64OptionalType: ::std::default::Default,
        Fields::U64RepeatedType: ::std::default::Default,
        Fields::S32UnlabeledType: ::std::default::Default,
        Fields::S32OptionalType: ::std::default::Default,
        Fields::S32RepeatedType: ::std::default::Default,
        Fields::S64UnlabeledType: ::std::default::Default,
        Fields::S64OptionalType: ::std::default::Default,
        Fields::S64RepeatedType: ::std::default::Default,
        Fields::Fixed32UnlabeledType: ::std::default::Default,
        Fields::Fixed32OptionalType: ::std::default::Default,
        Fields::Fixed32RepeatedType: ::std::default::Default,
        Fields::Fixed64UnlabeledType: ::std::default::Default,
        Fields::Fixed64OptionalType: ::std::default::Default,
        Fields::Fixed64RepeatedType: ::std::default::Default,
        Fields::Sfixed32UnlabeledType: ::std::default::Default,
        Fields::Sfixed32OptionalType: ::std::default::Default,
        Fields::Sfixed32RepeatedType: ::std::default::Default,
        Fields::Sfixed64UnlabeledType: ::std::default::Default,
        Fields::Sfixed64OptionalType: ::std::default::Default,
        Fields::Sfixed64RepeatedType: ::std::default::Default,
        Fields::F64UnlabeledType: ::std::default::Default,
        Fields::F64OptionalType: ::std::default::Default,
        Fields::F64RepeatedType: ::std::default::Default,
    {
        fn default() -> Self {
            Self {
                _shared: ::std::default::Default::default(),
                _phantom: ::std::default::Default::default(),
                i32_unlabeled: ::std::default::Default::default(),
                i32_optional: ::std::default::Default::default(),
                i32_repeated: ::std::default::Default::default(),
                float_unlabeled: ::std::default::Default::default(),
                float_optional: ::std::default::Default::default(),
                float_repeated: ::std::default::Default::default(),
                bytes_unlabeled: ::std::default::Default::default(),
                bytes_optional: ::std::default::Default::default(),
                bytes_repeated: ::std::default::Default::default(),
                string_unlabeled: ::std::default::Default::default(),
                string_optional: ::std::default::Default::default(),
                string_repeated: ::std::default::Default::default(),
                enum_unlabeled: ::std::default::Default::default(),
                enum_optional: ::std::default::Default::default(),
                enum_repeated: ::std::default::Default::default(),
                submsg_unlabeled: ::std::default::Default::default(),
                submsg_optional: ::std::default::Default::default(),
                submsg_repeated: ::std::default::Default::default(),
                i64_unlabeled: ::std::default::Default::default(),
                i64_optional: ::std::default::Default::default(),
                i64_repeated: ::std::default::Default::default(),
                u32_unlabeled: ::std::default::Default::default(),
                u32_optional: ::std::default::Default::default(),
                u32_repeated: ::std::default::Default::default(),
                u64_unlabeled: ::std::default::Default::default(),
                u64_optional: ::std::default::Default::default(),
                u64_repeated: ::std::default::Default::default(),
                s32_unlabeled: ::std::default::Default::default(),
                s32_optional: ::std::default::Default::default(),
                s32_repeated: ::std::default::Default::default(),
                s64_unlabeled: ::std::default::Default::default(),
                s64_optional: ::std::default::Default::default(),
                s64_repeated: ::std::default::Default::default(),
                fixed32_unlabeled: ::std::default::Default::default(),
                fixed32_optional: ::std::default::Default::default(),
                fixed32_repeated: ::std::default::Default::default(),
                fixed64_unlabeled: ::std::default::Default::default(),
                fixed64_optional: ::std::default::Default::default(),
                fixed64_repeated: ::std::default::Default::default(),
                sfixed32_unlabeled: ::std::default::Default::default(),
                sfixed32_optional: ::std::default::Default::default(),
                sfixed32_repeated: ::std::default::Default::default(),
                sfixed64_unlabeled: ::std::default::Default::default(),
                sfixed64_optional: ::std::default::Default::default(),
                sfixed64_repeated: ::std::default::Default::default(),
                f64_unlabeled: ::std::default::Default::default(),
                f64_optional: ::std::default::Default::default(),
                f64_repeated: ::std::default::Default::default(),
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
        type FloatUnlabeledType;
        type FloatOptionalType;
        type FloatRepeatedType;
        type BytesUnlabeledType;
        type BytesOptionalType;
        type BytesRepeatedType;
        type StringUnlabeledType;
        type StringOptionalType;
        type StringRepeatedType;
        type EnumUnlabeledType;
        type EnumOptionalType;
        type EnumRepeatedType;
        type SubmsgUnlabeledType;
        type SubmsgOptionalType;
        type SubmsgRepeatedType;
        type I64UnlabeledType;
        type I64OptionalType;
        type I64RepeatedType;
        type U32UnlabeledType;
        type U32OptionalType;
        type U32RepeatedType;
        type U64UnlabeledType;
        type U64OptionalType;
        type U64RepeatedType;
        type S32UnlabeledType;
        type S32OptionalType;
        type S32RepeatedType;
        type S64UnlabeledType;
        type S64OptionalType;
        type S64RepeatedType;
        type Fixed32UnlabeledType;
        type Fixed32OptionalType;
        type Fixed32RepeatedType;
        type Fixed64UnlabeledType;
        type Fixed64OptionalType;
        type Fixed64RepeatedType;
        type Sfixed32UnlabeledType;
        type Sfixed32OptionalType;
        type Sfixed32RepeatedType;
        type Sfixed64UnlabeledType;
        type Sfixed64OptionalType;
        type Sfixed64RepeatedType;
        type F64UnlabeledType;
        type F64OptionalType;
        type F64RepeatedType;
    }

    impl MsgTemplateFieldTypes for ::puroro::internal::SimpleFields {
        type ImplTag = ::puroro::tags::SimpleImpl;
        type I32UnlabeledType = i32;
        type I32OptionalType = i32;
        type I32RepeatedType = ::std::vec::Vec<i32>;
        type FloatUnlabeledType = f32;
        type FloatOptionalType = f32;
        type FloatRepeatedType = ::std::vec::Vec<f32>;
        type BytesUnlabeledType = ::std::vec::Vec<u8>;
        type BytesOptionalType = ::std::vec::Vec<u8>;
        type BytesRepeatedType = ::std::vec::Vec<::std::vec::Vec<u8>>;
        type StringUnlabeledType = ::std::string::String;
        type StringOptionalType = ::std::string::String;
        type StringRepeatedType = ::std::vec::Vec<::std::string::String>;
        type EnumUnlabeledType = self::_puroro_root::full_coverage3::Enum;
        type EnumOptionalType = self::_puroro_root::full_coverage3::Enum;
        type EnumRepeatedType = ::std::vec::Vec<self::_puroro_root::full_coverage3::Enum>;
        type SubmsgUnlabeledType = ::std::option::Option<::std::boxed::Box<self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_simple_impl::Submsg>>;
        type SubmsgOptionalType = ::std::option::Option<::std::boxed::Box<self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_simple_impl::Submsg>>;
        type SubmsgRepeatedType = ::std::vec::Vec<
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_simple_impl::Submsg,
        >;
        type I64UnlabeledType = i64;
        type I64OptionalType = i64;
        type I64RepeatedType = ::std::vec::Vec<i64>;
        type U32UnlabeledType = u32;
        type U32OptionalType = u32;
        type U32RepeatedType = ::std::vec::Vec<u32>;
        type U64UnlabeledType = u64;
        type U64OptionalType = u64;
        type U64RepeatedType = ::std::vec::Vec<u64>;
        type S32UnlabeledType = i32;
        type S32OptionalType = i32;
        type S32RepeatedType = ::std::vec::Vec<i32>;
        type S64UnlabeledType = i64;
        type S64OptionalType = i64;
        type S64RepeatedType = ::std::vec::Vec<i64>;
        type Fixed32UnlabeledType = u32;
        type Fixed32OptionalType = u32;
        type Fixed32RepeatedType = ::std::vec::Vec<u32>;
        type Fixed64UnlabeledType = u64;
        type Fixed64OptionalType = u64;
        type Fixed64RepeatedType = ::std::vec::Vec<u64>;
        type Sfixed32UnlabeledType = i32;
        type Sfixed32OptionalType = i32;
        type Sfixed32RepeatedType = ::std::vec::Vec<i32>;
        type Sfixed64UnlabeledType = i64;
        type Sfixed64OptionalType = i64;
        type Sfixed64RepeatedType = ::std::vec::Vec<i64>;
        type F64UnlabeledType = f64;
        type F64OptionalType = f64;
        type F64RepeatedType = ::std::vec::Vec<f64>;
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
                i32_unlabeled: <Fields as SubmsgTemplateFieldTypes>::I32UnlabeledType,
                i32_optional: <Fields as SubmsgTemplateFieldTypes>::I32OptionalType,
                i64_unlabeled: <Fields as SubmsgTemplateFieldTypes>::I64UnlabeledType,
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
                Fields::I32OptionalType: ::std::default::Default,
                Fields::I64UnlabeledType: ::std::default::Default,
            {
                fn new_in(alloc: AllocatorType) -> Self {
                    Self {
                        _shared: ::std::convert::Into::into(alloc),
                        _phantom: ::std::default::Default::default(),
                        i32_unlabeled: ::std::default::Default::default(),
                        i32_optional: ::std::default::Default::default(),
                        i64_unlabeled: ::std::default::Default::default(),
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
                const DEFAULT_VALUE:
                    <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
            }
            impl ::puroro::internal::FieldProperties for SubmsgFieldProperties<2> {
                type MessageProperties = self::SubmsgMessageProperties;
                const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
                type LabelTag = ::puroro::tags::Unlabeled;
                type TypeTag = ::puroro::tags::Int32;
                const DEFAULT_VALUE:
                    <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
            }
            impl ::puroro::internal::FieldProperties for SubmsgFieldProperties<101> {
                type MessageProperties = self::SubmsgMessageProperties;
                const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
                type LabelTag = ::puroro::tags::Unlabeled;
                type TypeTag = ::puroro::tags::Int64;
                const DEFAULT_VALUE:
                    <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
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
                    ::puroro::internal::FieldAndSharedRef::new(&self.i32_unlabeled, &self._shared)
                        .get()
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
                    ::puroro::internal::FieldAndSharedRef::new(&self.i32_unlabeled, &self._shared)
                        .get_opt()
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
                    ::puroro::internal::FieldAndSharedMut::new(
                        &mut self.i32_unlabeled,
                        &mut self._shared,
                    )
                    .get_mut()
                }
            }

            impl<Fields, Shared> SubmsgTemplate<Fields, Shared>
            where
                Fields: SubmsgTemplateFieldTypes,
                for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I32OptionalType, Shared>:
                    ::puroro::internal::methods::GetFieldMethod<
                        'a,
                        self::SubmsgFieldProperties<2>,
                        Fields::ImplTag,
                    >,
            {
                pub fn i32_optional(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I32OptionalType, Shared> as
                ::puroro::internal::methods::GetFieldMethod<
                    self::SubmsgFieldProperties<2>,
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
                        self::SubmsgFieldProperties<2>,
                        Fields::ImplTag,
                    >,
            {
                pub fn i32_optional_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I32OptionalType, Shared> as
                ::puroro::internal::methods::GetOptFieldMethod<
                    self::SubmsgFieldProperties<2>,
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
                        self::SubmsgFieldProperties<2>,
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
                        self::SubmsgFieldProperties<2>,
                        Fields::ImplTag,
                    >,
            {
                pub fn i32_optional_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::I32OptionalType, Shared> as
                ::puroro::internal::methods::GetMutFieldMethod<
                    self::SubmsgFieldProperties<2>,
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

            impl<Fields, Shared> SubmsgTemplate<Fields, Shared>
            where
                Fields: SubmsgTemplateFieldTypes,
                for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I64UnlabeledType, Shared>:
                    ::puroro::internal::methods::GetFieldMethod<
                        'a,
                        self::SubmsgFieldProperties<101>,
                        Fields::ImplTag,
                    >,
            {
                pub fn i64_unlabeled(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I64UnlabeledType, Shared> as
                ::puroro::internal::methods::GetFieldMethod<
                    self::SubmsgFieldProperties<101>,
                    Fields::ImplTag,
                >
                >::GetterType{
                    use ::puroro::internal::methods::GetFieldMethod as _;
                    ::puroro::internal::FieldAndSharedRef::new(&self.i64_unlabeled, &self._shared)
                        .get()
                }
            }

            impl<Fields, Shared> SubmsgTemplate<Fields, Shared>
            where
                Fields: SubmsgTemplateFieldTypes,
                for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I64UnlabeledType, Shared>:
                    ::puroro::internal::methods::GetOptFieldMethod<
                        'a,
                        self::SubmsgFieldProperties<101>,
                        Fields::ImplTag,
                    >,
            {
                pub fn i64_unlabeled_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I64UnlabeledType, Shared> as
                ::puroro::internal::methods::GetOptFieldMethod<
                    self::SubmsgFieldProperties<101>,
                    Fields::ImplTag,
                >
                >::GetterType{
                    use ::puroro::internal::methods::GetOptFieldMethod as _;
                    ::puroro::internal::FieldAndSharedRef::new(&self.i64_unlabeled, &self._shared)
                        .get_opt()
                }
            }

            impl<Fields, Shared, OptionInnerType> SubmsgTemplate<Fields, Shared>
            where
                Fields: SubmsgTemplateFieldTypes,
                for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I64UnlabeledType, Shared>:
                    ::puroro::internal::methods::GetOptFieldMethod<
                        'a,
                        self::SubmsgFieldProperties<101>,
                        Fields::ImplTag,
                        GetterType = Option<OptionInnerType>,
                    >,
            {
                pub fn has_i64_unlabeled(&self) -> bool {
                    self.i64_unlabeled_opt().is_some()
                }
            }

            impl<Fields, Shared> SubmsgTemplate<Fields, Shared>
            where
                Fields: SubmsgTemplateFieldTypes,
                for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::I64UnlabeledType, Shared>:
                    ::puroro::internal::methods::GetMutFieldMethod<
                        'a,
                        self::SubmsgFieldProperties<101>,
                        Fields::ImplTag,
                    >,
            {
                pub fn i64_unlabeled_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::I64UnlabeledType, Shared> as
                ::puroro::internal::methods::GetMutFieldMethod<
                    self::SubmsgFieldProperties<101>,
                    Fields::ImplTag,
                >
                >::GetterType{
                    use ::puroro::internal::methods::GetMutFieldMethod as _;
                    ::puroro::internal::FieldAndSharedMut::new(
                        &mut self.i64_unlabeled,
                        &mut self._shared,
                    )
                    .get_mut()
                }
            }

            impl<Fields, Shared> ::std::default::Default for SubmsgTemplate<Fields, Shared>
            where
                Fields: SubmsgTemplateFieldTypes,
                Shared: ::std::default::Default,
                Fields::I32UnlabeledType: ::std::default::Default,
                Fields::I32OptionalType: ::std::default::Default,
                Fields::I64UnlabeledType: ::std::default::Default,
            {
                fn default() -> Self {
                    Self {
                        _shared: ::std::default::Default::default(),
                        _phantom: ::std::default::Default::default(),
                        i32_unlabeled: ::std::default::Default::default(),
                        i32_optional: ::std::default::Default::default(),
                        i64_unlabeled: ::std::default::Default::default(),
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
                type I32UnlabeledType;
                type I32OptionalType;
                type I64UnlabeledType;
            }

            impl SubmsgTemplateFieldTypes for ::puroro::internal::SimpleFields {
                type ImplTag = ::puroro::tags::SimpleImpl;
                type I32UnlabeledType = i32;
                type I32OptionalType = i32;
                type I64UnlabeledType = i64;
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
