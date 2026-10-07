use std::{fs::File, io::Read};
use crate::parser;

pub fn get_processes(mut file: File) -> Vec<String> {
    let mut processes: Vec<String> = vec!{};

    // read
    let mut content = String::new();
    file.read_to_string(&mut content);

    parser::parse_processes(content)
}
