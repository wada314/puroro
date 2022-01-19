use crate::internal::Bitfield;
use crate::internal::{FieldAndSharedRef, SharedObjects};
use crate::internal::{FieldProperties, MessageProperties};
use crate::tags;

pub trait GetFieldMethodImpl<'a, FP, ImplTag, LabelTag, TypeTag> {
    type GetterTypeImpl;
    fn get_impl(&self) -> Self::GetterTypeImpl;
}
pub trait GetFieldMethod<'a, FP, ImplTag> {
    type GetterType;
    fn get(&self) -> Self::GetterType;
}
impl<'a, FP, ImplTag, T> GetFieldMethod<'a, FP, ImplTag> for T
where
    FP: FieldProperties,
    T: GetFieldMethodImpl<'a, FP, ImplTag, FP::LabelTag, FP::TypeTag>,
{
    type GetterType =
        <Self as GetFieldMethodImpl<'a, FP, ImplTag, FP::LabelTag, FP::TypeTag>>::GetterTypeImpl;
    fn get(&self) -> Self::GetterType {
        <Self as GetFieldMethodImpl<'a, FP, ImplTag, FP::LabelTag, FP::TypeTag>>::get_impl(self)
    }
}

pub trait GetOptFieldMethodImpl<'a, FP, ImplTag, LabelTag, TypeTag> {
    type GetterTypeImpl;
    fn get_opt_impl(&self) -> Self::GetterTypeImpl;
}
pub trait GetOptFieldMethod<'a, FP, ImplTag> {
    type GetterType;
    fn get_opt(&self) -> Self::GetterType;
}
impl<'a, FP, ImplTag, T> GetOptFieldMethod<'a, FP, ImplTag> for T
where
    FP: FieldProperties,
    T: GetOptFieldMethodImpl<'a, FP, ImplTag, FP::LabelTag, FP::TypeTag>,
{
    type GetterType =
        <Self as GetOptFieldMethodImpl<'a, FP, ImplTag, FP::LabelTag, FP::TypeTag>>::GetterTypeImpl;
    fn get_opt(&self) -> Self::GetterType {
        <Self as GetOptFieldMethodImpl<'a, FP, ImplTag, FP::LabelTag, FP::TypeTag>>::get_opt_impl(
            &self,
        )
    }
}

// for [optional|required] numeric types
impl<'a, _1, _2, _3, _4, _5, FP, FieldType, Shared>
    GetOptFieldMethodImpl<
        'a,
        FP,
        tags::SimpleImpl,
        tags::NeedOptionalBitLabel<_1, _2>,
        tags::NonLdType<_3, _4, _5>,
    > for FieldAndSharedRef<'a, FieldType, Shared>
where
    FP: FieldProperties<
        LabelTag = tags::NeedOptionalBitLabel<_1, _2>,
        TypeTag = tags::NonLdType<_3, _4, _5>,
    >,
    tags::NonLdType<_3, _4, _5>: tags::NumericalTypeTag,
    FP::TypeTag: tags::NumericalTypeTag,
    FieldType: Clone + Into<<FP::TypeTag as tags::NumericalTypeTag>::NativeType>,
    Shared: SharedObjects,
{
    type GetterTypeImpl = Option<<FP::TypeTag as tags::NumericalTypeTag>::NativeType>;
    fn get_opt_impl(&self) -> Self::GetterTypeImpl {
        let opt_bit_index = FP::OPTIONAL_FIELD_BITFIELD_INDEX
            + FP::MessageProperties::OPTIONAL_FIELD_BITFIELD_START_INDEX;
        if self.shared.bitfield().get(opt_bit_index) {
            Some(self.field.clone().into())
        } else {
            None
        }
    }
}

// for [optional|required] string type
impl<'a, _1, _2, FP, FieldType, Shared>
    GetOptFieldMethodImpl<
        'a,
        FP,
        tags::SimpleImpl,
        tags::NeedOptionalBitLabel<_1, _2>,
        tags::String,
    > for FieldAndSharedRef<'a, FieldType, Shared>
where
    FP: FieldProperties<LabelTag = tags::NeedOptionalBitLabel<_1, _2>, TypeTag = tags::String>,
    FieldType: AsRef<str>,
    Shared: SharedObjects,
{
    type GetterTypeImpl = Option<&'a str>;
    fn get_opt_impl(&self) -> Self::GetterTypeImpl {
        let opt_bit_index = FP::OPTIONAL_FIELD_BITFIELD_INDEX
            + FP::MessageProperties::OPTIONAL_FIELD_BITFIELD_START_INDEX;
        if self.shared.bitfield().get(opt_bit_index) {
            Some(AsRef::as_ref(self.field))
        } else {
            None
        }
    }
}

// for repeated message type
impl<'a, FP, MP, MessageType, Shared>
    GetFieldMethodImpl<'a, FP, tags::SimpleImpl, tags::Repeated, tags::Message<MP>>
    for FieldAndSharedRef<'a, Vec<MessageType>, Shared>
where
    FP: FieldProperties<LabelTag = tags::Repeated, TypeTag = tags::Message<MP>>,
    Shared: SharedObjects,
{
    type GetterTypeImpl = &'a [MessageType];
    fn get_impl(&self) -> Self::GetterTypeImpl {
        self.field
    }
}
