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

use ::anyhow::Result;
use ::ipc_channel::ipc::{bytes_channel, IpcSender};
use ::puroro::google::protobuf::compiler::CodeGeneratorRequest;
use ::puroro::message::MessageLite;
use ::std::io::{stdin, stdout, Read, Write};

fn main() -> Result<()> {
    let mut input_buffer = Vec::new();
    stdin().read_to_end(&mut input_buffer)?;
    let request = CodeGeneratorRequest::deser_from_read(input_buffer.as_slice()).unwrap();

    let ipc_init_key = request.parameter()?.unwrap_or_default();
    let ipc_init_send = IpcSender::connect(ipc_init_key.to_string())?;
    let (req_send, req_recv) = bytes_channel()?;
    let (res_send, res_recv) = bytes_channel()?;
    ipc_init_send.send((req_recv, res_send))?;

    req_send.send(&input_buffer)?;
    let response = res_recv.recv()?;

    stdout().write_all(&response)?;

    Ok(())
}
