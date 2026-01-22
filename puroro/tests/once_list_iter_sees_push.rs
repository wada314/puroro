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

use ::once_list2::OnceListWithTailLen as OnceList;
#[test]
fn once_list_iter_sees_push_after_exhausted() {
    let list = OnceList::<i32>::new();
    list.push(1);

    let mut it = list.iter();
    assert_eq!(it.next().copied(), Some(1));
    assert_eq!(it.next().copied(), None);

    // After the iterator reached the end, pushing a new element should make it visible
    // from the same iterator. This property is relied upon by `LazyRepeatedIter`.
    list.push(2);
    assert_eq!(it.next().copied(), Some(2));
    assert_eq!(it.next().copied(), None);
}
