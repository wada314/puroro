// A generated source code by puroro library
// package full_coverage2

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub use _puroro_simple_impl::Msg;
pub mod _puroro_simple_impl {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct Msg {
    pub i32_required: ::std::option::Option<i32>,
    pub i32_optional: ::std::option::Option<i32>,
    pub i32_repeated: ::std::vec::Vec<i32>,
    pub float_required: ::std::option::Option<f32>,
    pub float_optional: ::std::option::Option<f32>,
    pub float_repeated: ::std::vec::Vec<f32>,
    pub bytes_required: ::std::option::Option<::std::vec::Vec<u8>>,
    pub bytes_optional: ::std::option::Option<::std::vec::Vec<u8>>,
    pub bytes_repeated: ::std::vec::Vec<::std::vec::Vec<u8>>,
    pub string_required: ::std::option::Option<::std::string::String>,
    pub string_optional: ::std::option::Option<::std::string::String>,
    pub string_repeated: ::std::vec::Vec<::std::string::String>,
    pub enum_required: ::std::option::Option<self::_puroro_root::full_coverage2::Enum>,
    pub enum_optional: ::std::option::Option<self::_puroro_root::full_coverage2::Enum>,
    pub enum_repeated: ::std::vec::Vec<self::_puroro_root::full_coverage2::Enum>,
    pub submsg_required: ::std::option::Option<::std::boxed::Box<self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_simple_impl::Submsg>>,
    pub submsg_optional: ::std::option::Option<::std::boxed::Box<self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_simple_impl::Submsg>>,
    pub submsg_repeated: ::std::vec::Vec<self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_simple_impl::Submsg>,
    pub i64_required: ::std::option::Option<i64>,
    pub i64_optional: ::std::option::Option<i64>,
    pub i64_repeated: ::std::vec::Vec<i64>,
    pub u32_required: ::std::option::Option<u32>,
    pub u32_optional: ::std::option::Option<u32>,
    pub u32_repeated: ::std::vec::Vec<u32>,
    pub u64_required: ::std::option::Option<u64>,
    pub u64_optional: ::std::option::Option<u64>,
    pub u64_repeated: ::std::vec::Vec<u64>,
    pub s32_required: ::std::option::Option<i32>,
    pub s32_optional: ::std::option::Option<i32>,
    pub s32_repeated: ::std::vec::Vec<i32>,
    pub s64_required: ::std::option::Option<i64>,
    pub s64_optional: ::std::option::Option<i64>,
    pub s64_repeated: ::std::vec::Vec<i64>,
    pub fixed32_required: ::std::option::Option<u32>,
    pub fixed32_optional: ::std::option::Option<u32>,
    pub fixed32_repeated: ::std::vec::Vec<u32>,
    pub fixed64_required: ::std::option::Option<u64>,
    pub fixed64_optional: ::std::option::Option<u64>,
    pub fixed64_repeated: ::std::vec::Vec<u64>,
    pub sfixed32_required: ::std::option::Option<i32>,
    pub sfixed32_optional: ::std::option::Option<i32>,
    pub sfixed32_repeated: ::std::vec::Vec<i32>,
    pub sfixed64_required: ::std::option::Option<i64>,
    pub sfixed64_optional: ::std::option::Option<i64>,
    pub sfixed64_repeated: ::std::vec::Vec<i64>,
    pub f64_required: ::std::option::Option<f64>,
    pub f64_optional: ::std::option::Option<f64>,
    pub f64_repeated: ::std::vec::Vec<f64>,
}
    impl ::puroro::Message<Msg> for Msg {}

    impl super::_puroro_traits::MsgTrait for Msg {
        fn i32_required<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.i32_required)
        }
        fn i32_optional<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.i32_optional)
        }
        type Field3RepeatedType<'this> = ::std::iter::Cloned<::std::slice::Iter<'this, i32>>;

        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            self.i32_repeated.iter().cloned()
        }
        fn float_required<'this>(&'this self) -> Option<f32> {
            Clone::clone(&self.float_required)
        }
        fn float_optional<'this>(&'this self) -> Option<f32> {
            Clone::clone(&self.float_optional)
        }
        type Field13RepeatedType<'this> = ::std::iter::Cloned<::std::slice::Iter<'this, f32>>;

        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            self.float_repeated.iter().cloned()
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = &'this [u8];
        fn bytes_required<'this>(&'this self) -> Option<Self::Field21BytesType<'this>> {
            self.bytes_required.as_ref().map(|v| v.as_ref())
        }
        type Field22BytesType<'this>
        where
            Self: 'this,
        = &'this [u8];
        fn bytes_optional<'this>(&'this self) -> Option<Self::Field22BytesType<'this>> {
            self.bytes_optional.as_ref().map(|v| v.as_ref())
        }
        type Field23BytesType<'this>
        where
            Self: 'this,
        = &'this [u8];
        type Field23RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            [u8],
            ::std::slice::Iter<'this, ::std::vec::Vec<u8>>,
        >;

        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.bytes_repeated.iter())
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = &'this str;
        fn string_required<'this>(&'this self) -> Option<Self::Field31StringType<'this>> {
            self.string_required.as_ref().map(|v| v.as_ref())
        }
        type Field32StringType<'this>
        where
            Self: 'this,
        = &'this str;
        fn string_optional<'this>(&'this self) -> Option<Self::Field32StringType<'this>> {
            self.string_optional.as_ref().map(|v| v.as_ref())
        }
        type Field33StringType<'this>
        where
            Self: 'this,
        = &'this str;
        type Field33RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            str,
            ::std::slice::Iter<'this, ::std::string::String>,
        >;

        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.string_repeated.iter())
        }
        fn enum_required<'this>(&'this self) -> Option<self::_puroro_root::full_coverage2::Enum> {
            Clone::clone(&self.enum_required)
        }
        fn enum_optional<'this>(&'this self) -> Option<self::_puroro_root::full_coverage2::Enum> {
            Clone::clone(&self.enum_optional)
        }
        type Field43RepeatedType<'this> = ::std::iter::Cloned<
            ::std::slice::Iter<'this, self::_puroro_root::full_coverage2::Enum>,
        >;

        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            self.enum_repeated.iter().cloned()
        }
        type Field51MessageType<'this> where Self: 'this = &'this self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_simple_impl::Submsg;
        fn submsg_required<'this>(&'this self) -> Option<Self::Field51MessageType<'this>> {
            self.submsg_required.as_ref().map(|v| v.as_ref())
        }
        type Field52MessageType<'this> where Self: 'this = &'this self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_simple_impl::Submsg;
        fn submsg_optional<'this>(&'this self) -> Option<Self::Field52MessageType<'this>> {
            self.submsg_optional.as_ref().map(|v| v.as_ref())
        }
        type Field53MessageType<'this> where Self: 'this = &'this self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_simple_impl::Submsg;
        type Field53RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
    self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_simple_impl::Submsg,
    ::std::slice::Iter<'this, self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_simple_impl::Submsg>>;

        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.submsg_repeated.iter())
        }
        fn i64_required<'this>(&'this self) -> Option<i64> {
            Clone::clone(&self.i64_required)
        }
        fn i64_optional<'this>(&'this self) -> Option<i64> {
            Clone::clone(&self.i64_optional)
        }
        type Field103RepeatedType<'this> = ::std::iter::Cloned<::std::slice::Iter<'this, i64>>;

        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            self.i64_repeated.iter().cloned()
        }
        fn u32_required<'this>(&'this self) -> Option<u32> {
            Clone::clone(&self.u32_required)
        }
        fn u32_optional<'this>(&'this self) -> Option<u32> {
            Clone::clone(&self.u32_optional)
        }
        type Field113RepeatedType<'this> = ::std::iter::Cloned<::std::slice::Iter<'this, u32>>;

        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            self.u32_repeated.iter().cloned()
        }
        fn u64_required<'this>(&'this self) -> Option<u64> {
            Clone::clone(&self.u64_required)
        }
        fn u64_optional<'this>(&'this self) -> Option<u64> {
            Clone::clone(&self.u64_optional)
        }
        type Field123RepeatedType<'this> = ::std::iter::Cloned<::std::slice::Iter<'this, u64>>;

        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            self.u64_repeated.iter().cloned()
        }
        fn s32_required<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.s32_required)
        }
        fn s32_optional<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.s32_optional)
        }
        type Field133RepeatedType<'this> = ::std::iter::Cloned<::std::slice::Iter<'this, i32>>;

        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            self.s32_repeated.iter().cloned()
        }
        fn s64_required<'this>(&'this self) -> Option<i64> {
            Clone::clone(&self.s64_required)
        }
        fn s64_optional<'this>(&'this self) -> Option<i64> {
            Clone::clone(&self.s64_optional)
        }
        type Field143RepeatedType<'this> = ::std::iter::Cloned<::std::slice::Iter<'this, i64>>;

        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            self.s64_repeated.iter().cloned()
        }
        fn fixed32_required<'this>(&'this self) -> Option<u32> {
            Clone::clone(&self.fixed32_required)
        }
        fn fixed32_optional<'this>(&'this self) -> Option<u32> {
            Clone::clone(&self.fixed32_optional)
        }
        type Field153RepeatedType<'this> = ::std::iter::Cloned<::std::slice::Iter<'this, u32>>;

        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            self.fixed32_repeated.iter().cloned()
        }
        fn fixed64_required<'this>(&'this self) -> Option<u64> {
            Clone::clone(&self.fixed64_required)
        }
        fn fixed64_optional<'this>(&'this self) -> Option<u64> {
            Clone::clone(&self.fixed64_optional)
        }
        type Field163RepeatedType<'this> = ::std::iter::Cloned<::std::slice::Iter<'this, u64>>;

        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            self.fixed64_repeated.iter().cloned()
        }
        fn sfixed32_required<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.sfixed32_required)
        }
        fn sfixed32_optional<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.sfixed32_optional)
        }
        type Field173RepeatedType<'this> = ::std::iter::Cloned<::std::slice::Iter<'this, i32>>;

        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            self.sfixed32_repeated.iter().cloned()
        }
        fn sfixed64_required<'this>(&'this self) -> Option<i64> {
            Clone::clone(&self.sfixed64_required)
        }
        fn sfixed64_optional<'this>(&'this self) -> Option<i64> {
            Clone::clone(&self.sfixed64_optional)
        }
        type Field183RepeatedType<'this> = ::std::iter::Cloned<::std::slice::Iter<'this, i64>>;

        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            self.sfixed64_repeated.iter().cloned()
        }
        fn f64_required<'this>(&'this self) -> Option<f64> {
            Clone::clone(&self.f64_required)
        }
        fn f64_optional<'this>(&'this self) -> Option<f64> {
            Clone::clone(&self.f64_optional)
        }
        type Field193RepeatedType<'this> = ::std::iter::Cloned<::std::slice::Iter<'this, f64>>;

        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            self.f64_repeated.iter().cloned()
        }
    }

    impl ::puroro::MessageRepresentativeImpl for Msg {
        fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
            use ::puroro::once_cell::sync::Lazy;
            static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 48]> =
                Lazy::new(|| {
                    [
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "i32_required",
                                number: 1,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "i32_optional",
                                number: 2,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "i32_repeated",
                                number: 3,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "float_required",
                                number: 11,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "float_optional",
                                number: 12,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "float_repeated",
                                number: 13,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "bytes_required",
                                number: 21,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "bytes_optional",
                                number: 22,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "bytes_repeated",
                                number: 23,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "string_required",
                                number: 31,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "string_optional",
                                number: 32,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "string_repeated",
                                number: 33,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "enum_required",
                                number: 41,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "enum_optional",
                                number: 42,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "enum_repeated",
                                number: 43,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "submsg_required",
                                number: 51,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "submsg_optional",
                                number: 52,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "submsg_repeated",
                                number: 53,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "i64_required",
                                number: 101,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "i64_optional",
                                number: 102,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "i64_repeated",
                                number: 103,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "u32_required",
                                number: 111,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "u32_optional",
                                number: 112,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "u32_repeated",
                                number: 113,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "u64_required",
                                number: 121,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "u64_optional",
                                number: 122,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "u64_repeated",
                                number: 123,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "s32_required",
                                number: 131,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "s32_optional",
                                number: 132,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "s32_repeated",
                                number: 133,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "s64_required",
                                number: 141,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "s64_optional",
                                number: 142,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "s64_repeated",
                                number: 143,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "fixed32_required",
                                number: 151,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "fixed32_optional",
                                number: 152,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "fixed32_repeated",
                                number: 153,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "fixed64_required",
                                number: 161,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "fixed64_optional",
                                number: 162,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "fixed64_repeated",
                                number: 163,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "sfixed32_required",
                                number: 171,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "sfixed32_optional",
                                number: 172,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "sfixed32_repeated",
                                number: 173,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "sfixed64_required",
                                number: 181,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "sfixed64_optional",
                                number: 182,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "sfixed64_repeated",
                                number: 183,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "f64_required",
                                number: 191,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "f64_optional",
                                number: 192,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "f64_repeated",
                                number: 193,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                    ]
                });
            static LAZY_DESCRIPTOR: Lazy<::puroro::desc::MessageDescriptor> = Lazy::new(|| {
                let init = ::puroro::internal::MessageDescriptorInitializer {
                    name: "Msg",
                    lazy_fields: Lazy::new(|| Lazy::force(&LAZY_FIELD_DESCRIPTOR_ARRAY).as_ref()),
                };
                ::puroro::internal::init_message_descriptor(init)
            });
            Lazy::force(&LAZY_DESCRIPTOR)
        }
    }

    impl ::puroro::internal::DeserializableMessageFromBytesIterator for Msg {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro::internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro::internal::de::DeserFieldsFromBytesIter for Msg {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro::internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro::internal::impls::simple::de::DeserFieldFromBytesIter;
            match field_number {
            1 => DeserFieldFromBytesIter::<
                ::puroro::tags::Required, ::puroro::tags::Int32
            >::deser_field(&mut self.i32_required, data),
            2 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Int32
            >::deser_field(&mut self.i32_optional, data),
            3 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Int32
            >::deser_field(&mut self.i32_repeated, data),
            11 => DeserFieldFromBytesIter::<
                ::puroro::tags::Required, ::puroro::tags::Float
            >::deser_field(&mut self.float_required, data),
            12 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Float
            >::deser_field(&mut self.float_optional, data),
            13 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Float
            >::deser_field(&mut self.float_repeated, data),
            21 => DeserFieldFromBytesIter::<
                ::puroro::tags::Required, ::puroro::tags::Bytes
            >::deser_field(&mut self.bytes_required, data),
            22 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bytes
            >::deser_field(&mut self.bytes_optional, data),
            23 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Bytes
            >::deser_field(&mut self.bytes_repeated, data),
            31 => DeserFieldFromBytesIter::<
                ::puroro::tags::Required, ::puroro::tags::String
            >::deser_field(&mut self.string_required, data),
            32 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.string_optional, data),
            33 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::String
            >::deser_field(&mut self.string_repeated, data),
            41 => DeserFieldFromBytesIter::<
                ::puroro::tags::Required, ::puroro::tags::Enum2<self::_puroro_root::full_coverage2::Enum>
            >::deser_field(&mut self.enum_required, data),
            42 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Enum2<self::_puroro_root::full_coverage2::Enum>
            >::deser_field(&mut self.enum_optional, data),
            43 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Enum2<self::_puroro_root::full_coverage2::Enum>
            >::deser_field(&mut self.enum_repeated, data),
            51 => DeserFieldFromBytesIter::<
                ::puroro::tags::Required, ::puroro::tags::Message<::std::boxed::Box<self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_simple_impl::Submsg>>
            >::deser_field(&mut self.submsg_required, data),
            52 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Message<::std::boxed::Box<self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_simple_impl::Submsg>>
            >::deser_field(&mut self.submsg_optional, data),
            53 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_simple_impl::Submsg>
            >::deser_field(&mut self.submsg_repeated, data),
            101 => DeserFieldFromBytesIter::<
                ::puroro::tags::Required, ::puroro::tags::Int64
            >::deser_field(&mut self.i64_required, data),
            102 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Int64
            >::deser_field(&mut self.i64_optional, data),
            103 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Int64
            >::deser_field(&mut self.i64_repeated, data),
            111 => DeserFieldFromBytesIter::<
                ::puroro::tags::Required, ::puroro::tags::UInt32
            >::deser_field(&mut self.u32_required, data),
            112 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::UInt32
            >::deser_field(&mut self.u32_optional, data),
            113 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::UInt32
            >::deser_field(&mut self.u32_repeated, data),
            121 => DeserFieldFromBytesIter::<
                ::puroro::tags::Required, ::puroro::tags::UInt64
            >::deser_field(&mut self.u64_required, data),
            122 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::UInt64
            >::deser_field(&mut self.u64_optional, data),
            123 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::UInt64
            >::deser_field(&mut self.u64_repeated, data),
            131 => DeserFieldFromBytesIter::<
                ::puroro::tags::Required, ::puroro::tags::SInt32
            >::deser_field(&mut self.s32_required, data),
            132 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::SInt32
            >::deser_field(&mut self.s32_optional, data),
            133 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::SInt32
            >::deser_field(&mut self.s32_repeated, data),
            141 => DeserFieldFromBytesIter::<
                ::puroro::tags::Required, ::puroro::tags::SInt64
            >::deser_field(&mut self.s64_required, data),
            142 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::SInt64
            >::deser_field(&mut self.s64_optional, data),
            143 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::SInt64
            >::deser_field(&mut self.s64_repeated, data),
            151 => DeserFieldFromBytesIter::<
                ::puroro::tags::Required, ::puroro::tags::Fixed32
            >::deser_field(&mut self.fixed32_required, data),
            152 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Fixed32
            >::deser_field(&mut self.fixed32_optional, data),
            153 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Fixed32
            >::deser_field(&mut self.fixed32_repeated, data),
            161 => DeserFieldFromBytesIter::<
                ::puroro::tags::Required, ::puroro::tags::Fixed64
            >::deser_field(&mut self.fixed64_required, data),
            162 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Fixed64
            >::deser_field(&mut self.fixed64_optional, data),
            163 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Fixed64
            >::deser_field(&mut self.fixed64_repeated, data),
            171 => DeserFieldFromBytesIter::<
                ::puroro::tags::Required, ::puroro::tags::SFixed32
            >::deser_field(&mut self.sfixed32_required, data),
            172 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::SFixed32
            >::deser_field(&mut self.sfixed32_optional, data),
            173 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::SFixed32
            >::deser_field(&mut self.sfixed32_repeated, data),
            181 => DeserFieldFromBytesIter::<
                ::puroro::tags::Required, ::puroro::tags::SFixed64
            >::deser_field(&mut self.sfixed64_required, data),
            182 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::SFixed64
            >::deser_field(&mut self.sfixed64_optional, data),
            183 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::SFixed64
            >::deser_field(&mut self.sfixed64_repeated, data),
            191 => DeserFieldFromBytesIter::<
                ::puroro::tags::Required, ::puroro::tags::Double
            >::deser_field(&mut self.f64_required, data),
            192 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Double
            >::deser_field(&mut self.f64_optional, data),
            193 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Double
            >::deser_field(&mut self.f64_repeated, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for Msg {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Required, ::puroro::tags::Int32>::ser_field(
                &self.i32_required,
                1,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Int32>::ser_field(
                &self.i32_optional,
                2,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::Int32>::ser_field(
                &self.i32_repeated,
                3,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Required, ::puroro::tags::Float>::ser_field(
                &self.float_required,
                11,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Float>::ser_field(
                &self.float_optional,
                12,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::Float>::ser_field(
                &self.float_repeated,
                13,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Required, ::puroro::tags::Bytes>::ser_field(
                &self.bytes_required,
                21,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bytes>::ser_field(
                &self.bytes_optional,
                22,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::Bytes>::ser_field(
                &self.bytes_repeated,
                23,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Required, ::puroro::tags::String>::ser_field(
                &self.string_required,
                31,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.string_optional,
                32,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::String>::ser_field(
                &self.string_repeated,
                33,
                out,
            )?;
            SerFieldToIoWrite::<
                ::puroro::tags::Required,
                ::puroro::tags::Enum2<self::_puroro_root::full_coverage2::Enum>,
            >::ser_field(&self.enum_required, 41, out)?;
            SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Enum2<self::_puroro_root::full_coverage2::Enum>,
            >::ser_field(&self.enum_optional, 42, out)?;
            SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Enum2<self::_puroro_root::full_coverage2::Enum>,
            >::ser_field(&self.enum_repeated, 43, out)?;
            SerFieldToIoWrite::<
            ::puroro::tags::Required, ::puroro::tags::Message<::std::boxed::Box<self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_simple_impl::Submsg>>
        >::ser_field(&self.submsg_required, 51, out)?;
            SerFieldToIoWrite::<
            ::puroro::tags::Optional, ::puroro::tags::Message<::std::boxed::Box<self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_simple_impl::Submsg>>
        >::ser_field(&self.submsg_optional, 52, out)?;
            SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_simple_impl::Submsg>
        >::ser_field(&self.submsg_repeated, 53, out)?;
            SerFieldToIoWrite::<::puroro::tags::Required, ::puroro::tags::Int64>::ser_field(
                &self.i64_required,
                101,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Int64>::ser_field(
                &self.i64_optional,
                102,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::Int64>::ser_field(
                &self.i64_repeated,
                103,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Required, ::puroro::tags::UInt32>::ser_field(
                &self.u32_required,
                111,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::UInt32>::ser_field(
                &self.u32_optional,
                112,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::UInt32>::ser_field(
                &self.u32_repeated,
                113,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Required, ::puroro::tags::UInt64>::ser_field(
                &self.u64_required,
                121,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::UInt64>::ser_field(
                &self.u64_optional,
                122,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::UInt64>::ser_field(
                &self.u64_repeated,
                123,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Required, ::puroro::tags::SInt32>::ser_field(
                &self.s32_required,
                131,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::SInt32>::ser_field(
                &self.s32_optional,
                132,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::SInt32>::ser_field(
                &self.s32_repeated,
                133,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Required, ::puroro::tags::SInt64>::ser_field(
                &self.s64_required,
                141,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::SInt64>::ser_field(
                &self.s64_optional,
                142,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::SInt64>::ser_field(
                &self.s64_repeated,
                143,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Required, ::puroro::tags::Fixed32>::ser_field(
                &self.fixed32_required,
                151,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Fixed32>::ser_field(
                &self.fixed32_optional,
                152,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::Fixed32>::ser_field(
                &self.fixed32_repeated,
                153,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Required, ::puroro::tags::Fixed64>::ser_field(
                &self.fixed64_required,
                161,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Fixed64>::ser_field(
                &self.fixed64_optional,
                162,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::Fixed64>::ser_field(
                &self.fixed64_repeated,
                163,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Required, ::puroro::tags::SFixed32>::ser_field(
                &self.sfixed32_required,
                171,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::SFixed32>::ser_field(
                &self.sfixed32_optional,
                172,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::SFixed32>::ser_field(
                &self.sfixed32_repeated,
                173,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Required, ::puroro::tags::SFixed64>::ser_field(
                &self.sfixed64_required,
                181,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::SFixed64>::ser_field(
                &self.sfixed64_optional,
                182,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::SFixed64>::ser_field(
                &self.sfixed64_repeated,
                183,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Required, ::puroro::tags::Double>::ser_field(
                &self.f64_required,
                191,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Double>::ser_field(
                &self.f64_optional,
                192,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::Double>::ser_field(
                &self.f64_repeated,
                193,
                out,
            )?;

            ::std::result::Result::Ok(())
        }
    }
}

pub use _puroro_impls::*;
pub mod _puroro_impls {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
    use super::_puroro_traits::*;
    impl MsgTrait for () {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field22BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field23BytesType<'this>>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field32StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field33StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }
    impl<T, U> MsgTrait for (T, U)
    where
        T: MsgTrait,
        U: MsgTrait,
    {
        fn i32_required<'this>(&'this self) -> Option<i32> {
            <U as MsgTrait>::i32_required(&self.1)
                .or_else(|| <T as MsgTrait>::i32_required(&self.0))
        }
        fn i32_optional<'this>(&'this self) -> Option<i32> {
            <U as MsgTrait>::i32_optional(&self.1)
                .or_else(|| <T as MsgTrait>::i32_optional(&self.0))
        }
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::Field3RepeatedType<'this>,
            <U as MsgTrait>::Field3RepeatedType<'this>,
        >;

        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as MsgTrait>::i32_repeated(&self.0),
                <U as MsgTrait>::i32_repeated(&self.1),
            )
        }
        fn float_required<'this>(&'this self) -> Option<f32> {
            <U as MsgTrait>::float_required(&self.1)
                .or_else(|| <T as MsgTrait>::float_required(&self.0))
        }
        fn float_optional<'this>(&'this self) -> Option<f32> {
            <U as MsgTrait>::float_optional(&self.1)
                .or_else(|| <T as MsgTrait>::float_optional(&self.0))
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::Field13RepeatedType<'this>,
            <U as MsgTrait>::Field13RepeatedType<'this>,
        >;

        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as MsgTrait>::float_repeated(&self.0),
                <U as MsgTrait>::float_repeated(&self.1),
            )
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as MsgTrait>::Field21BytesType<'this>,
            <U as MsgTrait>::Field21BytesType<'this>,
        >;
        fn bytes_required<'this>(&'this self) -> Option<Self::Field21BytesType<'this>> {
            if let Some(right) = <U as MsgTrait>::bytes_required(&self.1) {
                Some(::puroro::Either::Right(right))
            } else if let Some(left) = <T as MsgTrait>::bytes_required(&self.0) {
                Some(::puroro::Either::Left(left))
            } else {
                None
            }
        }
        type Field22BytesType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as MsgTrait>::Field22BytesType<'this>,
            <U as MsgTrait>::Field22BytesType<'this>,
        >;
        fn bytes_optional<'this>(&'this self) -> Option<Self::Field22BytesType<'this>> {
            if let Some(right) = <U as MsgTrait>::bytes_optional(&self.1) {
                Some(::puroro::Either::Right(right))
            } else if let Some(left) = <T as MsgTrait>::bytes_optional(&self.0) {
                Some(::puroro::Either::Left(left))
            } else {
                None
            }
        }
        type Field23BytesType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as MsgTrait>::Field23BytesType<'this>,
            <U as MsgTrait>::Field23BytesType<'this>,
        >;
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedLDField<
            <T as MsgTrait>::Field23RepeatedType<'this>,
            <U as MsgTrait>::Field23RepeatedType<'this>,
        >;

        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedLDField::new(
                <T as MsgTrait>::bytes_repeated(&self.0),
                <U as MsgTrait>::bytes_repeated(&self.1),
            )
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as MsgTrait>::Field31StringType<'this>,
            <U as MsgTrait>::Field31StringType<'this>,
        >;
        fn string_required<'this>(&'this self) -> Option<Self::Field31StringType<'this>> {
            if let Some(right) = <U as MsgTrait>::string_required(&self.1) {
                Some(::puroro::Either::Right(right))
            } else if let Some(left) = <T as MsgTrait>::string_required(&self.0) {
                Some(::puroro::Either::Left(left))
            } else {
                None
            }
        }
        type Field32StringType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as MsgTrait>::Field32StringType<'this>,
            <U as MsgTrait>::Field32StringType<'this>,
        >;
        fn string_optional<'this>(&'this self) -> Option<Self::Field32StringType<'this>> {
            if let Some(right) = <U as MsgTrait>::string_optional(&self.1) {
                Some(::puroro::Either::Right(right))
            } else if let Some(left) = <T as MsgTrait>::string_optional(&self.0) {
                Some(::puroro::Either::Left(left))
            } else {
                None
            }
        }
        type Field33StringType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as MsgTrait>::Field33StringType<'this>,
            <U as MsgTrait>::Field33StringType<'this>,
        >;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedLDField<
            <T as MsgTrait>::Field33RepeatedType<'this>,
            <U as MsgTrait>::Field33RepeatedType<'this>,
        >;

        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedLDField::new(
                <T as MsgTrait>::string_repeated(&self.0),
                <U as MsgTrait>::string_repeated(&self.1),
            )
        }
        fn enum_required<'this>(&'this self) -> Option<self::_puroro_root::full_coverage2::Enum> {
            <U as MsgTrait>::enum_required(&self.1)
                .or_else(|| <T as MsgTrait>::enum_required(&self.0))
        }
        fn enum_optional<'this>(&'this self) -> Option<self::_puroro_root::full_coverage2::Enum> {
            <U as MsgTrait>::enum_optional(&self.1)
                .or_else(|| <T as MsgTrait>::enum_optional(&self.0))
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::Field43RepeatedType<'this>,
            <U as MsgTrait>::Field43RepeatedType<'this>,
        >;

        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as MsgTrait>::enum_repeated(&self.0),
                <U as MsgTrait>::enum_repeated(&self.1),
            )
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = (
            ::std::option::Option<<T as MsgTrait>::Field51MessageType<'this>>,
            ::std::option::Option<<U as MsgTrait>::Field51MessageType<'this>>,
        );
        fn submsg_required<'this>(&'this self) -> Option<Self::Field51MessageType<'this>> {
            match (
                <T as MsgTrait>::submsg_required(&self.0),
                <U as MsgTrait>::submsg_required(&self.1),
            ) {
                (None, None) => None,
                (Some(t), None) => Some((Some(t), None)),
                (None, Some(u)) => Some((None, Some(u))),
                (Some(t), Some(u)) => Some((Some(t), Some(u))),
            }
        }
        type Field52MessageType<'this>
        where
            Self: 'this,
        = (
            ::std::option::Option<<T as MsgTrait>::Field52MessageType<'this>>,
            ::std::option::Option<<U as MsgTrait>::Field52MessageType<'this>>,
        );
        fn submsg_optional<'this>(&'this self) -> Option<Self::Field52MessageType<'this>> {
            match (
                <T as MsgTrait>::submsg_optional(&self.0),
                <U as MsgTrait>::submsg_optional(&self.1),
            ) {
                (None, None) => None,
                (Some(t), None) => Some((Some(t), None)),
                (None, Some(u)) => Some((None, Some(u))),
                (Some(t), Some(u)) => Some((Some(t), Some(u))),
            }
        }
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as MsgTrait>::Field53MessageType<'this>,
            <U as MsgTrait>::Field53MessageType<'this>,
        >;
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedMessageField<
            <T as MsgTrait>::Field53RepeatedType<'this>,
            <U as MsgTrait>::Field53RepeatedType<'this>,
        >;

        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
                <T as MsgTrait>::submsg_repeated(&self.0),
                <U as MsgTrait>::submsg_repeated(&self.1),
            )
        }
        fn i64_required<'this>(&'this self) -> Option<i64> {
            <U as MsgTrait>::i64_required(&self.1)
                .or_else(|| <T as MsgTrait>::i64_required(&self.0))
        }
        fn i64_optional<'this>(&'this self) -> Option<i64> {
            <U as MsgTrait>::i64_optional(&self.1)
                .or_else(|| <T as MsgTrait>::i64_optional(&self.0))
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::Field103RepeatedType<'this>,
            <U as MsgTrait>::Field103RepeatedType<'this>,
        >;

        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as MsgTrait>::i64_repeated(&self.0),
                <U as MsgTrait>::i64_repeated(&self.1),
            )
        }
        fn u32_required<'this>(&'this self) -> Option<u32> {
            <U as MsgTrait>::u32_required(&self.1)
                .or_else(|| <T as MsgTrait>::u32_required(&self.0))
        }
        fn u32_optional<'this>(&'this self) -> Option<u32> {
            <U as MsgTrait>::u32_optional(&self.1)
                .or_else(|| <T as MsgTrait>::u32_optional(&self.0))
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::Field113RepeatedType<'this>,
            <U as MsgTrait>::Field113RepeatedType<'this>,
        >;

        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as MsgTrait>::u32_repeated(&self.0),
                <U as MsgTrait>::u32_repeated(&self.1),
            )
        }
        fn u64_required<'this>(&'this self) -> Option<u64> {
            <U as MsgTrait>::u64_required(&self.1)
                .or_else(|| <T as MsgTrait>::u64_required(&self.0))
        }
        fn u64_optional<'this>(&'this self) -> Option<u64> {
            <U as MsgTrait>::u64_optional(&self.1)
                .or_else(|| <T as MsgTrait>::u64_optional(&self.0))
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::Field123RepeatedType<'this>,
            <U as MsgTrait>::Field123RepeatedType<'this>,
        >;

        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as MsgTrait>::u64_repeated(&self.0),
                <U as MsgTrait>::u64_repeated(&self.1),
            )
        }
        fn s32_required<'this>(&'this self) -> Option<i32> {
            <U as MsgTrait>::s32_required(&self.1)
                .or_else(|| <T as MsgTrait>::s32_required(&self.0))
        }
        fn s32_optional<'this>(&'this self) -> Option<i32> {
            <U as MsgTrait>::s32_optional(&self.1)
                .or_else(|| <T as MsgTrait>::s32_optional(&self.0))
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::Field133RepeatedType<'this>,
            <U as MsgTrait>::Field133RepeatedType<'this>,
        >;

        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as MsgTrait>::s32_repeated(&self.0),
                <U as MsgTrait>::s32_repeated(&self.1),
            )
        }
        fn s64_required<'this>(&'this self) -> Option<i64> {
            <U as MsgTrait>::s64_required(&self.1)
                .or_else(|| <T as MsgTrait>::s64_required(&self.0))
        }
        fn s64_optional<'this>(&'this self) -> Option<i64> {
            <U as MsgTrait>::s64_optional(&self.1)
                .or_else(|| <T as MsgTrait>::s64_optional(&self.0))
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::Field143RepeatedType<'this>,
            <U as MsgTrait>::Field143RepeatedType<'this>,
        >;

        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as MsgTrait>::s64_repeated(&self.0),
                <U as MsgTrait>::s64_repeated(&self.1),
            )
        }
        fn fixed32_required<'this>(&'this self) -> Option<u32> {
            <U as MsgTrait>::fixed32_required(&self.1)
                .or_else(|| <T as MsgTrait>::fixed32_required(&self.0))
        }
        fn fixed32_optional<'this>(&'this self) -> Option<u32> {
            <U as MsgTrait>::fixed32_optional(&self.1)
                .or_else(|| <T as MsgTrait>::fixed32_optional(&self.0))
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::Field153RepeatedType<'this>,
            <U as MsgTrait>::Field153RepeatedType<'this>,
        >;

        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as MsgTrait>::fixed32_repeated(&self.0),
                <U as MsgTrait>::fixed32_repeated(&self.1),
            )
        }
        fn fixed64_required<'this>(&'this self) -> Option<u64> {
            <U as MsgTrait>::fixed64_required(&self.1)
                .or_else(|| <T as MsgTrait>::fixed64_required(&self.0))
        }
        fn fixed64_optional<'this>(&'this self) -> Option<u64> {
            <U as MsgTrait>::fixed64_optional(&self.1)
                .or_else(|| <T as MsgTrait>::fixed64_optional(&self.0))
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::Field163RepeatedType<'this>,
            <U as MsgTrait>::Field163RepeatedType<'this>,
        >;

        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as MsgTrait>::fixed64_repeated(&self.0),
                <U as MsgTrait>::fixed64_repeated(&self.1),
            )
        }
        fn sfixed32_required<'this>(&'this self) -> Option<i32> {
            <U as MsgTrait>::sfixed32_required(&self.1)
                .or_else(|| <T as MsgTrait>::sfixed32_required(&self.0))
        }
        fn sfixed32_optional<'this>(&'this self) -> Option<i32> {
            <U as MsgTrait>::sfixed32_optional(&self.1)
                .or_else(|| <T as MsgTrait>::sfixed32_optional(&self.0))
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::Field173RepeatedType<'this>,
            <U as MsgTrait>::Field173RepeatedType<'this>,
        >;

        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as MsgTrait>::sfixed32_repeated(&self.0),
                <U as MsgTrait>::sfixed32_repeated(&self.1),
            )
        }
        fn sfixed64_required<'this>(&'this self) -> Option<i64> {
            <U as MsgTrait>::sfixed64_required(&self.1)
                .or_else(|| <T as MsgTrait>::sfixed64_required(&self.0))
        }
        fn sfixed64_optional<'this>(&'this self) -> Option<i64> {
            <U as MsgTrait>::sfixed64_optional(&self.1)
                .or_else(|| <T as MsgTrait>::sfixed64_optional(&self.0))
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::Field183RepeatedType<'this>,
            <U as MsgTrait>::Field183RepeatedType<'this>,
        >;

        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as MsgTrait>::sfixed64_repeated(&self.0),
                <U as MsgTrait>::sfixed64_repeated(&self.1),
            )
        }
        fn f64_required<'this>(&'this self) -> Option<f64> {
            <U as MsgTrait>::f64_required(&self.1)
                .or_else(|| <T as MsgTrait>::f64_required(&self.0))
        }
        fn f64_optional<'this>(&'this self) -> Option<f64> {
            <U as MsgTrait>::f64_optional(&self.1)
                .or_else(|| <T as MsgTrait>::f64_optional(&self.0))
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::Field193RepeatedType<'this>,
            <U as MsgTrait>::Field193RepeatedType<'this>,
        >;

        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as MsgTrait>::f64_repeated(&self.0),
                <U as MsgTrait>::f64_repeated(&self.1),
            )
        }
    }
    impl<T, U> MsgTrait for ::puroro::Either<T, U>
    where
        T: MsgTrait,
        U: MsgTrait,
    {
        fn i32_required<'this>(&'this self) -> Option<i32> {
            self.as_ref().either(
                |t| <T as MsgTrait>::i32_required(t),
                |u| <U as MsgTrait>::i32_required(u),
            )
        }
        fn i32_optional<'this>(&'this self) -> Option<i32> {
            self.as_ref().either(
                |t| <T as MsgTrait>::i32_optional(t),
                |u| <U as MsgTrait>::i32_optional(u),
            )
        }
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as MsgTrait>::Field3RepeatedType<'this>,
            <U as MsgTrait>::Field3RepeatedType<'this>,
        >;

        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::i32_repeated(t))
                    .map_right(|u| <U as MsgTrait>::i32_repeated(u)),
            )
        }
        fn float_required<'this>(&'this self) -> Option<f32> {
            self.as_ref().either(
                |t| <T as MsgTrait>::float_required(t),
                |u| <U as MsgTrait>::float_required(u),
            )
        }
        fn float_optional<'this>(&'this self) -> Option<f32> {
            self.as_ref().either(
                |t| <T as MsgTrait>::float_optional(t),
                |u| <U as MsgTrait>::float_optional(u),
            )
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as MsgTrait>::Field13RepeatedType<'this>,
            <U as MsgTrait>::Field13RepeatedType<'this>,
        >;

        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::float_repeated(t))
                    .map_right(|u| <U as MsgTrait>::float_repeated(u)),
            )
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as MsgTrait>::Field21BytesType<'this>,
            <U as MsgTrait>::Field21BytesType<'this>,
        >;
        fn bytes_required<'this>(&'this self) -> Option<Self::Field21BytesType<'this>> {
            self.as_ref().either(
                |t| <T as MsgTrait>::bytes_required(t).map(|t| ::puroro::Either::Left(t)),
                |u| <U as MsgTrait>::bytes_required(u).map(|u| ::puroro::Either::Right(u)),
            )
        }
        type Field22BytesType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as MsgTrait>::Field22BytesType<'this>,
            <U as MsgTrait>::Field22BytesType<'this>,
        >;
        fn bytes_optional<'this>(&'this self) -> Option<Self::Field22BytesType<'this>> {
            self.as_ref().either(
                |t| <T as MsgTrait>::bytes_optional(t).map(|t| ::puroro::Either::Left(t)),
                |u| <U as MsgTrait>::bytes_optional(u).map(|u| ::puroro::Either::Right(u)),
            )
        }
        type Field23BytesType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as MsgTrait>::Field23BytesType<'this>,
            <U as MsgTrait>::Field23BytesType<'this>,
        >;
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedLDField<
            <T as MsgTrait>::Field23RepeatedType<'this>,
            <U as MsgTrait>::Field23RepeatedType<'this>,
        >;

        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedLDField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::bytes_repeated(t))
                    .map_right(|u| <U as MsgTrait>::bytes_repeated(u)),
            )
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as MsgTrait>::Field31StringType<'this>,
            <U as MsgTrait>::Field31StringType<'this>,
        >;
        fn string_required<'this>(&'this self) -> Option<Self::Field31StringType<'this>> {
            self.as_ref().either(
                |t| <T as MsgTrait>::string_required(t).map(|t| ::puroro::Either::Left(t)),
                |u| <U as MsgTrait>::string_required(u).map(|u| ::puroro::Either::Right(u)),
            )
        }
        type Field32StringType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as MsgTrait>::Field32StringType<'this>,
            <U as MsgTrait>::Field32StringType<'this>,
        >;
        fn string_optional<'this>(&'this self) -> Option<Self::Field32StringType<'this>> {
            self.as_ref().either(
                |t| <T as MsgTrait>::string_optional(t).map(|t| ::puroro::Either::Left(t)),
                |u| <U as MsgTrait>::string_optional(u).map(|u| ::puroro::Either::Right(u)),
            )
        }
        type Field33StringType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as MsgTrait>::Field33StringType<'this>,
            <U as MsgTrait>::Field33StringType<'this>,
        >;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedLDField<
            <T as MsgTrait>::Field33RepeatedType<'this>,
            <U as MsgTrait>::Field33RepeatedType<'this>,
        >;

        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedLDField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::string_repeated(t))
                    .map_right(|u| <U as MsgTrait>::string_repeated(u)),
            )
        }
        fn enum_required<'this>(&'this self) -> Option<self::_puroro_root::full_coverage2::Enum> {
            self.as_ref().either(
                |t| <T as MsgTrait>::enum_required(t),
                |u| <U as MsgTrait>::enum_required(u),
            )
        }
        fn enum_optional<'this>(&'this self) -> Option<self::_puroro_root::full_coverage2::Enum> {
            self.as_ref().either(
                |t| <T as MsgTrait>::enum_optional(t),
                |u| <U as MsgTrait>::enum_optional(u),
            )
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as MsgTrait>::Field43RepeatedType<'this>,
            <U as MsgTrait>::Field43RepeatedType<'this>,
        >;

        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::enum_repeated(t))
                    .map_right(|u| <U as MsgTrait>::enum_repeated(u)),
            )
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as MsgTrait>::Field51MessageType<'this>,
            <U as MsgTrait>::Field51MessageType<'this>,
        >;
        fn submsg_required<'this>(&'this self) -> Option<Self::Field51MessageType<'this>> {
            self.as_ref().either(
                |t| <T as MsgTrait>::submsg_required(t).map(|t| ::puroro::Either::Left(t)),
                |u| <U as MsgTrait>::submsg_required(u).map(|u| ::puroro::Either::Right(u)),
            )
        }
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as MsgTrait>::Field52MessageType<'this>,
            <U as MsgTrait>::Field52MessageType<'this>,
        >;
        fn submsg_optional<'this>(&'this self) -> Option<Self::Field52MessageType<'this>> {
            self.as_ref().either(
                |t| <T as MsgTrait>::submsg_optional(t).map(|t| ::puroro::Either::Left(t)),
                |u| <U as MsgTrait>::submsg_optional(u).map(|u| ::puroro::Either::Right(u)),
            )
        }
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as MsgTrait>::Field53MessageType<'this>,
            <U as MsgTrait>::Field53MessageType<'this>,
        >;
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedMessageField<
            <T as MsgTrait>::Field53RepeatedType<'this>,
            <U as MsgTrait>::Field53RepeatedType<'this>,
        >;

        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::submsg_repeated(t))
                    .map_right(|u| <U as MsgTrait>::submsg_repeated(u)),
            )
        }
        fn i64_required<'this>(&'this self) -> Option<i64> {
            self.as_ref().either(
                |t| <T as MsgTrait>::i64_required(t),
                |u| <U as MsgTrait>::i64_required(u),
            )
        }
        fn i64_optional<'this>(&'this self) -> Option<i64> {
            self.as_ref().either(
                |t| <T as MsgTrait>::i64_optional(t),
                |u| <U as MsgTrait>::i64_optional(u),
            )
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as MsgTrait>::Field103RepeatedType<'this>,
            <U as MsgTrait>::Field103RepeatedType<'this>,
        >;

        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::i64_repeated(t))
                    .map_right(|u| <U as MsgTrait>::i64_repeated(u)),
            )
        }
        fn u32_required<'this>(&'this self) -> Option<u32> {
            self.as_ref().either(
                |t| <T as MsgTrait>::u32_required(t),
                |u| <U as MsgTrait>::u32_required(u),
            )
        }
        fn u32_optional<'this>(&'this self) -> Option<u32> {
            self.as_ref().either(
                |t| <T as MsgTrait>::u32_optional(t),
                |u| <U as MsgTrait>::u32_optional(u),
            )
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as MsgTrait>::Field113RepeatedType<'this>,
            <U as MsgTrait>::Field113RepeatedType<'this>,
        >;

        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::u32_repeated(t))
                    .map_right(|u| <U as MsgTrait>::u32_repeated(u)),
            )
        }
        fn u64_required<'this>(&'this self) -> Option<u64> {
            self.as_ref().either(
                |t| <T as MsgTrait>::u64_required(t),
                |u| <U as MsgTrait>::u64_required(u),
            )
        }
        fn u64_optional<'this>(&'this self) -> Option<u64> {
            self.as_ref().either(
                |t| <T as MsgTrait>::u64_optional(t),
                |u| <U as MsgTrait>::u64_optional(u),
            )
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as MsgTrait>::Field123RepeatedType<'this>,
            <U as MsgTrait>::Field123RepeatedType<'this>,
        >;

        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::u64_repeated(t))
                    .map_right(|u| <U as MsgTrait>::u64_repeated(u)),
            )
        }
        fn s32_required<'this>(&'this self) -> Option<i32> {
            self.as_ref().either(
                |t| <T as MsgTrait>::s32_required(t),
                |u| <U as MsgTrait>::s32_required(u),
            )
        }
        fn s32_optional<'this>(&'this self) -> Option<i32> {
            self.as_ref().either(
                |t| <T as MsgTrait>::s32_optional(t),
                |u| <U as MsgTrait>::s32_optional(u),
            )
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as MsgTrait>::Field133RepeatedType<'this>,
            <U as MsgTrait>::Field133RepeatedType<'this>,
        >;

        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::s32_repeated(t))
                    .map_right(|u| <U as MsgTrait>::s32_repeated(u)),
            )
        }
        fn s64_required<'this>(&'this self) -> Option<i64> {
            self.as_ref().either(
                |t| <T as MsgTrait>::s64_required(t),
                |u| <U as MsgTrait>::s64_required(u),
            )
        }
        fn s64_optional<'this>(&'this self) -> Option<i64> {
            self.as_ref().either(
                |t| <T as MsgTrait>::s64_optional(t),
                |u| <U as MsgTrait>::s64_optional(u),
            )
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as MsgTrait>::Field143RepeatedType<'this>,
            <U as MsgTrait>::Field143RepeatedType<'this>,
        >;

        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::s64_repeated(t))
                    .map_right(|u| <U as MsgTrait>::s64_repeated(u)),
            )
        }
        fn fixed32_required<'this>(&'this self) -> Option<u32> {
            self.as_ref().either(
                |t| <T as MsgTrait>::fixed32_required(t),
                |u| <U as MsgTrait>::fixed32_required(u),
            )
        }
        fn fixed32_optional<'this>(&'this self) -> Option<u32> {
            self.as_ref().either(
                |t| <T as MsgTrait>::fixed32_optional(t),
                |u| <U as MsgTrait>::fixed32_optional(u),
            )
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as MsgTrait>::Field153RepeatedType<'this>,
            <U as MsgTrait>::Field153RepeatedType<'this>,
        >;

        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::fixed32_repeated(t))
                    .map_right(|u| <U as MsgTrait>::fixed32_repeated(u)),
            )
        }
        fn fixed64_required<'this>(&'this self) -> Option<u64> {
            self.as_ref().either(
                |t| <T as MsgTrait>::fixed64_required(t),
                |u| <U as MsgTrait>::fixed64_required(u),
            )
        }
        fn fixed64_optional<'this>(&'this self) -> Option<u64> {
            self.as_ref().either(
                |t| <T as MsgTrait>::fixed64_optional(t),
                |u| <U as MsgTrait>::fixed64_optional(u),
            )
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as MsgTrait>::Field163RepeatedType<'this>,
            <U as MsgTrait>::Field163RepeatedType<'this>,
        >;

        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::fixed64_repeated(t))
                    .map_right(|u| <U as MsgTrait>::fixed64_repeated(u)),
            )
        }
        fn sfixed32_required<'this>(&'this self) -> Option<i32> {
            self.as_ref().either(
                |t| <T as MsgTrait>::sfixed32_required(t),
                |u| <U as MsgTrait>::sfixed32_required(u),
            )
        }
        fn sfixed32_optional<'this>(&'this self) -> Option<i32> {
            self.as_ref().either(
                |t| <T as MsgTrait>::sfixed32_optional(t),
                |u| <U as MsgTrait>::sfixed32_optional(u),
            )
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as MsgTrait>::Field173RepeatedType<'this>,
            <U as MsgTrait>::Field173RepeatedType<'this>,
        >;

        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::sfixed32_repeated(t))
                    .map_right(|u| <U as MsgTrait>::sfixed32_repeated(u)),
            )
        }
        fn sfixed64_required<'this>(&'this self) -> Option<i64> {
            self.as_ref().either(
                |t| <T as MsgTrait>::sfixed64_required(t),
                |u| <U as MsgTrait>::sfixed64_required(u),
            )
        }
        fn sfixed64_optional<'this>(&'this self) -> Option<i64> {
            self.as_ref().either(
                |t| <T as MsgTrait>::sfixed64_optional(t),
                |u| <U as MsgTrait>::sfixed64_optional(u),
            )
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as MsgTrait>::Field183RepeatedType<'this>,
            <U as MsgTrait>::Field183RepeatedType<'this>,
        >;

        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::sfixed64_repeated(t))
                    .map_right(|u| <U as MsgTrait>::sfixed64_repeated(u)),
            )
        }
        fn f64_required<'this>(&'this self) -> Option<f64> {
            self.as_ref().either(
                |t| <T as MsgTrait>::f64_required(t),
                |u| <U as MsgTrait>::f64_required(u),
            )
        }
        fn f64_optional<'this>(&'this self) -> Option<f64> {
            self.as_ref().either(
                |t| <T as MsgTrait>::f64_optional(t),
                |u| <U as MsgTrait>::f64_optional(u),
            )
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as MsgTrait>::Field193RepeatedType<'this>,
            <U as MsgTrait>::Field193RepeatedType<'this>,
        >;

        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::f64_repeated(t))
                    .map_right(|u| <U as MsgTrait>::f64_repeated(u)),
            )
        }
    }
    impl<T> MsgTrait for ::std::option::Option<T>
    where
        T: MsgTrait,
    {
        fn i32_required<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().and_then(|msg| msg.i32_required())
        }
        fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().and_then(|msg| msg.i32_optional())
        }
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field3RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.i32_repeated().into_iter())
                .into_iter()
                .flatten()
        }
        fn float_required<'this>(&'this self) -> ::std::option::Option<f32> {
            self.as_ref().and_then(|msg| msg.float_required())
        }
        fn float_optional<'this>(&'this self) -> ::std::option::Option<f32> {
            self.as_ref().and_then(|msg| msg.float_optional())
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field13RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.float_repeated().into_iter())
                .into_iter()
                .flatten()
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = T::Field21BytesType<'this>;
        fn bytes_required<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field21BytesType<'this>> {
            self.as_ref().and_then(|msg| msg.bytes_required())
        }
        type Field22BytesType<'this>
        where
            Self: 'this,
        = T::Field22BytesType<'this>;
        fn bytes_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field22BytesType<'this>> {
            self.as_ref().and_then(|msg| msg.bytes_optional())
        }
        type Field23BytesType<'this>
        where
            Self: 'this,
        = T::Field23BytesType<'this>;
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field23RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.bytes_repeated().into_iter())
                .into_iter()
                .flatten()
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = T::Field31StringType<'this>;
        fn string_required<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field31StringType<'this>> {
            self.as_ref().and_then(|msg| msg.string_required())
        }
        type Field32StringType<'this>
        where
            Self: 'this,
        = T::Field32StringType<'this>;
        fn string_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field32StringType<'this>> {
            self.as_ref().and_then(|msg| msg.string_optional())
        }
        type Field33StringType<'this>
        where
            Self: 'this,
        = T::Field33StringType<'this>;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field33RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.string_repeated().into_iter())
                .into_iter()
                .flatten()
        }
        fn enum_required<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::full_coverage2::Enum> {
            self.as_ref().and_then(|msg| msg.enum_required())
        }
        fn enum_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::full_coverage2::Enum> {
            self.as_ref().and_then(|msg| msg.enum_optional())
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field43RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.enum_repeated().into_iter())
                .into_iter()
                .flatten()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = T::Field51MessageType<'this>;
        fn submsg_required<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field51MessageType<'this>> {
            self.as_ref().and_then(|msg| msg.submsg_required())
        }
        type Field52MessageType<'this>
        where
            Self: 'this,
        = T::Field52MessageType<'this>;
        fn submsg_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field52MessageType<'this>> {
            self.as_ref().and_then(|msg| msg.submsg_optional())
        }
        type Field53MessageType<'this>
        where
            Self: 'this,
        = T::Field53MessageType<'this>;
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field53RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.submsg_repeated().into_iter())
                .into_iter()
                .flatten()
        }
        fn i64_required<'this>(&'this self) -> ::std::option::Option<i64> {
            self.as_ref().and_then(|msg| msg.i64_required())
        }
        fn i64_optional<'this>(&'this self) -> ::std::option::Option<i64> {
            self.as_ref().and_then(|msg| msg.i64_optional())
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field103RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.i64_repeated().into_iter())
                .into_iter()
                .flatten()
        }
        fn u32_required<'this>(&'this self) -> ::std::option::Option<u32> {
            self.as_ref().and_then(|msg| msg.u32_required())
        }
        fn u32_optional<'this>(&'this self) -> ::std::option::Option<u32> {
            self.as_ref().and_then(|msg| msg.u32_optional())
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field113RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.u32_repeated().into_iter())
                .into_iter()
                .flatten()
        }
        fn u64_required<'this>(&'this self) -> ::std::option::Option<u64> {
            self.as_ref().and_then(|msg| msg.u64_required())
        }
        fn u64_optional<'this>(&'this self) -> ::std::option::Option<u64> {
            self.as_ref().and_then(|msg| msg.u64_optional())
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field123RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.u64_repeated().into_iter())
                .into_iter()
                .flatten()
        }
        fn s32_required<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().and_then(|msg| msg.s32_required())
        }
        fn s32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().and_then(|msg| msg.s32_optional())
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field133RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.s32_repeated().into_iter())
                .into_iter()
                .flatten()
        }
        fn s64_required<'this>(&'this self) -> ::std::option::Option<i64> {
            self.as_ref().and_then(|msg| msg.s64_required())
        }
        fn s64_optional<'this>(&'this self) -> ::std::option::Option<i64> {
            self.as_ref().and_then(|msg| msg.s64_optional())
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field143RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.s64_repeated().into_iter())
                .into_iter()
                .flatten()
        }
        fn fixed32_required<'this>(&'this self) -> ::std::option::Option<u32> {
            self.as_ref().and_then(|msg| msg.fixed32_required())
        }
        fn fixed32_optional<'this>(&'this self) -> ::std::option::Option<u32> {
            self.as_ref().and_then(|msg| msg.fixed32_optional())
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field153RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.fixed32_repeated().into_iter())
                .into_iter()
                .flatten()
        }
        fn fixed64_required<'this>(&'this self) -> ::std::option::Option<u64> {
            self.as_ref().and_then(|msg| msg.fixed64_required())
        }
        fn fixed64_optional<'this>(&'this self) -> ::std::option::Option<u64> {
            self.as_ref().and_then(|msg| msg.fixed64_optional())
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field163RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.fixed64_repeated().into_iter())
                .into_iter()
                .flatten()
        }
        fn sfixed32_required<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().and_then(|msg| msg.sfixed32_required())
        }
        fn sfixed32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().and_then(|msg| msg.sfixed32_optional())
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field173RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.sfixed32_repeated().into_iter())
                .into_iter()
                .flatten()
        }
        fn sfixed64_required<'this>(&'this self) -> ::std::option::Option<i64> {
            self.as_ref().and_then(|msg| msg.sfixed64_required())
        }
        fn sfixed64_optional<'this>(&'this self) -> ::std::option::Option<i64> {
            self.as_ref().and_then(|msg| msg.sfixed64_optional())
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field183RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.sfixed64_repeated().into_iter())
                .into_iter()
                .flatten()
        }
        fn f64_required<'this>(&'this self) -> ::std::option::Option<f64> {
            self.as_ref().and_then(|msg| msg.f64_required())
        }
        fn f64_optional<'this>(&'this self) -> ::std::option::Option<f64> {
            self.as_ref().and_then(|msg| msg.f64_optional())
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field193RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.f64_repeated().into_iter())
                .into_iter()
                .flatten()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField1 {
        pub i32_required: ::std::option::Option<i32>,
    }

    impl ::puroro::Message<super::Msg> for MsgSingleField1 {}

    impl super::_puroro_traits::MsgTrait for MsgSingleField1 {
        fn i32_required<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.i32_required)
        }
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field22BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field23BytesType<'this>>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field32StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field33StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for MsgSingleField1 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Required, ::puroro::tags::Int32>::ser_field::<
                (),
                _,
                _,
            >(&self.i32_required, 1, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<i32>> for MsgSingleField1 {
        fn from(value: ::std::option::Option<i32>) -> Self {
            Self {
                i32_required: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField2 {
        pub i32_optional: ::std::option::Option<i32>,
    }

    impl ::puroro::Message<super::Msg> for MsgSingleField2 {}

    impl super::_puroro_traits::MsgTrait for MsgSingleField2 {
        fn i32_optional<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.i32_optional)
        }
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field22BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field23BytesType<'this>>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field32StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field33StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for MsgSingleField2 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Int32>::ser_field::<
                (),
                _,
                _,
            >(&self.i32_optional, 2, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<i32>> for MsgSingleField2 {
        fn from(value: ::std::option::Option<i32>) -> Self {
            Self {
                i32_optional: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField3<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i32>,
    {
        pub i32_repeated: RepeatedType,
    }

    impl<RepeatedType> ::puroro::Message<super::Msg> for MsgSingleField3<RepeatedType> where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i32>
    {
    }

    impl<RepeatedType> super::_puroro_traits::MsgTrait for MsgSingleField3<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i32>,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Cloned<<&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter>;

        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::std::iter::Iterator::cloned(::std::iter::IntoIterator::into_iter(&self.i32_repeated))
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field22BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field23BytesType<'this>>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field32StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field33StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
        for MsgSingleField3<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i32>,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::Int32>::ser_field::<
                (),
                _,
                _,
            >(&self.i32_repeated, 3, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<RepeatedType> ::std::convert::From<RepeatedType> for MsgSingleField3<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i32>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                i32_repeated: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField11 {
        pub float_required: ::std::option::Option<f32>,
    }

    impl ::puroro::Message<super::Msg> for MsgSingleField11 {}

    impl super::_puroro_traits::MsgTrait for MsgSingleField11 {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_required<'this>(&'this self) -> Option<f32> {
            Clone::clone(&self.float_required)
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field22BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field23BytesType<'this>>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field32StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field33StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for MsgSingleField11 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Required, ::puroro::tags::Float>::ser_field::<
                (),
                _,
                _,
            >(&self.float_required, 11, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<f32>> for MsgSingleField11 {
        fn from(value: ::std::option::Option<f32>) -> Self {
            Self {
                float_required: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField12 {
        pub float_optional: ::std::option::Option<f32>,
    }

    impl ::puroro::Message<super::Msg> for MsgSingleField12 {}

    impl super::_puroro_traits::MsgTrait for MsgSingleField12 {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_optional<'this>(&'this self) -> Option<f32> {
            Clone::clone(&self.float_optional)
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field22BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field23BytesType<'this>>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field32StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field33StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for MsgSingleField12 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Float>::ser_field::<
                (),
                _,
                _,
            >(&self.float_optional, 12, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<f32>> for MsgSingleField12 {
        fn from(value: ::std::option::Option<f32>) -> Self {
            Self {
                float_optional: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField13<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a f32>,
    {
        pub float_repeated: RepeatedType,
    }

    impl<RepeatedType> ::puroro::Message<super::Msg> for MsgSingleField13<RepeatedType> where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a f32>
    {
    }

    impl<RepeatedType> super::_puroro_traits::MsgTrait for MsgSingleField13<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a f32>,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Cloned<<&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter>;

        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::std::iter::Iterator::cloned(::std::iter::IntoIterator::into_iter(
                &self.float_repeated,
            ))
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field22BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field23BytesType<'this>>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field32StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field33StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
        for MsgSingleField13<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a f32>,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::Float>::ser_field::<
                (),
                _,
                _,
            >(&self.float_repeated, 13, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<RepeatedType> ::std::convert::From<RepeatedType> for MsgSingleField13<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a f32>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                float_repeated: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField21<ScalarType>
    where
        ScalarType: ::std::ops::Deref<Target = [u8]>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub bytes_required: ::std::option::Option<ScalarType>,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField21<ScalarType> where
        ScalarType: ::std::ops::Deref<Target = [u8]>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField21<ScalarType>
    where
        ScalarType: ::std::ops::Deref<Target = [u8]>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = &'this [u8];
        fn bytes_required<'this>(&'this self) -> Option<Self::Field21BytesType<'this>> {
            self.bytes_required.as_deref()
        }
        type Field22BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field23BytesType<'this>>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field32StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field33StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite for MsgSingleField21<ScalarType>
    where
        ScalarType: ::std::ops::Deref<Target = [u8]>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Required, ::puroro::tags::Bytes>::ser_field::<
                ScalarType,
                _,
                _,
            >(&self.bytes_required, 21, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<::std::option::Option<ScalarType>>
        for MsgSingleField21<ScalarType>
    where
        ScalarType: ::std::ops::Deref<Target = [u8]>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ::std::option::Option<ScalarType>) -> Self {
            Self {
                bytes_required: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField22<ScalarType>
    where
        ScalarType: ::std::ops::Deref<Target = [u8]>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub bytes_optional: ::std::option::Option<ScalarType>,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField22<ScalarType> where
        ScalarType: ::std::ops::Deref<Target = [u8]>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField22<ScalarType>
    where
        ScalarType: ::std::ops::Deref<Target = [u8]>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field22BytesType<'this>
        where
            Self: 'this,
        = &'this [u8];
        fn bytes_optional<'this>(&'this self) -> Option<Self::Field22BytesType<'this>> {
            self.bytes_optional.as_deref()
        }
        type Field23BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field23BytesType<'this>>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field32StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field33StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite for MsgSingleField22<ScalarType>
    where
        ScalarType: ::std::ops::Deref<Target = [u8]>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bytes>::ser_field::<
                ScalarType,
                _,
                _,
            >(&self.bytes_optional, 22, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<::std::option::Option<ScalarType>>
        for MsgSingleField22<ScalarType>
    where
        ScalarType: ::std::ops::Deref<Target = [u8]>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ::std::option::Option<ScalarType>) -> Self {
            Self {
                bytes_optional: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField23<ScalarType, RepeatedType>
    where
        ScalarType: ::std::ops::Deref<Target = [u8]>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub bytes_repeated: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::Msg>
        for MsgSingleField23<ScalarType, RepeatedType>
    where
        ScalarType: ::std::ops::Deref<Target = [u8]>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::MsgTrait
        for MsgSingleField23<ScalarType, RepeatedType>
    where
        ScalarType: ::std::ops::Deref<Target = [u8]>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field22BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23BytesType<'this>
        where
            Self: 'this,
        = &'this [u8];
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::single_field::DerefIter<
            <&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter,
        >;

        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::single_field::DerefIter::new(
                ::std::iter::IntoIterator::into_iter(&self.bytes_repeated),
            )
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field32StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field33StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
        for MsgSingleField23<ScalarType, RepeatedType>
    where
        ScalarType: ::std::ops::Deref<Target = [u8]>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::Bytes>::ser_field::<
                ScalarType,
                _,
                _,
            >(&self.bytes_repeated, 23, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for MsgSingleField23<ScalarType, RepeatedType>
    where
        ScalarType: ::std::ops::Deref<Target = [u8]>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                bytes_repeated: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField31<ScalarType>
    where
        ScalarType: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub string_required: ::std::option::Option<ScalarType>,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField31<ScalarType> where
        ScalarType: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField31<ScalarType>
    where
        ScalarType: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field22BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field23BytesType<'this>>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = &'this str;
        fn string_required<'this>(&'this self) -> Option<Self::Field31StringType<'this>> {
            self.string_required.as_deref()
        }
        type Field32StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field33StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite for MsgSingleField31<ScalarType>
    where
        ScalarType: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Required, ::puroro::tags::String>::ser_field::<
                ScalarType,
                _,
                _,
            >(&self.string_required, 31, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<::std::option::Option<ScalarType>>
        for MsgSingleField31<ScalarType>
    where
        ScalarType: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ::std::option::Option<ScalarType>) -> Self {
            Self {
                string_required: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField32<ScalarType>
    where
        ScalarType: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub string_optional: ::std::option::Option<ScalarType>,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField32<ScalarType> where
        ScalarType: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField32<ScalarType>
    where
        ScalarType: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field22BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field23BytesType<'this>>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field32StringType<'this>
        where
            Self: 'this,
        = &'this str;
        fn string_optional<'this>(&'this self) -> Option<Self::Field32StringType<'this>> {
            self.string_optional.as_deref()
        }
        type Field33StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field33StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite for MsgSingleField32<ScalarType>
    where
        ScalarType: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field::<
                ScalarType,
                _,
                _,
            >(&self.string_optional, 32, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<::std::option::Option<ScalarType>>
        for MsgSingleField32<ScalarType>
    where
        ScalarType: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ::std::option::Option<ScalarType>) -> Self {
            Self {
                string_optional: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField33<ScalarType, RepeatedType>
    where
        ScalarType: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub string_repeated: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::Msg>
        for MsgSingleField33<ScalarType, RepeatedType>
    where
        ScalarType: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::MsgTrait
        for MsgSingleField33<ScalarType, RepeatedType>
    where
        ScalarType: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field22BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field23BytesType<'this>>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field32StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33StringType<'this>
        where
            Self: 'this,
        = &'this str;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::single_field::DerefIter<
            <&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter,
        >;

        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::single_field::DerefIter::new(
                ::std::iter::IntoIterator::into_iter(&self.string_repeated),
            )
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
        for MsgSingleField33<ScalarType, RepeatedType>
    where
        ScalarType: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::String>::ser_field::<
                ScalarType,
                _,
                _,
            >(&self.string_repeated, 33, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for MsgSingleField33<ScalarType, RepeatedType>
    where
        ScalarType: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                string_repeated: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField41 {
        pub enum_required: ::std::option::Option<self::_puroro_root::full_coverage2::Enum>,
    }

    impl ::puroro::Message<super::Msg> for MsgSingleField41 {}

    impl super::_puroro_traits::MsgTrait for MsgSingleField41 {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field22BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field23BytesType<'this>>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field32StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field33StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_required<'this>(&'this self) -> Option<self::_puroro_root::full_coverage2::Enum> {
            Clone::clone(&self.enum_required)
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for MsgSingleField41 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
                ::puroro::tags::Required,
                ::puroro::tags::Enum2<self::_puroro_root::full_coverage2::Enum>,
            >::ser_field::<(), _, _>(&self.enum_required, 41, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<self::_puroro_root::full_coverage2::Enum>>
        for MsgSingleField41
    {
        fn from(value: ::std::option::Option<self::_puroro_root::full_coverage2::Enum>) -> Self {
            Self {
                enum_required: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField42 {
        pub enum_optional: ::std::option::Option<self::_puroro_root::full_coverage2::Enum>,
    }

    impl ::puroro::Message<super::Msg> for MsgSingleField42 {}

    impl super::_puroro_traits::MsgTrait for MsgSingleField42 {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field22BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field23BytesType<'this>>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field32StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field33StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_optional<'this>(&'this self) -> Option<self::_puroro_root::full_coverage2::Enum> {
            Clone::clone(&self.enum_optional)
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for MsgSingleField42 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Enum2<self::_puroro_root::full_coverage2::Enum>,
            >::ser_field::<(), _, _>(&self.enum_optional, 42, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<self::_puroro_root::full_coverage2::Enum>>
        for MsgSingleField42
    {
        fn from(value: ::std::option::Option<self::_puroro_root::full_coverage2::Enum>) -> Self {
            Self {
                enum_optional: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField43<RepeatedType>
    where
        for<'a> &'a RepeatedType:
            ::std::iter::IntoIterator<Item = &'a self::_puroro_root::full_coverage2::Enum>,
    {
        pub enum_repeated: RepeatedType,
    }

    impl<RepeatedType> ::puroro::Message<super::Msg> for MsgSingleField43<RepeatedType> where
        for<'a> &'a RepeatedType:
            ::std::iter::IntoIterator<Item = &'a self::_puroro_root::full_coverage2::Enum>
    {
    }

    impl<RepeatedType> super::_puroro_traits::MsgTrait for MsgSingleField43<RepeatedType>
    where
        for<'a> &'a RepeatedType:
            ::std::iter::IntoIterator<Item = &'a self::_puroro_root::full_coverage2::Enum>,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field22BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field23BytesType<'this>>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field32StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field33StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Cloned<<&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter>;

        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::std::iter::Iterator::cloned(::std::iter::IntoIterator::into_iter(&self.enum_repeated))
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
        for MsgSingleField43<RepeatedType>
    where
        for<'a> &'a RepeatedType:
            ::std::iter::IntoIterator<Item = &'a self::_puroro_root::full_coverage2::Enum>,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Enum2<self::_puroro_root::full_coverage2::Enum>,
            >::ser_field::<(), _, _>(&self.enum_repeated, 43, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<RepeatedType> ::std::convert::From<RepeatedType> for MsgSingleField43<RepeatedType>
    where
        for<'a> &'a RepeatedType:
            ::std::iter::IntoIterator<Item = &'a self::_puroro_root::full_coverage2::Enum>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                enum_repeated: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField51<ScalarType>
    where
        ScalarType:
            self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_traits::SubmsgTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
    {
        pub submsg_required: ::std::option::Option<ScalarType>,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField51<ScalarType> where
        ScalarType:
            self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_traits::SubmsgTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField51<ScalarType>
    where
        ScalarType:
            self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_traits::SubmsgTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field22BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field23BytesType<'this>>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field32StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field33StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = &'this ScalarType;
        fn submsg_required<'this>(&'this self) -> Option<Self::Field51MessageType<'this>> {
            self.submsg_required.as_ref()
        }
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite for MsgSingleField51<ScalarType>
    where
        ScalarType:
            self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_traits::SubmsgTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        ScalarType: ::puroro::internal::SerializableMessageToIoWrite,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
            ::puroro::tags::Required, ::puroro::tags::Message<ScalarType>
        >::ser_field::
        <ScalarType, _, _>
        (
            &self.submsg_required,
            51,
            out
        )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<::std::option::Option<ScalarType>>
        for MsgSingleField51<ScalarType>
    where
        ScalarType:
            self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_traits::SubmsgTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
    {
        fn from(value: ::std::option::Option<ScalarType>) -> Self {
            Self {
                submsg_required: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField52<ScalarType>
    where
        ScalarType:
            self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_traits::SubmsgTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
    {
        pub submsg_optional: ::std::option::Option<ScalarType>,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField52<ScalarType> where
        ScalarType:
            self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_traits::SubmsgTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField52<ScalarType>
    where
        ScalarType:
            self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_traits::SubmsgTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field22BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field23BytesType<'this>>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field32StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field33StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = &'this ScalarType;
        fn submsg_optional<'this>(&'this self) -> Option<Self::Field52MessageType<'this>> {
            self.submsg_optional.as_ref()
        }
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite for MsgSingleField52<ScalarType>
    where
        ScalarType:
            self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_traits::SubmsgTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        ScalarType: ::puroro::internal::SerializableMessageToIoWrite,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
            ::puroro::tags::Optional, ::puroro::tags::Message<ScalarType>
        >::ser_field::
        <ScalarType, _, _>
        (
            &self.submsg_optional,
            52,
            out
        )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<::std::option::Option<ScalarType>>
        for MsgSingleField52<ScalarType>
    where
        ScalarType:
            self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_traits::SubmsgTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
    {
        fn from(value: ::std::option::Option<ScalarType>) -> Self {
            Self {
                submsg_optional: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField53<ScalarType, RepeatedType>
    where
        ScalarType:
            self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_traits::SubmsgTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub submsg_repeated: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::Msg>
        for MsgSingleField53<ScalarType, RepeatedType>
    where
        ScalarType:
            self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_traits::SubmsgTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::MsgTrait
        for MsgSingleField53<ScalarType, RepeatedType>
    where
        ScalarType:
            self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_traits::SubmsgTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field22BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field23BytesType<'this>>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field32StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field33StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = &'this ScalarType;
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = <&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter;

        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::std::iter::IntoIterator::into_iter(&self.submsg_repeated)
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
        for MsgSingleField53<ScalarType, RepeatedType>
    where
        ScalarType:
            self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_traits::SubmsgTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
        ScalarType: ::puroro::internal::SerializableMessageToIoWrite,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::Message<ScalarType>
        >::ser_field::
        <ScalarType, _, _>
        (
            &self.submsg_repeated,
            53,
            out
        )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for MsgSingleField53<ScalarType, RepeatedType>
    where
        ScalarType:
            self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_traits::SubmsgTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                submsg_repeated: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField101 {
        pub i64_required: ::std::option::Option<i64>,
    }

    impl ::puroro::Message<super::Msg> for MsgSingleField101 {}

    impl super::_puroro_traits::MsgTrait for MsgSingleField101 {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field22BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field23BytesType<'this>>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field32StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field33StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn i64_required<'this>(&'this self) -> Option<i64> {
            Clone::clone(&self.i64_required)
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for MsgSingleField101 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Required, ::puroro::tags::Int64>::ser_field::<
                (),
                _,
                _,
            >(&self.i64_required, 101, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<i64>> for MsgSingleField101 {
        fn from(value: ::std::option::Option<i64>) -> Self {
            Self {
                i64_required: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField102 {
        pub i64_optional: ::std::option::Option<i64>,
    }

    impl ::puroro::Message<super::Msg> for MsgSingleField102 {}

    impl super::_puroro_traits::MsgTrait for MsgSingleField102 {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field22BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field23BytesType<'this>>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field32StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field33StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn i64_optional<'this>(&'this self) -> Option<i64> {
            Clone::clone(&self.i64_optional)
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for MsgSingleField102 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Int64>::ser_field::<
                (),
                _,
                _,
            >(&self.i64_optional, 102, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<i64>> for MsgSingleField102 {
        fn from(value: ::std::option::Option<i64>) -> Self {
            Self {
                i64_optional: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField103<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i64>,
    {
        pub i64_repeated: RepeatedType,
    }

    impl<RepeatedType> ::puroro::Message<super::Msg> for MsgSingleField103<RepeatedType> where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i64>
    {
    }

    impl<RepeatedType> super::_puroro_traits::MsgTrait for MsgSingleField103<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i64>,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field22BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field23BytesType<'this>>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field32StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field33StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Cloned<<&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter>;

        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::std::iter::Iterator::cloned(::std::iter::IntoIterator::into_iter(&self.i64_repeated))
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
        for MsgSingleField103<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i64>,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::Int64>::ser_field::<
                (),
                _,
                _,
            >(&self.i64_repeated, 103, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<RepeatedType> ::std::convert::From<RepeatedType> for MsgSingleField103<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i64>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                i64_repeated: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField111 {
        pub u32_required: ::std::option::Option<u32>,
    }

    impl ::puroro::Message<super::Msg> for MsgSingleField111 {}

    impl super::_puroro_traits::MsgTrait for MsgSingleField111 {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field22BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field23BytesType<'this>>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field32StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field33StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn u32_required<'this>(&'this self) -> Option<u32> {
            Clone::clone(&self.u32_required)
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for MsgSingleField111 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Required, ::puroro::tags::UInt32>::ser_field::<
                (),
                _,
                _,
            >(&self.u32_required, 111, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<u32>> for MsgSingleField111 {
        fn from(value: ::std::option::Option<u32>) -> Self {
            Self {
                u32_required: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField112 {
        pub u32_optional: ::std::option::Option<u32>,
    }

    impl ::puroro::Message<super::Msg> for MsgSingleField112 {}

    impl super::_puroro_traits::MsgTrait for MsgSingleField112 {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field22BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field23BytesType<'this>>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field32StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field33StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn u32_optional<'this>(&'this self) -> Option<u32> {
            Clone::clone(&self.u32_optional)
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for MsgSingleField112 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::UInt32>::ser_field::<
                (),
                _,
                _,
            >(&self.u32_optional, 112, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<u32>> for MsgSingleField112 {
        fn from(value: ::std::option::Option<u32>) -> Self {
            Self {
                u32_optional: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField113<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a u32>,
    {
        pub u32_repeated: RepeatedType,
    }

    impl<RepeatedType> ::puroro::Message<super::Msg> for MsgSingleField113<RepeatedType> where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a u32>
    {
    }

    impl<RepeatedType> super::_puroro_traits::MsgTrait for MsgSingleField113<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a u32>,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field22BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field23BytesType<'this>>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field32StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field33StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Cloned<<&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter>;

        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::std::iter::Iterator::cloned(::std::iter::IntoIterator::into_iter(&self.u32_repeated))
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
        for MsgSingleField113<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a u32>,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::UInt32>::ser_field::<
                (),
                _,
                _,
            >(&self.u32_repeated, 113, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<RepeatedType> ::std::convert::From<RepeatedType> for MsgSingleField113<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a u32>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                u32_repeated: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField121 {
        pub u64_required: ::std::option::Option<u64>,
    }

    impl ::puroro::Message<super::Msg> for MsgSingleField121 {}

    impl super::_puroro_traits::MsgTrait for MsgSingleField121 {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field22BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field23BytesType<'this>>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field32StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field33StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn u64_required<'this>(&'this self) -> Option<u64> {
            Clone::clone(&self.u64_required)
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for MsgSingleField121 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Required, ::puroro::tags::UInt64>::ser_field::<
                (),
                _,
                _,
            >(&self.u64_required, 121, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<u64>> for MsgSingleField121 {
        fn from(value: ::std::option::Option<u64>) -> Self {
            Self {
                u64_required: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField122 {
        pub u64_optional: ::std::option::Option<u64>,
    }

    impl ::puroro::Message<super::Msg> for MsgSingleField122 {}

    impl super::_puroro_traits::MsgTrait for MsgSingleField122 {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field22BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field23BytesType<'this>>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field32StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field33StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn u64_optional<'this>(&'this self) -> Option<u64> {
            Clone::clone(&self.u64_optional)
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for MsgSingleField122 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::UInt64>::ser_field::<
                (),
                _,
                _,
            >(&self.u64_optional, 122, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<u64>> for MsgSingleField122 {
        fn from(value: ::std::option::Option<u64>) -> Self {
            Self {
                u64_optional: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField123<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a u64>,
    {
        pub u64_repeated: RepeatedType,
    }

    impl<RepeatedType> ::puroro::Message<super::Msg> for MsgSingleField123<RepeatedType> where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a u64>
    {
    }

    impl<RepeatedType> super::_puroro_traits::MsgTrait for MsgSingleField123<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a u64>,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field22BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field23BytesType<'this>>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field32StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field33StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Cloned<<&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter>;

        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::std::iter::Iterator::cloned(::std::iter::IntoIterator::into_iter(&self.u64_repeated))
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
        for MsgSingleField123<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a u64>,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::UInt64>::ser_field::<
                (),
                _,
                _,
            >(&self.u64_repeated, 123, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<RepeatedType> ::std::convert::From<RepeatedType> for MsgSingleField123<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a u64>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                u64_repeated: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField131 {
        pub s32_required: ::std::option::Option<i32>,
    }

    impl ::puroro::Message<super::Msg> for MsgSingleField131 {}

    impl super::_puroro_traits::MsgTrait for MsgSingleField131 {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field22BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field23BytesType<'this>>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field32StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field33StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn s32_required<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.s32_required)
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for MsgSingleField131 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Required, ::puroro::tags::SInt32>::ser_field::<
                (),
                _,
                _,
            >(&self.s32_required, 131, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<i32>> for MsgSingleField131 {
        fn from(value: ::std::option::Option<i32>) -> Self {
            Self {
                s32_required: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField132 {
        pub s32_optional: ::std::option::Option<i32>,
    }

    impl ::puroro::Message<super::Msg> for MsgSingleField132 {}

    impl super::_puroro_traits::MsgTrait for MsgSingleField132 {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field22BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field23BytesType<'this>>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field32StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field33StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn s32_optional<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.s32_optional)
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for MsgSingleField132 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::SInt32>::ser_field::<
                (),
                _,
                _,
            >(&self.s32_optional, 132, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<i32>> for MsgSingleField132 {
        fn from(value: ::std::option::Option<i32>) -> Self {
            Self {
                s32_optional: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField133<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i32>,
    {
        pub s32_repeated: RepeatedType,
    }

    impl<RepeatedType> ::puroro::Message<super::Msg> for MsgSingleField133<RepeatedType> where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i32>
    {
    }

    impl<RepeatedType> super::_puroro_traits::MsgTrait for MsgSingleField133<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i32>,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field22BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field23BytesType<'this>>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field32StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field33StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Cloned<<&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter>;

        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::std::iter::Iterator::cloned(::std::iter::IntoIterator::into_iter(&self.s32_repeated))
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
        for MsgSingleField133<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i32>,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::SInt32>::ser_field::<
                (),
                _,
                _,
            >(&self.s32_repeated, 133, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<RepeatedType> ::std::convert::From<RepeatedType> for MsgSingleField133<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i32>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                s32_repeated: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField141 {
        pub s64_required: ::std::option::Option<i64>,
    }

    impl ::puroro::Message<super::Msg> for MsgSingleField141 {}

    impl super::_puroro_traits::MsgTrait for MsgSingleField141 {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field22BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field23BytesType<'this>>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field32StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field33StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn s64_required<'this>(&'this self) -> Option<i64> {
            Clone::clone(&self.s64_required)
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for MsgSingleField141 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Required, ::puroro::tags::SInt64>::ser_field::<
                (),
                _,
                _,
            >(&self.s64_required, 141, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<i64>> for MsgSingleField141 {
        fn from(value: ::std::option::Option<i64>) -> Self {
            Self {
                s64_required: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField142 {
        pub s64_optional: ::std::option::Option<i64>,
    }

    impl ::puroro::Message<super::Msg> for MsgSingleField142 {}

    impl super::_puroro_traits::MsgTrait for MsgSingleField142 {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field22BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field23BytesType<'this>>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field32StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field33StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn s64_optional<'this>(&'this self) -> Option<i64> {
            Clone::clone(&self.s64_optional)
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for MsgSingleField142 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::SInt64>::ser_field::<
                (),
                _,
                _,
            >(&self.s64_optional, 142, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<i64>> for MsgSingleField142 {
        fn from(value: ::std::option::Option<i64>) -> Self {
            Self {
                s64_optional: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField143<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i64>,
    {
        pub s64_repeated: RepeatedType,
    }

    impl<RepeatedType> ::puroro::Message<super::Msg> for MsgSingleField143<RepeatedType> where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i64>
    {
    }

    impl<RepeatedType> super::_puroro_traits::MsgTrait for MsgSingleField143<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i64>,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field22BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field23BytesType<'this>>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field32StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field33StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Cloned<<&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter>;

        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::std::iter::Iterator::cloned(::std::iter::IntoIterator::into_iter(&self.s64_repeated))
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
        for MsgSingleField143<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i64>,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::SInt64>::ser_field::<
                (),
                _,
                _,
            >(&self.s64_repeated, 143, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<RepeatedType> ::std::convert::From<RepeatedType> for MsgSingleField143<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i64>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                s64_repeated: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField151 {
        pub fixed32_required: ::std::option::Option<u32>,
    }

    impl ::puroro::Message<super::Msg> for MsgSingleField151 {}

    impl super::_puroro_traits::MsgTrait for MsgSingleField151 {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field22BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field23BytesType<'this>>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field32StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field33StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn fixed32_required<'this>(&'this self) -> Option<u32> {
            Clone::clone(&self.fixed32_required)
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for MsgSingleField151 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Required, ::puroro::tags::Fixed32>::ser_field::<
                (),
                _,
                _,
            >(&self.fixed32_required, 151, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<u32>> for MsgSingleField151 {
        fn from(value: ::std::option::Option<u32>) -> Self {
            Self {
                fixed32_required: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField152 {
        pub fixed32_optional: ::std::option::Option<u32>,
    }

    impl ::puroro::Message<super::Msg> for MsgSingleField152 {}

    impl super::_puroro_traits::MsgTrait for MsgSingleField152 {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field22BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field23BytesType<'this>>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field32StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field33StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn fixed32_optional<'this>(&'this self) -> Option<u32> {
            Clone::clone(&self.fixed32_optional)
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for MsgSingleField152 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Fixed32>::ser_field::<
                (),
                _,
                _,
            >(&self.fixed32_optional, 152, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<u32>> for MsgSingleField152 {
        fn from(value: ::std::option::Option<u32>) -> Self {
            Self {
                fixed32_optional: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField153<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a u32>,
    {
        pub fixed32_repeated: RepeatedType,
    }

    impl<RepeatedType> ::puroro::Message<super::Msg> for MsgSingleField153<RepeatedType> where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a u32>
    {
    }

    impl<RepeatedType> super::_puroro_traits::MsgTrait for MsgSingleField153<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a u32>,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field22BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field23BytesType<'this>>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field32StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field33StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Cloned<<&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter>;

        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::std::iter::Iterator::cloned(::std::iter::IntoIterator::into_iter(
                &self.fixed32_repeated,
            ))
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
        for MsgSingleField153<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a u32>,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::Fixed32>::ser_field::<
                (),
                _,
                _,
            >(&self.fixed32_repeated, 153, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<RepeatedType> ::std::convert::From<RepeatedType> for MsgSingleField153<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a u32>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                fixed32_repeated: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField161 {
        pub fixed64_required: ::std::option::Option<u64>,
    }

    impl ::puroro::Message<super::Msg> for MsgSingleField161 {}

    impl super::_puroro_traits::MsgTrait for MsgSingleField161 {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field22BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field23BytesType<'this>>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field32StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field33StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn fixed64_required<'this>(&'this self) -> Option<u64> {
            Clone::clone(&self.fixed64_required)
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for MsgSingleField161 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Required, ::puroro::tags::Fixed64>::ser_field::<
                (),
                _,
                _,
            >(&self.fixed64_required, 161, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<u64>> for MsgSingleField161 {
        fn from(value: ::std::option::Option<u64>) -> Self {
            Self {
                fixed64_required: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField162 {
        pub fixed64_optional: ::std::option::Option<u64>,
    }

    impl ::puroro::Message<super::Msg> for MsgSingleField162 {}

    impl super::_puroro_traits::MsgTrait for MsgSingleField162 {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field22BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field23BytesType<'this>>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field32StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field33StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn fixed64_optional<'this>(&'this self) -> Option<u64> {
            Clone::clone(&self.fixed64_optional)
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for MsgSingleField162 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Fixed64>::ser_field::<
                (),
                _,
                _,
            >(&self.fixed64_optional, 162, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<u64>> for MsgSingleField162 {
        fn from(value: ::std::option::Option<u64>) -> Self {
            Self {
                fixed64_optional: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField163<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a u64>,
    {
        pub fixed64_repeated: RepeatedType,
    }

    impl<RepeatedType> ::puroro::Message<super::Msg> for MsgSingleField163<RepeatedType> where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a u64>
    {
    }

    impl<RepeatedType> super::_puroro_traits::MsgTrait for MsgSingleField163<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a u64>,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field22BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field23BytesType<'this>>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field32StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field33StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Cloned<<&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter>;

        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::std::iter::Iterator::cloned(::std::iter::IntoIterator::into_iter(
                &self.fixed64_repeated,
            ))
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
        for MsgSingleField163<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a u64>,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::Fixed64>::ser_field::<
                (),
                _,
                _,
            >(&self.fixed64_repeated, 163, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<RepeatedType> ::std::convert::From<RepeatedType> for MsgSingleField163<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a u64>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                fixed64_repeated: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField171 {
        pub sfixed32_required: ::std::option::Option<i32>,
    }

    impl ::puroro::Message<super::Msg> for MsgSingleField171 {}

    impl super::_puroro_traits::MsgTrait for MsgSingleField171 {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field22BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field23BytesType<'this>>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field32StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field33StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn sfixed32_required<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.sfixed32_required)
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for MsgSingleField171 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Required, ::puroro::tags::SFixed32>::ser_field::<
                (),
                _,
                _,
            >(&self.sfixed32_required, 171, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<i32>> for MsgSingleField171 {
        fn from(value: ::std::option::Option<i32>) -> Self {
            Self {
                sfixed32_required: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField172 {
        pub sfixed32_optional: ::std::option::Option<i32>,
    }

    impl ::puroro::Message<super::Msg> for MsgSingleField172 {}

    impl super::_puroro_traits::MsgTrait for MsgSingleField172 {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field22BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field23BytesType<'this>>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field32StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field33StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn sfixed32_optional<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.sfixed32_optional)
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for MsgSingleField172 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::SFixed32>::ser_field::<
                (),
                _,
                _,
            >(&self.sfixed32_optional, 172, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<i32>> for MsgSingleField172 {
        fn from(value: ::std::option::Option<i32>) -> Self {
            Self {
                sfixed32_optional: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField173<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i32>,
    {
        pub sfixed32_repeated: RepeatedType,
    }

    impl<RepeatedType> ::puroro::Message<super::Msg> for MsgSingleField173<RepeatedType> where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i32>
    {
    }

    impl<RepeatedType> super::_puroro_traits::MsgTrait for MsgSingleField173<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i32>,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field22BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field23BytesType<'this>>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field32StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field33StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Cloned<<&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter>;

        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::std::iter::Iterator::cloned(::std::iter::IntoIterator::into_iter(
                &self.sfixed32_repeated,
            ))
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
        for MsgSingleField173<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i32>,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::SFixed32>::ser_field::<
                (),
                _,
                _,
            >(&self.sfixed32_repeated, 173, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<RepeatedType> ::std::convert::From<RepeatedType> for MsgSingleField173<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i32>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                sfixed32_repeated: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField181 {
        pub sfixed64_required: ::std::option::Option<i64>,
    }

    impl ::puroro::Message<super::Msg> for MsgSingleField181 {}

    impl super::_puroro_traits::MsgTrait for MsgSingleField181 {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field22BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field23BytesType<'this>>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field32StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field33StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn sfixed64_required<'this>(&'this self) -> Option<i64> {
            Clone::clone(&self.sfixed64_required)
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for MsgSingleField181 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Required, ::puroro::tags::SFixed64>::ser_field::<
                (),
                _,
                _,
            >(&self.sfixed64_required, 181, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<i64>> for MsgSingleField181 {
        fn from(value: ::std::option::Option<i64>) -> Self {
            Self {
                sfixed64_required: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField182 {
        pub sfixed64_optional: ::std::option::Option<i64>,
    }

    impl ::puroro::Message<super::Msg> for MsgSingleField182 {}

    impl super::_puroro_traits::MsgTrait for MsgSingleField182 {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field22BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field23BytesType<'this>>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field32StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field33StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn sfixed64_optional<'this>(&'this self) -> Option<i64> {
            Clone::clone(&self.sfixed64_optional)
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for MsgSingleField182 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::SFixed64>::ser_field::<
                (),
                _,
                _,
            >(&self.sfixed64_optional, 182, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<i64>> for MsgSingleField182 {
        fn from(value: ::std::option::Option<i64>) -> Self {
            Self {
                sfixed64_optional: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField183<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i64>,
    {
        pub sfixed64_repeated: RepeatedType,
    }

    impl<RepeatedType> ::puroro::Message<super::Msg> for MsgSingleField183<RepeatedType> where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i64>
    {
    }

    impl<RepeatedType> super::_puroro_traits::MsgTrait for MsgSingleField183<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i64>,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field22BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field23BytesType<'this>>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field32StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field33StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Cloned<<&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter>;

        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::std::iter::Iterator::cloned(::std::iter::IntoIterator::into_iter(
                &self.sfixed64_repeated,
            ))
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
        for MsgSingleField183<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i64>,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::SFixed64>::ser_field::<
                (),
                _,
                _,
            >(&self.sfixed64_repeated, 183, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<RepeatedType> ::std::convert::From<RepeatedType> for MsgSingleField183<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i64>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                sfixed64_repeated: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField191 {
        pub f64_required: ::std::option::Option<f64>,
    }

    impl ::puroro::Message<super::Msg> for MsgSingleField191 {}

    impl super::_puroro_traits::MsgTrait for MsgSingleField191 {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field22BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field23BytesType<'this>>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field32StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field33StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn f64_required<'this>(&'this self) -> Option<f64> {
            Clone::clone(&self.f64_required)
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for MsgSingleField191 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Required, ::puroro::tags::Double>::ser_field::<
                (),
                _,
                _,
            >(&self.f64_required, 191, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<f64>> for MsgSingleField191 {
        fn from(value: ::std::option::Option<f64>) -> Self {
            Self {
                f64_required: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField192 {
        pub f64_optional: ::std::option::Option<f64>,
    }

    impl ::puroro::Message<super::Msg> for MsgSingleField192 {}

    impl super::_puroro_traits::MsgTrait for MsgSingleField192 {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field22BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field23BytesType<'this>>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field32StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field33StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn f64_optional<'this>(&'this self) -> Option<f64> {
            Clone::clone(&self.f64_optional)
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for MsgSingleField192 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Double>::ser_field::<
                (),
                _,
                _,
            >(&self.f64_optional, 192, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<f64>> for MsgSingleField192 {
        fn from(value: ::std::option::Option<f64>) -> Self {
            Self {
                f64_optional: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField193<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a f64>,
    {
        pub f64_repeated: RepeatedType,
    }

    impl<RepeatedType> ::puroro::Message<super::Msg> for MsgSingleField193<RepeatedType> where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a f64>
    {
    }

    impl<RepeatedType> super::_puroro_traits::MsgTrait for MsgSingleField193<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a f64>,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field13RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field22BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23BytesType<'this>
        where
            Self: 'this,
        = &'static [u8];
        type Field23RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field23BytesType<'this>>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field31StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field32StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field33RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field33StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field43RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field51MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field52MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field53RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field103RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field113RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field123RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field133RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field143RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field153RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field163RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field173RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field183RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field193RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Cloned<<&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter>;

        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            ::std::iter::Iterator::cloned(::std::iter::IntoIterator::into_iter(&self.f64_repeated))
        }
    }

    impl<RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
        for MsgSingleField193<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a f64>,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::Double>::ser_field::<
                (),
                _,
                _,
            >(&self.f64_repeated, 193, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<RepeatedType> ::std::convert::From<RepeatedType> for MsgSingleField193<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a f64>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                f64_repeated: value,
            }
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct MsgSimpleByValue {}
    impl ::puroro::Message<super::Msg> for MsgSimpleByValue {}

    impl MsgTrait for MsgSimpleByValue {
        fn i32_required<'this>(&'this self) -> Option<i32> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn i32_optional<'this>(&'this self) -> Option<i32> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field3RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn float_required<'this>(&'this self) -> Option<f32> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn float_optional<'this>(&'this self) -> Option<f32> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field13RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field21BytesType<'this> = ::std::vec::Vec<u8>;
        fn bytes_required<'this>(&'this self) -> Option<Self::Field21BytesType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field22BytesType<'this> = ::std::vec::Vec<u8>;
        fn bytes_optional<'this>(&'this self) -> Option<Self::Field22BytesType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field23BytesType<'this> = ::std::vec::Vec<u8>;
        type Field23RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field23BytesType<'this>>;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field31StringType<'this> = ::std::string::String;
        fn string_required<'this>(&'this self) -> Option<Self::Field31StringType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field32StringType<'this> = ::std::string::String;
        fn string_optional<'this>(&'this self) -> Option<Self::Field32StringType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field33StringType<'this> = ::std::string::String;
        type Field33RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field33StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn enum_required<'this>(&'this self) -> Option<self::_puroro_root::full_coverage2::Enum> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn enum_optional<'this>(&'this self) -> Option<self::_puroro_root::full_coverage2::Enum> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field43RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field51MessageType<'this> = ::std::boxed::Box<
            self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_simple_impl::Submsg,
        >;
        fn submsg_required<'this>(&'this self) -> Option<Self::Field51MessageType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field52MessageType<'this> = ::std::boxed::Box<
            self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_simple_impl::Submsg,
        >;
        fn submsg_optional<'this>(&'this self) -> Option<Self::Field52MessageType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field53MessageType<'this> = ::std::boxed::Box<
            self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_simple_impl::Submsg,
        >;
        type Field53RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field53MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn i64_required<'this>(&'this self) -> Option<i64> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn i64_optional<'this>(&'this self) -> Option<i64> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field103RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn u32_required<'this>(&'this self) -> Option<u32> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn u32_optional<'this>(&'this self) -> Option<u32> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field113RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn u64_required<'this>(&'this self) -> Option<u64> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn u64_optional<'this>(&'this self) -> Option<u64> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field123RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn s32_required<'this>(&'this self) -> Option<i32> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn s32_optional<'this>(&'this self) -> Option<i32> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field133RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn s64_required<'this>(&'this self) -> Option<i64> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn s64_optional<'this>(&'this self) -> Option<i64> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field143RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn fixed32_required<'this>(&'this self) -> Option<u32> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn fixed32_optional<'this>(&'this self) -> Option<u32> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field153RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<u32>;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn fixed64_required<'this>(&'this self) -> Option<u64> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn fixed64_optional<'this>(&'this self) -> Option<u64> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field163RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<u64>;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn sfixed32_required<'this>(&'this self) -> Option<i32> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn sfixed32_optional<'this>(&'this self) -> Option<i32> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field173RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn sfixed64_required<'this>(&'this self) -> Option<i64> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn sfixed64_optional<'this>(&'this self) -> Option<i64> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field183RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<i64>;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn f64_required<'this>(&'this self) -> Option<f64> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn f64_optional<'this>(&'this self) -> Option<f64> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field193RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<f64>;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
    }
    pub struct MsgBuilder<T>(T);

    impl<T> MsgBuilder<T>
    where
        T: MsgTrait,
    {
        pub fn append_i32_required(
            self,
            value: ::std::option::Option<i32>,
        ) -> MsgBuilder<(T, MsgSingleField1)> {
            MsgBuilder((
                self.0,
                MsgSingleField1 {
                    i32_required: value,
                },
            ))
        }

        pub fn append_i32_optional(
            self,
            value: ::std::option::Option<i32>,
        ) -> MsgBuilder<(T, MsgSingleField2)> {
            MsgBuilder((
                self.0,
                MsgSingleField2 {
                    i32_optional: value,
                },
            ))
        }

        pub fn append_i32_repeated<RepeatedType>(
            self,
            value: RepeatedType,
        ) -> MsgBuilder<(T, MsgSingleField3<RepeatedType>)>
        where
            for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i32>,
        {
            MsgBuilder((
                self.0,
                MsgSingleField3 {
                    i32_repeated: value,
                },
            ))
        }

        pub fn append_float_required(
            self,
            value: ::std::option::Option<f32>,
        ) -> MsgBuilder<(T, MsgSingleField11)> {
            MsgBuilder((
                self.0,
                MsgSingleField11 {
                    float_required: value,
                },
            ))
        }

        pub fn append_float_optional(
            self,
            value: ::std::option::Option<f32>,
        ) -> MsgBuilder<(T, MsgSingleField12)> {
            MsgBuilder((
                self.0,
                MsgSingleField12 {
                    float_optional: value,
                },
            ))
        }

        pub fn append_float_repeated<RepeatedType>(
            self,
            value: RepeatedType,
        ) -> MsgBuilder<(T, MsgSingleField13<RepeatedType>)>
        where
            for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a f32>,
        {
            MsgBuilder((
                self.0,
                MsgSingleField13 {
                    float_repeated: value,
                },
            ))
        }

        pub fn append_bytes_required<ScalarType>(
            self,
            value: ::std::option::Option<ScalarType>,
        ) -> MsgBuilder<(T, MsgSingleField21<ScalarType>)>
        where
            ScalarType: ::std::ops::Deref<Target = [u8]>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            MsgBuilder((
                self.0,
                MsgSingleField21 {
                    bytes_required: value,
                },
            ))
        }

        pub fn append_bytes_optional<ScalarType>(
            self,
            value: ::std::option::Option<ScalarType>,
        ) -> MsgBuilder<(T, MsgSingleField22<ScalarType>)>
        where
            ScalarType: ::std::ops::Deref<Target = [u8]>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            MsgBuilder((
                self.0,
                MsgSingleField22 {
                    bytes_optional: value,
                },
            ))
        }

        pub fn append_bytes_repeated<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> MsgBuilder<(T, MsgSingleField23<ScalarType, RepeatedType>)>
        where
            ScalarType: ::std::ops::Deref<Target = [u8]>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
            for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            MsgBuilder((
                self.0,
                MsgSingleField23 {
                    bytes_repeated: value,
                },
            ))
        }

        pub fn append_string_required<ScalarType>(
            self,
            value: ::std::option::Option<ScalarType>,
        ) -> MsgBuilder<(T, MsgSingleField31<ScalarType>)>
        where
            ScalarType: ::std::ops::Deref<Target = str>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            MsgBuilder((
                self.0,
                MsgSingleField31 {
                    string_required: value,
                },
            ))
        }

        pub fn append_string_optional<ScalarType>(
            self,
            value: ::std::option::Option<ScalarType>,
        ) -> MsgBuilder<(T, MsgSingleField32<ScalarType>)>
        where
            ScalarType: ::std::ops::Deref<Target = str>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            MsgBuilder((
                self.0,
                MsgSingleField32 {
                    string_optional: value,
                },
            ))
        }

        pub fn append_string_repeated<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> MsgBuilder<(T, MsgSingleField33<ScalarType, RepeatedType>)>
        where
            ScalarType: ::std::ops::Deref<Target = str>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
            for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            MsgBuilder((
                self.0,
                MsgSingleField33 {
                    string_repeated: value,
                },
            ))
        }

        pub fn append_enum_required(
            self,
            value: ::std::option::Option<self::_puroro_root::full_coverage2::Enum>,
        ) -> MsgBuilder<(T, MsgSingleField41)> {
            MsgBuilder((
                self.0,
                MsgSingleField41 {
                    enum_required: value,
                },
            ))
        }

        pub fn append_enum_optional(
            self,
            value: ::std::option::Option<self::_puroro_root::full_coverage2::Enum>,
        ) -> MsgBuilder<(T, MsgSingleField42)> {
            MsgBuilder((
                self.0,
                MsgSingleField42 {
                    enum_optional: value,
                },
            ))
        }

        pub fn append_enum_repeated<RepeatedType>(
            self,
            value: RepeatedType,
        ) -> MsgBuilder<(T, MsgSingleField43<RepeatedType>)>
        where
            for<'a> &'a RepeatedType:
                ::std::iter::IntoIterator<Item = &'a self::_puroro_root::full_coverage2::Enum>,
        {
            MsgBuilder((
                self.0,
                MsgSingleField43 {
                    enum_repeated: value,
                },
            ))
        }

    pub fn append_submsg_required<ScalarType>(self, value: ::std::option::Option<ScalarType>)
        -> MsgBuilder<(T, MsgSingleField51<ScalarType>)>
where
ScalarType: self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_traits::SubmsgTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
    {
            MsgBuilder((
                self.0,
                MsgSingleField51 {
                    submsg_required: value,
                },
            ))
        }

    pub fn append_submsg_optional<ScalarType>(self, value: ::std::option::Option<ScalarType>)
        -> MsgBuilder<(T, MsgSingleField52<ScalarType>)>
where
ScalarType: self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_traits::SubmsgTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
    {
            MsgBuilder((
                self.0,
                MsgSingleField52 {
                    submsg_optional: value,
                },
            ))
        }

    pub fn append_submsg_repeated<ScalarType, RepeatedType>(self, value: RepeatedType)
        -> MsgBuilder<(T, MsgSingleField53<ScalarType, RepeatedType>)>
where
ScalarType: self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_traits::SubmsgTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
for <'a> &'a RepeatedType: ::std::iter::IntoIterator<
    Item = &'a ScalarType
>,
    {
            MsgBuilder((
                self.0,
                MsgSingleField53 {
                    submsg_repeated: value,
                },
            ))
        }

        pub fn append_i64_required(
            self,
            value: ::std::option::Option<i64>,
        ) -> MsgBuilder<(T, MsgSingleField101)> {
            MsgBuilder((
                self.0,
                MsgSingleField101 {
                    i64_required: value,
                },
            ))
        }

        pub fn append_i64_optional(
            self,
            value: ::std::option::Option<i64>,
        ) -> MsgBuilder<(T, MsgSingleField102)> {
            MsgBuilder((
                self.0,
                MsgSingleField102 {
                    i64_optional: value,
                },
            ))
        }

        pub fn append_i64_repeated<RepeatedType>(
            self,
            value: RepeatedType,
        ) -> MsgBuilder<(T, MsgSingleField103<RepeatedType>)>
        where
            for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i64>,
        {
            MsgBuilder((
                self.0,
                MsgSingleField103 {
                    i64_repeated: value,
                },
            ))
        }

        pub fn append_u32_required(
            self,
            value: ::std::option::Option<u32>,
        ) -> MsgBuilder<(T, MsgSingleField111)> {
            MsgBuilder((
                self.0,
                MsgSingleField111 {
                    u32_required: value,
                },
            ))
        }

        pub fn append_u32_optional(
            self,
            value: ::std::option::Option<u32>,
        ) -> MsgBuilder<(T, MsgSingleField112)> {
            MsgBuilder((
                self.0,
                MsgSingleField112 {
                    u32_optional: value,
                },
            ))
        }

        pub fn append_u32_repeated<RepeatedType>(
            self,
            value: RepeatedType,
        ) -> MsgBuilder<(T, MsgSingleField113<RepeatedType>)>
        where
            for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a u32>,
        {
            MsgBuilder((
                self.0,
                MsgSingleField113 {
                    u32_repeated: value,
                },
            ))
        }

        pub fn append_u64_required(
            self,
            value: ::std::option::Option<u64>,
        ) -> MsgBuilder<(T, MsgSingleField121)> {
            MsgBuilder((
                self.0,
                MsgSingleField121 {
                    u64_required: value,
                },
            ))
        }

        pub fn append_u64_optional(
            self,
            value: ::std::option::Option<u64>,
        ) -> MsgBuilder<(T, MsgSingleField122)> {
            MsgBuilder((
                self.0,
                MsgSingleField122 {
                    u64_optional: value,
                },
            ))
        }

        pub fn append_u64_repeated<RepeatedType>(
            self,
            value: RepeatedType,
        ) -> MsgBuilder<(T, MsgSingleField123<RepeatedType>)>
        where
            for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a u64>,
        {
            MsgBuilder((
                self.0,
                MsgSingleField123 {
                    u64_repeated: value,
                },
            ))
        }

        pub fn append_s32_required(
            self,
            value: ::std::option::Option<i32>,
        ) -> MsgBuilder<(T, MsgSingleField131)> {
            MsgBuilder((
                self.0,
                MsgSingleField131 {
                    s32_required: value,
                },
            ))
        }

        pub fn append_s32_optional(
            self,
            value: ::std::option::Option<i32>,
        ) -> MsgBuilder<(T, MsgSingleField132)> {
            MsgBuilder((
                self.0,
                MsgSingleField132 {
                    s32_optional: value,
                },
            ))
        }

        pub fn append_s32_repeated<RepeatedType>(
            self,
            value: RepeatedType,
        ) -> MsgBuilder<(T, MsgSingleField133<RepeatedType>)>
        where
            for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i32>,
        {
            MsgBuilder((
                self.0,
                MsgSingleField133 {
                    s32_repeated: value,
                },
            ))
        }

        pub fn append_s64_required(
            self,
            value: ::std::option::Option<i64>,
        ) -> MsgBuilder<(T, MsgSingleField141)> {
            MsgBuilder((
                self.0,
                MsgSingleField141 {
                    s64_required: value,
                },
            ))
        }

        pub fn append_s64_optional(
            self,
            value: ::std::option::Option<i64>,
        ) -> MsgBuilder<(T, MsgSingleField142)> {
            MsgBuilder((
                self.0,
                MsgSingleField142 {
                    s64_optional: value,
                },
            ))
        }

        pub fn append_s64_repeated<RepeatedType>(
            self,
            value: RepeatedType,
        ) -> MsgBuilder<(T, MsgSingleField143<RepeatedType>)>
        where
            for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i64>,
        {
            MsgBuilder((
                self.0,
                MsgSingleField143 {
                    s64_repeated: value,
                },
            ))
        }

        pub fn append_fixed32_required(
            self,
            value: ::std::option::Option<u32>,
        ) -> MsgBuilder<(T, MsgSingleField151)> {
            MsgBuilder((
                self.0,
                MsgSingleField151 {
                    fixed32_required: value,
                },
            ))
        }

        pub fn append_fixed32_optional(
            self,
            value: ::std::option::Option<u32>,
        ) -> MsgBuilder<(T, MsgSingleField152)> {
            MsgBuilder((
                self.0,
                MsgSingleField152 {
                    fixed32_optional: value,
                },
            ))
        }

        pub fn append_fixed32_repeated<RepeatedType>(
            self,
            value: RepeatedType,
        ) -> MsgBuilder<(T, MsgSingleField153<RepeatedType>)>
        where
            for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a u32>,
        {
            MsgBuilder((
                self.0,
                MsgSingleField153 {
                    fixed32_repeated: value,
                },
            ))
        }

        pub fn append_fixed64_required(
            self,
            value: ::std::option::Option<u64>,
        ) -> MsgBuilder<(T, MsgSingleField161)> {
            MsgBuilder((
                self.0,
                MsgSingleField161 {
                    fixed64_required: value,
                },
            ))
        }

        pub fn append_fixed64_optional(
            self,
            value: ::std::option::Option<u64>,
        ) -> MsgBuilder<(T, MsgSingleField162)> {
            MsgBuilder((
                self.0,
                MsgSingleField162 {
                    fixed64_optional: value,
                },
            ))
        }

        pub fn append_fixed64_repeated<RepeatedType>(
            self,
            value: RepeatedType,
        ) -> MsgBuilder<(T, MsgSingleField163<RepeatedType>)>
        where
            for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a u64>,
        {
            MsgBuilder((
                self.0,
                MsgSingleField163 {
                    fixed64_repeated: value,
                },
            ))
        }

        pub fn append_sfixed32_required(
            self,
            value: ::std::option::Option<i32>,
        ) -> MsgBuilder<(T, MsgSingleField171)> {
            MsgBuilder((
                self.0,
                MsgSingleField171 {
                    sfixed32_required: value,
                },
            ))
        }

        pub fn append_sfixed32_optional(
            self,
            value: ::std::option::Option<i32>,
        ) -> MsgBuilder<(T, MsgSingleField172)> {
            MsgBuilder((
                self.0,
                MsgSingleField172 {
                    sfixed32_optional: value,
                },
            ))
        }

        pub fn append_sfixed32_repeated<RepeatedType>(
            self,
            value: RepeatedType,
        ) -> MsgBuilder<(T, MsgSingleField173<RepeatedType>)>
        where
            for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i32>,
        {
            MsgBuilder((
                self.0,
                MsgSingleField173 {
                    sfixed32_repeated: value,
                },
            ))
        }

        pub fn append_sfixed64_required(
            self,
            value: ::std::option::Option<i64>,
        ) -> MsgBuilder<(T, MsgSingleField181)> {
            MsgBuilder((
                self.0,
                MsgSingleField181 {
                    sfixed64_required: value,
                },
            ))
        }

        pub fn append_sfixed64_optional(
            self,
            value: ::std::option::Option<i64>,
        ) -> MsgBuilder<(T, MsgSingleField182)> {
            MsgBuilder((
                self.0,
                MsgSingleField182 {
                    sfixed64_optional: value,
                },
            ))
        }

        pub fn append_sfixed64_repeated<RepeatedType>(
            self,
            value: RepeatedType,
        ) -> MsgBuilder<(T, MsgSingleField183<RepeatedType>)>
        where
            for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i64>,
        {
            MsgBuilder((
                self.0,
                MsgSingleField183 {
                    sfixed64_repeated: value,
                },
            ))
        }

        pub fn append_f64_required(
            self,
            value: ::std::option::Option<f64>,
        ) -> MsgBuilder<(T, MsgSingleField191)> {
            MsgBuilder((
                self.0,
                MsgSingleField191 {
                    f64_required: value,
                },
            ))
        }

        pub fn append_f64_optional(
            self,
            value: ::std::option::Option<f64>,
        ) -> MsgBuilder<(T, MsgSingleField192)> {
            MsgBuilder((
                self.0,
                MsgSingleField192 {
                    f64_optional: value,
                },
            ))
        }

        pub fn append_f64_repeated<RepeatedType>(
            self,
            value: RepeatedType,
        ) -> MsgBuilder<(T, MsgSingleField193<RepeatedType>)>
        where
            for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a f64>,
        {
            MsgBuilder((
                self.0,
                MsgSingleField193 {
                    f64_repeated: value,
                },
            ))
        }

        pub fn build(self) -> T {
            self.0
        }
    }

    impl MsgBuilder<()> {
        pub fn new() -> Self {
            Self(())
        }
    }
}
pub use _puroro_traits::*;
pub mod _puroro_traits {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }

    pub trait MsgTrait {
        fn i32_required<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::default::Default::default()
        }
        fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::default::Default::default()
        }
        type Field3RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = i32>
        where
            Self: 'this;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this>;
        fn float_required<'this>(&'this self) -> ::std::option::Option<f32> {
            ::std::default::Default::default()
        }
        fn float_optional<'this>(&'this self) -> ::std::option::Option<f32> {
            ::std::default::Default::default()
        }
        type Field13RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = f32>
        where
            Self: 'this;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this>;
        type Field21BytesType<'this>: ::std::ops::Deref<Target = [u8]>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
        where
            Self: 'this;
        fn bytes_required<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field21BytesType<'this>> {
            ::std::default::Default::default()
        }
        type Field22BytesType<'this>: ::std::ops::Deref<Target = [u8]>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
        where
            Self: 'this;
        fn bytes_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field22BytesType<'this>> {
            ::std::default::Default::default()
        }
        type Field23BytesType<'this>: ::std::ops::Deref<Target = [u8]>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
        where
            Self: 'this;
        type Field23RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field23BytesType<'this>>
        where
            Self: 'this;
        fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this>;
        type Field31StringType<'this>: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
        where
            Self: 'this;
        fn string_required<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field31StringType<'this>> {
            ::std::default::Default::default()
        }
        type Field32StringType<'this>: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
        where
            Self: 'this;
        fn string_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field32StringType<'this>> {
            ::std::default::Default::default()
        }
        type Field33StringType<'this>: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
        where
            Self: 'this;
        type Field33RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field33StringType<'this>>
        where
            Self: 'this;
        fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this>;
        fn enum_required<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::full_coverage2::Enum> {
            ::std::default::Default::default()
        }
        fn enum_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::full_coverage2::Enum> {
            ::std::default::Default::default()
        }
        type Field43RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = self::_puroro_root::full_coverage2::Enum>
        where
            Self: 'this;
        fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this>;
        type Field51MessageType<'this>:
            self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_traits::SubmsgTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug
            where Self: 'this;
        fn submsg_required<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field51MessageType<'this>> {
            ::std::default::Default::default()
        }
        type Field52MessageType<'this>:
            self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_traits::SubmsgTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug
            where Self: 'this;
        fn submsg_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field52MessageType<'this>> {
            ::std::default::Default::default()
        }
        type Field53MessageType<'this>:
            self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_traits::SubmsgTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug
            where Self: 'this;
        type Field53RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field53MessageType<'this>>
        where
            Self: 'this;
        fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this>;
        fn i64_required<'this>(&'this self) -> ::std::option::Option<i64> {
            ::std::default::Default::default()
        }
        fn i64_optional<'this>(&'this self) -> ::std::option::Option<i64> {
            ::std::default::Default::default()
        }
        type Field103RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = i64>
        where
            Self: 'this;
        fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this>;
        fn u32_required<'this>(&'this self) -> ::std::option::Option<u32> {
            ::std::default::Default::default()
        }
        fn u32_optional<'this>(&'this self) -> ::std::option::Option<u32> {
            ::std::default::Default::default()
        }
        type Field113RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = u32>
        where
            Self: 'this;
        fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this>;
        fn u64_required<'this>(&'this self) -> ::std::option::Option<u64> {
            ::std::default::Default::default()
        }
        fn u64_optional<'this>(&'this self) -> ::std::option::Option<u64> {
            ::std::default::Default::default()
        }
        type Field123RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = u64>
        where
            Self: 'this;
        fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this>;
        fn s32_required<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::default::Default::default()
        }
        fn s32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::default::Default::default()
        }
        type Field133RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = i32>
        where
            Self: 'this;
        fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this>;
        fn s64_required<'this>(&'this self) -> ::std::option::Option<i64> {
            ::std::default::Default::default()
        }
        fn s64_optional<'this>(&'this self) -> ::std::option::Option<i64> {
            ::std::default::Default::default()
        }
        type Field143RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = i64>
        where
            Self: 'this;
        fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this>;
        fn fixed32_required<'this>(&'this self) -> ::std::option::Option<u32> {
            ::std::default::Default::default()
        }
        fn fixed32_optional<'this>(&'this self) -> ::std::option::Option<u32> {
            ::std::default::Default::default()
        }
        type Field153RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = u32>
        where
            Self: 'this;
        fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this>;
        fn fixed64_required<'this>(&'this self) -> ::std::option::Option<u64> {
            ::std::default::Default::default()
        }
        fn fixed64_optional<'this>(&'this self) -> ::std::option::Option<u64> {
            ::std::default::Default::default()
        }
        type Field163RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = u64>
        where
            Self: 'this;
        fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this>;
        fn sfixed32_required<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::default::Default::default()
        }
        fn sfixed32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::default::Default::default()
        }
        type Field173RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = i32>
        where
            Self: 'this;
        fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this>;
        fn sfixed64_required<'this>(&'this self) -> ::std::option::Option<i64> {
            ::std::default::Default::default()
        }
        fn sfixed64_optional<'this>(&'this self) -> ::std::option::Option<i64> {
            ::std::default::Default::default()
        }
        type Field183RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = i64>
        where
            Self: 'this;
        fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this>;
        fn f64_required<'this>(&'this self) -> ::std::option::Option<f64> {
            ::std::default::Default::default()
        }
        fn f64_optional<'this>(&'this self) -> ::std::option::Option<f64> {
            ::std::default::Default::default()
        }
        type Field193RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = f64>
        where
            Self: 'this;
        fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this>;
    }

    macro_rules! msg_delegate {
        ($ty:ty) => {
            fn i32_required<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).i32_required()
            }
            fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).i32_optional()
            }
            type Field3RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field3RepeatedType<'this>;
            fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
                (**self).i32_repeated()
            }
            fn float_required<'this>(&'this self) -> ::std::option::Option<f32> {
                (**self).float_required()
            }
            fn float_optional<'this>(&'this self) -> ::std::option::Option<f32> {
                (**self).float_optional()
            }
            type Field13RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field13RepeatedType<'this>;
            fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
                (**self).float_repeated()
            }
            type Field21BytesType<'this>
            where
                Self: 'this,
            = <$ty>::Field21BytesType<'this>;
            fn bytes_required<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field21BytesType<'this>> {
                (**self).bytes_required()
            }
            type Field22BytesType<'this>
            where
                Self: 'this,
            = <$ty>::Field22BytesType<'this>;
            fn bytes_optional<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field22BytesType<'this>> {
                (**self).bytes_optional()
            }
            type Field23BytesType<'this>
            where
                Self: 'this,
            = <$ty>::Field23BytesType<'this>;
            type Field23RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field23RepeatedType<'this>;
            fn bytes_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
                (**self).bytes_repeated()
            }
            type Field31StringType<'this>
            where
                Self: 'this,
            = <$ty>::Field31StringType<'this>;
            fn string_required<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field31StringType<'this>> {
                (**self).string_required()
            }
            type Field32StringType<'this>
            where
                Self: 'this,
            = <$ty>::Field32StringType<'this>;
            fn string_optional<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field32StringType<'this>> {
                (**self).string_optional()
            }
            type Field33StringType<'this>
            where
                Self: 'this,
            = <$ty>::Field33StringType<'this>;
            type Field33RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field33RepeatedType<'this>;
            fn string_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
                (**self).string_repeated()
            }
            fn enum_required<'this>(
                &'this self,
            ) -> ::std::option::Option<self::_puroro_root::full_coverage2::Enum> {
                (**self).enum_required()
            }
            fn enum_optional<'this>(
                &'this self,
            ) -> ::std::option::Option<self::_puroro_root::full_coverage2::Enum> {
                (**self).enum_optional()
            }
            type Field43RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field43RepeatedType<'this>;
            fn enum_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
                (**self).enum_repeated()
            }
            type Field51MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field51MessageType<'this>;
            fn submsg_required<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field51MessageType<'this>> {
                (**self).submsg_required()
            }
            type Field52MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field52MessageType<'this>;
            fn submsg_optional<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field52MessageType<'this>> {
                (**self).submsg_optional()
            }
            type Field53MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field53MessageType<'this>;
            type Field53RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field53RepeatedType<'this>;
            fn submsg_repeated<'this>(&'this self) -> Self::Field53RepeatedType<'this> {
                (**self).submsg_repeated()
            }
            fn i64_required<'this>(&'this self) -> ::std::option::Option<i64> {
                (**self).i64_required()
            }
            fn i64_optional<'this>(&'this self) -> ::std::option::Option<i64> {
                (**self).i64_optional()
            }
            type Field103RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field103RepeatedType<'this>;
            fn i64_repeated<'this>(&'this self) -> Self::Field103RepeatedType<'this> {
                (**self).i64_repeated()
            }
            fn u32_required<'this>(&'this self) -> ::std::option::Option<u32> {
                (**self).u32_required()
            }
            fn u32_optional<'this>(&'this self) -> ::std::option::Option<u32> {
                (**self).u32_optional()
            }
            type Field113RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field113RepeatedType<'this>;
            fn u32_repeated<'this>(&'this self) -> Self::Field113RepeatedType<'this> {
                (**self).u32_repeated()
            }
            fn u64_required<'this>(&'this self) -> ::std::option::Option<u64> {
                (**self).u64_required()
            }
            fn u64_optional<'this>(&'this self) -> ::std::option::Option<u64> {
                (**self).u64_optional()
            }
            type Field123RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field123RepeatedType<'this>;
            fn u64_repeated<'this>(&'this self) -> Self::Field123RepeatedType<'this> {
                (**self).u64_repeated()
            }
            fn s32_required<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).s32_required()
            }
            fn s32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).s32_optional()
            }
            type Field133RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field133RepeatedType<'this>;
            fn s32_repeated<'this>(&'this self) -> Self::Field133RepeatedType<'this> {
                (**self).s32_repeated()
            }
            fn s64_required<'this>(&'this self) -> ::std::option::Option<i64> {
                (**self).s64_required()
            }
            fn s64_optional<'this>(&'this self) -> ::std::option::Option<i64> {
                (**self).s64_optional()
            }
            type Field143RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field143RepeatedType<'this>;
            fn s64_repeated<'this>(&'this self) -> Self::Field143RepeatedType<'this> {
                (**self).s64_repeated()
            }
            fn fixed32_required<'this>(&'this self) -> ::std::option::Option<u32> {
                (**self).fixed32_required()
            }
            fn fixed32_optional<'this>(&'this self) -> ::std::option::Option<u32> {
                (**self).fixed32_optional()
            }
            type Field153RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field153RepeatedType<'this>;
            fn fixed32_repeated<'this>(&'this self) -> Self::Field153RepeatedType<'this> {
                (**self).fixed32_repeated()
            }
            fn fixed64_required<'this>(&'this self) -> ::std::option::Option<u64> {
                (**self).fixed64_required()
            }
            fn fixed64_optional<'this>(&'this self) -> ::std::option::Option<u64> {
                (**self).fixed64_optional()
            }
            type Field163RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field163RepeatedType<'this>;
            fn fixed64_repeated<'this>(&'this self) -> Self::Field163RepeatedType<'this> {
                (**self).fixed64_repeated()
            }
            fn sfixed32_required<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).sfixed32_required()
            }
            fn sfixed32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).sfixed32_optional()
            }
            type Field173RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field173RepeatedType<'this>;
            fn sfixed32_repeated<'this>(&'this self) -> Self::Field173RepeatedType<'this> {
                (**self).sfixed32_repeated()
            }
            fn sfixed64_required<'this>(&'this self) -> ::std::option::Option<i64> {
                (**self).sfixed64_required()
            }
            fn sfixed64_optional<'this>(&'this self) -> ::std::option::Option<i64> {
                (**self).sfixed64_optional()
            }
            type Field183RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field183RepeatedType<'this>;
            fn sfixed64_repeated<'this>(&'this self) -> Self::Field183RepeatedType<'this> {
                (**self).sfixed64_repeated()
            }
            fn f64_required<'this>(&'this self) -> ::std::option::Option<f64> {
                (**self).f64_required()
            }
            fn f64_optional<'this>(&'this self) -> ::std::option::Option<f64> {
                (**self).f64_optional()
            }
            type Field193RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field193RepeatedType<'this>;
            fn f64_repeated<'this>(&'this self) -> Self::Field193RepeatedType<'this> {
                (**self).f64_repeated()
            }
        };
    }

    impl<T> MsgTrait for &'_ T
    where
        T: MsgTrait,
    {
        msg_delegate!(T);
    }

    impl<T> MsgTrait for &'_ mut T
    where
        T: MsgTrait,
    {
        msg_delegate!(T);
    }

    impl<T> MsgTrait for ::std::boxed::Box<T>
    where
        T: MsgTrait,
    {
        msg_delegate!(T);
    }
}
#[derive(::std::fmt::Debug, ::std::clone::Clone, ::std::cmp::PartialEq)]
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
pub use _puroro_nested::*;
pub mod _puroro_nested {
    pub mod msg {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }

        pub use _puroro_simple_impl::Submsg;
        pub mod _puroro_simple_impl {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }
            #[derive(
                ::std::clone::Clone,
                ::std::default::Default,
                ::std::cmp::PartialEq,
                ::std::fmt::Debug,
            )]
            pub struct Submsg {
                pub i32_required: ::std::option::Option<i32>,
                pub i64_required: ::std::option::Option<i64>,
            }
            impl ::puroro::Message<Submsg> for Submsg {}

            impl super::_puroro_traits::SubmsgTrait for Submsg {
                fn i32_required<'this>(&'this self) -> Option<i32> {
                    Clone::clone(&self.i32_required)
                }
                fn i64_required<'this>(&'this self) -> Option<i64> {
                    Clone::clone(&self.i64_required)
                }
            }

            impl ::puroro::MessageRepresentativeImpl for Submsg {
                fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
                    use ::puroro::once_cell::sync::Lazy;
                    static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 2]> =
                        Lazy::new(|| {
                            [
                                {
                                    let init = ::puroro::internal::FieldDescriptorInitializer {
                                        name: "i32_required",
                                        number: 1,
                                        lazy_containing_type: Lazy::new(|| {
                                            <Submsg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                        }),
                                    };
                                    ::puroro::internal::init_field_descriptor(init)
                                },
                                {
                                    let init = ::puroro::internal::FieldDescriptorInitializer {
                                        name: "i64_required",
                                        number: 101,
                                        lazy_containing_type: Lazy::new(|| {
                                            <Submsg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                        }),
                                    };
                                    ::puroro::internal::init_field_descriptor(init)
                                },
                            ]
                        });
                    static LAZY_DESCRIPTOR: Lazy<::puroro::desc::MessageDescriptor> =
                        Lazy::new(|| {
                            let init = ::puroro::internal::MessageDescriptorInitializer {
                                name: "Submsg",
                                lazy_fields: Lazy::new(|| {
                                    Lazy::force(&LAZY_FIELD_DESCRIPTOR_ARRAY).as_ref()
                                }),
                            };
                            ::puroro::internal::init_message_descriptor(init)
                        });
                    Lazy::force(&LAZY_DESCRIPTOR)
                }
            }

            impl ::puroro::internal::DeserializableMessageFromBytesIterator for Submsg {
                fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    ::puroro::internal::de::from_iter::deser_from_iter(self, iter)
                }
            }

            impl ::puroro::internal::de::DeserFieldsFromBytesIter for Submsg {
                fn deser_field<I>(
                    &mut self,
                    field_number: i32,
                    data: ::puroro::types::FieldData<
                        &mut ::puroro::internal::de::from_iter::ScopedIter<I>,
                    >,
                ) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    use ::puroro::internal::impls::simple::de::DeserFieldFromBytesIter;
                    match field_number {
                        1 => DeserFieldFromBytesIter::<
                            ::puroro::tags::Required,
                            ::puroro::tags::Int32,
                        >::deser_field(&mut self.i32_required, data),
                        101 => DeserFieldFromBytesIter::<
                            ::puroro::tags::Required,
                            ::puroro::tags::Int64,
                        >::deser_field(&mut self.i64_required, data),

                        _ => unimplemented!("TODO: This case should be handled properly..."),
                    }
                }
            }

            impl ::puroro::internal::SerializableMessageToIoWrite for Submsg {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Required, ::puroro::tags::Int32
                >::ser_field(&self.i32_required, 1, out)?;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Required, ::puroro::tags::Int64
                >::ser_field(&self.i64_required, 101, out)?;

                    ::std::result::Result::Ok(())
                }
            }
        }

        pub use _puroro_impls::*;
        pub mod _puroro_impls {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }
            use super::_puroro_traits::*;
            impl SubmsgTrait for () {}
            impl<T, U> SubmsgTrait for (T, U)
            where
                T: SubmsgTrait,
                U: SubmsgTrait,
            {
                fn i32_required<'this>(&'this self) -> Option<i32> {
                    <U as SubmsgTrait>::i32_required(&self.1)
                        .or_else(|| <T as SubmsgTrait>::i32_required(&self.0))
                }
                fn i64_required<'this>(&'this self) -> Option<i64> {
                    <U as SubmsgTrait>::i64_required(&self.1)
                        .or_else(|| <T as SubmsgTrait>::i64_required(&self.0))
                }
            }
            impl<T, U> SubmsgTrait for ::puroro::Either<T, U>
            where
                T: SubmsgTrait,
                U: SubmsgTrait,
            {
                fn i32_required<'this>(&'this self) -> Option<i32> {
                    self.as_ref().either(
                        |t| <T as SubmsgTrait>::i32_required(t),
                        |u| <U as SubmsgTrait>::i32_required(u),
                    )
                }
                fn i64_required<'this>(&'this self) -> Option<i64> {
                    self.as_ref().either(
                        |t| <T as SubmsgTrait>::i64_required(t),
                        |u| <U as SubmsgTrait>::i64_required(u),
                    )
                }
            }
            impl<T> SubmsgTrait for ::std::option::Option<T>
            where
                T: SubmsgTrait,
            {
                fn i32_required<'this>(&'this self) -> ::std::option::Option<i32> {
                    self.as_ref().and_then(|msg| msg.i32_required())
                }
                fn i64_required<'this>(&'this self) -> ::std::option::Option<i64> {
                    self.as_ref().and_then(|msg| msg.i64_required())
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

            pub struct SubmsgSingleField1 {
                pub i32_required: ::std::option::Option<i32>,
            }

            impl ::puroro::Message<super::Submsg> for SubmsgSingleField1 {}

            impl super::_puroro_traits::SubmsgTrait for SubmsgSingleField1 {
                fn i32_required<'this>(&'this self) -> Option<i32> {
                    Clone::clone(&self.i32_required)
                }
            }

            impl ::puroro::internal::SerializableMessageToIoWrite for SubmsgSingleField1 {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Required, ::puroro::tags::Int32
                >::ser_field::
                <(), _, _>
                (
                    &self.i32_required,
                    1,
                    out
                )?;
                    ::std::result::Result::Ok(())
                }
            }

            impl ::std::convert::From<::std::option::Option<i32>> for SubmsgSingleField1 {
                fn from(value: ::std::option::Option<i32>) -> Self {
                    Self {
                        i32_required: value,
                    }
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

            pub struct SubmsgSingleField101 {
                pub i64_required: ::std::option::Option<i64>,
            }

            impl ::puroro::Message<super::Submsg> for SubmsgSingleField101 {}

            impl super::_puroro_traits::SubmsgTrait for SubmsgSingleField101 {
                fn i64_required<'this>(&'this self) -> Option<i64> {
                    Clone::clone(&self.i64_required)
                }
            }

            impl ::puroro::internal::SerializableMessageToIoWrite for SubmsgSingleField101 {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Required, ::puroro::tags::Int64
                >::ser_field::
                <(), _, _>
                (
                    &self.i64_required,
                    101,
                    out
                )?;
                    ::std::result::Result::Ok(())
                }
            }

            impl ::std::convert::From<::std::option::Option<i64>> for SubmsgSingleField101 {
                fn from(value: ::std::option::Option<i64>) -> Self {
                    Self {
                        i64_required: value,
                    }
                }
            }
            #[derive(
                ::std::clone::Clone,
                ::std::default::Default,
                ::std::cmp::PartialEq,
                ::std::fmt::Debug,
            )]
            pub struct SubmsgSimpleByValue {}
            impl ::puroro::Message<super::Submsg> for SubmsgSimpleByValue {}

            impl SubmsgTrait for SubmsgSimpleByValue {
                fn i32_required<'this>(&'this self) -> Option<i32> {
                    unimplemented!("Please don't use / instantiate this struct!!")
                }
                fn i64_required<'this>(&'this self) -> Option<i64> {
                    unimplemented!("Please don't use / instantiate this struct!!")
                }
            }
            pub struct SubmsgBuilder<T>(T);

            impl<T> SubmsgBuilder<T>
            where
                T: SubmsgTrait,
            {
                pub fn append_i32_required(
                    self,
                    value: ::std::option::Option<i32>,
                ) -> SubmsgBuilder<(T, SubmsgSingleField1)> {
                    SubmsgBuilder((
                        self.0,
                        SubmsgSingleField1 {
                            i32_required: value,
                        },
                    ))
                }

                pub fn append_i64_required(
                    self,
                    value: ::std::option::Option<i64>,
                ) -> SubmsgBuilder<(T, SubmsgSingleField101)> {
                    SubmsgBuilder((
                        self.0,
                        SubmsgSingleField101 {
                            i64_required: value,
                        },
                    ))
                }

                pub fn build(self) -> T {
                    self.0
                }
            }

            impl SubmsgBuilder<()> {
                pub fn new() -> Self {
                    Self(())
                }
            }
        }
        pub use _puroro_traits::*;
        pub mod _puroro_traits {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }

            pub trait SubmsgTrait {
                fn i32_required<'this>(&'this self) -> ::std::option::Option<i32> {
                    ::std::default::Default::default()
                }
                fn i64_required<'this>(&'this self) -> ::std::option::Option<i64> {
                    ::std::default::Default::default()
                }
            }

            macro_rules! submsg_delegate {
                ($ty:ty) => {
                    fn i32_required<'this>(&'this self) -> ::std::option::Option<i32> {
                        (**self).i32_required()
                    }
                    fn i64_required<'this>(&'this self) -> ::std::option::Option<i64> {
                        (**self).i64_required()
                    }
                };
            }

            impl<T> SubmsgTrait for &'_ T
            where
                T: SubmsgTrait,
            {
                submsg_delegate!(T);
            }

            impl<T> SubmsgTrait for &'_ mut T
            where
                T: SubmsgTrait,
            {
                submsg_delegate!(T);
            }

            impl<T> SubmsgTrait for ::std::boxed::Box<T>
            where
                T: SubmsgTrait,
            {
                submsg_delegate!(T);
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
