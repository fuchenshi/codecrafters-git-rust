mod cat_file;
mod hash_object;
mod init;
mod utils;

use clap::{Parser, Subcommand};

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
        Commands::Init => init::init(),
        Commands::CatFile { pretty, hash } => {
            if *pretty {
                cat_file::read_blob(hash);
            } else {
                println!("unknown command");
            }
        }
        Commands::HashObject { write, path } => {
            if *write {
                hash_object::create_blob(path);
            } else {
                println!("unknown command");
            }
        }
    }
}
