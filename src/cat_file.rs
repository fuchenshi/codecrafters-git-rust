use crate::utils;

pub fn read_blob(hash: &str) {
    let object = utils::read_object(hash);
    let (_, object_content_bytes) = utils::split_once_at_value(&object, 0);
    let object_content = str::from_utf8(object_content_bytes).unwrap();
    print!("{object_content}");
}
