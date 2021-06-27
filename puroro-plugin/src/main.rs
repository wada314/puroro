#![cfg_attr(feature = "puroro-nightly", feature(backtrace))]
#![cfg_attr(feature = "puroro-nightly", feature(generic_associated_types))]
#![cfg_attr(feature = "puroro-nightly", feature(min_type_alias_impl_trait))]
#![allow(incomplete_features)]

mod error;
mod protos;
mod utils;
mod wrappers;

use ::puroro::Serializable;
use ::puroro_internal::deser::MergeableMessageFromIter;
use itertools::Itertools;

use error::GeneratorError;
type Result<T> = std::result::Result<T, GeneratorError>;

use std::collections::{HashMap, HashSet};
use std::io::Read;
use std::io::{stdin, stdout};

use google::protobuf::compiler::code_generator_response::File;
use google::protobuf::compiler::{CodeGeneratorRequest, CodeGeneratorResponse};
use google::protobuf::FileDescriptorProto;
pub use protos::simple::google;

use askama::Template;

fn make_package_to_subpackages_map(
    files: &Vec<google::protobuf::FileDescriptorProto>,
) -> HashMap<String, HashSet<String>> {
    let mut map = HashMap::new();
    for file in files {
        let package_string = file.package.clone().unwrap_or_default();
        let package_vec = package_string
            .split('.')
            .filter_map(|p| {
                if p.is_empty() {
                    None
                } else {
                    Some(p.to_string())
                }
            })
            .collect::<Vec<_>>();
        for i in 0..(package_vec.len()) {
            let cur_package = package_vec.iter().take(i).join(".");
            let subpackage = package_vec[i].clone();
            map.entry(cur_package)
                .or_insert(HashSet::new())
                .insert(subpackage);
        }
    }
    map
}

fn package_to_filename(package: &str) -> String {
    if package.is_empty() {
        "mod.rs".to_string()
    } else {
        package
            .split('.')
            .map(|f| utils::get_keyword_safe_ident(f))
            .join("/")
            + ".rs"
    }
}

#[derive(Template, Default)]
#[template(path = "mod.rs.txt")]
pub struct Variables {
    subpackages: HashSet<String>,
    file: Option<FileDescriptorProto>,
}
mod filters {
    pub fn unwrap_or_default<T>(val: &Option<T>) -> ::askama::Result<T>
    where
        T: Default + Clone,
    {
        Ok(val.clone().unwrap_or_default())
    }
}

fn main() -> Result<()> {
    let mut cgreq = CodeGeneratorRequest::default();
    cgreq.merge_from_iter(&mut stdin().bytes()).unwrap();
    let mut cgres = CodeGeneratorResponse::default();
    let mut mod_rs = File::default();
    mod_rs.name = Some("mod.rs".to_string());
    let package_to_subpackage_map = make_package_to_subpackages_map(&cgreq.proto_file);
    let package_to_file_descriptor_map = cgreq
        .proto_file
        .into_iter()
        .map(|file| (file.package.clone().unwrap_or_default(), file))
        .collect::<HashMap<_, _>>();
    // merge 2 hashmaps
    let mut package_to_variables = HashMap::<String, Variables>::new();
    for (package, subpackages) in package_to_subpackage_map {
        package_to_variables
            .entry(package)
            .or_insert_with(Default::default)
            .subpackages = subpackages;
    }
    for (package, file) in package_to_file_descriptor_map {
        package_to_variables
            .entry(package)
            .or_insert_with(Default::default)
            .file = Some(file);
    }

    for (package, variables) in package_to_variables {
        let filename = package_to_filename(&package);
        let contents = variables.render().unwrap();

        let mut output_file = <File as Default>::default();
        output_file.name = Some(filename);
        output_file.content = Some(contents);
        cgres.file.push(output_file);
    }

    cgres.serialize(&mut stdout())?;
    Ok(())
}
