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
struct Context {
    packages: Vec<Rc<Package>>,
    files: Vec<Rc<File>>,
    messages: Vec<Rc<Message>>,
}
impl Context {
    fn packages(&self) -> &[Rc<Package>] {
        &self.packages
    }
    fn files(&self) -> &[Rc<File>] {
        &self.files
    }
    fn messages(&self) -> &[Rc<Message>] {
        &&self.messages
    }
    fn push_package_then<F: FnOnce(&mut Self) -> R, R>(&mut self, package: Rc<Package>, f: F) -> R {
        self.packages.push(package);
        let r = f(self);
        self.packages.pop();
        r
    }
    fn push_file_then<F: FnOnce(&mut Self) -> R, R>(&mut self, file: Rc<File>, f: F) -> R {
        self.files.push(file);
        let r = f(self);
        self.files.pop();
        r
    }
    fn push_message_then<F: FnOnce(&mut Self) -> R, R>(&mut self, message: Rc<Message>, f: F) -> R {
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
