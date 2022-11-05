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

    fn group_by_key<F, K>(&self, pred: F) -> GroupByKey<'_, T, F, K>
    where
        F: Fn(&T) -> K,
        K: PartialEq;
}

impl<T> SliceExt<T> for [T] {
    fn split_until<F>(&self, mut pred: F) -> (&[T], &[T])
    where
        F: FnMut(&T) -> bool,
    {
        let mut i = 0;
        while i < self.len() {
            if !pred(&self[i]) {
                break;
            }
            i = i + 1;
        }
        self.split_at(i)
    }

    fn group_by_key<F, K>(&self, pred: F) -> GroupByKey<'_, T, F, K>
    where
        F: Fn(&T) -> K,
        K: PartialEq,
    {
        GroupByKey::new(self, pred)
    }
}

pub struct GroupByKey<'a, T, F, K>
where
    F: Fn(&T) -> K,
    K: PartialEq,
{
    v: &'a [T],
    pred: F,
    finished: bool,
}

impl<'a, T, F, K> GroupByKey<'a, T, F, K>
where
    F: Fn(&T) -> K,
    K: PartialEq,
{
    pub fn new(slice: &'a [T], pred: F) -> Self {
        Self {
            v: slice,
            pred,
            finished: false,
        }
    }
}

impl<'a, T, F, K> Iterator for GroupByKey<'a, T, F, K>
where
    F: Fn(&T) -> K,
    K: PartialEq,
{
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        if let Some((first, rest)) = self.v.split_first() {
            let key = (self.pred)(first);
            if let Some(pos) = rest.iter().position(|x| (self.pred)(x) != key) {
                let ret = Some(&self.v[..pos + 1]);
                self.v = &self.v[pos + 1..];
                ret
            } else {
                self.finished = true;
                Some(self.v)
            }
        } else {
            self.finished = true;
            None
        }
    }
}
