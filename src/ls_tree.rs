use crate::utils;
use anyhow::{Context, Result};

pub fn ls_tree(hash: &str, name_only: bool) -> Result<()> {
    let object = utils::read_object(hash)
        .with_context(|| format!("Failed to read the object from {hash}"))?;
    let (_, mut remaining) = utils::split_once_at_value(&object, 0)
        .context("Failed to split the object into header and content")?;
    while !remaining.is_empty() {
        let (mode, name, hash, rest) = get_first_object(remaining)?;
        if name_only {
            println!("{name}");
        } else {
            print_full(mode, name, hash);
        }
        remaining = rest;
    }
    Ok(())
}

fn get_first_object(content: &[u8]) -> Result<(&str, &str, &[u8], &[u8])> {
    let (mode_and_name, hash_and_rest) =
        utils::split_once_at_value(content, 0).context("Failed to extract entry name and hash")?;
    let (mode, name) = utils::split_once_at_value(mode_and_name, b' ')
        .context("Failed to extract entry mode and name")?;
    let (hash, rest) = hash_and_rest.split_at(20);
    let mode_string =
        str::from_utf8(mode).context("Failed to convert the entry mode from UTF-8 to string")?;
    let name_string =
        str::from_utf8(name).context("Failed to convert the entry name from UTF-8 to string")?;
    Ok((mode_string, name_string, hash, rest))
}

fn print_full(mode: &str, name: &str, hash: &[u8]) {
    let object_type = get_object_type(mode);
    print!("{:0>6} {object_type} ", mode);
    for byte in hash {
        print!("{:02x}", byte);
    }
    println!("\t{name}");
}

fn get_object_type(mode: &str) -> &'static str {
    match mode {
        "40000" => "tree",
        "100644" | "100755" | "120000" => "blob",
        _ => "unknown",
    }
}
