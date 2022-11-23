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

use super::util::{StrExt, WeakExt};
use super::{Enum, EnumImpl, Field, FieldImpl, InputFile, Package, PackageOrMessage};
use crate::Result;
use ::once_cell::unsync::OnceCell;
use ::proc_macro2::TokenStream;
use ::puroro_protobuf_compiled::google::protobuf::{
    DescriptorProto, EnumDescriptorProto, FieldDescriptorProto,
};
use ::quote::{format_ident, quote};
use ::std::borrow::Cow;
use ::std::fmt::Debug;
use ::std::iter;
use ::std::rc::{Rc, Weak};

pub(super) trait Message: Debug + PackageOrMessage {
    fn as_dyn_rc(self: Rc<Self>) -> Rc<dyn Message>;
    fn name(&self) -> Result<&str>;
    fn input_file(&self) -> Result<Rc<dyn InputFile>>;
    fn bitfield_size(&self) -> Result<usize>;
    fn gen_rust_struct_path(&self) -> Result<Rc<TokenStream>>;
    fn gen_struct(&self) -> Result<TokenStream>;

    fn should_generate_module_file(&self) -> Result<bool> {
        let has_submessages = self.messages()?.next().is_some();
        let has_subenums = self.enums()?.next().is_some();
        let has_oneofs = false; // TODO!
        Ok(has_submessages || has_subenums || has_oneofs)
    }
}

#[derive(Debug)]
pub(super) struct MessageImpl {
    name: String,
    fields: Vec<Rc<dyn Field>>,
    messages: Vec<Rc<dyn Message>>,
    enums: Vec<Rc<dyn Enum>>,
    input_file: Weak<dyn InputFile>,
    parent: Weak<dyn PackageOrMessage>,
    rust_module_path: OnceCell<Rc<TokenStream>>,
    rust_struct_path: OnceCell<Rc<TokenStream>>,
    module_file_dir: OnceCell<String>,
    bitfield_size: OnceCell<usize>,
}

impl MessageImpl {
    pub(super) fn new(
        proto: &DescriptorProto,
        input_file: Weak<dyn InputFile>,
        parent: Weak<dyn PackageOrMessage>,
    ) -> Rc<Self> {
        Self::new_with(
            proto,
            input_file,
            parent,
            FieldImpl::new,
            MessageImpl::new,
            EnumImpl::new,
        )
    }

    pub(super) fn new_with<FF, F, FM, M, FE, E>(
        proto: &DescriptorProto,
        input_file: Weak<dyn InputFile>,
        parent: Weak<dyn PackageOrMessage>,
        ff: FF,
        fm: FM,
        fe: FE,
    ) -> Rc<Self>
    where
        FF: Fn(&FieldDescriptorProto, Weak<dyn Message>) -> Rc<F>,
        FM: Fn(&DescriptorProto, Weak<dyn InputFile>, Weak<dyn PackageOrMessage>) -> Rc<M>,
        FE: Fn(&EnumDescriptorProto, Weak<dyn InputFile>, Weak<dyn PackageOrMessage>) -> Rc<E>,
        F: 'static + Field,
        M: 'static + Message,
        E: 'static + Enum,
    {
        let name = proto.name().to_string();
        Rc::new_cyclic(|weak_message| MessageImpl {
            name,
            input_file: Weak::clone(&input_file),
            parent,
            fields: proto
                .field()
                .into_iter()
                .filter(|f| !f.has_oneof_index() || f.has_proto3_optional())
                .map(|f| ff(f, Weak::clone(weak_message) as Weak<dyn Message>) as Rc<dyn Field>)
                .collect(),
            messages: proto
                .nested_type()
                .into_iter()
                .map(|m| {
                    fm(
                        m,
                        Weak::clone(&input_file),
                        Weak::clone(weak_message) as Weak<dyn PackageOrMessage>,
                    ) as Rc<dyn Message>
                })
                .collect(),
            enums: proto
                .enum_type()
                .into_iter()
                .map(|e| {
                    fe(
                        e,
                        Weak::clone(&input_file),
                        Weak::clone(weak_message) as Weak<dyn PackageOrMessage>,
                    ) as Rc<dyn Enum>
                })
                .collect(),
            rust_module_path: OnceCell::new(),
            rust_struct_path: OnceCell::new(),
            module_file_dir: OnceCell::new(),
            bitfield_size: OnceCell::new(),
        })
    }
}

impl PackageOrMessage for MessageImpl {
    fn messages(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<dyn Message>>>> {
        Ok(Box::new(self.messages.iter().cloned()))
    }
    fn enums(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<dyn Enum>>>> {
        Ok(Box::new(self.enums.iter().cloned()))
    }
    fn subpackages(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<dyn Package>>>> {
        Ok(Box::new(iter::empty()))
    }
    fn root_package(&self) -> Result<Rc<super::package::RootPackage>> {
        self.parent.try_upgrade()?.root_package()
    }
    fn parent(&self) -> Result<Option<Rc<dyn PackageOrMessage>>> {
        Ok(Some(self.parent.try_upgrade()?))
    }

    fn module_name(&self) -> Result<Cow<'_, str>> {
        Ok(self
            .name()?
            .to_lower_snake_case()
            .escape_rust_keywords()
            .to_string()
            .into())
    }
    fn module_file_path(&self) -> Result<Cow<'_, str>> {
        Ok(format!(
            "{}{}.rs",
            self.parent.try_upgrade()?.module_file_dir()?,
            self.name.to_lower_snake_case()
        )
        .into())
    }

    fn module_file_dir(&self) -> Result<Cow<'_, str>> {
        self.module_file_dir
            .get_or_try_init(|| {
                Ok(format!(
                    "{}{}/",
                    self.parent.try_upgrade()?.module_file_dir()?,
                    self.name.to_lower_snake_case()
                ))
            })
            .map(|s| s.into())
    }

    fn gen_rust_module_path(&self) -> Result<Rc<TokenStream>> {
        self.rust_module_path
            .get_or_try_init(|| {
                let parent = self.parent.try_upgrade()?.gen_rust_module_path()?;
                let ident = format_ident!(
                    "{}",
                    self.name()?.to_lower_snake_case().escape_rust_keywords()
                );
                Ok(Rc::new(quote! { #parent :: #ident }))
            })
            .cloned()
    }
}

impl Message for MessageImpl {
    fn as_dyn_rc(self: Rc<Self>) -> Rc<dyn Message> {
        self
    }
    fn name(&self) -> Result<&str> {
        Ok(&self.name)
    }
    fn input_file(&self) -> Result<Rc<dyn InputFile>> {
        Ok(self.input_file.try_upgrade()?)
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

    fn gen_rust_struct_path(&self) -> Result<Rc<TokenStream>> {
        self.rust_struct_path
            .get_or_try_init(|| {
                let parent = self.parent.try_upgrade()?.gen_rust_module_path()?;
                let ident =
                    format_ident!("{}", self.name()?.to_camel_case().escape_rust_keywords());
                Ok(Rc::new(quote! { #parent :: #ident }))
            })
            .cloned()
    }

    fn gen_struct(&self) -> Result<TokenStream> {
        let ident = self.gen_struct_ident()?;
        let fields = self
            .fields
            .iter()
            .map(|f| f.gen_struct_field_decl())
            .collect::<Result<Vec<_>>>()?;
        let methods = self
            .fields
            .iter()
            .map(|f| f.gen_struct_field_methods())
            .collect::<Result<Vec<_>>>()?;
        let bitfield_size_in_u32_array = (self.bitfield_size()? + 31) / 32;
        let message_impl = self.gen_struct_message_impl()?;
        let clone_impl = self.gen_struct_clone_impl()?;
        Ok(quote! {
            #[derive(::std::default::Default)]
            pub struct #ident {
                #(#fields)*
                _bitfield: self::_puroro::bitvec::BitArray<#bitfield_size_in_u32_array>,
            }

            impl #ident {
                #(#methods)*
            }

            #message_impl
            #clone_impl
        })
    }
}

impl MessageImpl {
    fn gen_struct_ident(&self) -> Result<TokenStream> {
        let ident = format_ident!(
            "{}",
            self.name.to_camel_case().escape_rust_keywords().to_string()
        );
        Ok(quote! { #ident })
    }

    fn gen_struct_message_impl(&self) -> Result<TokenStream> {
        let ident = self.gen_struct_ident()?;
        let field_data_ident = quote! { field_data };
        let deser_arms = self
            .fields
            .iter()
            .map(|f| f.gen_struct_field_deser_arm(&field_data_ident))
            .collect::<Result<Vec<_>>>()?;

        Ok(quote! {
            impl self::_puroro::Message for #ident {
                fn from_bytes_iter<I: ::std::iter::Iterator<Item=::std::io::Result<u8>>>(iter: I) -> self::_puroro::Result<Self> {
                    let mut msg = <Self as ::std::default::Default>::default();
                    msg.merge_from_bytes_iter(iter)?;
                    ::std::result::Result::Ok(msg)
                }

                fn merge_from_bytes_iter<I: ::std::iter::Iterator<Item =::std::io::Result<u8>>>(&mut self, mut iter: I) -> self::_puroro::Result<()> {
                    use self::_puroro::internal::ser::FieldData;
                    while let Some((number, #field_data_ident)) = FieldData::from_bytes_iter(iter.by_ref())? {
                        match number {
                            #(#deser_arms)*
                            _ => todo!(), // Unknown field number
                        }
                    }
                    ::std::result::Result::Ok(())
                }

                fn to_bytes<W: ::std::io::Write>(&self, #[allow(unused)] out: &mut W) -> self::_puroro::Result<()> {
                    #[allow(unused)]
                    use ::std::result::Result::Ok;
                    Ok(todo!())
                }
            }
        })
    }

    fn gen_struct_clone_impl(&self) -> Result<TokenStream> {
        let ident = self.gen_struct_ident()?;
        let field_clones = self
            .fields
            .iter()
            .map(|f| f.gen_struct_field_clone_arm())
            .collect::<Result<Vec<_>>>()?;
        Ok(quote! {
            impl ::std::clone::Clone for #ident {
                fn clone(&self) -> Self {
                    Self {
                        #(#field_clones)*
                        _bitfield: ::std::clone::Clone::clone(&self._bitfield),
                    }
                }
            }
        })
    }
}
