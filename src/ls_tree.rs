use crate::utils;

pub fn ls_tree(hash: &str, name_only: bool) {
    let object = utils::read_object(hash);
    let (_, mut remaining) = utils::split_once_at_value(&object, 0);
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

fn get_first_object(content: &[u8]) -> (&str, &str, &[u8], &[u8]) {
    let (mode_and_name, hash_and_rest) = utils::split_once_at_value(content, 0);
    let (mode, name) = utils::split_once_at_value(mode_and_name, ' ' as u8);
    let (hash, rest) = hash_and_rest.split_at(20);
    (
        str::from_utf8(mode).unwrap(),
        str::from_utf8(name).unwrap(),
        hash,
        rest,
    )
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
