use flate2::read::ZlibDecoder;
#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let cmd = &args[1];
            match &cmd[..] {
                "init" => {
                    fs::create_dir(".git").unwrap();
                    fs::create_dir(".git/objects").unwrap();
                    fs::create_dir(".git/refs").unwrap();
                    fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
                    println!("Initialized git directory")
                }
                _ => println!("unknown command"),
            }
        }
        4 => {
            let cmd = &args[1];
            match &cmd[..] {
                "cat-file" => {
                    let flag = &args[2];
                    match &flag[..] {
                        "-p" => {
                            let object_hash = &args[3];
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
                        _ => println!("unknown command"),
                    }
                }
                _ => println!("unknown command"),
            }
        }
        _ => println!("unknown command"),
    }
}
