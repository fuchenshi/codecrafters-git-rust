use std::path::{Path, PathBuf};

pub fn get_path_from_hash(hash: &str) -> PathBuf {
    let (hash_prefix, hash_suffix) = hash.split_at(2);
    Path::new(".git/objects")
        .join(hash_prefix)
        .join(hash_suffix)
}
