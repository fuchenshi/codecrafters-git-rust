use clap::{Parser, Subcommand};
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use sha1::{Digest, Sha1};
use std::fs;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// initialize a git repository
    Init,
    /// view contents of objects
    CatFile {
        /// pretty-print the contents
        #[arg(short = 'p')]
        pretty: bool,
        /// object hash
        hash: String,
    },
    /// compute object ID
    HashObject {
        /// write the object
        #[arg(short = 'w')]
        write: bool,
        /// file path
        path: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => init(),
        Commands::CatFile { pretty, hash } => {
            if *pretty {
                read_blob(hash);
            } else {
                println!("unknown command");
            }
        }
        Commands::HashObject { write, path } => {
            if *write {
                create_blob(path);
            } else {
                println!("unknown command");
            }
        }
    }
}

fn init() {
    fs::create_dir(".git").unwrap();
    fs::create_dir(".git/objects").unwrap();
    fs::create_dir(".git/refs").unwrap();
    fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
    println!("Initialized git directory")
}

fn read_blob(object_hash: &str) {
    let object_path = get_path_from_hash(object_hash);
    let compressed_data = fs::read(object_path).unwrap();
    let decompressed_data = decompress(&compressed_data);
    let (_, object_content) = decompressed_data.split_once('\0').unwrap();
    print!("{}", object_content)
}

fn create_blob(file_path: &str) {
    let object_data = get_blob_from_file(file_path);
    let compressed_data = compress(&object_data);
    let object_hash = hash(&object_data);
    let object_path = get_path_from_hash(&object_hash);
    save_object(&object_path, &compressed_data);
    println!("{}", object_hash)
}

fn get_path_from_hash(hash: &str) -> PathBuf {
    let (hash_prefix, hash_suffix) = hash.split_at(2);
    Path::new(".git/objects")
        .join(hash_prefix)
        .join(hash_suffix)
}

fn compress(data: &[u8]) -> Vec<u8> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data).unwrap();
    encoder.finish().unwrap()
}

fn decompress(data: &[u8]) -> String {
    let mut decoder = ZlibDecoder::new(&data[..]);
    let mut decompressed_data = String::new();
    decoder.read_to_string(&mut decompressed_data).unwrap();
    decompressed_data
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

fn get_blob_from_file(path: &str) -> Vec<u8> {
    let file_content = fs::read(path).unwrap();
    let object_header = format!("blob {}\0", file_content.len());
    let object_header_bytes = object_header.into_bytes();
    [object_header_bytes, file_content].concat()
}
