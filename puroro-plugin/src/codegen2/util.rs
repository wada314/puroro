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

pub trait SliceExt<T> {
    fn split_until<F>(&self, pred: F) -> (&[T], &[T])
    where
        F: FnMut(&T) -> bool;
}

impl<T> SliceExt<T> for [T] {
    fn split_until<F>(&self, pred: F) -> (&[T], &[T])
    where
        F: FnMut(&T) -> bool,
    {
        let i = 0;
        while i < self.len() {
            if !pred(&self[i]) {
                break;
            }
        }
        self.split_at(i)
    }
}
