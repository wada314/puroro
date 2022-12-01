pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}
pub mod _puroro {
    pub use ::puroro::*;
}
pub mod msg;
#[derive(::std::default::Default)]
pub struct Msg {
    i32_unlabeled: self::_puroro::internal::field_type::SingularNumericalField::<
        i32,
        self::_puroro::tags::Int32,
    >,
    i32_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        i32,
        self::_puroro::tags::Int32,
    >,
    float_unlabeled: self::_puroro::internal::field_type::SingularNumericalField::<
        f32,
        self::_puroro::tags::Float,
    >,
    float_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        f32,
        self::_puroro::tags::Float,
    >,
    string_unlabeled: self::_puroro::internal::field_type::SingularStringField,
    string_repeated: self::_puroro::internal::field_type::RepeatedStringField,
    submsg_unlabeled: self::_puroro::internal::field_type::SingularHeapMessageField::<
        self::_puroro_root::ser_tests3::msg::Submsg,
    >,
    submsg_repeated: self::_puroro::internal::field_type::RepeatedMessageField::<
        self::_puroro_root::ser_tests3::msg::Submsg,
    >,
    enum_unlabeled: self::_puroro::internal::field_type::SingularNumericalField::<
        self::_puroro_root::ser_tests3::Enum,
        self::_puroro::tags::Enum3::<self::_puroro_root::ser_tests3::Enum>,
    >,
    enum_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        self::_puroro_root::ser_tests3::Enum,
        self::_puroro::tags::Enum3::<self::_puroro_root::ser_tests3::Enum>,
    >,
    very_large_field_number: self::_puroro::internal::field_type::SingularNumericalField::<
        i32,
        self::_puroro::tags::Int32,
    >,
    _bitfield: self::_puroro::bitvec::BitArray<0usize>,
}
impl Msg {
    pub fn i32_unlabeled(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            i32,
            self::_puroro::tags::Int32,
        > as NonRepeatedFieldType>::get_field(
            &self.i32_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn i32_unlabeled_opt(&self) -> ::std::option::Option::<i32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            i32,
            self::_puroro::tags::Int32,
        > as NonRepeatedFieldType>::get_field_opt(&self.i32_unlabeled, &self._bitfield)
    }
    pub fn i32_unlabeled_mut(&mut self) -> &mut i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            i32,
            self::_puroro::tags::Int32,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.i32_unlabeled,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_i32_unlabeled(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            i32,
            self::_puroro::tags::Int32,
        > as NonRepeatedFieldType>::get_field_opt(&self.i32_unlabeled, &self._bitfield)
            .is_some()
    }
    pub fn clear_i32_unlabeled(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            i32,
            self::_puroro::tags::Int32,
        > as NonRepeatedFieldType>::clear(&mut self.i32_unlabeled, &mut self._bitfield)
    }
    pub fn i32_repeated(&self) -> &[i32] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            i32,
            self::_puroro::tags::Int32,
        > as RepeatedFieldType>::get_field(&self.i32_repeated, &self._bitfield)
    }
    pub fn i32_repeated_mut(&mut self) -> &mut ::std::vec::Vec::<i32> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            i32,
            self::_puroro::tags::Int32,
        > as RepeatedFieldType>::mut_field(&mut self.i32_repeated, &mut self._bitfield)
    }
    pub fn clear_i32_repeated(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            i32,
            self::_puroro::tags::Int32,
        > as RepeatedFieldType>::clear(&mut self.i32_repeated, &mut self._bitfield)
    }
    pub fn float_unlabeled(&self) -> f32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            f32,
            self::_puroro::tags::Float,
        > as NonRepeatedFieldType>::get_field(
            &self.float_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn float_unlabeled_opt(&self) -> ::std::option::Option::<f32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            f32,
            self::_puroro::tags::Float,
        > as NonRepeatedFieldType>::get_field_opt(&self.float_unlabeled, &self._bitfield)
    }
    pub fn float_unlabeled_mut(&mut self) -> &mut f32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            f32,
            self::_puroro::tags::Float,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.float_unlabeled,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_float_unlabeled(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            f32,
            self::_puroro::tags::Float,
        > as NonRepeatedFieldType>::get_field_opt(&self.float_unlabeled, &self._bitfield)
            .is_some()
    }
    pub fn clear_float_unlabeled(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            f32,
            self::_puroro::tags::Float,
        > as NonRepeatedFieldType>::clear(&mut self.float_unlabeled, &mut self._bitfield)
    }
    pub fn float_repeated(&self) -> &[f32] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            f32,
            self::_puroro::tags::Float,
        > as RepeatedFieldType>::get_field(&self.float_repeated, &self._bitfield)
    }
    pub fn float_repeated_mut(&mut self) -> &mut ::std::vec::Vec::<f32> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            f32,
            self::_puroro::tags::Float,
        > as RepeatedFieldType>::mut_field(&mut self.float_repeated, &mut self._bitfield)
    }
    pub fn clear_float_repeated(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            f32,
            self::_puroro::tags::Float,
        > as RepeatedFieldType>::clear(&mut self.float_repeated, &mut self._bitfield)
    }
    pub fn string_unlabeled(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularStringField as NonRepeatedFieldType>::get_field(
            &self.string_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn string_unlabeled_opt(&self) -> ::std::option::Option::<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularStringField as NonRepeatedFieldType>::get_field_opt(
            &self.string_unlabeled,
            &self._bitfield,
        )
    }
    pub fn string_unlabeled_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularStringField as NonRepeatedFieldType>::mut_field(
            &mut self.string_unlabeled,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_string_unlabeled(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularStringField as NonRepeatedFieldType>::get_field_opt(
                &self.string_unlabeled,
                &self._bitfield,
            )
            .is_some()
    }
    pub fn clear_string_unlabeled(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularStringField as NonRepeatedFieldType>::clear(
            &mut self.string_unlabeled,
            &mut self._bitfield,
        )
    }
    pub fn string_repeated(&self) -> &[impl ::std::ops::Deref::<Target = str>] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedStringField as RepeatedFieldType>::get_field(
            &self.string_repeated,
            &self._bitfield,
        )
    }
    pub fn string_repeated_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<impl ::std::ops::Deref::<Target = str>> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedStringField as RepeatedFieldType>::mut_field(
            &mut self.string_repeated,
            &mut self._bitfield,
        )
    }
    pub fn clear_string_repeated(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedStringField as RepeatedFieldType>::clear(
            &mut self.string_repeated,
            &mut self._bitfield,
        )
    }
    pub fn submsg_unlabeled(
        &self,
    ) -> ::std::option::Option::<&self::_puroro_root::ser_tests3::msg::Submsg> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField::<
            self::_puroro_root::ser_tests3::msg::Submsg,
        > as NonRepeatedFieldType>::get_field(
            &self.submsg_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn submsg_unlabeled_opt(
        &self,
    ) -> ::std::option::Option::<&self::_puroro_root::ser_tests3::msg::Submsg> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField::<
            self::_puroro_root::ser_tests3::msg::Submsg,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.submsg_unlabeled,
            &self._bitfield,
        )
    }
    pub fn submsg_unlabeled_mut(
        &mut self,
    ) -> &mut self::_puroro_root::ser_tests3::msg::Submsg {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField::<
            self::_puroro_root::ser_tests3::msg::Submsg,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.submsg_unlabeled,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_submsg_unlabeled(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField::<
            self::_puroro_root::ser_tests3::msg::Submsg,
        > as NonRepeatedFieldType>::get_field_opt(
                &self.submsg_unlabeled,
                &self._bitfield,
            )
            .is_some()
    }
    pub fn clear_submsg_unlabeled(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField::<
            self::_puroro_root::ser_tests3::msg::Submsg,
        > as NonRepeatedFieldType>::clear(
            &mut self.submsg_unlabeled,
            &mut self._bitfield,
        )
    }
    pub fn submsg_repeated(&self) -> &[self::_puroro_root::ser_tests3::msg::Submsg] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField::<
            self::_puroro_root::ser_tests3::msg::Submsg,
        > as RepeatedFieldType>::get_field(&self.submsg_repeated, &self._bitfield)
    }
    pub fn submsg_repeated_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_puroro_root::ser_tests3::msg::Submsg> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField::<
            self::_puroro_root::ser_tests3::msg::Submsg,
        > as RepeatedFieldType>::mut_field(
            &mut self.submsg_repeated,
            &mut self._bitfield,
        )
    }
    pub fn clear_submsg_repeated(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField::<
            self::_puroro_root::ser_tests3::msg::Submsg,
        > as RepeatedFieldType>::clear(&mut self.submsg_repeated, &mut self._bitfield)
    }
    pub fn enum_unlabeled(&self) -> self::_puroro_root::ser_tests3::Enum {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            self::_puroro_root::ser_tests3::Enum,
            self::_puroro::tags::Enum3::<self::_puroro_root::ser_tests3::Enum>,
        > as NonRepeatedFieldType>::get_field(
            &self.enum_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn enum_unlabeled_opt(
        &self,
    ) -> ::std::option::Option::<self::_puroro_root::ser_tests3::Enum> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            self::_puroro_root::ser_tests3::Enum,
            self::_puroro::tags::Enum3::<self::_puroro_root::ser_tests3::Enum>,
        > as NonRepeatedFieldType>::get_field_opt(&self.enum_unlabeled, &self._bitfield)
    }
    pub fn enum_unlabeled_mut(&mut self) -> &mut self::_puroro_root::ser_tests3::Enum {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            self::_puroro_root::ser_tests3::Enum,
            self::_puroro::tags::Enum3::<self::_puroro_root::ser_tests3::Enum>,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.enum_unlabeled,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_enum_unlabeled(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            self::_puroro_root::ser_tests3::Enum,
            self::_puroro::tags::Enum3::<self::_puroro_root::ser_tests3::Enum>,
        > as NonRepeatedFieldType>::get_field_opt(&self.enum_unlabeled, &self._bitfield)
            .is_some()
    }
    pub fn clear_enum_unlabeled(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            self::_puroro_root::ser_tests3::Enum,
            self::_puroro::tags::Enum3::<self::_puroro_root::ser_tests3::Enum>,
        > as NonRepeatedFieldType>::clear(&mut self.enum_unlabeled, &mut self._bitfield)
    }
    pub fn enum_repeated(&self) -> &[self::_puroro_root::ser_tests3::Enum] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            self::_puroro_root::ser_tests3::Enum,
            self::_puroro::tags::Enum3::<self::_puroro_root::ser_tests3::Enum>,
        > as RepeatedFieldType>::get_field(&self.enum_repeated, &self._bitfield)
    }
    pub fn enum_repeated_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_puroro_root::ser_tests3::Enum> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            self::_puroro_root::ser_tests3::Enum,
            self::_puroro::tags::Enum3::<self::_puroro_root::ser_tests3::Enum>,
        > as RepeatedFieldType>::mut_field(&mut self.enum_repeated, &mut self._bitfield)
    }
    pub fn clear_enum_repeated(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            self::_puroro_root::ser_tests3::Enum,
            self::_puroro::tags::Enum3::<self::_puroro_root::ser_tests3::Enum>,
        > as RepeatedFieldType>::clear(&mut self.enum_repeated, &mut self._bitfield)
    }
    pub fn very_large_field_number(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            i32,
            self::_puroro::tags::Int32,
        > as NonRepeatedFieldType>::get_field(
            &self.very_large_field_number,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn very_large_field_number_opt(&self) -> ::std::option::Option::<i32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            i32,
            self::_puroro::tags::Int32,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.very_large_field_number,
            &self._bitfield,
        )
    }
    pub fn very_large_field_number_mut(&mut self) -> &mut i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            i32,
            self::_puroro::tags::Int32,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.very_large_field_number,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_very_large_field_number(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            i32,
            self::_puroro::tags::Int32,
        > as NonRepeatedFieldType>::get_field_opt(
                &self.very_large_field_number,
                &self._bitfield,
            )
            .is_some()
    }
    pub fn clear_very_large_field_number(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            i32,
            self::_puroro::tags::Int32,
        > as NonRepeatedFieldType>::clear(
            &mut self.very_large_field_number,
            &mut self._bitfield,
        )
    }
}
impl self::_puroro::Message for Msg {
    fn from_bytes_iter<I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>>(
        iter: I,
    ) -> self::_puroro::Result<Self> {
        let mut msg = <Self as ::std::default::Default>::default();
        msg.merge_from_bytes_iter(iter)?;
        ::std::result::Result::Ok(msg)
    }
    fn merge_from_bytes_iter<I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>>(
        &mut self,
        mut iter: I,
    ) -> self::_puroro::Result<()> {
        use self::_puroro::internal::ser::FieldData;
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion as _;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            match number {
                1i32 => {
                    <self::_puroro::internal::field_type::SingularNumericalField::<
                        i32,
                        self::_puroro::tags::Int32,
                    > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.i32_unlabeled,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                2i32 => {
                    <self::_puroro::internal::field_type::RepeatedNumericalField::<
                        i32,
                        self::_puroro::tags::Int32,
                    > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.i32_repeated,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                3i32 => {
                    <self::_puroro::internal::field_type::SingularNumericalField::<
                        f32,
                        self::_puroro::tags::Float,
                    > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.float_unlabeled,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                4i32 => {
                    <self::_puroro::internal::field_type::RepeatedNumericalField::<
                        f32,
                        self::_puroro::tags::Float,
                    > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.float_repeated,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                5i32 => {
                    <self::_puroro::internal::field_type::SingularStringField as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.string_unlabeled,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                6i32 => {
                    <self::_puroro::internal::field_type::RepeatedStringField as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.string_repeated,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                7i32 => {
                    <self::_puroro::internal::field_type::SingularHeapMessageField::<
                        self::_puroro_root::ser_tests3::msg::Submsg,
                    > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.submsg_unlabeled,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                8i32 => {
                    <self::_puroro::internal::field_type::RepeatedMessageField::<
                        self::_puroro_root::ser_tests3::msg::Submsg,
                    > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.submsg_repeated,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                9i32 => {
                    <self::_puroro::internal::field_type::SingularNumericalField::<
                        self::_puroro_root::ser_tests3::Enum,
                        self::_puroro::tags::Enum3::<
                            self::_puroro_root::ser_tests3::Enum,
                        >,
                    > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.enum_unlabeled,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                10i32 => {
                    <self::_puroro::internal::field_type::RepeatedNumericalField::<
                        self::_puroro_root::ser_tests3::Enum,
                        self::_puroro::tags::Enum3::<
                            self::_puroro_root::ser_tests3::Enum,
                        >,
                    > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.enum_repeated,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                536870911i32 => {
                    <self::_puroro::internal::field_type::SingularNumericalField::<
                        i32,
                        self::_puroro::tags::Int32,
                    > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.very_large_field_number,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                _ => todo!(),
            }
        }
        ::std::result::Result::Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion as _;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            i32,
            self::_puroro::tags::Int32,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.i32_unlabeled,
            &self._bitfield,
            1i32,
            out,
        )?;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            i32,
            self::_puroro::tags::Int32,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.i32_repeated,
            &self._bitfield,
            2i32,
            out,
        )?;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            f32,
            self::_puroro::tags::Float,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.float_unlabeled,
            &self._bitfield,
            3i32,
            out,
        )?;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            f32,
            self::_puroro::tags::Float,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.float_repeated,
            &self._bitfield,
            4i32,
            out,
        )?;
        <self::_puroro::internal::field_type::SingularStringField as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.string_unlabeled,
            &self._bitfield,
            5i32,
            out,
        )?;
        <self::_puroro::internal::field_type::RepeatedStringField as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.string_repeated,
            &self._bitfield,
            6i32,
            out,
        )?;
        <self::_puroro::internal::field_type::SingularHeapMessageField::<
            self::_puroro_root::ser_tests3::msg::Submsg,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.submsg_unlabeled,
            &self._bitfield,
            7i32,
            out,
        )?;
        <self::_puroro::internal::field_type::RepeatedMessageField::<
            self::_puroro_root::ser_tests3::msg::Submsg,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.submsg_repeated,
            &self._bitfield,
            8i32,
            out,
        )?;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            self::_puroro_root::ser_tests3::Enum,
            self::_puroro::tags::Enum3::<self::_puroro_root::ser_tests3::Enum>,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.enum_unlabeled,
            &self._bitfield,
            9i32,
            out,
        )?;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            self::_puroro_root::ser_tests3::Enum,
            self::_puroro::tags::Enum3::<self::_puroro_root::ser_tests3::Enum>,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.enum_repeated,
            &self._bitfield,
            10i32,
            out,
        )?;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            i32,
            self::_puroro::tags::Int32,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.very_large_field_number,
            &self._bitfield,
            536870911i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for Msg {
    fn clone(&self) -> Self {
        Self {
            i32_unlabeled: <self::_puroro::internal::field_type::SingularNumericalField::<
                i32,
                self::_puroro::tags::Int32,
            > as ::std::clone::Clone>::clone(&self.i32_unlabeled),
            i32_repeated: <self::_puroro::internal::field_type::RepeatedNumericalField::<
                i32,
                self::_puroro::tags::Int32,
            > as ::std::clone::Clone>::clone(&self.i32_repeated),
            float_unlabeled: <self::_puroro::internal::field_type::SingularNumericalField::<
                f32,
                self::_puroro::tags::Float,
            > as ::std::clone::Clone>::clone(&self.float_unlabeled),
            float_repeated: <self::_puroro::internal::field_type::RepeatedNumericalField::<
                f32,
                self::_puroro::tags::Float,
            > as ::std::clone::Clone>::clone(&self.float_repeated),
            string_unlabeled: <self::_puroro::internal::field_type::SingularStringField as ::std::clone::Clone>::clone(
                &self.string_unlabeled,
            ),
            string_repeated: <self::_puroro::internal::field_type::RepeatedStringField as ::std::clone::Clone>::clone(
                &self.string_repeated,
            ),
            submsg_unlabeled: <self::_puroro::internal::field_type::SingularHeapMessageField::<
                self::_puroro_root::ser_tests3::msg::Submsg,
            > as ::std::clone::Clone>::clone(&self.submsg_unlabeled),
            submsg_repeated: <self::_puroro::internal::field_type::RepeatedMessageField::<
                self::_puroro_root::ser_tests3::msg::Submsg,
            > as ::std::clone::Clone>::clone(&self.submsg_repeated),
            enum_unlabeled: <self::_puroro::internal::field_type::SingularNumericalField::<
                self::_puroro_root::ser_tests3::Enum,
                self::_puroro::tags::Enum3::<self::_puroro_root::ser_tests3::Enum>,
            > as ::std::clone::Clone>::clone(&self.enum_unlabeled),
            enum_repeated: <self::_puroro::internal::field_type::RepeatedNumericalField::<
                self::_puroro_root::ser_tests3::Enum,
                self::_puroro::tags::Enum3::<self::_puroro_root::ser_tests3::Enum>,
            > as ::std::clone::Clone>::clone(&self.enum_repeated),
            very_large_field_number: <self::_puroro::internal::field_type::SingularNumericalField::<
                i32,
                self::_puroro::tags::Int32,
            > as ::std::clone::Clone>::clone(&self.very_large_field_number),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
impl ::std::ops::Drop for Msg {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion as _;
    }
}
#[derive(
    ::std::clone::Clone,
    ::std::marker::Copy,
    ::std::cmp::PartialEq,
    ::std::cmp::Eq,
    ::std::cmp::PartialOrd,
    ::std::cmp::Ord,
    ::std::hash::Hash,
)]
pub enum Enum {
    Zeroth,
    First,
    Tenth,
    _None(i32),
}
impl ::std::default::Default for Enum {
    fn default() -> Self {
        Self::Zeroth
    }
}
impl ::std::convert::From::<Enum> for i32 {
    fn from(val: Enum) -> i32 {
        match val {
            Enum::Zeroth => 0i32,
            Enum::First => 1i32,
            Enum::Tenth => 10i32,
            self::Enum::_None(i) => i,
        }
    }
}
impl ::std::convert::From::<i32> for Enum {
    fn from(val: i32) -> Self {
        match val {
            0i32 => self::Enum::Zeroth,
            1i32 => self::Enum::First,
            10i32 => self::Enum::Tenth,
            _ => Enum::_None(val),
        }
    }
}
