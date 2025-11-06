use crate::utils;

pub fn read_blob(hash: &str) {
    let (_, object_content) = utils::read_object(hash);
    print!("{object_content}");
}
