// A generated source code by puroro library
// package ser_tests2

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub use _puroro_impls::MsgSimple as Msg;
pub mod _puroro_impls {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }

#[derive(Clone, Default, PartialEq, Debug)]
pub struct MsgSimple {pub i32_optional: ::std::option::Option<i32>,pub i32_repeated: ::std::vec::Vec<i32>,pub float_optional: ::std::option::Option<f32>,pub float_repeated: ::std::vec::Vec<f32>,pub string_optional: ::std::option::Option<::std::string::String>,pub string_repeated: ::std::vec::Vec<::std::string::String>,pub submsg_optional: ::std::option::Option<::std::boxed::Box<self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple>>,pub submsg_repeated: ::std::vec::Vec<self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple>,pub enum_optional: ::std::option::Option<self::_puroro_root::ser_tests2::Enum>,pub enum_repeated: ::std::vec::Vec<self::_puroro_root::ser_tests2::Enum>,pub very_large_field_number: ::std::option::Option<i32>,
}
impl ::puroro::Message for MsgSimple {}

impl super::_puroro_traits::MsgTrait for MsgSimple {
    fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
        ::std::clone::Clone::clone(&self.i32_optional)
    }
    type Field2RepeatedType<'this> = ::puroro_internal::impls::simple::VecWrapper<'this, i32>;

    fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
        ::puroro_internal::impls::simple::VecWrapper::new(&self.i32_repeated)
    }
    fn float_optional<'this>(&'this self) -> ::std::option::Option<f32> {
        ::std::clone::Clone::clone(&self.float_optional)
    }
    type Field4RepeatedType<'this> = ::puroro_internal::impls::simple::VecWrapper<'this, f32>;

    fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
        ::puroro_internal::impls::simple::VecWrapper::new(&self.float_repeated)
    }
    fn string_optional<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
        self.string_optional.as_ref().map(|v| ::std::borrow::Cow::Borrowed(v.as_ref()))
    }
    type Field6RepeatedType<'this> = ::puroro_internal::impls::simple::VecCowWrapper<'this, str>;

    fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
        ::puroro_internal::impls::simple::VecCowWrapper::new(&self.string_repeated)
    }
    type Field7MessageType<'this> = self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple;
    fn submsg_optional<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field7MessageType<'this>>> {
        self.submsg_optional.as_ref().map(|boxed| ::std::borrow::Cow::Borrowed(boxed.as_ref()))
    }
    type Field8MessageType<'this> = self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple;
    type Field8RepeatedType<'this> = ::puroro_internal::impls::simple::VecCowWrapper<'this, self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple>;

    fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
        ::puroro_internal::impls::simple::VecCowWrapper::new(&self.submsg_repeated)
    }
    fn enum_optional<'this>(&'this self) -> ::std::option::Option<self::_puroro_root::ser_tests2::Enum> {
        ::std::clone::Clone::clone(&self.enum_optional)
    }
    type Field10RepeatedType<'this> = ::puroro_internal::impls::simple::VecWrapper<'this, self::_puroro_root::ser_tests2::Enum>;

    fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
        ::puroro_internal::impls::simple::VecWrapper::new(&self.enum_repeated)
    }
    fn very_large_field_number<'this>(&'this self) -> ::std::option::Option<i32> {
        ::std::clone::Clone::clone(&self.very_large_field_number)
    }
}

impl ::puroro::DeserFromBytesIter for MsgSimple {
    fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
    where
        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>
    {
        ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
    }
}

impl ::puroro_internal::de::DeserFieldsFromBytesIter for MsgSimple {
    fn deser_field<I>(
        &mut self,
        field_number: i32,
        data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
    ) -> ::puroro::Result<()>
    where
        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>
    {
        match field_number {
            1 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Int32
            >::deser_field(&mut self.i32_optional, data),
            2 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Int32
            >::deser_field(&mut self.i32_repeated, data),
            3 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Float
            >::deser_field(&mut self.float_optional, data),
            4 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Float
            >::deser_field(&mut self.float_repeated, data),
            5 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.string_optional, data),
            6 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::String
            >::deser_field(&mut self.string_repeated, data),
            7 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Message<self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple>
            >::deser_field(&mut self.submsg_optional, data),
            8 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple>
            >::deser_field(&mut self.submsg_repeated, data),
            9 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Enum2<self::_puroro_root::ser_tests2::Enum>
            >::deser_field(&mut self.enum_optional, data),
            10 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Enum2<self::_puroro_root::ser_tests2::Enum>
            >::deser_field(&mut self.enum_repeated, data),
            536870911 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Int32
            >::deser_field(&mut self.very_large_field_number, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
    }
}

impl ::puroro::SerToIoWrite for MsgSimple {
    fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
    where
        W: ::std::io::Write
    {
        ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
            ::puroro::tags::Optional, ::puroro::tags::Int32
        >::ser_field(&self.i32_optional, 1, out)?;
        ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::Int32
        >::ser_field(&self.i32_repeated, 2, out)?;
        ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
            ::puroro::tags::Optional, ::puroro::tags::Float
        >::ser_field(&self.float_optional, 3, out)?;
        ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::Float
        >::ser_field(&self.float_repeated, 4, out)?;
        ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
            ::puroro::tags::Optional, ::puroro::tags::String
        >::ser_field(&self.string_optional, 5, out)?;
        ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::String
        >::ser_field(&self.string_repeated, 6, out)?;
        ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
            ::puroro::tags::Optional, ::puroro::tags::Message<self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple>
        >::ser_field(&self.submsg_optional, 7, out)?;
        ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple>
        >::ser_field(&self.submsg_repeated, 8, out)?;
        ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
            ::puroro::tags::Optional, ::puroro::tags::Enum2<self::_puroro_root::ser_tests2::Enum>
        >::ser_field(&self.enum_optional, 9, out)?;
        ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::Enum2<self::_puroro_root::ser_tests2::Enum>
        >::ser_field(&self.enum_repeated, 10, out)?;
        ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
            ::puroro::tags::Optional, ::puroro::tags::Int32
        >::ser_field(&self.very_large_field_number, 536870911, out)?;
        ::std::result::Result::Ok(())
    }
}

#[derive(Clone, Default, PartialEq, Debug)]
pub struct MsgEmpty;

impl ::puroro::Message for MsgEmpty {}

impl super::_puroro_traits::MsgTrait for MsgEmpty {
    fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
        None
    }
    type Field2RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
    fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    fn float_optional<'this>(&'this self) -> ::std::option::Option<f32> {
        None
    }
    type Field4RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
    fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    fn string_optional<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
        None
    }
    type Field6RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<::std::borrow::Cow<'this, str>>;
    fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    type Field7MessageType<'this> = self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
    fn submsg_optional<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field7MessageType<'this>>> {
        None
    }
    type Field8MessageType<'this> = self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
    type Field8RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<::std::borrow::Cow<'this, Self::Field8MessageType<'this>>>;
    fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    fn enum_optional<'this>(&'this self) -> ::std::option::Option<self::_puroro_root::ser_tests2::Enum> {
        None
    }
    type Field10RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<self::_puroro_root::ser_tests2::Enum>;
    fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    fn very_large_field_number<'this>(&'this self) -> ::std::option::Option<i32> {
        None
    }
}

impl ::puroro::SerToIoWrite for MsgEmpty {
    fn ser<W>(&self, _out: &mut W) -> ::puroro::Result<()>
    where
        W: ::std::io::Write
    {
        ::std::result::Result::Ok(())
    }
}

#[derive(Clone, PartialEq, Debug)]
struct MsgSingleField1<FieldType> {
    field: FieldType,
}

impl<FieldType> ::puroro::Message for MsgSingleField1<FieldType> {}

impl<FieldType> super::_puroro_traits::MsgTrait for MsgSingleField1<FieldType>
where
    FieldType: Clone, PartialEq, Debug,
{
    fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
        None
    }
    type Field2RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
    fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    fn float_optional<'this>(&'this self) -> ::std::option::Option<f32> {
        None
    }
    type Field4RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
    fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    fn string_optional<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
        None
    }
    type Field6RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<::std::borrow::Cow<'this, str>>;
    fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    type Field7MessageType<'this> = self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
    fn submsg_optional<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field7MessageType<'this>>> {
        None
    }
    type Field8MessageType<'this> = self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
    type Field8RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<::std::borrow::Cow<'this, Self::Field8MessageType<'this>>>;
    fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    fn enum_optional<'this>(&'this self) -> ::std::option::Option<self::_puroro_root::ser_tests2::Enum> {
        None
    }
    type Field10RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<self::_puroro_root::ser_tests2::Enum>;
    fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    fn very_large_field_number<'this>(&'this self) -> ::std::option::Option<i32> {
        None
    }
}

#[derive(Clone, PartialEq, Debug)]
struct MsgSingleField2<FieldType> {
    field: FieldType,
}

impl<FieldType> ::puroro::Message for MsgSingleField2<FieldType> {}

impl<FieldType> super::_puroro_traits::MsgTrait for MsgSingleField2<FieldType>
where
    FieldType: Clone, PartialEq, Debug,
{
    fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
        None
    }
    type Field2RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
    fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    fn float_optional<'this>(&'this self) -> ::std::option::Option<f32> {
        None
    }
    type Field4RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
    fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    fn string_optional<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
        None
    }
    type Field6RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<::std::borrow::Cow<'this, str>>;
    fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    type Field7MessageType<'this> = self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
    fn submsg_optional<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field7MessageType<'this>>> {
        None
    }
    type Field8MessageType<'this> = self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
    type Field8RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<::std::borrow::Cow<'this, Self::Field8MessageType<'this>>>;
    fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    fn enum_optional<'this>(&'this self) -> ::std::option::Option<self::_puroro_root::ser_tests2::Enum> {
        None
    }
    type Field10RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<self::_puroro_root::ser_tests2::Enum>;
    fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    fn very_large_field_number<'this>(&'this self) -> ::std::option::Option<i32> {
        None
    }
}

#[derive(Clone, PartialEq, Debug)]
struct MsgSingleField3<FieldType> {
    field: FieldType,
}

impl<FieldType> ::puroro::Message for MsgSingleField3<FieldType> {}

impl<FieldType> super::_puroro_traits::MsgTrait for MsgSingleField3<FieldType>
where
    FieldType: Clone, PartialEq, Debug,
{
    fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
        None
    }
    type Field2RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
    fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    fn float_optional<'this>(&'this self) -> ::std::option::Option<f32> {
        None
    }
    type Field4RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
    fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    fn string_optional<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
        None
    }
    type Field6RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<::std::borrow::Cow<'this, str>>;
    fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    type Field7MessageType<'this> = self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
    fn submsg_optional<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field7MessageType<'this>>> {
        None
    }
    type Field8MessageType<'this> = self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
    type Field8RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<::std::borrow::Cow<'this, Self::Field8MessageType<'this>>>;
    fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    fn enum_optional<'this>(&'this self) -> ::std::option::Option<self::_puroro_root::ser_tests2::Enum> {
        None
    }
    type Field10RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<self::_puroro_root::ser_tests2::Enum>;
    fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    fn very_large_field_number<'this>(&'this self) -> ::std::option::Option<i32> {
        None
    }
}

#[derive(Clone, PartialEq, Debug)]
struct MsgSingleField4<FieldType> {
    field: FieldType,
}

impl<FieldType> ::puroro::Message for MsgSingleField4<FieldType> {}

impl<FieldType> super::_puroro_traits::MsgTrait for MsgSingleField4<FieldType>
where
    FieldType: Clone, PartialEq, Debug,
{
    fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
        None
    }
    type Field2RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
    fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    fn float_optional<'this>(&'this self) -> ::std::option::Option<f32> {
        None
    }
    type Field4RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
    fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    fn string_optional<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
        None
    }
    type Field6RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<::std::borrow::Cow<'this, str>>;
    fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    type Field7MessageType<'this> = self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
    fn submsg_optional<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field7MessageType<'this>>> {
        None
    }
    type Field8MessageType<'this> = self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
    type Field8RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<::std::borrow::Cow<'this, Self::Field8MessageType<'this>>>;
    fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    fn enum_optional<'this>(&'this self) -> ::std::option::Option<self::_puroro_root::ser_tests2::Enum> {
        None
    }
    type Field10RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<self::_puroro_root::ser_tests2::Enum>;
    fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    fn very_large_field_number<'this>(&'this self) -> ::std::option::Option<i32> {
        None
    }
}

#[derive(Clone, PartialEq, Debug)]
struct MsgSingleField5<FieldType> {
    field: FieldType,
}

impl<FieldType> ::puroro::Message for MsgSingleField5<FieldType> {}

impl<FieldType> super::_puroro_traits::MsgTrait for MsgSingleField5<FieldType>
where
    FieldType: Clone, PartialEq, Debug,
{
    fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
        None
    }
    type Field2RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
    fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    fn float_optional<'this>(&'this self) -> ::std::option::Option<f32> {
        None
    }
    type Field4RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
    fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    fn string_optional<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
        None
    }
    type Field6RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<::std::borrow::Cow<'this, str>>;
    fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    type Field7MessageType<'this> = self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
    fn submsg_optional<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field7MessageType<'this>>> {
        None
    }
    type Field8MessageType<'this> = self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
    type Field8RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<::std::borrow::Cow<'this, Self::Field8MessageType<'this>>>;
    fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    fn enum_optional<'this>(&'this self) -> ::std::option::Option<self::_puroro_root::ser_tests2::Enum> {
        None
    }
    type Field10RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<self::_puroro_root::ser_tests2::Enum>;
    fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    fn very_large_field_number<'this>(&'this self) -> ::std::option::Option<i32> {
        None
    }
}

#[derive(Clone, PartialEq, Debug)]
struct MsgSingleField6<FieldType> {
    field: FieldType,
}

impl<FieldType> ::puroro::Message for MsgSingleField6<FieldType> {}

impl<FieldType> super::_puroro_traits::MsgTrait for MsgSingleField6<FieldType>
where
    FieldType: Clone, PartialEq, Debug,
{
    fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
        None
    }
    type Field2RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
    fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    fn float_optional<'this>(&'this self) -> ::std::option::Option<f32> {
        None
    }
    type Field4RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
    fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    fn string_optional<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
        None
    }
    type Field6RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<::std::borrow::Cow<'this, str>>;
    fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    type Field7MessageType<'this> = self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
    fn submsg_optional<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field7MessageType<'this>>> {
        None
    }
    type Field8MessageType<'this> = self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
    type Field8RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<::std::borrow::Cow<'this, Self::Field8MessageType<'this>>>;
    fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    fn enum_optional<'this>(&'this self) -> ::std::option::Option<self::_puroro_root::ser_tests2::Enum> {
        None
    }
    type Field10RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<self::_puroro_root::ser_tests2::Enum>;
    fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    fn very_large_field_number<'this>(&'this self) -> ::std::option::Option<i32> {
        None
    }
}

#[derive(Clone, PartialEq, Debug)]
struct MsgSingleField7<FieldType> {
    field: FieldType,
}

impl<FieldType> ::puroro::Message for MsgSingleField7<FieldType> {}

impl<FieldType> super::_puroro_traits::MsgTrait for MsgSingleField7<FieldType>
where
    FieldType: Clone, PartialEq, Debug,
{
    fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
        None
    }
    type Field2RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
    fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    fn float_optional<'this>(&'this self) -> ::std::option::Option<f32> {
        None
    }
    type Field4RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
    fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    fn string_optional<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
        None
    }
    type Field6RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<::std::borrow::Cow<'this, str>>;
    fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    type Field7MessageType<'this> = self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
    fn submsg_optional<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field7MessageType<'this>>> {
        None
    }
    type Field8MessageType<'this> = self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
    type Field8RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<::std::borrow::Cow<'this, Self::Field8MessageType<'this>>>;
    fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    fn enum_optional<'this>(&'this self) -> ::std::option::Option<self::_puroro_root::ser_tests2::Enum> {
        None
    }
    type Field10RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<self::_puroro_root::ser_tests2::Enum>;
    fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    fn very_large_field_number<'this>(&'this self) -> ::std::option::Option<i32> {
        None
    }
}

#[derive(Clone, PartialEq, Debug)]
struct MsgSingleField8<FieldType> {
    field: FieldType,
}

impl<FieldType> ::puroro::Message for MsgSingleField8<FieldType> {}

impl<FieldType> super::_puroro_traits::MsgTrait for MsgSingleField8<FieldType>
where
    FieldType: Clone, PartialEq, Debug,
{
    fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
        None
    }
    type Field2RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
    fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    fn float_optional<'this>(&'this self) -> ::std::option::Option<f32> {
        None
    }
    type Field4RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
    fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    fn string_optional<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
        None
    }
    type Field6RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<::std::borrow::Cow<'this, str>>;
    fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    type Field7MessageType<'this> = self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
    fn submsg_optional<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field7MessageType<'this>>> {
        None
    }
    type Field8MessageType<'this> = self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
    type Field8RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<::std::borrow::Cow<'this, Self::Field8MessageType<'this>>>;
    fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    fn enum_optional<'this>(&'this self) -> ::std::option::Option<self::_puroro_root::ser_tests2::Enum> {
        None
    }
    type Field10RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<self::_puroro_root::ser_tests2::Enum>;
    fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    fn very_large_field_number<'this>(&'this self) -> ::std::option::Option<i32> {
        None
    }
}

#[derive(Clone, PartialEq, Debug)]
struct MsgSingleField9<FieldType> {
    field: FieldType,
}

impl<FieldType> ::puroro::Message for MsgSingleField9<FieldType> {}

impl<FieldType> super::_puroro_traits::MsgTrait for MsgSingleField9<FieldType>
where
    FieldType: Clone, PartialEq, Debug,
{
    fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
        None
    }
    type Field2RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
    fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    fn float_optional<'this>(&'this self) -> ::std::option::Option<f32> {
        None
    }
    type Field4RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
    fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    fn string_optional<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
        None
    }
    type Field6RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<::std::borrow::Cow<'this, str>>;
    fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    type Field7MessageType<'this> = self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
    fn submsg_optional<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field7MessageType<'this>>> {
        None
    }
    type Field8MessageType<'this> = self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
    type Field8RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<::std::borrow::Cow<'this, Self::Field8MessageType<'this>>>;
    fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    fn enum_optional<'this>(&'this self) -> ::std::option::Option<self::_puroro_root::ser_tests2::Enum> {
        None
    }
    type Field10RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<self::_puroro_root::ser_tests2::Enum>;
    fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    fn very_large_field_number<'this>(&'this self) -> ::std::option::Option<i32> {
        None
    }
}

#[derive(Clone, PartialEq, Debug)]
struct MsgSingleField10<FieldType> {
    field: FieldType,
}

impl<FieldType> ::puroro::Message for MsgSingleField10<FieldType> {}

impl<FieldType> super::_puroro_traits::MsgTrait for MsgSingleField10<FieldType>
where
    FieldType: Clone, PartialEq, Debug,
{
    fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
        None
    }
    type Field2RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
    fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    fn float_optional<'this>(&'this self) -> ::std::option::Option<f32> {
        None
    }
    type Field4RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
    fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    fn string_optional<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
        None
    }
    type Field6RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<::std::borrow::Cow<'this, str>>;
    fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    type Field7MessageType<'this> = self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
    fn submsg_optional<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field7MessageType<'this>>> {
        None
    }
    type Field8MessageType<'this> = self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
    type Field8RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<::std::borrow::Cow<'this, Self::Field8MessageType<'this>>>;
    fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    fn enum_optional<'this>(&'this self) -> ::std::option::Option<self::_puroro_root::ser_tests2::Enum> {
        None
    }
    type Field10RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<self::_puroro_root::ser_tests2::Enum>;
    fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    fn very_large_field_number<'this>(&'this self) -> ::std::option::Option<i32> {
        None
    }
}

#[derive(Clone, PartialEq, Debug)]
struct MsgSingleField536870911<FieldType> {
    field: FieldType,
}

impl<FieldType> ::puroro::Message for MsgSingleField536870911<FieldType> {}

impl<FieldType> super::_puroro_traits::MsgTrait for MsgSingleField536870911<FieldType>
where
    FieldType: Clone, PartialEq, Debug,
{
    fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
        None
    }
    type Field2RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
    fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    fn float_optional<'this>(&'this self) -> ::std::option::Option<f32> {
        None
    }
    type Field4RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
    fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    fn string_optional<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
        None
    }
    type Field6RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<::std::borrow::Cow<'this, str>>;
    fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    type Field7MessageType<'this> = self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
    fn submsg_optional<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field7MessageType<'this>>> {
        None
    }
    type Field8MessageType<'this> = self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
    type Field8RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<::std::borrow::Cow<'this, Self::Field8MessageType<'this>>>;
    fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    fn enum_optional<'this>(&'this self) -> ::std::option::Option<self::_puroro_root::ser_tests2::Enum> {
        None
    }
    type Field10RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<self::_puroro_root::ser_tests2::Enum>;
    fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
        ::puroro_internal::impls::empty::EmptyRepeatedField::new()
    }
    fn very_large_field_number<'this>(&'this self) -> ::std::option::Option<i32> {
        None
    }
}
}
pub use _puroro_traits::*;
pub mod _puroro_traits {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
    
    pub trait MsgTrait: ::std::clone::Clone {
        fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32>;
        type Field2RepeatedType<'this>: ::puroro::RepeatedField<'this, i32>;
    
        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this>;
        fn float_optional<'this>(&'this self) -> ::std::option::Option<f32>;
        type Field4RepeatedType<'this>: ::puroro::RepeatedField<'this, f32>;
    
        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this>;
        fn string_optional<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>>;
        type Field6RepeatedType<'this>: ::puroro::RepeatedField<'this, ::std::borrow::Cow<'this, str>>;
    
        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this>;
        type Field7MessageType<'this>: 'this + self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_traits::SubmsgTrait;
        fn submsg_optional<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field7MessageType<'this>>>;
        type Field8MessageType<'this>: 'this + self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_traits::SubmsgTrait;
        type Field8RepeatedType<'this>: ::puroro::RepeatedField<'this, ::std::borrow::Cow<'this, Self::Field8MessageType<'this>>>;
    
        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this>;
        fn enum_optional<'this>(&'this self) -> ::std::option::Option<self::_puroro_root::ser_tests2::Enum>;
        type Field10RepeatedType<'this>: ::puroro::RepeatedField<'this, self::_puroro_root::ser_tests2::Enum>;
    
        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this>;
        fn very_large_field_number<'this>(&'this self) -> ::std::option::Option<i32>;
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
        
        pub use _puroro_impls::SubmsgSimple as Submsg;
        pub mod _puroro_impls {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }
        
        #[derive(Clone, Default, PartialEq, Debug)]
        pub struct SubmsgSimple {pub i32_optional: ::std::option::Option<i32>,
        }
        impl ::puroro::Message for SubmsgSimple {}
        
        impl super::_puroro_traits::SubmsgTrait for SubmsgSimple {
            fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
                ::std::clone::Clone::clone(&self.i32_optional)
            }
        }
        
        impl ::puroro::DeserFromBytesIter for SubmsgSimple {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>
            {
                ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
            }
        }
        
        impl ::puroro_internal::de::DeserFieldsFromBytesIter for SubmsgSimple {
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>
            {
                match field_number {
                    1 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                        ::puroro::tags::Optional, ::puroro::tags::Int32
                    >::deser_field(&mut self.i32_optional, data),
        
                    _ => unimplemented!("TODO: This case should be handled properly..."),
                }
            }
        }
        
        impl ::puroro::SerToIoWrite for SubmsgSimple {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write
            {
                ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::Int32
                >::ser_field(&self.i32_optional, 1, out)?;
                ::std::result::Result::Ok(())
            }
        }
        
        #[derive(Clone, Default, PartialEq, Debug)]
        pub struct SubmsgEmpty;
        
        impl ::puroro::Message for SubmsgEmpty {}
        
        impl super::_puroro_traits::SubmsgTrait for SubmsgEmpty {
            fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
                None
            }
        }
        
        impl ::puroro::SerToIoWrite for SubmsgEmpty {
            fn ser<W>(&self, _out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write
            {
                ::std::result::Result::Ok(())
            }
        }
        
        #[derive(Clone, PartialEq, Debug)]
        struct SubmsgSingleField1<FieldType> {
            field: FieldType,
        }
        
        impl<FieldType> ::puroro::Message for SubmsgSingleField1<FieldType> {}
        
        impl<FieldType> super::_puroro_traits::SubmsgTrait for SubmsgSingleField1<FieldType>
        where
            FieldType: Clone, PartialEq, Debug,
        {
            fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
                None
            }
        }
        }
        pub use _puroro_traits::*;
        pub mod _puroro_traits {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }
            
            pub trait SubmsgTrait: ::std::clone::Clone {
                fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32>;
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