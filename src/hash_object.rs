use crate::utils;
use anyhow::{Context, Result};
use flate2::{write::ZlibEncoder, Compression};
use sha1::{Digest, Sha1};
use std::{fs, io::prelude::*, path::PathBuf};

pub fn create_blob(file_path: &str) -> Result<()> {
    let object_data = get_blob_from_file(file_path)
        .with_context(|| format!("Failed to read the file from {file_path}"))?;
    let compressed_data = compress(&object_data).context("Failed to compress the object")?;
    let object_hash = hash(&object_data);
    let object_path = utils::get_path_from_hash(&object_hash);
    save_object(&object_path, &compressed_data).context("Failed to save the object")?;
    println!("{object_hash}");
    Ok(())
}

fn get_blob_from_file(path: &str) -> Result<Vec<u8>> {
    let file_content = fs::read(path)?;
    let file_size = file_content.len();
    let object_header = format!("blob {file_size}\0");
    let object_header_bytes = object_header.into_bytes();
    let object_data = [object_header_bytes, file_content].concat();
    Ok(object_data)
}

fn compress(data: &[u8]) -> Result<Vec<u8>> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data)?;
    let compressed = encoder.finish()?;
    Ok(compressed)
}

fn hash(data: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(data);
    let object_hash = hasher.finalize();
    format!("{:x}", object_hash)
}

fn save_object(path: &PathBuf, contents: &[u8]) -> Result<()> {
    let dir = path
        .parent()
        .expect("Object path should at least have a parent");
    fs::create_dir_all(dir)?;
    fs::write(path, contents)?;
    Ok(())
}
