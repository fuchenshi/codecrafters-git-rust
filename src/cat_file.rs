use crate::utils;
use flate2::read::ZlibDecoder;
use std::{fs, io::prelude::*};

pub fn read_blob(object_hash: &str) {
    let object_path = utils::get_path_from_hash(object_hash);
    let compressed_data = fs::read(object_path).unwrap();
    let decompressed_data = decompress(&compressed_data);
    let (_, object_content) = decompressed_data.split_once('\0').unwrap();
    print!("{}", object_content)
}

fn decompress(data: &[u8]) -> String {
    let mut decoder = ZlibDecoder::new(&data[..]);
    let mut decompressed_data = String::new();
    decoder.read_to_string(&mut decompressed_data).unwrap();
    decompressed_data
}
