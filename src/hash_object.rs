use crate::utils;
use flate2::{write::ZlibEncoder, Compression};
use sha1::{Digest, Sha1};
use std::{fs, io::prelude::*, path::PathBuf};

pub fn create_blob(file_path: &str) {
    let object_data = get_blob_from_file(file_path);
    let compressed_data = compress(&object_data);
    let object_hash = hash(&object_data);
    let object_path = utils::get_path_from_hash(&object_hash);
    save_object(&object_path, &compressed_data);
    println!("{}", object_hash)
}

fn get_blob_from_file(path: &str) -> Vec<u8> {
    let file_content = fs::read(path).unwrap();
    let object_header = format!("blob {}\0", file_content.len());
    let object_header_bytes = object_header.into_bytes();
    [object_header_bytes, file_content].concat()
}

fn compress(data: &[u8]) -> Vec<u8> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data).unwrap();
    encoder.finish().unwrap()
}

fn hash(data: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(data);
    let object_hash = hasher.finalize();
    format!("{:x}", object_hash)
}

fn save_object(path: &PathBuf, contents: &[u8]) {
    let dir = path.parent().unwrap();
    fs::create_dir_all(dir).unwrap();
    fs::write(path, contents).unwrap();
}
