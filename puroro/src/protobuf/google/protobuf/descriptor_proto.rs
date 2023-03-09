mod _root {
    #[allow(unused)]
    pub(crate) use super::super::_root::*;
}
mod _puroro {
    #[allow(unused)]
    pub(crate) use super::_root::_puroro::*;
}
mod _pinternal {
    #[allow(unused)]
    pub(crate) use super::_root::_pinternal::*;
}
#[derive(::std::default::Default)]
#[derive(::std::cmp::PartialEq)]
pub struct ExtensionRange(
    ::std::boxed::Box<
        self::_root::google::protobuf::descriptor_proto::_view::ExtensionRangeView,
    >,
);
impl ExtensionRange {
    pub fn start_mut(&mut self) -> &mut i32 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::descriptor_proto::_view::ExtensionRangeView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.start,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_start(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::descriptor_proto::_view::ExtensionRangeView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.start,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn end_mut(&mut self) -> &mut i32 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::descriptor_proto::_view::ExtensionRangeView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.end,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_end(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::descriptor_proto::_view::ExtensionRangeView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.end,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn options_mut(
        &mut self,
    ) -> &mut self::_root::google::protobuf::ExtensionRangeOptions {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::descriptor_proto::_view::ExtensionRangeView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.options,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_options(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::descriptor_proto::_view::ExtensionRangeView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.options,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub const START_FIELD_NUMBER: i32 = 1i32;
    pub const END_FIELD_NUMBER: i32 = 2i32;
    pub const OPTIONS_FIELD_NUMBER: i32 = 3i32;
}
impl self::_puroro::Message for ExtensionRange {
    fn from_bytes_iter<I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>>(
        iter: I,
    ) -> self::_puroro::Result<Self> {
        let mut msg = <Self as ::std::default::Default>::default();
        msg.merge_from_bytes_iter(iter)?;
        ::std::result::Result::Ok(msg)
    }
    fn merge_from_bytes_iter<I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>>(
        &mut self,
        iter: I,
    ) -> self::_puroro::Result<()> {
        let mut pos_iter = self::_pinternal::PosIter::new(iter);
        let mut scoped_iter = self::_pinternal::ScopedIter::from_mut_pos_iter(
            &mut pos_iter,
        );
        <Self as self::_pinternal::MessageInternal>::merge_from_scoped_bytes_iter(
            self,
            &mut scoped_iter,
        )?;
        scoped_iter.drop_and_check_scope_completed()?;
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.start,
            self.shared.bitfield(),
            1i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.end,
            self.shared.bitfield(),
            2i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.options,
            self.shared.bitfield(),
            3i32,
            out,
        )?;
        self.shared.unknown_fields().ser_to_write(out)?;
        ::std::result::Result::Ok(())
    }
}
impl self::_pinternal::MessageInternal for ExtensionRange {
    fn merge_from_scoped_bytes_iter<
        'a,
        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
    >(
        &mut self,
        iter: &mut self::_pinternal::ScopedIter<'a, I>,
    ) -> self::_puroro::Result<()> {
        use self::_pinternal::ser::FieldData;
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        #[allow(unused)]
        use ::std::result::Result;
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        #[allow(unused)]
        use ::std::vec::Vec;
        use self::_puroro::PuroroError;
        while let Some((number, field_data))
            = FieldData::from_bytes_scoped_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    1i32 => {
                        let view_ref: &mut self::_root::google::protobuf::descriptor_proto::_view::ExtensionRangeView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.start,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    2i32 => {
                        let view_ref: &mut self::_root::google::protobuf::descriptor_proto::_view::ExtensionRangeView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.end,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    3i32 => {
                        let view_ref: &mut self::_root::google::protobuf::descriptor_proto::_view::ExtensionRangeView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.options,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    _ => {
                        let field_data = field_data
                            .map(|iter| { iter.collect::<Result<Vec<_>, _>>() })
                            .transpose()?;
                        Err(PuroroError::UnknownFieldNumber(field_data))?
                    }
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(PuroroError::UnknownFieldNumber(field_data)) => {
                    self.0.shared.unknown_fields_mut().push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
}
impl ::std::borrow::Borrow<
    self::_root::google::protobuf::descriptor_proto::_view::ExtensionRangeView,
> for ExtensionRange {
    fn borrow(
        &self,
    ) -> &self::_root::google::protobuf::descriptor_proto::_view::ExtensionRangeView {
        &self
    }
}
impl ::std::clone::Clone for ExtensionRange {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use ::std::borrow::ToOwned;
        <self::_root::google::protobuf::descriptor_proto::_view::ExtensionRangeView as ToOwned>::to_owned(
            &self,
        )
    }
}
impl ::std::fmt::Debug for ExtensionRange {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        <self::_root::google::protobuf::descriptor_proto::_view::ExtensionRangeView as ::std::fmt::Debug>::fmt(
            &self,
            fmt,
        )
    }
}
impl ::std::ops::Deref for ExtensionRange {
    type Target = self::_root::google::protobuf::descriptor_proto::_view::ExtensionRangeView;
    fn deref(&self) -> &Self::Target {
        <::std::boxed::Box<_> as ::std::ops::Deref>::deref(&self.0)
    }
}
#[derive(::std::default::Default)]
#[derive(::std::cmp::PartialEq)]
/** Range of reserved tag numbers. Reserved tag numbers may not be used by
 fields or extension ranges in the same message. Reserved ranges may
 not overlap.
*/
pub struct ReservedRange(
    ::std::boxed::Box<
        self::_root::google::protobuf::descriptor_proto::_view::ReservedRangeView,
    >,
);
impl ReservedRange {
    pub fn start_mut(&mut self) -> &mut i32 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::descriptor_proto::_view::ReservedRangeView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.start,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_start(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::descriptor_proto::_view::ReservedRangeView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.start,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn end_mut(&mut self) -> &mut i32 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::descriptor_proto::_view::ReservedRangeView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.end,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_end(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::descriptor_proto::_view::ReservedRangeView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.end,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub const START_FIELD_NUMBER: i32 = 1i32;
    pub const END_FIELD_NUMBER: i32 = 2i32;
}
impl self::_puroro::Message for ReservedRange {
    fn from_bytes_iter<I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>>(
        iter: I,
    ) -> self::_puroro::Result<Self> {
        let mut msg = <Self as ::std::default::Default>::default();
        msg.merge_from_bytes_iter(iter)?;
        ::std::result::Result::Ok(msg)
    }
    fn merge_from_bytes_iter<I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>>(
        &mut self,
        iter: I,
    ) -> self::_puroro::Result<()> {
        let mut pos_iter = self::_pinternal::PosIter::new(iter);
        let mut scoped_iter = self::_pinternal::ScopedIter::from_mut_pos_iter(
            &mut pos_iter,
        );
        <Self as self::_pinternal::MessageInternal>::merge_from_scoped_bytes_iter(
            self,
            &mut scoped_iter,
        )?;
        scoped_iter.drop_and_check_scope_completed()?;
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.start,
            self.shared.bitfield(),
            1i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.end,
            self.shared.bitfield(),
            2i32,
            out,
        )?;
        self.shared.unknown_fields().ser_to_write(out)?;
        ::std::result::Result::Ok(())
    }
}
impl self::_pinternal::MessageInternal for ReservedRange {
    fn merge_from_scoped_bytes_iter<
        'a,
        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
    >(
        &mut self,
        iter: &mut self::_pinternal::ScopedIter<'a, I>,
    ) -> self::_puroro::Result<()> {
        use self::_pinternal::ser::FieldData;
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        #[allow(unused)]
        use ::std::result::Result;
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        #[allow(unused)]
        use ::std::vec::Vec;
        use self::_puroro::PuroroError;
        while let Some((number, field_data))
            = FieldData::from_bytes_scoped_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    1i32 => {
                        let view_ref: &mut self::_root::google::protobuf::descriptor_proto::_view::ReservedRangeView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.start,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    2i32 => {
                        let view_ref: &mut self::_root::google::protobuf::descriptor_proto::_view::ReservedRangeView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.end,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    _ => {
                        let field_data = field_data
                            .map(|iter| { iter.collect::<Result<Vec<_>, _>>() })
                            .transpose()?;
                        Err(PuroroError::UnknownFieldNumber(field_data))?
                    }
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(PuroroError::UnknownFieldNumber(field_data)) => {
                    self.0.shared.unknown_fields_mut().push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
}
impl ::std::borrow::Borrow<
    self::_root::google::protobuf::descriptor_proto::_view::ReservedRangeView,
> for ReservedRange {
    fn borrow(
        &self,
    ) -> &self::_root::google::protobuf::descriptor_proto::_view::ReservedRangeView {
        &self
    }
}
impl ::std::clone::Clone for ReservedRange {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use ::std::borrow::ToOwned;
        <self::_root::google::protobuf::descriptor_proto::_view::ReservedRangeView as ToOwned>::to_owned(
            &self,
        )
    }
}
impl ::std::fmt::Debug for ReservedRange {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        <self::_root::google::protobuf::descriptor_proto::_view::ReservedRangeView as ::std::fmt::Debug>::fmt(
            &self,
            fmt,
        )
    }
}
impl ::std::ops::Deref for ReservedRange {
    type Target = self::_root::google::protobuf::descriptor_proto::_view::ReservedRangeView;
    fn deref(&self) -> &Self::Target {
        <::std::boxed::Box<_> as ::std::ops::Deref>::deref(&self.0)
    }
}
#[doc(hidden)]
pub mod _view {
    mod _root {
        #[allow(unused)]
        pub(crate) use super::super::_root::*;
    }
    mod _puroro {
        #[allow(unused)]
        pub(crate) use super::_root::_puroro::*;
    }
    mod _pinternal {
        #[allow(unused)]
        pub(crate) use super::_root::_pinternal::*;
    }
    #[derive(::std::default::Default)]
    pub struct ExtensionRangeView {
        pub(super) fields: self::_root::google::protobuf::descriptor_proto::_fields::ExtensionRangeFields::<
            self::_pinternal::OptionalNumericalField::<
                i32,
                self::_pinternal::tags::Int32,
                0usize,
            >,
            self::_pinternal::OptionalNumericalField::<
                i32,
                self::_pinternal::tags::Int32,
                1usize,
            >,
            self::_pinternal::SingularMessageField::<
                self::_root::google::protobuf::ExtensionRangeOptions,
            >,
        >,
        pub(super) shared: self::_pinternal::SharedItemsImpl<1usize>,
    }
    impl ExtensionRangeView {
        pub fn start(&self) -> i32 {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.start,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        /** Inclusive.
*/
        pub fn start_opt(&self) -> ::std::option::Option::<i32> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.start,
                self.shared.bitfield(),
            )
        }
        pub fn has_start(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.start,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn end(&self) -> i32 {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.end,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        /** Exclusive.
*/
        pub fn end_opt(&self) -> ::std::option::Option::<i32> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(&self.fields.end, self.shared.bitfield())
        }
        pub fn has_end(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(&self.fields.end, self.shared.bitfield())
                .is_some()
        }
        pub fn options(
            &self,
        ) -> ::std::option::Option::<
            &self::_root::google::protobuf::_view::ExtensionRangeOptionsView,
        > {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.options,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn options_opt(
            &self,
        ) -> ::std::option::Option::<
            &self::_root::google::protobuf::_view::ExtensionRangeOptionsView,
        > {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.options,
                self.shared.bitfield(),
            )
        }
        pub fn has_options(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.options,
                    self.shared.bitfield(),
                )
                .is_some()
        }
    }
    impl ::std::ops::Drop for ExtensionRangeView {
        fn drop(&mut self) {
            #[allow(unused)]
            use self::_pinternal::{OneofUnion as _, SharedItems as _};
        }
    }
    impl ::std::fmt::Debug for ExtensionRangeView {
        fn fmt(
            &self,
            fmt: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            let mut debug_struct = fmt.debug_struct(stringify!(ExtensionRangeView));
            debug_struct
                .field(stringify!(start), &self.start_opt())
                .field(stringify!(end), &self.end_opt())
                .field(stringify!(options), &self.options_opt());
            self.shared.unknown_fields().debug_struct_fields(&mut debug_struct)?;
            debug_struct.finish()
        }
    }
    impl ::std::cmp::PartialEq for ExtensionRangeView {
        fn eq(&self, rhs: &Self) -> bool {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::SharedItems as _;
            true && self.start_opt() == rhs.start_opt()
                && self.end_opt() == rhs.end_opt()
                && self.options_opt() == rhs.options_opt()
                && self.shared.unknown_fields() == rhs.shared.unknown_fields()
        }
    }
    impl ::std::borrow::ToOwned for ExtensionRangeView {
        type Owned = self::_root::google::protobuf::descriptor_proto::ExtensionRange;
        fn to_owned(&self) -> Self::Owned {
            #[allow(unused)]
            use self::_pinternal::SharedItems;
            self::_root::google::protobuf::descriptor_proto::ExtensionRange(
                ::std::boxed::Box::new(Self {
                    fields: self::_root::google::protobuf::descriptor_proto::_fields::ExtensionRangeFields {
                        start: ::std::clone::Clone::clone(&self.fields.start),
                        end: ::std::clone::Clone::clone(&self.fields.end),
                        options: ::std::clone::Clone::clone(&self.fields.options),
                    },
                    shared: ::std::clone::Clone::clone(&self.shared),
                }),
            )
        }
    }
    #[derive(::std::default::Default)]
    pub struct ReservedRangeView {
        pub(super) fields: self::_root::google::protobuf::descriptor_proto::_fields::ReservedRangeFields::<
            self::_pinternal::OptionalNumericalField::<
                i32,
                self::_pinternal::tags::Int32,
                0usize,
            >,
            self::_pinternal::OptionalNumericalField::<
                i32,
                self::_pinternal::tags::Int32,
                1usize,
            >,
        >,
        pub(super) shared: self::_pinternal::SharedItemsImpl<1usize>,
    }
    impl ReservedRangeView {
        pub fn start(&self) -> i32 {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.start,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        /** Inclusive.
*/
        pub fn start_opt(&self) -> ::std::option::Option::<i32> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.start,
                self.shared.bitfield(),
            )
        }
        pub fn has_start(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.start,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn end(&self) -> i32 {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.end,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        /** Exclusive.
*/
        pub fn end_opt(&self) -> ::std::option::Option::<i32> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(&self.fields.end, self.shared.bitfield())
        }
        pub fn has_end(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(&self.fields.end, self.shared.bitfield())
                .is_some()
        }
    }
    impl ::std::ops::Drop for ReservedRangeView {
        fn drop(&mut self) {
            #[allow(unused)]
            use self::_pinternal::{OneofUnion as _, SharedItems as _};
        }
    }
    impl ::std::fmt::Debug for ReservedRangeView {
        fn fmt(
            &self,
            fmt: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            let mut debug_struct = fmt.debug_struct(stringify!(ReservedRangeView));
            debug_struct
                .field(stringify!(start), &self.start_opt())
                .field(stringify!(end), &self.end_opt());
            self.shared.unknown_fields().debug_struct_fields(&mut debug_struct)?;
            debug_struct.finish()
        }
    }
    impl ::std::cmp::PartialEq for ReservedRangeView {
        fn eq(&self, rhs: &Self) -> bool {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::SharedItems as _;
            true && self.start_opt() == rhs.start_opt()
                && self.end_opt() == rhs.end_opt()
                && self.shared.unknown_fields() == rhs.shared.unknown_fields()
        }
    }
    impl ::std::borrow::ToOwned for ReservedRangeView {
        type Owned = self::_root::google::protobuf::descriptor_proto::ReservedRange;
        fn to_owned(&self) -> Self::Owned {
            #[allow(unused)]
            use self::_pinternal::SharedItems;
            self::_root::google::protobuf::descriptor_proto::ReservedRange(
                ::std::boxed::Box::new(Self {
                    fields: self::_root::google::protobuf::descriptor_proto::_fields::ReservedRangeFields {
                        start: ::std::clone::Clone::clone(&self.fields.start),
                        end: ::std::clone::Clone::clone(&self.fields.end),
                    },
                    shared: ::std::clone::Clone::clone(&self.shared),
                }),
            )
        }
    }
}
#[doc(inline)]
pub use self::_view::*;
#[doc(hidden)]
pub mod _fields {
    mod _root {
        #[allow(unused)]
        pub(crate) use super::super::_root::*;
    }
    mod _puroro {
        #[allow(unused)]
        pub(crate) use super::_root::_puroro::*;
    }
    mod _pinternal {
        #[allow(unused)]
        pub(crate) use super::_root::_pinternal::*;
    }
    #[derive(::std::default::Default)]
    pub struct ExtensionRangeFields<TStart, TEnd, TOptions> {
        pub start: TStart,
        pub end: TEnd,
        pub options: TOptions,
    }
    #[derive(::std::default::Default)]
    pub struct ReservedRangeFields<TStart, TEnd> {
        pub start: TStart,
        pub end: TEnd,
    }
}
#[doc(hidden)]
pub use self::_fields::*;
