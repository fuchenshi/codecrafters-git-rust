use flate2::read::ZlibDecoder;
use std::{
    fs,
    io::prelude::*,
    path::{Path, PathBuf},
};

pub fn get_path_from_hash(hash: &str) -> PathBuf {
    let (hash_prefix, hash_suffix) = hash.split_at(2);
    Path::new(".git/objects")
        .join(hash_prefix)
        .join(hash_suffix)
}

pub fn read_object(hash: &str) -> (String, String) {
    let object_path = get_path_from_hash(hash);
    let compressed_data = fs::read(object_path).unwrap();
    let decompressed_data = decompress(&compressed_data);
    let (object_header, object_content) = decompressed_data.split_once('\0').unwrap();
    (object_header.to_string(), object_content.to_string())
}

fn decompress(data: &[u8]) -> String {
    let mut decoder = ZlibDecoder::new(&data[..]);
    let mut decompressed_data = String::new();
    decoder.read_to_string(&mut decompressed_data).unwrap();
    decompressed_data
}
