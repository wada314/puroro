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
use ::puroro::google::protobuf::compiler::{CodeGeneratorRequest, CodeGeneratorResponse};
use ::puroro::message::MessageLite;
use ::std::env;
use ::std::process::Command;
use ::thiserror::Error;

const PLUGIN_PATH: &'static str = env!("CARGO_BIN_FILE_PURORO_PLUGIN");

#[derive(Error, Debug)]
pub enum ErrorKind {
    #[error("IpcError: {0}")]
    IpcError(#[from] ::ipc_channel::ipc::IpcError),
    #[error("IoError: {0}")]
    IoError(#[from] ::std::io::Error),
    #[error("PuroroError: {0}")]
    PuroroError(#[from] ::puroro::ErrorKind),
}
pub type Result<T> = ::std::result::Result<T, ErrorKind>;

pub struct Protoc {
    protoc_path: String,
    out_dir: String,
    proto_files: Vec<String>,
    proto_paths: Vec<String>,
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
    pub fn proto_path(mut self, path: &str) -> Self {
        self.proto_paths.push(path.to_string());
        self
    }

    pub fn run(self) -> Result<()> {
        let (ipc_init_server, ipc_init_name) = IpcOneShotServer::new()?;

        let process = Command::new(&self.protoc_path)
            .args(&[
                format!("--plugin=puroro={}", PLUGIN_PATH),
                format!("--puroro_out={}", self.out_dir),
                format!("--puroro_opt={}", ipc_init_name),
            ])
            .args(
                self.proto_paths
                    .iter()
                    .map(|x| format!("--proto_path={}", x)),
            )
            .args(&self.proto_files)
            .spawn()?;

        // revieve the ipc channels from the plugin exe.
        let (req_recv, res_send): (IpcBytesReceiver, IpcBytesSender) = ipc_init_server.accept()?.1;

        let req = CodeGeneratorRequest::deser_from_read(req_recv.recv()?.as_slice())?;

        todo!();
        Ok(())
    }
}
