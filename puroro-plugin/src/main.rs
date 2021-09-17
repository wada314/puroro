#![cfg_attr(feature = "puroro-nightly", feature(backtrace))]
#![feature(generic_associated_types)]
#![feature(arc_new_cyclic)]
#![allow(incomplete_features)]

mod error;
mod generators;
mod utils;
mod wrappers;

use ::askama::Template as _;
use ::itertools::Itertools;
use ::protobuf_compiled::google;
use ::protobuf_compiled::google::protobuf::compiler::code_generator_response::{Feature, File};
use ::protobuf_compiled::google::protobuf::compiler::{
    CodeGeneratorRequest, CodeGeneratorResponse,
};
use ::puroro::{Message, SerToIoWrite};
use ::std::collections::{HashMap, HashSet};
use ::std::env;
use ::std::io::{stdin, stdout, Read};
use ::std::process::Command;
use ::std::process::Stdio;

use error::{ErrorKind, GeneratorError};
type Result<T> = std::result::Result<T, GeneratorError>;

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
        "lib.rs".to_string()
    } else {
        package
            .split('.')
            .map(|f| utils::get_keyword_safe_ident(f))
            .join("/")
            + ".rs"
    }
}

fn format_rust_file(input: &str) -> Option<String> {
    use ::std::io::Write as _;
    if input.is_empty() {
        return None;
    }

    let rustfmt_exe = env::var("RUSTFMT").unwrap_or("rustfmt".to_string());
    let mut rustfmt = Command::new(&rustfmt_exe)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    if let Some(mut stdin) = rustfmt.stdin {
        //let input_string = input.to_string();
        //::std::thread::spawn(move || stdin.write_all(input_string.as_bytes()));
        stdin.write_all(input.as_bytes()).ok()?;
    }

    if let Some(ref mut stdout) = rustfmt.stdout {
        let mut out = String::new();
        stdout.read_to_string(&mut out).ok()?;
        if !out.is_empty() {
            return Some(out);
        }
    }

    return None;
}

fn main() -> Result<()> {
    let cgreq = CodeGeneratorRequest::from_bytes(&mut stdin().bytes()).unwrap();

    let wrapped_cgreq = wrappers::Context::try_from_proto(cgreq.clone())?;

    let mut cgres: CodeGeneratorResponse = Default::default();
    cgres.supported_features = Some(Feature::FeatureProto3Optional as u64);

    let package_to_subpackage_map = make_package_to_subpackages_map(&cgreq.proto_file);
    let package_to_file_descriptor_map = wrapped_cgreq
        .input_files()
        .iter()
        .map(|file| {
            Ok((
                file.package().iter().join("."),
                generators::MessagesAndEnums::try_new(file)?,
            ))
        })
        .collect::<Result<HashMap<_, _>>>()?;

    // merge 2 hashmaps, create a HashMap of OutputFile
    let mut output_file_contexts = HashMap::<String, generators::OutputFile>::new();
    for (package, subpackages) in package_to_subpackage_map {
        let mut v = subpackages.into_iter().collect_vec();
        v.sort();
        output_file_contexts
            .entry(package.clone())
            .or_insert_with(|| generators::OutputFile::new(&package))
            .subpackages = v;
    }
    for (package, file) in package_to_file_descriptor_map {
        output_file_contexts
            .entry(package.clone())
            .or_insert_with(|| generators::OutputFile::new(&package))
            .input_file = Some(file);
    }

    for output_contexts in output_file_contexts.values() {
        let filename = package_to_filename(&output_contexts.package);
        // Do render!
        let mut contents = output_contexts.render().unwrap();
        if let Some(new_contents) = format_rust_file(&contents) {
            contents = new_contents;
        } else {
            dbg!("failed to run rustfmt");
        }

        let mut output_file = <File as Default>::default();
        output_file.name = Some(filename.into());
        output_file.content = Some(contents.into());
        cgres.file.push(output_file);
    }

    cgres.ser(&mut stdout())?;
    Ok(())
}
