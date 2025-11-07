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

pub fn read_object(hash: &str) -> Vec<u8> {
    let object_path = get_path_from_hash(hash);
    let compressed_data = fs::read(object_path).unwrap();
    decompress(&compressed_data)
}

pub fn split_once_at_value(vec: &[u8], value: u8) -> (&[u8], &[u8]) {
    vec.iter()
        .position(|&b| b == value)
        .map(|idx| (&vec[..idx], &vec[idx + 1..]))
        .unwrap()
}

fn decompress(data: &[u8]) -> Vec<u8> {
    let mut decoder = ZlibDecoder::new(&data[..]);
    let mut decompressed_data = Vec::new();
    decoder.read_to_end(&mut decompressed_data).unwrap();
    decompressed_data
}
