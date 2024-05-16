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

use ::ipc_channel::ipc::{IpcBytesReceiver, IpcBytesSender, IpcOneShotServer};
use ::thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorKind {
    #[error("IpcError: {0}")]
    IpcError(#[from] ::ipc_channel::Error),
    #[error("IoError: {0}")]
    IoError(#[from] ::std::io::Error),
}
pub type Result<T> = ::std::result::Result<T, ErrorKind>;

pub struct Protoc {
    protoc_path: String,
    out_dir: String,
    proto_files: Vec<String>,
}

impl Protoc {
    pub fn protoc_path(mut self, path: &str) -> Self {
        self.protoc_path = path.to_string();
        self
    }
    pub fn out_dir(mut self, path: &str) -> Self {
        self.out_dir = path.to_string();
        self
    }
    pub fn proto_file(mut self, path: &str) -> Self {
        self.proto_files.push(path.to_string());
        self
    }

    pub fn run(self) -> Result<()> {
        let (ipc_init_server, ipc_init_name) = IpcOneShotServer::new()?;
        // process
        let (req_recv, res_send): (IpcBytesReceiver, IpcBytesSender) = ipc_init_server.accept()?.1;
        todo!();
        Ok(())
    }
}
