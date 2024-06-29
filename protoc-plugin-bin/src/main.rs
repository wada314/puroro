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

use ::anyhow::{anyhow, Result};
use ::ipc_channel::ipc::{bytes_channel, IpcSender};
use ::std::io::{stdin, stdout, Read, Write};

fn eat_variant_i64(input: &mut &[u8]) -> Result<i64> {
    let mut result = 0i64;
    for i in 0..9 {
        let Some((byte, new_input)) = input.split_first() else {
            return Err(anyhow!("unexpected EOF while parsing variant"));
        };
        *input = new_input;
        if i != 8 {
            result |= ((byte & 0x7F) as i64) << (i * 7);
            if byte & 0x80 == 0 {
                return Ok(result);
            }
        } else {
            if byte & 0xFE != 0 {
                return Err(anyhow!("variant is too long"));
            }
            result |= ((byte & 0x01) as i64) << (7 * 9);
            return Ok(result);
        }
    }
    unreachable!();
}

fn eat_variant_i32(input: &mut &[u8]) -> Result<i32> {
    Ok(eat_variant_i64(input)?.try_into()?)
}

fn find_last_string_field(mut input: &[u8], target_field_number: i32) -> Result<Option<String>> {
    enum Wire {
        Varint,
        I64,
        Len,
        SGroup,
        EGroup,
        I32,
    }
    impl TryFrom<i32> for Wire {
        type Error = anyhow::Error;
        fn try_from(value: i32) -> Result<Self> {
            match value {
                0 => Ok(Self::Varint),
                1 => Ok(Self::I64),
                2 => Ok(Self::Len),
                3 => Ok(Self::SGroup),
                4 => Ok(Self::EGroup),
                5 => Ok(Self::I32),
                _ => Err(anyhow!("invalid wire type")),
            }
        }
    }

    let mut result: Option<String> = None;
    while !input.is_empty() {
        let tag = eat_variant_i32(&mut input)?;
        let wire: Wire = (tag & 0x07).try_into()?;
        let field_number = tag >> 3;
        match wire {
            Wire::Varint => {
                eat_variant_i64(&mut input)?;
            }
            Wire::I64 => {
                let Some((_, new_input)) = input.split_at_checked(8) else {
                    return Err(anyhow!("unexpected EOF while parsing i64"));
                };
                input = new_input;
            }
            Wire::Len => {
                let len = eat_variant_i32(&mut input)?;
                let Some((payload, new_input)) = input.split_at_checked(len.try_into()?) else {
                    return Err(anyhow!("unexpected EOF while parsing LEN tag payload"));
                };
                input = new_input;
                if field_number == target_field_number {
                    result = Some(String::from_utf8(payload.to_vec())?);
                }
            }
            Wire::SGroup | Wire::EGroup => {
                return Err(anyhow!(
                    "{} does not support protobuf groups.",
                    env!("CARGO_PKG_NAME")
                ));
            }
            Wire::I32 => {
                let Some((_, new_input)) = input.split_at_checked(4) else {
                    return Err(anyhow!("unexpected EOF while parsing i32"));
                };
                input = new_input;
            }
        }
    }
    Ok(result)
}

fn main() -> Result<()> {
    let mut input_buffer = Vec::new();
    stdin().read_to_end(&mut input_buffer)?;

    let ipc_init_key = find_last_string_field(&input_buffer, 2)?.ok_or_else(|| {
        anyhow!(
            "input CodeGeneratorRequest proto does not contain a parameter field (2) (IPC init key)."
        )
    })?;
    let ipc_init_send = IpcSender::connect(ipc_init_key)?;
    let (req_send, req_recv) = bytes_channel()?;
    let (res_send, res_recv) = bytes_channel()?;
    ipc_init_send.send((req_recv, res_send))?;

    req_send.send(&input_buffer)?;
    let response = res_recv.recv()?;

    stdout().write_all(&response)?;

    Ok(())
}
