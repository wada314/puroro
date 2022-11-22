// Copyright 2021 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::util::WeakExt;
use super::{
    Enum, EnumTrait, Field, FieldTrait, InputFileTrait, MessageOrEnum, PackageOrMessage,
    PackageTrait,
};
use crate::codegen::utils::StrExt;
use crate::Result;
use ::once_cell::unsync::OnceCell;
use ::proc_macro2::TokenStream;
use ::puroro_protobuf_compiled::google::protobuf::{
    DescriptorProto, EnumDescriptorProto, FieldDescriptorProto,
};
use ::quote::{format_ident, quote};
use ::std::fmt::Debug;
use ::std::rc::{Rc, Weak};

pub(super) trait MessageTrait: Debug {
    fn input_file(&self) -> Result<Rc<dyn InputFileTrait>>;
    fn parent(&self) -> Result<PackageOrMessage<Rc<dyn PackageTrait>, Rc<dyn MessageTrait>>>;
    fn bitfield_size(&self) -> Result<usize>;
    fn name(&self) -> &str;
    fn messages(&self) -> Result<&[Rc<dyn MessageTrait>]>;
    fn enums(&self) -> Result<&[Rc<dyn EnumTrait>]>;
    fn gen_struct(&self) -> Result<TokenStream>;
    fn gen_rust_module_path(&self) -> Result<Rc<TokenStream>>;
    fn gen_rust_struct_path(&self) -> Result<Rc<TokenStream>>;
    fn as_dyn_rc(self: Rc<Self>) -> Rc<dyn MessageTrait>;

    fn should_generate_module_file(&self) -> Result<bool> {
        let has_submessages = !self.messages()?.is_empty();
        let has_subenums = !self.enums()?.is_empty();
        let has_oneofs = false; // TODO!
        Ok(has_submessages || has_subenums || has_oneofs)
    }
    fn resolve_type_name(
        self: Rc<Self>,
        type_name: &str,
    ) -> Result<MessageOrEnum<Rc<dyn MessageTrait>, Rc<dyn EnumTrait>>> {
        PackageOrMessage::Message(self.as_dyn_rc()).resolve_type_name(type_name)
    }
}

#[derive(Debug)]
pub(super) struct Message {
    name: String,
    fields: Vec<Rc<dyn FieldTrait>>,
    messages: Vec<Rc<dyn MessageTrait>>,
    enums: Vec<Rc<dyn EnumTrait>>,
    input_file: Weak<dyn InputFileTrait>,
    parent: PackageOrMessage<Weak<dyn PackageTrait>, Weak<dyn MessageTrait>>,
    rust_module_path: OnceCell<Rc<TokenStream>>,
    rust_struct_path: OnceCell<Rc<TokenStream>>,

    bitfield_size: OnceCell<usize>,
}

impl Message {
    pub(super) fn new(
        proto: &DescriptorProto,
        input_file: Weak<dyn InputFileTrait>,
        parent: PackageOrMessage<Weak<dyn PackageTrait>, Weak<dyn MessageTrait>>,
    ) -> Rc<Self> {
        Self::new_with(
            proto,
            input_file,
            parent,
            Field::new,
            Message::new,
            Enum::new,
        )
    }

    pub(super) fn new_with<FF, F, FM, M, FE, E>(
        proto: &DescriptorProto,
        input_file: Weak<dyn InputFileTrait>,
        parent: PackageOrMessage<Weak<dyn PackageTrait>, Weak<dyn MessageTrait>>,
        ff: FF,
        fm: FM,
        fe: FE,
    ) -> Rc<Self>
    where
        FF: Fn(&FieldDescriptorProto, Weak<dyn MessageTrait>) -> Rc<F>,
        FM: Fn(
            &DescriptorProto,
            Weak<dyn InputFileTrait>,
            PackageOrMessage<Weak<dyn PackageTrait>, Weak<dyn MessageTrait>>,
        ) -> Rc<M>,
        FE: Fn(
            &EnumDescriptorProto,
            Weak<dyn InputFileTrait>,
            PackageOrMessage<Weak<dyn PackageTrait>, Weak<dyn MessageTrait>>,
        ) -> Rc<E>,
        F: 'static + FieldTrait,
        M: 'static + MessageTrait,
        E: 'static + EnumTrait,
    {
        let name = proto.name().to_string();
        Rc::new_cyclic(|weak_message| Message {
            name,
            input_file: Weak::clone(&input_file),
            parent,
            fields: proto
                .field()
                .into_iter()
                .filter(|f| !f.has_oneof_index() || f.has_proto3_optional())
                .map(|f| {
                    ff(f, Weak::clone(weak_message) as Weak<dyn MessageTrait>) as Rc<dyn FieldTrait>
                })
                .collect(),
            messages: proto
                .nested_type()
                .into_iter()
                .map(|m| {
                    fm(
                        m,
                        Weak::clone(&input_file),
                        PackageOrMessage::Message(
                            Weak::clone(weak_message) as Weak<dyn MessageTrait>
                        ),
                    ) as Rc<dyn MessageTrait>
                })
                .collect(),
            enums: proto
                .enum_type()
                .into_iter()
                .map(|e| {
                    fe(
                        e,
                        Weak::clone(&input_file),
                        PackageOrMessage::Message(
                            Weak::clone(weak_message) as Weak<dyn MessageTrait>
                        ),
                    ) as Rc<dyn EnumTrait>
                })
                .collect(),
            rust_module_path: OnceCell::new(),
            rust_struct_path: OnceCell::new(),
            bitfield_size: OnceCell::new(),
        })
    }
}

impl MessageTrait for Message {
    fn name(&self) -> &str {
        &self.name
    }

    fn gen_struct(&self) -> Result<TokenStream> {
        let ident = format_ident!(
            "{}",
            self.name.to_camel_case().escape_rust_keywords().to_string()
        );
        let fields = self
            .fields
            .iter()
            .map(|f| f.gen_struct_field_decl())
            .collect::<Result<Vec<_>>>()?;
        let bitfield_size_in_u32_array = (self.bitfield_size()? + 31) / 32;
        Ok(quote! {
            pub struct #ident {
                #(#fields)*
                _bitfield: self::_puroro::bitvec::BitArray<#bitfield_size_in_u32_array>,
            }
        })
    }
    fn input_file(&self) -> Result<Rc<dyn InputFileTrait>> {
        Ok(self.input_file.try_upgrade()?)
    }
    fn parent(&self) -> Result<PackageOrMessage<Rc<dyn PackageTrait>, Rc<dyn MessageTrait>>> {
        Ok(match &self.parent {
            PackageOrMessage::Package(p) => PackageOrMessage::Package(p.try_upgrade()?),
            PackageOrMessage::Message(m) => PackageOrMessage::Message(m.try_upgrade()?),
        })
    }

    fn bitfield_size(&self) -> Result<usize> {
        self.bitfield_size
            .get_or_try_init(|| {
                let mut tail = 0;
                for field in &self.fields {
                    if let Some(next_tail) = field.maybe_allocated_bitfield_tail()? {
                        tail = next_tail;
                    } else {
                        tail = field.assign_and_get_bitfield_tail(tail)?;
                    }
                }
                Ok(tail)
            })
            .cloned()
    }

    fn messages(&self) -> Result<&[Rc<dyn MessageTrait>]> {
        Ok(&self.messages)
    }

    fn enums(&self) -> Result<&[Rc<dyn EnumTrait>]> {
        Ok(&self.enums)
    }

    fn gen_rust_module_path(&self) -> Result<Rc<TokenStream>> {
        self.rust_module_path
            .get_or_try_init(|| {
                let parent = self.parent()?.gen_rust_module_path()?;
                let ident = format_ident!(
                    "{}",
                    self.name().to_lower_snake_case().escape_rust_keywords()
                );
                Ok(Rc::new(quote! { #parent :: #ident }))
            })
            .cloned()
    }

    fn gen_rust_struct_path(&self) -> Result<Rc<TokenStream>> {
        self.rust_struct_path
            .get_or_try_init(|| {
                let parent = self.parent()?.gen_rust_module_path()?;
                let ident = format_ident!("{}", self.name().to_camel_case().escape_rust_keywords());
                Ok(Rc::new(quote! { #parent :: #ident }))
            })
            .cloned()
    }

    fn as_dyn_rc(self: Rc<Self>) -> Rc<dyn MessageTrait> {
        self
    }
}
