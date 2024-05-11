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
// limitations under the License.<

use ::std::env::args;

fn main() {
    let mut out_dir = String::new();
    let mut params = Vec::new();
    let mut proto_files = Vec::new();

    for arg in args().skip(1) {
        if arg.starts_with("--out_dir=") {
            out_dir = arg.split_at("--out_dir=".len()).1.to_string();
        } else if arg.starts_with("--parameter=") {
            let comma_separated_params = arg.split_at("--parameter=".len()).1;
            for param in comma_separated_params.split(',') {
                params.push(param.to_string());
            }
        } else {
            proto_files.push(arg.to_string());
        }
    }

    println!("Hello, world!");
}
