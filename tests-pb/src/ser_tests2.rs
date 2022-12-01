pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}
pub mod _puroro {
    pub use ::puroro::*;
}
pub mod msg;
#[derive(::std::default::Default)]
pub struct Msg {
    i32_optional: self::_puroro::internal::field_type::OptionalNumericalField::<
        i32,
        self::_puroro::tags::Int32,
        0usize,
    >,
    i32_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        i32,
        self::_puroro::tags::Int32,
    >,
    float_optional: self::_puroro::internal::field_type::OptionalNumericalField::<
        f32,
        self::_puroro::tags::Float,
        1usize,
    >,
    float_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        f32,
        self::_puroro::tags::Float,
    >,
    string_optional: self::_puroro::internal::field_type::OptionalStringField::<2usize>,
    string_repeated: self::_puroro::internal::field_type::RepeatedStringField,
    submsg_optional: self::_puroro::internal::field_type::SingularHeapMessageField::<
        self::_puroro_root::ser_tests2::msg::Submsg,
    >,
    submsg_repeated: self::_puroro::internal::field_type::RepeatedMessageField::<
        self::_puroro_root::ser_tests2::msg::Submsg,
    >,
    enum_optional: self::_puroro::internal::field_type::OptionalNumericalField::<
        self::_puroro_root::ser_tests2::Enum,
        self::_puroro::tags::Enum2::<self::_puroro_root::ser_tests2::Enum>,
        3usize,
    >,
    enum_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        self::_puroro_root::ser_tests2::Enum,
        self::_puroro::tags::Enum2::<self::_puroro_root::ser_tests2::Enum>,
    >,
    very_large_field_number: self::_puroro::internal::field_type::OptionalNumericalField::<
        i32,
        self::_puroro::tags::Int32,
        4usize,
    >,
    _bitfield: self::_puroro::bitvec::BitArray<1usize>,
}
impl Msg {
    pub fn i32_optional(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::get_field(
            &self.i32_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn i32_optional_opt(&self) -> ::std::option::Option::<i32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.i32_optional, &self._bitfield)
    }
    pub fn i32_optional_mut(&mut self) -> &mut i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.i32_optional,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_i32_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.i32_optional, &self._bitfield)
            .is_some()
    }
    pub fn clear_i32_optional(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::clear(&mut self.i32_optional, &mut self._bitfield)
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
    pub fn float_optional(&self) -> f32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            f32,
            self::_puroro::tags::Float,
            1usize,
        > as NonRepeatedFieldType>::get_field(
            &self.float_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn float_optional_opt(&self) -> ::std::option::Option::<f32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            f32,
            self::_puroro::tags::Float,
            1usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.float_optional, &self._bitfield)
    }
    pub fn float_optional_mut(&mut self) -> &mut f32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            f32,
            self::_puroro::tags::Float,
            1usize,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.float_optional,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_float_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            f32,
            self::_puroro::tags::Float,
            1usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.float_optional, &self._bitfield)
            .is_some()
    }
    pub fn clear_float_optional(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            f32,
            self::_puroro::tags::Float,
            1usize,
        > as NonRepeatedFieldType>::clear(&mut self.float_optional, &mut self._bitfield)
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
    pub fn string_optional(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            2usize,
        > as NonRepeatedFieldType>::get_field(
            &self.string_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn string_optional_opt(&self) -> ::std::option::Option::<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            2usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.string_optional, &self._bitfield)
    }
    pub fn string_optional_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            2usize,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.string_optional,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_string_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            2usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.string_optional, &self._bitfield)
            .is_some()
    }
    pub fn clear_string_optional(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            2usize,
        > as NonRepeatedFieldType>::clear(&mut self.string_optional, &mut self._bitfield)
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
    pub fn submsg_optional(
        &self,
    ) -> ::std::option::Option::<&self::_puroro_root::ser_tests2::msg::Submsg> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField::<
            self::_puroro_root::ser_tests2::msg::Submsg,
        > as NonRepeatedFieldType>::get_field(
            &self.submsg_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn submsg_optional_opt(
        &self,
    ) -> ::std::option::Option::<&self::_puroro_root::ser_tests2::msg::Submsg> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField::<
            self::_puroro_root::ser_tests2::msg::Submsg,
        > as NonRepeatedFieldType>::get_field_opt(&self.submsg_optional, &self._bitfield)
    }
    pub fn submsg_optional_mut(
        &mut self,
    ) -> &mut self::_puroro_root::ser_tests2::msg::Submsg {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField::<
            self::_puroro_root::ser_tests2::msg::Submsg,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.submsg_optional,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_submsg_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField::<
            self::_puroro_root::ser_tests2::msg::Submsg,
        > as NonRepeatedFieldType>::get_field_opt(&self.submsg_optional, &self._bitfield)
            .is_some()
    }
    pub fn clear_submsg_optional(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField::<
            self::_puroro_root::ser_tests2::msg::Submsg,
        > as NonRepeatedFieldType>::clear(&mut self.submsg_optional, &mut self._bitfield)
    }
    pub fn submsg_repeated(&self) -> &[self::_puroro_root::ser_tests2::msg::Submsg] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField::<
            self::_puroro_root::ser_tests2::msg::Submsg,
        > as RepeatedFieldType>::get_field(&self.submsg_repeated, &self._bitfield)
    }
    pub fn submsg_repeated_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_puroro_root::ser_tests2::msg::Submsg> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField::<
            self::_puroro_root::ser_tests2::msg::Submsg,
        > as RepeatedFieldType>::mut_field(
            &mut self.submsg_repeated,
            &mut self._bitfield,
        )
    }
    pub fn clear_submsg_repeated(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField::<
            self::_puroro_root::ser_tests2::msg::Submsg,
        > as RepeatedFieldType>::clear(&mut self.submsg_repeated, &mut self._bitfield)
    }
    pub fn enum_optional(&self) -> self::_puroro_root::ser_tests2::Enum {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            self::_puroro_root::ser_tests2::Enum,
            self::_puroro::tags::Enum2::<self::_puroro_root::ser_tests2::Enum>,
            3usize,
        > as NonRepeatedFieldType>::get_field(
            &self.enum_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn enum_optional_opt(
        &self,
    ) -> ::std::option::Option::<self::_puroro_root::ser_tests2::Enum> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            self::_puroro_root::ser_tests2::Enum,
            self::_puroro::tags::Enum2::<self::_puroro_root::ser_tests2::Enum>,
            3usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.enum_optional, &self._bitfield)
    }
    pub fn enum_optional_mut(&mut self) -> &mut self::_puroro_root::ser_tests2::Enum {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            self::_puroro_root::ser_tests2::Enum,
            self::_puroro::tags::Enum2::<self::_puroro_root::ser_tests2::Enum>,
            3usize,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.enum_optional,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_enum_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            self::_puroro_root::ser_tests2::Enum,
            self::_puroro::tags::Enum2::<self::_puroro_root::ser_tests2::Enum>,
            3usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.enum_optional, &self._bitfield)
            .is_some()
    }
    pub fn clear_enum_optional(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            self::_puroro_root::ser_tests2::Enum,
            self::_puroro::tags::Enum2::<self::_puroro_root::ser_tests2::Enum>,
            3usize,
        > as NonRepeatedFieldType>::clear(&mut self.enum_optional, &mut self._bitfield)
    }
    pub fn enum_repeated(&self) -> &[self::_puroro_root::ser_tests2::Enum] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            self::_puroro_root::ser_tests2::Enum,
            self::_puroro::tags::Enum2::<self::_puroro_root::ser_tests2::Enum>,
        > as RepeatedFieldType>::get_field(&self.enum_repeated, &self._bitfield)
    }
    pub fn enum_repeated_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_puroro_root::ser_tests2::Enum> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            self::_puroro_root::ser_tests2::Enum,
            self::_puroro::tags::Enum2::<self::_puroro_root::ser_tests2::Enum>,
        > as RepeatedFieldType>::mut_field(&mut self.enum_repeated, &mut self._bitfield)
    }
    pub fn clear_enum_repeated(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            self::_puroro_root::ser_tests2::Enum,
            self::_puroro::tags::Enum2::<self::_puroro_root::ser_tests2::Enum>,
        > as RepeatedFieldType>::clear(&mut self.enum_repeated, &mut self._bitfield)
    }
    pub fn very_large_field_number(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            4usize,
        > as NonRepeatedFieldType>::get_field(
            &self.very_large_field_number,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn very_large_field_number_opt(&self) -> ::std::option::Option::<i32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            4usize,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.very_large_field_number,
            &self._bitfield,
        )
    }
    pub fn very_large_field_number_mut(&mut self) -> &mut i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            4usize,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.very_large_field_number,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_very_large_field_number(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            4usize,
        > as NonRepeatedFieldType>::get_field_opt(
                &self.very_large_field_number,
                &self._bitfield,
            )
            .is_some()
    }
    pub fn clear_very_large_field_number(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            4usize,
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
                    <self::_puroro::internal::field_type::OptionalNumericalField::<
                        i32,
                        self::_puroro::tags::Int32,
                        0usize,
                    > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.i32_optional,
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
                    <self::_puroro::internal::field_type::OptionalNumericalField::<
                        f32,
                        self::_puroro::tags::Float,
                        1usize,
                    > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.float_optional,
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
                    <self::_puroro::internal::field_type::OptionalStringField::<
                        2usize,
                    > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.string_optional,
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
                        self::_puroro_root::ser_tests2::msg::Submsg,
                    > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.submsg_optional,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                8i32 => {
                    <self::_puroro::internal::field_type::RepeatedMessageField::<
                        self::_puroro_root::ser_tests2::msg::Submsg,
                    > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.submsg_repeated,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                9i32 => {
                    <self::_puroro::internal::field_type::OptionalNumericalField::<
                        self::_puroro_root::ser_tests2::Enum,
                        self::_puroro::tags::Enum2::<
                            self::_puroro_root::ser_tests2::Enum,
                        >,
                        3usize,
                    > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.enum_optional,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                10i32 => {
                    <self::_puroro::internal::field_type::RepeatedNumericalField::<
                        self::_puroro_root::ser_tests2::Enum,
                        self::_puroro::tags::Enum2::<
                            self::_puroro_root::ser_tests2::Enum,
                        >,
                    > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.enum_repeated,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                536870911i32 => {
                    <self::_puroro::internal::field_type::OptionalNumericalField::<
                        i32,
                        self::_puroro::tags::Int32,
                        4usize,
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
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            0usize,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.i32_optional,
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
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            f32,
            self::_puroro::tags::Float,
            1usize,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.float_optional,
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
        <self::_puroro::internal::field_type::OptionalStringField::<
            2usize,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.string_optional,
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
            self::_puroro_root::ser_tests2::msg::Submsg,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.submsg_optional,
            &self._bitfield,
            7i32,
            out,
        )?;
        <self::_puroro::internal::field_type::RepeatedMessageField::<
            self::_puroro_root::ser_tests2::msg::Submsg,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.submsg_repeated,
            &self._bitfield,
            8i32,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            self::_puroro_root::ser_tests2::Enum,
            self::_puroro::tags::Enum2::<self::_puroro_root::ser_tests2::Enum>,
            3usize,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.enum_optional,
            &self._bitfield,
            9i32,
            out,
        )?;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            self::_puroro_root::ser_tests2::Enum,
            self::_puroro::tags::Enum2::<self::_puroro_root::ser_tests2::Enum>,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.enum_repeated,
            &self._bitfield,
            10i32,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            4usize,
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
            i32_optional: <self::_puroro::internal::field_type::OptionalNumericalField::<
                i32,
                self::_puroro::tags::Int32,
                0usize,
            > as ::std::clone::Clone>::clone(&self.i32_optional),
            i32_repeated: <self::_puroro::internal::field_type::RepeatedNumericalField::<
                i32,
                self::_puroro::tags::Int32,
            > as ::std::clone::Clone>::clone(&self.i32_repeated),
            float_optional: <self::_puroro::internal::field_type::OptionalNumericalField::<
                f32,
                self::_puroro::tags::Float,
                1usize,
            > as ::std::clone::Clone>::clone(&self.float_optional),
            float_repeated: <self::_puroro::internal::field_type::RepeatedNumericalField::<
                f32,
                self::_puroro::tags::Float,
            > as ::std::clone::Clone>::clone(&self.float_repeated),
            string_optional: <self::_puroro::internal::field_type::OptionalStringField::<
                2usize,
            > as ::std::clone::Clone>::clone(&self.string_optional),
            string_repeated: <self::_puroro::internal::field_type::RepeatedStringField as ::std::clone::Clone>::clone(
                &self.string_repeated,
            ),
            submsg_optional: <self::_puroro::internal::field_type::SingularHeapMessageField::<
                self::_puroro_root::ser_tests2::msg::Submsg,
            > as ::std::clone::Clone>::clone(&self.submsg_optional),
            submsg_repeated: <self::_puroro::internal::field_type::RepeatedMessageField::<
                self::_puroro_root::ser_tests2::msg::Submsg,
            > as ::std::clone::Clone>::clone(&self.submsg_repeated),
            enum_optional: <self::_puroro::internal::field_type::OptionalNumericalField::<
                self::_puroro_root::ser_tests2::Enum,
                self::_puroro::tags::Enum2::<self::_puroro_root::ser_tests2::Enum>,
                3usize,
            > as ::std::clone::Clone>::clone(&self.enum_optional),
            enum_repeated: <self::_puroro::internal::field_type::RepeatedNumericalField::<
                self::_puroro_root::ser_tests2::Enum,
                self::_puroro::tags::Enum2::<self::_puroro_root::ser_tests2::Enum>,
            > as ::std::clone::Clone>::clone(&self.enum_repeated),
            very_large_field_number: <self::_puroro::internal::field_type::OptionalNumericalField::<
                i32,
                self::_puroro::tags::Int32,
                4usize,
            > as ::std::clone::Clone>::clone(&self.very_large_field_number),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
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
        }
    }
}
impl ::std::convert::TryFrom::<i32> for Enum {
    type Error = self::_puroro::PuroroError;
    fn try_from(val: i32) -> ::std::result::Result<Self, Self::Error> {
        match val {
            0i32 => ::std::result::Result::Ok(self::Enum::Zeroth),
            1i32 => ::std::result::Result::Ok(self::Enum::First),
            10i32 => ::std::result::Result::Ok(self::Enum::Tenth),
            _ => {
                ::std::result::Result::Err(
                    self::_puroro::ErrorKind::UnknownEnumVariant(val),
                )?
            }
        }
    }
}
