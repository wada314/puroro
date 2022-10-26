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

#[derive(Debug, Clone)]
enum ContextItem<'a> {
    Package(&'a Package),
    File(&'a File),
    Message(&'a Message),
}
struct Context<'a> {
    items: Vec<ContextItem<'a>>,
}
impl<'a> Context<'a> {
    fn packages(&self) -> impl '_ + Iterator<Item = &'a Package> {
        self.items.iter().cloned().filter_map(|ci| {
            if let ContextItem::Package(p) = ci {
                Some(p)
            } else {
                None
            }
        })
    }
    fn files(&self) -> impl '_ + Iterator<Item = &'a File> {
        self.items.iter().cloned().filter_map(|ci| {
            if let ContextItem::File(f) = ci {
                Some(f)
            } else {
                None
            }
        })
    }
    fn messages(&self) -> impl '_ + Iterator<Item = &'a Message> {
        self.items.iter().cloned().filter_map(|ci| {
            if let ContextItem::Message(m) = ci {
                Some(m)
            } else {
                None
            }
        })
    }
    fn push_package_then<F: FnOnce(&mut Self) -> R, R>(&mut self, package: &'a Package, f: F) -> R {
        self.items.push(ContextItem::Package(package));
        let r = f(self);
        self.items.pop();
        r
    }
    fn push_file_then<F: FnOnce(&mut Self) -> R, R>(&mut self, file: &'a File, f: F) -> R {
        self.items.push(ContextItem::File(file));
        let r = f(self);
        self.items.pop();
        r
    }
    fn push_message_then<F: FnOnce(&mut Self) -> R, R>(&mut self, message: &'a Message, f: F) -> R {
        self.items.push(ContextItem::Message(message));
        let r = f(self);
        self.items.pop();
        r
    }
}

#[derive(Debug)]
pub struct Package {}

#[derive(Debug)]
pub struct File {}

#[derive(Debug)]
pub struct Message {}
