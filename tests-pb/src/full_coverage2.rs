// A generated source code by puroro library
// package full_coverage2

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
        i32_required: <Fields as MsgTemplateFieldTypes>::I32RequiredType,
        i32_optional: <Fields as MsgTemplateFieldTypes>::I32OptionalType,
        i32_repeated: <Fields as MsgTemplateFieldTypes>::I32RepeatedType,
        float_required: <Fields as MsgTemplateFieldTypes>::FloatRequiredType,
        float_optional: <Fields as MsgTemplateFieldTypes>::FloatOptionalType,
        float_repeated: <Fields as MsgTemplateFieldTypes>::FloatRepeatedType,
        bytes_required: <Fields as MsgTemplateFieldTypes>::BytesRequiredType,
        bytes_optional: <Fields as MsgTemplateFieldTypes>::BytesOptionalType,
        bytes_repeated: <Fields as MsgTemplateFieldTypes>::BytesRepeatedType,
        string_required: <Fields as MsgTemplateFieldTypes>::StringRequiredType,
        string_optional: <Fields as MsgTemplateFieldTypes>::StringOptionalType,
        string_repeated: <Fields as MsgTemplateFieldTypes>::StringRepeatedType,
        enum_required: <Fields as MsgTemplateFieldTypes>::EnumRequiredType,
        enum_optional: <Fields as MsgTemplateFieldTypes>::EnumOptionalType,
        enum_repeated: <Fields as MsgTemplateFieldTypes>::EnumRepeatedType,
        submsg_required: <Fields as MsgTemplateFieldTypes>::SubmsgRequiredType,
        submsg_optional: <Fields as MsgTemplateFieldTypes>::SubmsgOptionalType,
        submsg_repeated: <Fields as MsgTemplateFieldTypes>::SubmsgRepeatedType,
        i64_required: <Fields as MsgTemplateFieldTypes>::I64RequiredType,
        i64_optional: <Fields as MsgTemplateFieldTypes>::I64OptionalType,
        i64_repeated: <Fields as MsgTemplateFieldTypes>::I64RepeatedType,
        u32_required: <Fields as MsgTemplateFieldTypes>::U32RequiredType,
        u32_optional: <Fields as MsgTemplateFieldTypes>::U32OptionalType,
        u32_repeated: <Fields as MsgTemplateFieldTypes>::U32RepeatedType,
        u64_required: <Fields as MsgTemplateFieldTypes>::U64RequiredType,
        u64_optional: <Fields as MsgTemplateFieldTypes>::U64OptionalType,
        u64_repeated: <Fields as MsgTemplateFieldTypes>::U64RepeatedType,
        s32_required: <Fields as MsgTemplateFieldTypes>::S32RequiredType,
        s32_optional: <Fields as MsgTemplateFieldTypes>::S32OptionalType,
        s32_repeated: <Fields as MsgTemplateFieldTypes>::S32RepeatedType,
        s64_required: <Fields as MsgTemplateFieldTypes>::S64RequiredType,
        s64_optional: <Fields as MsgTemplateFieldTypes>::S64OptionalType,
        s64_repeated: <Fields as MsgTemplateFieldTypes>::S64RepeatedType,
        fixed32_required: <Fields as MsgTemplateFieldTypes>::Fixed32RequiredType,
        fixed32_optional: <Fields as MsgTemplateFieldTypes>::Fixed32OptionalType,
        fixed32_repeated: <Fields as MsgTemplateFieldTypes>::Fixed32RepeatedType,
        fixed64_required: <Fields as MsgTemplateFieldTypes>::Fixed64RequiredType,
        fixed64_optional: <Fields as MsgTemplateFieldTypes>::Fixed64OptionalType,
        fixed64_repeated: <Fields as MsgTemplateFieldTypes>::Fixed64RepeatedType,
        sfixed32_required: <Fields as MsgTemplateFieldTypes>::Sfixed32RequiredType,
        sfixed32_optional: <Fields as MsgTemplateFieldTypes>::Sfixed32OptionalType,
        sfixed32_repeated: <Fields as MsgTemplateFieldTypes>::Sfixed32RepeatedType,
        sfixed64_required: <Fields as MsgTemplateFieldTypes>::Sfixed64RequiredType,
        sfixed64_optional: <Fields as MsgTemplateFieldTypes>::Sfixed64OptionalType,
        sfixed64_repeated: <Fields as MsgTemplateFieldTypes>::Sfixed64RepeatedType,
        f64_required: <Fields as MsgTemplateFieldTypes>::F64RequiredType,
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
        Fields::I32RequiredType: ::std::default::Default,
        Fields::I32OptionalType: ::std::default::Default,
        Fields::I32RepeatedType: ::std::default::Default,
        Fields::FloatRequiredType: ::std::default::Default,
        Fields::FloatOptionalType: ::std::default::Default,
        Fields::FloatRepeatedType: ::std::default::Default,
        Fields::BytesRequiredType: ::std::default::Default,
        Fields::BytesOptionalType: ::std::default::Default,
        Fields::BytesRepeatedType: ::std::default::Default,
        Fields::StringRequiredType: ::std::default::Default,
        Fields::StringOptionalType: ::std::default::Default,
        Fields::StringRepeatedType: ::std::default::Default,
        Fields::EnumRequiredType: ::std::default::Default,
        Fields::EnumOptionalType: ::std::default::Default,
        Fields::EnumRepeatedType: ::std::default::Default,
        Fields::SubmsgRequiredType: ::std::default::Default,
        Fields::SubmsgOptionalType: ::std::default::Default,
        Fields::SubmsgRepeatedType: ::std::default::Default,
        Fields::I64RequiredType: ::std::default::Default,
        Fields::I64OptionalType: ::std::default::Default,
        Fields::I64RepeatedType: ::std::default::Default,
        Fields::U32RequiredType: ::std::default::Default,
        Fields::U32OptionalType: ::std::default::Default,
        Fields::U32RepeatedType: ::std::default::Default,
        Fields::U64RequiredType: ::std::default::Default,
        Fields::U64OptionalType: ::std::default::Default,
        Fields::U64RepeatedType: ::std::default::Default,
        Fields::S32RequiredType: ::std::default::Default,
        Fields::S32OptionalType: ::std::default::Default,
        Fields::S32RepeatedType: ::std::default::Default,
        Fields::S64RequiredType: ::std::default::Default,
        Fields::S64OptionalType: ::std::default::Default,
        Fields::S64RepeatedType: ::std::default::Default,
        Fields::Fixed32RequiredType: ::std::default::Default,
        Fields::Fixed32OptionalType: ::std::default::Default,
        Fields::Fixed32RepeatedType: ::std::default::Default,
        Fields::Fixed64RequiredType: ::std::default::Default,
        Fields::Fixed64OptionalType: ::std::default::Default,
        Fields::Fixed64RepeatedType: ::std::default::Default,
        Fields::Sfixed32RequiredType: ::std::default::Default,
        Fields::Sfixed32OptionalType: ::std::default::Default,
        Fields::Sfixed32RepeatedType: ::std::default::Default,
        Fields::Sfixed64RequiredType: ::std::default::Default,
        Fields::Sfixed64OptionalType: ::std::default::Default,
        Fields::Sfixed64RepeatedType: ::std::default::Default,
        Fields::F64RequiredType: ::std::default::Default,
        Fields::F64OptionalType: ::std::default::Default,
        Fields::F64RepeatedType: ::std::default::Default,
    {
        fn new_in(alloc: AllocatorType) -> Self {
            Self {
                _shared: ::std::convert::Into::into(alloc),
                _phantom: ::std::default::Default::default(),
                i32_required: ::std::default::Default::default(),
                i32_optional: ::std::default::Default::default(),
                i32_repeated: ::std::default::Default::default(),
                float_required: ::std::default::Default::default(),
                float_optional: ::std::default::Default::default(),
                float_repeated: ::std::default::Default::default(),
                bytes_required: ::std::default::Default::default(),
                bytes_optional: ::std::default::Default::default(),
                bytes_repeated: ::std::default::Default::default(),
                string_required: ::std::default::Default::default(),
                string_optional: ::std::default::Default::default(),
                string_repeated: ::std::default::Default::default(),
                enum_required: ::std::default::Default::default(),
                enum_optional: ::std::default::Default::default(),
                enum_repeated: ::std::default::Default::default(),
                submsg_required: ::std::default::Default::default(),
                submsg_optional: ::std::default::Default::default(),
                submsg_repeated: ::std::default::Default::default(),
                i64_required: ::std::default::Default::default(),
                i64_optional: ::std::default::Default::default(),
                i64_repeated: ::std::default::Default::default(),
                u32_required: ::std::default::Default::default(),
                u32_optional: ::std::default::Default::default(),
                u32_repeated: ::std::default::Default::default(),
                u64_required: ::std::default::Default::default(),
                u64_optional: ::std::default::Default::default(),
                u64_repeated: ::std::default::Default::default(),
                s32_required: ::std::default::Default::default(),
                s32_optional: ::std::default::Default::default(),
                s32_repeated: ::std::default::Default::default(),
                s64_required: ::std::default::Default::default(),
                s64_optional: ::std::default::Default::default(),
                s64_repeated: ::std::default::Default::default(),
                fixed32_required: ::std::default::Default::default(),
                fixed32_optional: ::std::default::Default::default(),
                fixed32_repeated: ::std::default::Default::default(),
                fixed64_required: ::std::default::Default::default(),
                fixed64_optional: ::std::default::Default::default(),
                fixed64_repeated: ::std::default::Default::default(),
                sfixed32_required: ::std::default::Default::default(),
                sfixed32_optional: ::std::default::Default::default(),
                sfixed32_repeated: ::std::default::Default::default(),
                sfixed64_required: ::std::default::Default::default(),
                sfixed64_optional: ::std::default::Default::default(),
                sfixed64_repeated: ::std::default::Default::default(),
                f64_required: ::std::default::Default::default(),
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
        const BITFIELD_OPTIONAL_FIELD_COUNT: usize = 30;
    }

    pub struct MsgFieldProperties<const FIELD_NUMBER: i32>;
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<1> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Required;
        type TypeTag = ::puroro::tags::Int32;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<2> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 1;
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
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 2;
        type LabelTag = ::puroro::tags::Required;
        type TypeTag = ::puroro::tags::Float;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            0.0f32;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<12> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 3;
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
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 4;
        type LabelTag = ::puroro::tags::Required;
        type TypeTag = ::puroro::tags::Bytes;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            b"";
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<22> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 5;
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
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 6;
        type LabelTag = ::puroro::tags::Required;
        type TypeTag = ::puroro::tags::String;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = "";
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<32> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 7;
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
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 8;
        type LabelTag = ::puroro::tags::Required;
        type TypeTag = ::puroro::tags::Enum2<self::_puroro_root::full_coverage2::Enum>;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            self::_puroro_root::full_coverage2::Enum::Zeroth;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<42> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 9;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Enum2<self::_puroro_root::full_coverage2::Enum>;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            self::_puroro_root::full_coverage2::Enum::Zeroth;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<43> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Repeated;
        type TypeTag = ::puroro::tags::Enum2<self::_puroro_root::full_coverage2::Enum>;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            self::_puroro_root::full_coverage2::Enum::Zeroth;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<51> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Required;
        type TypeTag = ::puroro::tags::Message<
            self::_puroro_root::full_coverage2::_puroro_nested::msg::SubmsgMessageProperties,
        >;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = ();
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<52> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Message<
            self::_puroro_root::full_coverage2::_puroro_nested::msg::SubmsgMessageProperties,
        >;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = ();
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<53> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
        type LabelTag = ::puroro::tags::Repeated;
        type TypeTag = ::puroro::tags::Message<
            self::_puroro_root::full_coverage2::_puroro_nested::msg::SubmsgMessageProperties,
        >;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = ();
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<101> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 10;
        type LabelTag = ::puroro::tags::Required;
        type TypeTag = ::puroro::tags::Int64;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<102> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 11;
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
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 12;
        type LabelTag = ::puroro::tags::Required;
        type TypeTag = ::puroro::tags::UInt32;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<112> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 13;
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
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 14;
        type LabelTag = ::puroro::tags::Required;
        type TypeTag = ::puroro::tags::UInt64;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<122> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 15;
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
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 16;
        type LabelTag = ::puroro::tags::Required;
        type TypeTag = ::puroro::tags::SInt32;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<132> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 17;
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
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 18;
        type LabelTag = ::puroro::tags::Required;
        type TypeTag = ::puroro::tags::SInt64;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<142> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 19;
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
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 20;
        type LabelTag = ::puroro::tags::Required;
        type TypeTag = ::puroro::tags::Fixed32;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<152> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 21;
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
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 22;
        type LabelTag = ::puroro::tags::Required;
        type TypeTag = ::puroro::tags::Fixed64;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<162> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 23;
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
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 24;
        type LabelTag = ::puroro::tags::Required;
        type TypeTag = ::puroro::tags::SFixed32;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<172> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 25;
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
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 26;
        type LabelTag = ::puroro::tags::Required;
        type TypeTag = ::puroro::tags::SFixed64;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<182> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 27;
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
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 28;
        type LabelTag = ::puroro::tags::Required;
        type TypeTag = ::puroro::tags::Double;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            0.0f64;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<192> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 29;
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
        ::puroro::internal::SimpleShared<{ (30 + 31) / 32 }>,
    >;

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I32RequiredType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<1>,
                Fields::ImplTag,
            >,
    {
        pub fn i32_required(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I32RequiredType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<1>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.i32_required, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I32RequiredType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<1>,
                Fields::ImplTag,
            >,
    {
        pub fn i32_required_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I32RequiredType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<1>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.i32_required, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I32RequiredType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<1>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_i32_required(&self) -> bool {
            self.i32_required_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::I32RequiredType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<1>,
                Fields::ImplTag,
            >,
    {
        pub fn i32_required_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::I32RequiredType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<1>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.i32_required, &mut self._shared)
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
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::FloatRequiredType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<11>,
                Fields::ImplTag,
            >,
    {
        pub fn float_required(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::FloatRequiredType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<11>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.float_required, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::FloatRequiredType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<11>,
                Fields::ImplTag,
            >,
    {
        pub fn float_required_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::FloatRequiredType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<11>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.float_required, &self._shared)
                .get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::FloatRequiredType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<11>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_float_required(&self) -> bool {
            self.float_required_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::FloatRequiredType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<11>,
                Fields::ImplTag,
            >,
    {
        pub fn float_required_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::FloatRequiredType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<11>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.float_required, &mut self._shared)
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
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::BytesRequiredType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<21>,
                Fields::ImplTag,
            >,
    {
        pub fn bytes_required(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::BytesRequiredType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<21>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.bytes_required, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::BytesRequiredType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<21>,
                Fields::ImplTag,
            >,
    {
        pub fn bytes_required_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::BytesRequiredType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<21>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.bytes_required, &self._shared)
                .get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::BytesRequiredType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<21>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_bytes_required(&self) -> bool {
            self.bytes_required_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::BytesRequiredType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<21>,
                Fields::ImplTag,
            >,
    {
        pub fn bytes_required_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::BytesRequiredType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<21>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.bytes_required, &mut self._shared)
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
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::StringRequiredType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<31>,
                Fields::ImplTag,
            >,
    {
        pub fn string_required(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::StringRequiredType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<31>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.string_required, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::StringRequiredType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<31>,
                Fields::ImplTag,
            >,
    {
        pub fn string_required_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::StringRequiredType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<31>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.string_required, &self._shared)
                .get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::StringRequiredType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<31>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_string_required(&self) -> bool {
            self.string_required_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::StringRequiredType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<31>,
                Fields::ImplTag,
            >,
    {
        pub fn string_required_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::StringRequiredType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<31>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.string_required, &mut self._shared)
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
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::EnumRequiredType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<41>,
                Fields::ImplTag,
            >,
    {
        pub fn enum_required(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::EnumRequiredType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<41>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.enum_required, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::EnumRequiredType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<41>,
                Fields::ImplTag,
            >,
    {
        pub fn enum_required_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::EnumRequiredType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<41>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.enum_required, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::EnumRequiredType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<41>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_enum_required(&self) -> bool {
            self.enum_required_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::EnumRequiredType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<41>,
                Fields::ImplTag,
            >,
    {
        pub fn enum_required_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::EnumRequiredType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<41>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.enum_required, &mut self._shared)
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
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::SubmsgRequiredType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<51>,
                Fields::ImplTag,
            >,
    {
        pub fn submsg_required(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::SubmsgRequiredType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<51>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.submsg_required, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::SubmsgRequiredType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<51>,
                Fields::ImplTag,
            >,
    {
        pub fn submsg_required_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::SubmsgRequiredType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<51>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.submsg_required, &self._shared)
                .get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::SubmsgRequiredType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<51>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_submsg_required(&self) -> bool {
            self.submsg_required_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::SubmsgRequiredType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<51>,
                Fields::ImplTag,
            >,
    {
        pub fn submsg_required_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::SubmsgRequiredType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<51>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.submsg_required, &mut self._shared)
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
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I64RequiredType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<101>,
                Fields::ImplTag,
            >,
    {
        pub fn i64_required(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I64RequiredType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<101>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.i64_required, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I64RequiredType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<101>,
                Fields::ImplTag,
            >,
    {
        pub fn i64_required_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I64RequiredType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<101>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.i64_required, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I64RequiredType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<101>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_i64_required(&self) -> bool {
            self.i64_required_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::I64RequiredType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<101>,
                Fields::ImplTag,
            >,
    {
        pub fn i64_required_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::I64RequiredType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<101>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.i64_required, &mut self._shared)
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
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U32RequiredType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<111>,
                Fields::ImplTag,
            >,
    {
        pub fn u32_required(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::U32RequiredType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<111>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.u32_required, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U32RequiredType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<111>,
                Fields::ImplTag,
            >,
    {
        pub fn u32_required_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::U32RequiredType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<111>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.u32_required, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U32RequiredType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<111>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_u32_required(&self) -> bool {
            self.u32_required_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::U32RequiredType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<111>,
                Fields::ImplTag,
            >,
    {
        pub fn u32_required_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::U32RequiredType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<111>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.u32_required, &mut self._shared)
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
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U64RequiredType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<121>,
                Fields::ImplTag,
            >,
    {
        pub fn u64_required(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::U64RequiredType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<121>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.u64_required, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U64RequiredType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<121>,
                Fields::ImplTag,
            >,
    {
        pub fn u64_required_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::U64RequiredType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<121>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.u64_required, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U64RequiredType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<121>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_u64_required(&self) -> bool {
            self.u64_required_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::U64RequiredType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<121>,
                Fields::ImplTag,
            >,
    {
        pub fn u64_required_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::U64RequiredType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<121>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.u64_required, &mut self._shared)
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
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::S32RequiredType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<131>,
                Fields::ImplTag,
            >,
    {
        pub fn s32_required(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::S32RequiredType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<131>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.s32_required, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::S32RequiredType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<131>,
                Fields::ImplTag,
            >,
    {
        pub fn s32_required_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::S32RequiredType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<131>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.s32_required, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::S32RequiredType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<131>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_s32_required(&self) -> bool {
            self.s32_required_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::S32RequiredType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<131>,
                Fields::ImplTag,
            >,
    {
        pub fn s32_required_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::S32RequiredType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<131>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.s32_required, &mut self._shared)
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
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::S64RequiredType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<141>,
                Fields::ImplTag,
            >,
    {
        pub fn s64_required(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::S64RequiredType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<141>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.s64_required, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::S64RequiredType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<141>,
                Fields::ImplTag,
            >,
    {
        pub fn s64_required_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::S64RequiredType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<141>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.s64_required, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::S64RequiredType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<141>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_s64_required(&self) -> bool {
            self.s64_required_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::S64RequiredType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<141>,
                Fields::ImplTag,
            >,
    {
        pub fn s64_required_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::S64RequiredType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<141>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.s64_required, &mut self._shared)
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
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::Fixed32RequiredType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<151>,
                Fields::ImplTag,
            >,
    {
        pub fn fixed32_required(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::Fixed32RequiredType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<151>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.fixed32_required, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::Fixed32RequiredType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<151>,
                Fields::ImplTag,
            >,
    {
        pub fn fixed32_required_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::Fixed32RequiredType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<151>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.fixed32_required, &self._shared)
                .get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::Fixed32RequiredType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<151>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_fixed32_required(&self) -> bool {
            self.fixed32_required_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::Fixed32RequiredType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<151>,
                Fields::ImplTag,
            >,
    {
        pub fn fixed32_required_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::Fixed32RequiredType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<151>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(
                &mut self.fixed32_required,
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
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::Fixed64RequiredType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<161>,
                Fields::ImplTag,
            >,
    {
        pub fn fixed64_required(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::Fixed64RequiredType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<161>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.fixed64_required, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::Fixed64RequiredType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<161>,
                Fields::ImplTag,
            >,
    {
        pub fn fixed64_required_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::Fixed64RequiredType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<161>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.fixed64_required, &self._shared)
                .get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::Fixed64RequiredType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<161>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_fixed64_required(&self) -> bool {
            self.fixed64_required_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::Fixed64RequiredType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<161>,
                Fields::ImplTag,
            >,
    {
        pub fn fixed64_required_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::Fixed64RequiredType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<161>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(
                &mut self.fixed64_required,
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
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::Sfixed32RequiredType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<171>,
                Fields::ImplTag,
            >,
    {
        pub fn sfixed32_required(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::Sfixed32RequiredType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<171>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.sfixed32_required, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::Sfixed32RequiredType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<171>,
                Fields::ImplTag,
            >,
    {
        pub fn sfixed32_required_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::Sfixed32RequiredType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<171>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.sfixed32_required, &self._shared)
                .get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::Sfixed32RequiredType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<171>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_sfixed32_required(&self) -> bool {
            self.sfixed32_required_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::Sfixed32RequiredType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<171>,
                Fields::ImplTag,
            >,
    {
        pub fn sfixed32_required_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::Sfixed32RequiredType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<171>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(
                &mut self.sfixed32_required,
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
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::Sfixed64RequiredType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<181>,
                Fields::ImplTag,
            >,
    {
        pub fn sfixed64_required(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::Sfixed64RequiredType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<181>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.sfixed64_required, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::Sfixed64RequiredType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<181>,
                Fields::ImplTag,
            >,
    {
        pub fn sfixed64_required_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::Sfixed64RequiredType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<181>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.sfixed64_required, &self._shared)
                .get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::Sfixed64RequiredType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<181>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_sfixed64_required(&self) -> bool {
            self.sfixed64_required_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::Sfixed64RequiredType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<181>,
                Fields::ImplTag,
            >,
    {
        pub fn sfixed64_required_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::Sfixed64RequiredType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<181>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(
                &mut self.sfixed64_required,
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
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F64RequiredType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<191>,
                Fields::ImplTag,
            >,
    {
        pub fn f64_required(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::F64RequiredType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<191>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.f64_required, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F64RequiredType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<191>,
                Fields::ImplTag,
            >,
    {
        pub fn f64_required_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::F64RequiredType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<191>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.f64_required, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F64RequiredType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<191>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_f64_required(&self) -> bool {
            self.f64_required_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::F64RequiredType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<191>,
                Fields::ImplTag,
            >,
    {
        pub fn f64_required_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::F64RequiredType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<191>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.f64_required, &mut self._shared)
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
        Fields::I32RequiredType: ::std::default::Default,
        Fields::I32OptionalType: ::std::default::Default,
        Fields::I32RepeatedType: ::std::default::Default,
        Fields::FloatRequiredType: ::std::default::Default,
        Fields::FloatOptionalType: ::std::default::Default,
        Fields::FloatRepeatedType: ::std::default::Default,
        Fields::BytesRequiredType: ::std::default::Default,
        Fields::BytesOptionalType: ::std::default::Default,
        Fields::BytesRepeatedType: ::std::default::Default,
        Fields::StringRequiredType: ::std::default::Default,
        Fields::StringOptionalType: ::std::default::Default,
        Fields::StringRepeatedType: ::std::default::Default,
        Fields::EnumRequiredType: ::std::default::Default,
        Fields::EnumOptionalType: ::std::default::Default,
        Fields::EnumRepeatedType: ::std::default::Default,
        Fields::SubmsgRequiredType: ::std::default::Default,
        Fields::SubmsgOptionalType: ::std::default::Default,
        Fields::SubmsgRepeatedType: ::std::default::Default,
        Fields::I64RequiredType: ::std::default::Default,
        Fields::I64OptionalType: ::std::default::Default,
        Fields::I64RepeatedType: ::std::default::Default,
        Fields::U32RequiredType: ::std::default::Default,
        Fields::U32OptionalType: ::std::default::Default,
        Fields::U32RepeatedType: ::std::default::Default,
        Fields::U64RequiredType: ::std::default::Default,
        Fields::U64OptionalType: ::std::default::Default,
        Fields::U64RepeatedType: ::std::default::Default,
        Fields::S32RequiredType: ::std::default::Default,
        Fields::S32OptionalType: ::std::default::Default,
        Fields::S32RepeatedType: ::std::default::Default,
        Fields::S64RequiredType: ::std::default::Default,
        Fields::S64OptionalType: ::std::default::Default,
        Fields::S64RepeatedType: ::std::default::Default,
        Fields::Fixed32RequiredType: ::std::default::Default,
        Fields::Fixed32OptionalType: ::std::default::Default,
        Fields::Fixed32RepeatedType: ::std::default::Default,
        Fields::Fixed64RequiredType: ::std::default::Default,
        Fields::Fixed64OptionalType: ::std::default::Default,
        Fields::Fixed64RepeatedType: ::std::default::Default,
        Fields::Sfixed32RequiredType: ::std::default::Default,
        Fields::Sfixed32OptionalType: ::std::default::Default,
        Fields::Sfixed32RepeatedType: ::std::default::Default,
        Fields::Sfixed64RequiredType: ::std::default::Default,
        Fields::Sfixed64OptionalType: ::std::default::Default,
        Fields::Sfixed64RepeatedType: ::std::default::Default,
        Fields::F64RequiredType: ::std::default::Default,
        Fields::F64OptionalType: ::std::default::Default,
        Fields::F64RepeatedType: ::std::default::Default,
    {
        fn default() -> Self {
            Self {
                _shared: ::std::default::Default::default(),
                _phantom: ::std::default::Default::default(),
                i32_required: ::std::default::Default::default(),
                i32_optional: ::std::default::Default::default(),
                i32_repeated: ::std::default::Default::default(),
                float_required: ::std::default::Default::default(),
                float_optional: ::std::default::Default::default(),
                float_repeated: ::std::default::Default::default(),
                bytes_required: ::std::default::Default::default(),
                bytes_optional: ::std::default::Default::default(),
                bytes_repeated: ::std::default::Default::default(),
                string_required: ::std::default::Default::default(),
                string_optional: ::std::default::Default::default(),
                string_repeated: ::std::default::Default::default(),
                enum_required: ::std::default::Default::default(),
                enum_optional: ::std::default::Default::default(),
                enum_repeated: ::std::default::Default::default(),
                submsg_required: ::std::default::Default::default(),
                submsg_optional: ::std::default::Default::default(),
                submsg_repeated: ::std::default::Default::default(),
                i64_required: ::std::default::Default::default(),
                i64_optional: ::std::default::Default::default(),
                i64_repeated: ::std::default::Default::default(),
                u32_required: ::std::default::Default::default(),
                u32_optional: ::std::default::Default::default(),
                u32_repeated: ::std::default::Default::default(),
                u64_required: ::std::default::Default::default(),
                u64_optional: ::std::default::Default::default(),
                u64_repeated: ::std::default::Default::default(),
                s32_required: ::std::default::Default::default(),
                s32_optional: ::std::default::Default::default(),
                s32_repeated: ::std::default::Default::default(),
                s64_required: ::std::default::Default::default(),
                s64_optional: ::std::default::Default::default(),
                s64_repeated: ::std::default::Default::default(),
                fixed32_required: ::std::default::Default::default(),
                fixed32_optional: ::std::default::Default::default(),
                fixed32_repeated: ::std::default::Default::default(),
                fixed64_required: ::std::default::Default::default(),
                fixed64_optional: ::std::default::Default::default(),
                fixed64_repeated: ::std::default::Default::default(),
                sfixed32_required: ::std::default::Default::default(),
                sfixed32_optional: ::std::default::Default::default(),
                sfixed32_repeated: ::std::default::Default::default(),
                sfixed64_required: ::std::default::Default::default(),
                sfixed64_optional: ::std::default::Default::default(),
                sfixed64_repeated: ::std::default::Default::default(),
                f64_required: ::std::default::Default::default(),
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
        type I32RequiredType;
        type I32OptionalType;
        type I32RepeatedType;
        type FloatRequiredType;
        type FloatOptionalType;
        type FloatRepeatedType;
        type BytesRequiredType;
        type BytesOptionalType;
        type BytesRepeatedType;
        type StringRequiredType;
        type StringOptionalType;
        type StringRepeatedType;
        type EnumRequiredType;
        type EnumOptionalType;
        type EnumRepeatedType;
        type SubmsgRequiredType;
        type SubmsgOptionalType;
        type SubmsgRepeatedType;
        type I64RequiredType;
        type I64OptionalType;
        type I64RepeatedType;
        type U32RequiredType;
        type U32OptionalType;
        type U32RepeatedType;
        type U64RequiredType;
        type U64OptionalType;
        type U64RepeatedType;
        type S32RequiredType;
        type S32OptionalType;
        type S32RepeatedType;
        type S64RequiredType;
        type S64OptionalType;
        type S64RepeatedType;
        type Fixed32RequiredType;
        type Fixed32OptionalType;
        type Fixed32RepeatedType;
        type Fixed64RequiredType;
        type Fixed64OptionalType;
        type Fixed64RepeatedType;
        type Sfixed32RequiredType;
        type Sfixed32OptionalType;
        type Sfixed32RepeatedType;
        type Sfixed64RequiredType;
        type Sfixed64OptionalType;
        type Sfixed64RepeatedType;
        type F64RequiredType;
        type F64OptionalType;
        type F64RepeatedType;
    }

    impl MsgTemplateFieldTypes for ::puroro::internal::SimpleFields {
        type ImplTag = ::puroro::tags::SimpleImpl;
        type I32RequiredType = i32;
        type I32OptionalType = i32;
        type I32RepeatedType = ::std::vec::Vec<i32>;
        type FloatRequiredType = f32;
        type FloatOptionalType = f32;
        type FloatRepeatedType = ::std::vec::Vec<f32>;
        type BytesRequiredType = ::std::vec::Vec<u8>;
        type BytesOptionalType = ::std::vec::Vec<u8>;
        type BytesRepeatedType = ::std::vec::Vec<::std::vec::Vec<u8>>;
        type StringRequiredType = ::std::string::String;
        type StringOptionalType = ::std::string::String;
        type StringRepeatedType = ::std::vec::Vec<::std::string::String>;
        type EnumRequiredType = self::_puroro_root::full_coverage2::Enum;
        type EnumOptionalType = self::_puroro_root::full_coverage2::Enum;
        type EnumRepeatedType = ::std::vec::Vec<self::_puroro_root::full_coverage2::Enum>;
        type SubmsgRequiredType = ::std::option::Option<::std::boxed::Box<self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_simple_impl::Submsg>>;
        type SubmsgOptionalType = ::std::option::Option<::std::boxed::Box<self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_simple_impl::Submsg>>;
        type SubmsgRepeatedType = ::std::vec::Vec<
            self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_simple_impl::Submsg,
        >;
        type I64RequiredType = i64;
        type I64OptionalType = i64;
        type I64RepeatedType = ::std::vec::Vec<i64>;
        type U32RequiredType = u32;
        type U32OptionalType = u32;
        type U32RepeatedType = ::std::vec::Vec<u32>;
        type U64RequiredType = u64;
        type U64OptionalType = u64;
        type U64RepeatedType = ::std::vec::Vec<u64>;
        type S32RequiredType = i32;
        type S32OptionalType = i32;
        type S32RepeatedType = ::std::vec::Vec<i32>;
        type S64RequiredType = i64;
        type S64OptionalType = i64;
        type S64RepeatedType = ::std::vec::Vec<i64>;
        type Fixed32RequiredType = u32;
        type Fixed32OptionalType = u32;
        type Fixed32RepeatedType = ::std::vec::Vec<u32>;
        type Fixed64RequiredType = u64;
        type Fixed64OptionalType = u64;
        type Fixed64RepeatedType = ::std::vec::Vec<u64>;
        type Sfixed32RequiredType = i32;
        type Sfixed32OptionalType = i32;
        type Sfixed32RepeatedType = ::std::vec::Vec<i32>;
        type Sfixed64RequiredType = i64;
        type Sfixed64OptionalType = i64;
        type Sfixed64RepeatedType = ::std::vec::Vec<i64>;
        type F64RequiredType = f64;
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
                i32_required: <Fields as SubmsgTemplateFieldTypes>::I32RequiredType,
                i64_required: <Fields as SubmsgTemplateFieldTypes>::I64RequiredType,
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
                Fields::I32RequiredType: ::std::default::Default,
                Fields::I64RequiredType: ::std::default::Default,
            {
                fn new_in(alloc: AllocatorType) -> Self {
                    Self {
                        _shared: ::std::convert::Into::into(alloc),
                        _phantom: ::std::default::Default::default(),
                        i32_required: ::std::default::Default::default(),
                        i64_required: ::std::default::Default::default(),
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
                const BITFIELD_OPTIONAL_FIELD_COUNT: usize = 2;
            }

            pub struct SubmsgFieldProperties<const FIELD_NUMBER: i32>;
            impl ::puroro::internal::FieldProperties for SubmsgFieldProperties<1> {
                type MessageProperties = self::SubmsgMessageProperties;
                const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
                type LabelTag = ::puroro::tags::Required;
                type TypeTag = ::puroro::tags::Int32;
                const DEFAULT_VALUE:
                    <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
            }
            impl ::puroro::internal::FieldProperties for SubmsgFieldProperties<101> {
                type MessageProperties = self::SubmsgMessageProperties;
                const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 1;
                type LabelTag = ::puroro::tags::Required;
                type TypeTag = ::puroro::tags::Int64;
                const DEFAULT_VALUE:
                    <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
            }

            pub type SubmsgSimple2 = SubmsgTemplate<
                ::puroro::internal::SimpleFields,
                ::puroro::internal::SimpleShared<{ (2 + 31) / 32 }>,
            >;

            impl<Fields, Shared> SubmsgTemplate<Fields, Shared>
            where
                Fields: SubmsgTemplateFieldTypes,
                for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I32RequiredType, Shared>:
                    ::puroro::internal::methods::GetFieldMethod<
                        'a,
                        self::SubmsgFieldProperties<1>,
                        Fields::ImplTag,
                    >,
            {
                pub fn i32_required(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I32RequiredType, Shared> as
                ::puroro::internal::methods::GetFieldMethod<
                    self::SubmsgFieldProperties<1>,
                    Fields::ImplTag,
                >
                >::GetterType{
                    use ::puroro::internal::methods::GetFieldMethod as _;
                    ::puroro::internal::FieldAndSharedRef::new(&self.i32_required, &self._shared)
                        .get()
                }
            }

            impl<Fields, Shared> SubmsgTemplate<Fields, Shared>
            where
                Fields: SubmsgTemplateFieldTypes,
                for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I32RequiredType, Shared>:
                    ::puroro::internal::methods::GetOptFieldMethod<
                        'a,
                        self::SubmsgFieldProperties<1>,
                        Fields::ImplTag,
                    >,
            {
                pub fn i32_required_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I32RequiredType, Shared> as
                ::puroro::internal::methods::GetOptFieldMethod<
                    self::SubmsgFieldProperties<1>,
                    Fields::ImplTag,
                >
                >::GetterType{
                    use ::puroro::internal::methods::GetOptFieldMethod as _;
                    ::puroro::internal::FieldAndSharedRef::new(&self.i32_required, &self._shared)
                        .get_opt()
                }
            }

            impl<Fields, Shared, OptionInnerType> SubmsgTemplate<Fields, Shared>
            where
                Fields: SubmsgTemplateFieldTypes,
                for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I32RequiredType, Shared>:
                    ::puroro::internal::methods::GetOptFieldMethod<
                        'a,
                        self::SubmsgFieldProperties<1>,
                        Fields::ImplTag,
                        GetterType = Option<OptionInnerType>,
                    >,
            {
                pub fn has_i32_required(&self) -> bool {
                    self.i32_required_opt().is_some()
                }
            }

            impl<Fields, Shared> SubmsgTemplate<Fields, Shared>
            where
                Fields: SubmsgTemplateFieldTypes,
                for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::I32RequiredType, Shared>:
                    ::puroro::internal::methods::GetMutFieldMethod<
                        'a,
                        self::SubmsgFieldProperties<1>,
                        Fields::ImplTag,
                    >,
            {
                pub fn i32_required_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::I32RequiredType, Shared> as
                ::puroro::internal::methods::GetMutFieldMethod<
                    self::SubmsgFieldProperties<1>,
                    Fields::ImplTag,
                >
                >::GetterType{
                    use ::puroro::internal::methods::GetMutFieldMethod as _;
                    ::puroro::internal::FieldAndSharedMut::new(
                        &mut self.i32_required,
                        &mut self._shared,
                    )
                    .get_mut()
                }
            }

            impl<Fields, Shared> SubmsgTemplate<Fields, Shared>
            where
                Fields: SubmsgTemplateFieldTypes,
                for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I64RequiredType, Shared>:
                    ::puroro::internal::methods::GetFieldMethod<
                        'a,
                        self::SubmsgFieldProperties<101>,
                        Fields::ImplTag,
                    >,
            {
                pub fn i64_required(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I64RequiredType, Shared> as
                ::puroro::internal::methods::GetFieldMethod<
                    self::SubmsgFieldProperties<101>,
                    Fields::ImplTag,
                >
                >::GetterType{
                    use ::puroro::internal::methods::GetFieldMethod as _;
                    ::puroro::internal::FieldAndSharedRef::new(&self.i64_required, &self._shared)
                        .get()
                }
            }

            impl<Fields, Shared> SubmsgTemplate<Fields, Shared>
            where
                Fields: SubmsgTemplateFieldTypes,
                for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I64RequiredType, Shared>:
                    ::puroro::internal::methods::GetOptFieldMethod<
                        'a,
                        self::SubmsgFieldProperties<101>,
                        Fields::ImplTag,
                    >,
            {
                pub fn i64_required_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I64RequiredType, Shared> as
                ::puroro::internal::methods::GetOptFieldMethod<
                    self::SubmsgFieldProperties<101>,
                    Fields::ImplTag,
                >
                >::GetterType{
                    use ::puroro::internal::methods::GetOptFieldMethod as _;
                    ::puroro::internal::FieldAndSharedRef::new(&self.i64_required, &self._shared)
                        .get_opt()
                }
            }

            impl<Fields, Shared, OptionInnerType> SubmsgTemplate<Fields, Shared>
            where
                Fields: SubmsgTemplateFieldTypes,
                for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I64RequiredType, Shared>:
                    ::puroro::internal::methods::GetOptFieldMethod<
                        'a,
                        self::SubmsgFieldProperties<101>,
                        Fields::ImplTag,
                        GetterType = Option<OptionInnerType>,
                    >,
            {
                pub fn has_i64_required(&self) -> bool {
                    self.i64_required_opt().is_some()
                }
            }

            impl<Fields, Shared> SubmsgTemplate<Fields, Shared>
            where
                Fields: SubmsgTemplateFieldTypes,
                for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::I64RequiredType, Shared>:
                    ::puroro::internal::methods::GetMutFieldMethod<
                        'a,
                        self::SubmsgFieldProperties<101>,
                        Fields::ImplTag,
                    >,
            {
                pub fn i64_required_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::I64RequiredType, Shared> as
                ::puroro::internal::methods::GetMutFieldMethod<
                    self::SubmsgFieldProperties<101>,
                    Fields::ImplTag,
                >
                >::GetterType{
                    use ::puroro::internal::methods::GetMutFieldMethod as _;
                    ::puroro::internal::FieldAndSharedMut::new(
                        &mut self.i64_required,
                        &mut self._shared,
                    )
                    .get_mut()
                }
            }

            impl<Fields, Shared> ::std::default::Default for SubmsgTemplate<Fields, Shared>
            where
                Fields: SubmsgTemplateFieldTypes,
                Shared: ::std::default::Default,
                Fields::I32RequiredType: ::std::default::Default,
                Fields::I64RequiredType: ::std::default::Default,
            {
                fn default() -> Self {
                    Self {
                        _shared: ::std::default::Default::default(),
                        _phantom: ::std::default::Default::default(),
                        i32_required: ::std::default::Default::default(),
                        i64_required: ::std::default::Default::default(),
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
                type I32RequiredType;
                type I64RequiredType;
            }

            impl SubmsgTemplateFieldTypes for ::puroro::internal::SimpleFields {
                type ImplTag = ::puroro::tags::SimpleImpl;
                type I32RequiredType = i32;
                type I64RequiredType = i64;
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
