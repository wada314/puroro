// A generated source code by puroro library
// package oneofs3

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
    {
        fn new_in(alloc: AllocatorType) -> Self {
            Self {
                _shared: ::std::convert::Into::into(alloc),
                _phantom: ::std::default::Default::default(),
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

    pub type MsgSimple2 = MsgTemplate<
        ::puroro::internal::SimpleFields,
        ::puroro::internal::SimpleShared<{ (5 + 31) / 32 }>,
    >;

    impl<Fields, Shared> ::std::default::Default for MsgTemplate<Fields, Shared>
    where
        Fields: MsgTemplateFieldTypes,
        Shared: ::std::default::Default,
    {
        fn default() -> Self {
            Self {
                _shared: ::std::default::Default::default(),
                _phantom: ::std::default::Default::default(),
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
        type G1Int32Type;
        type G1StringType;
        type G2F32Type;
        type G2StringType;
        type G2SubmsgType;
        type G3Int32Type;
    }

    impl MsgTemplateFieldTypes for ::puroro::internal::SimpleFields {
        type ImplTag = ::puroro::tags::SimpleImpl;
        type G1Int32Type = i32;
        type G1StringType = ::std::string::String;
        type G2F32Type = f32;
        type G2StringType = ::std::string::String;
        type G2SubmsgType =
            ::std::boxed::Box<self::_puroro_root::oneofs3::_puroro_simple_impl::Submsg>;
        type G3Int32Type = i32;
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

        pub use _puroro_oneofs::*;
        pub mod _puroro_oneofs {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }

            pub enum GroupOne<G1Int32, G1String> {
                G1Int32(G1Int32),
                G1String(G1String),
            }
            impl<G1Int32, G1String> GroupOne<G1Int32, G1String> {
                pub fn g1_int32(self) -> ::std::option::Option<G1Int32> {
                    match self {
                        Self::G1Int32(v) => ::std::option::Option::Some(v),
                        #[allow(unreachable_patterns)]
                        _ => None,
                    }
                }
                pub fn g1_string(self) -> ::std::option::Option<G1String> {
                    match self {
                        Self::G1String(v) => ::std::option::Option::Some(v),
                        #[allow(unreachable_patterns)]
                        _ => None,
                    }
                }
            }

            impl<G1Int32, G1String> ::std::fmt::Debug for GroupOne<G1Int32, G1String>
            where
                G1Int32: ::std::fmt::Debug,
                G1String: ::std::fmt::Debug,
            {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    match self {
                        Self::G1Int32(v) => f.debug_tuple("GroupOne::G1Int32").field(&v).finish(),
                        Self::G1String(v) => f.debug_tuple("GroupOne::G1String").field(&v).finish(),
                    }
                }
            }

            impl<G1Int32, G1String> ::std::clone::Clone for GroupOne<G1Int32, G1String>
            where
                G1Int32: ::std::clone::Clone,
                G1String: ::std::clone::Clone,
            {
                fn clone(&self) -> Self {
                    match self {
                        Self::G1Int32(ref v) => {
                            Self::G1Int32(<G1Int32 as ::std::clone::Clone>::clone(v))
                        }
                        Self::G1String(ref v) => {
                            Self::G1String(<G1String as ::std::clone::Clone>::clone(v))
                        }
                    }
                }
            }

            impl<G1Int32, G1String> ::std::cmp::PartialEq for GroupOne<G1Int32, G1String>
            where
                G1Int32: ::std::cmp::PartialEq,
                G1String: ::std::cmp::PartialEq,
            {
                fn eq(&self, rhs: &Self) -> bool {
                    match (self, rhs) {
                        (Self::G1Int32(left), Self::G1Int32(right)) => left == right,
                        (Self::G1String(left), Self::G1String(right)) => left == right,
                        #[allow(unreachable_patterns)]
                        _ => false,
                    }
                }
            }

            pub enum GroupTwo<G2F32, G2String, G2Submsg> {
                G2F32(G2F32),
                G2String(G2String),
                G2Submsg(G2Submsg),
            }
            impl<G2F32, G2String, G2Submsg> GroupTwo<G2F32, G2String, G2Submsg> {
                pub fn g2_f32(self) -> ::std::option::Option<G2F32> {
                    match self {
                        Self::G2F32(v) => ::std::option::Option::Some(v),
                        #[allow(unreachable_patterns)]
                        _ => None,
                    }
                }
                pub fn g2_string(self) -> ::std::option::Option<G2String> {
                    match self {
                        Self::G2String(v) => ::std::option::Option::Some(v),
                        #[allow(unreachable_patterns)]
                        _ => None,
                    }
                }
                pub fn g2_submsg(self) -> ::std::option::Option<G2Submsg> {
                    match self {
                        Self::G2Submsg(v) => ::std::option::Option::Some(v),
                        #[allow(unreachable_patterns)]
                        _ => None,
                    }
                }
            }

            impl<G2F32, G2String, G2Submsg> ::std::fmt::Debug for GroupTwo<G2F32, G2String, G2Submsg>
            where
                G2F32: ::std::fmt::Debug,
                G2String: ::std::fmt::Debug,
                G2Submsg: ::std::fmt::Debug,
            {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    match self {
                        Self::G2F32(v) => f.debug_tuple("GroupTwo::G2F32").field(&v).finish(),
                        Self::G2String(v) => f.debug_tuple("GroupTwo::G2String").field(&v).finish(),
                        Self::G2Submsg(v) => f.debug_tuple("GroupTwo::G2Submsg").field(&v).finish(),
                    }
                }
            }

            impl<G2F32, G2String, G2Submsg> ::std::clone::Clone for GroupTwo<G2F32, G2String, G2Submsg>
            where
                G2F32: ::std::clone::Clone,
                G2String: ::std::clone::Clone,
                G2Submsg: ::std::clone::Clone,
            {
                fn clone(&self) -> Self {
                    match self {
                        Self::G2F32(ref v) => Self::G2F32(<G2F32 as ::std::clone::Clone>::clone(v)),
                        Self::G2String(ref v) => {
                            Self::G2String(<G2String as ::std::clone::Clone>::clone(v))
                        }
                        Self::G2Submsg(ref v) => {
                            Self::G2Submsg(<G2Submsg as ::std::clone::Clone>::clone(v))
                        }
                    }
                }
            }

            impl<G2F32, G2String, G2Submsg> ::std::cmp::PartialEq for GroupTwo<G2F32, G2String, G2Submsg>
            where
                G2F32: ::std::cmp::PartialEq,
                G2String: ::std::cmp::PartialEq,
                G2Submsg: ::std::cmp::PartialEq,
            {
                fn eq(&self, rhs: &Self) -> bool {
                    match (self, rhs) {
                        (Self::G2F32(left), Self::G2F32(right)) => left == right,
                        (Self::G2String(left), Self::G2String(right)) => left == right,
                        (Self::G2Submsg(left), Self::G2Submsg(right)) => left == right,
                        #[allow(unreachable_patterns)]
                        _ => false,
                    }
                }
            }

            pub enum GroupThree<G3Int32> {
                G3Int32(G3Int32),
            }
            impl<G3Int32> GroupThree<G3Int32> {
                pub fn g3_int32(self) -> ::std::option::Option<G3Int32> {
                    match self {
                        Self::G3Int32(v) => ::std::option::Option::Some(v),
                        #[allow(unreachable_patterns)]
                        _ => None,
                    }
                }
            }

            impl<G3Int32> ::std::fmt::Debug for GroupThree<G3Int32>
            where
                G3Int32: ::std::fmt::Debug,
            {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    match self {
                        Self::G3Int32(v) => f.debug_tuple("GroupThree::G3Int32").field(&v).finish(),
                    }
                }
            }

            impl<G3Int32> ::std::clone::Clone for GroupThree<G3Int32>
            where
                G3Int32: ::std::clone::Clone,
            {
                fn clone(&self) -> Self {
                    match self {
                        Self::G3Int32(ref v) => {
                            Self::G3Int32(<G3Int32 as ::std::clone::Clone>::clone(v))
                        }
                    }
                }
            }

            impl<G3Int32> ::std::cmp::PartialEq for GroupThree<G3Int32>
            where
                G3Int32: ::std::cmp::PartialEq,
            {
                fn eq(&self, rhs: &Self) -> bool {
                    match (self, rhs) {
                        (Self::G3Int32(left), Self::G3Int32(right)) => left == right,
                        #[allow(unreachable_patterns)]
                        _ => false,
                    }
                }
            }
        }
    }
    pub mod submsg {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }
    }
}
