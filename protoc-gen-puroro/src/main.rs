//! Thin `protoc` plugin entrypoint.
//!
//! All real work lives in the `protoc_gen_puroro` library.

use ::anyhow::{Context, Result};
use ::protoc_gen_puroro::run_plugin;

fn main() -> Result<()> {
    run_plugin().context("protoc-gen-puroro failed")?;
    Ok(())
}
