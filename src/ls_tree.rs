use crate::utils;

pub fn ls_tree(hash: &str, name_only: bool) {
    let (_, object_content) = utils::read_object(hash);
    let mut remaining = &object_content[..];
    while !remaining.is_empty() {
        let (mode, name, hash, rest) = get_first_object(remaining);
        if name_only {
            println!("{name}");
        } else {
            print_full(mode, name, hash);
        }
        remaining = rest;
    }
}

fn get_first_object(content: &str) -> (&str, &str, &[u8], &str) {
    let (mode_and_name, hash_and_rest) = content.split_once('\0').unwrap();
    let (mode, name) = mode_and_name.split_once(' ').unwrap();
    let (hash, rest) = hash_and_rest.split_at(20);
    (mode, name, hash.as_bytes(), rest)
}

fn print_full(mode: &str, name: &str, hash: &[u8]) {
    let object_type = get_object_type(mode);
    print!("{:0>6} {object_type} ", mode);
    for byte in hash {
        print!("{:02x}", byte);
    }
    println!("\t{name}");
}

fn get_object_type(mode: &str) -> &'static str {
    match mode {
        "40000" => "tree",
        "100644" | "100755" | "120000" => "blob",
        _ => "unknown",
    }
}
