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

mod package;
use ::std::rc::Rc;
use package::Package;

#[derive(Debug, Default)]
struct Context<'a> {
    packages: Vec<&'a Package>,
    file: Option<&'a File>,
    messages: Vec<&'a Message>,
}
impl<'a> Context<'a> {
    fn packages(&self) -> &[&'a Package] {
        &self.packages
    }
    fn file(&self) -> Option<&'a File> {
        self.file
    }
    fn messages(&self) -> &[&'a Message] {
        &self.messages
    }
    fn push_package_then<F: FnOnce(&mut Self) -> R, R>(&mut self, package: &'a Package, f: F) -> R {
        self.packages.push(package);
        let r = f(self);
        self.packages.pop();
        r
    }
    fn push_file_then<F: FnOnce(&mut Self) -> R, R>(&mut self, file: &'a File, f: F) -> R {
        self.file = Some(file);
        let r = f(self);
        self.file = None;
        r
    }
    fn push_message_then<F: FnOnce(&mut Self) -> R, R>(&mut self, message: &'a Message, f: F) -> R {
        self.messages.push(message);
        let r = f(self);
        self.messages.pop();
        r
    }
}

#[derive(Debug)]
pub struct File {}

#[derive(Debug)]
pub struct Message {}

#[derive(Debug)]
pub struct Enum {}
