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

use super::{Address, AddressImpl, AddressMut};

#[test]
fn test_address_impl_standalone() {
    let mut addr = AddressImpl::new();
    addr.set_street("123 Main St");
    assert_eq!(addr.street(), "123 Main St");
    assert_eq!(addr.city(), "");
    assert_eq!(addr.zip_code(), 0);

    addr.clear_street();
    assert_eq!(addr.street(), "");
}
