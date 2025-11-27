use crate::utils;
use anyhow::{bail, Context, Result};
use std::io::{self, Write};

pub fn read_blob(hash: &str) -> Result<()> {
    if hash.len() != 40 {
        bail!("Hash must be 40 characters long");
    }
    let object = utils::read_object(hash)
        .with_context(|| format!("Failed to read the object from {hash}"))?;
    let (_, content) = utils::split_once_at_value(&object, 0)
        .context("Failed to split the object into header and content")?;
    io::stdout().write_all(content)?;
    Ok(())
}
