use std::{fs::File, io::Read};
use crate::parser;

pub fn get_processes(mut file: File, req_group: &str) -> Vec<String> {
    let mut content = String::new();
    file.read_to_string(&mut content);

    parser::parse_processes(content, req_group.to_string())
}
