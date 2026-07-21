//! Fake emitter for a single field-less message.

use crate::ir::{MessageDesc, ProtoFile};

/// Render one Rust source file for an empty message.
pub(super) fn render(proto: &ProtoFile, message: &MessageDesc) -> String {
    let name = message.name.as_str();
    let mut out = String::new();

    out.push_str(&format!(
        "//! @generated from {} — do not edit\n",
        proto.name
    ));
    if !proto.package.is_empty() {
        out.push_str(&format!("//! Package `{}`\n", proto.package));
    }
    out.push('\n');
    out.push_str(
        "// Fake empty-message output from protoc-gen-puroro.\n\
         // Field catalog emission is not implemented yet.\n\n",
    );
    out.push_str(
        "type __Presence = ::bitvec::array::BitArray<[u8; 0], ::bitvec::order::Lsb0>;\n\n",
    );

    out.push_str(&format!(
        r#"pub struct {name}<
    A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone =
        ::allocator_api2::alloc::Global,
> {{
    _common: ::puroro_rt::MessageCommon<__Presence, A>,
}}

impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> {name}<A> {{
    pub fn new_in(alloc: A) -> Self {{
        Self {{
            _common: ::puroro_rt::MessageCommon::new_in(
                ::bitvec::array::BitArray::ZERO,
                alloc,
            ),
        }}
    }}

    pub fn visit_fields<V: ::puroro_rt::FieldVisitor<__Presence, A>>(
        &self,
        _v: &mut V,
    ) -> ::core::ops::ControlFlow<V::Break> {{
        ::core::ops::ControlFlow::Continue(())
    }}

    pub fn visit_field_pairs<V: ::puroro_rt::FieldPairVisitor<__Presence, A>>(
        &self,
        _other: &Self,
        _v: &mut V,
    ) -> ::core::ops::ControlFlow<V::Break> {{
        ::core::ops::ControlFlow::Continue(())
    }}

    pub fn visit_field_pairs_mut<V: ::puroro_rt::FieldPairVisitorMut<__Presence, A>>(
        &self,
        _dst: &mut Self,
        _v: &mut V,
    ) -> ::core::ops::ControlFlow<V::Break> {{
        ::core::ops::ControlFlow::Continue(())
    }}

    pub fn visit_fields_mut<V: ::puroro_rt::FieldVisitorMut<__Presence, A>>(
        &mut self,
        _v: &mut V,
    ) -> ::core::ops::ControlFlow<V::Break> {{
        ::core::ops::ControlFlow::Continue(())
    }}
}}

impl {name}<::allocator_api2::alloc::Global> {{
    pub fn new() -> Self {{
        Self::new_in(::allocator_api2::alloc::Global)
    }}
}}

impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone + ::core::default::Default>
    ::core::default::Default for {name}<A>
{{
    fn default() -> Self {{
        Self::new_in(A::default())
    }}
}}

impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> ::puroro_rt::CloneIn<A>
    for {name}<A>
{{
    fn clone_in(&self, alloc: A) -> Self {{
        let mut dst = Self::new_in(alloc.clone());
        let mut v = ::puroro_rt::CloneFieldsVisitor::new(&self._common, &dst._common);
        let _ = self.visit_field_pairs_mut(&mut dst, &mut v);
        let mut old = ::core::mem::replace(&mut dst._common, self._common.clone_in(alloc));
        old.deallocate();
        dst
    }}
}}

impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> ::core::clone::Clone
    for {name}<A>
{{
    #[inline]
    fn clone(&self) -> Self {{
        ::puroro_rt::CloneIn::clone_in(self, self._common.alloc.clone())
    }}
}}

impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> ::core::cmp::PartialEq
    for {name}<A>
{{
    fn eq(&self, other: &Self) -> bool {{
        matches!(
            self.visit_field_pairs(
                other,
                &mut ::puroro_rt::FieldEqVisitor::new(&self._common, &other._common),
            ),
            ::core::ops::ControlFlow::Continue(())
        ) && self._common.unknown_fields_eq(&other._common)
    }}
}}

impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> ::core::fmt::Debug
    for {name}<A>
{{
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {{
        let mut v = ::puroro_rt::DebugStructVisitor::new(f.debug_struct("{name}"), &self._common);
        let _ = self.visit_fields(&mut v);
        v.finish()
    }}
}}

impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> ::core::ops::Drop for {name}<A> {{
    fn drop(&mut self) {{
        let mut v = ::puroro_rt::FieldDeallocVisitor::new(&self._common);
        let _ = self.visit_fields_mut(&mut v);
        self._common.deallocate();
    }}
}}

impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> ::puroro_rt::DeallocateIn<A>
    for {name}<A>
{{
    #[inline]
    unsafe fn deallocate_in(self, _alloc: A) {{
        ::core::mem::drop(self);
    }}
}}

impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> ::puroro::Message for {name}<A> {{
    type Alloc = A;

    fn new_in(alloc: A) -> Self {{
        Self::new_in(alloc)
    }}

    fn encoded_len(&self) -> usize {{
        let mut v = ::puroro_rt::EncodedLenVisitor::new(&self._common);
        let _ = self.visit_fields(&mut v);
        v.len + self._common.unknown_fields.len()
    }}

    fn encode_raw<B: ::bytes::BufMut>(&self, buf: &mut B) {{
        let _ = self.visit_fields(&mut ::puroro_rt::EncodeRawVisitor::new(&self._common, buf));
        let unknown: &[u8] = &self._common.unknown_fields;
        ::bytes::BufMut::put_slice(buf, unknown);
    }}

    fn merge_from_with_depth<B: ::bytes::Buf>(
        &mut self,
        buf: &mut B,
        depth: usize,
    ) -> ::core::result::Result<(), ::puroro::DecodeError> {{
        if depth >= ::puroro::RECURSION_LIMIT {{
            return ::core::result::Result::Err(::puroro::DecodeError::RecursionLimitExceeded);
        }}
        while ::bytes::Buf::has_remaining(buf) {{
            let (field_number, wire_type) = ::puroro_rt::decode::decode_tag(buf)?;
            // No known fields — preserve everything as unknown.
            ::puroro_rt::decode::skip_field_and_save(
                field_number,
                wire_type,
                buf,
                &mut self._common.unknown_fields,
                self._common.alloc.clone(),
            )?;
        }}
        ::core::result::Result::Ok(())
    }}

    fn unknown_fields(&self) -> impl ::core::iter::Iterator<Item = ::puroro::UnknownField<'_>> + '_ {{
        self._common.iter_unknown_fields()
    }}

    fn validate(&self) -> ::core::result::Result<(), ::puroro::DecodeError> {{
        ::core::result::Result::Ok(())
    }}
}}
"#
    ));

    out
}
