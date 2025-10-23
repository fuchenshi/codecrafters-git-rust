use clap::{Parser, Subcommand};
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use sha1::{Digest, Sha1};
use std::fs;
use std::io::prelude::*;
use std::path::Path;

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

fn create_blob(file_path: &str) {
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
