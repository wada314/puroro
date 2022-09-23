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

use ::std::convert::TryFrom;
use ::tests_pb::enum2::Enum as Enum2;
use ::tests_pb::enum3::Enum as Enum3;

#[test]
fn test_enum2_default() {
    assert_eq!(Enum2::ValueSeven, Enum2::default());
}

#[test]
fn test_enum2_convert() {
    assert_eq!(Some(Enum2::ValueSeven), Enum2::try_from(7).ok());
    assert_eq!(Some(Enum2::ValueZero), Enum2::try_from(0).ok());
    assert_eq!(Some(Enum2::ValueFourtyTwo), Enum2::try_from(42).ok());
    assert!(Enum2::try_from(12345).is_err());
}
