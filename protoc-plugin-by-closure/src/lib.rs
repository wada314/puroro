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

//! A library for running protoc command with your own plugin code as a closure.
//!
//! When you develop your own protoc compiler plugin, normally you need to compile your code as
//! an executable binary and run `protoc` command by yourself with passing the path to the binary.
//! This crate provides a convenient replacement for that process: You just need to pass your
//! code as a closure to this library, then it will run the `protoc` command as like as your closure
//! is an executable binary.
//!
//! # How it works
//!
//! This crate is using the `ipc-channel` crate for the communication between the internal plugin
//! binary and your closure code.
//! When running the `protoc` command, this crate generates an one-time key for the IPC channel,
//! and pass it to the internal plugin binary as a `protoc` argument.
//! The internal plugin binary then creates the IPC channel with the key, and sends the whole
//! input `CodeGeneratorRequest` bytes back to the caller process.
//! The generated `CodeGeneratorResponse` bytes are sent back to the internal plugin binary in the
//! same way, and the binary outputs that bytes to the stdout.
//!
//! # Features
//!
//! - `on-memory`: Enabled by default.
//! Provides [`ProtocOnMemory`] struct which makes you to run the `protoc` command without touching
//! the actual filesystem. Because this feature is using the `tempfile` crate, you can disable it
//! if you don't need it.
//!
//! # Requirements
//!
//! Nightly channel required.
//!
//! This crate requires an unstable (as of 2024/06)
//! [cargo feature `bindeps`](https://rust-lang.github.io/rfcs/3028-cargo-binary-dependencies.html).
//! This crate contains [.cargo/config.toml](.cargo/config.toml) file which enables this feature
//! so you normally don't need to worry about it, but if you include this crate as a part of
//! your [Cargo workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html),
//! then you need to create the `.cargo/config.toml` file with the following contents
//! *under your cargo workspace directory* manually
//! (see [the official document](https://doc.rust-lang.org/cargo/reference/config.html) for the reasons):
//!
//! ```toml
//! [unstable]
//! bindeps = true
//! ```

use ::ipc_channel::ipc::{IpcBytesReceiver, IpcBytesSender, IpcOneShotServer};
use ::std::env;
use ::std::path::PathBuf;
use ::std::process::{Command, ExitStatus};
use ::std::time::Duration;
#[cfg(feature = "on-memory")]
use ::tempfile::TempDir;
use ::thiserror::Error;
use ::wait_timeout::ChildExt;

const PLUGIN_PATH: &'static str = env!("CARGO_BIN_FILE_PROTOC_PLUGIN_BIN");

/// Error type for this crate.
#[derive(Error, Debug)]
pub enum ErrorKind {
    #[error("IpcIpcError: {0}")]
    IpcIpcError(#[from] ::ipc_channel::ipc::IpcError),
    #[error("IpcError: {0}")]
    IpcError(#[from] ::ipc_channel::Error),
    #[error("IoError: {0}")]
    IoError(#[from] ::std::io::Error),
    #[error("CallbackError: {0}")]
    CallbackError(String),
    #[error("ProtocTimeoutError")]
    ProtocTimeoutError,
    #[error("ProtocProcessError: {0}")]
    ProtocProcessError(ExitStatus),
    #[error("FileNameError")]
    FileNameError,
}

/// Result type for this crate.
pub type Result<T> = ::std::result::Result<T, ErrorKind>;

/// A convenient wrapper for running protoc command with your own plugin code as a closure.
///
/// See the [crate level documentation](crate) for the basic explanation.
///
/// # Example
/// ```no_run
/// # fn run_protoc() {
///     use protoc_plugin_by_closure::Protoc;
///     use std::time::Duration;
///     Protoc::new()
///         .proto_file("my_protobuf_file.proto")
///         .proto_file("my_protobuf_file2.proto")
///         .proto_path("path/to/my/input_proto_dir/")
///         .out_dir("path/to/my/output_dir/")
///         .run(Duration::from_sec(3), |request_bytes| {
///             // Your plugin logic here, which takes the CodeGeneratorRequest bytes
///             // and returns the Result of CodeGeneratorResponse bytes.
/// #           unimplemented!()
///         })
///         .unwrap();
///
///     // The generated file names depend on your plugin logic and the contents of
///     // the input proto files, but typically they will be like this:
///     assert!(std::path::Path("path/to/my/output_dir/my_protobuf_file.rs").exists());
///     assert!(std::path::Path("path/to/my/output_dir/my_protobuf_file2.rs").exists());
/// # }
/// ```
pub struct Protoc {
    protoc_path: PathBuf,
    out_dir: Option<PathBuf>,
    proto_files: Vec<PathBuf>,
    proto_paths: Vec<PathBuf>,
}

impl Protoc {
    /// Creates a new `Protoc` instance.
    pub fn new() -> Self {
        Self {
            protoc_path: "protoc".into(),
            out_dir: None,
            proto_files: Vec::new(),
            proto_paths: Vec::new(),
        }
    }
    /// Sets the path to the `protoc` command. Default is `"protoc"`.
    pub fn protoc_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.protoc_path = path.into();
        self
    }
    /// Sets the output directory for the generated files. Corresponds to `--rust_out` option of `protoc`.
    pub fn out_dir(mut self, path: impl Into<PathBuf>) -> Self {
        self.out_dir = Some(path.into());
        self
    }
    /// Sets the path to the input proto file. Corresponds to the unnamed argument of `protoc`.
    pub fn proto_file(mut self, path: impl Into<PathBuf>) -> Self {
        self.proto_files.push(path.into());
        self
    }
    /// Sets the paths to the input proto files. Corresponds to the unnamed arguments of `protoc`.
    pub fn proto_files<I>(mut self, paths: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<PathBuf>,
    {
        self.proto_files.extend(paths.into_iter().map(|p| p.into()));
        self
    }
    /// Sets the path to the input proto file directory. Corresponds to `--proto_path` option of `protoc`.
    pub fn proto_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.proto_paths.push(path.into());
        self
    }

    /// Runs the `protoc` command with the given closure as a plugin code.
    ///
    /// The `body` param can be any `FnOnce` closure which takes the encoded `CodeGeneratorRequest` bytes
    /// and returns the `Result` of encoded `CodeGeneratorResponse` bytes.
    ///
    /// Set the `timeout` to the maximum duration of the `protoc` command execution.
    pub fn run<F>(self, timeout: Duration, body: F) -> Result<()>
    where
        F: FnOnce(&[u8]) -> ::std::result::Result<Vec<u8>, String>,
    {
        let (ipc_init_server, ipc_init_name) = IpcOneShotServer::new()?;

        let mut process = Command::new(&self.protoc_path)
            .args(&[
                // We name our plugin binary name as "rust-ppbc" here.
                format!("--plugin=protoc-gen-rust-ppbc={}", PLUGIN_PATH),
                format!(
                    "--rust-ppbc_out={}",
                    self.out_dir
                        .as_ref()
                        .map(|p| p.to_str().ok_or(ErrorKind::FileNameError))
                        .transpose()?
                        .unwrap_or(".")
                ),
                format!("--rust-ppbc_opt={}", ipc_init_name),
            ])
            .args(
                self.proto_paths
                    .iter()
                    .map(|x| {
                        Ok(format!(
                            "--proto_path={}",
                            x.to_str().ok_or(ErrorKind::FileNameError)?
                        ))
                    })
                    .collect::<Result<Vec<_>>>()?,
            )
            .args(&self.proto_files)
            .spawn()?;

        {
            // recieve the ipc channels from the plugin exe.
            let (req_recv, res_send): (IpcBytesReceiver, IpcBytesSender) =
                ipc_init_server.accept()?.1;

            let req = req_recv.recv()?;
            let res = (body)(&req).map_err(|x| ErrorKind::CallbackError(x))?;

            res_send.send(&res)?;
        }

        let Some(exit_code) = process.wait_timeout(timeout)? else {
            return Err(ErrorKind::ProtocTimeoutError);
        };
        if !exit_code.success() {
            return Err(ErrorKind::ProtocProcessError(exit_code));
        }

        Ok(())
    }
}

/// A variant of [`Protoc`] which you can run the `protoc` command without touching the actual filesystem.
///
/// Instead of using the actual filesystem, you can pass the name-value pairs of
/// proto files to this struct, and it returns the generated files as name-value pairs.
///
/// This is useful when you want to test your plugin code, or when you want to implement your
/// procedual macro which generates the inlined generated code.
///
/// See the [crate level documentation](crate) or [`Protoc`] for the basic explanations.
///
/// # Example
/// ```no_run
/// # fn run_protoc() {
///     use protoc_plugin_by_closure::ProtocOnMemory;
///     use std::time::Duration;
///     let result_files = Protoc::new()
///         .add_file("my_protobuf_file.proto", r#"
/// syntax = "proto3";
/// package my_package;
/// message MyMessage {
///   string name = 1;
/// }"#)
///         .add_file("another/path/to/my_protobuf_file2.proto", r#"
/// syntax = "proto3";
/// package my_package2;
/// message MyMessage2 {
///   string name2 = 2;
/// }"#)
///         .run(Duration::from_sec(3), |request_bytes| {
///             // Your plugin logic here, which takes the CodeGeneratorRequest bytes
///             // and returns the Result of CodeGeneratorResponse bytes.
/// #           unimplemented!()
///         })
///         .unwrap();
///
///     // The generated filenames depend on your plugin logic, but typically they will be like this:
///     assert!(result_files.iter().any(|(name, _)| name == "my_package.rs"));
///     assert!(result_files.iter().any(|(name, _)| name == "my_package2.rs"));
/// # }
/// ```
#[cfg(feature = "on-memory")]
pub struct ProtocOnMemory {
    protoc: Protoc,
    in_files: Vec<(String, String)>,
}

impl ProtocOnMemory {
    /// Creates a new `ProtocOnMemory` instance.
    pub fn new() -> Self {
        Self {
            protoc: Protoc::new(),
            in_files: Vec::new(),
        }
    }
    /// Sets the path to the `protoc` command. Default is `"protoc"`.
    pub fn protoc_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.protoc = self.protoc.protoc_path(path);
        self
    }
    /// Adds a (virtual) input proto file. Corresponds to the `protoc` command's unnamed argument.
    pub fn add_file(mut self, name: &str, content: &str) -> Self {
        self.in_files.push((name.to_string(), content.to_string()));
        self
    }
    /// Adds (virtual) input proto files. Corresponds to the `protoc` command's unnamed arguments.
    pub fn add_files<I>(mut self, files: I) -> Self
    where
        I: IntoIterator<Item = (String, String)>,
    {
        self.in_files.extend(files);
        self
    }

    /// Runs the `protoc` command with the given closure as a plugin code.
    ///
    /// The `body` param can be any `FnOnce` closure which takes the encoded `CodeGeneratorRequest` bytes
    /// and returns the `Result` of encoded `CodeGeneratorResponse` bytes.
    ///
    /// Set the `timeout` to the maximum duration of the `protoc` command execution.
    pub fn run<F>(self, timeout: Duration, func: F) -> Result<Vec<(String, String)>>
    where
        F: FnOnce(&[u8]) -> ::std::result::Result<Vec<u8>, String>,
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
            .run(timeout, func)?;

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
