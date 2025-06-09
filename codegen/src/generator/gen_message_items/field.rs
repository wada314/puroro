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

use crate::Result;
use crate::cases::{Case, convert_into_case};
use crate::descriptor::{FieldDescriptorExt, FieldType, LenType};
use crate::generator::{CodeGeneratorOptions, FieldPresense, avoid_reserved_keywords, to_ident};
use crate::proto_path::{ProtoPath, ProtoPathBuf};
use ::culpa::throws;
use ::quote::quote;
use ::std::rc::Rc;
use ::syn::{Ident, Lifetime, Signature, Type, parse_quote, parse2};

type Error = crate::ErrorKind;

pub enum FieldContext {
    Trait,
    Struct,
}

#[derive(Debug)]
pub enum Field {
    Repeated(RepeatedField),
    Explicit(ScalarField),
    Implicit(ScalarField),
}

#[derive(Debug)]
pub struct RepeatedField {
    pub number: i32,
    pub options: Rc<CodeGeneratorOptions>,
    /// The protobuf-style absolute path of the parent scope (package or enclosing message),
    /// used as the base for generating relative paths to other items.
    pub base_proto_path: Rc<ProtoPathBuf>,
    pub scalar_proto_type: FieldType<ProtoPathBuf, ProtoPathBuf>,
    pub getter_signatures: GetterSignatures,
}

#[derive(Debug)]
pub struct ScalarField {
    pub number: i32,
    pub options: Rc<CodeGeneratorOptions>,
    pub base_proto_path: Rc<ProtoPathBuf>,
    pub scalar_proto_type: FieldType<ProtoPathBuf, ProtoPathBuf>,
    pub getter_signatures: GetterSignatures,
    pub has_method_signatures: HasMethodSignatures,
}

#[derive(Debug)]
pub struct GetterSignatures {
    pub trait_getter: Signature,
    pub struct_getter: Signature,
    pub trait_try_getter: Signature,
    pub struct_try_getter: Signature,
}

#[derive(Debug)]
pub struct HasMethodSignatures {
    pub has_method: Signature,
    pub try_has_method: Signature,
}

impl Field {
    #[throws]
    pub fn try_new<'a>(
        desc: &'a FieldDescriptorExt<'a>,
        current_proto_path: Rc<ProtoPathBuf>,
        options: Rc<CodeGeneratorOptions>,
    ) -> Self {
        FieldFactory::new(desc, current_proto_path, options)?.build()?
    }

    pub fn number(&self) -> i32 {
        match self {
            Field::Repeated(RepeatedField { number, .. })
            | Field::Explicit(ScalarField { number, .. })
            | Field::Implicit(ScalarField { number, .. }) => *number,
        }
    }

    pub fn base_proto_path(&self) -> &Rc<ProtoPathBuf> {
        match self {
            Field::Repeated(RepeatedField { base_proto_path, .. })
            | Field::Explicit(ScalarField { base_proto_path, .. })
            | Field::Implicit(ScalarField { base_proto_path, .. }) => base_proto_path,
        }
    }

    pub fn scalar_proto_type(&self) -> &FieldType<ProtoPathBuf, ProtoPathBuf> {
        match self {
            Field::Repeated(RepeatedField { scalar_proto_type, .. })
            | Field::Explicit(ScalarField { scalar_proto_type, .. })
            | Field::Implicit(ScalarField { scalar_proto_type, .. }) => scalar_proto_type,
        }
    }

    pub fn getter_signatures(&self) -> &GetterSignatures {
        match self {
            Field::Repeated(RepeatedField { getter_signatures, .. })
            | Field::Explicit(ScalarField { getter_signatures, .. })
            | Field::Implicit(ScalarField { getter_signatures, .. }) => getter_signatures,
        }
    }
}

struct FieldFactory {
    number: i32,
    base_proto_path: Rc<ProtoPathBuf>,
    current_proto_path: Rc<ProtoPathBuf>,
    options: Rc<CodeGeneratorOptions>,
    lower_cased: String,
    presense: FieldPresense,
    scalar_proto_type: FieldType<ProtoPathBuf, ProtoPathBuf>,
}

impl FieldFactory {
    #[throws]
    pub fn new(
        desc: &FieldDescriptorExt,
        current_proto_path: Rc<ProtoPathBuf>,
        options: Rc<CodeGeneratorOptions>,
    ) -> Self {
        let lower_cased = convert_into_case(&desc.name(), Case::LowerSnakeCase);

        let presense = FieldPresense::from_field_desc(desc);
        let scalar_proto_type = desc.type_with_full_path()?;
        let number = desc.number();
        let base_proto_path = Rc::clone(&current_proto_path);
        Self {
            number,
            base_proto_path,
            current_proto_path,
            options,
            lower_cased,
            presense,
            scalar_proto_type,
        }
    }

    #[throws]
    pub fn build(self) -> Field {
        let scalar_proto_type = self.scalar_proto_type.clone();
        let number = self.number;
        let base_proto_path = Rc::clone(&self.base_proto_path);
        let options = Rc::clone(&self.options);
        match &self.presense {
            FieldPresense::Implicit => Field::Implicit(ScalarField {
                number,
                options,
                base_proto_path,
                scalar_proto_type,
                getter_signatures: self.gen_getter_signatures()?,
                has_method_signatures: self.gen_has_method_signatures()?,
            }),
            FieldPresense::Explicit => Field::Explicit(ScalarField {
                number,
                options,
                base_proto_path,
                scalar_proto_type,
                getter_signatures: self.gen_getter_signatures()?,
                has_method_signatures: self.gen_has_method_signatures()?,
            }),
            FieldPresense::Repeated => Field::Repeated(RepeatedField {
                number,
                options,
                base_proto_path,
                scalar_proto_type,
                getter_signatures: self.gen_getter_signatures()?,
            }),
        }
    }

    fn gen_getter_signature(&self, context: FieldContext, is_try: bool) -> Signature {
        let name = if is_try {
            to_ident(&format!("try_{}", &self.lower_cased))
        } else {
            to_ident(&format!("{}", &self.lower_cased))
        };
        let scalar_ref_type: Type = gen_scalar_maybe_ref_type(
            &self.scalar_proto_type,
            &self.base_proto_path,
            &self.options,
            |path| {
                let view_trait_path = path
                    .to_rust_path_with(&self.options, |name| {
                        if is_try {
                            Ok(gen_try_view_trait_name(name).into())
                        } else {
                            Ok(gen_view_trait_name(name).into())
                        }
                    })
                    .expect("This should not happen -- maybe needs further refactoring");
                match context {
                    FieldContext::Trait => parse_quote! { impl #view_trait_path },
                    FieldContext::Struct => {
                        let struct_path = path
                            .to_rust_path_with(&self.options, |name| {
                                Ok(gen_struct_name(name).into())
                            })
                            .expect("This should not happen -- maybe needs further refactoring");
                        parse_quote! { #struct_path :: <impl #view_trait_path> }
                    }
                }
            },
        );
        let return_type: Type = match self.presense {
            FieldPresense::Repeated => {
                let item_type = if is_try {
                    self.options.puroro_result_type(&scalar_ref_type)
                } else {
                    scalar_ref_type
                };
                let repeated_view_trait = self.options.puroro_repeated_view_trait(&item_type);
                let result_inner_type = parse_quote! {
                    impl #repeated_view_trait
                };
                // Do we really need to wrap the repeated view trait in a result type?
                // We have already wrapped the item type in a result type, and we can easily convert
                // the `Result<Iterator<Result<T>>>` to `Result<Iterator<T>>` by using `quither` crate.
                if is_try {
                    self.options.puroro_result_type(&result_inner_type)
                } else {
                    result_inner_type
                }
            }
            FieldPresense::Explicit | FieldPresense::Implicit => {
                let result_inner_type = match self.scalar_proto_type {
                    FieldType::Message(_) => self.options.option_type(&scalar_ref_type),
                    _ => scalar_ref_type,
                };
                let result_type = if is_try {
                    self.options.puroro_result_type(&result_inner_type)
                } else {
                    result_inner_type
                };
                result_type
            }
        };
        parse_quote! {
            fn #name(&self) -> #return_type
        }
    }

    #[throws]
    fn gen_getter_signatures(&self) -> GetterSignatures {
        GetterSignatures {
            trait_getter: self.gen_getter_signature(FieldContext::Trait, false),
            struct_getter: self.gen_getter_signature(FieldContext::Struct, false),
            trait_try_getter: self.gen_getter_signature(FieldContext::Trait, true),
            struct_try_getter: self.gen_getter_signature(FieldContext::Struct, true),
        }
    }

    fn gen_has_method_signature(&self, is_try: bool) -> Signature {
        if let FieldPresense::Repeated = self.presense {
            panic!("has method is not allowed for repeated fields");
        }
        let name = if is_try {
            to_ident(&format!("try_has_{}", &self.lower_cased))
        } else {
            to_ident(&format!("has_{}", &self.lower_cased))
        };
        let bool_type = self.options.primitive_type("bool");
        let return_type = if is_try {
            self.options.puroro_result_type(&bool_type)
        } else {
            bool_type
        };
        parse_quote! {
            fn #name(&self) -> #return_type
        }
    }

    #[throws]
    fn gen_has_method_signatures(&self) -> HasMethodSignatures {
        HasMethodSignatures {
            has_method: self.gen_has_method_signature(false),
            try_has_method: self.gen_has_method_signature(true),
        }
    }
}

fn gen_scalar_maybe_ref_type<M, E, F>(
    field_type: &FieldType<M, E>,
    current_path: &ProtoPath,
    options: &CodeGeneratorOptions,
    gen_message_type: F,
) -> Type
where
    M: AsRef<ProtoPath>,
    E: AsRef<ProtoPath>,
    F: FnOnce(&ProtoPath) -> Type,
{
    match field_type
        .as_ref()
        .maybe_into_primitive_type(current_path, options)
    {
        Ok(primitive_type) => primitive_type,
        Err(len_type) => match len_type {
            LenType::Message(path) => {
                let path = path
                    .as_ref()
                    .to_relative_path(current_path)
                    .unwrap_or(path.as_ref());
                gen_message_type(&path)
            }
            LenType::String => {
                let str_type = options.primitive_type("str");
                parse_quote! { &#str_type }
            }
            LenType::Bytes => {
                let u8_type = options.primitive_type("u8");
                parse_quote! { &[#u8_type] }
            }
        },
    }
}

fn gen_view_trait_name(message_name: &str) -> Ident {
    to_ident(&format!(
        "{}View",
        convert_into_case(message_name, Case::CamelCase)
    ))
}

fn gen_try_view_trait_name(message_name: &str) -> Ident {
    to_ident(&format!(
        "Try{}View",
        convert_into_case(message_name, Case::CamelCase)
    ))
}

fn gen_struct_name(message_name: &str) -> Ident {
    to_ident(&convert_into_case(message_name, Case::CamelCase))
}
