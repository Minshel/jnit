fn flush(process: &mut String, processes: &mut Vec<String>) {
    let command = process.trim().to_string();
    if !command.is_empty() {
        processes.push(command);
    }
    process.clear();
}

pub fn parse_processes(content: String, req_group: String) -> Vec<String> {
    let mut processes: Vec<String> = vec![];
    let (mut in_group, mut found) = (false, false);
    let mut process = String::new();
    let chars: Vec<char> = content.chars().collect();
    let mut pos = 0;

    while pos < chars.len() {
        if chars[pos] == '{' {
            let mut name = String::new();
            pos += 1;
            while pos < chars.len() && chars[pos] != '}' {
                name.push(chars[pos]);
                pos += 1;
            }

            if pos < chars.len() {
                pos += 1;
            }

            let name = name.trim().to_string();
            flush(&mut process, &mut processes);
            if name.is_empty() {
                in_group = false;
            } else {
                in_group = name == req_group;
                if in_group {
                    found = true;
                }
            }
            continue;
        }

        if in_group {
            match chars[pos] {
                ';' => flush(&mut process, &mut processes),
                '\r' => {}
                _ => process.push(chars[pos]),
            }
        }
        pos += 1;
    }

    flush(&mut process, &mut processes);

    if !found {
        panic!("JNit: requested process group '{}' not found in config file", req_group);
    }
    processes
}
