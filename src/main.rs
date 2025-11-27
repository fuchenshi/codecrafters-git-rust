mod cat_file;
mod hash_object;
mod init;
mod ls_tree;
mod utils;

use anyhow::{bail, Result};
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
    /// show the contents of a tree object
    LsTree {
        /// show only file names
        #[arg(long = "name-only")]
        name_only: bool,
        /// tree hash
        hash: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => init::init()?,
        Commands::CatFile { pretty, hash } => {
            if *pretty {
                cat_file::read_blob(hash)?;
            } else {
                bail!("Only -p is supported for now");
            }
        }
        Commands::HashObject { write, path } => {
            if *write {
                hash_object::create_blob(path)?;
            } else {
                bail!("Only -w is supported for now");
            }
        }
        Commands::LsTree { name_only, hash } => ls_tree::ls_tree(hash, *name_only)?,
    }

    Ok(())
}
