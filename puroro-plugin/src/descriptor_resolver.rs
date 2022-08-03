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

use super::descriptor_ext::{FileDescriptorExt, MessageOrEnum};
use crate::descriptor_ext::FileOrMessage;
use crate::{ErrorKind, Result};
use ::itertools::Itertools;
use ::puroro_protobuf_compiled::google::protobuf::{
    DescriptorProto, EnumDescriptorProto, FileDescriptorProto,
};
use ::std::collections::HashMap;

#[derive(Debug)]
pub struct DescriptorResolver<'a> {
    fqtn_to_desc_map: HashMap<String, &'a dyn MessageOrEnum>,
    package_contents: HashMap<String, PackageContents<'a>>,
}
impl<'a> DescriptorResolver<'a> {
    pub fn new<I>(file_descriptors_iter: I) -> Result<Self>
    where
        I: Iterator<Item = &'a FileDescriptorProto>,
    {
        let mut fqtn_to_desc_map = HashMap::new();
        let mut package_contents: HashMap<_, PackageContents> = HashMap::new();
        for f in file_descriptors_iter {
            // package_contents for parent packages
            for (cur_package, subpackage) in f.package_ext().packages_and_subpackages() {
                let item = package_contents
                    .entry(cur_package.full_package_path().to_string())
                    .or_insert_with(|| PackageContents {
                        package_name: cur_package.leaf_package_name().map(|s| s.to_string()),
                        full_package: cur_package.full_package_path().to_string(),
                        subpackages: Vec::new(),
                        input_files: Vec::new(),
                    });
                item.subpackages.push(subpackage.to_string());
            }

            // package_contents for the leaf package
            let term_item = package_contents.entry(f.package().to_string()).or_default();
            term_item.package_name = f.package_ext().leaf_package_name().map(|s| s.to_string());
            term_item.full_package = f.package().to_string();
            term_item.input_files.push(f);
        }
        Ok(Self {
            fqtn_to_desc_map,
            package_contents,
        })
    }

    pub fn package_contents(&self, package: &str) -> Option<&PackageContents> {
        self.package_contents.get(package)
    }

    pub fn package_contents_or_err(&self, package: &str) -> Result<&PackageContents> {
        Ok(self
            .package_contents(package)
            .ok_or(ErrorKind::FqtnNotFound {
                fqtn: package.into(),
            })?)
    }

    #[allow(unused)]
    pub fn all_packages(&self) -> impl Iterator<Item = &PackageContents> {
        dbg!(&self.package_contents);
        self.package_contents.values()
    }
}

#[derive(Debug, Default)]
pub struct PackageContents<'a> {
    pub package_name: Option<String>,
    pub full_package: String,
    pub subpackages: Vec<String>,
    pub input_files: Vec<&'a FileDescriptorProto>,
}

// need test
fn visit_messages_and_enums<VM, VE>(
    file: &FileDescriptorProto,
    mut visit_message: VM,
    mut visit_enum: VE,
) where
    VM: FnMut(&DescriptorProto, &[&DescriptorProto]),
    VE: FnMut(&EnumDescriptorProto, &[&DescriptorProto]),
{
    let mut path = Vec::new();
    let mut iters_queue = Vec::new();
    iters_queue.push(file.messages().into_iter());
    while let Some(mut tail_iter) = iters_queue.pop() {
        if let Some(next_leaf) = tail_iter.next() {
            iters_queue.push(tail_iter);
            iters_queue.push(next_leaf.messages().into_iter());
            path.push(next_leaf);
        } else {
            let f_or_m: &dyn FileOrMessage = path.pop().map_or(file, |m| m);
            for m in f_or_m.messages() {
                visit_message(m, &path);
            }
            for e in f_or_m.enums() {
                visit_enum(e, &path);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::DescriptorProto as DP;
    use super::EnumDescriptorProto as EDP;
    use super::FileDescriptorProto as FDP;
    use super::*;
    use ::std::collections::HashSet;

    #[test]
    fn test_visit() {
        let mut enum1 = EDP::default();
        *enum1.name_mut() = "e1".to_string();
        let mut msg1 = DP::default();
        *msg1.name_mut() = "m1".to_string();
        let mut enum11 = EDP::default();
        *enum11.name_mut() = "e11".to_string();
        msg1.enum_type_mut().push(enum11);
        let mut msg11 = DP::default();
        msg1.nested_type_mut().push(msg11);
        let mut file = FDP::default();
        file.message_type_mut().push(msg1);
        file.enum_type_mut().push(enum1);

        let mut visited_msgs = HashSet::new();
        let mut visited_enums = HashSet::new();
        visit_messages_and_enums(
            &file,
            |m, path| {
                assert!(visited_msgs.insert(format!(
                    "{}::{}",
                    path.iter().map(|item| item.name()).join("."),
                    m.name(),
                )))
            },
            |e, path| {
                assert!(visited_enums.insert(format!(
                    "{}::{}",
                    path.iter().map(|item| item.name()).join("."),
                    e.name(),
                )))
            },
        );

        // assert!(visited_msgs.contains("::m1"));
        // assert!(visited_msgs.contains("m1::m11"));
        // assert_eq!(2usize, visited_msgs.len());
        assert_eq!(
            ["::m1", "m1::m11"]
                .into_iter()
                .map(str::to_string)
                .collect::<HashSet<_>>(),
            visited_msgs
        );

        assert!(visited_enums.contains("::e1"));
        assert!(visited_enums.contains("m1::e11"));
        assert_eq!(2usize, visited_enums.len());
    }
}
