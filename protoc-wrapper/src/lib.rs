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
use ::std::process::{Command, ExitStatus};
use ::std::time::Duration;
#[cfg(feature = "on-memory")]
use ::tempfile::TempDir;
use ::thiserror::Error;
use ::wait_timeout::ChildExt;

const PLUGIN_PATH: &'static str = env!("CARGO_BIN_FILE_PURORO_PLUGIN");

#[derive(Error, Debug)]
pub enum ErrorKind {
    #[error("IpcIpcError: {0}")]
    IpcIpcError(#[from] ::ipc_channel::ipc::IpcError),
    #[error("IpcError: {0}")]
    IpcError(#[from] ::ipc_channel::Error),
    #[error("IoError: {0}")]
    IoError(#[from] ::std::io::Error),
    #[error("PuroroError: {0}")]
    PuroroError(#[from] ::puroro::ErrorKind),
    #[error("CallbackError: {0}")]
    CallbackError(String),
    #[error("ProtocTimeoutError")]
    ProtocTimeoutError,
    #[error("ProtocProcessError: {0}")]
    ProtocProcessError(ExitStatus),
    #[cfg(feature = "on-memory")]
    #[error("FileNameError")]
    FileNameError,
}
pub type Result<T> = ::std::result::Result<T, ErrorKind>;

pub struct Protoc {
    protoc_path: String,
    out_dir: Option<String>,
    proto_files: Vec<String>,
    proto_paths: Vec<String>,
}

impl Protoc {
    pub fn new() -> Self {
        Self {
            protoc_path: "protoc".to_string(),
            out_dir: None,
            proto_files: Vec::new(),
            proto_paths: Vec::new(),
        }
    }
    pub fn protoc_path(mut self, path: &str) -> Self {
        self.protoc_path = path.to_string();
        self
    }
    pub fn out_dir(mut self, path: &str) -> Self {
        self.out_dir = Some(path.to_string());
        self
    }
    pub fn proto_file(mut self, path: &str) -> Self {
        self.proto_files.push(path.to_string());
        self
    }
    pub fn proto_files<I>(mut self, paths: I) -> Self
    where
        I: IntoIterator<Item = String>,
    {
        self.proto_files.extend(paths);
        self
    }
    pub fn proto_path(mut self, path: &str) -> Self {
        self.proto_paths.push(path.to_string());
        self
    }

    pub fn run<F>(self, body: F) -> Result<()>
    where
        F: FnOnce(CodeGeneratorRequest) -> ::std::result::Result<CodeGeneratorResponse, String>,
    {
        let (ipc_init_server, ipc_init_name) = IpcOneShotServer::new()?;

        let mut process = Command::new(&self.protoc_path)
            .args(&[
                format!("--plugin=protoc-gen-puroro={}", PLUGIN_PATH),
                format!("--puroro_out={}", self.out_dir.unwrap_or(".".to_string())),
                format!("--puroro_opt={}", ipc_init_name),
            ])
            .args(
                self.proto_paths
                    .iter()
                    .map(|x| format!("--proto_path={}", x)),
            )
            .args(&self.proto_files)
            .spawn()?;

        {
            // revieve the ipc channels from the plugin exe.
            let (req_recv, res_send): (IpcBytesReceiver, IpcBytesSender) =
                ipc_init_server.accept()?.1;

            let req = CodeGeneratorRequest::deser_from_read(req_recv.recv()?.as_slice())?;
            let res = (body)(req).map_err(|x| ErrorKind::CallbackError(x))?;

            let mut res_bytes = Vec::new();
            res.write(&mut res_bytes)?;
            res_send.send(&res_bytes)?;
        }

        let Some(exit_code) = process.wait_timeout(Duration::from_secs(1))? else {
            return Err(ErrorKind::ProtocTimeoutError);
        };
        if !exit_code.success() {
            return Err(ErrorKind::ProtocProcessError(exit_code));
        }

        Ok(())
    }
}

#[cfg(feature = "on-memory")]
pub struct ProtocOnMemory {
    protoc: Protoc,
    in_files: Vec<(String, String)>,
}

impl ProtocOnMemory {
    pub fn new() -> Self {
        Self {
            protoc: Protoc::new(),
            in_files: Vec::new(),
        }
    }
    pub fn protoc_path(mut self, path: &str) -> Self {
        self.protoc = self.protoc.protoc_path(path);
        self
    }
    pub fn add_file(mut self, name: &str, content: &str) -> Self {
        self.in_files.push((name.to_string(), content.to_string()));
        self
    }
    pub fn add_files<I>(mut self, files: I) -> Self
    where
        I: IntoIterator<Item = (String, String)>,
    {
        self.in_files.extend(files);
        self
    }

    pub fn run<F>(self, func: F) -> Result<Vec<(String, String)>>
    where
        F: FnOnce(CodeGeneratorRequest) -> ::std::result::Result<CodeGeneratorResponse, String>,
    {
        let proto_dir = TempDir::new()?;
        let out_dir = TempDir::new()?;
        let out_dir_path = out_dir.path().to_str().unwrap().to_string();

        // write the proto files to the temp dir.
        for (name, content) in &self.in_files {
            let path = proto_dir.path().join(&name);
            ::std::fs::write(&path, &content)?;
        }

        // run the protoc
        let proto_file_paths = self
            .in_files
            .iter()
            .map(|(name, _)| {
                proto_dir
                    .path()
                    .join(name)
                    .to_str()
                    .ok_or(ErrorKind::FileNameError)
                    .map(str::to_string)
            })
            .collect::<Result<Vec<_>>>()?;

        self.protoc
            .out_dir(&out_dir_path)
            .proto_path(proto_dir.path().to_str().unwrap())
            .proto_files(proto_file_paths)
            .run(func)?;

        // read the generated files
        let output_files = ::std::fs::read_dir(out_dir.path())?
            .map(|entry| -> Result<_> {
                let path = entry?.path();
                let name = path
                    .strip_prefix(out_dir.path())
                    .map_err(|_| ErrorKind::FileNameError)?
                    .to_str()
                    .ok_or(ErrorKind::FileNameError)?
                    .to_string();
                let content = ::std::fs::read_to_string(&path)?;
                Ok((name, content))
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(output_files)
    }
}
