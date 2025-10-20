use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use sha1::{Digest, Sha1};
use std::env;
use std::fs;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let cmd = &args[1];
            match &cmd[..] {
                "init" => init(),
                _ => println!("unknown command"),
            }
        }
        4 => {
            let cmd = &args[1];
            match &cmd[..] {
                "cat-file" => {
                    let flag = &args[2];
                    match &flag[..] {
                        "-p" => read_blob(&args[3]),
                        _ => println!("unknown command"),
                    }
                }
                "hash-object" => {
                    let flag = &args[2];
                    match &flag[..] {
                        "-w" => create_blob(&args[3]),
                        _ => println!("unknown command"),
                    }
                }
                _ => println!("unknown command"),
            }
        }
        _ => println!("unknown command"),
    }
}

fn init() {
    fs::create_dir(".git").unwrap();
    fs::create_dir(".git/objects").unwrap();
    fs::create_dir(".git/refs").unwrap();
    fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
    println!("Initialized git directory")
}

fn read_blob(object_hash: &String) {
    let (hash_prefix, hash_suffix) = object_hash.split_at(2);
    let object_path = Path::new(".git/objects")
        .join(hash_prefix)
        .join(hash_suffix);

    let compressed_data = fs::read(object_path).unwrap();
    let mut decoder = ZlibDecoder::new(&compressed_data[..]);
    let mut decompressed_data = String::new();
    decoder.read_to_string(&mut decompressed_data).unwrap();

    let (_, object_content) = decompressed_data.split_once('\0').unwrap();
    print!("{}", object_content)
}

fn create_blob(file_path: &String) {
    let file_content = fs::read(file_path).unwrap();

    let object_header = format!("blob {}\0", file_content.len());
    let object_header_bytes = object_header.into_bytes();
    let object_data = [object_header_bytes, file_content].concat();

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&object_data).unwrap();
    let compressed_data = encoder.finish().unwrap();

    let mut hasher = Sha1::new();
    hasher.update(&object_data);
    let object_hash = hasher.finalize();
    let object_hash_string = format!("{:x}", object_hash);
    let (hash_prefix, hash_suffix) = object_hash_string.split_at(2);
    let object_dir = Path::new(".git/objects").join(hash_prefix);
    let object_path = object_dir.join(hash_suffix);

    fs::create_dir_all(object_dir).unwrap();
    fs::write(object_path, compressed_data).unwrap();
    println!("{}", object_hash_string)
}
