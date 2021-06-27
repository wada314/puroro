#![cfg_attr(feature = "puroro-nightly", feature(backtrace))]
#![cfg_attr(feature = "puroro-nightly", feature(generic_associated_types))]
#![cfg_attr(feature = "puroro-nightly", feature(min_type_alias_impl_trait))]
#![allow(incomplete_features)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use ::puroro::apply::FieldVisitor;
use ::puroro::tags;

use ::sample_pb;
use sample_pb::simple::sample2::Msg;
use sample_pb::tags::sample2::MsgTag;

pub trait GetFieldDefaultValueType {
    type Type;
}
impl<L, V> GetFieldDefaultValueType for (L, (tags::wire::Variant, V))
where
    L: tags::FieldLabelTag,
    V: tags::VariantTypeTag,
{
    type Type = <V as tags::VariantTypeTag>::NativeType;
}
impl<L> GetFieldDefaultValueType for (L, tags::String)
where
    L: tags::FieldLabelTag,
{
    type Type = &'static str;
}
impl<L, M> GetFieldDefaultValueType for (L, tags::Message<M>)
where
    L: tags::FieldLabelTag,
{
    type Type = ();
}

pub trait FieldInfo {
    // Something like (tags::Repeated, (tags::wire::Variant, tags::value::Int32)).
    type LabelAndTypeTag: tags::FieldLabelAndTypeTag + GetFieldDefaultValueType;

    // Returns a default value specified by proto2's [default = ???] annotation,
    // if it's available.
    const FIELD_DEFAULT_VALUE: Option<<Self::LabelAndTypeTag as GetFieldDefaultValueType>::Type>;
}

pub trait FieldInfoOf<const FIELD_NUMBER: usize> {
    type Type: FieldInfo;
}

pub struct MsgTagField1;
impl FieldInfo for MsgTagField1 {
    type LabelAndTypeTag = (tags::Required, tags::Int32);
    const FIELD_DEFAULT_VALUE: Option<i32> = Some(3);
}
impl FieldInfoOf<1> for MsgTag {
    type Type = MsgTagField1;
}

pub struct MsgTagField9;
impl FieldInfo for MsgTagField9 {
    type LabelAndTypeTag = (tags::Repeated, tags::String);
    const FIELD_DEFAULT_VALUE: Option<&'static str> = Some("hello!");
}
impl FieldInfoOf<9> for MsgTag {
    type Type = MsgTagField9;
}

pub struct MsgTagField10;
impl FieldInfo for MsgTagField10 {
    type LabelAndTypeTag = (tags::Required, tags::Message<MsgTag>);
    const FIELD_DEFAULT_VALUE: Option<()> = None;
}
impl FieldInfoOf<10> for MsgTag {
    type Type = MsgTagField10;
}

pub trait FieldImplInfo {
    type FieldInfoType: FieldInfo;
    // Something like tags::SimpleStruct
    type ImplTypeTag: tags::ImplTypeTag;

    fn field_new() -> <Self::ImplTypeTag as ::puroro_internal::FieldTypeGen<
        <Self::FieldInfoType as FieldInfo>::LabelAndTypeTag,
    >>::Type
    where
        Self::ImplTypeTag:
            ::puroro_internal::FieldTypeGen<<Self::FieldInfoType as FieldInfo>::LabelAndTypeTag>;
}

pub trait FieldImplInfoOf<const FIELD_NUMBER: usize> {
    type Type: FieldImplInfo;
}

pub struct MsgField1Info;
impl FieldImplInfo for MsgField1Info {
    type FieldInfoType = MsgTagField1;
    type ImplTypeTag = tags::SimpleStruct;
    fn field_new() -> <Self::ImplTypeTag as ::puroro_internal::FieldTypeGen<
        <Self::FieldInfoType as FieldInfo>::LabelAndTypeTag,
    >>::Type
    where
        Self::ImplTypeTag:
            ::puroro_internal::FieldTypeGen<<Self::FieldInfoType as FieldInfo>::LabelAndTypeTag>,
    {
        Self::FieldInfoType::FIELD_DEFAULT_VALUE.unwrap_or(Default::default())
    }
}
impl FieldImplInfoOf<1> for Msg {
    type Type = MsgField1Info;
}

pub struct MsgField9Info<'bump>(std::marker::PhantomData<&'bump ()>);
impl<'bump> FieldImplInfo for MsgField9Info<'bump> {
    type FieldInfoType = MsgTagField9;
    type ImplTypeTag = tags::Bumpalo<'bump>;
    fn field_new() -> <Self::ImplTypeTag as ::puroro_internal::FieldTypeGen<
        <Self::FieldInfoType as FieldInfo>::LabelAndTypeTag,
    >>::Type
    where
        Self::ImplTypeTag:
            ::puroro_internal::FieldTypeGen<<Self::FieldInfoType as FieldInfo>::LabelAndTypeTag>,
    {
        todo!()
    }
}

pub struct MsgField10Info;
impl FieldImplInfo for MsgField10Info {
    type FieldInfoType = MsgTagField10;
    type ImplTypeTag = tags::SimpleStruct;

    fn field_new() -> <Self::ImplTypeTag as ::puroro_internal::FieldTypeGen<
        <Self::FieldInfoType as FieldInfo>::LabelAndTypeTag,
    >>::Type
    where
        Self::ImplTypeTag:
            ::puroro_internal::FieldTypeGen<<Self::FieldInfoType as FieldInfo>::LabelAndTypeTag>,
    {
        Default::default()
    }
}
impl FieldImplInfoOf<10> for Msg {
    type Type = MsgField10Info;
}

#[cfg(test)]
mod tests {
    #[allow(unused)]
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test1() {
        use sample_pb::traits::sample2::Test1Trait;
        use std::convert::TryFrom;
        let slice = [0x08, 0x96, 0x01];

        let test1_simple = <sample_pb::simple::sample2::Test1 as ::puroro::DeserializableFromSlice>::deser_from_slice(
            &slice,
        )
        .unwrap();
        assert_eq!(Some(150), test1_simple.a());

        let test1_slice_view =
            sample_pb::slice_view::sample2::Test1::<_>::try_from(&slice as &[u8]).unwrap();
        assert_eq!(Some(150), test1_slice_view.a());
    }

    #[test]
    fn test2() {
        use sample_pb::traits::sample2::Test2Trait;
        use std::convert::TryFrom;
        let slice = [0x12, 0x07, 0x74, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67];

        let test2_simple = <sample_pb::simple::sample2::Test2 as ::puroro::DeserializableFromSlice>::deser_from_slice(
            &slice,
        )
        .unwrap();
        assert_eq!(Some("testing"), test2_simple.b().as_deref());

        let test2_slice_view =
            sample_pb::slice_view::sample2::Test2::<_>::try_from(&slice as &[u8]).unwrap();
        assert_eq!(Some("testing"), test2_slice_view.b().as_deref());
    }

    #[test]
    fn test3() {
        use sample_pb::traits::sample2::{Test1Trait, Test3Trait};
        use std::convert::TryFrom;
        let slice = [0x1a, 0x03, 0x08, 0x96, 0x01];

        let test3_simple = <sample_pb::simple::sample2::Test3 as ::puroro::DeserializableFromSlice>::deser_from_slice(
            &slice,
        )
        .unwrap();
        assert!(test3_simple.c().is_some());
        let test1_simple = test3_simple.c().unwrap();
        assert_eq!(Some(150), test1_simple.a());

        let test3_slice_view =
            sample_pb::slice_view::sample2::Test3::<_>::try_from(&slice as &[u8]).unwrap();
        assert!(test3_slice_view.c().is_some());
        let test1_slice_view = test3_slice_view.c().unwrap();
        assert_eq!(Some(150), test1_slice_view.a());
    }

    #[test]
    fn test4() {
        use ::puroro::RepeatedField;
        use sample_pb::traits::sample2::Test4Trait;
        use std::convert::TryFrom;
        let slice = [0x22, 0x06, 0x03, 0x8E, 0x02, 0x9E, 0xA7, 0x05];

        let test4_simple = <sample_pb::simple::sample2::Test4 as ::puroro::DeserializableFromSlice>::deser_from_slice(
            &slice,
        )
        .unwrap();
        assert_eq!(
            // WTF!
            <&Vec<i32> as ::puroro::RepeatedField<::puroro::tags::Int32, i32>>::iter(
                &test4_simple.d()
            )
            .collect::<Vec<_>>(),
            vec![3, 270, 86942]
        );

        let test4_slice_view =
            sample_pb::slice_view::sample2::Test4::<_>::try_from(&slice as &[u8]).unwrap();
        assert_eq!(
            test4_slice_view.d().iter().collect::<Vec<_>>(),
            vec![3, 270, 86942]
        );
    }
}
