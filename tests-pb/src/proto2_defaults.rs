// A generated source code by puroro library
// package proto2_defaults

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
        i32_default: <Fields as MsgTemplateFieldTypes>::I32DefaultType,
        i32_0: <Fields as MsgTemplateFieldTypes>::I320Type,
        i32_42: <Fields as MsgTemplateFieldTypes>::I3242Type,
        i32_m42: <Fields as MsgTemplateFieldTypes>::I32M42Type,
        i32_2147483647: <Fields as MsgTemplateFieldTypes>::I322147483647Type,
        i32_m2147483648: <Fields as MsgTemplateFieldTypes>::I32M2147483648Type,
        i32_0123: <Fields as MsgTemplateFieldTypes>::I320123Type,
        i32_0x123: <Fields as MsgTemplateFieldTypes>::I320x123Type,
        u32_default: <Fields as MsgTemplateFieldTypes>::U32DefaultType,
        u32_0: <Fields as MsgTemplateFieldTypes>::U320Type,
        u32_42: <Fields as MsgTemplateFieldTypes>::U3242Type,
        u32_4294967295: <Fields as MsgTemplateFieldTypes>::U324294967295Type,
        u32_0123: <Fields as MsgTemplateFieldTypes>::U320123Type,
        u32_0x123: <Fields as MsgTemplateFieldTypes>::U320x123Type,
        i64_default: <Fields as MsgTemplateFieldTypes>::I64DefaultType,
        i64_0: <Fields as MsgTemplateFieldTypes>::I640Type,
        i64_42: <Fields as MsgTemplateFieldTypes>::I6442Type,
        i64_m42: <Fields as MsgTemplateFieldTypes>::I64M42Type,
        i64_9223372036854775807: <Fields as MsgTemplateFieldTypes>::I649223372036854775807Type,
        i64_m9223372036854775808: <Fields as MsgTemplateFieldTypes>::I64M9223372036854775808Type,
        i64_0123: <Fields as MsgTemplateFieldTypes>::I640123Type,
        i64_0x123: <Fields as MsgTemplateFieldTypes>::I640x123Type,
        u64_default: <Fields as MsgTemplateFieldTypes>::U64DefaultType,
        u64_0: <Fields as MsgTemplateFieldTypes>::U640Type,
        u64_42: <Fields as MsgTemplateFieldTypes>::U6442Type,
        u64_18446744073709551615: <Fields as MsgTemplateFieldTypes>::U6418446744073709551615Type,
        u64_0123: <Fields as MsgTemplateFieldTypes>::U640123Type,
        u64_0x123: <Fields as MsgTemplateFieldTypes>::U640x123Type,
        f32_default: <Fields as MsgTemplateFieldTypes>::F32DefaultType,
        f32_0: <Fields as MsgTemplateFieldTypes>::F320Type,
        f32_m0: <Fields as MsgTemplateFieldTypes>::F32M0Type,
        f32_0p: <Fields as MsgTemplateFieldTypes>::F320pType,
        f32_p0: <Fields as MsgTemplateFieldTypes>::F32P0Type,
        f32_0p0: <Fields as MsgTemplateFieldTypes>::F320p0Type,
        f32_42: <Fields as MsgTemplateFieldTypes>::F3242Type,
        f32_m42: <Fields as MsgTemplateFieldTypes>::F32M42Type,
        f32_0p25: <Fields as MsgTemplateFieldTypes>::F320p25Type,
        f32_1p5e2: <Fields as MsgTemplateFieldTypes>::F321p5e2Type,
        f32_inf: <Fields as MsgTemplateFieldTypes>::F32InfType,
        f32_minf: <Fields as MsgTemplateFieldTypes>::F32MinfType,
        f32_nan: <Fields as MsgTemplateFieldTypes>::F32NanType,
        f32_mnan: <Fields as MsgTemplateFieldTypes>::F32MnanType,
        bool_default: <Fields as MsgTemplateFieldTypes>::BoolDefaultType,
        bool_true: <Fields as MsgTemplateFieldTypes>::BoolTrueType,
        bool_false: <Fields as MsgTemplateFieldTypes>::BoolFalseType,
        string_default: <Fields as MsgTemplateFieldTypes>::StringDefaultType,
        string_empty: <Fields as MsgTemplateFieldTypes>::StringEmptyType,
        string_abc: <Fields as MsgTemplateFieldTypes>::StringAbcType,
        string_aiu: <Fields as MsgTemplateFieldTypes>::StringAiuType,
        string_backslash: <Fields as MsgTemplateFieldTypes>::StringBackslashType,
        string_tab: <Fields as MsgTemplateFieldTypes>::StringTabType,
        string_crlf: <Fields as MsgTemplateFieldTypes>::StringCrlfType,
        bytes_default: <Fields as MsgTemplateFieldTypes>::BytesDefaultType,
        bytes_empty: <Fields as MsgTemplateFieldTypes>::BytesEmptyType,
        bytes_abc: <Fields as MsgTemplateFieldTypes>::BytesAbcType,
        bytes_aiu: <Fields as MsgTemplateFieldTypes>::BytesAiuType,
        bytes_backslash: <Fields as MsgTemplateFieldTypes>::BytesBackslashType,
        bytes_tab: <Fields as MsgTemplateFieldTypes>::BytesTabType,
        bytes_crlf: <Fields as MsgTemplateFieldTypes>::BytesCrlfType,
        enum_default: <Fields as MsgTemplateFieldTypes>::EnumDefaultType,
        enum_one: <Fields as MsgTemplateFieldTypes>::EnumOneType,
        enum_fourty_two: <Fields as MsgTemplateFieldTypes>::EnumFourtyTwoType,
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
        Fields::I32DefaultType: ::std::default::Default,
        Fields::I320Type: ::std::default::Default,
        Fields::I3242Type: ::std::default::Default,
        Fields::I32M42Type: ::std::default::Default,
        Fields::I322147483647Type: ::std::default::Default,
        Fields::I32M2147483648Type: ::std::default::Default,
        Fields::I320123Type: ::std::default::Default,
        Fields::I320x123Type: ::std::default::Default,
        Fields::U32DefaultType: ::std::default::Default,
        Fields::U320Type: ::std::default::Default,
        Fields::U3242Type: ::std::default::Default,
        Fields::U324294967295Type: ::std::default::Default,
        Fields::U320123Type: ::std::default::Default,
        Fields::U320x123Type: ::std::default::Default,
        Fields::I64DefaultType: ::std::default::Default,
        Fields::I640Type: ::std::default::Default,
        Fields::I6442Type: ::std::default::Default,
        Fields::I64M42Type: ::std::default::Default,
        Fields::I649223372036854775807Type: ::std::default::Default,
        Fields::I64M9223372036854775808Type: ::std::default::Default,
        Fields::I640123Type: ::std::default::Default,
        Fields::I640x123Type: ::std::default::Default,
        Fields::U64DefaultType: ::std::default::Default,
        Fields::U640Type: ::std::default::Default,
        Fields::U6442Type: ::std::default::Default,
        Fields::U6418446744073709551615Type: ::std::default::Default,
        Fields::U640123Type: ::std::default::Default,
        Fields::U640x123Type: ::std::default::Default,
        Fields::F32DefaultType: ::std::default::Default,
        Fields::F320Type: ::std::default::Default,
        Fields::F32M0Type: ::std::default::Default,
        Fields::F320pType: ::std::default::Default,
        Fields::F32P0Type: ::std::default::Default,
        Fields::F320p0Type: ::std::default::Default,
        Fields::F3242Type: ::std::default::Default,
        Fields::F32M42Type: ::std::default::Default,
        Fields::F320p25Type: ::std::default::Default,
        Fields::F321p5e2Type: ::std::default::Default,
        Fields::F32InfType: ::std::default::Default,
        Fields::F32MinfType: ::std::default::Default,
        Fields::F32NanType: ::std::default::Default,
        Fields::F32MnanType: ::std::default::Default,
        Fields::BoolDefaultType: ::std::default::Default,
        Fields::BoolTrueType: ::std::default::Default,
        Fields::BoolFalseType: ::std::default::Default,
        Fields::StringDefaultType: ::std::default::Default,
        Fields::StringEmptyType: ::std::default::Default,
        Fields::StringAbcType: ::std::default::Default,
        Fields::StringAiuType: ::std::default::Default,
        Fields::StringBackslashType: ::std::default::Default,
        Fields::StringTabType: ::std::default::Default,
        Fields::StringCrlfType: ::std::default::Default,
        Fields::BytesDefaultType: ::std::default::Default,
        Fields::BytesEmptyType: ::std::default::Default,
        Fields::BytesAbcType: ::std::default::Default,
        Fields::BytesAiuType: ::std::default::Default,
        Fields::BytesBackslashType: ::std::default::Default,
        Fields::BytesTabType: ::std::default::Default,
        Fields::BytesCrlfType: ::std::default::Default,
        Fields::EnumDefaultType: ::std::default::Default,
        Fields::EnumOneType: ::std::default::Default,
        Fields::EnumFourtyTwoType: ::std::default::Default,
    {
        fn new_in(alloc: AllocatorType) -> Self {
            Self {
                _shared: ::std::convert::Into::into(alloc),
                _phantom: ::std::default::Default::default(),
                i32_default: ::std::default::Default::default(),
                i32_0: ::std::default::Default::default(),
                i32_42: ::std::default::Default::default(),
                i32_m42: ::std::default::Default::default(),
                i32_2147483647: ::std::default::Default::default(),
                i32_m2147483648: ::std::default::Default::default(),
                i32_0123: ::std::default::Default::default(),
                i32_0x123: ::std::default::Default::default(),
                u32_default: ::std::default::Default::default(),
                u32_0: ::std::default::Default::default(),
                u32_42: ::std::default::Default::default(),
                u32_4294967295: ::std::default::Default::default(),
                u32_0123: ::std::default::Default::default(),
                u32_0x123: ::std::default::Default::default(),
                i64_default: ::std::default::Default::default(),
                i64_0: ::std::default::Default::default(),
                i64_42: ::std::default::Default::default(),
                i64_m42: ::std::default::Default::default(),
                i64_9223372036854775807: ::std::default::Default::default(),
                i64_m9223372036854775808: ::std::default::Default::default(),
                i64_0123: ::std::default::Default::default(),
                i64_0x123: ::std::default::Default::default(),
                u64_default: ::std::default::Default::default(),
                u64_0: ::std::default::Default::default(),
                u64_42: ::std::default::Default::default(),
                u64_18446744073709551615: ::std::default::Default::default(),
                u64_0123: ::std::default::Default::default(),
                u64_0x123: ::std::default::Default::default(),
                f32_default: ::std::default::Default::default(),
                f32_0: ::std::default::Default::default(),
                f32_m0: ::std::default::Default::default(),
                f32_0p: ::std::default::Default::default(),
                f32_p0: ::std::default::Default::default(),
                f32_0p0: ::std::default::Default::default(),
                f32_42: ::std::default::Default::default(),
                f32_m42: ::std::default::Default::default(),
                f32_0p25: ::std::default::Default::default(),
                f32_1p5e2: ::std::default::Default::default(),
                f32_inf: ::std::default::Default::default(),
                f32_minf: ::std::default::Default::default(),
                f32_nan: ::std::default::Default::default(),
                f32_mnan: ::std::default::Default::default(),
                bool_default: ::std::default::Default::default(),
                bool_true: ::std::default::Default::default(),
                bool_false: ::std::default::Default::default(),
                string_default: ::std::default::Default::default(),
                string_empty: ::std::default::Default::default(),
                string_abc: ::std::default::Default::default(),
                string_aiu: ::std::default::Default::default(),
                string_backslash: ::std::default::Default::default(),
                string_tab: ::std::default::Default::default(),
                string_crlf: ::std::default::Default::default(),
                bytes_default: ::std::default::Default::default(),
                bytes_empty: ::std::default::Default::default(),
                bytes_abc: ::std::default::Default::default(),
                bytes_aiu: ::std::default::Default::default(),
                bytes_backslash: ::std::default::Default::default(),
                bytes_tab: ::std::default::Default::default(),
                bytes_crlf: ::std::default::Default::default(),
                enum_default: ::std::default::Default::default(),
                enum_one: ::std::default::Default::default(),
                enum_fourty_two: ::std::default::Default::default(),
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
        const BITFIELD_OPTIONAL_FIELD_COUNT: usize = 62;
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
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 1;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Int32;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<3> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 2;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Int32;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 42;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<4> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 3;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Int32;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            -42;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<5> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 4;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Int32;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            2147483647;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<6> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 5;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Int32;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            -2147483648;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<7> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 6;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Int32;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 83;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<8> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 7;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Int32;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            291;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<11> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 8;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::UInt32;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<12> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 9;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::UInt32;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<13> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 10;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::UInt32;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 42;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<15> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 11;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::UInt32;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            4294967295;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<17> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 12;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::UInt32;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 83;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<18> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 13;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::UInt32;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            291;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<21> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 14;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Int64;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<22> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 15;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Int64;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<23> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 16;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Int64;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 42;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<24> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 17;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Int64;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            -42;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<25> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 18;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Int64;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            9223372036854775807;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<26> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 19;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Int64;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            -9223372036854775808;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<27> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 20;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Int64;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 83;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<28> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 21;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Int64;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            291;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<31> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 22;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::UInt64;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<32> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 23;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::UInt64;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 0;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<33> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 24;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::UInt64;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 42;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<35> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 25;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::UInt64;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            18446744073709551615;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<37> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 26;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::UInt64;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = 83;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<38> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 27;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::UInt64;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            291;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<41> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 28;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Float;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            0.0f32;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<42> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 29;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Float;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            0f32;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<43> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 30;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Float;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            -0f32;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<44> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 31;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Float;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            0f32;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<45> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 32;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Float;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            0f32;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<46> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 33;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Float;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            0f32;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<47> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 34;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Float;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            42f32;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<48> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 35;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Float;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            -42f32;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<49> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 36;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Float;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            0.25f32;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<50> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 37;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Float;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            150f32;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<51> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 38;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Float;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            f32::INFINITY;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<52> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 39;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Float;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            f32::NEG_INFINITY;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<53> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 40;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Float;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            f32::NAN;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<54> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 41;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Float;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            f32::NAN;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<61> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 42;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Bool;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            false;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<62> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 43;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Bool;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            true;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<63> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 44;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Bool;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            false;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<71> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 45;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::String;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = "";
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<72> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 46;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::String;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType = "";
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<73> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 47;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::String;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            "abc";
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<74> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 48;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::String;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            "\u{3042}\u{3044}\u{3046}";
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<75> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 49;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::String;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            "\\";
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<76> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 50;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::String;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            "\t";
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<77> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 51;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::String;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            "\r\n";
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<81> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 52;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Bytes;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            b"";
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<82> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 53;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Bytes;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            b"";
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<83> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 54;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Bytes;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            b"\x61\x62\x63";
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<84> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 55;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Bytes;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            b"\xe3\x81\x82\xe3\x81\x84\xe3\x81\x86";
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<85> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 56;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Bytes;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            b"\x5c";
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<86> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 57;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Bytes;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            b"\x09";
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<87> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 58;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Bytes;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            b"\x0d\x0a";
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<91> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 59;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Enum2<self::_puroro_root::proto2_defaults::MyEnum>;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            self::_puroro_root::proto2_defaults::MyEnum::One;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<92> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 60;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Enum2<self::_puroro_root::proto2_defaults::MyEnum>;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            self::_puroro_root::proto2_defaults::MyEnum::One;
    }
    impl ::puroro::internal::FieldProperties for MsgFieldProperties<93> {
        type MessageProperties = self::MsgMessageProperties;
        const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 61;
        type LabelTag = ::puroro::tags::Optional;
        type TypeTag = ::puroro::tags::Enum2<self::_puroro_root::proto2_defaults::MyEnum>;
        const DEFAULT_VALUE: <Self::TypeTag as ::puroro::tags::FieldTypeTag>::DefaultValueType =
            self::_puroro_root::proto2_defaults::MyEnum::FourtyTwo;
    }

    pub type MsgSimple2 = MsgTemplate<
        ::puroro::internal::SimpleFields,
        ::puroro::internal::SimpleShared<{ (62 + 31) / 32 }>,
    >;

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I32DefaultType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<1>,
                Fields::ImplTag,
            >,
    {
        pub fn i32_default(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I32DefaultType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<1>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.i32_default, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I32DefaultType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<1>,
                Fields::ImplTag,
            >,
    {
        pub fn i32_default_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I32DefaultType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<1>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.i32_default, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I32DefaultType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<1>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_i32_default(&self) -> bool {
            self.i32_default_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::I32DefaultType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<1>,
                Fields::ImplTag,
            >,
    {
        pub fn i32_default_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::I32DefaultType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<1>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.i32_default, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I320Type, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<2>,
                Fields::ImplTag,
            >,
    {
        pub fn i32_0(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I320Type, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<2>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.i32_0, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I320Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<2>,
                Fields::ImplTag,
            >,
    {
        pub fn i32_0_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I320Type, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<2>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.i32_0, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I320Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<2>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_i32_0(&self) -> bool {
            self.i32_0_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::I320Type, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<2>,
                Fields::ImplTag,
            >,
    {
        pub fn i32_0_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::I320Type, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<2>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.i32_0, &mut self._shared).get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I3242Type, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<3>,
                Fields::ImplTag,
            >,
    {
        pub fn i32_42(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I3242Type, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<3>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.i32_42, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I3242Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<3>,
                Fields::ImplTag,
            >,
    {
        pub fn i32_42_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I3242Type, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<3>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.i32_42, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I3242Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<3>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_i32_42(&self) -> bool {
            self.i32_42_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::I3242Type, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<3>,
                Fields::ImplTag,
            >,
    {
        pub fn i32_42_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::I3242Type, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<3>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.i32_42, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I32M42Type, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<4>,
                Fields::ImplTag,
            >,
    {
        pub fn i32_m42(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I32M42Type, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<4>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.i32_m42, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I32M42Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<4>,
                Fields::ImplTag,
            >,
    {
        pub fn i32_m42_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I32M42Type, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<4>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.i32_m42, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I32M42Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<4>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_i32_m42(&self) -> bool {
            self.i32_m42_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::I32M42Type, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<4>,
                Fields::ImplTag,
            >,
    {
        pub fn i32_m42_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::I32M42Type, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<4>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.i32_m42, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I322147483647Type, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<5>,
                Fields::ImplTag,
            >,
    {
        pub fn i32_2147483647(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I322147483647Type, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<5>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.i32_2147483647, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I322147483647Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<5>,
                Fields::ImplTag,
            >,
    {
        pub fn i32_2147483647_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I322147483647Type, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<5>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.i32_2147483647, &self._shared)
                .get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I322147483647Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<5>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_i32_2147483647(&self) -> bool {
            self.i32_2147483647_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::I322147483647Type, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<5>,
                Fields::ImplTag,
            >,
    {
        pub fn i32_2147483647_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::I322147483647Type, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<5>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.i32_2147483647, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I32M2147483648Type, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<6>,
                Fields::ImplTag,
            >,
    {
        pub fn i32_m2147483648(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I32M2147483648Type, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<6>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.i32_m2147483648, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I32M2147483648Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<6>,
                Fields::ImplTag,
            >,
    {
        pub fn i32_m2147483648_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I32M2147483648Type, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<6>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.i32_m2147483648, &self._shared)
                .get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I32M2147483648Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<6>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_i32_m2147483648(&self) -> bool {
            self.i32_m2147483648_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::I32M2147483648Type, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<6>,
                Fields::ImplTag,
            >,
    {
        pub fn i32_m2147483648_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::I32M2147483648Type, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<6>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.i32_m2147483648, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I320123Type, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<7>,
                Fields::ImplTag,
            >,
    {
        pub fn i32_0123(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I320123Type, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<7>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.i32_0123, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I320123Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<7>,
                Fields::ImplTag,
            >,
    {
        pub fn i32_0123_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I320123Type, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<7>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.i32_0123, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I320123Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<7>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_i32_0123(&self) -> bool {
            self.i32_0123_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::I320123Type, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<7>,
                Fields::ImplTag,
            >,
    {
        pub fn i32_0123_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::I320123Type, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<7>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.i32_0123, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I320x123Type, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<8>,
                Fields::ImplTag,
            >,
    {
        pub fn i32_0x123(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I320x123Type, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<8>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.i32_0x123, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I320x123Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<8>,
                Fields::ImplTag,
            >,
    {
        pub fn i32_0x123_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I320x123Type, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<8>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.i32_0x123, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I320x123Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<8>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_i32_0x123(&self) -> bool {
            self.i32_0x123_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::I320x123Type, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<8>,
                Fields::ImplTag,
            >,
    {
        pub fn i32_0x123_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::I320x123Type, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<8>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.i32_0x123, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U32DefaultType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<11>,
                Fields::ImplTag,
            >,
    {
        pub fn u32_default(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::U32DefaultType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<11>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.u32_default, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U32DefaultType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<11>,
                Fields::ImplTag,
            >,
    {
        pub fn u32_default_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::U32DefaultType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<11>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.u32_default, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U32DefaultType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<11>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_u32_default(&self) -> bool {
            self.u32_default_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::U32DefaultType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<11>,
                Fields::ImplTag,
            >,
    {
        pub fn u32_default_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::U32DefaultType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<11>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.u32_default, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U320Type, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<12>,
                Fields::ImplTag,
            >,
    {
        pub fn u32_0(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::U320Type, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<12>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.u32_0, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U320Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<12>,
                Fields::ImplTag,
            >,
    {
        pub fn u32_0_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::U320Type, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<12>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.u32_0, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U320Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<12>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_u32_0(&self) -> bool {
            self.u32_0_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::U320Type, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<12>,
                Fields::ImplTag,
            >,
    {
        pub fn u32_0_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::U320Type, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<12>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.u32_0, &mut self._shared).get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U3242Type, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<13>,
                Fields::ImplTag,
            >,
    {
        pub fn u32_42(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::U3242Type, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<13>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.u32_42, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U3242Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<13>,
                Fields::ImplTag,
            >,
    {
        pub fn u32_42_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::U3242Type, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<13>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.u32_42, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U3242Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<13>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_u32_42(&self) -> bool {
            self.u32_42_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::U3242Type, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<13>,
                Fields::ImplTag,
            >,
    {
        pub fn u32_42_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::U3242Type, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<13>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.u32_42, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U324294967295Type, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<15>,
                Fields::ImplTag,
            >,
    {
        pub fn u32_4294967295(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::U324294967295Type, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<15>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.u32_4294967295, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U324294967295Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<15>,
                Fields::ImplTag,
            >,
    {
        pub fn u32_4294967295_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::U324294967295Type, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<15>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.u32_4294967295, &self._shared)
                .get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U324294967295Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<15>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_u32_4294967295(&self) -> bool {
            self.u32_4294967295_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::U324294967295Type, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<15>,
                Fields::ImplTag,
            >,
    {
        pub fn u32_4294967295_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::U324294967295Type, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<15>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.u32_4294967295, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U320123Type, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<17>,
                Fields::ImplTag,
            >,
    {
        pub fn u32_0123(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::U320123Type, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<17>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.u32_0123, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U320123Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<17>,
                Fields::ImplTag,
            >,
    {
        pub fn u32_0123_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::U320123Type, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<17>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.u32_0123, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U320123Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<17>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_u32_0123(&self) -> bool {
            self.u32_0123_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::U320123Type, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<17>,
                Fields::ImplTag,
            >,
    {
        pub fn u32_0123_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::U320123Type, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<17>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.u32_0123, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U320x123Type, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<18>,
                Fields::ImplTag,
            >,
    {
        pub fn u32_0x123(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::U320x123Type, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<18>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.u32_0x123, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U320x123Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<18>,
                Fields::ImplTag,
            >,
    {
        pub fn u32_0x123_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::U320x123Type, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<18>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.u32_0x123, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U320x123Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<18>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_u32_0x123(&self) -> bool {
            self.u32_0x123_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::U320x123Type, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<18>,
                Fields::ImplTag,
            >,
    {
        pub fn u32_0x123_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::U320x123Type, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<18>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.u32_0x123, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I64DefaultType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<21>,
                Fields::ImplTag,
            >,
    {
        pub fn i64_default(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I64DefaultType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<21>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.i64_default, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I64DefaultType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<21>,
                Fields::ImplTag,
            >,
    {
        pub fn i64_default_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I64DefaultType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<21>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.i64_default, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I64DefaultType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<21>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_i64_default(&self) -> bool {
            self.i64_default_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::I64DefaultType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<21>,
                Fields::ImplTag,
            >,
    {
        pub fn i64_default_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::I64DefaultType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<21>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.i64_default, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I640Type, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<22>,
                Fields::ImplTag,
            >,
    {
        pub fn i64_0(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I640Type, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<22>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.i64_0, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I640Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<22>,
                Fields::ImplTag,
            >,
    {
        pub fn i64_0_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I640Type, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<22>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.i64_0, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I640Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<22>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_i64_0(&self) -> bool {
            self.i64_0_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::I640Type, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<22>,
                Fields::ImplTag,
            >,
    {
        pub fn i64_0_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::I640Type, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<22>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.i64_0, &mut self._shared).get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I6442Type, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<23>,
                Fields::ImplTag,
            >,
    {
        pub fn i64_42(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I6442Type, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<23>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.i64_42, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I6442Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<23>,
                Fields::ImplTag,
            >,
    {
        pub fn i64_42_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I6442Type, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<23>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.i64_42, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I6442Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<23>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_i64_42(&self) -> bool {
            self.i64_42_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::I6442Type, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<23>,
                Fields::ImplTag,
            >,
    {
        pub fn i64_42_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::I6442Type, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<23>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.i64_42, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I64M42Type, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<24>,
                Fields::ImplTag,
            >,
    {
        pub fn i64_m42(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I64M42Type, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<24>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.i64_m42, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I64M42Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<24>,
                Fields::ImplTag,
            >,
    {
        pub fn i64_m42_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I64M42Type, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<24>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.i64_m42, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I64M42Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<24>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_i64_m42(&self) -> bool {
            self.i64_m42_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::I64M42Type, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<24>,
                Fields::ImplTag,
            >,
    {
        pub fn i64_m42_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::I64M42Type, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<24>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.i64_m42, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I649223372036854775807Type, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<25>,
                Fields::ImplTag,
            >,
    {
        pub fn i64_9223372036854775807(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I649223372036854775807Type, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<25>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.i64_9223372036854775807, &self._shared)
                .get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I649223372036854775807Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<25>,
                Fields::ImplTag,
            >,
    {
        pub fn i64_9223372036854775807_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I649223372036854775807Type, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<25>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.i64_9223372036854775807, &self._shared)
                .get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I649223372036854775807Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<25>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_i64_9223372036854775807(&self) -> bool {
            self.i64_9223372036854775807_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::I649223372036854775807Type, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<25>,
                Fields::ImplTag,
            >,
    {
        pub fn i64_9223372036854775807_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::I649223372036854775807Type, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<25>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(
                &mut self.i64_9223372036854775807,
                &mut self._shared,
            )
            .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I64M9223372036854775808Type, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<26>,
                Fields::ImplTag,
            >,
    {
        pub fn i64_m9223372036854775808(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I64M9223372036854775808Type, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<26>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(
                &self.i64_m9223372036854775808,
                &self._shared,
            )
            .get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I64M9223372036854775808Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<26>,
                Fields::ImplTag,
            >,
    {
        pub fn i64_m9223372036854775808_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I64M9223372036854775808Type, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<26>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(
                &self.i64_m9223372036854775808,
                &self._shared,
            )
            .get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I64M9223372036854775808Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<26>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_i64_m9223372036854775808(&self) -> bool {
            self.i64_m9223372036854775808_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::I64M9223372036854775808Type, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<26>,
                Fields::ImplTag,
            >,
    {
        pub fn i64_m9223372036854775808_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::I64M9223372036854775808Type, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<26>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(
                &mut self.i64_m9223372036854775808,
                &mut self._shared,
            )
            .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I640123Type, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<27>,
                Fields::ImplTag,
            >,
    {
        pub fn i64_0123(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I640123Type, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<27>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.i64_0123, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I640123Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<27>,
                Fields::ImplTag,
            >,
    {
        pub fn i64_0123_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I640123Type, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<27>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.i64_0123, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I640123Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<27>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_i64_0123(&self) -> bool {
            self.i64_0123_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::I640123Type, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<27>,
                Fields::ImplTag,
            >,
    {
        pub fn i64_0123_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::I640123Type, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<27>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.i64_0123, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I640x123Type, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<28>,
                Fields::ImplTag,
            >,
    {
        pub fn i64_0x123(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I640x123Type, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<28>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.i64_0x123, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I640x123Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<28>,
                Fields::ImplTag,
            >,
    {
        pub fn i64_0x123_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::I640x123Type, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<28>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.i64_0x123, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::I640x123Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<28>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_i64_0x123(&self) -> bool {
            self.i64_0x123_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::I640x123Type, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<28>,
                Fields::ImplTag,
            >,
    {
        pub fn i64_0x123_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::I640x123Type, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<28>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.i64_0x123, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U64DefaultType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<31>,
                Fields::ImplTag,
            >,
    {
        pub fn u64_default(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::U64DefaultType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<31>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.u64_default, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U64DefaultType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<31>,
                Fields::ImplTag,
            >,
    {
        pub fn u64_default_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::U64DefaultType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<31>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.u64_default, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U64DefaultType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<31>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_u64_default(&self) -> bool {
            self.u64_default_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::U64DefaultType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<31>,
                Fields::ImplTag,
            >,
    {
        pub fn u64_default_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::U64DefaultType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<31>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.u64_default, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U640Type, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<32>,
                Fields::ImplTag,
            >,
    {
        pub fn u64_0(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::U640Type, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<32>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.u64_0, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U640Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<32>,
                Fields::ImplTag,
            >,
    {
        pub fn u64_0_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::U640Type, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<32>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.u64_0, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U640Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<32>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_u64_0(&self) -> bool {
            self.u64_0_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::U640Type, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<32>,
                Fields::ImplTag,
            >,
    {
        pub fn u64_0_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::U640Type, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<32>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.u64_0, &mut self._shared).get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U6442Type, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<33>,
                Fields::ImplTag,
            >,
    {
        pub fn u64_42(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::U6442Type, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<33>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.u64_42, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U6442Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<33>,
                Fields::ImplTag,
            >,
    {
        pub fn u64_42_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::U6442Type, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<33>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.u64_42, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U6442Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<33>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_u64_42(&self) -> bool {
            self.u64_42_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::U6442Type, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<33>,
                Fields::ImplTag,
            >,
    {
        pub fn u64_42_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::U6442Type, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<33>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.u64_42, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U6418446744073709551615Type, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<35>,
                Fields::ImplTag,
            >,
    {
        pub fn u64_18446744073709551615(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::U6418446744073709551615Type, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<35>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(
                &self.u64_18446744073709551615,
                &self._shared,
            )
            .get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U6418446744073709551615Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<35>,
                Fields::ImplTag,
            >,
    {
        pub fn u64_18446744073709551615_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::U6418446744073709551615Type, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<35>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(
                &self.u64_18446744073709551615,
                &self._shared,
            )
            .get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U6418446744073709551615Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<35>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_u64_18446744073709551615(&self) -> bool {
            self.u64_18446744073709551615_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::U6418446744073709551615Type, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<35>,
                Fields::ImplTag,
            >,
    {
        pub fn u64_18446744073709551615_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::U6418446744073709551615Type, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<35>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(
                &mut self.u64_18446744073709551615,
                &mut self._shared,
            )
            .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U640123Type, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<37>,
                Fields::ImplTag,
            >,
    {
        pub fn u64_0123(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::U640123Type, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<37>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.u64_0123, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U640123Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<37>,
                Fields::ImplTag,
            >,
    {
        pub fn u64_0123_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::U640123Type, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<37>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.u64_0123, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U640123Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<37>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_u64_0123(&self) -> bool {
            self.u64_0123_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::U640123Type, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<37>,
                Fields::ImplTag,
            >,
    {
        pub fn u64_0123_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::U640123Type, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<37>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.u64_0123, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U640x123Type, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<38>,
                Fields::ImplTag,
            >,
    {
        pub fn u64_0x123(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::U640x123Type, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<38>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.u64_0x123, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U640x123Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<38>,
                Fields::ImplTag,
            >,
    {
        pub fn u64_0x123_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::U640x123Type, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<38>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.u64_0x123, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::U640x123Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<38>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_u64_0x123(&self) -> bool {
            self.u64_0x123_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::U640x123Type, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<38>,
                Fields::ImplTag,
            >,
    {
        pub fn u64_0x123_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::U640x123Type, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<38>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.u64_0x123, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F32DefaultType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<41>,
                Fields::ImplTag,
            >,
    {
        pub fn f32_default(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::F32DefaultType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<41>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.f32_default, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F32DefaultType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<41>,
                Fields::ImplTag,
            >,
    {
        pub fn f32_default_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::F32DefaultType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<41>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.f32_default, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F32DefaultType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<41>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_f32_default(&self) -> bool {
            self.f32_default_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::F32DefaultType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<41>,
                Fields::ImplTag,
            >,
    {
        pub fn f32_default_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::F32DefaultType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<41>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.f32_default, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F320Type, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<42>,
                Fields::ImplTag,
            >,
    {
        pub fn f32_0(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::F320Type, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<42>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.f32_0, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F320Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<42>,
                Fields::ImplTag,
            >,
    {
        pub fn f32_0_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::F320Type, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<42>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.f32_0, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F320Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<42>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_f32_0(&self) -> bool {
            self.f32_0_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::F320Type, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<42>,
                Fields::ImplTag,
            >,
    {
        pub fn f32_0_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::F320Type, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<42>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.f32_0, &mut self._shared).get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F32M0Type, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<43>,
                Fields::ImplTag,
            >,
    {
        pub fn f32_m0(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::F32M0Type, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<43>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.f32_m0, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F32M0Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<43>,
                Fields::ImplTag,
            >,
    {
        pub fn f32_m0_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::F32M0Type, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<43>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.f32_m0, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F32M0Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<43>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_f32_m0(&self) -> bool {
            self.f32_m0_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::F32M0Type, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<43>,
                Fields::ImplTag,
            >,
    {
        pub fn f32_m0_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::F32M0Type, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<43>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.f32_m0, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F320pType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<44>,
                Fields::ImplTag,
            >,
    {
        pub fn f32_0p(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::F320pType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<44>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.f32_0p, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F320pType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<44>,
                Fields::ImplTag,
            >,
    {
        pub fn f32_0p_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::F320pType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<44>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.f32_0p, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F320pType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<44>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_f32_0p(&self) -> bool {
            self.f32_0p_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::F320pType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<44>,
                Fields::ImplTag,
            >,
    {
        pub fn f32_0p_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::F320pType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<44>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.f32_0p, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F32P0Type, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<45>,
                Fields::ImplTag,
            >,
    {
        pub fn f32_p0(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::F32P0Type, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<45>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.f32_p0, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F32P0Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<45>,
                Fields::ImplTag,
            >,
    {
        pub fn f32_p0_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::F32P0Type, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<45>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.f32_p0, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F32P0Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<45>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_f32_p0(&self) -> bool {
            self.f32_p0_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::F32P0Type, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<45>,
                Fields::ImplTag,
            >,
    {
        pub fn f32_p0_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::F32P0Type, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<45>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.f32_p0, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F320p0Type, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<46>,
                Fields::ImplTag,
            >,
    {
        pub fn f32_0p0(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::F320p0Type, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<46>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.f32_0p0, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F320p0Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<46>,
                Fields::ImplTag,
            >,
    {
        pub fn f32_0p0_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::F320p0Type, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<46>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.f32_0p0, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F320p0Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<46>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_f32_0p0(&self) -> bool {
            self.f32_0p0_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::F320p0Type, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<46>,
                Fields::ImplTag,
            >,
    {
        pub fn f32_0p0_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::F320p0Type, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<46>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.f32_0p0, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F3242Type, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<47>,
                Fields::ImplTag,
            >,
    {
        pub fn f32_42(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::F3242Type, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<47>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.f32_42, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F3242Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<47>,
                Fields::ImplTag,
            >,
    {
        pub fn f32_42_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::F3242Type, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<47>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.f32_42, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F3242Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<47>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_f32_42(&self) -> bool {
            self.f32_42_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::F3242Type, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<47>,
                Fields::ImplTag,
            >,
    {
        pub fn f32_42_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::F3242Type, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<47>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.f32_42, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F32M42Type, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<48>,
                Fields::ImplTag,
            >,
    {
        pub fn f32_m42(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::F32M42Type, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<48>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.f32_m42, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F32M42Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<48>,
                Fields::ImplTag,
            >,
    {
        pub fn f32_m42_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::F32M42Type, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<48>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.f32_m42, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F32M42Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<48>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_f32_m42(&self) -> bool {
            self.f32_m42_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::F32M42Type, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<48>,
                Fields::ImplTag,
            >,
    {
        pub fn f32_m42_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::F32M42Type, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<48>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.f32_m42, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F320p25Type, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<49>,
                Fields::ImplTag,
            >,
    {
        pub fn f32_0p25(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::F320p25Type, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<49>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.f32_0p25, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F320p25Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<49>,
                Fields::ImplTag,
            >,
    {
        pub fn f32_0p25_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::F320p25Type, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<49>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.f32_0p25, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F320p25Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<49>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_f32_0p25(&self) -> bool {
            self.f32_0p25_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::F320p25Type, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<49>,
                Fields::ImplTag,
            >,
    {
        pub fn f32_0p25_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::F320p25Type, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<49>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.f32_0p25, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F321p5e2Type, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<50>,
                Fields::ImplTag,
            >,
    {
        pub fn f32_1p5e2(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::F321p5e2Type, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<50>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.f32_1p5e2, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F321p5e2Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<50>,
                Fields::ImplTag,
            >,
    {
        pub fn f32_1p5e2_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::F321p5e2Type, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<50>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.f32_1p5e2, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F321p5e2Type, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<50>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_f32_1p5e2(&self) -> bool {
            self.f32_1p5e2_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::F321p5e2Type, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<50>,
                Fields::ImplTag,
            >,
    {
        pub fn f32_1p5e2_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::F321p5e2Type, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<50>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.f32_1p5e2, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F32InfType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<51>,
                Fields::ImplTag,
            >,
    {
        pub fn f32_inf(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::F32InfType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<51>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.f32_inf, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F32InfType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<51>,
                Fields::ImplTag,
            >,
    {
        pub fn f32_inf_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::F32InfType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<51>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.f32_inf, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F32InfType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<51>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_f32_inf(&self) -> bool {
            self.f32_inf_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::F32InfType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<51>,
                Fields::ImplTag,
            >,
    {
        pub fn f32_inf_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::F32InfType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<51>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.f32_inf, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F32MinfType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<52>,
                Fields::ImplTag,
            >,
    {
        pub fn f32_minf(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::F32MinfType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<52>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.f32_minf, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F32MinfType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<52>,
                Fields::ImplTag,
            >,
    {
        pub fn f32_minf_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::F32MinfType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<52>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.f32_minf, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F32MinfType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<52>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_f32_minf(&self) -> bool {
            self.f32_minf_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::F32MinfType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<52>,
                Fields::ImplTag,
            >,
    {
        pub fn f32_minf_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::F32MinfType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<52>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.f32_minf, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F32NanType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<53>,
                Fields::ImplTag,
            >,
    {
        pub fn f32_nan(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::F32NanType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<53>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.f32_nan, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F32NanType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<53>,
                Fields::ImplTag,
            >,
    {
        pub fn f32_nan_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::F32NanType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<53>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.f32_nan, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F32NanType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<53>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_f32_nan(&self) -> bool {
            self.f32_nan_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::F32NanType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<53>,
                Fields::ImplTag,
            >,
    {
        pub fn f32_nan_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::F32NanType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<53>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.f32_nan, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F32MnanType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<54>,
                Fields::ImplTag,
            >,
    {
        pub fn f32_mnan(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::F32MnanType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<54>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.f32_mnan, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F32MnanType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<54>,
                Fields::ImplTag,
            >,
    {
        pub fn f32_mnan_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::F32MnanType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<54>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.f32_mnan, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::F32MnanType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<54>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_f32_mnan(&self) -> bool {
            self.f32_mnan_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::F32MnanType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<54>,
                Fields::ImplTag,
            >,
    {
        pub fn f32_mnan_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::F32MnanType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<54>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.f32_mnan, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::BoolDefaultType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<61>,
                Fields::ImplTag,
            >,
    {
        pub fn bool_default(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::BoolDefaultType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<61>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.bool_default, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::BoolDefaultType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<61>,
                Fields::ImplTag,
            >,
    {
        pub fn bool_default_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::BoolDefaultType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<61>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.bool_default, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::BoolDefaultType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<61>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_bool_default(&self) -> bool {
            self.bool_default_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::BoolDefaultType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<61>,
                Fields::ImplTag,
            >,
    {
        pub fn bool_default_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::BoolDefaultType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<61>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.bool_default, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::BoolTrueType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<62>,
                Fields::ImplTag,
            >,
    {
        pub fn bool_true(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::BoolTrueType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<62>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.bool_true, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::BoolTrueType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<62>,
                Fields::ImplTag,
            >,
    {
        pub fn bool_true_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::BoolTrueType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<62>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.bool_true, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::BoolTrueType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<62>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_bool_true(&self) -> bool {
            self.bool_true_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::BoolTrueType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<62>,
                Fields::ImplTag,
            >,
    {
        pub fn bool_true_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::BoolTrueType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<62>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.bool_true, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::BoolFalseType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<63>,
                Fields::ImplTag,
            >,
    {
        pub fn bool_false(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::BoolFalseType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<63>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.bool_false, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::BoolFalseType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<63>,
                Fields::ImplTag,
            >,
    {
        pub fn bool_false_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::BoolFalseType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<63>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.bool_false, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::BoolFalseType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<63>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_bool_false(&self) -> bool {
            self.bool_false_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::BoolFalseType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<63>,
                Fields::ImplTag,
            >,
    {
        pub fn bool_false_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::BoolFalseType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<63>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.bool_false, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::StringDefaultType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<71>,
                Fields::ImplTag,
            >,
    {
        pub fn string_default(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::StringDefaultType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<71>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.string_default, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::StringDefaultType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<71>,
                Fields::ImplTag,
            >,
    {
        pub fn string_default_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::StringDefaultType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<71>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.string_default, &self._shared)
                .get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::StringDefaultType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<71>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_string_default(&self) -> bool {
            self.string_default_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::StringDefaultType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<71>,
                Fields::ImplTag,
            >,
    {
        pub fn string_default_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::StringDefaultType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<71>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.string_default, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::StringEmptyType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<72>,
                Fields::ImplTag,
            >,
    {
        pub fn string_empty(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::StringEmptyType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<72>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.string_empty, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::StringEmptyType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<72>,
                Fields::ImplTag,
            >,
    {
        pub fn string_empty_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::StringEmptyType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<72>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.string_empty, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::StringEmptyType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<72>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_string_empty(&self) -> bool {
            self.string_empty_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::StringEmptyType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<72>,
                Fields::ImplTag,
            >,
    {
        pub fn string_empty_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::StringEmptyType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<72>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.string_empty, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::StringAbcType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<73>,
                Fields::ImplTag,
            >,
    {
        pub fn string_abc(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::StringAbcType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<73>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.string_abc, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::StringAbcType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<73>,
                Fields::ImplTag,
            >,
    {
        pub fn string_abc_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::StringAbcType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<73>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.string_abc, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::StringAbcType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<73>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_string_abc(&self) -> bool {
            self.string_abc_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::StringAbcType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<73>,
                Fields::ImplTag,
            >,
    {
        pub fn string_abc_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::StringAbcType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<73>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.string_abc, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::StringAiuType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<74>,
                Fields::ImplTag,
            >,
    {
        pub fn string_aiu(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::StringAiuType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<74>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.string_aiu, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::StringAiuType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<74>,
                Fields::ImplTag,
            >,
    {
        pub fn string_aiu_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::StringAiuType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<74>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.string_aiu, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::StringAiuType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<74>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_string_aiu(&self) -> bool {
            self.string_aiu_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::StringAiuType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<74>,
                Fields::ImplTag,
            >,
    {
        pub fn string_aiu_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::StringAiuType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<74>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.string_aiu, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::StringBackslashType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<75>,
                Fields::ImplTag,
            >,
    {
        pub fn string_backslash(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::StringBackslashType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<75>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.string_backslash, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::StringBackslashType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<75>,
                Fields::ImplTag,
            >,
    {
        pub fn string_backslash_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::StringBackslashType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<75>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.string_backslash, &self._shared)
                .get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::StringBackslashType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<75>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_string_backslash(&self) -> bool {
            self.string_backslash_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::StringBackslashType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<75>,
                Fields::ImplTag,
            >,
    {
        pub fn string_backslash_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::StringBackslashType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<75>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(
                &mut self.string_backslash,
                &mut self._shared,
            )
            .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::StringTabType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<76>,
                Fields::ImplTag,
            >,
    {
        pub fn string_tab(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::StringTabType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<76>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.string_tab, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::StringTabType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<76>,
                Fields::ImplTag,
            >,
    {
        pub fn string_tab_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::StringTabType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<76>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.string_tab, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::StringTabType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<76>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_string_tab(&self) -> bool {
            self.string_tab_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::StringTabType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<76>,
                Fields::ImplTag,
            >,
    {
        pub fn string_tab_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::StringTabType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<76>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.string_tab, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::StringCrlfType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<77>,
                Fields::ImplTag,
            >,
    {
        pub fn string_crlf(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::StringCrlfType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<77>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.string_crlf, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::StringCrlfType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<77>,
                Fields::ImplTag,
            >,
    {
        pub fn string_crlf_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::StringCrlfType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<77>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.string_crlf, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::StringCrlfType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<77>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_string_crlf(&self) -> bool {
            self.string_crlf_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::StringCrlfType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<77>,
                Fields::ImplTag,
            >,
    {
        pub fn string_crlf_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::StringCrlfType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<77>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.string_crlf, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::BytesDefaultType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<81>,
                Fields::ImplTag,
            >,
    {
        pub fn bytes_default(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::BytesDefaultType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<81>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.bytes_default, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::BytesDefaultType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<81>,
                Fields::ImplTag,
            >,
    {
        pub fn bytes_default_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::BytesDefaultType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<81>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.bytes_default, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::BytesDefaultType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<81>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_bytes_default(&self) -> bool {
            self.bytes_default_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::BytesDefaultType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<81>,
                Fields::ImplTag,
            >,
    {
        pub fn bytes_default_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::BytesDefaultType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<81>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.bytes_default, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::BytesEmptyType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<82>,
                Fields::ImplTag,
            >,
    {
        pub fn bytes_empty(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::BytesEmptyType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<82>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.bytes_empty, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::BytesEmptyType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<82>,
                Fields::ImplTag,
            >,
    {
        pub fn bytes_empty_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::BytesEmptyType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<82>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.bytes_empty, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::BytesEmptyType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<82>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_bytes_empty(&self) -> bool {
            self.bytes_empty_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::BytesEmptyType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<82>,
                Fields::ImplTag,
            >,
    {
        pub fn bytes_empty_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::BytesEmptyType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<82>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.bytes_empty, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::BytesAbcType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<83>,
                Fields::ImplTag,
            >,
    {
        pub fn bytes_abc(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::BytesAbcType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<83>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.bytes_abc, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::BytesAbcType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<83>,
                Fields::ImplTag,
            >,
    {
        pub fn bytes_abc_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::BytesAbcType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<83>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.bytes_abc, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::BytesAbcType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<83>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_bytes_abc(&self) -> bool {
            self.bytes_abc_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::BytesAbcType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<83>,
                Fields::ImplTag,
            >,
    {
        pub fn bytes_abc_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::BytesAbcType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<83>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.bytes_abc, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::BytesAiuType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<84>,
                Fields::ImplTag,
            >,
    {
        pub fn bytes_aiu(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::BytesAiuType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<84>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.bytes_aiu, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::BytesAiuType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<84>,
                Fields::ImplTag,
            >,
    {
        pub fn bytes_aiu_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::BytesAiuType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<84>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.bytes_aiu, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::BytesAiuType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<84>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_bytes_aiu(&self) -> bool {
            self.bytes_aiu_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::BytesAiuType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<84>,
                Fields::ImplTag,
            >,
    {
        pub fn bytes_aiu_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::BytesAiuType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<84>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.bytes_aiu, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::BytesBackslashType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<85>,
                Fields::ImplTag,
            >,
    {
        pub fn bytes_backslash(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::BytesBackslashType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<85>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.bytes_backslash, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::BytesBackslashType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<85>,
                Fields::ImplTag,
            >,
    {
        pub fn bytes_backslash_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::BytesBackslashType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<85>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.bytes_backslash, &self._shared)
                .get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::BytesBackslashType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<85>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_bytes_backslash(&self) -> bool {
            self.bytes_backslash_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::BytesBackslashType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<85>,
                Fields::ImplTag,
            >,
    {
        pub fn bytes_backslash_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::BytesBackslashType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<85>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.bytes_backslash, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::BytesTabType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<86>,
                Fields::ImplTag,
            >,
    {
        pub fn bytes_tab(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::BytesTabType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<86>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.bytes_tab, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::BytesTabType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<86>,
                Fields::ImplTag,
            >,
    {
        pub fn bytes_tab_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::BytesTabType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<86>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.bytes_tab, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::BytesTabType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<86>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_bytes_tab(&self) -> bool {
            self.bytes_tab_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::BytesTabType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<86>,
                Fields::ImplTag,
            >,
    {
        pub fn bytes_tab_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::BytesTabType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<86>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.bytes_tab, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::BytesCrlfType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<87>,
                Fields::ImplTag,
            >,
    {
        pub fn bytes_crlf(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::BytesCrlfType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<87>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.bytes_crlf, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::BytesCrlfType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<87>,
                Fields::ImplTag,
            >,
    {
        pub fn bytes_crlf_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::BytesCrlfType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<87>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.bytes_crlf, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::BytesCrlfType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<87>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_bytes_crlf(&self) -> bool {
            self.bytes_crlf_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::BytesCrlfType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<87>,
                Fields::ImplTag,
            >,
    {
        pub fn bytes_crlf_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::BytesCrlfType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<87>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.bytes_crlf, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::EnumDefaultType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<91>,
                Fields::ImplTag,
            >,
    {
        pub fn enum_default(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::EnumDefaultType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<91>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.enum_default, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::EnumDefaultType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<91>,
                Fields::ImplTag,
            >,
    {
        pub fn enum_default_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::EnumDefaultType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<91>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.enum_default, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::EnumDefaultType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<91>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_enum_default(&self) -> bool {
            self.enum_default_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::EnumDefaultType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<91>,
                Fields::ImplTag,
            >,
    {
        pub fn enum_default_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::EnumDefaultType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<91>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.enum_default, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::EnumOneType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<92>,
                Fields::ImplTag,
            >,
    {
        pub fn enum_one(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::EnumOneType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<92>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.enum_one, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::EnumOneType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<92>,
                Fields::ImplTag,
            >,
    {
        pub fn enum_one_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::EnumOneType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<92>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.enum_one, &self._shared).get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::EnumOneType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<92>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_enum_one(&self) -> bool {
            self.enum_one_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::EnumOneType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<92>,
                Fields::ImplTag,
            >,
    {
        pub fn enum_one_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::EnumOneType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<92>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.enum_one, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::EnumFourtyTwoType, Shared>:
            ::puroro::internal::methods::GetFieldMethod<
                'a,
                self::MsgFieldProperties<93>,
                Fields::ImplTag,
            >,
    {
        pub fn enum_fourty_two(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::EnumFourtyTwoType, Shared> as
        ::puroro::internal::methods::GetFieldMethod<
            self::MsgFieldProperties<93>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.enum_fourty_two, &self._shared).get()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::EnumFourtyTwoType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<93>,
                Fields::ImplTag,
            >,
    {
        pub fn enum_fourty_two_opt(&self) -> <::puroro::internal::FieldAndSharedRef<Fields::EnumFourtyTwoType, Shared> as
        ::puroro::internal::methods::GetOptFieldMethod<
            self::MsgFieldProperties<93>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetOptFieldMethod as _;
            ::puroro::internal::FieldAndSharedRef::new(&self.enum_fourty_two, &self._shared)
                .get_opt()
        }
    }

    impl<Fields, Shared, OptionInnerType> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedRef<'a, Fields::EnumFourtyTwoType, Shared>:
            ::puroro::internal::methods::GetOptFieldMethod<
                'a,
                self::MsgFieldProperties<93>,
                Fields::ImplTag,
                GetterType = Option<OptionInnerType>,
            >,
    {
        pub fn has_enum_fourty_two(&self) -> bool {
            self.enum_fourty_two_opt().is_some()
        }
    }

    impl<Fields, Shared> MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        for<'a> ::puroro::internal::FieldAndSharedMut<'a, Fields::EnumFourtyTwoType, Shared>:
            ::puroro::internal::methods::GetMutFieldMethod<
                'a,
                self::MsgFieldProperties<93>,
                Fields::ImplTag,
            >,
    {
        pub fn enum_fourty_two_mut(&mut self) -> <::puroro::internal::FieldAndSharedMut<Fields::EnumFourtyTwoType, Shared> as
        ::puroro::internal::methods::GetMutFieldMethod<
            self::MsgFieldProperties<93>,
            Fields::ImplTag,
        >
        >::GetterType{
            use ::puroro::internal::methods::GetMutFieldMethod as _;
            ::puroro::internal::FieldAndSharedMut::new(&mut self.enum_fourty_two, &mut self._shared)
                .get_mut()
        }
    }

    impl<Fields, Shared> ::std::default::Default for MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        Shared: ::std::default::Default,
        Fields::I32DefaultType: ::std::default::Default,
        Fields::I320Type: ::std::default::Default,
        Fields::I3242Type: ::std::default::Default,
        Fields::I32M42Type: ::std::default::Default,
        Fields::I322147483647Type: ::std::default::Default,
        Fields::I32M2147483648Type: ::std::default::Default,
        Fields::I320123Type: ::std::default::Default,
        Fields::I320x123Type: ::std::default::Default,
        Fields::U32DefaultType: ::std::default::Default,
        Fields::U320Type: ::std::default::Default,
        Fields::U3242Type: ::std::default::Default,
        Fields::U324294967295Type: ::std::default::Default,
        Fields::U320123Type: ::std::default::Default,
        Fields::U320x123Type: ::std::default::Default,
        Fields::I64DefaultType: ::std::default::Default,
        Fields::I640Type: ::std::default::Default,
        Fields::I6442Type: ::std::default::Default,
        Fields::I64M42Type: ::std::default::Default,
        Fields::I649223372036854775807Type: ::std::default::Default,
        Fields::I64M9223372036854775808Type: ::std::default::Default,
        Fields::I640123Type: ::std::default::Default,
        Fields::I640x123Type: ::std::default::Default,
        Fields::U64DefaultType: ::std::default::Default,
        Fields::U640Type: ::std::default::Default,
        Fields::U6442Type: ::std::default::Default,
        Fields::U6418446744073709551615Type: ::std::default::Default,
        Fields::U640123Type: ::std::default::Default,
        Fields::U640x123Type: ::std::default::Default,
        Fields::F32DefaultType: ::std::default::Default,
        Fields::F320Type: ::std::default::Default,
        Fields::F32M0Type: ::std::default::Default,
        Fields::F320pType: ::std::default::Default,
        Fields::F32P0Type: ::std::default::Default,
        Fields::F320p0Type: ::std::default::Default,
        Fields::F3242Type: ::std::default::Default,
        Fields::F32M42Type: ::std::default::Default,
        Fields::F320p25Type: ::std::default::Default,
        Fields::F321p5e2Type: ::std::default::Default,
        Fields::F32InfType: ::std::default::Default,
        Fields::F32MinfType: ::std::default::Default,
        Fields::F32NanType: ::std::default::Default,
        Fields::F32MnanType: ::std::default::Default,
        Fields::BoolDefaultType: ::std::default::Default,
        Fields::BoolTrueType: ::std::default::Default,
        Fields::BoolFalseType: ::std::default::Default,
        Fields::StringDefaultType: ::std::default::Default,
        Fields::StringEmptyType: ::std::default::Default,
        Fields::StringAbcType: ::std::default::Default,
        Fields::StringAiuType: ::std::default::Default,
        Fields::StringBackslashType: ::std::default::Default,
        Fields::StringTabType: ::std::default::Default,
        Fields::StringCrlfType: ::std::default::Default,
        Fields::BytesDefaultType: ::std::default::Default,
        Fields::BytesEmptyType: ::std::default::Default,
        Fields::BytesAbcType: ::std::default::Default,
        Fields::BytesAiuType: ::std::default::Default,
        Fields::BytesBackslashType: ::std::default::Default,
        Fields::BytesTabType: ::std::default::Default,
        Fields::BytesCrlfType: ::std::default::Default,
        Fields::EnumDefaultType: ::std::default::Default,
        Fields::EnumOneType: ::std::default::Default,
        Fields::EnumFourtyTwoType: ::std::default::Default,
    {
        fn default() -> Self {
            Self {
                _shared: ::std::default::Default::default(),
                _phantom: ::std::default::Default::default(),
                i32_default: ::std::default::Default::default(),
                i32_0: ::std::default::Default::default(),
                i32_42: ::std::default::Default::default(),
                i32_m42: ::std::default::Default::default(),
                i32_2147483647: ::std::default::Default::default(),
                i32_m2147483648: ::std::default::Default::default(),
                i32_0123: ::std::default::Default::default(),
                i32_0x123: ::std::default::Default::default(),
                u32_default: ::std::default::Default::default(),
                u32_0: ::std::default::Default::default(),
                u32_42: ::std::default::Default::default(),
                u32_4294967295: ::std::default::Default::default(),
                u32_0123: ::std::default::Default::default(),
                u32_0x123: ::std::default::Default::default(),
                i64_default: ::std::default::Default::default(),
                i64_0: ::std::default::Default::default(),
                i64_42: ::std::default::Default::default(),
                i64_m42: ::std::default::Default::default(),
                i64_9223372036854775807: ::std::default::Default::default(),
                i64_m9223372036854775808: ::std::default::Default::default(),
                i64_0123: ::std::default::Default::default(),
                i64_0x123: ::std::default::Default::default(),
                u64_default: ::std::default::Default::default(),
                u64_0: ::std::default::Default::default(),
                u64_42: ::std::default::Default::default(),
                u64_18446744073709551615: ::std::default::Default::default(),
                u64_0123: ::std::default::Default::default(),
                u64_0x123: ::std::default::Default::default(),
                f32_default: ::std::default::Default::default(),
                f32_0: ::std::default::Default::default(),
                f32_m0: ::std::default::Default::default(),
                f32_0p: ::std::default::Default::default(),
                f32_p0: ::std::default::Default::default(),
                f32_0p0: ::std::default::Default::default(),
                f32_42: ::std::default::Default::default(),
                f32_m42: ::std::default::Default::default(),
                f32_0p25: ::std::default::Default::default(),
                f32_1p5e2: ::std::default::Default::default(),
                f32_inf: ::std::default::Default::default(),
                f32_minf: ::std::default::Default::default(),
                f32_nan: ::std::default::Default::default(),
                f32_mnan: ::std::default::Default::default(),
                bool_default: ::std::default::Default::default(),
                bool_true: ::std::default::Default::default(),
                bool_false: ::std::default::Default::default(),
                string_default: ::std::default::Default::default(),
                string_empty: ::std::default::Default::default(),
                string_abc: ::std::default::Default::default(),
                string_aiu: ::std::default::Default::default(),
                string_backslash: ::std::default::Default::default(),
                string_tab: ::std::default::Default::default(),
                string_crlf: ::std::default::Default::default(),
                bytes_default: ::std::default::Default::default(),
                bytes_empty: ::std::default::Default::default(),
                bytes_abc: ::std::default::Default::default(),
                bytes_aiu: ::std::default::Default::default(),
                bytes_backslash: ::std::default::Default::default(),
                bytes_tab: ::std::default::Default::default(),
                bytes_crlf: ::std::default::Default::default(),
                enum_default: ::std::default::Default::default(),
                enum_one: ::std::default::Default::default(),
                enum_fourty_two: ::std::default::Default::default(),
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
        type I32DefaultType;
        type I320Type;
        type I3242Type;
        type I32M42Type;
        type I322147483647Type;
        type I32M2147483648Type;
        type I320123Type;
        type I320x123Type;
        type U32DefaultType;
        type U320Type;
        type U3242Type;
        type U324294967295Type;
        type U320123Type;
        type U320x123Type;
        type I64DefaultType;
        type I640Type;
        type I6442Type;
        type I64M42Type;
        type I649223372036854775807Type;
        type I64M9223372036854775808Type;
        type I640123Type;
        type I640x123Type;
        type U64DefaultType;
        type U640Type;
        type U6442Type;
        type U6418446744073709551615Type;
        type U640123Type;
        type U640x123Type;
        type F32DefaultType;
        type F320Type;
        type F32M0Type;
        type F320pType;
        type F32P0Type;
        type F320p0Type;
        type F3242Type;
        type F32M42Type;
        type F320p25Type;
        type F321p5e2Type;
        type F32InfType;
        type F32MinfType;
        type F32NanType;
        type F32MnanType;
        type BoolDefaultType;
        type BoolTrueType;
        type BoolFalseType;
        type StringDefaultType;
        type StringEmptyType;
        type StringAbcType;
        type StringAiuType;
        type StringBackslashType;
        type StringTabType;
        type StringCrlfType;
        type BytesDefaultType;
        type BytesEmptyType;
        type BytesAbcType;
        type BytesAiuType;
        type BytesBackslashType;
        type BytesTabType;
        type BytesCrlfType;
        type EnumDefaultType;
        type EnumOneType;
        type EnumFourtyTwoType;
    }

    impl MsgTemplateFieldTypes for ::puroro::internal::SimpleFields {
        type ImplTag = ::puroro::tags::SimpleImpl;
        type I32DefaultType = i32;
        type I320Type = i32;
        type I3242Type = i32;
        type I32M42Type = i32;
        type I322147483647Type = i32;
        type I32M2147483648Type = i32;
        type I320123Type = i32;
        type I320x123Type = i32;
        type U32DefaultType = u32;
        type U320Type = u32;
        type U3242Type = u32;
        type U324294967295Type = u32;
        type U320123Type = u32;
        type U320x123Type = u32;
        type I64DefaultType = i64;
        type I640Type = i64;
        type I6442Type = i64;
        type I64M42Type = i64;
        type I649223372036854775807Type = i64;
        type I64M9223372036854775808Type = i64;
        type I640123Type = i64;
        type I640x123Type = i64;
        type U64DefaultType = u64;
        type U640Type = u64;
        type U6442Type = u64;
        type U6418446744073709551615Type = u64;
        type U640123Type = u64;
        type U640x123Type = u64;
        type F32DefaultType = f32;
        type F320Type = f32;
        type F32M0Type = f32;
        type F320pType = f32;
        type F32P0Type = f32;
        type F320p0Type = f32;
        type F3242Type = f32;
        type F32M42Type = f32;
        type F320p25Type = f32;
        type F321p5e2Type = f32;
        type F32InfType = f32;
        type F32MinfType = f32;
        type F32NanType = f32;
        type F32MnanType = f32;
        type BoolDefaultType = bool;
        type BoolTrueType = bool;
        type BoolFalseType = bool;
        type StringDefaultType = ::std::string::String;
        type StringEmptyType = ::std::string::String;
        type StringAbcType = ::std::string::String;
        type StringAiuType = ::std::string::String;
        type StringBackslashType = ::std::string::String;
        type StringTabType = ::std::string::String;
        type StringCrlfType = ::std::string::String;
        type BytesDefaultType = ::std::vec::Vec<u8>;
        type BytesEmptyType = ::std::vec::Vec<u8>;
        type BytesAbcType = ::std::vec::Vec<u8>;
        type BytesAiuType = ::std::vec::Vec<u8>;
        type BytesBackslashType = ::std::vec::Vec<u8>;
        type BytesTabType = ::std::vec::Vec<u8>;
        type BytesCrlfType = ::std::vec::Vec<u8>;
        type EnumDefaultType = self::_puroro_root::proto2_defaults::MyEnum;
        type EnumOneType = self::_puroro_root::proto2_defaults::MyEnum;
        type EnumFourtyTwoType = self::_puroro_root::proto2_defaults::MyEnum;
    }
}
pub use _puroro_traits::*;
pub mod _puroro_traits {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
}
#[derive(::std::fmt::Debug, ::std::clone::Clone, ::std::marker::Copy, ::std::cmp::PartialEq)]
pub enum MyEnum {
    One,
    FourtyTwo,
}

impl ::puroro::Enum2 for MyEnum {}
impl ::std::convert::TryFrom<i32> for MyEnum {
    type Error = i32;
    fn try_from(value: i32) -> ::std::result::Result<Self, i32> {
        ::std::result::Result::Ok(match value {
            1 => MyEnum::One,
            42 => MyEnum::FourtyTwo,
            _ => Err(value)?,
        })
    }
}

impl ::std::convert::From<MyEnum> for i32 {
    fn from(value: MyEnum) -> i32 {
        match value {
            MyEnum::One => 1,
            MyEnum::FourtyTwo => 42,
        }
    }
}

impl ::std::default::Default for MyEnum {
    fn default() -> Self {
        MyEnum::One
    }
}

impl<'bump> ::puroro::internal::BumpDefault<'bump> for MyEnum {
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
    }
}
