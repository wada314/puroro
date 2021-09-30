rem Copyright 2021 Google LLC
rem
rem Licensed under the Apache License, Version 2.0 (the "License");
rem you may not use this file except in compliance with the License.
rem You may obtain a copy of the License at
rem
rem      http://www.apache.org/licenses/LICENSE-2.0
rem
rem Unless required by applicable law or agreed to in writing, software
rem distributed under the License is distributed on an "AS IS" BASIS,
rem WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
rem See the License for the specific language governing permissions and
rem limitations under the License.

cargo build -p puroro-plugin && ^
rmdir .\protobuf-compiled\src /S /Q && ^
mkdir .\protobuf-compiled\src
call %PURORO_PROTOC_PATH% ^
    protobuf/src/google/protobuf/compiler/plugin.proto ^
    --plugin=protoc-gen-rust=./target/debug/puroro-plugin.exe ^
    --rust_out=./protobuf-compiled/src/ ^
    --proto_path=./protobuf/src/